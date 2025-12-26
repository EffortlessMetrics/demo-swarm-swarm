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
            "recommended_action drifted (expected: recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV)",
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

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::headings;
    use crate::contracts::Regexes;

    // -------------------------------------------------------------------------
    // Smoke signal regex tests (Check 36)
    // -------------------------------------------------------------------------

    /// Smoke signal enum line is detected correctly.
    #[test]
    fn test_smoke_signal_regex_valid() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let valid = "smoke_signal: STABLE | INVESTIGATE | ROLLBACK";
        assert!(
            re.smoke_signal.is_match(valid),
            "Should match valid smoke_signal line"
        );

        // With indentation
        let indented = "  smoke_signal: STABLE | INVESTIGATE | ROLLBACK";
        assert!(
            re.smoke_signal.is_match(indented),
            "Should match with indentation"
        );
    }

    /// Invalid smoke signal lines are rejected.
    #[test]
    fn test_smoke_signal_regex_invalid() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Wrong order
        let wrong_order = "smoke_signal: INVESTIGATE | STABLE | ROLLBACK";
        assert!(
            !re.smoke_signal.is_match(wrong_order),
            "Should reject wrong order"
        );

        // Missing value
        let missing = "smoke_signal: STABLE | INVESTIGATE";
        assert!(
            !re.smoke_signal.is_match(missing),
            "Should reject missing value"
        );

        // Wrong field name
        let wrong_field = "signal: STABLE | INVESTIGATE | ROLLBACK";
        assert!(
            !re.smoke_signal.is_match(wrong_field),
            "Should reject wrong field name"
        );
    }

    /// Smoke signal with whitespace variations.
    #[test]
    fn test_smoke_signal_whitespace() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // Extra spaces
        let extra_spaces = "smoke_signal:  STABLE  |  INVESTIGATE  |  ROLLBACK";
        assert!(
            re.smoke_signal.is_match(extra_spaces),
            "Should match with extra spaces"
        );

        // Tab indentation
        let tabbed = "\tsmoke_signal: STABLE | INVESTIGATE | ROLLBACK";
        assert!(
            re.smoke_signal.is_match(tabbed),
            "Should match with tab indentation"
        );

        // Trailing whitespace
        let trailing = "smoke_signal: STABLE | INVESTIGATE | ROLLBACK   ";
        assert!(
            re.smoke_signal.is_match(trailing),
            "Should match with trailing whitespace"
        );
    }

    // -------------------------------------------------------------------------
    // Smoke Verifier Result heading tests (Check 36)
    // -------------------------------------------------------------------------

    /// Smoke Verifier Result heading constant is correct.
    #[test]
    fn test_smoke_verifier_result_heading() {
        assert_eq!(
            headings::SMOKE_VERIFIER_RESULT_H2,
            "## Smoke Verifier Result",
            "Smoke Verifier Result heading should be exact"
        );
    }

    /// Smoke Verifier Result heading detection in content.
    #[test]
    fn test_smoke_verifier_result_in_content() {
        let with_heading = r#"
Some content

## Smoke Verifier Result
smoke_signal: STABLE | INVESTIGATE | ROLLBACK
"#;
        assert!(with_heading.contains(headings::SMOKE_VERIFIER_RESULT_H2));

        let without_heading = r#"
Some content

## Other Result
status: VERIFIED
"#;
        assert!(!without_heading.contains(headings::SMOKE_VERIFIER_RESULT_H2));
    }

    // -------------------------------------------------------------------------
    // REG marker tests (Check 41)
    // -------------------------------------------------------------------------

    /// REG marker literal constant is correct.
    #[test]
    fn test_reg_marker_literal() {
        let contracts = crate::contracts::Contracts::default();

        assert_eq!(
            contracts.reg_marker_literal, "^### REG-[0-9]{3}:",
            "REG marker literal should be the heading-based regex"
        );
    }

    /// REG marker grep regex matches expected patterns.
    #[test]
    fn test_grep_reg_marker_regex() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // The grep pattern for heading-based REG markers
        // The regex pattern `grep.*\^### REG-\[0-9\]\{3\}:` expects:
        // - literal caret (^) for line start
        // - literal brackets [] and braces {} in the grep command
        // These are the grep BRE syntax characters
        let grep_cmd = r#"grep "^### REG-[0-9]{3}:""#;
        assert!(
            re.grep_reg_marker.is_match(grep_cmd),
            "Should match heading-based REG marker grep"
        );

        // Legacy list-based pattern should not match (no ^### prefix)
        let legacy_grep = r#"grep "- REG-[0-9]{3}:""#;
        assert!(
            !re.grep_reg_marker.is_match(legacy_grep),
            "Should not match legacy list-based REG marker grep"
        );
    }

    /// Legacy list-based REG marker detection.
    #[test]
    fn test_legacy_reg_marker_detection() {
        // This is what the check looks for to flag legacy patterns
        let legacy_pattern = "- REG-NNN:";
        let content_with_legacy = "Document REG markers as:\n- REG-NNN: description";

        assert!(content_with_legacy.contains(legacy_pattern));

        let content_with_heading = "Document REG markers as:\n### REG-001: description";
        assert!(!content_with_heading.contains(legacy_pattern));
    }

    // -------------------------------------------------------------------------
    // Swarm-Proposed status tests (Check 24)
    // -------------------------------------------------------------------------

    /// Swarm-Proposed status detection in content.
    #[test]
    fn test_swarm_proposed_detection() {
        let with_status = "ADRs should use status: Swarm-Proposed";
        assert!(with_status.contains("Swarm-Proposed"));

        let without_status = "ADRs should use status: Proposed";
        assert!(!without_status.contains("Swarm-Proposed"));
    }

    /// Swarm-Proposed in ADR template context.
    #[test]
    fn test_swarm_proposed_in_adr() {
        let adr_content = r#"
# ADR-001: Example Decision

## Status
Swarm-Proposed

## Context
...
"#;
        assert!(adr_content.contains("Swarm-Proposed"));
    }

    // -------------------------------------------------------------------------
    // Domain verdict separation tests (Check 36)
    // -------------------------------------------------------------------------

    /// smoke_signal field detection in content.
    #[test]
    fn test_smoke_signal_field_detection() {
        let with_field = "smoke_signal: STABLE";
        assert!(with_field.contains("smoke_signal:"));

        let without_field = "signal: STABLE";
        assert!(!without_field.contains("smoke_signal:"));
    }

    /// Domain verdict vs Machine Summary status are separate.
    #[test]
    fn test_domain_verdict_separation() {
        // smoke-verifier should have BOTH:
        // 1. Machine Summary with status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
        // 2. Smoke Verifier Result with smoke_signal: STABLE | INVESTIGATE | ROLLBACK

        let correct_content = r#"
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV

## Smoke Verifier Result
smoke_signal: STABLE | INVESTIGATE | ROLLBACK
"#;

        // Has Machine Summary status
        assert!(correct_content.contains("status: VERIFIED | UNVERIFIED | CANNOT_PROCEED"));

        // Has separate smoke_signal
        assert!(correct_content.contains("smoke_signal: STABLE | INVESTIGATE | ROLLBACK"));

        // Wrong: mixing domains (using ROLLBACK in Machine Summary)
        let wrong_content = "status: VERIFIED | UNVERIFIED | ROLLBACK";
        assert!(!wrong_content.contains("CANNOT_PROCEED"));
    }

    // -------------------------------------------------------------------------
    // Flow 6 document structure tests
    // -------------------------------------------------------------------------

    /// Flow 6 should reference regression markers.
    #[test]
    fn test_flow6_regression_references() {
        let contracts = crate::contracts::Contracts::default();

        // Flow 6 documentation should contain the REG marker literal
        let expected_marker = contracts.reg_marker_literal;
        assert!(
            !expected_marker.is_empty(),
            "REG marker literal should be defined"
        );
    }

    /// wisdom-cleanup should be alignable to heading-based markers.
    #[test]
    fn test_wisdom_cleanup_marker_alignment() {
        let contracts = crate::contracts::Contracts::default();

        // The heading-based marker pattern
        let marker = contracts.reg_marker_literal;

        // Should be a regex for heading-based format
        assert!(
            marker.starts_with("^### "),
            "Marker should be heading-based"
        );
        assert!(marker.contains("REG-"), "Marker should contain REG prefix");
    }

    // -------------------------------------------------------------------------
    // Issue-First Invariant heading tests
    // -------------------------------------------------------------------------

    /// Issue-First Invariant heading constant is correct.
    #[test]
    fn test_issue_first_invariant_heading() {
        assert_eq!(
            headings::ISSUE_FIRST_INVARIANT,
            "Issue-First Invariant",
            "Issue-First Invariant heading should be exact"
        );
    }

    // -------------------------------------------------------------------------
    // Decision Spine heading tests
    // -------------------------------------------------------------------------

    /// Decision Spine heading constant is correct.
    #[test]
    fn test_decision_spine_heading() {
        assert_eq!(
            headings::DECISION_SPINE,
            "Decision Spine",
            "Decision Spine heading should be exact"
        );
    }

    /// Decision Spine detection in content.
    #[test]
    fn test_decision_spine_in_content() {
        let with_spine = "The ADR uses Decision Spine for structured decisions.";
        assert!(with_spine.contains(headings::DECISION_SPINE));

        let without_spine = "The ADR uses structured decisions.";
        assert!(!without_spine.contains(headings::DECISION_SPINE));
    }

    // -------------------------------------------------------------------------
    // Smoke verifier complete contract tests
    // -------------------------------------------------------------------------

    /// smoke-verifier requires all domain verdict components.
    #[test]
    fn test_smoke_verifier_complete_contract() {
        let complete_content = r#"
## Smoke Verifier Result
smoke_signal: STABLE | INVESTIGATE | ROLLBACK

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
"#;

        // Check all required components
        assert!(complete_content.contains("smoke_signal:"));
        assert!(complete_content.contains("STABLE | INVESTIGATE | ROLLBACK"));
        assert!(complete_content.contains(headings::SMOKE_VERIFIER_RESULT_H2));
        assert!(complete_content.contains("recommended_action:"));
    }

    /// Missing smoke_signal field is detectable.
    #[test]
    fn test_missing_smoke_signal() {
        let missing_signal = r#"
## Smoke Verifier Result
status: STABLE

## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
"#;
        assert!(!missing_signal.contains("smoke_signal:"));
    }

    /// Missing Smoke Verifier Result heading is detectable.
    #[test]
    fn test_missing_smoke_verifier_result() {
        let missing_heading = r#"
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

smoke_signal: STABLE
"#;
        assert!(!missing_heading.contains(headings::SMOKE_VERIFIER_RESULT_H2));
    }

    // -------------------------------------------------------------------------
    // Canon action in smoke-verifier tests
    // -------------------------------------------------------------------------

    /// smoke-verifier should have canonical recommended_action.
    #[test]
    fn test_smoke_verifier_canon_action() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        let with_canon_action = "recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV";
        assert!(re.canon_action.is_match(with_canon_action));

        // Should NOT use domain-specific actions in recommended_action
        let with_domain_action = "recommended_action: ROLLBACK";
        assert!(!re.canon_action.is_match(with_domain_action));
    }

    /// Domain actions belong in smoke_signal, not recommended_action.
    #[test]
    fn test_domain_actions_in_smoke_signal() {
        let re = Regexes::compile().expect("Failed to compile regexes");

        // ROLLBACK and INVESTIGATE are domain-specific
        let smoke_signal_line = "smoke_signal: STABLE | INVESTIGATE | ROLLBACK";
        assert!(re.smoke_signal.is_match(smoke_signal_line));

        // These should NOT appear in recommended_action line
        let bad_rec_action = "recommended_action: INVESTIGATE";
        assert!(!re.canon_action.is_match(bad_rec_action));
    }
}
