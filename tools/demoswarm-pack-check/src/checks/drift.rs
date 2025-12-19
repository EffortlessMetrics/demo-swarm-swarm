//! Drift checks: banned patterns, old taxonomy, raw git, shim enforcement, GH hygiene.
//!
//! Checks: 7, 8, 14, 23, 30, 38, 39, 40, 42, 45, 46, 47, 48, 49, 50, 52, 53

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
        CheckSpec {
            id: 50,
            title: "Checking GH agents use heredoc (not --body-file) and no forbidden patterns...",
            run: check_gh_body_hygiene,
        },
        CheckSpec {
            id: 52,
            title: "Checking flow commands do not contain demoswarm.sh or skill CLI syntax...",
            run: check_flow_boundary_enforcement,
        },
        CheckSpec {
            id: 53,
            title: "Checking OpenQ QID patterns use canonical flow codes...",
            run: check_openq_prefix_validation,
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
                && !cx.c.index_allowed_agents.contains(&agent_name)
            {
                violations.push(format!(
                    "{}:{}:{} (only cleanup/run-prep agents may use index upsert-status)",
                    cx.ctx.rel(agent_file),
                    idx + 1,
                    line.trim_end()
                ));
            }

            // Check secrets scan|redact
            if cx.re.secrets_cmd.is_match(line)
                && !cx.c.secrets_allowed_agents.contains(&agent_name)
            {
                violations.push(format!(
                    "{}:{}:{} (only secrets-sanitizer may use secrets scan/redact)",
                    cx.ctx.rel(agent_file),
                    idx + 1,
                    line.trim_end()
                ));
            }

            // Check openq next-id|append
            if cx.re.openq_cmd.is_match(line) && !cx.c.openq_allowed_agents.contains(&agent_name) {
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

/// Check 50: GH agents must use heredoc pattern (not --body-file) and avoid forbidden patterns.
///
/// This prevents the class of failures where:
/// - Windows paths in --body-file are misinterpreted
/// - Temp file paths leak into commands
/// - Absolute paths or placeholders get posted
///
/// Requirements for GH posting agents:
/// 1. Must NOT use --body-file (fails on Windows paths)
/// 2. Must use heredoc pattern: -f body="$(cat <<'EOF' ... EOF)"
/// 3. Must NOT contain forbidden patterns (absolute paths, temp dirs, placeholders)
fn check_gh_body_hygiene(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut violations = Vec::new();
    let mut missing_heredoc = Vec::new();

    for agent_name in cx.c.gh_posting_agents {
        let Some(agent_file) = cx.inv.agent(agent_name) else {
            continue;
        };

        let content = match cx.ctx.read_utf8(agent_file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Track if we're inside a code block (fenced with ```)
        let mut in_code_block = false;

        // Check for forbidden patterns
        for (idx, line) in content.lines().enumerate() {
            // Track code block boundaries
            if line.trim().starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            // Skip content inside code blocks (examples/documentation)
            if in_code_block {
                continue;
            }

            for pattern in cx.c.gh_body_forbidden_patterns {
                // Skip patterns that appear in "do NOT" or "don't" documentation
                let line_lower = line.to_lowercase();
                if line_lower.contains("do not")
                    || line_lower.contains("don't")
                    || line_lower.contains("must not")
                {
                    continue;
                }

                // Check if the pattern appears outside of documentation context
                if line.contains(pattern) {
                    violations.push(format!(
                        "{}:{}:{} (forbidden: '{}')",
                        cx.ctx.rel(agent_file),
                        idx + 1,
                        line.trim_end(),
                        pattern
                    ));
                }
            }
        }

        // Check for heredoc pattern (if agent posts to GH, should have heredoc example)
        if (content.contains("gh api") || content.contains("gh issue"))
            && !cx.re.gh_heredoc_pattern.is_match(&content)
        {
            missing_heredoc.push(format!(
                "{} uses gh commands but missing heredoc pattern (-f body=\"$(cat <<'EOF'...)",
                agent_name
            ));
        }
    }

    if !violations.is_empty() {
        rep.fail("Found forbidden patterns in GH agents (will cause failures):");
        rep.indent_lines(violations.into_iter().take(10));
    } else {
        rep.pass("No forbidden patterns in GH agents");
    }

    if !missing_heredoc.is_empty() {
        rep.fail("GH agents missing heredoc pattern:");
        rep.indent_lines(missing_heredoc);
    } else {
        rep.pass("All GH agents use heredoc pattern for body passing");
    }

    Ok(())
}

/// Check 52: Flow boundary enforcement - no demoswarm.sh or skill CLI syntax in flow commands.
///
/// Enforces the three-tier ownership model:
/// - Flow commands -> delegate to agents
/// - Agent docs -> use skills
/// - Skill docs -> implement CLI operations
///
/// Flow commands should NOT contain:
/// - "demoswarm.sh" (direct shim invocation)
/// - Skill CLI subcommands (count, ms, yaml, index, receipt, receipts, openapi, line, inv, time, openq, secrets)
///
/// This prevents flow commands from bypassing the agent layer and directly using skill-layer CLI.
fn check_flow_boundary_enforcement(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut shim_violations = Vec::new();
    let mut subcommand_violations = Vec::new();

    for flow_cmd in &cx.inv.flow_cmd_files {
        let content = match cx.ctx.read_utf8(flow_cmd) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (idx, line) in content.lines().enumerate() {
            // Check for demoswarm.sh invocation
            if line.contains("demoswarm.sh") {
                shim_violations.push(format!(
                    "{}:{}:{}",
                    cx.ctx.rel(flow_cmd),
                    idx + 1,
                    line.trim_end()
                ));
            }

            // Check for skill CLI subcommands in command-like contexts
            // We want to flag: "demoswarm count", "ms get", "yaml get", etc.
            // But NOT prose mentions like "the count of items" or "the index of"
            // Strategy: look for patterns that suggest CLI invocation context
            for subcommand in cx.c.skill_cli_subcommands {
                // Look for the subcommand in CLI-like contexts:
                // - After "demoswarm" (e.g., "demoswarm count")
                // - After "bash" or in backticks/code blocks
                // - As a shell command pattern
                let patterns = [
                    format!("demoswarm {}", subcommand),
                    format!("demoswarm.sh {}", subcommand),
                    format!("`{} ", subcommand),
                    format!("` {} ", subcommand),
                ];

                for pattern in &patterns {
                    if line.contains(pattern) {
                        subcommand_violations.push(format!(
                            "{}:{}:{} (skill subcommand: '{}')",
                            cx.ctx.rel(flow_cmd),
                            idx + 1,
                            line.trim_end(),
                            subcommand
                        ));
                        break;
                    }
                }
            }
        }
    }

    if !shim_violations.is_empty() {
        rep.warn("Found demoswarm.sh in flow commands (flows should delegate to agents):");
        rep.indent_lines(shim_violations.into_iter().take(10));
    } else {
        rep.pass("No demoswarm.sh in flow commands");
    }

    if !subcommand_violations.is_empty() {
        rep.warn("Found skill CLI subcommands in flow commands (flows should delegate to agents):");
        rep.indent_lines(subcommand_violations.into_iter().take(10));
    } else {
        rep.pass("No skill CLI subcommands in flow commands");
    }

    Ok(())
}

/// Check 53: OpenQ prefix validation - QIDs must use canonical flow codes.
///
/// Validates that QID patterns in `.runs/**/open_questions.md` files follow
/// the canonical format: OQ-<FLOW>-<NNN>
///
/// Where:
/// - <FLOW> is one of: SIG, PLN, BLD, GAT, DEP, WIS (canonical abbreviations)
/// - <NNN> is a three-digit zero-padded number (001-999)
///
/// Reports warnings for:
/// - Non-canonical flow codes (e.g., PLAN instead of PLN, BUILD instead of BLD)
/// - Invalid numeric suffixes (not three digits)
fn check_openq_prefix_validation(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    use walkdir::WalkDir;

    let runs_dir = cx.ctx.repo_root.join(".runs");
    if !runs_dir.exists() {
        rep.pass("No .runs directory found (skipping OpenQ validation)");
        return Ok(());
    }

    let mut invalid_codes = Vec::new();
    let mut invalid_format = Vec::new();

    // Find all open_questions.md files under .runs/
    for entry in WalkDir::new(&runs_dir).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name != "open_questions.md" {
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Look for QID patterns: OQ-XXX-NNN or similar
        // Valid: OQ-SIG-001, OQ-PLN-002, etc.
        // Invalid: OQ-PLAN-001, OQ-BUILD-002, OQ-SIG-1, etc.
        for (idx, line) in content.lines().enumerate() {
            // Look for QID: or - QID: patterns
            if !line.contains("QID:") && !line.contains("OQ-") {
                continue;
            }

            // Extract QID patterns using a simple approach
            // Pattern: OQ-<letters>-<digits>
            let mut pos = 0;
            while let Some(start) = line[pos..].find("OQ-") {
                let abs_start = pos + start;
                let rest = &line[abs_start + 3..];

                // Find the flow code part (letters until -)
                if let Some(dash_pos) = rest.find('-') {
                    let flow_code = &rest[..dash_pos];
                    let after_dash = &rest[dash_pos + 1..];

                    // Find the numeric part
                    let num_end = after_dash
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(after_dash.len());
                    let num_part = &after_dash[..num_end];

                    // Validate flow code
                    if !flow_code.is_empty() {
                        let is_canonical = cx.c.openq_flow_codes.contains(&flow_code);

                        if !is_canonical {
                            // Check if it's a known non-canonical code
                            let non_canonical_codes =
                                ["PLAN", "BUILD", "GATE", "DEPLOY", "SIGNAL", "WISDOM"];
                            let is_known_wrong = non_canonical_codes.contains(&flow_code);

                            if is_known_wrong {
                                invalid_codes.push(format!(
                                    "{}:{}:{} (use {} instead of {})",
                                    cx.ctx.rel(path),
                                    idx + 1,
                                    line.trim_end(),
                                    suggest_canonical_code(flow_code),
                                    flow_code
                                ));
                            } else if flow_code.chars().all(|c| c.is_ascii_uppercase()) {
                                // Unknown uppercase code
                                invalid_codes.push(format!(
                                    "{}:{}:{} (unknown flow code: {})",
                                    cx.ctx.rel(path),
                                    idx + 1,
                                    line.trim_end(),
                                    flow_code
                                ));
                            }
                        }

                        // Validate numeric suffix (should be 3 digits, zero-padded)
                        if !num_part.is_empty() && num_part.len() != 3 {
                            invalid_format.push(format!(
                                "{}:{}:{} (numeric suffix should be 3 digits: {})",
                                cx.ctx.rel(path),
                                idx + 1,
                                line.trim_end(),
                                num_part
                            ));
                        }
                    }
                }

                pos = abs_start + 3;
            }
        }
    }

    // Deduplicate violations (same line may match multiple patterns)
    invalid_codes.sort();
    invalid_codes.dedup();
    invalid_format.sort();
    invalid_format.dedup();

    if !invalid_codes.is_empty() {
        rep.warn("Found non-canonical OpenQ flow codes:");
        rep.indent_lines(invalid_codes.into_iter().take(10));
    } else {
        rep.pass("All OpenQ QIDs use canonical flow codes");
    }

    if !invalid_format.is_empty() {
        rep.warn("Found OpenQ QIDs with invalid format:");
        rep.indent_lines(invalid_format.into_iter().take(10));
    } else {
        rep.pass("All OpenQ QIDs use valid format (OQ-<FLOW>-<NNN>)");
    }

    Ok(())
}

/// Suggest the canonical flow code for a non-canonical one.
fn suggest_canonical_code(non_canonical: &str) -> &'static str {
    match non_canonical {
        "SIGNAL" => "SIG",
        "PLAN" => "PLN",
        "BUILD" => "BLD",
        "GATE" => "GAT",
        "DEPLOY" => "DEP",
        "WISDOM" => "WIS",
        _ => "???",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================================================
    // Tests for suggest_canonical_code
    // ==========================================================================

    #[test]
    fn test_suggest_canonical_code_signal() {
        assert_eq!(suggest_canonical_code("SIGNAL"), "SIG");
    }

    #[test]
    fn test_suggest_canonical_code_plan() {
        assert_eq!(suggest_canonical_code("PLAN"), "PLN");
    }

    #[test]
    fn test_suggest_canonical_code_build() {
        assert_eq!(suggest_canonical_code("BUILD"), "BLD");
    }

    #[test]
    fn test_suggest_canonical_code_gate() {
        assert_eq!(suggest_canonical_code("GATE"), "GAT");
    }

    #[test]
    fn test_suggest_canonical_code_deploy() {
        assert_eq!(suggest_canonical_code("DEPLOY"), "DEP");
    }

    #[test]
    fn test_suggest_canonical_code_wisdom() {
        assert_eq!(suggest_canonical_code("WISDOM"), "WIS");
    }

    #[test]
    fn test_suggest_canonical_code_unknown_returns_fallback() {
        assert_eq!(suggest_canonical_code("UNKNOWN"), "???");
    }

    #[test]
    fn test_suggest_canonical_code_empty_string() {
        assert_eq!(suggest_canonical_code(""), "???");
    }

    #[test]
    fn test_suggest_canonical_code_lowercase_not_matched() {
        // The function is case-sensitive, lowercase should return fallback
        assert_eq!(suggest_canonical_code("signal"), "???");
        assert_eq!(suggest_canonical_code("plan"), "???");
    }

    #[test]
    fn test_suggest_canonical_code_partial_match() {
        // Partial matches should not work
        assert_eq!(suggest_canonical_code("SIGN"), "???");
        assert_eq!(suggest_canonical_code("PLA"), "???");
        assert_eq!(suggest_canonical_code("SIGNALFOO"), "???");
    }

    // ==========================================================================
    // Tests for format_line_matches helper
    // ==========================================================================

    mod format_line_matches_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::util::LineMatch;
        use tempfile::TempDir;

        fn setup_test_ctx() -> (TempDir, Ctx, Inventory, Regexes, Contracts) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
            std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
            std::fs::create_dir_all(claude_dir.join("skills")).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();

            (temp, ctx, inv, re, c)
        }

        #[test]
        fn test_format_line_matches_empty_input() {
            let (_temp, ctx, inv, re, c) = setup_test_ctx();
            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let matches: Vec<LineMatch> = vec![];
            let result = format_line_matches(&cx, matches);
            assert!(result.is_empty());
        }

        #[test]
        fn test_format_line_matches_single_match() {
            let (temp, ctx, inv, re, c) = setup_test_ctx();
            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let test_file = temp.path().join("test.rs");
            let matches = vec![LineMatch {
                path: test_file,
                line_no: 42,
                line: "  some content here  ".to_string(),
            }];

            let result = format_line_matches(&cx, matches);
            assert_eq!(result.len(), 1);
            assert!(result[0].contains(":42:"));
            assert!(result[0].contains("some content here"));
        }

        #[test]
        fn test_format_line_matches_trims_trailing_whitespace() {
            let (temp, ctx, inv, re, c) = setup_test_ctx();
            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let test_file = temp.path().join("test.rs");
            let matches = vec![LineMatch {
                path: test_file,
                line_no: 1,
                line: "content with trailing spaces   \t  ".to_string(),
            }];

            let result = format_line_matches(&cx, matches);
            assert_eq!(result.len(), 1);
            // Should trim trailing whitespace
            assert!(result[0].ends_with("content with trailing spaces"));
        }

        #[test]
        fn test_format_line_matches_multiple_matches() {
            let (temp, ctx, inv, re, c) = setup_test_ctx();
            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let test_file = temp.path().join("test.rs");
            let matches = vec![
                LineMatch {
                    path: test_file.clone(),
                    line_no: 10,
                    line: "first line".to_string(),
                },
                LineMatch {
                    path: test_file.clone(),
                    line_no: 20,
                    line: "second line".to_string(),
                },
                LineMatch {
                    path: test_file,
                    line_no: 30,
                    line: "third line".to_string(),
                },
            ];

            let result = format_line_matches(&cx, matches);
            assert_eq!(result.len(), 3);
            assert!(result[0].contains(":10:"));
            assert!(result[1].contains(":20:"));
            assert!(result[2].contains(":30:"));
        }
    }

    // ==========================================================================
    // Tests for check_cleanup_uses_demoswarm_shim edge cases
    // ==========================================================================

    mod cleanup_shim_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_cleanup_agent(
            agent_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write cleanup agent file
            std::fs::write(agents_dir.join("test-cleanup.md"), agent_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_cleanup_agent_with_allowed_demoswarm_shim() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_cleanup_agent(
                r#"---
name: test-cleanup
---
# Test Cleanup Agent

Use the demoswarm.sh shim for operations:
```bash
bash .claude/scripts/demoswarm.sh count --file test.txt
```
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_cleanup_uses_demoswarm_shim(&cx, &mut rep);
            assert!(result.is_ok());
            // No violations expected since demoswarm.sh is used
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_cleanup_agent_with_do_not_context_allowed() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_cleanup_agent(
                r#"---
name: test-cleanup
---
# Test Cleanup Agent

# Do not use bespoke pipelines like this:
grep -c pattern file.txt

Instead use demoswarm.sh
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_cleanup_uses_demoswarm_shim(&cx, &mut rep);
            assert!(result.is_ok());
            // The "Do not" context should allow the grep pattern
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_cleanup_agent_empty_line_resets_context() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_cleanup_agent(
                r#"---
name: test-cleanup
---
# Test Cleanup Agent

# Do not use this

grep -c pattern file.txt
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_cleanup_uses_demoswarm_shim(&cx, &mut rep);
            assert!(result.is_ok());
            // Empty line resets the allowed context, so grep should be flagged
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_cleanup_agent_with_runs_reference_allowed() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_cleanup_agent(
                r#"---
name: test-cleanup
---
# Test Cleanup Agent

Use runs_ helper functions for derivation.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_cleanup_uses_demoswarm_shim(&cx, &mut rep);
            assert!(result.is_ok());
            // runs_ references are allowed
            assert_eq!(rep.errors, 0);
        }
    }

    // ==========================================================================
    // Tests for check_skill_ownership edge cases
    // ==========================================================================

    mod skill_ownership_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_agent(
            agent_name: &str,
            agent_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write agent file
            std::fs::write(agents_dir.join(format!("{}.md", agent_name)), agent_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_skill_ownership_index_allowed_for_cleanup() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                "signal-cleanup",
                r#"---
name: signal-cleanup
---
# Signal Cleanup

Use demoswarm.sh index upsert-status to update the index.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // signal-cleanup is allowed to use index upsert-status
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_skill_ownership_index_violation_for_unauthorized_agent() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                "code-implementer",
                r#"---
name: code-implementer
---
# Code Implementer

Use demoswarm.sh index upsert-status to update the index.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // code-implementer is NOT allowed to use index upsert-status
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_skill_ownership_secrets_allowed_for_sanitizer() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                "secrets-sanitizer",
                r#"---
name: secrets-sanitizer
---
# Secrets Sanitizer

Use demoswarm.sh secrets scan to find secrets.
Use demoswarm.sh secrets redact to remove them.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // secrets-sanitizer is allowed to use secrets commands
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_skill_ownership_secrets_violation_for_other_agent() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                "test-author",
                r#"---
name: test-author
---
# Test Author

Use demoswarm.sh secrets scan to check for secrets.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // test-author is NOT allowed to use secrets commands
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_skill_ownership_openq_allowed_for_clarifier() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                "clarifier",
                r#"---
name: clarifier
---
# Clarifier

Use demoswarm.sh openq next-id to get the next QID.
Use demoswarm.sh openq append to add questions.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // clarifier is allowed to use openq commands
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_skill_ownership_openq_violation_for_other_agent() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                "requirements-author",
                r#"---
name: requirements-author
---
# Requirements Author

Use demoswarm.sh openq append to add questions.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // requirements-author is NOT allowed to use openq commands
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_skill_ownership_unreadable_agent_skipped() {
            // This tests the continue path when file cannot be read
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Create agent file then remove read permissions (Unix only)
            // On Windows, we'll just test with empty dir
            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let mut rep = Reporter::new(OutputFormat::Json, false, false);

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skill_ownership(&cx, &mut rep);
            assert!(result.is_ok());
            // No agents to check, no errors
            assert_eq!(rep.errors, 0);
        }
    }

    // ==========================================================================
    // Tests for check_openq_prefix_validation
    // ==========================================================================

    mod openq_validation_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_runs(
            open_questions_content: Option<&str>,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");
            let runs_dir = temp.path().join(".runs").join("test-run").join("signal");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();
            std::fs::create_dir_all(&runs_dir).unwrap();

            if let Some(content) = open_questions_content {
                std::fs::write(runs_dir.join("open_questions.md"), content).unwrap();
            }

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_openq_validation_no_runs_directory() {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");

            std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
            std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
            std::fs::create_dir_all(claude_dir.join("skills")).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let mut rep = Reporter::new(OutputFormat::Json, false, false);

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // No .runs directory should pass with message
            assert_eq!(rep.errors, 0);
            assert_eq!(rep.warnings, 0);
        }

        #[test]
        fn test_openq_validation_valid_canonical_codes() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-SIG-001 - What is the scope?
- QID: OQ-PLN-002 - How to implement?
- QID: OQ-BLD-003 - Which tests?
- QID: OQ-GAT-004 - Gate criteria?
- QID: OQ-DEP-005 - Deploy target?
- QID: OQ-WIS-006 - Lessons learned?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // All canonical codes should pass
            assert_eq!(rep.warnings, 0);
        }

        #[test]
        fn test_openq_validation_non_canonical_flow_code_signal() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-SIGNAL-001 - What is the scope?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // SIGNAL instead of SIG should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_non_canonical_flow_code_plan() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-PLAN-001 - How to implement?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // PLAN instead of PLN should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_non_canonical_flow_code_build() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-BUILD-001 - Which tests?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // BUILD instead of BLD should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_non_canonical_flow_code_gate() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-GATE-001 - Gate criteria?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // GATE instead of GAT should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_non_canonical_flow_code_deploy() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-DEPLOY-001 - Deploy target?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // DEPLOY instead of DEP should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_non_canonical_flow_code_wisdom() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-WISDOM-001 - Lessons learned?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // WISDOM instead of WIS should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_unknown_uppercase_code() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-UNKNOWN-001 - What is this?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Unknown uppercase code should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_invalid_numeric_suffix_single_digit() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-SIG-1 - Single digit?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Single digit instead of 3 digits should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_invalid_numeric_suffix_two_digits() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-SIG-12 - Two digits?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Two digits instead of 3 should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_invalid_numeric_suffix_four_digits() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-SIG-1234 - Four digits?
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Four digits instead of 3 should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_multiple_qids_on_same_line() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

See also OQ-SIG-001 and OQ-PLAN-002 for related context.
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Both QIDs should be validated; PLAN should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_openq_validation_empty_open_questions_file() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(""));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Empty file should pass
            assert_eq!(rep.warnings, 0);
        }

        #[test]
        fn test_openq_validation_no_qid_lines() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

No questions yet.
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // No QIDs should pass
            assert_eq!(rep.warnings, 0);
        }

        #[test]
        fn test_openq_validation_deduplicates_violations() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_runs(Some(
                r#"# Open Questions

- QID: OQ-PLAN-001 - First
- QID: OQ-PLAN-001 - Duplicate
"#,
            ));

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_openq_prefix_validation(&cx, &mut rep);
            assert!(result.is_ok());
            // Duplicate violations should be deduplicated
            // Still expect warnings, but test that it doesn't fail
        }
    }

    // ==========================================================================
    // Tests for check_flow_boundary_enforcement
    // ==========================================================================

    mod flow_boundary_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_flow_command(
            flow_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write flow command file
            std::fs::write(commands_dir.join("flow-1-signal.md"), flow_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_flow_boundary_clean_flow_command() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_flow_command(
                r#"---
name: flow-1-signal
---
# Flow 1: Signal

Delegate to signal-run-prep agent.
Delegate to requirements-author agent.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_flow_boundary_enforcement(&cx, &mut rep);
            assert!(result.is_ok());
            // Clean flow command should have no warnings
            assert_eq!(rep.warnings, 0);
        }

        #[test]
        fn test_flow_boundary_demoswarm_shim_violation() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_flow_command(
                r#"---
name: flow-1-signal
---
# Flow 1: Signal

Run bash .claude/scripts/demoswarm.sh count --file test.txt
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_flow_boundary_enforcement(&cx, &mut rep);
            assert!(result.is_ok());
            // demoswarm.sh in flow command should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_flow_boundary_subcommand_violation() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_flow_command(
                r#"---
name: flow-1-signal
---
# Flow 1: Signal

Run `count pattern` to get the number.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_flow_boundary_enforcement(&cx, &mut rep);
            assert!(result.is_ok());
            // Skill CLI subcommand should trigger warning
            assert!(rep.warnings > 0);
        }

        #[test]
        fn test_flow_boundary_unreadable_file_skipped() {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");

            std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
            std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
            std::fs::create_dir_all(claude_dir.join("skills")).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let mut rep = Reporter::new(OutputFormat::Json, false, false);

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_flow_boundary_enforcement(&cx, &mut rep);
            assert!(result.is_ok());
            // No flow commands, should pass
            assert_eq!(rep.warnings, 0);
        }
    }

    // ==========================================================================
    // Tests for check_gh_body_hygiene
    // ==========================================================================

    mod gh_body_hygiene_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_gh_agent(
            agent_name: &str,
            agent_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write GH agent file
            std::fs::write(agents_dir.join(format!("{}.md", agent_name)), agent_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_gh_body_hygiene_clean_agent() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_gh_agent(
                "gh-reporter",
                r#"---
name: gh-reporter
---
# GH Reporter

Use heredoc for body:
```bash
gh api graphql -f body="$(cat <<'EOF'
Content here
EOF
)"
```
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_gh_body_hygiene(&cx, &mut rep);
            assert!(result.is_ok());
            // Clean agent should pass
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_gh_body_hygiene_forbidden_pattern_in_code_block_ok() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_gh_agent(
                "gh-reporter",
                r#"---
name: gh-reporter
---
# GH Reporter

Use heredoc for body:
```bash
# Example showing what NOT to do:
gh issue create --body-file /tmp/body.txt
```

But actually use -f body="$(cat <<'EOF'..."
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_gh_body_hygiene(&cx, &mut rep);
            assert!(result.is_ok());
            // Content inside code blocks should be skipped
        }

        #[test]
        fn test_gh_body_hygiene_do_not_context_allowed() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_gh_agent(
                "gh-reporter",
                r#"---
name: gh-reporter
---
# GH Reporter

Do NOT use --body-file as it fails on Windows paths.

Use -f body="$(cat <<'EOF'
content
EOF
)"
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_gh_body_hygiene(&cx, &mut rep);
            assert!(result.is_ok());
            // "Do NOT" context should allow the forbidden pattern mention
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_gh_body_hygiene_missing_agent_skipped() {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");

            std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
            std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
            std::fs::create_dir_all(claude_dir.join("skills")).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let mut rep = Reporter::new(OutputFormat::Json, false, false);

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_gh_body_hygiene(&cx, &mut rep);
            assert!(result.is_ok());
            // Missing agents should be skipped
            assert_eq!(rep.errors, 0);
        }
    }

    // ==========================================================================
    // Tests for check_shim_line_continuation
    // ==========================================================================

    mod shim_continuation_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_agent(
            agent_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write agent file
            std::fs::write(agents_dir.join("test-agent.md"), agent_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_shim_continuation_clean() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Use demoswarm.sh count --file test.txt
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_shim_line_continuation(&cx, &mut rep);
            assert!(result.is_ok());
            // No line continuation should pass
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_shim_continuation_violation() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

bash .claude/scripts/demoswarm.sh \
  count --file test.txt
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_shim_line_continuation(&cx, &mut rep);
            assert!(result.is_ok());
            // Line continuation after demoswarm.sh should fail
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_shim_continuation_empty_dirs() {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");

            std::fs::create_dir_all(claude_dir.join("agents")).unwrap();
            std::fs::create_dir_all(claude_dir.join("commands")).unwrap();
            std::fs::create_dir_all(claude_dir.join("skills")).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let mut rep = Reporter::new(OutputFormat::Json, false, false);

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_shim_line_continuation(&cx, &mut rep);
            assert!(result.is_ok());
            // Empty directories should pass
            assert_eq!(rep.errors, 0);
        }
    }

    // ==========================================================================
    // Tests for check_direct_demoswarm_invocation
    // ==========================================================================

    mod direct_invocation_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_agent(
            agent_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write agent file
            std::fs::write(agents_dir.join("test-agent.md"), agent_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_direct_invocation_via_shim_ok() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Use bash .claude/scripts/demoswarm.sh count --file test.txt
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_direct_demoswarm_invocation(&cx, &mut rep);
            assert!(result.is_ok());
            // Using shim should pass
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_direct_invocation_violation() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Run demoswarm count --file test.txt directly.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_direct_demoswarm_invocation(&cx, &mut rep);
            assert!(result.is_ok());
            // Direct invocation without shim should fail
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_direct_invocation_ms_subcommand() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Run demoswarm ms get --file test.txt directly.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_direct_demoswarm_invocation(&cx, &mut rep);
            assert!(result.is_ok());
            // Direct invocation with ms subcommand should fail
            assert!(rep.errors > 0);
        }
    }

    // ==========================================================================
    // Tests for check_skills_section_required
    // ==========================================================================

    mod skills_section_tests {
        use super::*;
        use crate::checks::CheckCtx;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use tempfile::TempDir;

        fn setup_test_env_with_agent(
            agent_content: &str,
        ) -> (TempDir, Ctx, Inventory, Regexes, Contracts, Reporter) {
            let temp = TempDir::new().unwrap();
            let claude_dir = temp.path().join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            std::fs::create_dir_all(&agents_dir).unwrap();
            std::fs::create_dir_all(&commands_dir).unwrap();
            std::fs::create_dir_all(&skills_dir).unwrap();

            // Write agent file
            std::fs::write(agents_dir.join("test-agent.md"), agent_content).unwrap();

            let ctx = Ctx::discover(Some(temp.path().to_path_buf())).unwrap();
            let inv = Inventory::from_ctx(&ctx).unwrap();
            let re = Regexes::compile().unwrap();
            let c = Contracts::default();
            let rep = Reporter::new(OutputFormat::Json, false, false);

            (temp, ctx, inv, re, c, rep)
        }

        #[test]
        fn test_skills_section_present() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Uses demoswarm.sh for operations.

## Skills

Uses the runs-derive skill.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skills_section_required(&cx, &mut rep);
            assert!(result.is_ok());
            // Agent with demoswarm.sh and Skills section should pass
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_skills_section_missing() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Uses demoswarm.sh for operations.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skills_section_required(&cx, &mut rep);
            assert!(result.is_ok());
            // Agent with demoswarm.sh but no Skills section should fail
            assert!(rep.errors > 0);
        }

        #[test]
        fn test_skills_section_not_required_without_demoswarm() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Does not use demoswarm.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skills_section_required(&cx, &mut rep);
            assert!(result.is_ok());
            // Agent without demoswarm.sh doesn't need Skills section
            assert_eq!(rep.errors, 0);
        }

        #[test]
        fn test_skills_section_singular_form_accepted() {
            let (_temp, ctx, inv, re, c, mut rep) = setup_test_env_with_agent(
                r#"---
name: test-agent
---
# Test Agent

Uses demoswarm.sh for operations.

## Skill

Uses the runs-derive skill.
"#,
            );

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };

            let result = check_skills_section_required(&cx, &mut rep);
            assert!(result.is_ok());
            // "## Skill" (singular) should also be accepted
            assert_eq!(rep.errors, 0);
        }
    }
}
