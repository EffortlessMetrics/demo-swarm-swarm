# Scope Estimate

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

counts:
functional_requirements: 6
non_functional_requirements: 6
bdd_scenarios: 40
open_questions: 10
integration_points: 3

scope:
tshirt_size: M
confidence: High

## Rationale (why this size)

- **Requirements**: 6 functional requirements (REQ-001 through REQ-006) with 4 acceptance criteria each = 24 testable criteria. 6 non-functional requirements with 2-3 metrics each. Well-scoped and traceable.

- **Scenarios**: 40 BDD scenarios across 6 feature files (updated from 39 in iteration 1). Good coverage depth but not excessive breadth. Coverage matrix shows all requirements have Happy/Edge/Error paths where applicable.

- **Integrations**: 3 main integration points:
  1. `tools/demoswarm-pack-check/` (Rust validation rules in drift.rs, control_plane.rs, structure.rs, contracts.rs)
  2. `scripts/check-doc-drift.sh` (Bash drift guards, though likely not extended per ASM-002 and OQ-SIG-005)
  3. `.runs/` artifacts (open_questions.md pattern validation, receipts for handshake testing)

- **NFR weight**: PERF (< 30s CI runtime), COMP (backward compatibility), and MAINT (pattern maintainability) are the key NFRs. All have concrete metrics defined in verification_notes.md.

- **Risk profile**: 1 HIGH risk (RSK-001: prior #49 bounce), 4 MEDIUM risks, 3 LOW risks. The HIGH risk is mitigated by warning-first design (REQ-005) and narrower scope (syntactic checks only).

## Complexity Drivers

- **Rust implementation required**: New validation rules go into pack-check (Rust), requiring Rust development skills (REQ-001, REQ-002, REQ-003, REQ-005). contracts.rs must be extended with constant definitions.

- **Prior work bounced**: Issue #49 (align-doc-ownership) bounced at Gate; this work must avoid same pitfalls (RSK-001). Explicit mitigation: narrower scope, warning-first mode.

- **Documentation inconsistency**: PLN vs PLAN prefix discrepancy needs resolution before REQ-003 can be finalized (OQ-SIG-002). ASM-003 assumes PLN/BLD is canonical.

- **Agent remediation alongside validation**: 4 agents missing Skills sections need content fixes concurrent with rule development (RSK-003). Must audit to enumerate before Build.

- **Warning-first infrastructure**: REQ-005 requires --strict flag plumbing through pack-check, affecting exit codes and output format. Foundation for ST1-ST3.

## Suggested Decomposition (for Plan/Work Planner)

- **ST1: Warning-First Mode Infrastructure (REQ-005)** -- pack-check --strict flag implementation affecting all new rules. Should be implemented first as foundation for ST2-ST4. Separable because it's infrastructure, not a validation rule.

- **ST2: Flow Boundary Enforcement (REQ-001)** -- Rust rule in drift.rs; scans flow commands for demoswarm.sh and skill subcommands. Separable because it has no dependencies on other new rules (uses ST1 infrastructure).

- **ST3: Skills Section Enforcement (REQ-002)** -- Rust rule scanning agents for demoswarm.sh + Skills heading. Separable because it targets different file set (.claude/agents/) than ST2.

- **ST4: OpenQ Prefix Validation (REQ-003)** -- Rust rule validating QID patterns in open_questions.md. Separable because it targets .runs/ artifacts, not .claude/ files.

- **ST5: Build-to-Gate Handshake Test Fixtures (REQ-004)** -- Test fixtures and documentation for receipt validation. Separable because it's test infrastructure, not validation rules. Per OQ-SIG-010, committed fixtures preferred.

- **ST6: No False Positives Baseline (REQ-006)** -- Validation that existing artifacts pass new rules. Should be final verification after ST1-ST5 complete. Separable because it's acceptance testing of the other work.

## Confidence Notes

- **What would change the estimate**:
  - If #49 work resumes and conflicts (RSK-007): could increase scope to L if coordination required.
  - If OQ-SIG-002 resolves to PLAN/BUILD (not PLN/BLD): would require updating openq-tools Rust code, adding implementation effort and potentially invalidating existing QIDs.
  - If warning-first mode is rejected (OQ-SIG-001): would require more careful rollout, potentially increasing scope.
  - If more than 4 agents are missing Skills sections: would increase remediation effort.
  - If dynamic skill list discovery is required (OQ-SIG-006 resolved to dynamic): adds complexity to REQ-001 implementation.

- **Why M not L**:
  - Requirements are well-defined with clear acceptance criteria (all 6 REQs have 4 ACs each)
  - Scenarios are comprehensive but not excessive (40 scenarios covering 6 REQs)
  - Integration points are limited to existing tooling (no new tools)
  - Risk mitigations are documented (warning-first mode)
  - Both requirements_critique.md and bdd_critique.md show VERIFIED with `can_further_iteration_help: no`

- **Why M not S**:
  - Rust implementation required (not trivial scripting)
  - 10 open questions indicate some design uncertainty (though all have suggested defaults)
  - Prior related work bounced at Gate (empirical complexity evidence from #49)
  - 40 scenarios is significant test surface
  - 8 risks identified (1 HIGH, 4 MEDIUM, 3 LOW)

## Iteration 2 Changes

- Updated counts: 40 BDD scenarios (was 39), 10 open questions (was 9).
- Reordered suggested decomposition: ST1 (warning-first mode) is now first, as it's infrastructure foundation.
- Added RSK-008 reference to confidence notes (skill list drift risk).
- Added OQ-SIG-010 reference to ST5 (fixture location decision).
- Added reference to critiques showing VERIFIED + `can_further_iteration_help: no` (spec stability evidence).
- Risk count updated: 8 total (1 HIGH, 4 MEDIUM, 3 LOW).
