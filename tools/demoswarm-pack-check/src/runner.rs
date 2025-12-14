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
