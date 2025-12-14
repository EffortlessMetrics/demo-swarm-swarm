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
