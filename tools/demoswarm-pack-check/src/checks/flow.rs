//! Flow command checks.
//!
//! Checks: 5, 11, 12, 13, 22, 25, 26, 27, 37, 43, 44, 45, 46, 47

use super::contracts::headings;
use crate::reporter::Reporter;
use crate::util::{contains_ignore_ascii_case, has_exact_line};

use super::{CheckCtx, CheckSpec};

pub fn checks() -> Vec<CheckSpec> {
    vec![
        CheckSpec {
            id: 5,
            title: "Checking flows reference sealing sequence (cleanup → secrets → repo-op → GH ops)...",
            run: check_sealing_sequence,
        },
        CheckSpec {
            id: 11,
            title: "Checking issue-first phrasing...",
            run: check_issue_first,
        },
        CheckSpec {
            id: 12,
            title: "Checking CANNOT_PROCEED semantics in cleanup agents...",
            run: check_cleanup_cannot_proceed,
        },
        CheckSpec {
            id: 13,
            title: "Checking reseal pattern in flow commands...",
            run: check_reseal_pattern,
        },
        CheckSpec {
            id: 22,
            title: "Checking decision spine marker contracts...",
            run: check_decision_spine,
        },
        CheckSpec {
            id: 25,
            title: "Checking decision spine extraction in plan-cleanup...",
            run: check_plan_cleanup_spine,
        },
        CheckSpec {
            id: 26,
            title: "Checking jq commands use single-line paths...",
            run: check_jq_paths,
        },
        CheckSpec {
            id: 27,
            title: "Checking Orchestrator Kickoff footers in flow commands...",
            run: check_kickoff_footer,
        },
        CheckSpec {
            id: 37,
            title: "Checking all control-plane agents (auto-detected)...",
            run: check_control_plane_agents,
        },
        CheckSpec {
            id: 43,
            title: "Checking subtask bridge contract...",
            run: check_subtask_bridge,
        },
        CheckSpec {
            id: 44,
            title: "Checking reseal non-convergence guard...",
            run: check_reseal_guard,
        },
        CheckSpec {
            id: 45,
            title: "Checking flow skill plumbing boundary...",
            run: check_flow_skill_plumbing,
        },
        CheckSpec {
            id: 46,
            title: "Checking agent docs have Skills section when using demoswarm.sh...",
            run: check_missing_skills_section,
        },
        CheckSpec {
            id: 47,
            title: "Checking flow output path patterns (advisory)...",
            run: check_flow_output_paths,
        },
    ]
}

/// Check 5: Flow commands have sealing sequence.
fn check_sealing_sequence(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        let mut missing = Vec::new();
        for marker in cx.c.sealing_markers {
            if !contains_ignore_ascii_case(&content, marker) {
                missing.push(*marker);
            }
        }

        if missing.is_empty() {
            rep.pass(format!("{flow_name} references sealing sequence"));
        } else {
            rep.warn(format!(
                "{flow_name} missing references: {}",
                missing.join(" ")
            ));
        }
    }

    Ok(())
}

/// Check 11: Issue-first phrasing.
fn check_issue_first(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(gh_reporter) = cx.inv.agent("gh-reporter") {
        let content = cx.ctx.read_utf8(gh_reporter)?;
        if content.contains(headings::ISSUE_FIRST_INVARIANT) {
            rep.pass("gh-reporter has issue-first invariant");
        } else {
            rep.fail("gh-reporter MISSING issue-first invariant section");
        }
    }

    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        let mut flagged = false;
        for line in content.lines() {
            if cx.re.pr_first.is_match(line) && !line.contains("(not PR)") {
                flagged = true;
                break;
            }
        }

        if flagged {
            rep.fail(format!(
                "{flow_name} has PR-first or ambiguous posting target"
            ));
        } else {
            rep.pass(format!("{flow_name} uses issue-first posting"));
        }
    }

    Ok(())
}

/// Check 12: CANNOT_PROCEED semantics in cleanup agents.
fn check_cleanup_cannot_proceed(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for path in &cx.inv.agent_md_files {
        let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if !name.ends_with("-cleanup.md") {
            continue;
        }

        let cleanup_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or(name);
        let content = cx.ctx.read_utf8(path)?;

        if cx.re.cannot_proceed_sem.is_match(&content) {
            if content.contains("missing_required") {
                rep.pass(format!(
                    "{cleanup_name} documents CANNOT_PROCEED as mechanical + missing_required"
                ));
            } else {
                rep.warn(format!(
                    "{cleanup_name} may be missing 'missing_required' documentation"
                ));
            }
        } else {
            rep.warn(format!(
                "{cleanup_name} may have unclear CANNOT_PROCEED semantics"
            ));
        }
    }

    Ok(())
}

/// Check 13: Reseal pattern in flow commands.
fn check_reseal_pattern(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        if cx.re.reseal_pattern.is_match(&content) {
            rep.pass(format!("{flow_name} references reseal-if-modified"));
        } else {
            rep.warn(format!("{flow_name} may be missing reseal-if-modified"));
        }
    }

    Ok(())
}

/// Check 22: Decision spine marker contracts.
fn check_decision_spine(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    // design-optioneer.md
    if let Some(design_optioneer) = cx.inv.agent("design-optioneer") {
        let content = cx.ctx.read_utf8(design_optioneer)?;
        let mut missing = Vec::new();
        if !content.contains(headings::MACHINE_SUMMARY_H2) {
            missing.push("## Machine Summary");
        }
        if !content.contains("TRADE_OFF") {
            missing.push("Requirements Fit enum (TRADE_OFF)");
        }

        if missing.is_empty() {
            rep.pass("design-optioneer.md has marker-level schema hints");
        } else {
            rep.warn(format!(
                "design-optioneer.md may be missing: {}",
                missing.join(" ")
            ));
        }
    } else {
        rep.fail("design-optioneer.md MISSING");
    }

    // adr-author.md
    if let Some(adr_author) = cx.inv.agent("adr-author") {
        let content = cx.ctx.read_utf8(adr_author)?;
        let mut issues = Vec::new();

        if !content.contains("Swarm-Proposed") {
            issues.push("Swarm-Proposed status");
        }
        if !content.contains("DRIVER:") {
            issues.push("DRIVER: marker lines");
        }
        if content.contains("drivers_bound") {
            issues.push("legacy drivers_bound field (should be removed)");
        }
        if !content.contains("ADR_CHOSEN_OPTION") {
            issues.push("ADR_CHOSEN_OPTION inventory marker");
        }

        if issues.is_empty() {
            rep.pass("adr-author.md matches marker-based ADR contract");
        } else {
            rep.fail(format!("adr-author.md drift: {}", issues.join(" ")));
        }
    } else {
        rep.fail("adr-author.md MISSING");
    }

    // design-critic.md
    if let Some(design_critic) = cx.inv.agent("design-critic") {
        let content = cx.ctx.read_utf8(design_critic)?;
        let mut issues = Vec::new();

        if !contains_ignore_ascii_case(&content, "Handshake") {
            issues.push("Handshake validation section");
        }
        if !content.contains(headings::ITERATION_CONTROL_H2) {
            issues.push("## Iteration Control section");
        }
        if content.contains("Enum column") {
            issues.push("legacy Enum column check (should be removed)");
        }
        if !content.contains("DC_CRITICAL:") {
            issues.push("DC_CRITICAL inventory marker");
        }

        if issues.is_empty() {
            rep.pass("design-critic.md matches updated handshake contract (semantic + markers)");
        } else {
            rep.fail(format!("design-critic.md drift: {}", issues.join(" ")));
        }
    } else {
        rep.fail("design-critic.md MISSING");
    }

    Ok(())
}

/// Check 25: Decision spine extraction in plan-cleanup.
fn check_plan_cleanup_spine(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(plan_cleanup) = cx.inv.agent("plan-cleanup") {
        let content = cx.ctx.read_utf8(plan_cleanup)?;
        let mut missing = Vec::new();

        if !content.contains(headings::DECISION_SPINE) {
            missing.push("Decision Spine section");
        }
        if !content.contains("decision_spine") {
            missing.push("decision_spine in receipt schema");
        }
        if !cx.re.spine_marker.is_match(&content) {
            missing.push("marker-based ADR extraction (ADR_CHOSEN_OPTION/ADR_DRIVER/DRIVER:)");
        }

        if missing.is_empty() {
            rep.pass("plan-cleanup.md has marker-based decision spine extraction");
        } else {
            rep.warn(format!(
                "plan-cleanup.md may be missing: {}",
                missing.join(" ")
            ));
        }
    }

    Ok(())
}

/// Check 26: jq commands use single-line paths.
fn check_jq_paths(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut split_jq_files = Vec::new();
    for cmd in &cx.inv.flow_cmd_files {
        let content = cx.ctx.read_utf8(cmd)?;

        let mut any_jq_runs = false;
        for line in content.lines() {
            if cx.re.jq_has_runs.is_match(line) {
                any_jq_runs = true;
                break;
            }
        }
        if !any_jq_runs {
            continue;
        }

        let mut flagged = false;
        for line in content.lines() {
            if cx.re.jq_quote.is_match(line) && !line.contains(".runs/") {
                flagged = true;
                break;
            }
        }

        if flagged {
            split_jq_files.push(cx.ctx.rel(cmd));
        }
    }

    if split_jq_files.is_empty() {
        rep.pass("jq commands appear to use single-line paths");
    } else {
        for f in &split_jq_files {
            rep.indent_lines([format!("{f} may have split jq path")]);
        }
        rep.warn("Some jq commands may have multiline path issues");
    }

    Ok(())
}

/// Check 27: Orchestrator Kickoff footer in flow commands.
fn check_kickoff_footer(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        let mut missing = Vec::new();

        if !content.contains(headings::ORCHESTRATOR_KICKOFF_H2) {
            missing.push("## Orchestrator Kickoff");
        }
        if !content.contains(headings::TODOWRITE_H3) {
            missing.push("### TodoWrite (copy exactly)");
        }

        if content.contains("safe_to_publish") && content.contains("proceed_to_github_ops") {
            if !cx.re.both_gates_same_line.is_match(&content) {
                missing.push("gating line with both gates");
            }
        } else {
            missing.push("gating line with both gates");
        }

        if missing.is_empty() {
            rep.pass(format!("{flow_name} has Orchestrator Kickoff footer"));
        } else {
            rep.fail(format!("{flow_name} missing: {}", missing.join(" ")));
        }
    }

    Ok(())
}

/// Check 37: Auto-detect control-plane agents.
fn check_control_plane_agents(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut detected_count = 0usize;

    for path in &cx.inv.agent_md_files {
        let content = cx.ctx.read_utf8(path)?;
        if !has_exact_line(&content, headings::MACHINE_SUMMARY_H2) {
            continue;
        }

        detected_count += 1;
        let agent = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");

        let mut issues = Vec::new();
        if !cx.re.canon_status.is_match(&content) {
            issues.push("missing canonical status line");
        }
        if !cx.re.canon_action.is_match(&content) {
            issues.push("missing canonical recommended_action line");
        }
        if !cx.re.route_to_agent.is_match(&content) {
            issues.push("missing route_to_agent");
        }
        if !cx.re.route_to_flow.is_match(&content) {
            issues.push("missing route_to_flow");
        }

        if issues.is_empty() {
            rep.pass(format!("{agent} has complete control-plane contract"));
        } else {
            rep.fail(format!(
                "{agent} control-plane issues: {}",
                issues.join(" ")
            ));
        }
    }

    rep.indent_lines([format!(
        "(Detected {detected_count} agents with '## Machine Summary')"
    )]);

    Ok(())
}

/// Check 43: Subtask bridge contract.
fn check_subtask_bridge(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(work_planner) = cx.inv.agent("work-planner") {
        let content = cx.ctx.read_utf8(work_planner)?;

        if content.contains("subtasks.yaml") {
            rep.pass("work-planner.md mentions subtasks.yaml output");
        } else {
            rep.fail(
                "work-planner.md does NOT mention subtasks.yaml (required for codetask bridge)",
            );
        }

        if cx.re.status_enum.is_match(&content) {
            rep.pass("work-planner.md documents subtask status enum (TODO | DOING | DONE)");
        } else {
            rep.fail("work-planner.md missing subtask status enum (TODO | DOING | DONE)");
        }

        if content.contains("allow_new_files_under") {
            rep.pass("work-planner.md documents allow_new_files_under scope hint");
        } else {
            rep.fail("work-planner.md missing allow_new_files_under scope hint");
        }
    }

    if let Some(context_loader) = cx.inv.agent("context-loader") {
        let content = cx.ctx.read_utf8(context_loader)?;

        if content.contains("subtasks.yaml") {
            rep.pass("context-loader.md references subtasks.yaml");
        } else {
            rep.fail("context-loader.md does NOT reference subtasks.yaml");
        }

        if cx.re.precedence.is_match(&content) && cx.re.todo_status.is_match(&content) {
            rep.pass("context-loader.md documents subtask selection precedence");
        } else {
            rep.fail(
                "context-loader.md missing selection precedence (explicit id → next TODO → fallback)",
            );
        }

        if content.contains("allow_new_files_under") {
            rep.pass("context-loader.md manifest includes allow_new_files_under");
        } else {
            rep.fail("context-loader.md manifest missing allow_new_files_under");
        }
    }

    Ok(())
}

/// Check 44: Reseal non-convergence guard.
fn check_reseal_guard(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut reseal_guard_found = 0usize;
    for cmd in &cx.inv.flow_cmd_files {
        let content = cx.ctx.read_utf8(cmd)?;
        if cx.re.reseal_guard.is_match(&content) {
            reseal_guard_found += 1;
        }
    }

    if reseal_guard_found >= 4 {
        rep.pass(format!(
            "Most flows have deterministic reseal guard ({reseal_guard_found}/6)"
        ));
    } else {
        rep.warn(format!(
            "Fewer than 4 flows have explicit reseal non-convergence guard (found: {reseal_guard_found})"
        ));
    }

    Ok(())
}

/// Check 45: Flow Skill Plumbing boundary.
///
/// Flow commands should not reference skill names directly or mention demoswarm.sh.
/// Skills are agent-level implementation details, not flow-level concepts.
fn check_flow_skill_plumbing(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut flagged_files = Vec::new();

    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        let mut issues = Vec::new();

        // Check for skill names in prose
        if cx.re.skill_names_in_prose.is_match(&content) {
            issues.push("skill name references");
        }

        // Check for demoswarm.sh references
        if cx.re.demoswarm_shim_ref.is_match(&content) {
            issues.push("demoswarm.sh references");
        }

        if !issues.is_empty() {
            flagged_files.push(format!("{flow_name}: {}", issues.join(", ")));
        }
    }

    if flagged_files.is_empty() {
        rep.pass("Flow commands do not leak skill plumbing");
    } else {
        for f in &flagged_files {
            rep.indent_lines([f.clone()]);
        }
        rep.fail("Flow commands contain skill plumbing (should be agent-level)");
    }

    Ok(())
}

/// Check 46: Missing Skills section in agent docs.
///
/// If an agent doc references demoswarm.sh, it should have a ## Skills section.
fn check_missing_skills_section(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut missing_section = Vec::new();

    for path in &cx.inv.agent_md_files {
        let agent = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(path)?;

        // Check if agent uses demoswarm.sh
        if cx.re.demoswarm_shim_ref.is_match(&content) {
            // Check if it has a Skills section
            if !content.contains("## Skills") {
                missing_section.push(agent.to_string());
            }
        }
    }

    if missing_section.is_empty() {
        rep.pass("Agents using demoswarm.sh have Skills section");
    } else {
        for agent in &missing_section {
            rep.indent_lines([format!(
                "{agent} uses demoswarm.sh but lacks ## Skills section"
            )]);
        }
        rep.warn("Some agents using demoswarm.sh lack Skills section");
    }

    Ok(())
}

/// Check 47: Flow output path patterns (advisory).
///
/// Scans flow commands for patterns like "agent -> .runs/" or "agent -> file"
/// which may indicate documentation of agent file outputs (informational only).
fn check_flow_output_paths(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut found_patterns = Vec::new();

    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        if cx.re.flow_output_arrow.is_match(&content) {
            found_patterns.push(flow_name.to_string());
        }
    }

    if found_patterns.is_empty() {
        rep.pass("No flow output arrow patterns detected");
    } else {
        for flow in &found_patterns {
            rep.indent_lines([format!("{flow} has output arrow patterns")]);
        }
        rep.warn("Flow commands have output arrow patterns (advisory: consider if this leaks implementation)");
    }

    Ok(())
}
