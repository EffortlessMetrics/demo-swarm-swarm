# Gate Fix Summary for compliance-drift-proofing

## Scope & Evidence

### Gate Artifacts Consulted

- `.runs/compliance-drift-proofing/gate/receipt_audit.md` (UNVERIFIED)
- `.runs/compliance-drift-proofing/gate/contract_compliance.md` (VERIFIED)
- `.runs/compliance-drift-proofing/gate/coverage_audit.md` (UNVERIFIED)
- `.runs/compliance-drift-proofing/gate/security_scan.md` (VERIFIED)
- `.runs/compliance-drift-proofing/gate/policy_analysis.md` (UNVERIFIED)

### Evidence Summary

The gate has surfaced:

1. **Mechanical formatting drift** (deterministic, automated fix via `cargo fmt`)
2. **Non-mechanical integrity failures** (coverage/test count discrepancies requiring data reconciliation)
3. **Non-mechanical policy violations** (receipt aspirational claims vs actual artifact state)

This report focuses exclusively on **mechanical fix-forward eligibility**. The test count inflation (420 vs 294), coverage metric mismatch (89.29% vs 75.12%), and receipt integrity failures are **non-mechanical** and require Flow 3 (Build) reseal and verification.

## Mechanical Fixes (apply in Flow 3)

### MECH-001: Rust formatting drift (cargo fmt)

**Evidence:**

- `cargo fmt --check` exit code 1 indicates formatting violations across 8 source files
- Drift pattern: Line wrapping and assertion formatting (deterministic, formatter-applied)
- Files affected: `control_plane.rs`, `flow.rs`, `structure.rs`, `wisdom.rs`, `ctx.rs`, `reporter.rs`, `util.rs`, `check_integration_test.rs`

**Files/Paths:**

- `tools/demoswarm-pack-check/src/checks/control_plane.rs`
- `tools/demoswarm-pack-check/src/checks/flow.rs`
- `tools/demoswarm-pack-check/src/checks/structure.rs`
- `tools/demoswarm-pack-check/src/checks/wisdom.rs`
- `tools/demoswarm-pack-check/src/ctx.rs`
- `tools/demoswarm-pack-check/src/reporter.rs`
- `tools/demoswarm-pack-check/src/util.rs`
- `tools/demoswarm-pack-check/tests/check_integration_test.rs`

**Category:** `format`

**Suggested Command:**

```bash
cd tools/demoswarm-pack-check && cargo fmt
```

**Why mechanical:**
Formatting is deterministic and does not change program behavior; cargo fmt applies idiomatic Rust style rules automatically with no judgment required.

---

## Non-Mechanical Findings (for merge-decider context)

### NONMECH-001: Test count fabrication in receipt

**Evidence:**

- `receipt_audit.md` (CRITICAL): Build receipt claims "420 passed (379 unit + 41 integration)" but `test_execution.md` canonical artifact documents only "253 unit + 41 integration = 294 total"
- Discrepancy: 126 tests (42.8% inflation)
- Root cause: Receipt updated aspirationally during reseal without canonical test artifact refresh

**Likely Target:** `Flow 3 (Build)`

**Why not mechanical:**
Reconciling test counts requires re-running test suite and regenerating test_execution.md artifact; this is a build reseal activity, not a formatting/import/hygiene fix.

---

### NONMECH-002: Coverage metric mismatch and evidence inconsistency

**Evidence:**

- `coverage_audit.md` (CRITICAL): 14.17 percentage-point gap between build_receipt.json (89.29%) and test_execution.md (75.12%)
- test_execution.md status UNVERIFIED due to coverage shortfall (below 80% threshold); build_receipt.json claims VERIFIED
- Module-level coverage in drift.rs: 80.2% (test_execution.md) vs 87.1% (cleanup_report.md) — no merged report explains the delta
- Branch coverage metrics missing entirely (required by test plan; threshold 70%)

**Likely Target:** `Flow 3 (Build)`

**Why not mechanical:**
Coverage reconciliation requires test re-execution, coverage instrumentation refresh, and artifact resealing. This is not formatting/lint/import; it is data integrity and measurement reconciliation.

---

### NONMECH-003: Receipt policy violation (aspirational claims vs actual state)

**Evidence:**

- `policy_analysis.md` (NON-COMPLIANT): "POL-004: Quality gates sourced from Machine Summaries"
- Receipt claims `lint.clippy_status: CLEAN` but lint_report.md artifact status is UNVERIFIED with active Clippy warning
- Receipt claims coverage metrics as VERIFIED without reconciling to test_execution.md canonical source
- Violation: CLAUDE.md policy requires quality_gates to reflect actual Machine Summary status, not aspirational success

**Likely Target:** `Flow 3 (Build)` (build-cleanup agent)

**Why not mechanical:**
This requires understanding the root cause of the mismatch (stale receipt, incomplete reseal, or incomplete lint fix) and correcting the underlying build state before receipt can be resealed with integrity. Not a formatting issue.

---

## Fix-forward Plan (machine readable)

<!-- PACK-CONTRACT: FIX_FORWARD_PLAN_V1 START -->

```yaml
version: 1
fix_forward_eligible: false
scope:
  - FORMAT

rationale: "Formatting-only drift (cargo fmt deterministic). However, ineligible for fix-forward because non-mechanical blockers (coverage/test count fabrication, receipt integrity violation) must be resolved in Flow 3 Build before gate can be resealed. Formatting fix alone will not resolve UNVERIFIED gate status."

apply_steps: []

verify_steps: []

change_scope:
  allowed_globs: []
  deny_globs:
    - ".runs/**"
    - ".github/**"
  max_files_changed: 0
  max_diff_lines: 0

post_conditions:
  needs_build_reseal_if_code_changed: false
  requires_repo_operator_commit: false
  rerun_receipt_checker: false
  rerun_gate_fixer: false

on_failure:
  recommended_action: BOUNCE
  route_to_flow: 3
  route_to_station: build-cleanup
  route_to_agent: null
```

<!-- PACK-CONTRACT: FIX_FORWARD_PLAN_V1 END -->

### Plan Rationale

**fix_forward_eligible: false**

Although formatting (MECH-001) is deterministic and fixable:

1. **Gate remains UNVERIFIED** due to non-mechanical blockers:
   - Test count fabrication (126-test inflation; requires test re-execution)
   - Coverage metric mismatch (14.17% gap; requires coverage instrumentation and reseal)
   - Receipt policy violation (aspirational claims; requires integrity reconciliation)

2. **Mechanical fix alone insufficient**: Applying `cargo fmt` will not resolve the underlying data integrity failures. Gate status will remain UNVERIFIED.

3. **Proper routing**: Per CLAUDE.md non-negotiables, blockers that prevent VERIFIED status route to the upstream agent responsible for fixing them. Here, that is Flow 3 Build (build-cleanup / test-author).

**Recommendation to merge-decider:** Route back to Flow 3 Build with focus on:

- Re-running full test suite (unit + integration) with coverage instrumentation
- Refreshing test_execution.md artifact with authoritative counts and coverage metrics
- Resealing build_receipt.json with data derived from updated artifacts (not aspirational)
- Rerunning lint with `cargo clippy` to ensure VERIFIED state before receipt sealing

Once Build reseal completes, Gate can be rerun to verify all blockers are cleared.

---

## Inventory (machine countable)

- MECH_FIX: MECH-001 category=format paths=[tools/demoswarm-pack-check/src/checks/control_plane.rs,tools/demoswarm-pack-check/src/checks/flow.rs,tools/demoswarm-pack-check/src/checks/structure.rs,tools/demoswarm-pack-check/src/checks/wisdom.rs,tools/demoswarm-pack-check/src/ctx.rs,tools/demoswarm-pack-check/src/reporter.rs,tools/demoswarm-pack-check/src/util.rs,tools/demoswarm-pack-check/tests/check_integration_test.rs]
- NON_MECH: NONMECH-001 target_flow=3
- NON_MECH: NONMECH-002 target_flow=3
- NON_MECH: NONMECH-003 target_flow=3
- MECH_FIX_FORWARD_ELIGIBLE: false
- MECH_FIX_CATEGORY: format

---

## Machine Summary

```yaml
status: VERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_station: build-cleanup
route_to_agent: null

blockers:
  - "Coverage/test integrity failures (non-mechanical): test count inflation 420 vs 294; coverage 89.29% vs 75.12%; requires data reconciliation"
  - "Receipt policy violation (non-mechanical): aspirational claims vs actual artifact state; requires Build reseal"

missing_required:
  - "Updated test_execution.md with authoritative test/coverage counts from resealed test run"
  - "Reconciliation of coverage metrics across build_receipt.json, test_execution.md, cleanup_report.md"
  - "Lint status reconciliation (receipt claims CLEAN; lint_report.md status UNVERIFIED)"

concerns:
  - "Mechanical formatting fix (MECH-001) is fixable but insufficient; gate remains UNVERIFIED due to non-mechanical blockers"
  - "Reseal operation in prior commit updated receipt without updating canonical test execution artifact; audit trail missing"
  - "Six critical/major findings in coverage_audit.md and policy_analysis.md require upstream resolution in Build"

observations:
  - "Formatting drift is minor and purely cosmetic; does not affect code behavior or test results"
  - "Core mechanical issue: receipt was sealed with aspirational claims rather than verified state from artifacts"
  - "Pack integrity depends on strict enforcement of artifact-driven receipt sealing; this run demonstrates risk when that is bypassed"

can_further_iteration_help: "no"

severity_summary:
  critical: 3
  major: 4
  minor: 1
```

---

## Rationale for Non-Eligibility

### The Core Issue

The gate discovered a **receipt integrity failure**: the build_receipt.json was updated to claim success (status=VERIFIED, coverage=89.29%) without corresponding updates to the canonical test_execution.md artifact or reconciliation of discrepancies.

This violates **CLAUDE.md policy POL-004**: "quality_gates sourced from agent Machine Summaries". The receipt should reflect actual measured state, not aspirational claims.

### Why Formatting Alone Is Insufficient

1. **Gate status depends on evidence integrity, not code formatting.**
   - Applying `cargo fmt` changes no test outputs, no coverage measurements, no receipt contents.
   - After formatting, gate will still be UNVERIFIED due to evidence blockers.

2. **Fix-forward is bounded hygiene, not a general-purpose iterative lane.**
   - Fix-forward is for deterministic, mechanical, behavior-preserving corrections (format/import/docs).
   - Data integrity reconciliation is not mechanical; it requires judgment, test execution, and artifact resealing.

3. **Proper lane routing maintains control plane separation.**
   - Build owns test execution and receipt sealing (Flow 3).
   - Gate owns verification and routing decision (Flow 4).
   - Fix-forward (Flow 4 subset) is only for blocking mechanical drift that Build already scoped correctly.

---

## Next Steps (for merge-decider)

1. **Do NOT merge.** BOUNCE back to Flow 3 Build.

2. **Route to build-cleanup agent** with task:
   - Rerun full test suite (unit + integration) with coverage instrumentation enabled
   - Update test_execution.md with authoritative counts and coverage metrics
   - Reseal build_receipt.json with data derived from refreshed artifacts
   - Verify lint/clippy status is VERIFIED before sealing

3. **Once Build completes reseal**, Flow 4 Gate can be rerun to:
   - Verify evidence consistency is resolved
   - Confirm all blockers cleared
   - Issue final merge verdict

---

_Gate Fixer Report: compliance-drift-proofing_
_Generated: 2025-12-19T03:30:00Z_
_Status: VERIFIED (analysis complete; findings mechanically derived from gate artifacts)_
