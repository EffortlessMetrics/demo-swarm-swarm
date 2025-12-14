//! Drift checks: banned patterns, old taxonomy, raw git, shim enforcement.
//!
//! Checks: 7, 8, 14, 23, 30, 38, 39, 40, 42, 45, 46, 47, 48, 49

use std::path::PathBuf;

use crate::reporter::Reporter;
use crate::util::{LineMatch, find_files_containing_recursive, find_matches_regex_recursive};

use super::{CheckCtx, CheckSpec};

pub fn checks() -> Vec<CheckSpec> {
    vec![
        CheckSpec {
            id: 7,
            title: "Checking for old taxonomy patterns...",
            run: check_old_taxonomy,
        },
        CheckSpec {
            id: 8,
            title: "Checking for removed/deprecated concepts...",
            run: check_banned_patterns,
        },
        CheckSpec {
            id: 14,
            title: "Checking for RUN_BASE alias (should use explicit paths)...",
            run: check_run_base_alias,
        },
        CheckSpec {
            id: 23,
            title: "Checking typed NFR ID contract...",
            run: check_typed_nfr,
        },
        CheckSpec {
            id: 30,
            title: "Checking for flow-specific action enum drift...",
            run: check_flow_specific_actions,
        },
        CheckSpec {
            id: 38,
            title: "Checking for 'operation: ensure_branch' drift in flow commands...",
            run: check_ensure_branch_op,
        },
        CheckSpec {
            id: 39,
            title: "Checking for raw git commands in flow commands...",
            run: check_raw_git,
        },
        CheckSpec {
            id: 40,
            title: "Checking for 'See CLAUDE.md > ...' behavioral substitution...",
            run: check_claude_substitution,
        },
        CheckSpec {
            id: 42,
            title: "Checking Flow 6 does not introduce alternate issue draft filenames...",
            run: check_issue_drafts,
        },
        CheckSpec {
            id: 45,
            title: "Checking cleanup agents use demoswarm shim (no bespoke pipelines)...",
            run: check_cleanup_uses_demoswarm_shim,
        },
        CheckSpec {
            id: 46,
            title: "Checking skill ownership boundaries...",
            run: check_skill_ownership,
        },
        CheckSpec {
            id: 47,
            title: "Checking shim line-continuation bypass...",
            run: check_shim_line_continuation,
        },
        CheckSpec {
            id: 48,
            title: "Checking direct demoswarm invocation (must use shim)...",
            run: check_direct_demoswarm_invocation,
        },
        CheckSpec {
            id: 49,
            title: "Checking agents using demoswarm.sh have ## Skills section...",
            run: check_skills_section_required,
        },
    ]
}

fn format_line_matches(cx: &CheckCtx, matches: Vec<LineMatch>) -> Vec<String> {
    matches
        .into_iter()
        .map(|m| {
            format!(
                "{}:{}:{}",
                cx.ctx.rel(&m.path),
                m.line_no,
                m.line.trim_end()
            )
        })
        .collect()
}

/// Check 7: No old taxonomy (FR-* instead of REQ-*).
fn check_old_taxonomy(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let roots = [cx.ctx.agents_dir.clone(), cx.ctx.commands_dir.clone()];

    let fr_hits =
        find_matches_regex_recursive(cx.ctx, &roots, &cx.re.old_fr_id, &["pack-check.sh"])?;
    let fr_hits_filtered: Vec<_> = fr_hits
        .into_iter()
        .filter(|m| !m.line.contains("NFR-"))
        .collect();

    if !fr_hits_filtered.is_empty() {
        rep.fail("Found old FR-* taxonomy (use REQ-* / typed NFR-*):");
        rep.indent_lines(format_line_matches(cx, fr_hits_filtered));
    } else {
        rep.pass("No old FR-* taxonomy found");
    }

    let old_bdd_hits =
        find_matches_regex_recursive(cx.ctx, &roots, &cx.re.old_bdd_tag, &["pack-check.sh"])?;
    if !old_bdd_hits.is_empty() {
        rep.fail("Found old @FR- tags in pack files");
        rep.indent_lines(format_line_matches(cx, old_bdd_hits));
    } else {
        rep.pass("No old @FR- tags found");
    }

    Ok(())
}

/// Check 8: No references to removed concepts.
fn check_banned_patterns(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for pattern in cx.c.banned_patterns {
        let matches = find_files_containing_recursive(
            cx.ctx,
            &cx.ctx.claude_dir,
            pattern,
            &["pack-check.sh"],
        )?;
        if !matches.is_empty() {
            rep.fail(format!("Found banned pattern '{pattern}' in:"));
            let rels = matches
                .into_iter()
                .map(|p| cx.ctx.rel(&p))
                .collect::<Vec<_>>();
            rep.indent_lines(rels);
        } else {
            rep.pass(format!("No references to '{pattern}'"));
        }
    }

    Ok(())
}

/// Check 14: No RUN_BASE alias in flows/agents.
fn check_run_base_alias(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut hits = find_files_containing_recursive(
        cx.ctx,
        &cx.ctx.commands_dir,
        "RUN_BASE",
        &["pack-check.sh"],
    )?;
    hits.extend(find_files_containing_recursive(
        cx.ctx,
        &cx.ctx.agents_dir,
        "RUN_BASE",
        &["pack-check.sh"],
    )?);

    if !hits.is_empty() {
        rep.fail("Found 'RUN_BASE' alias (use explicit .runs/<run-id>/ paths) in:");
        let rels = hits.into_iter().map(|p| cx.ctx.rel(&p)).collect::<Vec<_>>();
        rep.indent_lines(rels);
    } else {
        rep.pass("No RUN_BASE alias in flows/agents");
    }

    Ok(())
}

/// Check 23: Typed NFR ID contract.
fn check_typed_nfr(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let roots = [cx.ctx.agents_dir.clone(), cx.ctx.commands_dir.clone()];

    let bare_nfr_matches =
        find_matches_regex_recursive(cx.ctx, &roots, &cx.re.bare_nfr_id, &["pack-check.sh"])?;
    if !bare_nfr_matches.is_empty() {
        rep.fail("Found bare NFR-### patterns (should be NFR-<DOMAIN>-###) in:");
        rep.indent_lines(format_line_matches(cx, bare_nfr_matches));
    } else {
        rep.pass("No bare NFR-### patterns found (all NFRs are typed)");
    }

    let nfr_scale_hits = find_files_containing_recursive(
        cx.ctx,
        &cx.ctx.agents_dir,
        "NFR-SCALE-",
        &["pack-check.sh"],
    )?;
    let mut nfr_scale_hits2 = find_files_containing_recursive(
        cx.ctx,
        &cx.ctx.commands_dir,
        "NFR-SCALE-",
        &["pack-check.sh"],
    )?;
    let mut scale_all = nfr_scale_hits;
    scale_all.append(&mut nfr_scale_hits2);

    if !scale_all.is_empty() {
        rep.fail("Found deprecated NFR-SCALE-* (use NFR-PERF-* instead) in:");
        let rels = scale_all
            .into_iter()
            .map(|p| cx.ctx.rel(&p))
            .collect::<Vec<_>>();
        rep.indent_lines(rels);
    } else {
        rep.pass("No deprecated NFR-SCALE-* patterns found");
    }

    Ok(())
}

/// Check 30: No flow-specific action enums.
fn check_flow_specific_actions(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let roots: [PathBuf; 2] = [cx.ctx.agents_dir.clone(), cx.ctx.commands_dir.clone()];

    for pattern in cx.c.flow_specific_patterns {
        let matches =
            find_files_containing_recursive(cx.ctx, &roots[0], pattern, &["pack-check.sh"])?;
        let mut matches2 =
            find_files_containing_recursive(cx.ctx, &roots[1], pattern, &["pack-check.sh"])?;
        let mut all = matches;
        all.append(&mut matches2);

        if !all.is_empty() {
            rep.fail(format!(
                "Found flow-specific action '{pattern}' (use closed enum + route_to_flow) in:"
            ));
            let rels = all.into_iter().map(|p| cx.ctx.rel(&p)).collect::<Vec<_>>();
            rep.indent_lines(rels);
        } else {
            rep.pass(format!("No flow-specific '{pattern}'"));
        }
    }

    let domain_verdict_hits =
        find_matches_regex_recursive(cx.ctx, &roots, &cx.re.domain_verdict, &["pack-check.sh"])?;
    if !domain_verdict_hits.is_empty() {
        rep.fail("Found domain verdict keywords in recommended_action (must stay closed enum):");
        rep.indent_lines(format_line_matches(cx, domain_verdict_hits));
    } else {
        rep.pass("No domain verdict keywords in recommended_action");
    }

    Ok(())
}

/// Check 38: Reject operation: ensure_branch in flow commands.
fn check_ensure_branch_op(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let ensure_hits = find_matches_regex_recursive(
        cx.ctx,
        std::slice::from_ref(&cx.ctx.commands_dir),
        &cx.re.ensure_branch_op,
        &["pack-check.sh"],
    )?;

    if !ensure_hits.is_empty() {
        rep.fail("Found 'operation: ensure_branch' in flow commands (use task phrasing):");
        rep.indent_lines(format_line_matches(cx, ensure_hits));
    } else {
        rep.pass("No 'operation: ensure_branch' in flow commands");
    }

    Ok(())
}

/// Check 39: No raw git commands in flow commands.
fn check_raw_git(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut raw_git_found = 0usize;

    for (needle, display) in cx.c.raw_git_patterns {
        let mut non_allowed = Vec::new();

        for cmd in &cx.inv.flow_cmd_files {
            let content = cx.ctx.read_utf8(cmd)?;
            for (idx, line) in content.lines().enumerate() {
                if line.contains(needle)
                    && !line.contains("repo-operator")
                    && !line.contains("(not `")
                {
                    non_allowed.push(format!(
                        "{}:{}:{}",
                        cx.ctx.rel(cmd),
                        idx + 1,
                        line.trim_end()
                    ));
                }
            }
        }

        if !non_allowed.is_empty() {
            rep.fail(format!(
                "Raw git pattern '{display}' found in flow commands (pack bug: flows must delegate to repo-operator):"
            ));
            rep.indent_lines(non_allowed.into_iter().take(3));
            raw_git_found += 1;
        }
    }

    if raw_git_found == 0 {
        rep.pass("No problematic raw git commands in flow commands");
    }

    Ok(())
}

/// Check 40: No CLAUDE.md behavioral substitution.
fn check_claude_substitution(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let sub_hits = find_matches_regex_recursive(
        cx.ctx,
        std::slice::from_ref(&cx.ctx.commands_dir),
        &cx.re.claude_sub,
        &["pack-check.sh"],
    )?;

    if !sub_hits.is_empty() {
        rep.fail("Found 'See CLAUDE.md > ...' substitution in flow commands (inline the rule):");
        rep.indent_lines(format_line_matches(cx, sub_hits));
    } else {
        rep.pass("No CLAUDE.md behavioral substitution in flow commands");
    }

    Ok(())
}

/// Check 42: No alternate issue draft filenames.
fn check_issue_drafts(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let issue_drafts_refs = find_matches_regex_recursive(
        cx.ctx,
        &[cx.ctx.commands_dir.clone(), cx.ctx.agents_dir.clone()],
        &cx.re.issue_drafts,
        &["pack-check.sh"],
    )?;

    if !issue_drafts_refs.is_empty() {
        rep.fail("Found 'issue_drafts.md' reference (standardize on feedback_actions.md):");
        rep.indent_lines(format_line_matches(cx, issue_drafts_refs));
    } else {
        rep.pass("No issue_drafts.md references (feedback_actions.md is the standard)");
    }

    Ok(())
}

/// Check 45: Cleanup agents must use demoswarm shim, not bespoke pipelines.
///
/// This enforces the pack contract that cleanup agents use the standardized
/// `demoswarm.sh` shim for mechanical counts and extraction, not inline
/// `grep|sed|awk|jq` pipelines.
fn check_cleanup_uses_demoswarm_shim(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    // Only check cleanup agents
    let cleanup_files: Vec<PathBuf> = cx
        .inv
        .agent_md_files
        .iter()
        .filter(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().ends_with("-cleanup.md"))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let mut violations = Vec::new();

    for path in &cleanup_files {
        let content = match cx.ctx.read_utf8(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Check for bespoke pipeline patterns, but allow them in:
        // - Code blocks that are documenting the old way (```bash with comment about "old way")
        // - Lines that reference demoswarm.sh or runs_ (using the shim correctly)
        // - Lines that are in examples showing "do not do this"
        let mut in_allowed_context = false;

        for (idx, line) in content.lines().enumerate() {
            // Track if we're in an allowed context
            if line.contains("# Do not") || line.contains("# don't") || line.contains("old way") {
                in_allowed_context = true;
            }
            if line.trim().is_empty() {
                in_allowed_context = false;
            }

            // Skip lines that reference demoswarm.sh (they're using the shim correctly)
            if line.contains("demoswarm.sh") || line.contains("runs_") {
                continue;
            }

            // Skip allowed contexts
            if in_allowed_context {
                continue;
            }

            // Check for bespoke pipeline patterns
            if cx.re.bespoke_pipeline.is_match(line) {
                violations.push(format!(
                    "{}:{}:{}",
                    cx.ctx.rel(path),
                    idx + 1,
                    line.trim_end()
                ));
            }
        }
    }

    if !violations.is_empty() {
        rep.fail(
            "Found bespoke grep/sed/awk/jq pipelines in cleanup agents (use demoswarm.sh shim):",
        );
        rep.indent_lines(violations.into_iter().take(10));
    } else {
        rep.pass("Cleanup agents use demoswarm.sh shim (no bespoke pipelines)");
    }

    Ok(())
}

/// Check 46: Skill ownership boundaries.
///
/// Enforces that restricted skill commands are only used by allowed agents:
/// - `index upsert-status` → cleanup + run-prep agents only
/// - `secrets scan|redact` → secrets-sanitizer only
/// - `openq next-id|append` → clarifier only
fn check_skill_ownership(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut violations = Vec::new();

    for agent_file in &cx.inv.agent_md_files {
        let agent_name = agent_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let content = match cx.ctx.read_utf8(agent_file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (idx, line) in content.lines().enumerate() {
            // Check index upsert-status
            if cx.re.index_upsert_cmd.is_match(line)
                && !cx.c.index_allowed_agents.contains(&agent_name) {
                violations.push(format!(
                    "{}:{}:{} (only cleanup/run-prep agents may use index upsert-status)",
                    cx.ctx.rel(agent_file),
                    idx + 1,
                    line.trim_end()
                ));
            }

            // Check secrets scan|redact
            if cx.re.secrets_cmd.is_match(line)
                && !cx.c.secrets_allowed_agents.contains(&agent_name) {
                violations.push(format!(
                    "{}:{}:{} (only secrets-sanitizer may use secrets scan/redact)",
                    cx.ctx.rel(agent_file),
                    idx + 1,
                    line.trim_end()
                ));
            }

            // Check openq next-id|append
            if cx.re.openq_cmd.is_match(line)
                && !cx.c.openq_allowed_agents.contains(&agent_name) {
                violations.push(format!(
                    "{}:{}:{} (only clarifier may use openq next-id/append)",
                    cx.ctx.rel(agent_file),
                    idx + 1,
                    line.trim_end()
                ));
            }
        }
    }

    if !violations.is_empty() {
        rep.fail("Found skill ownership violations:");
        rep.indent_lines(violations.into_iter().take(10));
    } else {
        rep.pass("Skill ownership boundaries enforced");
    }

    Ok(())
}

/// Check 47: No line-continuation bypass after demoswarm.sh.
///
/// Prevents clever/accidental line-wrap from bypassing ownership checks:
/// ```text
/// bash .claude/scripts/demoswarm.sh \
///   secrets scan ...  # This splits the command, hiding the subcommand
/// ```
fn check_shim_line_continuation(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let roots = [cx.ctx.agents_dir.clone(), cx.ctx.commands_dir.clone()];

    let matches = find_matches_regex_recursive(cx.ctx, &roots, &cx.re.shim_line_continuation, &[])?;

    if !matches.is_empty() {
        rep.fail(
            "Found demoswarm.sh followed by line continuation (subcommand must be on same line):",
        );
        rep.indent_lines(format_line_matches(cx, matches).into_iter().take(5));
    } else {
        rep.pass("No shim line-continuation bypass patterns");
    }

    Ok(())
}

/// Check 48: No direct demoswarm invocation (must use shim).
///
/// Ensures `demoswarm` is not invoked directly (bypassing the shim):
/// ```text
/// demoswarm count ...        # BAD: bypasses shim
/// bash .claude/scripts/demoswarm.sh count ...  # GOOD: uses shim
/// ```
///
/// Exceptions:
/// - `tools/` directory (the CLI source itself)
/// - `docs/reference/` (documentation about the CLI)
fn check_direct_demoswarm_invocation(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let roots = [cx.ctx.agents_dir.clone(), cx.ctx.commands_dir.clone()];

    let matches =
        find_matches_regex_recursive(cx.ctx, &roots, &cx.re.direct_demoswarm_invocation, &[])?;

    // Filter out allowed contexts (lines that include demoswarm.sh - they're using the shim)
    let violations: Vec<_> = matches
        .into_iter()
        .filter(|m| !m.line.contains("demoswarm.sh"))
        .collect();

    if !violations.is_empty() {
        rep.fail("Found direct demoswarm invocation (must use bash .claude/scripts/demoswarm.sh):");
        rep.indent_lines(format_line_matches(cx, violations).into_iter().take(5));
    } else {
        rep.pass("No direct demoswarm invocations (all use shim)");
    }

    Ok(())
}

/// Check 49: Agents using demoswarm.sh must have a ## Skills section.
///
/// Improves discoverability by ensuring agents document which skills they use.
fn check_skills_section_required(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut missing_skills_section = Vec::new();

    for agent_file in &cx.inv.agent_md_files {
        let content = match cx.ctx.read_utf8(agent_file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Check if file uses demoswarm.sh
        if content.contains("demoswarm.sh") {
            // Check if it has a ## Skills section
            if !content.contains("## Skills") && !content.contains("## Skill") {
                missing_skills_section.push(cx.ctx.rel(agent_file));
            }
        }
    }

    if !missing_skills_section.is_empty() {
        rep.fail("Agents using demoswarm.sh must have a ## Skills section:");
        rep.indent_lines(missing_skills_section.into_iter().take(10));
    } else {
        rep.pass("All agents using demoswarm.sh have ## Skills section");
    }

    Ok(())
}
