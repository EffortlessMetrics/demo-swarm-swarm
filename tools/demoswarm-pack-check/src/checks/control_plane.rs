//! Control-plane checks: Claude-native patterns, gate blocks, handoff validation.
//!
//! == Claude-Native Migration ==
//! The pack has shifted from harness-era Machine Summary YAML blocks to Claude-native
//! prose handoffs. Checks that enforced YAML field contracts are now disabled.
//!
//! DISABLED (harness-era patterns):
//! - Check 3: Machine Summary axis (critics use Handoff sections now)
//! - Check 28: status enum (routing is prose, not enums)
//! - Check 29: recommended_action enum (same)
//! - Check 31: route_to_agent/route_to_flow fields (same)
//! - Check 33: can_further_iteration_help (expressed in prose handoff)
//! - Check 51: observations field (same)
//! - Check 34: cleanup route_to_flow (softened to pass-through)
//! - Check 35: gate unified action (softened to pass-through)
//!
//! ACTIVE:
//! - Check 4: Cleanup agents reference receipts (still valid - they write receipts)
//! - Check 17: gh-reporter output constraints (safety check)
//! - Check 18: repo-operator Result block (git ops still need structure)
//! - Check 19: GH agents enforce two gates (publish safety)
//! - Check 32: CANNOT_PROCEED invariant (useful for mechanical failures)
//! - Check 54: Critics have Handoff section (NEW - Claude-native)
//! - Check 55: Agents have clear job section (NEW - Claude-native)
//!
//! Checks: 4, 17, 18, 19, 32, 54, 55

use super::contracts::headings;
use crate::reporter::Reporter;

use super::{CheckCtx, CheckSpec};

pub fn checks() -> Vec<CheckSpec> {
    vec![
        // == ACTIVE CHECKS ==
        CheckSpec {
            id: 4,
            title: "Checking cleanup agents reference receipts + index.json...",
            run: check_cleanup_receipts,
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
            id: 32,
            title: "Checking CANNOT_PROCEED invariant...",
            run: check_cannot_proceed_invariant,
        },
        // == NEW CLAUDE-NATIVE CHECKS ==
        CheckSpec {
            id: 54,
            title: "Checking critics have Handoff section...",
            run: check_critics_handoff_section,
        },
        CheckSpec {
            id: 55,
            title: "Checking agents have clear job section...",
            run: check_agents_clear_job,
        },
    ]
}

// =============================================================================
// DEPRECATED HARNESS-ERA CHECKS (kept for reference, not called)
// =============================================================================
// The following checks enforced Machine Summary YAML block contracts.
// In Claude-native mode, critics use prose Handoff sections instead.
// These are retained for historical reference but not included in checks().

/// DEPRECATED: Check 3 - Critics have canonical Machine Summary axis.
/// Replaced by check_critics_handoff_section (Check 54).
#[allow(dead_code)]
fn check_critics_machine_summary(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: enforced YAML Machine Summary blocks with specific fields.
    // Claude-native: critics use ## Handoff sections with prose recommendations.
    // This check is disabled but retained for reference.
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

/// DEPRECATED: Check 28 - Machine Summary status enum.
/// Routing is now expressed in prose handoffs, not YAML enums.
#[allow(dead_code)]
fn check_status_enum(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: enforced status: VERIFIED | UNVERIFIED | CANNOT_PROCEED enum.
    // Claude-native: routing decisions are expressed in prose Handoff sections.
    // This check is disabled but retained for reference.
    Ok(())
}

/// DEPRECATED: Check 29 - recommended_action closed enum.
/// Routing is now expressed in prose handoffs, not YAML enums.
#[allow(dead_code)]
fn check_recommended_action_enum(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: enforced recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV.
    // Claude-native: recommendations are made in prose Handoff sections.
    // This check is disabled but retained for reference.
    Ok(())
}

/// DEPRECATED: Check 31 - route_to_agent and route_to_flow fields exist.
/// Routing is now expressed in prose handoffs, not YAML fields.
#[allow(dead_code)]
fn check_route_fields(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: enforced route_to_agent: and route_to_flow: YAML fields.
    // Claude-native: routing is expressed via prose recommendations in Handoff sections.
    // This check is disabled but retained for reference.
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

/// DEPRECATED: Check 33 - Critics have can_further_iteration_help.
/// Iteration guidance is now expressed in prose handoffs.
#[allow(dead_code)]
fn check_critics_iteration_help(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: enforced can_further_iteration_help: yes | no field.
    // Claude-native: iteration guidance is expressed in prose Handoff sections.
    // This check is disabled but retained for reference.
    Ok(())
}

/// DEPRECATED: Check 34 - Cleanup agents mention route_to_flow.
/// Routing is now expressed in prose handoffs.
#[allow(dead_code)]
fn check_cleanup_route_to_flow(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: checked for route_to_flow field documentation.
    // Claude-native: routing is expressed in prose Handoff sections.
    // This check is disabled but retained for reference.
    Ok(())
}

/// DEPRECATED: Check 35 - Gate agents use unified recommended_action.
/// Routing is now expressed in prose handoffs.
#[allow(dead_code)]
fn check_gate_unified_action(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: checked for recommended_action vs legacy recommended_gate_action.
    // Claude-native: routing is expressed in prose Handoff sections.
    // This check is disabled but retained for reference.
    Ok(())
}

/// DEPRECATED: Check 51 - Critics have observations field in Machine Summary.
/// Observations are now expressed in prose handoffs.
#[allow(dead_code)]
fn check_critics_observations_field(_cx: &CheckCtx, _rep: &mut Reporter) -> anyhow::Result<()> {
    // Harness-era pattern: enforced observations: [] field in Machine Summary.
    // Claude-native: observations are captured in prose Handoff sections.
    // This check is disabled but retained for reference.
    Ok(())
}

// =============================================================================
// NEW CLAUDE-NATIVE CHECKS
// =============================================================================

/// Check 54: Critics have Handoff section.
///
/// In Claude-native mode, critics communicate routing decisions via prose Handoff
/// sections instead of structured YAML Machine Summary blocks.
fn check_critics_handoff_section(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let mut missing = Vec::new();

    for critic in cx.c.critics {
        let Some(file) = cx.inv.agent(critic) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;

        // Look for ## Handoff section (Claude-native pattern)
        if !content.contains("## Handoff") {
            missing.push(critic.to_string());
        }
    }

    if missing.is_empty() {
        rep.pass("All critics have ## Handoff section");
    } else {
        // Warn rather than fail during transition period
        rep.warn(format!(
            "Critics missing ## Handoff section (Claude-native pattern): {}",
            missing.join(", ")
        ));
    }

    Ok(())
}

/// Check 55: Agents have clear job section.
///
/// Claude-native agents should have a clear job description, typically under
/// ## Your Job, ## Job, or similar heading.
fn check_agents_clear_job(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    // Check a subset of critical agents for clear job descriptions
    let critical_agents = [
        "code-implementer",
        "code-critic",
        "test-author",
        "test-critic",
        "repo-operator",
        "secrets-sanitizer",
    ];

    let mut missing = Vec::new();

    for agent in critical_agents {
        let Some(file) = cx.inv.agent(agent) else {
            continue;
        };

        let content = cx.ctx.read_utf8(file)?;

        // Look for job-related headings (Claude-native pattern)
        let has_job_section = content.contains("## Your Job")
            || content.contains("## Job")
            || content.contains("## Role")
            || content.contains("## Purpose")
            || content.contains("You are the");

        if !has_job_section {
            missing.push(agent.to_string());
        }
    }

    if missing.is_empty() {
        rep.pass("Critical agents have clear job descriptions");
    } else {
        // Warn rather than fail during transition period
        rep.warn(format!(
            "Agents missing clear job section: {}",
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
    use crate::contracts::test_utils::REGEXES;

    // -------------------------------------------------------------------------
    // Machine Summary status enum validation (Check 28)
    // -------------------------------------------------------------------------

    /// Valid canonical status line is matched correctly.
    #[test]
    fn test_canon_status_regex_matches_valid() {
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
            assert!(found, "Cleanup agents should map {} to {}", agent, receipt);
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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;
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
        assert!(headings::ITERATION_CONTROL_H2.starts_with("## "));
    }

    // -------------------------------------------------------------------------
    // Additional tests for uncovered error paths
    // -------------------------------------------------------------------------

    /// Test blocked_status regex edge cases.
    #[test]
    fn test_blocked_status_regex_edge_cases() {
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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
        let re = &*REGEXES;

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

    // =========================================================================
    // Integration tests using tempdir fixtures
    // =========================================================================

    mod integration {
        use super::*;
        use crate::cli::OutputFormat;
        use crate::contracts::test_utils::REGEXES;
        use crate::contracts::Contracts;
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
            c: Contracts,
        }

        impl TestFixture {
            /// Create a new test fixture with the given agents and commands.
            fn new(agents: &[(&str, &str)], commands: &[(&str, &str)]) -> anyhow::Result<Self> {
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
                let c = Contracts::default();

                Ok(Self {
                    _temp_dir: temp_dir,
                    ctx,
                    inv,
                    c,
                })
            }

            /// Create a CheckCtx from this fixture.
            fn check_ctx(&self) -> CheckCtx<'_> {
                CheckCtx {
                    ctx: &self.ctx,
                    inv: &self.inv,
                    re: &REGEXES,
                    c: &self.c,
                }
            }
        }

        /// Create a Reporter for testing.
        fn test_reporter() -> Reporter {
            Reporter::new(OutputFormat::Text, false, false)
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 3 tests - Machine Summary is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_critics_machine_summary_is_noop() {
            // The deprecated check should always pass (no-op)
            let critic_content = r#"# Requirements Critic
No Machine Summary section at all.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_machine_summary(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
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

            let fixture = TestFixture::new(&[("signal-cleanup", cleanup_content)], &[])
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

            let fixture = TestFixture::new(&[("signal-cleanup", cleanup_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_receipts(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing receipt reference"
            );
        }

        #[test]
        fn test_check_cleanup_receipts_missing_index() {
            let cleanup_content = r#"# Signal Cleanup

Write the signal_receipt.json to the flow directory.

No index update here.
"#;

            let fixture = TestFixture::new(&[("signal-cleanup", cleanup_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_receipts(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing index.json reference"
            );
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

            let fixture = TestFixture::new(&[("gh-reporter", reporter_content)], &[])
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

            let fixture = TestFixture::new(&[("gh-reporter", reporter_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_reporter_output(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.errors > 0,
                "Should have errors for missing Safe Output Contract"
            );
        }

        #[test]
        fn test_check_gh_reporter_output_missing_constraint() {
            let reporter_content = r#"# GH Reporter

## Safe Output Contract

This agent posts summaries.
"#;

            let fixture = TestFixture::new(&[("gh-reporter", reporter_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_reporter_output(&cx, &mut rep).expect("Check failed");
            assert!(
                rep.warnings > 0,
                "Should have warning for missing output constraint doc"
            );
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

            let fixture = TestFixture::new(&[("repo-operator", operator_content)], &[])
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

            let fixture = TestFixture::new(&[("repo-operator", operator_content)], &[])
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

            let fixture = TestFixture::new(&[("repo-operator", operator_content)], &[])
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
            assert!(
                rep.errors > 0,
                "Should have error for missing repo-operator.md"
            );
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

            let fixture = TestFixture::new(&[("gh-reporter", reporter_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gh_agents_two_gates(&cx, &mut rep).expect("Check failed");
            assert!(rep.errors > 0, "Should have errors for missing gate");
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 28 tests - Status enum is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_status_enum_is_noop() {
            // The deprecated check should always pass (no-op)
            let critic_content = r#"# Requirements Critic
No status enum at all.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_status_enum(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 29 tests - Recommended action enum is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_recommended_action_enum_is_noop() {
            // The deprecated check should always pass (no-op)
            let critic_content = r#"# Requirements Critic
No recommended_action at all.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_recommended_action_enum(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 31 tests - Route fields are no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_route_fields_is_noop() {
            // The deprecated check should always pass (no-op)
            let critic_content = r#"# Requirements Critic
No route fields at all.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_route_fields(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
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

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
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

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
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

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cannot_proceed_invariant(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "No CANNOT_PROCEED mention should pass");
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 33 tests - can_further_iteration_help is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_critics_iteration_help_is_noop() {
            // The deprecated check should always pass (no-op)
            let critic_content = r#"# Requirements Critic
No iteration help field at all.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_iteration_help(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 34 tests - Cleanup route_to_flow is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_cleanup_route_to_flow_is_noop() {
            // The deprecated check should always pass (no-op)
            let cleanup_content = r#"# Signal Cleanup
No route_to_flow at all.
"#;

            let fixture = TestFixture::new(&[("signal-cleanup", cleanup_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_cleanup_route_to_flow(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
            assert_eq!(
                rep.warnings, 0,
                "Deprecated check should be no-op (no warnings)"
            );
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 35 tests - Gate unified action is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_gate_unified_action_is_noop() {
            // The deprecated check should always pass (no-op)
            let enforcer_content = r#"# Contract Enforcer
No recommended_action at all.
"#;

            let fixture = TestFixture::new(&[("contract-enforcer", enforcer_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_gate_unified_action(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
            assert_eq!(
                rep.warnings, 0,
                "Deprecated check should be no-op (no warnings)"
            );
        }

        // ---------------------------------------------------------------------
        // DEPRECATED: Check 51 tests - observations field is no longer enforced
        // These tests verify the deprecated check is now a no-op
        // ---------------------------------------------------------------------

        #[test]
        fn test_deprecated_check_critics_observations_field_is_noop() {
            // The deprecated check should always pass (no-op)
            let critic_content = r#"# Requirements Critic
No observations field at all.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_observations_field(&cx, &mut rep).expect("Check failed");
            assert_eq!(
                rep.errors, 0,
                "Deprecated check should be no-op (always pass)"
            );
        }

        // ---------------------------------------------------------------------
        // NEW CLAUDE-NATIVE CHECKS: Check 54 and 55 tests
        // ---------------------------------------------------------------------

        #[test]
        fn test_check_critics_handoff_section_pass() {
            let critic_content = r#"# Requirements Critic

## Handoff

**What was done:** Reviewed requirements.

**What's left:** Nothing.

**Recommendation:** Proceed to implementation.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_handoff_section(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
            assert_eq!(
                rep.warnings, 0,
                "Should have no warnings with Handoff section"
            );
        }

        #[test]
        fn test_check_critics_handoff_section_missing() {
            let critic_content = r#"# Requirements Critic

No Handoff section here.

Just some other content.
"#;

            let fixture = TestFixture::new(&[("requirements-critic", critic_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_critics_handoff_section(&cx, &mut rep).expect("Check failed");
            // Should warn (not fail) during transition period
            assert_eq!(rep.errors, 0, "Should have no errors (warning only)");
            assert!(
                rep.warnings > 0,
                "Should have warning for missing Handoff section"
            );
        }

        #[test]
        fn test_check_agents_clear_job_pass() {
            let implementer_content = r#"# Code Implementer

You are the **Code Implementer**.

Build working code. Run tests. Report what happened.
"#;

            let fixture = TestFixture::new(&[("code-implementer", implementer_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_agents_clear_job(&cx, &mut rep).expect("Check failed");
            assert_eq!(rep.errors, 0, "Should have no errors");
            assert_eq!(rep.warnings, 0, "Should have no warnings with clear job");
        }

        #[test]
        fn test_check_agents_clear_job_missing() {
            let implementer_content = r#"# Code Implementer

Some vague content without clear job description.
"#;

            let fixture = TestFixture::new(&[("code-implementer", implementer_content)], &[])
                .expect("Failed to create fixture");

            let cx = fixture.check_ctx();
            let mut rep = test_reporter();

            check_agents_clear_job(&cx, &mut rep).expect("Check failed");
            // Should warn (not fail) during transition period
            assert_eq!(rep.errors, 0, "Should have no errors (warning only)");
            assert!(
                rep.warnings > 0,
                "Should have warning for missing clear job section"
            );
        }
    }
}
