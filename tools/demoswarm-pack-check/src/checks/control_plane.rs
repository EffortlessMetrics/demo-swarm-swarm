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
            title: "Checking flow commands gate GH ops on proceed_to_github_ops...",
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
                "{critic} recommended_action drifted (expected: recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV)"
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
                "{agent} recommended_action drifted (expected: recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV)"
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
