# Coverage Audit for compliance-drift-proofing

## Machine Summary

```yaml
status: UNVERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: test-author
blockers:
  - Evidence inconsistency: build_receipt.json claims 89.29% coverage; test_execution.md reports 75.12% coverage (14.17% discrepancy)
  - test_execution.md status is UNVERIFIED due to coverage shortfall; cleanup_report.md claims 89.29% but references outdated artifact
  - Cannot determine actual coverage from available evidence due to conflicting reports
missing_required:
  - Updated test_execution.md reflecting the resealed test results (+349 tests claimed in cleanup_report)
  - Coverage report output (HTML or JSON) that reconciles the discrepancy
  - Timestamp/audit trail showing when reseal occurred and what artifacts were refreshed
concerns:
  - Reseal operation in commit bacacfe updated build_receipt.json but did not update test_execution.md
  - Cleanup operation claims 420 tests passed (379 unit + 41 integration) but test_execution.md documents only 294 tests
  - P0 module (drift.rs) shows different coverages: 80.2% (test_execution.md) vs 87.1% (cleanup_report.md)
  - Module-level coverage in receipt differs substantially from test_execution.md (e.g., flow.rs 82.5% vs 70.0%)
coverage_line_percent: null
coverage_branch_percent: null
thresholds_defined: yes
severity_summary:
  critical: 2
  major: 3
  minor: 2
```

## Sources Consulted

- `.runs/compliance-drift-proofing/plan/test_plan.md` (coverage thresholds and test scope)
- `.runs/compliance-drift-proofing/build/test_execution.md` (test run output from initial build)
- `.runs/compliance-drift-proofing/build/build_receipt.json` (build receipt with claimed coverage metrics)
- `.runs/compliance-drift-proofing/build/cleanup_report.md` (cleanup/reseal report with reconciled metrics)
- `.runs/compliance-drift-proofing/build/test_changes_summary.md` (build-phase test summary)
- `.runs/compliance-drift-proofing/build/impl_changes_summary.md` (implementation changes inventory)
- Git commit history (bacacfe, 5a815d3) (evidence of reseal operation)

## Thresholds (from Plan)

```yaml
thresholds_status: PRESENT
line_required: 80
branch_required: 70
critical_path_defined: yes
critical_path_pointer: "test_plan.md lines 44-49; modules: drift.rs (P0 target 90%), cli.rs"
```

**Extracted from stable markers in test_plan.md (lines 40-49):**

- `COVERAGE_LINE_REQUIRED: 80` ✅ Marker found
- `COVERAGE_BRANCH_REQUIRED: 70` ✅ Marker found
- `COVERAGE_CRITICAL_PATH: tools/demoswarm-pack-check/src/checks/drift.rs, tools/demoswarm-pack-check/src/cli.rs` ✅ Marker found
- P0 critical-path coverage target: 90% for drift.rs checks 50, 51 (mentioned in test_plan.md line 49)

## Coverage Evidence Found

### Primary (Test Execution)

- **test_execution.md** — Canonical test-runner output; reports 75.12% line coverage (1386/1845 lines covered), 294 tests (253 unit + 41 integration), status UNVERIFIED

### Secondary (Build Receipt)

- **build_receipt.json** — Claims 89.29% line coverage with module-level breakdown; status VERIFIED; same completion timestamp as test_execution.md (2025-12-19T08:10:00Z)

### Tertiary (Reseal/Cleanup)

- **cleanup_report.md** — Generated during build cleanup (commit bacacfe); claims 89.29% line coverage; documents +349 new tests (+420 total vs initial); status VERIFIED
- **test_changes_summary.md** — Integration test summary; documents 38 tests added to control_plane.rs; references tarpaulin coverage but does not include numeric value

### Discrepancy Timeline (from git)

- **Commit 5a815d3** (2025-12-19 07:50:00Z): test_execution.md reports 75.12% line coverage; build_receipt.json status UNVERIFIED
- **Commit bacacfe** (2025-12-19 08:10:00Z): "reseal with 89.29% coverage"; build_receipt.json updated to status VERIFIED; test_execution.md appears unchanged in content

## Results (mechanical)

```yaml
line_actual: null
branch_actual: null
evidence_consistency: inconsistent
```

| Metric | Required | Actual (test_execution.md) | Actual (build_receipt.json) | Actual (cleanup_report.md) | Status   | Evidence                                                      |
| ------ | -------: | -------------------------: | --------------------------: | -------------------------: | -------- | ------------------------------------------------------------- |
| Line   |       80 |                      75.12 |                       89.29 |                      89.29 | CONFLICT | test_execution.md vs build_receipt.json discrepancy of 14.17% |
| Branch |       70 |                       null |                        null |                       null | UNKNOWN  | No branch metric available in any artifact                    |

### Module-Level Coverage (P0 Path: drift.rs)

| Module   | test_execution.md |      build_receipt.json | cleanup_report.md |                           Target | Status                   |
| -------- | ----------------: | ----------------------: | ----------------: | -------------------------------: | ------------------------ |
| drift.rs |   80.2% (357/445) |                   87.1% |             87.1% | 80% (P0 min); 90% (P0 preferred) | CONFLICT: 80.2% vs 87.1% |
| cli.rs   |      Not reported | Not reported separately |      Not reported |            Part of critical path | UNKNOWN                  |

## Critical Path Coverage

### Verification Status: **UNVERIFIABLE** (evidence conflict)

**Plan declares (test_plan.md lines 44, 49):**

- Critical-path modules: drift.rs (checks 50, 51 implementation)
- P0 target: 90% for drift.rs
- Secondary target: 80% line coverage overall

**Available evidence conflict:**

- test_execution.md: drift.rs at 80.2% (meets P0 minimum; below preferred 90%)
- cleanup_report.md: drift.rs at 87.1% (meets primary threshold; below preferred 90%)
- **Issue**: Two different coverage values for the same module with no merged test output or reconciliation

**What would make this verifiable:**

1. Updated test_execution.md showing full resealed test run output (to explain 420 vs 294 test count delta)
2. Coverage HTML/JSON report showing per-module line/branch numbers (to match against receipt claim)
3. Git log or audit comment explaining which artifacts were refreshed during reseal and why test_execution.md was not updated

## Findings

### CRITICAL

- **[CRITICAL] COV-CRIT-001: Evidence inconsistency — build receipt and test execution report conflicting coverage percentages**
  - build_receipt.json claims 89.29% line coverage (9 lines above 80% threshold)
  - test_execution.md reports 75.12% line coverage (5 lines below 80% threshold)
  - Same artifacts exist with same timestamp but different coverage values
  - Evidence: build_receipt.json line 25, test_execution.md line 25 (canonical summary section)
  - Impact: Cannot determine whether thresholds are met or missed; merge decision cannot be made on coverage evidence alone

- **[CRITICAL] COV-CRIT-002: Test count mismatch between build phases**
  - test_execution.md documents 294 tests (253 unit + 41 integration)
  - cleanup_report.md claims 420 tests (379 unit + 41 integration); +126 unit tests added during reseal
  - test_execution.md was not updated to reflect the reseal
  - Evidence: test_execution.md line 17, cleanup_report.md line 35
  - Impact: Cannot verify that coverage measurements correspond to the same test suite; reseal may have produced new coverage but artifact is stale

### MAJOR

- **[MAJOR] COV-MAJ-001: Module-level coverage discrepancy for P0 critical path (drift.rs)**
  - test_execution.md reports drift.rs at 80.2% (357/445 lines)
  - cleanup_report.md reports drift.rs at 87.1% (no line count provided)
  - P0 target is 90%; both values fall short, but the reconciliation method is unclear
  - Evidence: test_execution.md line 45, cleanup_report.md line 19
  - Impact: Cannot confirm whether reseal produced sufficient coverage improvements in critical module

- **[MAJOR] COV-MAJ-002: Branch coverage metrics absent from all evidence**
  - test_plan.md declares `COVERAGE_BRANCH_REQUIRED: 70`
  - No branch coverage percentages found in test_execution.md, build_receipt.json, or cleanup_report.md
  - Evidence: test_plan.md line 42; test_execution.md has no branch metrics; build_receipt.json line 27 shows null
  - Impact: Cannot verify branch coverage threshold compliance; threshold is gating but unverifiable

- **[MAJOR] COV-MAJ-003: Reseal operation did not update primary test execution artifact**
  - Commit bacacfe (reseal) modified build_receipt.json and cleanup_report.md
  - test_execution.md was NOT updated during reseal
  - test_changes_summary.md documents 38 tests added but does not include coverage percentages
  - Evidence: git show bacacfe files changed; test_execution.md content unchanged from 5a815d3
  - Impact: Cannot audit the chain of evidence from test runs to sealed receipt; test runner output is stale

### MINOR

- **[MINOR] COV-MIN-001: cli.rs critical-path coverage not separately reported**
  - test_plan.md declares critical path includes cli.rs (line 44)
  - No separate coverage metric for cli.rs in any artifact
  - Evidence: test_plan.md line 44
  - Impact: cli.rs coverage cannot be individually verified; assumed included in overall 89.29% but not proven

- **[MINOR] COV-MIN-002: No evidence of coverage report files (HTML, JSON, XML)**
  - Test plan mentions measurement via "cargo-llvm-cov or similar"; cargo tarpaulin used in actual run (per test_execution.md line 11)
  - No HTML coverage report (e.g., coverage/index.html) referenced in any artifact
  - Evidence: test_plan.md line 47 (mentions measurement method); test_execution.md line 11 (actual command); no report files listed
  - Impact: Cannot inspect raw coverage metrics or per-file breakdown; acceptance of percentages relies entirely on receipt claims

## Notes for Merge-Decider

Coverage evidence is **UNVERIFIED due to a critical inconsistency between the primary test execution output and the sealed build receipt.** The test_execution.md artifact (from the initial test run) reports 75.12% line coverage, which is 4.88% below the 80% threshold. However, the build_receipt.json (sealed during cleanup after a reseal operation) claims 89.29% coverage.

**Key issue:** The reseal operation (commit bacacfe) updated the receipt to 89.29% and added 126 unit tests, but did not update test_execution.md. Without an updated test execution report showing the full test run after the reseal (420 tests total, as claimed in cleanup_report.md), we cannot verify that the claimed 89.29% coverage corresponds to the actual test results.

Additionally:

- Branch coverage (70% threshold in plan) has no evidence at all—all artifacts show null or missing values.
- P0 critical-path module (drift.rs) shows conflicting percentages: 80.2% (test_execution.md) vs 87.1% (cleanup_report.md), and both fall below the 90% P0 target.

**Recommendation:** Return to Flow 3 (Build) with test-author to:

1. Re-run the test suite after the reseal (or provide the output from the 420-test run that was claimed)
2. Update test_execution.md with the resealed results
3. Capture branch coverage metrics via the same coverage tooling
4. Provide a reconciliation between the initial test_execution.md (75.12%, 294 tests) and the cleanup_report.md claim (89.29%, 420 tests)

Once test_execution.md is updated and evidence is consistent, coverage-enforcer can re-audit and produce a VERIFIED or UNVERIFIED verdict with full traceability.

## Inventory (machine countable)

- COV_CRITICAL: COV-CRIT-001, COV-CRIT-002
- COV_MAJOR: COV-MAJ-001, COV-MAJ-002, COV-MAJ-003
- COV_MINOR: COV-MIN-001, COV-MIN-002
- COV_METRIC: line required=80 actual=null status=CONFLICT
- COV_METRIC: branch required=70 actual=null status=UNKNOWN
- COV_THRESHOLD_STATUS: PRESENT
