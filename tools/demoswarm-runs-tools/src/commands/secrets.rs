//! Secret scanning and redaction commands.
//!
//! Security note: This module NEVER prints secret content to stdout.
//! - `scan` writes findings (file/line/type only) to JSON
//! - `redact` modifies files in-place, prints only status

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::common::write_json_atomic;
use crate::output::print_scalar;
use crate::walk::{walk_dir_excluding_verbose, SkippedItem};

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
    let root = Path::new(&args.path);
    let out_path = Path::new(&args.output);
    let verbose = args.verbose;

    if !root.exists() {
        let v = json!({ "status": "SCAN_PATH_MISSING", "findings": [], "skipped_count": 0 });
        write_json_atomic(out_path, &v).map_err(|e| {
            eprintln!(
                "Warning: failed to write secrets findings JSON ({}): {e:#}",
                out_path.display()
            );
            e
        })?;
        print_scalar("SCAN_PATH_MISSING");
        return Ok(());
    }

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
            write_json_atomic(out_path, &v).map_err(|write_e| {
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
    let mut walker = walk_dir_excluding_verbose(root, EXCLUDED_DIRS, verbose);

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
    write_json_atomic(out_path, &v).map_err(|e| {
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
    let path = Path::new(&args.file);
    if !path.is_file() {
        print_scalar("FILE_NOT_FOUND");
        return Ok(());
    }

    let bytes = match fs::read(path) {
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
