//! Wisdom and domain-specific checks.
//!
//! Checks: 24, 36, 41

use super::contracts::headings;
use crate::reporter::Reporter;

use super::{CheckCtx, CheckSpec};

pub fn checks() -> Vec<CheckSpec> {
    vec![
        CheckSpec {
            id: 24,
            title: "Checking Swarm-Proposed status in ADR author...",
            run: check_swarm_proposed,
        },
        CheckSpec {
            id: 36,
            title: "Checking smoke-verifier domain verdict separation...",
            run: check_smoke_verifier,
        },
        CheckSpec {
            id: 41,
            title: "Checking Flow 6 regression markers are grep-stable...",
            run: check_regression_markers,
        },
    ]
}

/// Check 24: Swarm-Proposed status in ADR author.
fn check_swarm_proposed(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(adr_author) = cx.inv.agent("adr-author") {
        let content = cx.ctx.read_utf8(adr_author)?;
        if content.contains("Swarm-Proposed") {
            rep.pass("adr-author.md uses Swarm-Proposed status");
        } else {
            rep.fail("adr-author.md does NOT use Swarm-Proposed status");
        }
    }

    Ok(())
}

/// Check 36: smoke-verifier domain verdict separation.
fn check_smoke_verifier(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    let Some(smoke_verifier) = cx.inv.agent("smoke-verifier") else {
        rep.fail("smoke-verifier.md MISSING");
        return Ok(());
    };

    let content = cx.ctx.read_utf8(smoke_verifier)?;
    let mut issues = Vec::new();

    if !content.contains("smoke_signal:") {
        issues.push("smoke_signal field missing");
    }
    if !cx.re.smoke_signal.is_match(&content) {
        issues.push("smoke_signal enum line drifted (expected: STABLE | INVESTIGATE | ROLLBACK)");
    }
    if !cx.re.canon_action.is_match(&content) {
        issues.push(
            "recommended_action drifted (expected: recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV)",
        );
    }
    if !content.contains(headings::SMOKE_VERIFIER_RESULT_H2) {
        issues.push("## Smoke Verifier Result block missing");
    }

    if issues.is_empty() {
        rep.pass("smoke-verifier has correct domain verdict separation");
    } else {
        rep.fail(format!("smoke-verifier issues: {}", issues.join(" ")));
    }

    Ok(())
}

/// Check 41: Flow 6 regression markers match wisdom-cleanup (grep-stable).
fn check_regression_markers(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if let Some(flow6_wisdom) = cx.inv.command("flow-6-wisdom") {
        let content = cx.ctx.read_utf8(flow6_wisdom)?;
        if content.contains(cx.c.reg_marker_literal) {
            rep.pass(format!(
                "Flow 6 documents heading-based REG marker ({})",
                cx.c.reg_marker_literal
            ));
        } else if content.contains("- REG-NNN:") {
            rep.fail(
                "Flow 6 still documents list-based REG markers (- REG-NNN:) (should use headings)",
            );
        } else {
            rep.warn("Flow 6 may be missing stable regression marker documentation");
        }
    } else {
        rep.warn("flow-6-wisdom.md not found (cannot validate regression marker docs)");
    }

    if let Some(wisdom_cleanup) = cx.inv.agent("wisdom-cleanup") {
        let content = cx.ctx.read_utf8(wisdom_cleanup)?;
        if content.contains(cx.c.reg_marker_literal) {
            rep.pass(format!(
                "wisdom-cleanup references heading-based REG marker ({})",
                cx.c.reg_marker_literal
            ));
        } else if cx.re.grep_reg_marker.is_match(&content) {
            rep.pass("wisdom-cleanup grep appears to use heading REG marker");
        } else {
            rep.warn("wisdom-cleanup may not be aligned to heading-based REG markers");
        }
    } else {
        rep.warn("wisdom-cleanup.md not found (cannot validate marker alignment)");
    }

    Ok(())
}
