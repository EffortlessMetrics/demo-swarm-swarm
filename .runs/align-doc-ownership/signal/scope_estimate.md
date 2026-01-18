# Scope Estimate

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

counts:
functional_requirements: 7
non_functional_requirements: 3
bdd_scenarios: 31
open_questions: 6
integration_points: 4

scope:
tshirt_size: M
confidence: High

## Rationale (why this size)

- **Requirements**: 7 functional requirements (REQ-001 through REQ-007) and 3 NFRs (MAINT, TEST, REGR). The requirements are well-specified with clear acceptance criteria. This is moderate complexity for documentation alignment.

- **Scenarios**: 31 BDD scenarios covering all requirements. Good coverage but scenarios are inspection-based (documentation checks), not runtime behavior. Lower execution complexity than integration tests.

- **Integrations**: 4 integration points identified:
  1. pack-check (may need Rust additions for boundary enforcement)
  2. doc-drift (`scripts/check-doc-drift.sh`) (existing, likely minimal changes)
  3. validation-log.md (new entries required)
  4. Toy Run A/B (flows 1-4 execution dependency)

- **NFR weight**: Maintainability (MAINT) and Test Tooling (TEST) are primary concerns. No security, performance, or compliance weight. This reduces risk profile.

- **Risk profile**: 1 HIGH risk (RSK-001: merge conflicts), 3 MEDIUM risks (RSK-002 through RSK-004), 2 LOW risks (RSK-005, RSK-006). The HIGH risk is mitigatable via sequential subtask execution and tight `touches` patterns.

## Complexity Drivers

1. **55 agent files require consistency audit**: Large surface area for REQ-002 (Agent Doc Consistency). Each file must be inspected for enum usage, Skills sections, and file-write rules. Source: context_brief.md.

2. **pack-check boundary enforcement additions**: New rules may require Rust development in `tools/demoswarm-pack-check/`. Complexity depends on whether shell script fallback is acceptable. Source: OQ-SIG-006, RSK-003.

3. **ST-004 scope concentration**: ST-004 carries Gate flow, cross-cutting enforcement, and CLAUDE.md normalization. Heavier than ST-001 through ST-003. Source: problem_statement.md (Concerns), RSK-002.

4. **Validation run dependency**: Work cannot be marked complete until Toy Run A/B (flows 1-4) succeeds and is recorded. Source: REQ-006, RSK-005.

5. **6 open questions with defaults**: All questions (OQ-SIG-001 through OQ-SIG-006) have suggested defaults and can proceed without blocking. Defaults are reasonable; Plan phase should confirm. Source: open_questions.md.

## Suggested Decomposition (for Plan/Work Planner)

- **ST-001: Flow 1 (Signal) alignment**: Audit signal-related agents, flow-1-signal.md. Distinct `touches` pattern: `*signal*.md`. Low merge conflict risk.

- **ST-002: Flow 2 (Plan) alignment**: Audit plan-related agents, flow-2-plan.md. Distinct `touches` pattern: `*plan*.md`. Low merge conflict risk.

- **ST-003: Flow 3 (Build) alignment**: Audit build-related agents, flow-3-build.md. Distinct `touches` pattern: `*build*.md`. Low merge conflict risk.

- **ST-004: Flow 4 (Gate) + cross-cutting + CLAUDE.md**: Heavier subtask. Audit gate-related agents, flow-4-gate.md, add pack-check boundary rules, normalize CLAUDE.md Skills table. Consider splitting if timeline pressure.

- **ST-005: Flow 5 (Deploy) alignment**: Audit deploy-related agents, flow-5-deploy.md. Distinct `touches` pattern: `*deploy*.md`. Low merge conflict risk.

- **ST-006: Flow 6 (Wisdom) + validation run**: Audit wisdom-related agents, flow-6-wisdom.md, execute Toy Run A/B, record in validation-log.md. Validation run is blocking for completion.

## Confidence Notes

- **What would change the estimate**:
  - If pack-check Rust development is complex (new parser, AST manipulation), estimate moves to **L**. Impact: RSK-003 severity increases.
  - If merge conflicts occur despite tight `touches` patterns, estimate stays **M** but timeline extends. Impact: RSK-001 triggers.
  - If validation run (Toy Run A/B) reveals regressions, estimate stays **M** but requires debugging iteration. Impact: RSK-005 triggers.

- **Confidence is High because**:
  - All requirements have clear acceptance criteria (requirements_critique: 0 major/critical issues).
  - All scenarios are testable (bdd_critique: 0 major/critical issues).
  - Open questions have reasonable defaults and are non-blocking.
  - Prior alignment commits (be0c81a, 186ea53) established trajectory; this work continues that direction.
  - Subtask partitioning (REQ-005) provides clean decomposition with minimal overlap.

## Scope Assessor Result

status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
tshirt_size: M
confidence: High
missing_required: []
blockers: []
