use std::path::{Path, PathBuf};

use regex::Regex;
use walkdir::WalkDir;

use crate::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct LineMatch {
    pub path: PathBuf,
    pub line_no: usize, // 1-based
    pub line: String,
}

pub fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

pub fn has_exact_line(content: &str, exact: &str) -> bool {
    content.lines().any(|l| l.trim_end() == exact)
}

pub fn has_line_starting_with(content: &str, prefix: &str) -> bool {
    content.lines().any(|l| l.starts_with(prefix))
}

pub fn find_matches_regex_recursive(
    ctx: &Ctx,
    roots: &[PathBuf],
    re: &Regex,
    ignore_file_names: &[&str],
) -> anyhow::Result<Vec<LineMatch>> {
    let mut out = Vec::new();

    for root in roots {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(root).follow_links(false) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if should_ignore(path, ignore_file_names) {
                continue;
            }

            // Best-effort: skip unreadable/non-UTF8 files.
            let content = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            for (i, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    out.push(LineMatch {
                        path: path.to_path_buf(),
                        line_no: i + 1,
                        line: line.to_string(),
                    });
                }
            }
        }
    }

    // Stable ordering (roughly grep-like)
    out.sort_by(|a, b| {
        ctx.rel(&a.path)
            .cmp(&ctx.rel(&b.path))
            .then(a.line_no.cmp(&b.line_no))
    });

    Ok(out)
}

pub fn find_files_containing_recursive(
    ctx: &Ctx,
    root: &Path,
    needle: &str,
    ignore_file_names: &[&str],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut hits = Vec::new();

    if !root.exists() {
        return Ok(hits);
    }

    for entry in WalkDir::new(root).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if should_ignore(path, ignore_file_names) {
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if content.contains(needle) {
            hits.push(path.to_path_buf());
        }
    }

    hits.sort_by_key(|a| ctx.rel(a));
    Ok(hits)
}

pub fn extract_frontmatter_name(content: &str) -> Option<String> {
    // Mimics:
    //   sed -n '/^---$/,/^---$/p' file | grep '^name:' | head -1

    let mut lines = content.lines();

    // Find first '---' line
    for line in lines.by_ref() {
        if line.trim_end() == "---" {
            break;
        }
    }

    for line in lines {
        if line.trim_end() == "---" {
            break;
        }
        if let Some(rest) = line.strip_prefix("name:") {
            let name = rest.trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
            break;
        }
    }

    None
}

fn should_ignore(path: &Path, ignore_file_names: &[&str]) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    ignore_file_names.contains(&name)
}
