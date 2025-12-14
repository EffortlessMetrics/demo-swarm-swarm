//! Precomputed file inventories.
//!
//! Build once at startup, share with all checks.

use std::collections::HashMap;
use std::path::PathBuf;

use walkdir::WalkDir;

use crate::ctx::Ctx;

/// Precomputed file lists for the pack.
#[derive(Debug)]
pub struct Inventory {
    /// All `.md` files in `.claude/agents/`.
    pub agent_md_files: Vec<PathBuf>,

    /// All `.md` files in `.claude/commands/`.
    pub command_md_files: Vec<PathBuf>,

    /// Flow command files (flow-*.md) in `.claude/commands/`.
    pub flow_cmd_files: Vec<PathBuf>,

    /// All `SKILL.md` files in `.claude/skills/` (recursive).
    pub skill_md_files: Vec<PathBuf>,

    /// Agent files indexed by stem (e.g., "work-planner" -> path).
    pub agents_by_stem: HashMap<String, PathBuf>,

    /// Command files indexed by stem (e.g., "flow-1-signal" -> path).
    pub commands_by_stem: HashMap<String, PathBuf>,
}

impl Inventory {
    /// Build the inventory from a Ctx.
    pub fn from_ctx(ctx: &Ctx) -> anyhow::Result<Self> {
        let agent_md_files = ctx.agent_md_files()?;
        let command_md_files = ctx.command_md_files()?;

        // Skills are in subdirectories: .claude/skills/<name>/SKILL.md
        let skill_md_files = list_skill_files(&ctx.skills_dir);

        // Filter flow commands
        let flow_cmd_files: Vec<_> = command_md_files
            .iter()
            .filter(|p| {
                p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|name| name.starts_with("flow-") && name.ends_with(".md"))
            })
            .cloned()
            .collect();

        // Build stem indexes
        let agents_by_stem = build_stem_index(&agent_md_files);
        let commands_by_stem = build_stem_index(&command_md_files);

        Ok(Self {
            agent_md_files,
            command_md_files,
            flow_cmd_files,
            skill_md_files,
            agents_by_stem,
            commands_by_stem,
        })
    }

    /// Get agent path by stem name.
    #[inline]
    pub fn agent(&self, stem: &str) -> Option<&PathBuf> {
        self.agents_by_stem.get(stem)
    }

    /// Get command path by stem name.
    #[inline]
    pub fn command(&self, stem: &str) -> Option<&PathBuf> {
        self.commands_by_stem.get(stem)
    }
}

/// Build a HashMap from file stem to path.
fn build_stem_index(files: &[PathBuf]) -> HashMap<String, PathBuf> {
    files
        .iter()
        .filter_map(|p| {
            p.file_stem()
                .and_then(|s| s.to_str())
                .map(|stem| (stem.to_string(), p.clone()))
        })
        .collect()
}

/// List all SKILL.md files under the skills directory.
fn list_skill_files(skills_dir: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if !skills_dir.is_dir() {
        return out;
    }

    for entry in WalkDir::new(skills_dir).follow_links(false) {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.is_file() && path.file_name().is_some_and(|n| n == "SKILL.md") {
            out.push(path.to_path_buf());
        }
    }
    out.sort();
    out
}
