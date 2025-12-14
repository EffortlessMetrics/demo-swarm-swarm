# Gate Cleanup Report

## Run: align-doc-ownership
## Completed: 2025-12-13T11:40:10Z

## Machine Summary
status: UNVERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: fixer
merge_verdict: BOUNCE
missing_required: []
missing_optional: []
blockers:
  - MECH-001: Rust formatting violations (9 files need cargo fmt)
concerns:
  - receipt_audit status UNVERIFIED (self_reviewer) - non-blocking for documentation run
  - mutation_score null - explicitly N/A per test plan
  - build_receipt.json inaccessible - cannot verify binding; gate artifacts are VERIFIED
  - REQ readiness UNKNOWN due to missing priority markers and inaccessible receipt

## Artifact Verification
| Artifact | Status |
|----------|--------|
| merge_decision.md | Found |
| receipt_audit.md | Found |
| contract_compliance.md | Found |
| security_scan.md | Found |
| coverage_audit.md | Found |
| policy_analysis.md | Found |
| risk_assessment.md | Found |
| gate_fix_summary.md | Found |

## Extracted Gate Statuses (Machine Summary)
| Check | Status | Source |
|------|--------|--------|
| merge_decider | VERIFIED | merge_decision.md |
| receipt_audit | UNVERIFIED | receipt_audit.md |
| contract_compliance | VERIFIED | contract_compliance.md |
| security_scan | VERIFIED | security_scan.md |
| coverage_audit | VERIFIED | coverage_audit.md |
| policy_analyst | VERIFIED | policy_analysis.md |
| risk_analyst | VERIFIED | risk_assessment.md |
| gate_fixer | VERIFIED | gate_fix_summary.md |

## Counts Derived (Stable Markers)
| Metric | Value | Source |
|--------|-------|--------|
| receipt_checks_total | null | receipt_audit.md (not explicitly stated in Machine Summary) |
| receipt_checks_passed | null | receipt_audit.md (not explicitly stated in Machine Summary) |
| contract_violations | 0 | contract_compliance.md (violations_total) |
| security_findings | 0 | security_scan.md (severity_summary: critical=0, major=0, minor=0) |
| policy_violations | 0 | policy_analysis.md (non_compliant: 0) |
| coverage_line_percent | null | coverage_audit.md (coverage_line_percent) |
| coverage_branch_percent | null | coverage_audit.md (coverage_branch_percent) |

## Index Updated
- Fields changed: status, last_flow, updated_at
- status: UNVERIFIED
- last_flow: gate
- updated_at: 2025-12-13T11:40:10Z

## Merge Verdict Analysis

The merge_decision.md returned verdict **BOUNCE** based on:

1. **MECH-001: Rust formatting violations** - `cargo fmt --check` detected diffs in 9 Rust source files in `tools/demoswarm-pack-check/src/`. This is a mechanical fix that can be resolved by running `cargo fmt`.

2. **Substantive checks passed:**
   - contract_compliance: VERIFIED (all validation rules OK, 0 violations)
   - security_scan: VERIFIED (0 critical/major/minor findings)
   - coverage_audit: VERIFIED (N/A per test plan for documentation run)
   - policy_analysis: VERIFIED (9 compliant, 0 non-compliant)
   - risk_assessment: VERIFIED (RSK-008 flagged formatting issue)

3. **Non-blocking concerns:**
   - receipt_audit UNVERIFIED due to self_reviewer status (acceptable for doc run)
   - mutation_score null (explicitly N/A per test plan)

## Routing

Per merge_decision.md and gate_fix_summary.md:
- **Target:** Flow 3 (Build)
- **Agent:** fixer
- **Action:** Run `cd tools/demoswarm-pack-check && cargo fmt` to fix formatting in 9 files

## Files Needing Formatting
1. tools/demoswarm-pack-check/src/checks/contracts.rs
2. tools/demoswarm-pack-check/src/checks/control_plane.rs
3. tools/demoswarm-pack-check/src/checks/drift.rs
4. tools/demoswarm-pack-check/src/checks/flow.rs
5. tools/demoswarm-pack-check/src/checks/mod.rs
6. tools/demoswarm-pack-check/src/contracts.rs
7. tools/demoswarm-pack-check/src/ctx.rs
8. tools/demoswarm-pack-check/src/reporter.rs
9. tools/demoswarm-pack-check/src/util.rs

## Expected Outcome After Fix

Once `cargo fmt` is applied:
- Re-run Gate (Flow 4)
- Expected verdict: MERGE (all substantive checks already VERIFIED)
- The BOUNCE is due solely to mechanical formatting; no judgment required
