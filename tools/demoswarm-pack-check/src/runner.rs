//! Runner: orchestration only.
//!
//! Build Ctx → Inventory → Contracts → Regexes, then run all checks, then report.

use std::process::ExitCode;

use anyhow::Context;

use crate::{
    checks::{self, CheckCtx},
    cli::{Cli, OutputFormat},
    contracts::{Contracts, Regexes},
    ctx::Ctx,
    inventory::Inventory,
    reporter::{PackCounts, Reporter},
};

pub fn run(cli: Cli) -> anyhow::Result<ExitCode> {
    // Discover repo root
    let ctx = Ctx::discover(cli.repo_root.clone()).context("discover repo root")?;

    // Build inventory
    let inv = Inventory::from_ctx(&ctx).context("build inventory")?;

    // Build contracts
    let contracts = Contracts::default();

    // Compile regexes
    let re = Regexes::compile().context("compile regexes")?;

    // Create reporter
    let mut rep = Reporter::new(cli.format, !cli.no_color, cli.strict_warnings);
    if cli.format == OutputFormat::Text {
        rep.print_banner();
    }

    // Build check context
    let check_ctx = CheckCtx {
        ctx: &ctx,
        inv: &inv,
        re: &re,
        c: &contracts,
    };

    // Run all checks (keep-going: catch errors and continue)
    for check in checks::all() {
        rep.section(check.id, check.title);
        if let Err(e) = (check.run)(&check_ctx, &mut rep) {
            rep.fail(format!("check crashed: {e:#}"));
        }
        rep.blank_line();
    }

    // Summary
    if cli.format == OutputFormat::Text {
        rep.print_summary_header();
    }

    let counts = PackCounts {
        agents: inv.agent_md_files.len(),
        commands: inv.command_md_files.len(),
        skills: inv.skill_md_files.len(),
    };

    if cli.format == OutputFormat::Text {
        rep.print_counts(&counts);
    }

    rep.finish(&ctx.repo_root.display().to_string(), counts)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // -------------------------------------------------------------------------
    // Cli struct tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_cli_default_values() {
        let cli = Cli {
            repo_root: None,
            format: OutputFormat::Text,
            no_color: false,
            strict_warnings: false,
        };

        assert!(cli.repo_root.is_none());
        assert_eq!(cli.format, OutputFormat::Text);
        assert!(!cli.no_color);
        assert!(!cli.strict_warnings);
    }

    #[test]
    fn test_cli_json_format() {
        let cli = Cli {
            repo_root: Some(PathBuf::from("/test/path")),
            format: OutputFormat::Json,
            no_color: true,
            strict_warnings: true,
        };

        assert_eq!(cli.repo_root, Some(PathBuf::from("/test/path")));
        assert_eq!(cli.format, OutputFormat::Json);
        assert!(cli.no_color);
        assert!(cli.strict_warnings);
    }

    // -------------------------------------------------------------------------
    // run() integration tests with minimal .claude directory
    // -------------------------------------------------------------------------

    /// Test that run() fails when .claude directory doesn't exist.
    #[test]
    fn test_run_fails_without_claude_dir() {
        let tmp = TempDir::new().unwrap();

        let cli = Cli {
            repo_root: Some(tmp.path().to_path_buf()),
            format: OutputFormat::Json,
            no_color: true,
            strict_warnings: false,
        };

        let result = run(cli);
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        assert!(
            err.contains(".claude") || err.contains("discover"),
            "Error should mention .claude or discover: {}",
            err
        );
    }

    /// Test that run() executes with minimal .claude directory (JSON mode).
    #[test]
    fn test_run_with_minimal_claude_dir_json() {
        let tmp = TempDir::new().unwrap();

        // Create minimal .claude directory structure
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();
        std::fs::create_dir(claude_dir.join("agents")).unwrap();
        std::fs::create_dir(claude_dir.join("commands")).unwrap();
        std::fs::create_dir(claude_dir.join("skills")).unwrap();

        let cli = Cli {
            repo_root: Some(tmp.path().to_path_buf()),
            format: OutputFormat::Json,
            no_color: true,
            strict_warnings: false,
        };

        // Should not panic or error - will have many warnings/errors but should complete
        let result = run(cli);
        assert!(result.is_ok());
    }

    /// Test that run() executes with minimal .claude directory (Text mode).
    #[test]
    fn test_run_with_minimal_claude_dir_text() {
        let tmp = TempDir::new().unwrap();

        // Create minimal .claude directory structure
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();
        std::fs::create_dir(claude_dir.join("agents")).unwrap();
        std::fs::create_dir(claude_dir.join("commands")).unwrap();
        std::fs::create_dir(claude_dir.join("skills")).unwrap();

        let cli = Cli {
            repo_root: Some(tmp.path().to_path_buf()),
            format: OutputFormat::Text,
            no_color: true,
            strict_warnings: false,
        };

        // Should not panic or error
        let result = run(cli);
        assert!(result.is_ok());
    }

    /// Test run() with strict_warnings mode.
    #[test]
    fn test_run_with_strict_warnings() {
        let tmp = TempDir::new().unwrap();

        // Create minimal .claude directory structure
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();
        std::fs::create_dir(claude_dir.join("agents")).unwrap();
        std::fs::create_dir(claude_dir.join("commands")).unwrap();
        std::fs::create_dir(claude_dir.join("skills")).unwrap();

        let cli = Cli {
            repo_root: Some(tmp.path().to_path_buf()),
            format: OutputFormat::Json,
            no_color: true,
            strict_warnings: true,
        };

        // Should complete, likely with non-zero exit code due to missing required files
        let result = run(cli);
        assert!(result.is_ok());
    }

    /// Test that run() handles path canonicalization errors.
    #[test]
    fn test_run_with_nonexistent_path() {
        let cli = Cli {
            repo_root: Some(PathBuf::from("/definitely/not/a/real/path/xyz123abc")),
            format: OutputFormat::Json,
            no_color: true,
            strict_warnings: false,
        };

        let result = run(cli);
        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // PackCounts construction test
    // -------------------------------------------------------------------------

    #[test]
    fn test_pack_counts_from_inventory() {
        let tmp = TempDir::new().unwrap();

        // Create .claude directory with some files
        let claude_dir = tmp.path().join(".claude");
        let agents_dir = claude_dir.join("agents");
        let commands_dir = claude_dir.join("commands");
        let skills_dir = claude_dir.join("skills").join("test-skill");

        std::fs::create_dir_all(&agents_dir).unwrap();
        std::fs::create_dir_all(&commands_dir).unwrap();
        std::fs::create_dir_all(&skills_dir).unwrap();

        // Create some agent files
        std::fs::write(agents_dir.join("agent1.md"), "# Agent 1").unwrap();
        std::fs::write(agents_dir.join("agent2.md"), "# Agent 2").unwrap();

        // Create some command files
        std::fs::write(commands_dir.join("flow-1-signal.md"), "# Flow 1").unwrap();

        // Create a skill file
        std::fs::write(skills_dir.join("SKILL.md"), "# Skill").unwrap();

        // Build context and inventory
        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let inv = Inventory::from_ctx(&ctx).unwrap();

        // Verify counts
        let counts = PackCounts {
            agents: inv.agent_md_files.len(),
            commands: inv.command_md_files.len(),
            skills: inv.skill_md_files.len(),
        };

        assert_eq!(counts.agents, 2);
        assert_eq!(counts.commands, 1);
        assert_eq!(counts.skills, 1);
    }
}
