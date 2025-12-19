//! Flow command checks.
//!
//! Checks: 5, 11, 12, 13, 22, 25, 26, 27, 37, 43, 44, 45, 46, 47, 48, 49, 50

use super::contracts::headings;
use crate::reporter::Reporter;
use crate::util::{contains_ignore_ascii_case, extract_frontmatter_name, has_exact_line};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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
        CheckSpec {
            id: 48,
            title: "Checking ms get keys are documented by producers...",
            run: check_ms_get_key_contracts,
        },
        CheckSpec {
            id: 49,
            title: "Checking inv get marker contracts (consumer vs producer)...",
            run: check_inv_marker_contracts,
        },
        CheckSpec {
            id: 50,
            title: "Checking command docs reference declared agent names...",
            run: check_flow_agent_name_resolution,
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

/// Check 48: Every `demoswarm.sh ms get --key X` used by a consumer must be documented
/// in the producing agent's `## Machine Summary` schema for that artifact.
fn check_ms_get_key_contracts(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let agent_docs = load_agent_docs(cx)?;
    let outputs_to_agents = map_outputs_to_agents(&agent_docs)?;

    let mut seen = HashSet::<(String, String)>::new();
    let mut unresolved = Vec::new();
    let mut drift = Vec::new();

    for (consumer_name, consumer_content) in &agent_docs {
        for cap in cx.re.ms_get_invocation.captures_iter(consumer_content) {
            let file = cap.get(1).unwrap().as_str().to_string();
            let key = cap.get(2).unwrap().as_str().to_string();
            if !seen.insert((file.clone(), key.clone())) {
                continue;
            }

            let Some(producers) = outputs_to_agents.get(&file) else {
                unresolved.push(format!(
                    "Cannot map `{file}` to a producer (ms get expects key `{key}`; referenced in {consumer_name})"
                ));
                continue;
            };

            let mut ok = false;
            for producer in producers {
                let Some(producer_content) = agent_docs.get(producer) else {
                    continue;
                };
                if producer_machine_summary_has_key(producer_content, &key) {
                    ok = true;
                    break;
                }
            }

            if !ok {
                drift.push(format!(
                    "`{file}`: consumer expects Machine Summary key `{key}`, but producer(s) {} do not document it",
                    producers
                        .iter()
                        .map(|p| format!("`{p}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
    }

    if drift.is_empty() {
        rep.pass("ms get keys appear documented by producers");
    } else {
        for msg in drift {
            rep.fail(msg);
        }
    }

    for msg in unresolved {
        rep.warn(msg);
    }

    Ok(())
}

/// Check 49: Every `demoswarm.sh inv get --marker X` used by a consumer must match a
/// documented marker in the producing agent's contract for that artifact.
fn check_inv_marker_contracts(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let agent_docs = load_agent_docs(cx)?;
    let outputs_to_agents = map_outputs_to_agents(&agent_docs)?;

    let mut seen = HashSet::<(String, String)>::new();
    let mut unresolved = Vec::new();
    let mut drift = Vec::new();

    for (consumer_name, consumer_content) in &agent_docs {
        for cap in cx.re.inv_get_invocation.captures_iter(consumer_content) {
            let file = cap.get(1).unwrap().as_str().to_string();
            let marker = cap.get(2).unwrap().as_str().to_string();
            if !seen.insert((file.clone(), marker.clone())) {
                continue;
            }

            let Some(producers) = outputs_to_agents.get(&file) else {
                unresolved.push(format!(
                    "Cannot map `{file}` to a producer (inv get expects marker `{marker}`; referenced in {consumer_name})"
                ));
                continue;
            };

            let needle = format!("{marker}:");
            let mut ok = false;
            for producer in producers {
                let Some(producer_content) = agent_docs.get(producer) else {
                    continue;
                };
                if producer_content.contains(&needle) {
                    ok = true;
                    break;
                }
            }

            if !ok {
                drift.push(format!(
                    "`{file}`: consumer expects inventory marker `{marker}`, but producer(s) {} do not document `{needle}`",
                    producers
                        .iter()
                        .map(|p| format!("`{p}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
    }

    if drift.is_empty() {
        rep.pass("inv get markers appear documented by producers");
    } else {
        for msg in drift {
            rep.fail(msg);
        }
    }

    for msg in unresolved {
        rep.warn(msg);
    }

    Ok(())
}

/// Check 50: Command docs only reference declared agent names (frontmatter `name:`).
fn check_flow_agent_name_resolution(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let agent_docs = load_agent_docs(cx)?;
    let declared: HashSet<String> = agent_docs.keys().cloned().collect();

    let skill_names: HashSet<String> = cx
        .inv
        .skill_md_files
        .iter()
        .filter_map(|p| {
            p.parent()
                .and_then(|d| d.file_name())
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        })
        .collect();

    let re_checkbox = Regex::new(r"^\s*-\s*\[\s*[xX ]\s*\]\s*(.+)$")?;
    let re_call_order = Regex::new(r"^\s*\d+\)\s*(.+)$")?;
    let re_plus = Regex::new(r"\+\s+([a-z][a-z0-9-]+)")?;
    let re_arrow = Regex::new(r"->\s+([a-z][a-z0-9-]+)")?;
    let re_parens = Regex::new(r"\(([^)]*)\)")?;
    let re_token = Regex::new(r"[a-z][a-z0-9-]+")?;
    let re_backtick = Regex::new(r"`([a-z][a-z0-9-]+)`")?;

    for cmd in &cx.inv.command_md_files {
        let rel = cx.ctx.rel(cmd);
        let content = cx.ctx.read_utf8(cmd)?;

        let mut missing: HashMap<String, Vec<usize>> = HashMap::new();
        let mut in_agents_section = false;

        for (idx, line) in content.lines().enumerate() {
            let trimmed = line.trim_end();

            if let Some(heading) = trimmed.strip_prefix("## ") {
                let h = heading.trim().to_ascii_lowercase();
                in_agents_section = h == "agents to use" || h == "subagents to use";
            }

            let task_text = if let Some(caps) = re_checkbox.captures(line) {
                Some(caps.get(1).unwrap().as_str())
            } else {
                re_call_order
                    .captures(line)
                    .map(|caps| caps.get(1).unwrap().as_str())
            };

            if let Some(task_text) = task_text {
                for token in extract_agent_candidates(
                    task_text, &declared, &re_plus, &re_arrow, &re_parens, &re_token,
                ) {
                    if skill_names.contains(&token) {
                        continue;
                    }
                    if !declared.contains(&token) {
                        missing.entry(token).or_default().push(idx + 1);
                    }
                }
            }

            for cap in re_backtick.captures_iter(trimmed) {
                let token = cap.get(1).unwrap().as_str();
                if skill_names.contains(token) {
                    continue;
                }
                if is_agent_token_candidate(token, &declared) && !declared.contains(token) {
                    missing.entry(token.to_string()).or_default().push(idx + 1);
                }
            }

            if in_agents_section {
                for m in re_token.find_iter(trimmed) {
                    let token = m.as_str();
                    if skill_names.contains(token) {
                        continue;
                    }
                    if is_agent_token_candidate(token, &declared) && !declared.contains(token) {
                        missing.entry(token.to_string()).or_default().push(idx + 1);
                    }
                }
            }
        }

        if missing.is_empty() {
            rep.pass(format!("{rel} agent names resolve"));
            continue;
        }

        let mut missing_names: Vec<String> = missing.keys().cloned().collect();
        missing_names.sort();
        rep.fail(format!(
            "{rel} references missing agent name(s): {}",
            missing_names.join(", ")
        ));

        let mut details = Vec::new();
        for name in missing_names {
            if let Some(lines) = missing.get(&name) {
                let mut uniq = lines.clone();
                uniq.sort();
                uniq.dedup();
                let line_list = uniq
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                details.push(format!("`{name}` at line(s): {line_list}"));
            }
        }
        rep.indent_lines(details);
    }

    Ok(())
}

fn extract_agent_candidates(
    task_text: &str,
    declared: &HashSet<String>,
    re_plus: &Regex,
    re_arrow: &Regex,
    re_parens: &Regex,
    re_token: &Regex,
) -> HashSet<String> {
    let mut out = HashSet::<String>::new();

    // 1) Leading agent sequence (e.g., "a / b (microloop)", "a + b (parallel)", "a: ...")
    let leading = parse_agent_sequence_prefix(task_text, declared);
    let has_leading_agent = !leading.is_empty();
    for token in leading {
        out.insert(token);
    }

    // 2) "+ agent" occurrences (covers cases like "reseal + repo-operator restage")
    for cap in re_plus.captures_iter(task_text) {
        let token = cap.get(1).unwrap().as_str();
        if is_agent_token_candidate(token, declared) {
            out.insert(token.to_string());
        }
    }

    // 3) "-> agent" occurrences (covers reseal cycles like "cleanup -> sanitizer")
    for cap in re_arrow.captures_iter(task_text) {
        let token = cap.get(1).unwrap().as_str();
        if is_agent_token_candidate(token, declared) {
            out.insert(token.to_string());
        }
    }

    // 4) Parenthetical agent mentions are only parsed when the line doesn't already start
    // with an agent. This avoids false positives like "(project-defined)" or "(re-verify tests)".
    if !has_leading_agent {
        for cap in re_parens.captures_iter(task_text) {
            let inner = cap.get(1).unwrap().as_str();
            for m in re_token.find_iter(inner) {
                let token = m.as_str();
                if is_agent_token_candidate(token, declared) {
                    out.insert(token.to_string());
                }
            }
        }
    }

    // 5) Full token scan: catches trailing agents and non-ASCII separators (e.g., "a ↔ b").
    for m in re_token.find_iter(task_text) {
        let token = m.as_str();
        if is_agent_token_candidate(token, declared) {
            out.insert(token.to_string());
        }
    }

    out
}

fn parse_agent_sequence_prefix(s: &str, declared: &HashSet<String>) -> Vec<String> {
    let s = s.trim_start();
    let bytes = s.as_bytes();
    let mut i = 0;

    let start = i;
    while i < bytes.len() && is_agent_char(bytes[i]) {
        i += 1;
    }
    if i == start {
        return Vec::new();
    }

    let first = &s[start..i];
    if !is_agent_token_candidate(first, declared) {
        return Vec::new();
    }

    let mut out = vec![first.to_string()];

    loop {
        let mut j = i;
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }

        // Require whitespace before separator so we don't misread "run/<run-id>" style segments.
        if j == i || j >= bytes.len() {
            break;
        }

        let sep = bytes[j] as char;
        if sep != '/' && sep != '+' {
            break;
        }

        // Require whitespace after separator (same reason as above).
        let mut k = j + 1;
        while k < bytes.len() && bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        if k == j + 1 || k >= bytes.len() {
            break;
        }

        let start2 = k;
        while k < bytes.len() && is_agent_char(bytes[k]) {
            k += 1;
        }
        if k == start2 {
            break;
        }

        let token = &s[start2..k];
        if is_agent_token_candidate(token, declared) {
            out.push(token.to_string());
        }

        i = k;
    }

    out
}

fn is_agent_token_candidate(token: &str, declared: &HashSet<String>) -> bool {
    if declared.contains(token) {
        return true;
    }

    // Heuristic: avoid flagging hyphenated prose like "no-op" or "fix-forward" by requiring
    // a common agent suffix when the token is not declared.
    const SUFFIXES: [&str; 31] = [
        "-analyst",
        "-analyzer",
        "-assessor",
        "-auditor",
        "-author",
        "-checker",
        "-cleanup",
        "-critic",
        "-customizer",
        "-decider",
        "-designer",
        "-enforcer",
        "-executor",
        "-fixer",
        "-framer",
        "-historian",
        "-loader",
        "-manager",
        "-monitor",
        "-normalizer",
        "-operator",
        "-planner",
        "-prep",
        "-reporter",
        "-researcher",
        "-resolver",
        "-runner",
        "-sanitizer",
        "-synthesizer",
        "-triager",
        "-verifier",
    ];

    token.contains('-') && SUFFIXES.iter().any(|s| token.ends_with(s))
}

fn is_agent_char(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'
}

fn load_agent_docs(cx: &CheckCtx) -> anyhow::Result<HashMap<String, Arc<str>>> {
    let mut out: HashMap<String, Arc<str>> = HashMap::new();
    for path in &cx.inv.agent_md_files {
        let content = cx.ctx.read_utf8(path)?;
        let name = extract_frontmatter_name(&content).unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("<unknown>")
                .to_string()
        });
        out.insert(name, content);
    }
    Ok(out)
}

fn map_outputs_to_agents(
    agent_docs: &HashMap<String, Arc<str>>,
) -> anyhow::Result<HashMap<String, Vec<String>>> {
    let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+")?;

    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for (agent_name, content) in agent_docs {
        for output_file in extract_output_files(&runs_re, content) {
            for expanded in expand_flow_placeholders(&output_file) {
                out.entry(expanded).or_default().push(agent_name.clone());
            }
        }
    }
    Ok(out)
}

fn expand_flow_placeholders(path: &str) -> Vec<String> {
    const FLOWS: [&str; 6] = ["signal", "plan", "build", "gate", "deploy", "wisdom"];

    if path.contains("/<flow>/") {
        return FLOWS
            .iter()
            .map(|flow| path.replace("/<flow>/", &format!("/{flow}/")))
            .collect();
    }
    if path.contains("/<current-flow>/") {
        return FLOWS
            .iter()
            .map(|flow| path.replace("/<current-flow>/", &format!("/{flow}/")))
            .collect();
    }
    vec![path.to_string()]
}

fn extract_output_files(runs_re: &Regex, content: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::<String>::new();

    // 1) output_file: <path>
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("output_file:") {
            let path = rest.trim();
            if path.starts_with(".runs/<run-id>/") && seen.insert(path.to_string()) {
                out.push(path.to_string());
            }
        }
    }

    // 2) "## Output" section bullet list (common)
    let mut in_output = false;
    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.starts_with("## Output") || trimmed.starts_with("## Outputs") {
            in_output = true;
            continue;
        }
        if in_output && trimmed.starts_with("## ") {
            in_output = false;
        }
        if !in_output {
            continue;
        }
        for m in runs_re.find_iter(trimmed) {
            let path = m.as_str().to_string();
            if seen.insert(path.clone()) {
                out.push(path);
            }
        }
    }

    // 3) "Output format" lines that include a concrete path
    for line in content.lines() {
        if !line.contains("Output format") || !line.contains(".runs/<run-id>/") {
            continue;
        }
        for m in runs_re.find_iter(line) {
            let path = m.as_str().to_string();
            if seen.insert(path.clone()) {
                out.push(path);
            }
        }
    }

    // 4) Heuristic: treat "write" lines containing .runs/<run-id>/ as output declarations.
    for line in content.lines() {
        if !line.contains(".runs/<run-id>/") {
            continue;
        }
        if !line.to_ascii_lowercase().contains("write") {
            continue;
        }
        for m in runs_re.find_iter(line) {
            let path = m.as_str().to_string();
            if seen.insert(path.clone()) {
                out.push(path);
            }
        }
    }

    out
}

fn producer_machine_summary_has_key(content: &str, key: &str) -> bool {
    let segments: Vec<&str> = key.split('.').collect();
    for block in extract_machine_summary_blocks(content) {
        let mut all = true;
        for seg in &segments {
            if !block.lines().any(|l| {
                let t = l.trim_start();
                t.starts_with(seg) && t.get(seg.len()..).is_some_and(|rest| rest.starts_with(':'))
            }) {
                all = false;
                break;
            }
        }
        if all {
            return true;
        }
    }
    false
}

fn extract_machine_summary_blocks(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut blocks = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        if lines[i].trim_end() == headings::MACHINE_SUMMARY_H2 {
            let start = i + 1;
            i = start;
            while i < lines.len() && !lines[i].starts_with("## ") {
                i += 1;
            }
            blocks.push(lines[start..i].join("\n"));
            continue;
        }
        i += 1;
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // =========================================================================
    // Tests for is_agent_char
    // =========================================================================

    #[test]
    fn test_is_agent_char_lowercase_letters() {
        for c in b'a'..=b'z' {
            assert!(is_agent_char(c), "lowercase '{}' should be agent char", c as char);
        }
    }

    #[test]
    fn test_is_agent_char_digits() {
        for c in b'0'..=b'9' {
            assert!(is_agent_char(c), "digit '{}' should be agent char", c as char);
        }
    }

    #[test]
    fn test_is_agent_char_hyphen() {
        assert!(is_agent_char(b'-'), "hyphen should be agent char");
    }

    #[test]
    fn test_is_agent_char_rejects_uppercase() {
        for c in b'A'..=b'Z' {
            assert!(!is_agent_char(c), "uppercase '{}' should NOT be agent char", c as char);
        }
    }

    #[test]
    fn test_is_agent_char_rejects_special() {
        let special = [b'_', b'.', b'/', b'\\', b' ', b'\t', b'\n', b'@', b'#'];
        for c in special {
            assert!(!is_agent_char(c), "special '{}' should NOT be agent char", c as char);
        }
    }

    // =========================================================================
    // Tests for is_agent_token_candidate
    // =========================================================================

    #[test]
    fn test_is_agent_token_candidate_declared_agent() {
        let mut declared = HashSet::new();
        declared.insert("my-agent".to_string());

        assert!(is_agent_token_candidate("my-agent", &declared));
    }

    #[test]
    fn test_is_agent_token_candidate_known_suffix() {
        let declared = HashSet::new();

        // Should match agents with known suffixes even if not declared
        assert!(is_agent_token_candidate("test-author", &declared));
        assert!(is_agent_token_candidate("code-critic", &declared));
        assert!(is_agent_token_candidate("build-cleanup", &declared));
        assert!(is_agent_token_candidate("data-loader", &declared));
        assert!(is_agent_token_candidate("deploy-operator", &declared));
    }

    #[test]
    fn test_is_agent_token_candidate_rejects_prose() {
        let declared = HashSet::new();

        // Hyphenated prose words should NOT match (no known suffix)
        assert!(!is_agent_token_candidate("no-op", &declared));
        assert!(!is_agent_token_candidate("fix-forward", &declared));
        assert!(!is_agent_token_candidate("re-run", &declared));
        assert!(!is_agent_token_candidate("non-blocking", &declared));
    }

    #[test]
    fn test_is_agent_token_candidate_no_hyphen_no_suffix() {
        let declared = HashSet::new();

        // Single words without hyphen should not match
        assert!(!is_agent_token_candidate("agent", &declared));
        assert!(!is_agent_token_candidate("flow", &declared));
        assert!(!is_agent_token_candidate("test", &declared));
    }

    // =========================================================================
    // Tests for parse_agent_sequence_prefix
    // =========================================================================

    #[test]
    fn test_parse_agent_sequence_prefix_single_agent() {
        let mut declared = HashSet::new();
        declared.insert("test-author".to_string());

        let result = parse_agent_sequence_prefix("test-author: some task", &declared);
        assert_eq!(result, vec!["test-author"]);
    }

    #[test]
    fn test_parse_agent_sequence_prefix_slash_separated() {
        let mut declared = HashSet::new();
        declared.insert("test-author".to_string());
        declared.insert("test-critic".to_string());

        let result = parse_agent_sequence_prefix("test-author / test-critic (microloop)", &declared);
        assert_eq!(result, vec!["test-author", "test-critic"]);
    }

    #[test]
    fn test_parse_agent_sequence_prefix_plus_separated() {
        let mut declared = HashSet::new();
        declared.insert("code-critic".to_string());
        declared.insert("test-critic".to_string());

        let result = parse_agent_sequence_prefix("code-critic + test-critic (parallel)", &declared);
        assert_eq!(result, vec!["code-critic", "test-critic"]);
    }

    #[test]
    fn test_parse_agent_sequence_prefix_empty_input() {
        let declared = HashSet::new();

        let result = parse_agent_sequence_prefix("", &declared);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_agent_sequence_prefix_no_agent_at_start() {
        let declared = HashSet::new();

        // Starts with uppercase or non-agent token
        let result = parse_agent_sequence_prefix("Some task description", &declared);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_agent_sequence_prefix_whitespace_handling() {
        let mut declared = HashSet::new();
        declared.insert("test-author".to_string());

        // Leading whitespace should be trimmed
        let result = parse_agent_sequence_prefix("   test-author: task", &declared);
        assert_eq!(result, vec!["test-author"]);
    }

    #[test]
    fn test_parse_agent_sequence_prefix_no_whitespace_around_separator() {
        let mut declared = HashSet::new();
        declared.insert("test-author".to_string());
        declared.insert("test-critic".to_string());

        // Requires whitespace around separator to avoid matching "run/<run-id>" style paths
        let result = parse_agent_sequence_prefix("test-author/test-critic", &declared);
        // Should only get the first agent since there's no space around /
        assert_eq!(result, vec!["test-author"]);
    }

    // =========================================================================
    // Tests for extract_agent_candidates
    // =========================================================================

    #[test]
    fn test_extract_agent_candidates_plus_pattern() {
        let mut declared = HashSet::new();
        declared.insert("repo-operator".to_string());

        let re_plus = Regex::new(r"\+\s+([a-z][a-z0-9-]+)").unwrap();
        let re_arrow = Regex::new(r"->\s+([a-z][a-z0-9-]+)").unwrap();
        let re_parens = Regex::new(r"\(([^)]*)\)").unwrap();
        let re_token = Regex::new(r"[a-z][a-z0-9-]+").unwrap();

        let result = extract_agent_candidates(
            "reseal + repo-operator restage",
            &declared,
            &re_plus,
            &re_arrow,
            &re_parens,
            &re_token,
        );

        assert!(result.contains("repo-operator"));
    }

    #[test]
    fn test_extract_agent_candidates_arrow_pattern() {
        let mut declared = HashSet::new();
        declared.insert("secrets-sanitizer".to_string());

        let re_plus = Regex::new(r"\+\s+([a-z][a-z0-9-]+)").unwrap();
        let re_arrow = Regex::new(r"->\s+([a-z][a-z0-9-]+)").unwrap();
        let re_parens = Regex::new(r"\(([^)]*)\)").unwrap();
        let re_token = Regex::new(r"[a-z][a-z0-9-]+").unwrap();

        let result = extract_agent_candidates(
            "cleanup -> secrets-sanitizer",
            &declared,
            &re_plus,
            &re_arrow,
            &re_parens,
            &re_token,
        );

        assert!(result.contains("secrets-sanitizer"));
    }

    #[test]
    fn test_extract_agent_candidates_empty_input() {
        let declared = HashSet::new();

        let re_plus = Regex::new(r"\+\s+([a-z][a-z0-9-]+)").unwrap();
        let re_arrow = Regex::new(r"->\s+([a-z][a-z0-9-]+)").unwrap();
        let re_parens = Regex::new(r"\(([^)]*)\)").unwrap();
        let re_token = Regex::new(r"[a-z][a-z0-9-]+").unwrap();

        let result = extract_agent_candidates(
            "",
            &declared,
            &re_plus,
            &re_arrow,
            &re_parens,
            &re_token,
        );

        assert!(result.is_empty());
    }

    // =========================================================================
    // Tests for expand_flow_placeholders
    // =========================================================================

    #[test]
    fn test_expand_flow_placeholders_with_flow_placeholder() {
        let result = expand_flow_placeholders(".runs/<run-id>/<flow>/receipt.json");

        assert_eq!(result.len(), 6);
        assert!(result.contains(&".runs/<run-id>/signal/receipt.json".to_string()));
        assert!(result.contains(&".runs/<run-id>/plan/receipt.json".to_string()));
        assert!(result.contains(&".runs/<run-id>/build/receipt.json".to_string()));
        assert!(result.contains(&".runs/<run-id>/gate/receipt.json".to_string()));
        assert!(result.contains(&".runs/<run-id>/deploy/receipt.json".to_string()));
        assert!(result.contains(&".runs/<run-id>/wisdom/receipt.json".to_string()));
    }

    #[test]
    fn test_expand_flow_placeholders_with_current_flow_placeholder() {
        let result = expand_flow_placeholders(".runs/<run-id>/<current-flow>/status.md");

        assert_eq!(result.len(), 6);
        assert!(result.contains(&".runs/<run-id>/signal/status.md".to_string()));
        assert!(result.contains(&".runs/<run-id>/wisdom/status.md".to_string()));
    }

    #[test]
    fn test_expand_flow_placeholders_no_placeholder() {
        let result = expand_flow_placeholders(".runs/<run-id>/build/receipt.json");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ".runs/<run-id>/build/receipt.json");
    }

    #[test]
    fn test_expand_flow_placeholders_empty_path() {
        let result = expand_flow_placeholders("");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "");
    }

    // =========================================================================
    // Tests for extract_output_files
    // =========================================================================

    #[test]
    fn test_extract_output_files_output_file_line() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "output_file: .runs/<run-id>/build/receipt.json";

        let result = extract_output_files(&runs_re, content);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], ".runs/<run-id>/build/receipt.json");
    }

    #[test]
    fn test_extract_output_files_output_section() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = r#"
## Outputs

- `.runs/<run-id>/signal/requirements.md`
- `.runs/<run-id>/signal/features/main.feature`

## Other Section
"#;

        let result = extract_output_files(&runs_re, content);

        assert!(result.contains(&".runs/<run-id>/signal/requirements.md".to_string()));
        assert!(result.contains(&".runs/<run-id>/signal/features/main.feature".to_string()));
    }

    #[test]
    fn test_extract_output_files_deduplicates() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = r#"
output_file: .runs/<run-id>/build/receipt.json

## Output

- `.runs/<run-id>/build/receipt.json`
"#;

        let result = extract_output_files(&runs_re, content);

        // Should only have one entry despite being mentioned twice
        let receipt_count = result.iter()
            .filter(|p| *p == ".runs/<run-id>/build/receipt.json")
            .count();
        assert_eq!(receipt_count, 1);
    }

    #[test]
    fn test_extract_output_files_empty_content() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "";

        let result = extract_output_files(&runs_re, content);

        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_output_files_no_runs_paths() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "Some content without any .runs/ paths";

        let result = extract_output_files(&runs_re, content);

        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_output_files_write_lines() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "Write the output to .runs/<run-id>/build/summary.md";

        let result = extract_output_files(&runs_re, content);

        assert!(result.contains(&".runs/<run-id>/build/summary.md".to_string()));
    }

    // =========================================================================
    // Tests for extract_machine_summary_blocks
    // =========================================================================

    #[test]
    fn test_extract_machine_summary_blocks_single_block() {
        let content = r#"
# Agent Doc

## Machine Summary

status: VERIFIED
recommended_action: PROCEED

## Other Section
"#;

        let blocks = extract_machine_summary_blocks(content);

        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("status: VERIFIED"));
        assert!(blocks[0].contains("recommended_action: PROCEED"));
    }

    #[test]
    fn test_extract_machine_summary_blocks_multiple_blocks() {
        let content = r#"
## Machine Summary

status: VERIFIED

## Other Section

## Machine Summary

status: UNVERIFIED

## Final Section
"#;

        let blocks = extract_machine_summary_blocks(content);

        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].contains("status: VERIFIED"));
        assert!(blocks[1].contains("status: UNVERIFIED"));
    }

    #[test]
    fn test_extract_machine_summary_blocks_no_blocks() {
        let content = "# Doc without Machine Summary section";

        let blocks = extract_machine_summary_blocks(content);

        assert!(blocks.is_empty());
    }

    #[test]
    fn test_extract_machine_summary_blocks_at_end_of_file() {
        let content = r#"
# Doc

## Machine Summary

status: VERIFIED
blockers: []"#;

        let blocks = extract_machine_summary_blocks(content);

        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("status: VERIFIED"));
        assert!(blocks[0].contains("blockers: []"));
    }

    #[test]
    fn test_extract_machine_summary_blocks_empty_content() {
        let content = "";

        let blocks = extract_machine_summary_blocks(content);

        assert!(blocks.is_empty());
    }

    // =========================================================================
    // Tests for producer_machine_summary_has_key
    // =========================================================================

    #[test]
    fn test_producer_machine_summary_has_key_simple_key() {
        let content = r#"
## Machine Summary

status: VERIFIED
recommended_action: PROCEED

## Other
"#;

        assert!(producer_machine_summary_has_key(content, "status"));
        assert!(producer_machine_summary_has_key(content, "recommended_action"));
    }

    #[test]
    fn test_producer_machine_summary_has_key_nested_key() {
        let content = r#"
## Machine Summary

counts:
  tests_added: 5
  files_changed: 3

## Other
"#;

        assert!(producer_machine_summary_has_key(content, "counts.tests_added"));
        assert!(producer_machine_summary_has_key(content, "counts.files_changed"));
    }

    #[test]
    fn test_producer_machine_summary_has_key_missing_key() {
        let content = r#"
## Machine Summary

status: VERIFIED

## Other
"#;

        assert!(!producer_machine_summary_has_key(content, "missing_key"));
        assert!(!producer_machine_summary_has_key(content, "status.nested"));
    }

    #[test]
    fn test_producer_machine_summary_has_key_no_machine_summary() {
        let content = "# Doc without Machine Summary";

        assert!(!producer_machine_summary_has_key(content, "status"));
    }

    #[test]
    fn test_producer_machine_summary_has_key_partial_match() {
        let content = r#"
## Machine Summary

status_code: 200

## Other
"#;

        // "status" should not match "status_code" (must have colon after key)
        assert!(!producer_machine_summary_has_key(content, "status"));
        assert!(producer_machine_summary_has_key(content, "status_code"));
    }

    // =========================================================================
    // Tests for map_outputs_to_agents (edge cases)
    // =========================================================================

    #[test]
    fn test_map_outputs_to_agents_empty_docs() {
        let agent_docs: HashMap<String, Arc<str>> = HashMap::new();

        let result = map_outputs_to_agents(&agent_docs).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_map_outputs_to_agents_no_outputs() {
        let mut agent_docs: HashMap<String, Arc<str>> = HashMap::new();
        agent_docs.insert(
            "test-agent".to_string(),
            Arc::from("# Agent doc with no output paths"),
        );

        let result = map_outputs_to_agents(&agent_docs).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn test_map_outputs_to_agents_multiple_agents_same_output() {
        let mut agent_docs: HashMap<String, Arc<str>> = HashMap::new();
        agent_docs.insert(
            "agent-a".to_string(),
            Arc::from("output_file: .runs/<run-id>/build/receipt.json"),
        );
        agent_docs.insert(
            "agent-b".to_string(),
            Arc::from("output_file: .runs/<run-id>/build/receipt.json"),
        );

        let result = map_outputs_to_agents(&agent_docs).unwrap();

        let producers = result.get(".runs/<run-id>/build/receipt.json").unwrap();
        assert_eq!(producers.len(), 2);
        assert!(producers.contains(&"agent-a".to_string()));
        assert!(producers.contains(&"agent-b".to_string()));
    }

    // =========================================================================
    // Edge case tests for glob patterns and file operations
    // =========================================================================

    #[test]
    fn test_flow_placeholder_preserves_other_placeholders() {
        // Ensures <run-id> is preserved while <flow> is expanded
        let result = expand_flow_placeholders(".runs/<run-id>/<flow>/file.txt");

        for path in &result {
            assert!(path.contains("<run-id>"), "Should preserve <run-id> placeholder");
            assert!(!path.contains("<flow>"), "Should replace <flow> placeholder");
        }
    }

    #[test]
    fn test_extract_output_files_output_format_line() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "Output format: .runs/<run-id>/gate/decision.md";

        let result = extract_output_files(&runs_re, content);

        assert!(result.contains(&".runs/<run-id>/gate/decision.md".to_string()));
    }

    #[test]
    fn test_extract_output_files_multiple_paths_per_line() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "Write .runs/<run-id>/a.md and .runs/<run-id>/b.md";

        let result = extract_output_files(&runs_re, content);

        assert!(result.contains(&".runs/<run-id>/a.md".to_string()));
        assert!(result.contains(&".runs/<run-id>/b.md".to_string()));
    }

    // =========================================================================
    // Tests for agent sequence parsing edge cases
    // =========================================================================

    #[test]
    fn test_parse_agent_sequence_prefix_triple_agent() {
        let mut declared = HashSet::new();
        declared.insert("a-author".to_string());
        declared.insert("b-critic".to_string());
        declared.insert("c-cleanup".to_string());

        let result = parse_agent_sequence_prefix("a-author / b-critic / c-cleanup", &declared);

        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a-author".to_string()));
        assert!(result.contains(&"b-critic".to_string()));
        assert!(result.contains(&"c-cleanup".to_string()));
    }

    #[test]
    fn test_parse_agent_sequence_prefix_mixed_separators() {
        let mut declared = HashSet::new();
        declared.insert("a-author".to_string());
        declared.insert("b-critic".to_string());

        // Mixed separators - should parse first two
        let result = parse_agent_sequence_prefix("a-author / b-critic + extra", &declared);

        assert!(result.contains(&"a-author".to_string()));
        assert!(result.contains(&"b-critic".to_string()));
    }

    #[test]
    fn test_extract_agent_candidates_unicode_arrow() {
        let mut declared = HashSet::new();
        declared.insert("test-author".to_string());
        declared.insert("test-critic".to_string());

        let re_plus = Regex::new(r"\+\s+([a-z][a-z0-9-]+)").unwrap();
        let re_arrow = Regex::new(r"->\s+([a-z][a-z0-9-]+)").unwrap();
        let re_parens = Regex::new(r"\(([^)]*)\)").unwrap();
        let re_token = Regex::new(r"[a-z][a-z0-9-]+").unwrap();

        // Unicode arrow (↔) - full token scan should still find agents
        let result = extract_agent_candidates(
            "test-author ↔ test-critic",
            &declared,
            &re_plus,
            &re_arrow,
            &re_parens,
            &re_token,
        );

        assert!(result.contains("test-author"));
        assert!(result.contains("test-critic"));
    }

    // =========================================================================
    // Tests for missing file handling (simulated via empty collections)
    // =========================================================================

    #[test]
    fn test_load_agent_docs_with_empty_inventory_paths() {
        // This tests the behavior when the inventory has no agent files
        // The actual load_agent_docs requires a CheckCtx, but we can test
        // that map_outputs_to_agents handles empty input gracefully
        let empty_docs: HashMap<String, Arc<str>> = HashMap::new();
        let result = map_outputs_to_agents(&empty_docs).unwrap();
        assert!(result.is_empty());
    }

    // =========================================================================
    // Tests for Machine Summary key edge cases
    // =========================================================================

    #[test]
    fn test_producer_machine_summary_deeply_nested_key() {
        let content = r#"
## Machine Summary

severity_summary:
  critical: 0
  major: 1

## Other
"#;

        assert!(producer_machine_summary_has_key(content, "severity_summary.critical"));
        assert!(producer_machine_summary_has_key(content, "severity_summary.major"));
    }

    #[test]
    fn test_producer_machine_summary_key_with_special_values() {
        let content = r#"
## Machine Summary

route_to_agent: null
route_to_flow: null

## Other
"#;

        assert!(producer_machine_summary_has_key(content, "route_to_agent"));
        assert!(producer_machine_summary_has_key(content, "route_to_flow"));
    }

    #[test]
    fn test_extract_machine_summary_blocks_with_code_block() {
        let content = r#"
## Machine Summary

```yaml
status: VERIFIED
```

## Other
"#;

        let blocks = extract_machine_summary_blocks(content);

        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("status: VERIFIED"));
    }

    // =========================================================================
    // Tests for output file extraction edge cases
    // =========================================================================

    #[test]
    fn test_extract_output_files_ignores_non_runs_paths() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = r#"
output_file: /tmp/test.json
output_file: ./local/file.txt
output_file: .runs/<run-id>/valid/path.md
"#;

        let result = extract_output_files(&runs_re, content);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&".runs/<run-id>/valid/path.md".to_string()));
    }

    #[test]
    fn test_extract_output_files_handles_trailing_characters() {
        let runs_re = Regex::new(r"\.runs/<run-id>/[A-Za-z0-9_./<>-]+").unwrap();
        let content = "Write to .runs/<run-id>/build/file.md, then continue";

        let result = extract_output_files(&runs_re, content);

        // Should extract just the path, not the comma
        assert!(result.iter().any(|p| p.ends_with(".md")));
    }
}
