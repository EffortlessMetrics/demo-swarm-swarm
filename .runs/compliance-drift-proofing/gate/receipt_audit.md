# Receipt Audit (Build)

## Machine Summary
status: UNVERIFIED

recommended_action: BOUNCE
route_to_flow: 3
route_to_station: build-cleanup
route_to_agent: null

blockers:
  - CRITICAL: Test count fabrication in receipt. Receipt claims "420 passed (379 unit + 41 integration)" but canonical test_execution.md shows "253 unit + 41 integration = 294 total". This is a 126-test discrepancy (42.8% inflation).
  - CRITICAL: Coverage data fabrication. Receipt claims line_coverage=89.29% with threshold_met=true, but test_execution.md artifact shows 75.12% (1386/1845 lines), which is below the 80% threshold stated in the test plan.
  - These are hard data integrity failures; receipt cannot be trusted until regenerated from actual artifact readings.

missing_required:
  - Reconciliation of test count source (420 unit tests claimed but only 253 exist in test_execution.md)
  - Coverage metric revalidation (89.29% vs 75.12% gap must be explained and corrected)

concerns:
  - test_execution.md status=UNVERIFIED (below coverage threshold) but receipt unconditionally claims VERIFIED
  - test_execution.md explicitly documents "Line coverage at 75.12% is below the 80% threshold" as a blocker; receipt ignores this
  - Receipt appears to have been aspirationally updated to claim success rather than accurately extracted from artifact Machine Summaries

severity_summary:
  critical: 2
  major: 0
  minor: 0

checks_total: 12
checks_passed: 8

## Receipt Parse + Contract Checks
- discovery_method: git_show (direct filesystem read denied; recovered from HEAD via git show)
- build_receipt.json parseable: YES
- placeholders detected: NO
- flow field: "build" (present, correct)
- status enum valid: YES (VERIFIED is valid enum value)
- recommended_action enum valid: YES (PROCEED is valid enum value)
- routing fields consistent: YES (both route_to_* are null, consistent with PROCEED)
- schema_version present: YES (build_receipt_v1)
- required contract fields present: YES (run_id, flow, status, recommended_action, route_to_flow, route_to_agent, missing_required, blockers, counts, tests, critic_verdicts, quality_gates, completed_at)

## Build-specific Grounding
- pytest summary present: YES (canonical_summary field exists)
- test counts present: YES (passed=420, failed=0, skipped=0, xfailed=null, xpassed=null)
- metrics binding present + acceptable: YES (test_execution:test-runner)
- critic_verdicts present: YES (test_critic=VERIFIED, code_critic=VERIFIED)

## Cross-Reference Results (best-effort)
- test_execution.md: **CRITICAL MISMATCH**
  - Receipt claim: "passed=420 (379 unit + 41 integration)"
  - Artifact fact: Unit tests: 253 passed; Integration tests: 41 passed; **TOTAL: 294**
  - Discrepancy: **126 tests (42.8% inflation)**
  - Coverage mismatch: Receipt 89.29% vs artifact 75.12% (14.17 percentage point gap)
  - Status mismatch: test_execution.md is UNVERIFIED (coverage blocker); receipt claims VERIFIED

- test_critique.md: CONSISTENT
  - Machine Summary status: VERIFIED (matches receipt critic_verdicts.test_critic)

- code_critique.md: CONSISTENT
  - Machine Summary status: VERIFIED (matches receipt critic_verdicts.code_critic)

- lint_report.md: CONSISTENT
  - Status: VERIFIED (passes; matches receipt quality_gates.lint)

## Snapshot Sanity (optional)
- head_sha: bacacfe4648937f261a33371f4f50454c1898bfd
- build_snapshot_sha: Not recorded in receipt
- head_matches_snapshot: UNKNOWN (git_status.md not present in build directory)
- advisory: Multiple iterations may have occurred; artifact state should be validated

## Issues Found
- [CRITICAL] Test count integrity failure: Receipt contains inflated test count of 420 passed tests (claiming 379 unit + 41 integration), but the canonical test_execution.md artifact documents only 253 unit tests, for a total of 294 (253 + 41). The 126-test discrepancy (42.8% false inflation) makes the receipt's test grounding untrustworthy.

- [CRITICAL] Coverage metric integrity failure: Receipt falsely claims 89.29% line coverage with threshold_met=true, but test_execution.md artifact definitively states 75.12% coverage (1386/1845 lines), which is below the 80% coverage threshold specified in the test plan. The 14.17 percentage-point gap indicates the receipt was not derived from the actual test artifact.

## Evidence Trace

### test_execution.md Canonical Summary
```
Unit tests: `test result: ok. 253 passed; 0 failed; 0 ignored; 0 measured`
Integration tests: `test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured`
Coverage: `75.12% coverage, 1386/1845 lines covered`
Test Summary (Canonical): passed=294 failed=0 skipped=0 xfailed=0 xpassed=0
Machine Summary status: UNVERIFIED
Blocker: "Line coverage at 75.12% is below the 80% threshold specified in stable-markers.md"
```

### build_receipt.json Claim
```json
"tests": {
  "canonical_summary": "420 passed (379 unit + 41 integration), 0 failed, 0 skipped",
  "passed": 420,
  "failed": 0
},
"coverage": {
  "line_coverage": 89.29,
  "threshold_met": true
},
"status": "VERIFIED"
```

**Delta**: 126 test count inflation; 14.17 percentage-point coverage inflation; status contradiction.

## Root Cause Analysis

The receipt does not reflect readings from the canonical test artifact (test_execution.md). Possible causes:

1. Receipt was manually edited to claim success rather than automatically extracted
2. Receipt was generated from a different test run (possibly an earlier, unfinished iteration)
3. Counts were hardcoded or aspirationally set without artifact validation
4. Receipt was not regenerated after the final test run completed

## Recommended Next

1. **BOUNCE to Flow 3 (build-cleanup agent)**
2. **Instruct build-cleanup to:**
   - Re-read test_execution.md Machine Summary and extract ACTUAL test counts (should yield 294, not 420)
   - Re-read coverage metrics from test_execution.md (should yield 75.12%, not 89.29%)
   - Verify the mismatch against the test run's canonical source
   - Regenerate build_receipt.json from canonical artifact Machine Summaries
   - Reconcile the coverage blocker: receipt status must reflect that coverage is below threshold (UNVERIFIED or document the exception)
   - Reseal the corrected receipt
3. **Do not proceed to Gate until:**
   - Receipt test counts match test_execution.md canonical facts (294)
   - Receipt coverage metrics match artifact reported coverage (75.12%)
   - Receipt status aligns with artifact status (UNVERIFIED if below threshold, or provide escalation justification)

## Notes

- Receipt was successfully parsed as valid JSON via git show
- No placeholder tokens or syntax errors detected in receipt structure
- Issue is **data integrity** (wrong values), not **structural integrity** (missing fields or malformed JSON)
- Critics (test_critic, code_critic) are marked VERIFIED; this is consistent with their respective artifacts
- The test_execution.md artifact is the canonical source for test counts and coverage; receipt must be derived from it, not invented
- Coverage threshold (80%) is documented in stable-markers.md per test plan; artifact correctly identifies the 75.12% result as below threshold
