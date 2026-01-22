//! Shared helpers for commands.

use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Args;
use serde_json::Value;
use tempfile::NamedTempFile;

/// Compatibility flag accepted by read-ish commands to avoid clap errors when
/// callers pass `--null-if-missing`. Semantics are handled upstream; we accept
/// the flag to keep the interface stable.
#[derive(Args, Debug, Default, Clone)]
pub struct CompatNullIfMissing {
    /// Optional compatibility flag; semantics are already null-safe.
    #[arg(long, default_value_t = false)]
    pub null_if_missing: bool,
}

/// Write JSON to a file atomically using a temporary file and rename.
///
/// This ensures that readers never see a partially-written file:
/// 1. Create a temporary file in the same directory as the target
/// 2. Write the JSON content to the temporary file
/// 3. Atomically rename the temporary file to the target path
///
/// The temporary file is created in the same directory to ensure
/// the rename operation is atomic (same filesystem).
pub fn write_json_atomic(path: &Path, v: &Value) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    // Create a temporary file in the same directory as the target
    // This ensures they're on the same filesystem for atomic rename
    let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut tmp = NamedTempFile::new_in(parent_dir)?;

    // Write the JSON content to the temporary file
    tmp.write_all(format!("{}\n", serde_json::to_string_pretty(v)?).as_bytes())?;

    // Persist the temporary file to the target path
    // This is an atomic operation that replaces the target if it exists
    tmp.persist(path)?;

    Ok(())
}
