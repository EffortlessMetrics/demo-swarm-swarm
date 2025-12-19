# Policy Analysis

## Machine Summary
status: UNVERIFIED

recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: build-cleanup

blockers:
  - "POL-004 CRITICAL: Receipt integrity failure - build_receipt.json claims status=VERIFIED, 420 tests, 89.29% coverage; test_execution.md canonical artifact shows status=UNVERIFIED, 294 tests, 75.12% coverage (126-test inflation, 14.17% coverage gap)"
  - "POL-003 NON-COMPLIANT: Receipt test counts (420) are not mechanical - canonical artifact shows 294 tests (253 unit + 41 integration); counts appear fabricated or stale"
  - "POL-015 NON-COMPLIANT: Receipt quality_gates.coverage claims VERIFIED but underlying test_execution.md has blocker (coverage below 80% threshold)"

missing_required:
  - "Reconciled test_execution.md with authoritative test counts matching receipt claims (or corrected receipt)"
  - "Coverage evidence reconciliation (89.29% vs 75.12%)"
  - "Branch coverage metrics (required 70% threshold in test_plan.md but null in all artifacts)"

concerns:
  - "Dependency audit could not run (cargo-audit CVSS 4.0 incompatibility); manual review substituted"
  - "Index.json metadata lags gate receipt (status=VERIFIED in index vs UNVERIFIED artifacts)"
  - "Iterations count drift (run_meta=7, index=6)"
  - "Prior gate iteration artifacts contain stale contract mismatch finding (corrected in iteration 6)"

compliance_summary:
  policies_found: 1
  policies_checked: 1
  compliant: 11
  non_compliant: 3
  not_applicable: 1
  unknown: 1
  waivers_needed: 0

## Context
- flow: gate
- run_id: compliance-drift-proofing
- policy_roots_searched:
  - policies/ (not found)
  - docs/policies/ (not found)
  - .policies/ (not found)
  - CLAUDE.md (pack-level policy, authoritative)
- inputs_used:
  - .runs/compliance-drift-proofing/run_meta.json
  - .runs/index.json
  - .runs/compliance-drift-proofing/gate/receipt_audit.md
  - .runs/compliance-drift-proofing/gate/security_scan.md
  - .runs/compliance-drift-proofing/gate/contract_compliance.md
  - .runs/compliance-drift-proofing/gate/coverage_audit.md
  - .runs/compliance-drift-proofing/gate/gate_fix_summary.md
  - .runs/compliance-drift-proofing/gate/traceability_audit.md
  - .runs/compliance-drift-proofing/gate/merge_decision.md
  - .runs/compliance-drift-proofing/gate/gate_receipt.json
  - .runs/compliance-drift-proofing/build/build_receipt.json (via git show)
  - .runs/compliance-drift-proofing/build/test_execution.md (via git show)
  - CLAUDE.md

## Policies Reviewed
- CLAUDE.md (pack-level policy) -- version: current HEAD (bacacfe)

## Compliance Register

Use stable `POL-NNN` markers for mechanical counting.

| ID | Policy | Section | Requirement | Status | Severity | Evidence |
|----|--------|---------|-------------|--------|----------|----------|
| POL-001 | CLAUDE.md | Machine Summary Contract | Status enum is VERIFIED/UNVERIFIED/CANNOT_PROCEED only | COMPLIANT | HIGH | All artifacts use valid status enum values |
| POL-002 | CLAUDE.md | Machine Summary Contract | recommended_action enum is PROCEED/RERUN/BOUNCE/FIX_ENV only | COMPLIANT | HIGH | gate_receipt.json:L6-7 uses valid enum |
| POL-003 | CLAUDE.md | Receipts | counts are mechanical (grep/wc/parse), never estimated | NON-COMPLIANT | CRITICAL | build_receipt.json claims 420 tests; test_execution.md L25 shows 294 |
| POL-004 | CLAUDE.md | Receipts | quality_gates sourced from agent Machine Summaries | NON-COMPLIANT | CRITICAL | build_receipt.json status=VERIFIED but test_execution.md status=UNVERIFIED |
| POL-005 | CLAUDE.md | CLI Tooling Surface | Agents invoke tools via shims only | COMPLIANT | MEDIUM | security_scan.md confirms shim usage |
| POL-006 | CLAUDE.md | Non-Negotiables #5 | run_id folders never rename | NOT_APPLICABLE | LOW | Gate flow does not modify run identity |
| POL-007 | CLAUDE.md | Canonical Status + Verdict Domains | Do not conflate status domains across contexts | COMPLIANT | HIGH | Artifacts use correct domain vocabulary |
| POL-008 | CLAUDE.md | Secrets Sanitizer | Secrets gate required for publish | COMPLIANT | CRITICAL | security_scan.md:L34 findings_total=0 |
| POL-009 | CLAUDE.md | Non-Negotiables #4 | Two gates required for GitHub operations | COMPLIANT | HIGH | traceability_audit.md:L93-96 confirms both gates satisfied |
| POL-010 | CLAUDE.md | Non-Negotiables #1 | All paths repo-root-relative | COMPLIANT | HIGH | All artifact paths use consistent repo-root format |
| POL-011 | CLAUDE.md | Non-Negotiables #3 | Control plane vs audit plane separation | COMPLIANT | HIGH | contract_compliance.md:L118-127 confirms alignment |
| POL-012 | CLAUDE.md | Receipts | Reporters summarize from receipts, not raw artifacts | COMPLIANT | MEDIUM | gate_receipt.json:L46-57 lists key_artifacts |
| POL-013 | CLAUDE.md | Machine Summary Contract | observations field optional but stable | COMPLIANT | LOW | gate_fix_summary.md:L193-196 observations section present |
| POL-014 | CLAUDE.md | CLI Tooling Surface | Dependency audit for vulnerable components | UNKNOWN | MEDIUM | security_scan.md:L120-133 cargo-audit not run |
| POL-015 | CLAUDE.md | Receipts | quality_gates reflect actual artifact state | NON-COMPLIANT | CRITICAL | build_receipt quality_gates.coverage=VERIFIED contradicts test_execution.md blocker |
| POL-016 | CLAUDE.md | Machine Summary Contract | blockers must explain what prevents VERIFIED | COMPLIANT | HIGH | All UNVERIFIED artifacts include blockers array |

## Compliance Details

### POL-001: Status enum frozen
- Policy: CLAUDE.md, Section "Machine Summary Contract"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - gate_receipt.json:L6 status: "UNVERIFIED" (valid enum)
  - All Machine Summary blocks use VERIFIED/UNVERIFIED/CANNOT_PROCEED
- Notes: Status values consistently within pack-standard enum

### POL-002: recommended_action enum frozen
- Policy: CLAUDE.md, Section "Machine Summary Contract"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - gate_receipt.json:L7 recommended_action: "BOUNCE" (valid enum)
  - All routing actions within pack-standard enum
- Notes: Action enum compliance verified

### POL-003: Mechanical counts only
- Policy: CLAUDE.md, Section "Receipts", Line 218: "counts are mechanical (grep/wc/parse), never estimated"
- Status: NON-COMPLIANT
- Severity: CRITICAL
- Evidence:
  - build_receipt.json claims: `"passed": 420` and `"canonical_summary": "420 passed (379 unit + 41 integration)"`
  - test_execution.md canonical artifact states: `"Test Summary (Canonical): passed=294 failed=0"` and `"Unit tests: 253 passed... Integration tests: 41 passed"`
  - Discrepancy: 126 tests (42.8% inflation)
  - receipt_audit.md:L50-54 documents this as CRITICAL MISMATCH
- Notes: Receipt counts are not mechanical - they do not match the canonical test artifact. The receipt appears to have been updated aspirationally or from a stale/different source. This violates the core receipt guarantee.

### POL-004: Quality gates sourced from Machine Summaries
- Policy: CLAUDE.md, Section "Receipts", Line 219: "quality_gates are sourced from agent Machine Summaries (no recomputation)"
- Status: NON-COMPLIANT
- Severity: CRITICAL
- Evidence:
  - build_receipt.json:L4 `"status": "VERIFIED"`
  - build_receipt.json:L46 `"quality_gates": { "lint": "VERIFIED", "coverage": "VERIFIED" }`
  - test_execution.md Machine Summary:L2 `status: UNVERIFIED` with blocker: "Line coverage at 75.12% is below the 80% threshold"
  - receipt_audit.md:L73 documents: "Receipt claims 89.29% coverage but test_execution.md shows 75.12%"
  - coverage_audit.md:L116-128 documents CRITICAL evidence inconsistency
- Notes: The build_receipt.json quality_gates were not sourced from the test_execution.md Machine Summary. The artifact has status=UNVERIFIED with an explicit coverage blocker, but the receipt claims VERIFIED. This is a policy violation.

### POL-005: Invoke tools via shims
- Policy: CLAUDE.md, Section "CLI Tooling Surface", Line 477-478
- Status: COMPLIANT
- Severity: MEDIUM
- Evidence:
  - security_scan.md confirms shim invocation patterns
  - Plan artifacts reference bash shim usage correctly
- Notes: Shim invocation consistently followed

### POL-006: run_id folders never rename
- Policy: CLAUDE.md, Section "Non-Negotiables #5", Line 81-82
- Status: NOT_APPLICABLE
- Severity: LOW
- Evidence:
  - run_id "compliance-drift-proofing" unchanged throughout all flows
  - Gate flow does not modify .runs/ folder structure
- Notes: Not applicable to Gate flow operations; identity stable

### POL-007: Status domain separation
- Policy: CLAUDE.md, Section "Canonical Status + Verdict Domains", Lines 314-336
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - coverage_audit.md:L4 uses UNVERIFIED correctly
  - security_scan.md:L4 uses VERIFIED correctly
  - gate_receipt.json uses flow-appropriate status domain
  - merge_decision.md uses BOUNCE verdict (not UNVERIFIED)
- Notes: Status domains not conflated; each artifact uses appropriate vocabulary

### POL-008: Secrets gate required for publish
- Policy: CLAUDE.md, Section "Secrets Sanitizer (Publish Gate)", Lines 411-423
- Status: COMPLIANT
- Severity: CRITICAL
- Evidence:
  - security_scan.md:L34 `findings_total: 0`
  - security_scan.md:L47-64 documents secrets patterns checked (AWS keys, GitHub tokens, private keys, bearer tokens)
  - traceability_audit.md:L93 `safe_to_publish: true`
- Notes: Secrets scan completed successfully with no findings

### POL-009: Two gates for GitHub operations
- Policy: CLAUDE.md, Section "Non-Negotiables #4", Lines 76-79
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - traceability_audit.md:L93-96 confirms both gates:
    - `safe_to_publish: true`
    - `proceed_to_github_ops: true`
    - `publish_surface: PUSHED`
- Notes: Two-gate requirement satisfied

### POL-010: Repo-root-relative paths
- Policy: CLAUDE.md, Section "Non-Negotiables #1", Line 65-66
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - contract_compliance.md:L25-37 all source paths repo-root-relative
  - All artifact paths use format: `.runs/compliance-drift-proofing/...`, `tools/demoswarm-pack-check/...`
- Notes: Path conventions consistently followed

### POL-011: Control plane vs audit plane separation
- Policy: CLAUDE.md, Section "Non-Negotiables #3", Lines 72-74
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - contract_compliance.md:L63-81 documents check IDs (49, 50, 52, 53) alignment
  - contract_compliance.md:L101-127 "Notes for Merge-Decider" confirms implementation matches contract
  - Iteration 6 corrected prior misreading; contract and implementation are aligned
- Notes: Control plane (api_contracts.yaml) and implementation (drift.rs) are correctly separated and aligned

### POL-012: Reporters summarize from receipts
- Policy: CLAUDE.md, Section "Receipts", Line 220
- Status: COMPLIANT
- Severity: MEDIUM
- Evidence:
  - gate_receipt.json:L46-57 `key_artifacts` list present
  - Receipt structure follows pack schema
- Notes: Receipt-based summarization followed

### POL-013: observations field structure
- Policy: CLAUDE.md, Section "Machine Summary Contract", Line 240
- Status: COMPLIANT
- Severity: LOW
- Evidence:
  - gate_fix_summary.md:L193-196 observations section present with valid content
  - Machine Summary blocks include optional observations field when appropriate
- Notes: Observations field used appropriately

### POL-014: Dependency vulnerability audit
- Policy: CLAUDE.md, Section "CLI Tooling Surface" (implied via security posture)
- Status: UNKNOWN
- Severity: MEDIUM
- Evidence:
  - security_scan.md:L120-133 `status: not_run`, `reason: Advisory database CVSS 4.0 format not supported`
  - Manual dependency review performed; dependencies are mainstream crates
- Notes: Automated audit could not run; manual review substituted but is not equivalent to tooled verification

### POL-015: Quality gates reflect actual artifact state
- Policy: CLAUDE.md, Section "Receipts", Lines 218-220 (implied by "sourced from agent Machine Summaries")
- Status: NON-COMPLIANT
- Severity: CRITICAL
- Evidence:
  - build_receipt.json:L40-45 `"quality_gates": { "coverage": "VERIFIED", "lint": "VERIFIED" }`
  - test_execution.md Machine Summary: `status: UNVERIFIED` with blocker about coverage shortfall
  - coverage_audit.md:L116-128 documents CRITICAL discrepancy: receipt 89.29% vs artifact 75.12%
  - traceability_audit.md:L15 notes: "quality_gates aspirationally updated without underlying artifact Machine Summaries"
- Notes: The quality_gates in build_receipt.json do not reflect the actual state of the underlying artifacts. test_execution.md is UNVERIFIED with a coverage blocker, but the receipt claims coverage=VERIFIED.

### POL-016: blockers must explain UNVERIFIED
- Policy: CLAUDE.md, Section "Machine Summary Contract", Lines 252-253
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - All UNVERIFIED artifacts include blockers array explaining what prevents VERIFIED
  - test_execution.md blockers include coverage threshold
  - gate_receipt.json blockers explain receipt integrity failure
- Notes: Blocker documentation requirement satisfied across artifacts

## Violations Summary

| ID | Policy | Section | Severity | Remediation | Owner |
|----|--------|---------|----------|------------|-------|
| POL-003 | CLAUDE.md | Receipts | CRITICAL | Regenerate build_receipt.json with mechanical counts from test_execution.md (294, not 420) | build-cleanup |
| POL-004 | CLAUDE.md | Receipts | CRITICAL | Source quality_gates from actual artifact Machine Summaries; receipt status must reflect test_execution.md UNVERIFIED state | build-cleanup |
| POL-015 | CLAUDE.md | Receipts | CRITICAL | Reconcile coverage metrics: either run tests to achieve 89.29% and update test_execution.md, or correct receipt to reflect 75.12% actual | build-cleanup |

## Waivers Needed
- None

## Evidence Cross-Reference

### Test Count Evidence Chain

| Source | Claimed | Evidence Location |
|--------|---------|-------------------|
| build_receipt.json | 420 passed (379 unit + 41 integration) | build_receipt.json:L23-24 via git show |
| test_execution.md (canonical) | 294 passed (253 unit + 41 integration) | test_execution.md:L25, L22-23 via git show |
| receipt_audit.md | Identifies 126-test discrepancy | receipt_audit.md:L50-54 |

**Conclusion**: Receipt contains fabricated test counts. Canonical test artifact shows 294 tests, not 420.

### Coverage Evidence Chain

| Source | Claimed Line Coverage | Evidence Location |
|--------|----------------------|-------------------|
| build_receipt.json | 89.29% | build_receipt.json:L32 via git show |
| test_execution.md (canonical) | 75.12% (1386/1845 lines) | test_execution.md:L22 via git show |
| coverage_audit.md | Identifies 14.17% gap | coverage_audit.md:L81-84 |

**Conclusion**: Receipt contains inflated coverage metrics. Canonical artifact shows 75.12%, which is below the 80% threshold.

### Status Evidence Chain

| Source | Claimed Status | Evidence Location |
|--------|----------------|-------------------|
| build_receipt.json | VERIFIED | build_receipt.json:L4 via git show |
| test_execution.md (canonical) | UNVERIFIED with blocker | test_execution.md:L2-6 via git show |
| gate_receipt.json | UNVERIFIED | gate_receipt.json:L6 |

**Conclusion**: Receipt status does not match canonical artifact. Build receipt claims VERIFIED but the underlying test execution artifact is UNVERIFIED with coverage blocker.

## Root Cause Analysis

The receipt integrity failures (POL-003, POL-004, POL-015) share a common root cause:

1. **Reseal operation did not refresh canonical artifact**: Commit bacacfe "reseal with 89.29% coverage" updated build_receipt.json but did not update test_execution.md with new test results.

2. **Receipt claims not derived from artifact state**: The build_receipt.json appears to have been updated with aspirational values (420 tests, 89.29% coverage, VERIFIED status) rather than values extracted from the test_execution.md Machine Summary.

3. **Policy violation pattern**: This violates the core receipt guarantee that "quality_gates are sourced from agent Machine Summaries (no recomputation)". Receipts must reflect actual measured state, not hoped-for outcomes.

## Recommended Next

1. **BOUNCE to Flow 3 (Build)** with route_to_agent: build-cleanup
2. **Re-run full test suite** with coverage instrumentation to produce authoritative measurements
3. **Update test_execution.md** with actual test counts and coverage metrics from the test run
4. **Regenerate build_receipt.json** by reading Machine Summary values from the updated test_execution.md
5. **Return to Gate** for re-verification after Build reseal completes with consistent evidence

## Inventory (machine countable)

- POL-001: Status enum frozen (COMPLIANT)
- POL-002: recommended_action enum frozen (COMPLIANT)
- POL-003: Mechanical counts only (NON-COMPLIANT)
- POL-004: Quality gates from Machine Summaries (NON-COMPLIANT)
- POL-005: Invoke tools via shims (COMPLIANT)
- POL-006: run_id folders never rename (NOT_APPLICABLE)
- POL-007: Status domain separation (COMPLIANT)
- POL-008: Secrets gate required for publish (COMPLIANT)
- POL-009: Two gates for GitHub operations (COMPLIANT)
- POL-010: Repo-root-relative paths (COMPLIANT)
- POL-011: Control plane vs audit plane separation (COMPLIANT)
- POL-012: Reporters summarize from receipts (COMPLIANT)
- POL-013: observations field structure (COMPLIANT)
- POL-014: Dependency vulnerability audit (UNKNOWN)
- POL-015: Quality gates reflect actual artifact state (NON-COMPLIANT)
- POL-016: blockers must explain UNVERIFIED (COMPLIANT)
