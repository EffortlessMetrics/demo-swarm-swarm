use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, bail};

#[derive(Debug)]
pub struct Ctx {
    pub repo_root: PathBuf,
    pub claude_dir: PathBuf,
    pub agents_dir: PathBuf,
    pub commands_dir: PathBuf,
    pub skills_dir: PathBuf,
    cache: RefCell<HashMap<PathBuf, Arc<str>>>,
}

impl Ctx {
    pub fn discover(repo_root_arg: Option<PathBuf>) -> anyhow::Result<Self> {
        let repo_root = match repo_root_arg {
            Some(p) => p,
            None => Self::find_repo_root_from_cwd()?,
        };
        let repo_root = repo_root.canonicalize().with_context(|| {
            format!("Failed to canonicalize repo root: {}", repo_root.display())
        })?;

        let claude_dir = repo_root.join(".claude");
        if !claude_dir.is_dir() {
            bail!(
                "Repo root does not contain a .claude/ directory: {}",
                repo_root.display()
            );
        }

        Ok(Self {
            agents_dir: claude_dir.join("agents"),
            commands_dir: claude_dir.join("commands"),
            skills_dir: claude_dir.join("skills"),
            repo_root,
            claude_dir,
            cache: RefCell::new(HashMap::new()),
        })
    }

    fn find_repo_root_from_cwd() -> anyhow::Result<PathBuf> {
        let mut dir = std::env::current_dir().context("Failed to get current directory")?;

        loop {
            // Case 1: we're at repo root
            if dir.join(".claude").is_dir() {
                return Ok(dir);
            }

            // Case 2: we're inside .claude itself
            if dir.file_name().is_some_and(|n| n == ".claude")
                && let Some(parent) = dir.parent()
            {
                return Ok(parent.to_path_buf());
            }

            if !dir.pop() {
                break;
            }
        }

        bail!(
            "Could not find a .claude/ directory by walking up from the current working directory"
        )
    }

    pub fn read_utf8(&self, path: &Path) -> anyhow::Result<Arc<str>> {
        if let Some(hit) = self.cache.borrow().get(path).cloned() {
            return Ok(hit);
        }

        let s = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file as UTF-8: {}", path.display()))?;
        let arc: Arc<str> = Arc::from(s);
        self.cache
            .borrow_mut()
            .insert(path.to_path_buf(), arc.clone());
        Ok(arc)
    }

    pub fn rel(&self, path: &Path) -> String {
        path.strip_prefix(&self.repo_root)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path.display().to_string())
    }

    pub fn agent_md_files(&self) -> anyhow::Result<Vec<PathBuf>> {
        list_md_files(&self.agents_dir)
    }

    pub fn command_md_files(&self) -> anyhow::Result<Vec<PathBuf>> {
        list_md_files(&self.commands_dir)
    }
}

fn list_md_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
    out.sort();
    Ok(out)
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // -------------------------------------------------------------------------
    // Ctx::discover tests
    // -------------------------------------------------------------------------

    /// Test that discover() succeeds when given a valid repo root with .claude dir.
    #[test]
    fn test_discover_with_explicit_repo_root() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        assert_eq!(ctx.claude_dir.file_name().unwrap(), ".claude");
        assert_eq!(ctx.agents_dir.file_name().unwrap(), "agents");
        assert_eq!(ctx.commands_dir.file_name().unwrap(), "commands");
        assert_eq!(ctx.skills_dir.file_name().unwrap(), "skills");
    }

    /// Test that discover() fails when .claude dir is missing.
    /// Covers line 32: bail when .claude dir doesn't exist.
    #[test]
    fn test_discover_fails_without_claude_dir() {
        let tmp = TempDir::new().unwrap();
        // No .claude directory created

        let result = Ctx::discover(Some(tmp.path().to_path_buf()));

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains(".claude"),
            "Error should mention .claude directory: {}",
            err
        );
    }

    /// Test that discover() fails when repo root cannot be canonicalized.
    /// Covers line 27: canonicalize error formatting.
    #[test]
    fn test_discover_fails_on_nonexistent_path() {
        let nonexistent = PathBuf::from("/definitely/not/a/real/path/xyz123");

        let result = Ctx::discover(Some(nonexistent));

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("canonicalize") || err.contains("repo root"),
            "Error should mention canonicalize failure: {}",
            err
        );
    }

    // -------------------------------------------------------------------------
    // Ctx::read_utf8 tests
    // -------------------------------------------------------------------------

    /// Test that read_utf8() returns cached content on subsequent calls.
    #[test]
    fn test_read_utf8_caches_content() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let test_file = tmp.path().join("test.md");
        std::fs::write(&test_file, "# Test Content").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        // First read
        let content1 = ctx.read_utf8(&test_file).unwrap();
        assert_eq!(&*content1, "# Test Content");

        // Second read should return same Arc (cached)
        let content2 = ctx.read_utf8(&test_file).unwrap();
        assert!(
            Arc::ptr_eq(&content1, &content2),
            "Should return cached Arc"
        );
    }

    /// Test that read_utf8() fails on nonexistent file.
    #[test]
    fn test_read_utf8_fails_on_missing_file() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let missing_file = tmp.path().join("does_not_exist.md");
        let result = ctx.read_utf8(&missing_file);

        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Ctx::rel tests
    // -------------------------------------------------------------------------

    /// Test that rel() returns relative path when file is under repo root.
    #[test]
    fn test_rel_returns_relative_path() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let file_path = ctx.repo_root.join("subdir").join("file.md");
        let rel = ctx.rel(&file_path);

        // On Windows, path separator might differ
        assert!(
            rel.contains("subdir") && rel.contains("file.md"),
            "Relative path should contain subdir/file.md: {}",
            rel
        );
    }

    /// Test that rel() returns full path when file is outside repo root.
    #[test]
    fn test_rel_returns_full_path_for_external() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        // Create a path that's definitely outside repo root
        let external_path = PathBuf::from("/some/external/path/file.md");
        let rel = ctx.rel(&external_path);

        // Should return the full path since it can't be stripped
        assert!(
            rel.contains("external") || rel.contains("file.md"),
            "Should return full path: {}",
            rel
        );
    }

    // -------------------------------------------------------------------------
    // list_md_files tests
    // -------------------------------------------------------------------------

    /// Test that list_md_files() returns empty vec for non-existent directory.
    /// Covers line 106: when directory doesn't exist.
    #[test]
    fn test_list_md_files_nonexistent_dir() {
        let nonexistent = PathBuf::from("/definitely/not/a/real/dir/xyz789");

        let result = list_md_files(&nonexistent).unwrap();

        assert!(
            result.is_empty(),
            "Should return empty vec for nonexistent dir"
        );
    }

    /// Test that list_md_files() returns only .md files.
    #[test]
    fn test_list_md_files_filters_by_extension() {
        let tmp = TempDir::new().unwrap();

        // Create various files
        std::fs::write(tmp.path().join("file1.md"), "markdown").unwrap();
        std::fs::write(tmp.path().join("file2.md"), "markdown").unwrap();
        std::fs::write(tmp.path().join("file3.txt"), "text").unwrap();
        std::fs::write(tmp.path().join("file4.rs"), "rust").unwrap();

        let result = list_md_files(tmp.path()).unwrap();

        assert_eq!(result.len(), 2, "Should only return .md files");
        for path in &result {
            assert_eq!(
                path.extension().unwrap(),
                "md",
                "All files should have .md extension"
            );
        }
    }

    /// Test that list_md_files() returns sorted results.
    #[test]
    fn test_list_md_files_returns_sorted() {
        let tmp = TempDir::new().unwrap();

        // Create files in non-alphabetical order
        std::fs::write(tmp.path().join("z_file.md"), "").unwrap();
        std::fs::write(tmp.path().join("a_file.md"), "").unwrap();
        std::fs::write(tmp.path().join("m_file.md"), "").unwrap();

        let result = list_md_files(tmp.path()).unwrap();

        assert_eq!(result.len(), 3);
        assert!(
            result[0].file_name().unwrap().to_str().unwrap()
                < result[1].file_name().unwrap().to_str().unwrap(),
            "Results should be sorted"
        );
        assert!(
            result[1].file_name().unwrap().to_str().unwrap()
                < result[2].file_name().unwrap().to_str().unwrap(),
            "Results should be sorted"
        );
    }

    /// Test that list_md_files() ignores subdirectories.
    #[test]
    fn test_list_md_files_ignores_dirs() {
        let tmp = TempDir::new().unwrap();

        std::fs::write(tmp.path().join("file.md"), "content").unwrap();
        std::fs::create_dir(tmp.path().join("subdir.md")).unwrap(); // A directory with .md name

        let result = list_md_files(tmp.path()).unwrap();

        assert_eq!(result.len(), 1, "Should only return files, not directories");
        assert!(result[0].is_file());
    }

    // -------------------------------------------------------------------------
    // agent_md_files and command_md_files tests
    // -------------------------------------------------------------------------

    /// Test agent_md_files returns files from agents directory.
    #[test]
    fn test_agent_md_files() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        let agents_dir = claude_dir.join("agents");
        std::fs::create_dir_all(&agents_dir).unwrap();

        std::fs::write(agents_dir.join("agent1.md"), "agent").unwrap();
        std::fs::write(agents_dir.join("agent2.md"), "agent").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let result = ctx.agent_md_files().unwrap();

        assert_eq!(result.len(), 2);
    }

    /// Test command_md_files returns files from commands directory.
    #[test]
    fn test_command_md_files() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        let commands_dir = claude_dir.join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        std::fs::write(commands_dir.join("flow-1-signal.md"), "command").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let result = ctx.command_md_files().unwrap();

        assert_eq!(result.len(), 1);
    }

    /// Test agent_md_files returns empty when agents dir doesn't exist.
    #[test]
    fn test_agent_md_files_empty_when_missing() {
        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();
        // Don't create agents dir

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let result = ctx.agent_md_files().unwrap();

        assert!(result.is_empty());
    }
}
