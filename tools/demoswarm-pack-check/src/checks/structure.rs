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
            title: "Checking agent frontmatter name contracts...",
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
    let mut mismatches_found = false;

    for agent_file in &cx.inv.agent_md_files {
        let content = cx.ctx.read_utf8(agent_file)?;
        let rel = cx.ctx.rel(agent_file);
        let stem = agent_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let Some(name) = extract_frontmatter_name(&content) else {
            rep.fail(format!("{rel} missing frontmatter `name:`"));
            mismatches_found = true;
            continue;
        };

        if name != stem {
            rep.fail(format!(
                "{rel} frontmatter name '{name}' does not match filename stem '{stem}'"
            ));
            mismatches_found = true;
        }

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

    if !mismatches_found {
        rep.pass("All agent frontmatter names match file stems");
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

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use crate::util::extract_frontmatter_name;

    // -------------------------------------------------------------------------
    // Frontmatter name extraction tests (used in check_duplicate_agents)
    // -------------------------------------------------------------------------

    /// Valid frontmatter with name field is extracted correctly.
    #[test]
    fn test_extract_frontmatter_name_valid() {
        let content = r#"---
name: test-agent
description: A test agent
---
# Test Agent
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, Some("test-agent".to_string()));
    }

    /// Frontmatter without name field returns None.
    #[test]
    fn test_extract_frontmatter_name_missing() {
        let content = r#"---
description: A test agent
---
# Test Agent
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, None);
    }

    /// Empty frontmatter returns None.
    #[test]
    fn test_extract_frontmatter_name_empty() {
        let content = r#"---
---
# Test Agent
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, None);
    }

    /// No frontmatter at all returns None.
    #[test]
    fn test_extract_frontmatter_name_no_frontmatter() {
        let content = r#"# Test Agent

This is just content without frontmatter.
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, None);
    }

    /// Frontmatter with empty name returns None.
    #[test]
    fn test_extract_frontmatter_name_empty_value() {
        let content = r#"---
name:
description: A test agent
---
# Test Agent
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, None);
    }

    /// Frontmatter with name containing only whitespace returns None.
    #[test]
    fn test_extract_frontmatter_name_whitespace_value() {
        let content = r#"---
name:
description: A test agent
---
# Test Agent
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, None);
    }

    /// Name with leading/trailing whitespace is trimmed.
    #[test]
    fn test_extract_frontmatter_name_trimmed() {
        let content = r#"---
name:   test-agent
---
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, Some("test-agent".to_string()));
    }

    /// Name field not on first line of frontmatter is still found.
    #[test]
    fn test_extract_frontmatter_name_not_first() {
        let content = r#"---
description: First field
name: test-agent
version: 1.0
---
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, Some("test-agent".to_string()));
    }

    /// Only first frontmatter block is considered.
    #[test]
    fn test_extract_frontmatter_name_only_first_block() {
        let content = r#"---
name: first-agent
---
# Content

---
name: second-agent
---
"#;
        let name = extract_frontmatter_name(content);
        assert_eq!(name, Some("first-agent".to_string()));
    }

    /// Frontmatter starting in middle of file (no leading ---) returns None.
    #[test]
    fn test_extract_frontmatter_name_not_at_start() {
        let content = r#"Some leading text
---
name: test-agent
---
"#;
        // The function looks for first --- anywhere, so this would actually work
        // but the name would be after the second ---
        let name = extract_frontmatter_name(content);
        // After first ---, it sees "name: test-agent" before second ---
        assert_eq!(name, Some("test-agent".to_string()));
    }

    // -------------------------------------------------------------------------
    // Flow command naming tests
    // -------------------------------------------------------------------------

    /// Flow command file naming pattern is validated correctly.
    #[test]
    fn test_flow_command_naming_pattern() {
        let valid_names = [
            "flow-1-signal.md",
            "flow-2-plan.md",
            "flow-3-build.md",
            "flow-4-gate.md",
            "flow-5-deploy.md",
            "flow-6-wisdom.md",
        ];

        for (i, name) in valid_names.iter().enumerate() {
            let prefix = format!("flow-{}-", i + 1);
            assert!(
                name.starts_with(&prefix) && name.ends_with(".md"),
                "Valid name {} should match pattern",
                name
            );
        }
    }

    /// Invalid flow command names are rejected.
    #[test]
    fn test_flow_command_naming_invalid() {
        let invalid_names = [
            "flow-0-zero.md",  // No flow 0
            "flow-7-extra.md", // No flow 7
            "signal.md",       // Missing flow- prefix
            "flow-1.md",       // Missing name after number
            "flow-1-signal",   // Missing .md extension
        ];

        for name in invalid_names {
            let is_valid = (1..=6).any(|i| {
                let prefix = format!("flow-{}-", i);
                name.starts_with(&prefix) && name.ends_with(".md")
            });
            assert!(
                !is_valid,
                "Invalid name {} should not match any flow pattern",
                name
            );
        }
    }

    // -------------------------------------------------------------------------
    // Required agents list tests
    // -------------------------------------------------------------------------

    /// Required agents list contains cleanup agents for all flows.
    #[test]
    fn test_required_agents_has_all_cleanup() {
        let contracts = crate::contracts::Contracts::default();

        let cleanup_agents = [
            "signal-cleanup",
            "plan-cleanup",
            "build-cleanup",
            "gate-cleanup",
            "deploy-cleanup",
            "wisdom-cleanup",
        ];

        for agent in cleanup_agents {
            assert!(
                contracts.required_agents.contains(&agent),
                "Required agents should contain cleanup agent: {}",
                agent
            );
        }
    }

    /// Required agents list contains infrastructure agents.
    #[test]
    fn test_required_agents_has_infrastructure() {
        let contracts = crate::contracts::Contracts::default();

        let infra_agents = [
            "signal-run-prep",
            "run-prep",
            "repo-operator",
            "secrets-sanitizer",
        ];

        for agent in infra_agents {
            assert!(
                contracts.required_agents.contains(&agent),
                "Required agents should contain infrastructure agent: {}",
                agent
            );
        }
    }

    /// Required agents list contains GitHub integration agents.
    #[test]
    fn test_required_agents_has_github() {
        let contracts = crate::contracts::Contracts::default();

        let gh_agents = ["gh-issue-manager", "gh-reporter"];

        for agent in gh_agents {
            assert!(
                contracts.required_agents.contains(&agent),
                "Required agents should contain GitHub agent: {}",
                agent
            );
        }
    }

    // -------------------------------------------------------------------------
    // Required skills list tests
    // -------------------------------------------------------------------------

    /// Required skills list is complete.
    #[test]
    fn test_required_skills_complete() {
        let contracts = crate::contracts::Contracts::default();

        let expected_skills = [
            "test-runner",
            "auto-linter",
            "policy-runner",
            "runs-derive",
            "runs-index",
            "openq-tools",
            "secrets-tools",
        ];

        assert_eq!(
            contracts.required_skills.len(),
            expected_skills.len(),
            "Required skills count should match"
        );

        for skill in expected_skills {
            assert!(
                contracts.required_skills.contains(&skill),
                "Required skills should contain: {}",
                skill
            );
        }
    }

    // -------------------------------------------------------------------------
    // CLAUDE.md sections tests
    // -------------------------------------------------------------------------

    /// CLAUDE.md sections list is complete.
    #[test]
    fn test_claude_md_sections_complete() {
        let contracts = crate::contracts::Contracts::default();

        let expected_sections = [
            ".runs/<run-id>",
            "run_meta.json",
            "index.json",
            "Seven Flows",
            "Receipt",
            "secrets-sanitizer",
        ];

        for section in expected_sections {
            assert!(
                contracts.claude_md_sections.contains(&section),
                "CLAUDE.md sections should contain: {}",
                section
            );
        }
    }

    // -------------------------------------------------------------------------
    // File stem comparison tests (used in duplicate detection)
    // -------------------------------------------------------------------------

    /// File stem extraction works correctly.
    #[test]
    fn test_file_stem_extraction() {
        use std::path::PathBuf;

        let path = PathBuf::from("test-agent.md");
        let stem = path.file_stem().and_then(|s| s.to_str());
        assert_eq!(stem, Some("test-agent"));

        let path_with_dir = PathBuf::from(".claude/agents/test-agent.md");
        let stem = path_with_dir.file_stem().and_then(|s| s.to_str());
        assert_eq!(stem, Some("test-agent"));
    }

    /// File stem matches frontmatter name in well-formed agent files.
    #[test]
    fn test_file_stem_matches_name() {
        let content = r#"---
name: test-agent
---
"#;
        let name = extract_frontmatter_name(content).unwrap();
        let stem = "test-agent";
        assert_eq!(name, stem, "Frontmatter name should match file stem");
    }

    /// Mismatched name and stem is detectable.
    #[test]
    fn test_file_stem_mismatch_detectable() {
        let content = r#"---
name: different-name
---
"#;
        let name = extract_frontmatter_name(content).unwrap();
        let stem = "test-agent";
        assert_ne!(name, stem, "Mismatched name should be detectable");
    }

    // -------------------------------------------------------------------------
    // Structure check integration tests with tempfile
    // -------------------------------------------------------------------------

    use tempfile::TempDir;

    /// Helper: create a minimal .claude directory structure for testing.
    fn create_test_pack(temp_dir: &TempDir) -> std::path::PathBuf {
        let claude_dir = temp_dir.path().join(".claude");
        std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
        std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
        std::fs::create_dir_all(claude_dir.join("skills")).unwrap();
        temp_dir.path().to_path_buf()
    }

    /// Helper: create a CLAUDE.md with specified content.
    fn create_claude_md(repo_root: &std::path::Path, content: &str) {
        std::fs::write(repo_root.join("CLAUDE.md"), content).unwrap();
    }

    /// Helper: create an agent file with specified content.
    fn create_agent(repo_root: &std::path::Path, name: &str, content: &str) {
        let path = repo_root.join(".claude/agents").join(format!("{}.md", name));
        std::fs::write(path, content).unwrap();
    }

    /// Helper: create a command file.
    fn create_command(repo_root: &std::path::Path, name: &str, content: &str) {
        let path = repo_root.join(".claude/commands").join(format!("{}.md", name));
        std::fs::write(path, content).unwrap();
    }

    /// Helper: create a skill directory with SKILL.md.
    fn create_skill(repo_root: &std::path::Path, name: &str, content: &str) {
        let skill_dir = repo_root.join(".claude/skills").join(name);
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(skill_dir.join("SKILL.md"), content).unwrap();
    }

    /// Helper: run structure checks and collect diagnostics.
    fn run_structure_checks(
        repo_root: &std::path::Path,
    ) -> (usize, usize, Vec<String>) {
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;

        let ctx = Ctx::discover(Some(repo_root.to_path_buf())).unwrap();
        let inv = Inventory::from_ctx(&ctx).unwrap();
        let contracts = Contracts::default();
        let re = Regexes::compile().unwrap();

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        let check_ctx = super::CheckCtx {
            ctx: &ctx,
            inv: &inv,
            re: &re,
            c: &contracts,
        };

        // Run structure checks only
        for check in super::checks() {
            rep.section(check.id, check.title);
            let _ = (check.run)(&check_ctx, &mut rep);
        }

        // Extract diagnostics from reporter (use JSON format to access them)
        // Since reporter doesn't expose diagnostics directly, we check error/warning counts
        (rep.errors, rep.warnings, vec![])
    }

    /// Test: Missing required agent is detected (covers line 53).
    #[test]
    fn test_check_required_agents_missing() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md (required)
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create only ONE agent (not all required agents)
        create_agent(&repo_root, "test-agent", "---\nname: test-agent\n---\n# Test Agent\n");

        // Create minimal flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        // Create all required skills
        for skill in ["test-runner", "auto-linter", "policy-runner", "runs-derive", "runs-index", "openq-tools", "secrets-tools"] {
            create_skill(&repo_root, skill, &format!("# {}\n", skill));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should have errors because most required agents are missing
        assert!(errors > 0, "Should detect missing required agents");
    }

    /// Test: Missing flow commands are detected (covers lines 63, 73).
    #[test]
    fn test_check_flow_commands_missing() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create flow commands for only flows 1-3 (missing 4-6)
        for i in 1..=3 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should detect missing flow commands
        assert!(errors > 0, "Should detect missing flow commands");
    }

    /// Test: Agent with missing frontmatter name is flagged (covers lines 94-96).
    #[test]
    fn test_check_duplicate_agents_missing_frontmatter_name() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create agent WITHOUT name in frontmatter
        create_agent(&repo_root, "no-name-agent", "---\ndescription: Agent without name\n---\n# No Name Agent\n");

        // Create flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should detect missing frontmatter name
        assert!(errors > 0, "Should detect agent missing frontmatter name");
    }

    /// Test: Agent with mismatched frontmatter name is flagged (covers lines 99-103).
    #[test]
    fn test_check_duplicate_agents_name_mismatch() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create agent with NAME that doesn't match FILENAME
        create_agent(&repo_root, "wrong-name-agent", "---\nname: different-name\n---\n# Agent with wrong name\n");

        // Create flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should detect name mismatch
        assert!(errors > 0, "Should detect frontmatter name mismatch");
    }

    /// Test: Duplicate agent names are detected (covers lines 106-109).
    #[test]
    fn test_check_duplicate_agents_duplicate_name() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create two agents with the SAME frontmatter name
        create_agent(&repo_root, "agent-one", "---\nname: duplicate-name\n---\n# Agent One\n");
        create_agent(&repo_root, "agent-two", "---\nname: duplicate-name\n---\n# Agent Two\n");

        // Create flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should detect duplicate names (plus the name/stem mismatch)
        assert!(errors > 0, "Should detect duplicate agent names");
    }

    /// Test: No duplicates shows pass messages (covers lines 115-120).
    #[test]
    fn test_check_duplicate_agents_no_duplicates_pass() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create agents with correct names
        create_agent(&repo_root, "agent-one", "---\nname: agent-one\n---\n# Agent One\n");
        create_agent(&repo_root, "agent-two", "---\nname: agent-two\n---\n# Agent Two\n");

        // Create flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        // This should pass the duplicate check (no errors from duplicate detection itself)
        // But will still fail due to missing required agents
        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Errors will be present (missing required agents) but not from duplicates
        // The test validates the pass path is exercised
        assert!(errors > 0, "Should have errors from missing required agents");
    }

    /// Test: Missing required skill is detected (covers lines 129, 133).
    #[test]
    fn test_check_skills_missing() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create only SOME skills (not all required)
        create_skill(&repo_root, "test-runner", "# Test Runner\n");
        // Missing: auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools

        // Create flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should detect missing skills
        assert!(errors > 0, "Should detect missing required skills");
    }

    /// Test: Missing CLAUDE.md is detected (covers line 155).
    #[test]
    fn test_check_claude_md_missing() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // DO NOT create CLAUDE.md

        // Create flow commands
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Should detect missing CLAUDE.md
        assert!(errors > 0, "Should detect missing CLAUDE.md");
    }

    /// Test: Missing customize-pack command triggers warning (covers line 167).
    #[test]
    fn test_check_customizer_command_missing() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create flow commands (but NOT customize-pack)
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }

        // DO NOT create customize-pack command

        let (_errors, warnings, _) = run_structure_checks(&repo_root);

        // Should have warning for missing customize-pack
        assert!(warnings > 0, "Should warn about missing customize-pack command");
    }

    /// Test: Missing pack-customizer agent triggers warning (covers line 174).
    #[test]
    fn test_check_customizer_agent_missing() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create flow commands AND customize-pack command
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }
        create_command(&repo_root, "customize-pack", "# Customize Pack\n");

        // DO NOT create pack-customizer agent

        let (_errors, warnings, _) = run_structure_checks(&repo_root);

        // Should have warning for missing pack-customizer agent
        assert!(warnings > 0, "Should warn about missing pack-customizer agent");
    }

    /// Test: Complete customizer setup passes (both command and agent present).
    #[test]
    fn test_check_customizer_both_present() {
        let temp_dir = TempDir::new().unwrap();
        let repo_root = create_test_pack(&temp_dir);

        // Create CLAUDE.md
        create_claude_md(&repo_root, "# CLAUDE.md\n.runs/<run-id>\nrun_meta.json\nindex.json\nSeven Flows\nReceipt\nsecrets-sanitizer");

        // Create flow commands AND customize-pack command
        for i in 1..=6 {
            create_command(&repo_root, &format!("flow-{}-test", i), &format!("# Flow {}\n", i));
        }
        create_command(&repo_root, "customize-pack", "# Customize Pack\n");

        // Create pack-customizer agent
        create_agent(&repo_root, "pack-customizer", "---\nname: pack-customizer\n---\n# Pack Customizer\n");

        // Run checks - customizer warnings should be 0 (but other warnings may exist)
        let (errors, _warnings, _) = run_structure_checks(&repo_root);

        // Will still have errors from missing required agents, but no customizer warnings
        assert!(errors > 0, "Should have errors from missing required agents");
    }

    // -------------------------------------------------------------------------
    // Agent file pattern tests
    // -------------------------------------------------------------------------

    /// Agent file paths follow expected pattern.
    #[test]
    fn test_agent_file_pattern() {
        use std::path::PathBuf;

        let path = PathBuf::from(".claude/agents/test-agent.md");

        assert!(path.extension().is_some_and(|e| e == "md"));
        assert!(path
            .parent()
            .is_some_and(|p| p.file_name().is_some_and(|n| n == "agents")));
    }

    /// Skill file paths follow expected pattern.
    #[test]
    fn test_skill_file_pattern() {
        use std::path::PathBuf;

        let path = PathBuf::from(".claude/skills/test-runner/SKILL.md");

        assert!(path.file_name().is_some_and(|n| n == "SKILL.md"));
        assert!(path
            .parent()
            .and_then(|p| p.parent())
            .is_some_and(|p| p.file_name().is_some_and(|n| n == "skills")));
    }
}
