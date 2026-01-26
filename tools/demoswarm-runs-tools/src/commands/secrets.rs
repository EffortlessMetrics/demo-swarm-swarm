//! Secret scanning and redaction commands.
//!
//! Security note: This module NEVER prints secret content to stdout.
//! - `scan` writes findings (file/line/type only) to JSON
//! - `redact` modifies files in-place, prints only status

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::common::write_json_atomic;
use crate::output::print_scalar;
use crate::walk::{SkippedItem, walk_dir_excluding_verbose};

/// Built-in secret detection patterns. Each tuple is (regex, type-name).
const BUILTIN_PATTERNS: &[(&str, &str)] = &[
    (r"gh[pousr]_[A-Za-z0-9_]{36,}", "github-token"),
    (r"AKIA[0-9A-Z]{16}", "aws-access-key"),
    (r"sk_live_[0-9a-zA-Z]{24,}", "stripe-key"),
    (r"-----BEGIN\s.*PRIVATE KEY-----", "private-key"),
    (
        r"eyJ[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*",
        "jwt-token",
    ),
];

/// A secret pattern definition for configuration files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDef {
    /// The regex pattern to match
    pub pattern: String,
    /// The type name for this secret (e.g., "github-token")
    #[serde(rename = "type")]
    pub type_name: String,
}

/// Configuration file format for custom patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternsConfig {
    /// List of custom secret patterns
    pub patterns: Vec<PatternDef>,
}

/// A compiled pattern ready for scanning.
#[derive(Debug)]
struct CompiledPattern {
    regex: Regex,
    type_name: String,
    #[allow(dead_code)]
    pattern_str: String,
}

#[derive(Args, Debug)]
pub struct SecretsCommand {
    #[command(subcommand)]
    pub command: SecretsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum SecretsSubcommand {
    /// Scan a file/directory for secrets (locations only); writes JSON findings
    Scan(SecretsScan),

    /// Redact a specific secret type in a file
    Redact(SecretsRedact),
}

#[derive(Args, Debug)]
pub struct SecretsScan {
    /// Path to scan (file or directory)
    #[arg(long)]
    pub path: String,

    /// Output file for JSON findings
    #[arg(long)]
    pub output: String,

    /// Path to a JSON or YAML file with additional patterns.
    /// Patterns are merged with built-in patterns (built-in first, config second).
    #[arg(long)]
    pub patterns_file: Option<String>,

    /// Log skipped paths with reasons to stderr
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct SecretsRedact {
    /// File to redact secrets from
    #[arg(long)]
    pub file: String,

    /// Type of secret to redact.
    /// Use a built-in type or a custom type defined in patterns file.
    #[arg(long)]
    pub r#type: String,

    /// Path to a JSON or YAML file with additional patterns.
    /// Required when redacting custom secret types not in built-in list.
    #[arg(long)]
    pub patterns_file: Option<String>,
}

pub fn run(cmd: SecretsCommand) -> Result<()> {
    match cmd.command {
        SecretsSubcommand::Scan(args) => scan(&args),
        SecretsSubcommand::Redact(args) => redact(&args),
    }
}

/// Load patterns from a JSON or YAML configuration file.
/// Returns an error if the file cannot be read or parsed, or if any regex is invalid.
fn load_patterns_from_file(path: &Path) -> Result<Vec<PatternDef>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read patterns file: {}", path.display()))?;

    let config: PatternsConfig = if path
        .extension()
        .is_some_and(|ext| ext == "yaml" || ext == "yml")
    {
        serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML patterns file: {}", path.display()))?
    } else {
        // Default to JSON
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON patterns file: {}", path.display()))?
    };

    // Validate all regexes at load time
    for (i, pat) in config.patterns.iter().enumerate() {
        Regex::new(&pat.pattern).with_context(|| {
            format!(
                "Invalid regex in patterns file at index {}: pattern='{}', type='{}'",
                i, pat.pattern, pat.type_name
            )
        })?;
    }

    Ok(config.patterns)
}

/// Compile patterns into ready-to-use regex objects.
/// Merges built-in patterns with config patterns (built-in first, config second).
fn compile_patterns(patterns_file: Option<&str>) -> Result<Vec<CompiledPattern>> {
    let mut compiled = Vec::new();

    // Add built-in patterns first
    for (pat, typ) in BUILTIN_PATTERNS {
        if let Ok(re) = Regex::new(pat) {
            compiled.push(CompiledPattern {
                regex: re,
                type_name: typ.to_string(),
                pattern_str: pat.to_string(),
            });
        }
    }

    // Add config patterns second (if provided)
    if let Some(path_str) = patterns_file {
        let path = Path::new(path_str);
        let custom_patterns = load_patterns_from_file(path)?;

        for pat in custom_patterns {
            // Regex already validated in load_patterns_from_file
            let re = Regex::new(&pat.pattern).expect("regex already validated");
            compiled.push(CompiledPattern {
                regex: re,
                type_name: pat.type_name,
                pattern_str: pat.pattern,
            });
        }
    }

    Ok(compiled)
}

/// Find the pattern string for a given type name.
fn find_pattern_for_type(type_name: &str, patterns_file: Option<&str>) -> Result<Option<String>> {
    // Check built-in patterns first
    for (pat, typ) in BUILTIN_PATTERNS {
        if *typ == type_name {
            return Ok(Some(pat.to_string()));
        }
    }

    // Check custom patterns if file provided
    if let Some(path_str) = patterns_file {
        let path = Path::new(path_str);
        let custom_patterns = load_patterns_from_file(path)?;
        for pat in custom_patterns {
            if pat.type_name == type_name {
                return Ok(Some(pat.pattern));
            }
        }
    }

    Ok(None)
}

fn scan(args: &SecretsScan) -> Result<()> {
    let verbose = args.verbose;

    // Validate output path first (before any scanning)
    let out_path = match validate_output_path_boundary(&args.output) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Security error: {e:#}");
            print_scalar("PATH_BOUNDARY_ERROR");
            return Ok(());
        }
    };

    // Special case: scan path doesn't exist yet - write SCAN_PATH_MISSING
    let root_raw = Path::new(&args.path);
    if !root_raw.exists() {
        let v = json!({ "status": "SCAN_PATH_MISSING", "findings": [], "skipped_count": 0 });
        write_json_atomic(&out_path, &v).map_err(|e| {
            eprintln!(
                "Warning: failed to write secrets findings JSON ({}): {e:#}",
                out_path.display()
            );
            e
        })?;
        print_scalar("SCAN_PATH_MISSING");
        return Ok(());
    }

    // Validate scan path is within repository boundary (security: prevents path traversal)
    let root = match validate_path_boundary(&args.path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Security error: {e:#}");
            let v = json!({
                "status": "PATH_BOUNDARY_ERROR",
                "error": format!("{e:#}"),
                "findings": [],
                "skipped_count": 0
            });
            write_json_atomic(&out_path, &v).map_err(|write_e| {
                eprintln!(
                    "Warning: failed to write error JSON ({}): {write_e:#}",
                    out_path.display()
                );
                write_e
            })?;
            print_scalar("PATH_BOUNDARY_ERROR");
            return Ok(());
        }
    };

    // Compile patterns (built-in + custom if provided)
    let compiled = match compile_patterns(args.patterns_file.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            // Pattern loading/validation failed - report error in output
            let v = json!({
                "status": "PATTERN_ERROR",
                "error": format!("{e:#}"),
                "findings": [],
                "skipped_count": 0
            });
            write_json_atomic(&out_path, &v).map_err(|write_e| {
                eprintln!(
                    "Warning: failed to write secrets findings JSON ({}): {write_e:#}",
                    out_path.display()
                );
                write_e
            })?;
            print_scalar("PATTERN_ERROR");
            return Ok(());
        }
    };

    let mut findings: Vec<Value> = Vec::new();
    let mut skipped: Vec<SkippedItem> = Vec::new();

    // Use shared walker with exclusions and verbose mode for security scanning
    let mut walker = walk_dir_excluding_verbose(&root, EXCLUDED_DIRS, verbose);

    // Stream files directly instead of collecting (reduces memory usage)
    for f in walker.by_ref() {
        scan_one_file(&f, &compiled, &mut findings, &mut skipped, verbose);
    }

    // Get skipped items from directory walking
    skipped.extend(walker.take_skipped_items());

    let skipped_count = skipped.len();

    let status = if findings.is_empty() {
        "CLEAN"
    } else {
        "SECRETS_FOUND"
    };
    let v = json!({ "status": status, "findings": findings, "skipped_count": skipped_count });
    write_json_atomic(&out_path, &v).map_err(|e| {
        eprintln!(
            "Warning: failed to write secrets findings JSON ({}): {e:#}",
            out_path.display()
        );
        e
    })?;
    print_scalar(status);
    Ok(())
}

/// Directory names to exclude from scanning (deterministic fixed list).
const EXCLUDED_DIRS: &[&str] = &[".git", "target", "node_modules", ".demoswarm"];

/// Validate and canonicalize a path, preventing path traversal attacks.
///
/// Security: This function prevents path traversal attacks by detecting when
/// relative paths with ".." components would escape the current working directory.
///
/// The validation strategy:
/// 1. Absolute paths are allowed (explicit, not traversal attacks)
/// 2. Relative paths are checked for ".." traversal that escapes CWD
/// 3. Paths that canonicalize to somewhere outside CWD when they started
///    with relative components are blocked
///
/// This allows legitimate use cases (temp directories, absolute paths) while
/// blocking the attack vector of crafted relative paths like "../../../etc/passwd".
fn validate_path_boundary(path: &str) -> Result<PathBuf> {
    let input_path = Path::new(path);

    // Canonicalize the input path first
    let canonical = input_path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path))?;

    // If the path is absolute (not relative), allow it.
    // Absolute paths are explicit - if someone provides /etc/passwd directly,
    // that's their intent, not a traversal attack.
    if input_path.is_absolute() {
        return Ok(canonical);
    }

    // For relative paths, check if they escape the CWD
    // This catches attacks like "../../../etc/passwd"
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let cwd_canonical = cwd
        .canonicalize()
        .context("Failed to canonicalize working directory")?;

    // Check if the canonical path is under CWD
    if canonical.starts_with(&cwd_canonical) {
        return Ok(canonical);
    }

    // Relative path escapes CWD - this is the attack pattern
    bail!(
        "Relative path '{}' resolves to '{}' which is outside the repository boundary '{}'. \
         Use an absolute path if you need to access files outside the working directory.",
        path,
        canonical.display(),
        cwd_canonical.display()
    )
}

/// Validate path boundary for output files - allows creating new files.
///
/// Security: For output paths that don't exist yet, we validate the parent directory.
/// Like validate_path_boundary, this allows absolute paths but blocks relative
/// path traversal attacks.
fn validate_output_path_boundary(path: &str) -> Result<PathBuf> {
    let input_path = Path::new(path);

    // If path is absolute, allow it (explicit intent, not traversal)
    if input_path.is_absolute() {
        // If path exists, canonicalize it
        if input_path.exists() {
            return input_path
                .canonicalize()
                .with_context(|| format!("Failed to canonicalize path: {}", path));
        }

        // For non-existent absolute paths, validate parent exists and return intended path
        let parent = input_path.parent().unwrap_or(Path::new("."));
        if !parent.exists() {
            bail!("Output path parent '{}' does not exist", parent.display());
        }
        let parent_canonical = parent
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize parent: {}", parent.display()))?;
        let filename = input_path.file_name().unwrap_or_default();
        return Ok(parent_canonical.join(filename));
    }

    // For relative paths, check boundary
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let cwd_canonical = cwd
        .canonicalize()
        .context("Failed to canonicalize working directory")?;

    // If path exists, canonicalize and check
    if input_path.exists() {
        let canonical = input_path
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize path: {}", path))?;

        if canonical.starts_with(&cwd_canonical) {
            return Ok(canonical);
        }

        bail!(
            "Relative output path '{}' resolves outside the repository boundary",
            path
        );
    }

    // For non-existent relative paths, validate the parent directory
    let parent = input_path.parent().unwrap_or(Path::new("."));
    let parent_canonical = if parent.as_os_str().is_empty() || parent == Path::new(".") {
        cwd_canonical.clone()
    } else {
        parent.canonicalize().with_context(|| {
            format!(
                "Output path parent '{}' does not exist or is inaccessible",
                parent.display()
            )
        })?
    };

    if parent_canonical.starts_with(&cwd_canonical) {
        // Return the intended output path (parent + filename)
        let filename = input_path.file_name().unwrap_or_default();
        return Ok(parent_canonical.join(filename));
    }

    bail!(
        "Relative output path '{}' would be created outside the repository boundary",
        path
    )
}

fn scan_one_file(
    path: &Path,
    patterns: &[CompiledPattern],
    findings: &mut Vec<Value>,
    skipped: &mut Vec<SkippedItem>,
    verbose: bool,
) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            let reason = format!("failed to read file: {}", e);
            if verbose {
                eprintln!("Warning: skipped {}: {}", path.display(), reason);
            }
            skipped.push(SkippedItem {
                path: path.to_path_buf(),
                reason,
            });
            return;
        }
    };

    // Lossy read; we never emit matched content
    let content = String::from_utf8_lossy(&bytes);
    let lines: Vec<&str> = content.lines().collect();

    for pat in patterns {
        let mut line_nums: Vec<String> = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if pat.regex.is_match(line) {
                line_nums.push((i + 1).to_string());
            }
        }
        if !line_nums.is_empty() {
            findings.push(json!({
                "file": path.to_string_lossy(),
                "type": pat.type_name,
                "lines": line_nums.join(",")
            }));
        }
    }
}

fn redact(args: &SecretsRedact) -> Result<()> {
    let path_raw = Path::new(&args.file);
    if !path_raw.is_file() {
        print_scalar("FILE_NOT_FOUND");
        return Ok(());
    }

    // Validate file path is within repository boundary (security: prevents path traversal)
    let path = match validate_path_boundary(&args.file) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Security error: {e:#}");
            print_scalar("PATH_BOUNDARY_ERROR");
            return Ok(());
        }
    };

    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(_) => {
            print_scalar("null");
            return Ok(());
        }
    };
    let s = String::from_utf8_lossy(&bytes).to_string();

    // Find the pattern for the requested type
    let pattern_opt = match find_pattern_for_type(&args.r#type, args.patterns_file.as_deref()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading patterns: {e:#}");
            print_scalar("null");
            return Ok(());
        }
    };

    let redacted = match pattern_opt {
        Some(pattern) => {
            let replacement = format!("[REDACTED:{}]", args.r#type);
            if args.r#type == "private-key" {
                // Special handling for private key blocks
                redact_private_key_blocks(&s)
            } else {
                redact_regex(&s, &pattern, &replacement)
            }
        }
        None => {
            eprintln!("Unknown secret type: {}", args.r#type);
            print_scalar("null");
            return Ok(());
        }
    };

    if fs::write(path, redacted).is_err() {
        print_scalar("null");
        return Ok(());
    }
    print_scalar("ok");
    Ok(())
}

fn redact_regex(content: &str, pat: &str, repl: &str) -> String {
    match Regex::new(pat) {
        Ok(re) => re.replace_all(content, repl).to_string(),
        Err(_) => content.to_string(),
    }
}

/// Deterministic, line-based replacement of PEM blocks.
/// No regex backtracking required.
fn redact_private_key_blocks(content: &str) -> String {
    let mut out = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        if !in_block {
            if line.contains("-----BEGIN") && line.contains("PRIVATE KEY-----") {
                in_block = true;
                out.push("[REDACTED:private-key]".to_string());
                continue;
            }
            out.push(line.to_string());
        } else {
            // Skip until end marker
            if line.contains("-----END") && line.contains("PRIVATE KEY-----") {
                in_block = false;
            }
        }
    }

    let mut joined = out.join("\n");
    joined.push('\n');
    joined
}
