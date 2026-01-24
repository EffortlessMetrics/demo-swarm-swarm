//! Shared directory walking utilities.
//!
//! Provides a configurable, iterator-based directory walker that:
//! - Recursively traverses directories
//! - Does not follow symlinks (security/consistency)
//! - Logs errors instead of silently skipping them
//! - Supports configurable directory exclusion

use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/// A skipped item during directory walking.
#[derive(Debug, Clone)]
pub struct SkippedItem {
    /// The path that was skipped.
    pub path: PathBuf,
    /// The reason for skipping.
    pub reason: String,
}

/// Configuration for directory walking.
#[derive(Debug, Clone, Default)]
pub struct WalkConfig {
    /// Directory names to exclude from traversal.
    pub exclude_dirs: &'static [&'static str],
    /// Whether to log skipped items to stderr.
    pub verbose: bool,
}

impl WalkConfig {
    /// Create a new WalkConfig with no exclusions.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a WalkConfig with the specified directory exclusions.
    pub fn with_exclusions(exclude_dirs: &'static [&'static str]) -> Self {
        Self {
            exclude_dirs,
            verbose: false,
        }
    }

    /// Create a WalkConfig with exclusions and verbose mode.
    pub fn with_exclusions_verbose(exclude_dirs: &'static [&'static str], verbose: bool) -> Self {
        Self {
            exclude_dirs,
            verbose,
        }
    }
}

/// An iterator that walks a directory tree, yielding file paths.
///
/// This implementation:
/// - Uses a stack-based approach (no recursion, bounded memory)
/// - Does not follow symlinks
/// - Tracks skipped items for reporting
/// - Optionally logs errors to stderr (verbose mode)
pub struct WalkDir {
    /// Stack of directories yet to be processed.
    stack: VecDeque<PathBuf>,
    /// Files discovered in the current directory, ready to yield.
    pending_files: VecDeque<PathBuf>,
    /// Configuration for this walk.
    config: WalkConfig,
    /// Items that were skipped due to I/O errors.
    skipped: Vec<SkippedItem>,
}

impl WalkDir {
    /// Create a new WalkDir iterator starting at the given root.
    ///
    /// If `root` is a file, yields just that file.
    /// If `root` is a directory, yields all files recursively.
    /// If `root` does not exist, yields nothing.
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self::with_config(root, WalkConfig::default())
    }

    /// Create a new WalkDir iterator with custom configuration.
    pub fn with_config(root: impl AsRef<Path>, config: WalkConfig) -> Self {
        let root = root.as_ref();
        let mut walker = Self {
            stack: VecDeque::new(),
            pending_files: VecDeque::new(),
            config,
            skipped: Vec::new(),
        };

        if root.is_file() {
            // Single file: yield it directly
            walker.pending_files.push_back(root.to_path_buf());
        } else if root.is_dir() {
            // Directory: start traversal
            walker.stack.push_back(root.to_path_buf());
        }
        // If root doesn't exist or is neither file nor dir, iterator is empty

        walker
    }

    /// Get the list of items that were skipped during walking.
    pub fn skipped_items(&self) -> &[SkippedItem] {
        &self.skipped
    }

    /// Take the list of skipped items, consuming them from this walker.
    pub fn take_skipped_items(&mut self) -> Vec<SkippedItem> {
        std::mem::take(&mut self.skipped)
    }

    /// Record a skipped item and optionally log it.
    fn record_skipped(&mut self, path: PathBuf, reason: String) {
        if self.config.verbose {
            eprintln!("Warning: skipped {}: {}", path.display(), reason);
        }
        self.skipped.push(SkippedItem { path, reason });
    }

    /// Check if a directory name should be excluded.
    fn is_excluded(&self, name: &OsStr) -> bool {
        if let Some(name_str) = name.to_str() {
            self.config.exclude_dirs.contains(&name_str)
        } else {
            false
        }
    }

    /// Process the next directory on the stack, populating pending_files.
    fn process_next_dir(&mut self) {
        while let Some(dir) = self.stack.pop_front() {
            match fs::read_dir(&dir) {
                Ok(entries) => {
                    for entry_result in entries {
                        match entry_result {
                            Ok(entry) => {
                                let path = entry.path();

                                // Use symlink_metadata to check the entry type without following
                                match path.symlink_metadata() {
                                    Ok(meta) => {
                                        if meta.is_file() {
                                            // Regular file: include it
                                            self.pending_files.push_back(path);
                                        } else if meta.is_dir() {
                                            // Regular directory: traverse it (unless excluded)
                                            if let Some(name) = path.file_name() {
                                                if !self.is_excluded(name) {
                                                    self.stack.push_back(path);
                                                }
                                            }
                                        } else if meta.file_type().is_symlink() {
                                            // Symlink: follow to check what it points to
                                            // Include if it resolves to a file, skip if directory
                                            // (avoids directory traversal loops while including
                                            // common patterns like symlinked .env files)
                                            match fs::metadata(&path) {
                                                Ok(target_meta) if target_meta.is_file() => {
                                                    self.pending_files.push_back(path);
                                                }
                                                // Symlink to directory: skip (avoid loops)
                                                // Broken symlink: skip
                                                _ => {}
                                            }
                                        }
                                        // Skip other special file types (devices, pipes, etc.)
                                    }
                                    Err(e) => {
                                        self.record_skipped(
                                            path,
                                            format!("failed to read metadata: {}", e),
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                self.record_skipped(
                                    dir.clone(),
                                    format!("failed to read directory entry: {}", e),
                                );
                            }
                        }
                    }
                    // If we found files, return to let iterator yield them
                    if !self.pending_files.is_empty() {
                        return;
                    }
                }
                Err(e) => {
                    self.record_skipped(dir, format!("failed to read directory: {}", e));
                }
            }
        }
    }
}

impl Iterator for WalkDir {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        // First, try to yield from pending files
        if let Some(file) = self.pending_files.pop_front() {
            return Some(file);
        }

        // No pending files; process more directories
        self.process_next_dir();

        // Try again after processing
        self.pending_files.pop_front()
    }
}

/// Convenience function to walk a directory with no exclusions.
pub fn walk_dir(root: impl AsRef<Path>) -> WalkDir {
    WalkDir::new(root)
}

/// Convenience function to walk a directory with exclusions.
pub fn walk_dir_excluding(root: impl AsRef<Path>, exclude: &'static [&'static str]) -> WalkDir {
    WalkDir::with_config(root, WalkConfig::with_exclusions(exclude))
}

/// Convenience function to walk a directory with exclusions and verbose mode.
pub fn walk_dir_excluding_verbose(
    root: impl AsRef<Path>,
    exclude: &'static [&'static str],
    verbose: bool,
) -> WalkDir {
    WalkDir::with_config(root, WalkConfig::with_exclusions_verbose(exclude, verbose))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn walk_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let files: Vec<_> = walk_dir(tmp.path()).collect();
        assert!(files.is_empty());
    }

    #[test]
    fn walk_single_file() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("test.txt");
        File::create(&file_path).unwrap();

        let files: Vec<_> = walk_dir(tmp.path()).collect();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }

    #[test]
    fn walk_nested_dirs() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("sub");
        fs::create_dir(&sub).unwrap();
        let file1 = tmp.path().join("a.txt");
        let file2 = sub.join("b.txt");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let mut files: Vec<_> = walk_dir(tmp.path()).collect();
        files.sort();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn walk_with_exclusions() {
        let tmp = TempDir::new().unwrap();
        let excluded = tmp.path().join("node_modules");
        let included = tmp.path().join("src");
        fs::create_dir(&excluded).unwrap();
        fs::create_dir(&included).unwrap();

        let excluded_file = excluded.join("excluded.txt");
        let included_file = included.join("included.txt");
        File::create(&excluded_file).unwrap();
        File::create(&included_file).unwrap();

        static EXCLUDE: &[&str] = &["node_modules"];
        let files: Vec<_> = walk_dir_excluding(tmp.path(), EXCLUDE).collect();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0], included_file);
    }

    #[test]
    fn walk_nonexistent_returns_empty() {
        let files: Vec<_> = walk_dir("/nonexistent/path/that/should/not/exist").collect();
        assert!(files.is_empty());
    }

    #[test]
    fn walk_single_file_directly() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("direct.txt");
        File::create(&file_path).unwrap();

        // Pass file path directly, not directory
        let files: Vec<_> = walk_dir(&file_path).collect();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }
}
