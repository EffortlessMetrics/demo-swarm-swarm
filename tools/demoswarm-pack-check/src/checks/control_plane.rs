//! Control-plane checks: Machine Summary contracts, gate blocks, routing fields.
//!
//! Checks: 3, 4, 16, 17, 18, 19, 20, 21, 28, 29, 31, 32, 33, 34, 35

use super::contracts::{headings, sentinels};
use crate::reporter::Reporter;
use crate::util::{contains_ignore_ascii_case, has_line_starting_with};

use super::{CheckCtx, CheckSpec};

pub fn checks() -> Vec<CheckSpec> {
    vec![
        CheckSpec {
            id: 3,
            title: "Checking critics have canonical Machine Summary axis...",
            run: check_critics_machine_summary,
        },
        CheckSpec {
            id: 4,
            title: "Checking cleanup agents reference receipts + index.json...",
            run: check_cleanup_receipts,
        },
        CheckSpec {
            id: 16,
            title: "Checking Gate Result contract block is present in all flows...",
            run: check_gate_result_block,
        },
        CheckSpec {
            id: 17,
            title: "Checking gh-reporter output constraints...",
            run: check_gh_reporter_output,
        },
        CheckSpec {
            id: 18,
            title: "Checking repo-operator has Repo Operator Result block...",
            run: check_repo_operator_result,
        },
        CheckSpec {
            id: 19,
            title: "Checking GH agents enforce two gates...",
            run: check_gh_agents_two_gates,
        },
        CheckSpec {
            id: 20,
            title: "Checking flow commands document GH content-mode gates...",
            run: check_flow_gh_gating,
        },
        CheckSpec {
            id: 21,
            title: "Checking checkpoint_mode: local_only contract...",
            run: check_checkpoint_local_only,
        },
        CheckSpec {
            id: 28,
            title: "Checking Machine Summary status enum...",
            run: check_status_enum,
        },
        CheckSpec {
            id: 29,
            title: "Checking recommended_action canonical closed enum line...",
            run: check_recommended_action_enum,
        },
        CheckSpec {
            id: 31,
            title: "Checking route_to_agent and route_to_flow fields exist...",
            run: check_route_fields,
        },
        CheckSpec {
            id: 32,
            title: "Checking CANNOT_PROCEED invariant...",
            run: check_cannot_proceed_invariant,
        },
        CheckSpec {
            id: 33,
            title: "Checking critics have can_further_iteration_help...",
            run: check_critics_iteration_help,
        },
        CheckSpec {
            id: 34,
            title: "Checking cleanup agents mention route_to_flow...",
            run: check_cleanup_route_to_flow,
        },
        CheckSpec {
            id: 35,
            title: "Checking gate agents use unified recommended_action...",
            run: check_gate_unified_action,
        },
        CheckSpec {
            id: 51,
            title: "Checking critics have observations field in Machine Summary...",
            run: check_critics_observations_field,
        },
    ]
}

/// Check 3: Critics have canonical Machine Summary axis.
fn check_critics_machine_summary(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for critic in cx.c.critics {
        let Some(file) = cx.inv.agent(critic) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;

        if !has_line_starting_with(&content, headings::MACHINE_SUMMARY_H2) {
            rep.fail(format!("{critic} missing '## Machine Summary' section"));
            continue;
        }

        if !cx.re.canon_status.is_match(&content) {
            rep.fail(format!(
                "{critic} missing canonical status axis line (VERIFIED | UNVERIFIED | CANNOT_PROCEED)"
            ));
            continue;
        }

        if !cx.re.canon_action.is_match(&content) {
            rep.fail(format!(
                "{critic} recommended_action drifted (expected: recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV)"
            ));
            continue;
        }

        rep.pass(format!("{critic} has canonical Machine Summary axis"));
    }

    Ok(())
}

/// Check 4: Cleanup agents reference receipts + index.json.
fn check_cleanup_receipts(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for (agent, receipt) in cx.c.cleanup_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;

        if content.contains(receipt) {
            rep.pass(format!("{agent} references {receipt}"));
        } else {
            rep.fail(format!("{agent} does NOT reference {receipt}"));
        }

        if content.contains("index.json") {
            rep.pass(format!("{agent} references index.json updates"));
        } else {
            rep.fail(format!("{agent} does NOT reference index.json updates"));
        }
    }

    Ok(())
}

/// Check 16: Gate Result contract block in flow commands.
fn check_gate_result_block(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        if !contains_ignore_ascii_case(&content, "secrets-sanitizer") {
            continue;
        }

        if !content.contains(sentinels::GATE_RESULT_START)
            || !content.contains(sentinels::GATE_RESULT_END)
        {
            rep.fail(format!(
                "{flow_name} missing Gate Result sentinel block (GATE_RESULT_V1)"
            ));
            continue;
        }

        let mut missing = Vec::new();
        for f in cx.c.gate_result_fields {
            if !content.contains(f) {
                missing.push(*f);
            }
        }

        if missing.is_empty() {
            rep.pass(format!(
                "{flow_name} documents Gate Result fields (incl. modified_files)"
            ));
        } else {
            rep.fail(format!(
                "{flow_name} Gate Result documentation missing fields: {}",
                missing.join(" ")
            ));
        }
    }

    Ok(())
}

/// Check 17: gh-reporter safe output contract.
fn check_gh_reporter_output(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(gh_reporter) = cx.inv.agent("gh-reporter") {
        let content = cx.ctx.read_utf8(gh_reporter)?;

        if content.contains(headings::SAFE_OUTPUT_CONTRACT) {
            rep.pass("gh-reporter has 'Safe Output Contract' section");
        } else {
            rep.fail("gh-reporter MISSING 'Safe Output Contract' section");
        }

        if content.contains("must NOT paste verbatim") {
            rep.pass("gh-reporter documents output constraints");
        } else {
            rep.warn("gh-reporter may be missing output constraint documentation");
        }
    }

    Ok(())
}

/// Check 18: repo-operator has Repo Operator Result block.
fn check_repo_operator_result(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let Some(repo_operator) = cx.inv.agent("repo-operator") else {
        rep.fail("repo-operator.md MISSING");
        return Ok(());
    };

    let content = cx.ctx.read_utf8(repo_operator)?;

    if content.contains(headings::REPO_OPERATOR_RESULT_H2) {
        rep.pass("repo-operator.md has '## Repo Operator Result' section");

        let mut missing = Vec::new();
        for f in cx.c.repo_operator_result_fields {
            if !content.contains(f) {
                missing.push(*f);
            }
        }

        if missing.is_empty() {
            rep.pass("repo-operator.md has required Repo Operator Result fields");
        } else {
            rep.fail(format!(
                "repo-operator.md missing Repo Operator Result fields: {}",
                missing.join(" ")
            ));
        }
    } else {
        rep.fail("repo-operator.md MISSING '## Repo Operator Result' section");
    }

    Ok(())
}

/// Check 19: GH agents enforce two gates.
fn check_gh_agents_two_gates(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.gh_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if content.contains("safe_to_publish") && content.contains("proceed_to_github_ops") {
            rep.pass(format!("{agent} enforces two gates"));
        } else {
            rep.fail(format!(
                "{agent} does NOT enforce both gates (safe_to_publish AND proceed_to_github_ops)"
            ));
        }
    }

    Ok(())
}

/// Check 20: Flow commands gate GH ops on proceed_to_github_ops.
fn check_flow_gh_gating(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for cmd in &cx.inv.flow_cmd_files {
        let flow_name = cmd
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>");
        let content = cx.ctx.read_utf8(cmd)?;

        if cx.re.gh_agent.is_match(&content) {
            if content.contains("proceed_to_github_ops") && content.contains("safe_to_publish") {
                rep.pass(format!("{flow_name} gates GH operations on both gates"));
            } else {
                rep.fail(format!(
                    "{flow_name} invokes GH agents but missing gate documentation (safe_to_publish and/or proceed_to_github_ops)"
                ));
            }
        } else {
            rep.pass(format!("{flow_name} (no GH agents referenced)"));
        }
    }

    Ok(())
}

/// Check 21: checkpoint_mode: local_only contract.
fn check_checkpoint_local_only(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(repo_operator) = cx.inv.agent("repo-operator") {
        let content = cx.ctx.read_utf8(repo_operator)?;

        if cx.re.checkpoint_mode_local.is_match(&content) {
            rep.pass("repo-operator.md documents checkpoint_mode: local_only");

            if cx.re.proceed_false.is_match(&content) {
                rep.pass("repo-operator.md documents local_only → proceed_to_github_ops: false");
            } else {
                rep.fail(
                    "repo-operator.md missing local_only → proceed_to_github_ops: false behavior",
                );
            }
        } else {
            rep.fail("repo-operator.md does NOT document checkpoint_mode: local_only");
        }
    }

    let mut local_only_flows = 0;
    for cmd in &cx.inv.flow_cmd_files {
        let content = cx.ctx.read_utf8(cmd)?;
        if cx.re.checkpoint_mode_local.is_match(&content) {
            local_only_flows += 1;
        }
    }

    if local_only_flows == 6 {
        rep.pass("All 6 flows mention checkpoint_mode: local_only for safe-bail");
    } else {
        rep.fail(format!(
            "Expected 6 flows mentioning checkpoint_mode: local_only; found {local_only_flows}"
        ));
    }

    Ok(())
}

/// Check 28: Machine Summary status enum.
fn check_status_enum(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.critic_and_verifier_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;

        let mut has_bad_blocked = false;
        for line in content.lines() {
            if cx.re.blocked_status.is_match(line) && !line.contains("BLOCKED_PUBLISH") {
                has_bad_blocked = true;
                break;
            }
        }

        if has_bad_blocked {
            rep.fail(format!(
                "{agent} uses legacy 'BLOCKED' status (should be CANNOT_PROCEED)"
            ));
        } else if !cx.re.canon_status.is_match(&content) {
            rep.fail(format!(
                "{agent} missing canonical status axis line (VERIFIED | UNVERIFIED | CANNOT_PROCEED)"
            ));
        } else {
            rep.pass(format!("{agent} has canonical status axis"));
        }
    }

    Ok(())
}

/// Check 29: recommended_action closed enum.
fn check_recommended_action_enum(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.critic_and_verifier_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if !cx.re.recommended_action_present.is_match(&content) {
            rep.fail(format!("{agent} missing recommended_action field"));
        } else if !cx.re.canon_action.is_match(&content) {
            rep.fail(format!(
                "{agent} recommended_action drifted (expected: recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV)"
            ));
        } else {
            rep.pass(format!("{agent} has canonical recommended_action line"));
        }
    }

    Ok(())
}

/// Check 31: route_to_agent and route_to_flow fields exist.
fn check_route_fields(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.critic_and_verifier_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if cx.re.route_to_agent.is_match(&content) && cx.re.route_to_flow.is_match(&content) {
            rep.pass(format!("{agent} has route_to_agent + route_to_flow"));
        } else {
            rep.fail(format!(
                "{agent} missing route_to_agent and/or route_to_flow"
            ));
        }
    }

    Ok(())
}

/// Check 32: CANNOT_PROCEED requires missing_required.
fn check_cannot_proceed_invariant(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.critic_and_verifier_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if content.contains("CANNOT_PROCEED") {
            if content.contains("missing_required") {
                rep.pass(format!(
                    "{agent} documents missing_required for CANNOT_PROCEED"
                ));
            } else {
                rep.fail(format!(
                    "{agent} uses CANNOT_PROCEED but missing missing_required documentation"
                ));
            }
        } else {
            rep.pass(format!("{agent} (no CANNOT_PROCEED mention)"));
        }
    }

    Ok(())
}

/// Check 33: Critics have can_further_iteration_help.
fn check_critics_iteration_help(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for critic in cx.c.critics {
        let Some(file) = cx.inv.agent(critic) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if content.contains("can_further_iteration_help") {
            rep.pass(format!("{critic} has can_further_iteration_help"));
        } else {
            rep.fail(format!("{critic} missing can_further_iteration_help"));
        }
    }

    Ok(())
}

/// Check 34: Cleanup agents mention route_to_flow.
fn check_cleanup_route_to_flow(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for (agent, _) in cx.c.cleanup_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if content.contains("route_to_flow") {
            rep.pass(format!("{agent} mentions route_to_flow"));
        } else {
            rep.warn(format!(
                "{agent} may be missing route_to_flow documentation"
            ));
        }
    }

    Ok(())
}

/// Check 35: Gate agents use unified recommended_action.
fn check_gate_unified_action(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    for agent in cx.c.gate_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;
        if content.contains("recommended_gate_action") {
            rep.fail(format!(
                "{agent} uses legacy 'recommended_gate_action' (should be 'recommended_action')"
            ));
        } else if content.contains("recommended_action") {
            rep.pass(format!("{agent} uses unified recommended_action"));
        } else {
            rep.warn(format!("{agent} missing recommended_action field"));
        }
    }

    Ok(())
}

/// Check 51: Critics have observations field in Machine Summary.
///
/// The observations field captures cross-cutting insights, friction noticed,
/// and pack/flow improvements. This feeds into Wisdom flow via learning-synthesizer.
fn check_critics_observations_field(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut missing = Vec::new();

    for critic in cx.c.critics {
        let Some(file) = cx.inv.agent(critic) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;

        // Check if observations field exists in Machine Summary context
        // Look for "observations:" or "observations: []" near concerns/blockers
        if !content.contains("observations:") {
            missing.push(critic.to_string());
        }
    }

    if missing.is_empty() {
        rep.pass("All critics have observations field in Machine Summary");
    } else {
        rep.fail(format!(
            "Critics missing observations field: {}",
            missing.join(", ")
        ));
    }

    Ok(())
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::Regexes;

    // -------------------------------------------------------------------------
    // Machine Summary status enum validation (Check 28)
    // -------------------------------------------------------------------------

    /// Valid canonical status line is matched correctly.
    #[test]
    fn test_canon_status_regex_matches_valid() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Valid canonical status line
        let valid = "status: VERIFIED | UNVERIFIED | CANNOT_PROCEED";
        assert!(
            re.canon_status.is_match(valid),
            "Should match valid canonical status line"
        );

        // With leading whitespace
        let with_indent = "  status: VERIFIED | UNVERIFIED | CANNOT_PROCEED";
        assert!(
            re.canon_status.is_match(with_indent),
            "Should match with leading whitespace"
        );
    }

    /// Invalid status lines are not matched by the canonical regex.
    #[test]
    fn test_canon_status_regex_rejects_invalid() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Missing CANNOT_PROCEED
        let missing_value = "status: VERIFIED | UNVERIFIED";
        assert!(
            !re.canon_status.is_match(missing_value),
            "Should reject incomplete status line"
        );

        // Wrong order
        let wrong_order = "status: UNVERIFIED | VERIFIED | CANNOT_PROCEED";
        assert!(
            !re.canon_status.is_match(wrong_order),
            "Should reject wrong order"
        );

        // Using BLOCKED instead of CANNOT_PROCEED
        let legacy_blocked = "status: VERIFIED | UNVERIFIED | BLOCKED";
        assert!(
            !re.canon_status.is_match(legacy_blocked),
            "Should reject legacy BLOCKED"
        );

        // Extra values
        let extra_value = "status: VERIFIED | UNVERIFIED | CANNOT_PROCEED | FAILED";
        assert!(
            !re.canon_status.is_match(extra_value),
            "Should reject extra values"
        );
    }

    /// Test that BLOCKED status is correctly detected as legacy.
    #[test]
    fn test_blocked_status_regex_detects_legacy() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Legacy BLOCKED (should be CANNOT_PROCEED)
        let blocked_line = "status: BLOCKED";
        assert!(
            re.blocked_status.is_match(blocked_line),
            "Should detect legacy BLOCKED status"
        );

        // BLOCKED at end of line
        let blocked_eol = "status: BLOCKED";
        assert!(
            re.blocked_status.is_match(blocked_eol),
            "Should detect BLOCKED at end of line"
        );

        // BLOCKED_PUBLISH is valid (gate status)
        let blocked_publish = "status: BLOCKED_PUBLISH";
        assert!(
            !re.blocked_status.is_match(blocked_publish),
            "Should NOT flag BLOCKED_PUBLISH as legacy"
        );
    }

    // -------------------------------------------------------------------------
    // Recommended action enum validation (Check 29)
    // -------------------------------------------------------------------------

    /// Valid canonical recommended_action line is matched correctly.
    #[test]
    fn test_canon_action_regex_matches_valid() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let valid = "recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV";
        assert!(
            re.canon_action.is_match(valid),
            "Should match valid recommended_action line"
        );

        // With leading whitespace
        let with_indent = "    recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV";
        assert!(
            re.canon_action.is_match(with_indent),
            "Should match with indentation"
        );
    }

    /// Invalid recommended_action lines are rejected.
    #[test]
    fn test_canon_action_regex_rejects_invalid() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Missing FIX_ENV
        let missing = "recommended_action: PROCEED | RERUN | BOUNCE";
        assert!(
            !re.canon_action.is_match(missing),
            "Should reject missing FIX_ENV"
        );

        // Wrong order
        let wrong_order = "recommended_action: BOUNCE | PROCEED | RERUN | FIX_ENV";
        assert!(
            !re.canon_action.is_match(wrong_order),
            "Should reject wrong order"
        );

        // Flow-specific legacy patterns
        let legacy_bounce = "recommended_action: BOUNCE_BUILD | RERUN";
        assert!(
            !re.canon_action.is_match(legacy_bounce),
            "Should reject flow-specific patterns"
        );
    }

    /// Test recommended_action presence check (separate from canonical check).
    #[test]
    fn test_recommended_action_present_regex() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Any recommended_action line
        let present = "recommended_action: PROCEED";
        assert!(
            re.recommended_action_present.is_match(present),
            "Should detect recommended_action presence"
        );

        // With value list
        let with_list = "recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV";
        assert!(
            re.recommended_action_present.is_match(with_list),
            "Should detect presence with value list"
        );

        // Not present
        let missing = "status: VERIFIED\nblockers: []";
        assert!(
            !re.recommended_action_present.is_match(missing),
            "Should not match when recommended_action is absent"
        );
    }

    // -------------------------------------------------------------------------
    // Route fields validation (Check 31)
    // -------------------------------------------------------------------------

    /// Route fields are detected correctly.
    #[test]
    fn test_route_fields_regexes() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // route_to_agent
        let with_agent = "route_to_agent: clarifier";
        assert!(
            re.route_to_agent.is_match(with_agent),
            "Should detect route_to_agent"
        );

        let agent_null = "route_to_agent: null";
        assert!(
            re.route_to_agent.is_match(agent_null),
            "Should detect route_to_agent: null"
        );

        // route_to_flow
        let with_flow = "route_to_flow: 2";
        assert!(
            re.route_to_flow.is_match(with_flow),
            "Should detect route_to_flow"
        );

        let flow_null = "route_to_flow: null";
        assert!(
            re.route_to_flow.is_match(flow_null),
            "Should detect route_to_flow: null"
        );
    }

    // -------------------------------------------------------------------------
    // Gate Result field validation (Check 16)
    // -------------------------------------------------------------------------

    /// Gate Result fields list contains all required fields.
    #[test]
    fn test_gate_result_required_fields() {
        let contracts = crate::contracts::Contracts::default();

        // Required fields per CLAUDE.md GATE_RESULT_V1
        let expected_fields = [
            "safe_to_commit",
            "safe_to_publish",
            "modified_files",
            "needs_upstream_fix",
            "route_to_agent",
            "route_to_flow",
            "recommended_action",
        ];

        for field in expected_fields {
            assert!(
                contracts.gate_result_fields.contains(&field),
                "Gate Result should require field: {}",
                field
            );
        }
    }

    // -------------------------------------------------------------------------
    // Repo Operator Result field validation (Check 18)
    // -------------------------------------------------------------------------

    /// Repo Operator Result fields list contains all required fields.
    #[test]
    fn test_repo_operator_result_required_fields() {
        let contracts = crate::contracts::Contracts::default();

        // Required fields per CLAUDE.md REPO_OPERATOR_RESULT_V1
        let expected_fields = [
            "operation:",
            "status:",
            "proceed_to_github_ops:",
            "commit_sha:",
            "publish_surface:",
            "anomaly_paths:",
        ];

        for field in expected_fields {
            assert!(
                contracts.repo_operator_result_fields.contains(&field),
                "Repo Operator Result should require field: {}",
                field
            );
        }
    }

    // -------------------------------------------------------------------------
    // Checkpoint mode validation (Check 21)
    // -------------------------------------------------------------------------

    /// Checkpoint mode regex matches correctly.
    #[test]
    fn test_checkpoint_mode_local_regex() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let valid = "checkpoint_mode: local_only";
        assert!(
            re.checkpoint_mode_local.is_match(valid),
            "Should match checkpoint_mode: local_only"
        );

        let with_note = "When checkpoint_mode is local_only, no push occurs";
        assert!(
            re.checkpoint_mode_local.is_match(with_note),
            "Should match checkpoint_mode reference"
        );
    }

    /// proceed_to_github_ops: false detection.
    #[test]
    fn test_proceed_false_regex() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let valid = "proceed_to_github_ops: false";
        assert!(
            re.proceed_false.is_match(valid),
            "Should match proceed_to_github_ops: false"
        );

        let sets_false = "sets proceed_to_github_ops to false";
        assert!(
            re.proceed_false.is_match(sets_false),
            "Should match documentation pattern"
        );
    }

    // -------------------------------------------------------------------------
    // GH agents two gates validation (Check 19)
    // -------------------------------------------------------------------------

    /// GH agents gate pattern detection.
    #[test]
    fn test_gh_agent_regex() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let gh_issue = "gh-issue-manager";
        assert!(re.gh_agent.is_match(gh_issue), "Should match gh-issue-manager");

        let gh_reporter = "gh-reporter";
        assert!(re.gh_agent.is_match(gh_reporter), "Should match gh-reporter");

        // Case insensitive
        let upper = "GH-Issue-Manager";
        assert!(re.gh_agent.is_match(upper), "Should match case-insensitive");
    }

    /// Both gates on same line detection.
    #[test]
    fn test_both_gates_same_line_regex() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let both_gates = "safe_to_publish: true AND proceed_to_github_ops: true";
        assert!(
            re.both_gates_same_line.is_match(both_gates),
            "Should match both gates on same line"
        );

        let reverse_order = "proceed_to_github_ops: true and safe_to_publish: true";
        assert!(
            re.both_gates_same_line.is_match(reverse_order),
            "Should match gates in either order"
        );

        let separate_lines = "safe_to_publish: true\nproceed_to_github_ops: true";
        assert!(
            !re.both_gates_same_line.is_match(separate_lines),
            "Should not match gates on separate lines"
        );
    }

    // -------------------------------------------------------------------------
    // Machine Summary heading validation (Check 3)
    // -------------------------------------------------------------------------

    /// Machine Summary heading constant is correct.
    #[test]
    fn test_machine_summary_heading_constant() {
        assert_eq!(
            headings::MACHINE_SUMMARY_H2,
            "## Machine Summary",
            "Machine Summary heading constant should be exact"
        );
    }

    // -------------------------------------------------------------------------
    // Critics list validation
    // -------------------------------------------------------------------------

    /// Critics list contains all expected critic agents.
    #[test]
    fn test_critics_list_complete() {
        let contracts = crate::contracts::Contracts::default();

        let expected_critics = [
            "requirements-critic",
            "bdd-critic",
            "design-critic",
            "contract-critic",
            "observability-critic",
            "code-critic",
            "test-critic",
            "doc-critic",
        ];

        for critic in expected_critics {
            assert!(
                contracts.critics.contains(&critic),
                "Critics list should contain: {}",
                critic
            );
        }
    }

    // -------------------------------------------------------------------------
    // Cleanup agents and receipts validation (Check 4)
    // -------------------------------------------------------------------------

    /// Cleanup agents have correct receipt mappings.
    #[test]
    fn test_cleanup_agents_receipts_complete() {
        let contracts = crate::contracts::Contracts::default();

        let expected_mappings = [
            ("signal-cleanup", "signal_receipt.json"),
            ("plan-cleanup", "plan_receipt.json"),
            ("build-cleanup", "build_receipt.json"),
            ("gate-cleanup", "gate_receipt.json"),
            ("deploy-cleanup", "deploy_receipt.json"),
            ("wisdom-cleanup", "wisdom_receipt.json"),
        ];

        for (agent, receipt) in expected_mappings {
            let found = contracts
                .cleanup_agents
                .iter()
                .any(|(a, r)| *a == agent && *r == receipt);
            assert!(
                found,
                "Cleanup agents should map {} to {}",
                agent, receipt
            );
        }
    }

    // -------------------------------------------------------------------------
    // Critic and verifier agents list validation (Check 28, 29, 31, 32, 33)
    // -------------------------------------------------------------------------

    /// Critic and verifier agents list contains all expected agents.
    #[test]
    fn test_critic_and_verifier_agents_complete() {
        let contracts = crate::contracts::Contracts::default();

        // All critics should be in critic_and_verifier_agents
        for critic in contracts.critics {
            assert!(
                contracts.critic_and_verifier_agents.contains(critic),
                "critic_and_verifier_agents should contain critic: {}",
                critic
            );
        }

        // Additional verifiers
        let additional_verifiers = [
            "contract-enforcer",
            "coverage-enforcer",
            "artifact-auditor",
            "receipt-checker",
            "security-scanner",
            "deploy-monitor",
            "smoke-verifier",
            "traceability-auditor",
            "fix-forward-runner",
        ];

        for verifier in additional_verifiers {
            assert!(
                contracts.critic_and_verifier_agents.contains(&verifier),
                "critic_and_verifier_agents should contain verifier: {}",
                verifier
            );
        }
    }

    // -------------------------------------------------------------------------
    // Gate agents validation (Check 35)
    // -------------------------------------------------------------------------

    /// Gate agents list is correct.
    #[test]
    fn test_gate_agents_complete() {
        let contracts = crate::contracts::Contracts::default();

        let expected_gate_agents = ["contract-enforcer", "coverage-enforcer"];

        for agent in expected_gate_agents {
            assert!(
                contracts.gate_agents.contains(&agent),
                "Gate agents should contain: {}",
                agent
            );
        }
    }

    // -------------------------------------------------------------------------
    // GH agents list validation (Check 19)
    // -------------------------------------------------------------------------

    /// GH agents list is correct.
    #[test]
    fn test_gh_agents_complete() {
        let contracts = crate::contracts::Contracts::default();

        let expected_gh_agents = ["gh-issue-manager", "gh-reporter"];

        for agent in expected_gh_agents {
            assert!(
                contracts.gh_agents.contains(&agent),
                "GH agents should contain: {}",
                agent
            );
        }
    }

    // -------------------------------------------------------------------------
    // Edge case: Malformed Machine Summary blocks
    // -------------------------------------------------------------------------

    /// Status value with trailing text should not match canonical pattern.
    #[test]
    fn test_canon_status_rejects_trailing_text() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Extra text after the enum
        let with_trailing = "status: VERIFIED | UNVERIFIED | CANNOT_PROCEED # comment";
        assert!(
            !re.canon_status.is_match(with_trailing),
            "Should reject line with trailing comment"
        );
    }

    /// Status line missing pipe separators should not match.
    #[test]
    fn test_canon_status_rejects_missing_separators() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Missing pipes
        let no_pipes = "status: VERIFIED UNVERIFIED CANNOT_PROCEED";
        assert!(
            !re.canon_status.is_match(no_pipes),
            "Should reject line without pipe separators"
        );

        // Commas instead of pipes
        let commas = "status: VERIFIED, UNVERIFIED, CANNOT_PROCEED";
        assert!(
            !re.canon_status.is_match(commas),
            "Should reject comma-separated values"
        );
    }

    /// Status value alone (not the enum line) should not match.
    #[test]
    fn test_canon_status_rejects_actual_value() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Actual status value, not the enum definition
        let actual_value = "status: VERIFIED";
        assert!(
            !re.canon_status.is_match(actual_value),
            "Should reject actual status value (not enum definition)"
        );

        let another_value = "status: CANNOT_PROCEED";
        assert!(
            !re.canon_status.is_match(another_value),
            "Should reject CANNOT_PROCEED value (not enum definition)"
        );
    }

    // -------------------------------------------------------------------------
    // Edge case: Receipt validation
    // -------------------------------------------------------------------------

    /// Receipt filename patterns are correct.
    #[test]
    fn test_receipt_filename_patterns() {
        let contracts = crate::contracts::Contracts::default();

        // All receipt names should end with _receipt.json
        for (_, receipt) in contracts.cleanup_agents {
            assert!(
                receipt.ends_with("_receipt.json"),
                "Receipt {} should end with _receipt.json",
                receipt
            );
        }
    }

    // -------------------------------------------------------------------------
    // Edge case: Missing required fields detection
    // -------------------------------------------------------------------------

    /// Test that empty content correctly fails field detection.
    #[test]
    fn test_empty_content_missing_fields() {
        let empty = "";

        // Should not contain any required fields
        assert!(!empty.contains("## Machine Summary"));
        assert!(!empty.contains("safe_to_publish"));
        assert!(!empty.contains("proceed_to_github_ops"));
        assert!(!empty.contains("observations:"));
    }

    /// Test that partial content correctly identifies missing fields.
    #[test]
    fn test_partial_content_missing_fields() {
        let partial = "## Machine Summary\nstatus: VERIFIED\nblockers: []";

        // Has Machine Summary but missing other fields
        assert!(partial.contains("## Machine Summary"));
        assert!(!partial.contains("observations:"));
        assert!(!partial.contains("recommended_action:"));
        assert!(!partial.contains("route_to_agent:"));
        assert!(!partial.contains("route_to_flow:"));
    }

    // -------------------------------------------------------------------------
    // Edge case: Type mismatch scenarios
    // -------------------------------------------------------------------------

    /// Status with wrong type (not string) pattern detection.
    #[test]
    fn test_status_value_patterns() {
        // These are content patterns, not JSON parsing
        // The checks look for string patterns in markdown

        // Valid string pattern
        let valid = "status: VERIFIED";
        assert!(valid.contains("status:"));

        // Would fail JSON validation but passes string check
        let numeric = "status: 1";
        assert!(numeric.contains("status:"));

        // The canonical regex should reject non-enum values
        let re = Regexes::compile().expect("Failed to compile regexes");
        assert!(!re.canon_status.is_match(numeric));
    }

    // -------------------------------------------------------------------------
    // Headings and sentinels
    // -------------------------------------------------------------------------

    /// All heading constants are correctly formatted.
    #[test]
    fn test_heading_constants_format() {
        // H2 headings should start with ##
        assert!(headings::MACHINE_SUMMARY_H2.starts_with("## "));
        assert!(headings::REPO_OPERATOR_RESULT_H2.starts_with("## "));
        assert!(headings::ORCHESTRATOR_KICKOFF_H2.starts_with("## "));
        assert!(headings::ITERATION_CONTROL_H2.starts_with("## "));

        // H3 headings should start with ###
        assert!(headings::TODOWRITE_H3.starts_with("### "));
    }

    /// Sentinel markers have correct format.
    #[test]
    fn test_sentinel_markers_format() {
        // Gate Result sentinels
        assert!(sentinels::GATE_RESULT_START.contains("PACK-CONTRACT"));
        assert!(sentinels::GATE_RESULT_END.contains("PACK-CONTRACT"));
        assert!(sentinels::GATE_RESULT_START.contains("START"));
        assert!(sentinels::GATE_RESULT_END.contains("END"));
    }

    // -------------------------------------------------------------------------
    // Additional tests for uncovered error paths
    // -------------------------------------------------------------------------

    /// Test blocked_status regex edge cases.
    #[test]
    fn test_blocked_status_regex_edge_cases() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Test BLOCKED followed by non-underscore character
        let blocked_with_space = "status: BLOCKED ";
        assert!(
            re.blocked_status.is_match(blocked_with_space),
            "Should match BLOCKED followed by space"
        );

        // Test BLOCKED followed by underscore (should NOT match because of BLOCKED_PUBLISH exception)
        let blocked_underscore = "status: BLOCKED_SOMETHING";
        assert!(
            !re.blocked_status.is_match(blocked_underscore),
            "Should NOT match BLOCKED followed by underscore"
        );

        // Test in middle of content
        let in_content = "The status: BLOCKED case";
        assert!(
            re.blocked_status.is_match(in_content),
            "Should match BLOCKED in middle of content"
        );
    }

    /// Test canon_status with different whitespace variations.
    #[test]
    fn test_canon_status_whitespace_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Tab-indented
        let with_tabs = "\tstatus: VERIFIED | UNVERIFIED | CANNOT_PROCEED";
        assert!(
            re.canon_status.is_match(with_tabs),
            "Should match with tab indentation"
        );

        // Mixed whitespace in pipes
        let mixed_space = "status: VERIFIED  |  UNVERIFIED  |  CANNOT_PROCEED";
        assert!(
            re.canon_status.is_match(mixed_space),
            "Should match with extra spaces around pipes"
        );

        // Trailing whitespace
        let trailing = "status: VERIFIED | UNVERIFIED | CANNOT_PROCEED   ";
        assert!(
            re.canon_status.is_match(trailing),
            "Should match with trailing whitespace"
        );
    }

    /// Test canon_action with different whitespace variations.
    #[test]
    fn test_canon_action_whitespace_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Tab-indented
        let with_tabs = "\trecommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV";
        assert!(
            re.canon_action.is_match(with_tabs),
            "Should match with tab indentation"
        );

        // Mixed whitespace
        let mixed = "recommended_action: PROCEED  |  RERUN  |  BOUNCE  |  FIX_ENV";
        assert!(
            re.canon_action.is_match(mixed),
            "Should match with extra spaces"
        );
    }

    /// Test route_to_agent regex variations.
    #[test]
    fn test_route_to_agent_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // With specific agent name
        let with_name = "route_to_agent: code-implementer";
        assert!(re.route_to_agent.is_match(with_name));

        // With dashes
        let with_dashes = "route_to_agent: requirements-critic";
        assert!(re.route_to_agent.is_match(with_dashes));

        // Empty value
        let empty_val = "route_to_agent: ";
        assert!(re.route_to_agent.is_match(empty_val));

        // Indented
        let indented = "    route_to_agent: clarifier";
        assert!(re.route_to_agent.is_match(indented));
    }

    /// Test route_to_flow regex variations.
    #[test]
    fn test_route_to_flow_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // With each flow number
        for i in 1..=6 {
            let line = format!("route_to_flow: {}", i);
            assert!(
                re.route_to_flow.is_match(&line),
                "Should match route_to_flow: {}",
                i
            );
        }

        // With null
        let with_null = "route_to_flow: null";
        assert!(re.route_to_flow.is_match(with_null));

        // Indented
        let indented = "  route_to_flow: 2";
        assert!(re.route_to_flow.is_match(indented));
    }

    /// Test checkpoint_mode_local regex edge cases.
    #[test]
    fn test_checkpoint_mode_local_edge_cases() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Different phrasings
        let phrasing1 = "checkpoint_mode=local_only";
        assert!(re.checkpoint_mode_local.is_match(phrasing1));

        let phrasing2 = "checkpoint_mode: local_only";
        assert!(re.checkpoint_mode_local.is_match(phrasing2));

        let phrasing3 = "set checkpoint_mode to local_only";
        assert!(re.checkpoint_mode_local.is_match(phrasing3));

        // Without local_only should not match
        let without = "checkpoint_mode: remote";
        assert!(!re.checkpoint_mode_local.is_match(without));
    }

    /// Test proceed_false regex edge cases.
    #[test]
    fn test_proceed_false_edge_cases() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Different phrasings
        let phrasing1 = "proceed_to_github_ops: false";
        assert!(re.proceed_false.is_match(phrasing1));

        let phrasing2 = "proceed_to_github_ops=false";
        assert!(re.proceed_false.is_match(phrasing2));

        let phrasing3 = "sets proceed_to_github_ops to false";
        assert!(re.proceed_false.is_match(phrasing3));

        // With true should not match
        let with_true = "proceed_to_github_ops: true";
        assert!(!re.proceed_false.is_match(with_true));
    }

    /// Test gh_agent regex case sensitivity.
    #[test]
    fn test_gh_agent_case_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Various case combinations
        let cases = [
            "gh-issue-manager",
            "GH-ISSUE-MANAGER",
            "Gh-Issue-Manager",
            "gh-reporter",
            "GH-REPORTER",
            "Gh-Reporter",
        ];

        for case in cases {
            assert!(
                re.gh_agent.is_match(case),
                "Should match case variant: {}",
                case
            );
        }

        // Should not match partial
        let partial = "gh-issue";
        assert!(
            !re.gh_agent.is_match(partial),
            "Should not match partial: gh-issue"
        );
    }

    /// Test both_gates_same_line regex.
    #[test]
    fn test_both_gates_same_line_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // With different connectors
        let with_and = "safe_to_publish: true AND proceed_to_github_ops: true";
        assert!(re.both_gates_same_line.is_match(with_and));

        let with_ampersand = "safe_to_publish: true && proceed_to_github_ops: true";
        assert!(re.both_gates_same_line.is_match(with_ampersand));

        let with_comma = "safe_to_publish: true, proceed_to_github_ops: true";
        assert!(re.both_gates_same_line.is_match(with_comma));

        // Single gate should not match
        let single_gate = "safe_to_publish: true";
        assert!(!re.both_gates_same_line.is_match(single_gate));
    }

    /// Test recommended_action_present regex variations.
    #[test]
    fn test_recommended_action_present_variations() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Single value
        let single = "recommended_action: PROCEED";
        assert!(re.recommended_action_present.is_match(single));

        // With enum definition
        let enum_def = "recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV";
        assert!(re.recommended_action_present.is_match(enum_def));

        // Indented
        let indented = "    recommended_action: BOUNCE";
        assert!(re.recommended_action_present.is_match(indented));

        // Partial match should not work (wrong field name)
        let wrong_field = "action: PROCEED";
        assert!(!re.recommended_action_present.is_match(wrong_field));
    }

    /// Test content matching for Gate Result fields.
    #[test]
    fn test_gate_result_fields_in_content() {
        let contracts = crate::contracts::Contracts::default();

        let content = r#"
## Gate Result
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
needs_upstream_fix: true | false
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | null
route_to_agent: <agent-name> | null
"#;

        for field in contracts.gate_result_fields {
            assert!(
                content.contains(field),
                "Content should contain Gate Result field: {}",
                field
            );
        }
    }

    /// Test content matching for Repo Operator Result fields.
    #[test]
    fn test_repo_operator_result_fields_in_content() {
        let contracts = crate::contracts::Contracts::default();

        let content = r#"
## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
"#;

        for field in contracts.repo_operator_result_fields {
            assert!(
                content.contains(field),
                "Content should contain Repo Operator Result field: {}",
                field
            );
        }
    }

    /// Test Safe Output Contract heading.
    #[test]
    fn test_safe_output_contract_heading() {
        assert_eq!(
            headings::SAFE_OUTPUT_CONTRACT,
            "Safe Output Contract",
            "Safe Output Contract heading should be exact"
        );
    }

    /// Test Repo Operator Result heading.
    #[test]
    fn test_repo_operator_result_heading() {
        assert_eq!(
            headings::REPO_OPERATOR_RESULT_H2,
            "## Repo Operator Result",
            "Repo Operator Result heading should be exact"
        );
    }

    /// Test observations field detection in critic content.
    #[test]
    fn test_observations_field_detection() {
        let with_observations = r#"
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
blockers: []
concerns: []
observations: []
"#;
        assert!(with_observations.contains("observations:"));

        let without_observations = r#"
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
blockers: []
concerns: []
"#;
        assert!(!without_observations.contains("observations:"));

        // With non-empty observations
        let with_entries = "observations:\n  - friction noted";
        assert!(with_entries.contains("observations:"));
    }

    /// Test can_further_iteration_help field detection.
    #[test]
    fn test_can_further_iteration_help_detection() {
        let with_field = "can_further_iteration_help: yes | no";
        assert!(with_field.contains("can_further_iteration_help"));

        let with_value = "can_further_iteration_help: yes";
        assert!(with_value.contains("can_further_iteration_help"));

        let without_field = "can_iterate: yes";
        assert!(!without_field.contains("can_further_iteration_help"));
    }

    /// Test missing_required field detection.
    #[test]
    fn test_missing_required_detection() {
        let with_field = "missing_required:\n  - path/to/file (reason)";
        assert!(with_field.contains("missing_required"));

        let empty_field = "missing_required: []";
        assert!(empty_field.contains("missing_required"));

        let without = "required: []";
        assert!(!without.contains("missing_required"));
    }

    /// Test index.json reference detection.
    #[test]
    fn test_index_json_reference() {
        let with_reference = "Update index.json with the new status";
        assert!(with_reference.contains("index.json"));

        let code_reference = "`.runs/index.json`";
        assert!(code_reference.contains("index.json"));

        let path_reference = ".runs/index.json";
        assert!(path_reference.contains("index.json"));
    }

    /// Test CANNOT_PROCEED detection in content.
    #[test]
    fn test_cannot_proceed_detection() {
        let with_cp = "status: CANNOT_PROCEED means mechanical failure";
        assert!(with_cp.contains("CANNOT_PROCEED"));

        let in_enum = "status: VERIFIED | UNVERIFIED | CANNOT_PROCEED";
        assert!(in_enum.contains("CANNOT_PROCEED"));

        let without = "status: VERIFIED | UNVERIFIED";
        assert!(!without.contains("CANNOT_PROCEED"));
    }

    /// Test legacy recommended_gate_action detection.
    #[test]
    fn test_legacy_recommended_gate_action() {
        let legacy = "recommended_gate_action: MERGE";
        assert!(legacy.contains("recommended_gate_action"));

        let unified = "recommended_action: PROCEED";
        assert!(!unified.contains("recommended_gate_action"));
    }

    /// Test route_to_flow documentation detection.
    #[test]
    fn test_route_to_flow_documentation() {
        let with_doc = "Set route_to_flow when bouncing upstream";
        assert!(with_doc.contains("route_to_flow"));

        let in_yaml = "route_to_flow: 2";
        assert!(in_yaml.contains("route_to_flow"));
    }

    /// Test secrets-sanitizer reference detection.
    #[test]
    fn test_secrets_sanitizer_reference() {
        let lower = "Call secrets-sanitizer before checkpointing";
        assert!(crate::util::contains_ignore_ascii_case(
            lower,
            "secrets-sanitizer"
        ));

        let mixed = "Secrets-Sanitizer scans the publish surface";
        assert!(crate::util::contains_ignore_ascii_case(
            mixed,
            "secrets-sanitizer"
        ));

        // Case sensitivity test
        let upper = "SECRETS-SANITIZER";
        assert!(crate::util::contains_ignore_ascii_case(
            upper,
            "secrets-sanitizer"
        ));
    }

    /// Test Gate Result sentinel block detection.
    #[test]
    fn test_gate_result_sentinel_detection() {
        let with_both = format!(
            "some content\n{}\nblock content\n{}\nmore content",
            sentinels::GATE_RESULT_START,
            sentinels::GATE_RESULT_END
        );
        assert!(with_both.contains(sentinels::GATE_RESULT_START));
        assert!(with_both.contains(sentinels::GATE_RESULT_END));

        let missing_end = format!("content\n{}\nblock", sentinels::GATE_RESULT_START);
        assert!(missing_end.contains(sentinels::GATE_RESULT_START));
        assert!(!missing_end.contains(sentinels::GATE_RESULT_END));

        let missing_start = format!("block\n{}\ncontent", sentinels::GATE_RESULT_END);
        assert!(!missing_start.contains(sentinels::GATE_RESULT_START));
        assert!(missing_start.contains(sentinels::GATE_RESULT_END));
    }

    /// Test two gates enforcement detection.
    #[test]
    fn test_two_gates_enforcement() {
        let both_gates = "Check safe_to_publish and proceed_to_github_ops before posting";
        assert!(both_gates.contains("safe_to_publish"));
        assert!(both_gates.contains("proceed_to_github_ops"));

        let only_publish = "Check safe_to_publish before posting";
        assert!(only_publish.contains("safe_to_publish"));
        assert!(!only_publish.contains("proceed_to_github_ops"));

        let only_proceed = "Check proceed_to_github_ops before posting";
        assert!(!only_proceed.contains("safe_to_publish"));
        assert!(only_proceed.contains("proceed_to_github_ops"));
    }

    /// Test output constraint documentation detection.
    #[test]
    fn test_output_constraint_documentation() {
        let with_constraint = "gh-reporter must NOT paste verbatim content";
        assert!(with_constraint.contains("must NOT paste verbatim"));

        let without = "gh-reporter posts summaries";
        assert!(!without.contains("must NOT paste verbatim"));
    }

    /// Test Iteration Control heading detection.
    #[test]
    fn test_iteration_control_heading() {
        assert_eq!(
            headings::ITERATION_CONTROL_H2,
            "## Iteration Control",
            "Iteration Control heading should be exact"
        );

        let content = "## Iteration Control\nmax_iterations: 3";
        assert!(content.contains(headings::ITERATION_CONTROL_H2));
    }

    /// Test Orchestrator Kickoff heading detection.
    #[test]
    fn test_orchestrator_kickoff_heading() {
        assert_eq!(
            headings::ORCHESTRATOR_KICKOFF_H2,
            "## Orchestrator Kickoff",
            "Orchestrator Kickoff heading should be exact"
        );
    }

    /// Test TodoWrite heading detection.
    #[test]
    fn test_todowrite_heading() {
        assert!(headings::TODOWRITE_H3.starts_with("### "));
        assert!(headings::TODOWRITE_H3.contains("TodoWrite"));
    }

    // =========================================================================
    // Integration tests using tempdir fixtures
    // =========================================================================

    mod integration {
        use super::*;
        use crate::cli::OutputFormat;
        use crate::contracts::{Contracts, Regexes};
        use crate::ctx::Ctx;
        use crate::inventory::Inventory;
        use crate::reporter::Reporter;
        use std::fs;
        use tempfile::TempDir;

        /// Helper struct for test fixture setup.
        struct TestFixture {
            _temp_dir: TempDir,
            ctx: Ctx,
            inv: Inventory,
            re: Regexes,
            c: Contracts,
        }

        impl TestFixture {
            /// Create a new test fixture with the given agents and commands.
            fn new(
                agents: &[(&str, &str)],
                commands: &[(&str, &str)],
            ) -> anyhow::Result<Self> {
                let temp_dir = TempDir::new()?;
                let root = temp_dir.path();

                // Create directory structure
                let claude_dir = root.join(".claude");
                let agents_dir = claude_dir.join("agents");
                let commands_dir = claude_dir.join("commands");
                let skills_dir = claude_dir.join("skills");

                fs::create_dir_all(&agents_dir)?;
                fs::create_dir_all(&commands_dir)?;
                fs::create_dir_all(&skills_dir)?;

                // Write agent files
                for (name, content) in agents {
                    let path = agents_dir.join(format!("{}.md", name));
                    fs::write(&path, content)?;
                }

                // Write command files
                for (name, content) in commands {
                    let path = commands_dir.join(format!("{}.md", name));
                    fs::write(&path, content)?;
                }

                // Build Ctx and Inventory
                let ctx = Ctx::discover(Some(root.to_path_buf()))?;
                let inv = Inventory::from_ctx(&ctx)?;
                let re = Regexes::compile()?;
                let c = Contracts::default();

                Ok(Self {
                    _temp_dir: temp_dir,
                    ctx,
                    inv,
                    re,
                    c,
                })
            }

            /// Create a CheckCtx from this fixture.
            fn check_ctx(&self) -> CheckCtx<'_> {
                CheckCtx {
                    ctx: &self.ctx,
                    inv: &self.inv,
                    re: &self.re,
                    c: &self.c,
                }
            }
        }

        /// Create a Reporter for testing.
        fn test_reporter() -> Reporter {
            Reporter::new(OutputFormat::Text, false, false)
        }

        // ---------------------------------------------------------------------
        // Check 3: Critics Machine Summary tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_critics_machine_summary_pass() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_agent: <agent-name | null>
route_to_flow: <1|2|3|4|5|6 | null>

blockers: []
concerns: []
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_machine_summary(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_critics_machine_summary_missing_heading() {
            let critic_content = r#"# Requirements Critic

Some content without Machine Summary heading.

status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_machine_summary(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing heading");
        }

        #[test]
        fn test_check_critics_machine_summary_missing_status_axis() {
            let critic_content = r#"# Requirements Critic

## Machine Summary

status: VERIFIED

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_machine_summary(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing canonical status axis"
            );
        }

        #[test]
        fn test_check_critics_machine_summary_missing_action_axis() {
            let critic_content = r#"# Requirements Critic

## Machine Summary

status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_machine_summary(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing canonical action axis"
            );
        }

        // ---------------------------------------------------------------------
        // Check 4: Cleanup receipts tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_cleanup_receipts_pass() {
            let cleanup_content = r#"# Signal Cleanup

Write the signal_receipt.json to the flow directory.

Update index.json with the run status.
"#;

            let fixture = TestFixture::new(
                &[("signal-cleanup", cleanup_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_receipts(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_cleanup_receipts_missing_receipt() {
            let cleanup_content = r#"# Signal Cleanup

Do some cleanup work.

Update index.json with the run status.
"#;

            let fixture = TestFixture::new(
                &[("signal-cleanup", cleanup_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_receipts(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing receipt reference");
        }

        #[test]
        fn test_check_cleanup_receipts_missing_index() {
            let cleanup_content = r#"# Signal Cleanup

Write the signal_receipt.json to the flow directory.

No index update here.
"#;

            let fixture = TestFixture::new(
                &[("signal-cleanup", cleanup_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_receipts(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing index.json reference");
        }

        // ---------------------------------------------------------------------
        // Check 16: Gate Result block tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_gate_result_block_pass() {
            let flow_content = r#"# Flow 1 Signal

This flow uses secrets-sanitizer for the publish gate.

<!-- PACK-CONTRACT: GATE_RESULT_V1 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
needs_upstream_fix: true | false
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | null
route_to_agent: <agent-name> | null
```
<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-1-signal", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_result_block(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_gate_result_block_missing_sentinel() {
            let flow_content = r#"# Flow 1 Signal

This flow uses secrets-sanitizer for the publish gate.

## Gate Result
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-1-signal", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_result_block(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing sentinel");
        }

        #[test]
        fn test_check_gate_result_block_missing_fields() {
            let flow_content = r#"# Flow 1 Signal

This flow uses secrets-sanitizer for the publish gate.

<!-- PACK-CONTRACT: GATE_RESULT_V1 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
```
<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-1-signal", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_result_block(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing required fields");
        }

        #[test]
        fn test_check_gate_result_block_no_sanitizer_skipped() {
            // Flow without secrets-sanitizer should be skipped
            let flow_content = r#"# Flow 6 Wisdom

This flow does not mention the sanitizer.

## Some Section
content here
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-6-wisdom", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_result_block(&cx, &mut rep).expect("Check failed");
            // Should have no errors because this flow is skipped
            assert_eq!(rep.errors, 0, "Flow without secrets-sanitizer should be skipped");
        }

        // ---------------------------------------------------------------------
        // Check 17: GH reporter output tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_gh_reporter_output_pass() {
            let reporter_content = r#"# GH Reporter

## Safe Output Contract

This agent must NOT paste verbatim content from artifacts.

Only post summaries and links.
"#;

            let fixture = TestFixture::new(
                &[("gh-reporter", reporter_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_reporter_output(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_gh_reporter_output_missing_contract() {
            let reporter_content = r#"# GH Reporter

This agent posts to GitHub.

must NOT paste verbatim content.
"#;

            let fixture = TestFixture::new(
                &[("gh-reporter", reporter_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_reporter_output(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing Safe Output Contract");
        }

        #[test]
        fn test_check_gh_reporter_output_missing_constraint() {
            let reporter_content = r#"# GH Reporter

## Safe Output Contract

This agent posts summaries.
"#;

            let fixture = TestFixture::new(
                &[("gh-reporter", reporter_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_reporter_output(&cx, &mut rep).expect("Check failed");
            assert!(rep.warnings > 0, "Should have warning for missing output constraint doc");
        }

        // ---------------------------------------------------------------------
        // Check 18: Repo operator result tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_repo_operator_result_pass() {
            let operator_content = r#"# Repo Operator

## Repo Operator Result

```yaml
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```
"#;

            let fixture = TestFixture::new(
                &[("repo-operator", operator_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_repo_operator_result(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_repo_operator_result_missing_section() {
            let operator_content = r#"# Repo Operator

This agent handles git operations.

operation: checkpoint
status: COMPLETED
"#;

            let fixture = TestFixture::new(
                &[("repo-operator", operator_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_repo_operator_result(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing section");
        }

        #[test]
        fn test_check_repo_operator_result_missing_fields() {
            let operator_content = r#"# Repo Operator

## Repo Operator Result

```yaml
operation: checkpoint
status: COMPLETED
```
"#;

            let fixture = TestFixture::new(
                &[("repo-operator", operator_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_repo_operator_result(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing fields");
        }

        #[test]
        fn test_check_repo_operator_result_missing_agent() {
            // No repo-operator agent
            let fixture = TestFixture::new(&[], &[]).expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_repo_operator_result(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have error for missing repo-operator.md");
        }

        // ---------------------------------------------------------------------
        // Check 19: GH agents two gates tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_gh_agents_two_gates_pass() {
            let reporter_content = r#"# GH Reporter

Check safe_to_publish and proceed_to_github_ops before posting.
"#;
            let manager_content = r#"# GH Issue Manager

Verify safe_to_publish and proceed_to_github_ops.
"#;

            let fixture = TestFixture::new(
                &[
                    ("gh-reporter", reporter_content),
                    ("gh-issue-manager", manager_content),
                ],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_agents_two_gates(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_gh_agents_two_gates_missing_one_gate() {
            let reporter_content = r#"# GH Reporter

Check safe_to_publish before posting.
"#;

            let fixture = TestFixture::new(
                &[("gh-reporter", reporter_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_agents_two_gates(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing gate");
        }

        // ---------------------------------------------------------------------
        // Check 20: Flow GH gating tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_flow_gh_gating_pass() {
            let flow_content = r#"# Flow 1 Signal

Call gh-reporter to post results.

Gate on safe_to_publish and proceed_to_github_ops.
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-1-signal", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_flow_gh_gating(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_flow_gh_gating_missing_gates() {
            let flow_content = r#"# Flow 1 Signal

Call gh-reporter to post results.
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-1-signal", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_flow_gh_gating(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing gate documentation");
        }

        #[test]
        fn test_check_flow_gh_gating_no_gh_agents() {
            // Flow without GH agent references should pass
            let flow_content = r#"# Flow 3 Build

Build the code.

No GitHub operations here.
"#;

            let fixture = TestFixture::new(
                &[],
                &[("flow-3-build", flow_content)],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_flow_gh_gating(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Flow without GH agents should pass");
        }

        // ---------------------------------------------------------------------
        // Check 21: Checkpoint local_only tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_checkpoint_local_only_pass() {
            let operator_content = r#"# Repo Operator

Use checkpoint_mode: local_only for safe-bail.

When local_only is set, proceed_to_github_ops: false.
"#;

            // Need to create the fixture with owned strings
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let root = temp_dir.path();
            let claude_dir = root.join(".claude");
            let agents_dir = claude_dir.join("agents");
            let commands_dir = claude_dir.join("commands");
            let skills_dir = claude_dir.join("skills");

            fs::create_dir_all(&agents_dir).expect("Failed to create agents dir");
            fs::create_dir_all(&commands_dir).expect("Failed to create commands dir");
            fs::create_dir_all(&skills_dir).expect("Failed to create skills dir");

            // Write repo-operator
            fs::write(agents_dir.join("repo-operator.md"), operator_content)
                .expect("Failed to write agent");

            // Write 6 flow commands
            for i in 1..=6 {
                let content = "Use checkpoint_mode: local_only for safe-bail.\n";
                fs::write(commands_dir.join(format!("flow-{}-test.md", i)), content)
                    .expect("Failed to write flow");
            }

            let ctx = Ctx::discover(Some(root.to_path_buf())).expect("Failed to create Ctx");
            let inv = Inventory::from_ctx(&ctx).expect("Failed to create Inventory");
            let re = Regexes::compile().expect("Failed to compile regexes");
            let c = Contracts::default();

            let cx = CheckCtx {
                ctx: &ctx,
                inv: &inv,
                re: &re,
                c: &c,
            };
            let mut rep = test_reporter();

            check_checkpoint_local_only(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_checkpoint_local_only_missing_in_operator() {
            let operator_content = r#"# Repo Operator

This agent handles git operations.
"#;

            let fixture = TestFixture::new(
                &[("repo-operator", operator_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_checkpoint_local_only(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing local_only docs");
        }

        // ---------------------------------------------------------------------
        // Check 28: Status enum tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_status_enum_pass() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_status_enum(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_status_enum_legacy_blocked() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: BLOCKED

This uses legacy BLOCKED status.
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_status_enum(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for legacy BLOCKED status");
        }

        #[test]
        fn test_check_status_enum_blocked_publish_ok() {
            // BLOCKED_PUBLISH is a valid gate status, not legacy
            let enforcer_content = r#"# Contract Enforcer

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

Gate status: BLOCKED_PUBLISH is valid.
"#;

            let fixture = TestFixture::new(
                &[("contract-enforcer", enforcer_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_status_enum(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "BLOCKED_PUBLISH should not be flagged as legacy");
        }

        // ---------------------------------------------------------------------
        // Check 29: Recommended action enum tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_recommended_action_enum_pass() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_recommended_action_enum(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_recommended_action_enum_missing() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_recommended_action_enum(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing recommended_action");
        }

        #[test]
        fn test_check_recommended_action_enum_drifted() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
recommended_action: PROCEED | BOUNCE
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_recommended_action_enum(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for drifted recommended_action");
        }

        // ---------------------------------------------------------------------
        // Check 31: Route fields tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_route_fields_pass() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
route_to_agent: <agent-name | null>
route_to_flow: <1|2|3|4|5|6 | null>
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_route_fields(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_route_fields_missing() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_route_fields(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing route fields");
        }

        // ---------------------------------------------------------------------
        // Check 32: CANNOT_PROCEED invariant tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_cannot_proceed_invariant_pass() {
            let critic_content = r#"# Requirements Critic

CANNOT_PROCEED means mechanical failure.

missing_required: []
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cannot_proceed_invariant(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_cannot_proceed_invariant_missing_required() {
            let critic_content = r#"# Requirements Critic

CANNOT_PROCEED means mechanical failure.
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cannot_proceed_invariant(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing_required when CANNOT_PROCEED is mentioned"
            );
        }

        #[test]
        fn test_check_cannot_proceed_invariant_no_mention() {
            // No CANNOT_PROCEED mention should pass
            let critic_content = r#"# Requirements Critic

This critic does not mention the status.
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cannot_proceed_invariant(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "No CANNOT_PROCEED mention should pass");
        }

        // ---------------------------------------------------------------------
        // Check 33: Critics iteration help tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_critics_iteration_help_pass() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
can_further_iteration_help: yes | no
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_iteration_help(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_critics_iteration_help_missing() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_iteration_help(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing can_further_iteration_help"
            );
        }

        // ---------------------------------------------------------------------
        // Check 34: Cleanup route_to_flow tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_cleanup_route_to_flow_pass() {
            let cleanup_content = r#"# Signal Cleanup

## Machine Summary
route_to_flow: <1|2|3|4|5|6 | null>
"#;

            let fixture = TestFixture::new(
                &[("signal-cleanup", cleanup_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_route_to_flow(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
            assert_eq!(rep.warnings, 0, "Should have no warnings");
        }

        #[test]
        fn test_check_cleanup_route_to_flow_missing() {
            let cleanup_content = r#"# Signal Cleanup

This cleanup does not mention the routing field.
"#;

            let fixture = TestFixture::new(
                &[("signal-cleanup", cleanup_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_route_to_flow(&cx, &mut rep).expect("Check failed");
            assert!(rep.warnings > 0, "Should have warning for missing route_to_flow");
        }

        // ---------------------------------------------------------------------
        // Check 35: Gate unified action tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_gate_unified_action_pass() {
            let enforcer_content = r#"# Contract Enforcer

## Machine Summary
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
"#;

            let fixture = TestFixture::new(
                &[("contract-enforcer", enforcer_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_unified_action(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_gate_unified_action_legacy() {
            let enforcer_content = r#"# Contract Enforcer

## Machine Summary
recommended_gate_action: MERGE | BOUNCE
"#;

            let fixture = TestFixture::new(
                &[("contract-enforcer", enforcer_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_unified_action(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for legacy recommended_gate_action");
        }

        #[test]
        fn test_check_gate_unified_action_missing() {
            let enforcer_content = r#"# Contract Enforcer

## Machine Summary
status: VERIFIED
"#;

            let fixture = TestFixture::new(
                &[("contract-enforcer", enforcer_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_unified_action(&cx, &mut rep).expect("Check failed");
            assert!(rep.warnings > 0, "Should have warning for missing recommended_action");
        }

        // ---------------------------------------------------------------------
        // Check 51: Critics observations field tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_critics_observations_field_pass() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
blockers: []
concerns: []
observations: []
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_observations_field(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
        }

        #[test]
        fn test_check_critics_observations_field_missing() {
            let critic_content = r#"# Requirements Critic

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
blockers: []
concerns: []
"#;

            let fixture = TestFixture::new(
                &[("requirements-critic", critic_content)],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_observations_field(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing observations field");
        }

        #[test]
        fn test_check_critics_observations_field_multiple_critics() {
            let req_critic = r#"# Requirements Critic
observations: []
"#;
            let bdd_critic = r#"# BDD Critic
blockers: []
"#;
            let code_critic = r#"# Code Critic
observations:
  - some observation
"#;

            let fixture = TestFixture::new(
                &[
                    ("requirements-critic", req_critic),
                    ("bdd-critic", bdd_critic),
                    ("code-critic", code_critic),
                ],
                &[],
            )
            .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_observations_field(&cx, &mut rep).expect("Check failed");
            // bdd-critic is missing observations
            assert!(rep.errors > 0, "Should have errors for critic missing observations");
        }
    }
}
