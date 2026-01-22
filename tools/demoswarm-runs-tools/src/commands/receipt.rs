//! Receipt field reading with git fallback.
//!
//! Implements the receipt discovery protocol:
//! 1. Try direct file read
//! 2. If that fails, try `git show HEAD:<path>`
//! 3. Return null if both fail

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use clap::{Args, Subcommand};
use serde_json::Value;

use super::common::CompatNullIfMissing;
use crate::output::{print_null, print_scalar};

#[derive(Args, Debug)]
pub struct ReceiptCommand {
    #[command(subcommand)]
    pub command: ReceiptSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ReceiptSubcommand {
    /// Read a field from a receipt JSON file
    Get {
        /// Receipt JSON file
        #[arg(long)]
        file: String,

        /// Top-level key to extract
        #[arg(long)]
        key: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: ReceiptCommand) -> Result<()> {
    match cmd.command {
        ReceiptSubcommand::Get { file, key, .. } => read_receipt_field(&file, &key),
    }
}

/// Discovery method used to read the receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiscoveryMethod {
    DirectRead,
    GitShow,
}

/// Try to read file content via git show HEAD:<path>.
fn try_git_show(path: &str) -> Option<String> {
    // Normalize path separators to forward slashes for git
    let git_path = path.replace('\\', "/");

    let output = Command::new("git")
        .args(["show", &format!("HEAD:{}", git_path)])
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}

/// Read receipt content using the discovery protocol:
/// 1. Try direct file read
/// 2. If that fails, try git show HEAD:<path>
/// Returns (content, method) or None if both fail.
fn discover_receipt_content(file: &str) -> Option<(String, DiscoveryMethod)> {
    let path = Path::new(file);

    // Try direct file read first
    if path.is_file() {
        if let Ok(content) = fs::read_to_string(path) {
            return Some((content, DiscoveryMethod::DirectRead));
        }
    }

    // Try git fallback
    if let Some(content) = try_git_show(file) {
        return Some((content, DiscoveryMethod::GitShow));
    }

    None
}

fn read_receipt_field(file: &str, key: &str) -> Result<()> {
    let (content, method) = match discover_receipt_content(file) {
        Some(result) => result,
        None => {
            // Log discovery failure to stderr for structured output
            eprintln!("discovery_method: missing");
            print_null();
            return Ok(());
        }
    };

    // Log which method succeeded
    let method_str = match method {
        DiscoveryMethod::DirectRead => "direct_read",
        DiscoveryMethod::GitShow => "git_show",
    };
    eprintln!("discovery_method: {}", method_str);

    let json: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    let value = match json.get(key) {
        Some(v) => v,
        None => {
            print_null();
            return Ok(());
        }
    };

    // Print scalar values only
    match value {
        Value::String(s) => print_scalar(s),
        Value::Number(n) => print_scalar(n),
        Value::Bool(b) => print_scalar(b),
        Value::Null => print_null(),
        _ => print_null(), // Arrays/objects not supported
    }

    Ok(())
}
