//! Structure checks: required files exist, no duplicates.
//!
//! Checks: 1, 2, 6, 9, 10, 15

use std::collections::HashMap;

use crate::reporter::Reporter;
use crate::util::extract_frontmatter_name;

use super::{CheckCtx, CheckSpec};

pub fn checks() -> Vec<CheckSpec> {
    vec![
        CheckSpec {
            id: 1,
            title: "Checking required agents...",
            run: check_required_agents,
        },
        CheckSpec {
            id: 2,
            title: "Checking flow commands...",
            run: check_flow_commands,
        },
        CheckSpec {
            id: 6,
            title: "Checking for duplicate agent names...",
            run: check_duplicate_agents,
        },
        CheckSpec {
            id: 9,
            title: "Checking skills...",
            run: check_skills,
        },
        CheckSpec {
            id: 10,
            title: "Checking CLAUDE.md...",
            run: check_claude_md,
        },
        CheckSpec {
            id: 15,
            title: "Checking customizer command...",
            run: check_customizer,
        },
    ]
}

/// Check 1: Required agents exist.
fn check_required_agents(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.required_agents {
        if cx.inv.agent(agent).is_some() {
            rep.pass(format!("{agent}.md exists"));
        } else {
            rep.fail(format!("{agent}.md MISSING"));
        }
    }

    Ok(())
}

/// Check 2: Required flow commands exist.
fn check_flow_commands(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for i in 1..=6 {
        let prefix = format!("flow-{i}-");
        let exists = cx.inv.command_md_files.iter().any(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|name| name.starts_with(&prefix) && name.ends_with(".md"))
        });

        if exists {
            rep.pass(format!("flow-{i} command exists"));
        } else {
            rep.fail(format!("flow-{i} command MISSING"));
        }
    }

    Ok(())
}

/// Check 6: No duplicate agent names.
fn check_duplicate_agents(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut agent_names: HashMap<String, String> = HashMap::new();
    let mut duplicates_found = false;

    for agent_file in &cx.inv.agent_md_files {
        let content = cx.ctx.read_utf8(agent_file)?;
        let Some(name) = extract_frontmatter_name(&content) else {
            continue;
        };

        let rel = cx.ctx.rel(agent_file);
        if let Some(prev) = agent_names.get(&name) {
            rep.fail(format!("Duplicate agent name '{name}' in:"));
            rep.indent_lines([prev.clone(), rel.clone()]);
            duplicates_found = true;
        } else {
            agent_names.insert(name, rel);
        }
    }

    if !duplicates_found {
        rep.pass("No duplicate agent names found");
    }

    Ok(())
}

/// Check 9: Required skills exist.
fn check_skills(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for skill in cx.c.required_skills {
        let path = cx.ctx.skills_dir.join(skill).join("SKILL.md");
        if path.is_file() {
            rep.pass(format!("{skill} skill exists"));
        } else {
            rep.fail(format!("{skill} skill MISSING"));
        }
    }

    Ok(())
}

/// Check 10: CLAUDE.md exists and has key sections.
fn check_claude_md(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let claude_md = cx.ctx.repo_root.join("CLAUDE.md");
    if claude_md.is_file() {
        rep.pass("CLAUDE.md exists");

        let content = cx.ctx.read_utf8(&claude_md)?;
        for section in cx.c.claude_md_sections {
            if content.contains(section) {
                rep.pass(format!("CLAUDE.md documents '{section}'"));
            } else {
                rep.warn(format!("CLAUDE.md missing documentation for '{section}'"));
            }
        }
    } else {
        rep.fail("CLAUDE.md MISSING");
    }

    Ok(())
}

/// Check 15: Customizer command exists.
fn check_customizer(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let customize_cmd = cx.ctx.commands_dir.join("customize-pack.md");
    if customize_cmd.is_file() {
        rep.pass("customize-pack command exists");
    } else {
        rep.warn("customize-pack command MISSING (optional but recommended)");
    }

    let pack_customizer = cx.ctx.agents_dir.join("pack-customizer.md");
    if pack_customizer.is_file() {
        rep.pass("pack-customizer agent exists");
    } else {
        rep.warn("pack-customizer agent MISSING (optional but recommended)");
    }

    Ok(())
}
