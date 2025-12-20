# Gate Cleanup Report

## Run: local-alignment-audit-aba1c6

## Completed: 2025-12-20T15:13:38Z

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
merge_verdict: MERGE
missing_required: []
missing_optional: []
blockers: []
concerns:
  - "RSK-001 (path traversal in secrets.rs) deferred to future security hardening run"
  - "Build receipt CANNOT_PROCEED is a permissions artifact, not content defect; artifacts verified via git"
```

## Artifact Verification

| Artifact | Status |
|----------|--------|
| merge_decision.md | ✓ Found |
| receipt_audit.md | ✓ Found |
| contract_compliance.md | ✓ Found |
| security_scan.md | ✓ Found |
| coverage_audit.md | ✓ Found |
| policy_analysis.md | ✓ Found |
| risk_assessment.md | ✓ Found |
| gate_fix_summary.md | ✓ Found |
| traceability_audit.md | ✓ Found |

## Extracted Gate Statuses (Machine Summary)

| Check | Status | Source |
|------|--------|--------|
| merge_decider | VERIFIED | merge_decision.md (verdict: MERGE) |
| receipt_audit | VERIFIED | receipt_audit.md |
| contract_compliance | VERIFIED | contract_compliance.md |
| security_scan | VERIFIED | security_scan.md |
| coverage_audit | VERIFIED | coverage_audit.md |

## Counts Derived (Stable Markers)

| Metric | Value | Source |
|--------|-------|--------|
| receipt_checks_total | 11 | receipt_audit.md (Machine Summary: checks_total) |
| receipt_checks_passed | 11 | receipt_audit.md (Machine Summary: checks_passed) |
| contract_violations | 0 | contract_compliance.md (Machine Summary: violations_total) |
| security_findings | 0 | security_scan.md (Machine Summary: findings_total) |
| policy_violations | 0 | policy_analysis.md (Machine Summary: compliance_summary.non_compliant) |
| coverage_line_percent | null | coverage_audit.md (not applicable for documentation-only run) |
| coverage_branch_percent | null | coverage_audit.md (not applicable for documentation-only run) |
| ac_total | 35 | receipt_audit.md (passthrough from build_receipt.json) |
| ac_completed | 35 | receipt_audit.md (passthrough from build_receipt.json) |

## Merge Verdict Analysis

- Verdict extracted: MERGE
- Status: VERIFIED
- Decision source: merge_decision.md Machine Summary
- Evidence chain:
  - Receipt audit: PASS (11/11 checks, AC loop 35/35 complete)
  - Contract compliance: PASS (0 violations, 7 flow commands verified)
  - Security scan: PASS (0 findings, 0 secrets, 13 files scanned)
  - Coverage audit: PASS (thresholds null/N/A for docs-only, 32 scenarios verified)
  - Policy analysis: PASS (12 policies checked, 10 compliant, 0 non-compliant)
  - Traceability audit: PASS (7 REQs covered, 32 BDD scenarios, 35 ACs mapped)

## Index Updated

- Fields changed: status, last_flow, updated_at
- status: VERIFIED
- last_flow: gate
- updated_at: 2025-12-20T15:13:38Z

## Gate Receipt Created

- File: .runs/local-alignment-audit-aba1c6/gate/gate_receipt.json
- Schema version: gate_receipt_v1
- Overall status: VERIFIED
- All required artifacts present and parseable
- All quality gates return VERIFIED
- Merge verdict: MERGE
- All required counts derived mechanically (no null required metrics)

## Routing Decision

- Merge verdict is MERGE
- All required gate statuses are VERIFIED
- Required counts are non-null (ac_total: 35, ac_completed: 35, checks: 11/11, contract violations: 0, security findings: 0)
- Recommended action: PROCEED to Flow 6 (Deploy)

## Summary

Gate cleanup complete for run `local-alignment-audit-aba1c6`. All required gate artifacts verified, counts mechanically derived, and receipt sealed. This run is ready for promotion to mainline in Flow 6 (Deploy).

Non-blocking concerns noted (RSK-001 deferred; build receipt I/O artifact) are suitable for separate tracking, not merge blockers.
