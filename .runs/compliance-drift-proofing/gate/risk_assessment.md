# Risk Assessment

## Machine Summary
status: UNVERIFIED

recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: build-cleanup

blockers:
  - "RSK-014 (CRITICAL): Receipt fabrication - build_receipt.json claims 420 tests and 89.29% coverage; test_execution.md reports 294 tests and 75.12% coverage. This is a 42.8% test count inflation and 14.17 percentage-point coverage inflation."
  - "RSK-015 (CRITICAL): Coverage threshold not met - actual coverage (75.12%) is 4.88% below the 80% threshold specified in test_plan.md."

missing_required: []

concerns:
  - "RSK-016: Audit trail gap - reseal operation (commit bacacfe) updated build_receipt.json but left test_execution.md stale"
  - "RSK-017: Branch coverage not measured - no evidence of branch coverage metrics despite 70% threshold in test_plan.md"
  - "cargo-audit could not run due to CVSS 4.0 format incompatibility (manual review substituted)"
  - "Index metadata lags gate (iterations 7 vs 6, status VERIFIED vs UNVERIFIED)"

severity_summary:
  critical: 2
  high: 1
  medium: 5
  low: 3

## Context
- flow: gate
- run_id: compliance-drift-proofing
- iteration: 7
- inputs_used:
  - `.runs/compliance-drift-proofing/run_meta.json`
  - `.runs/compliance-drift-proofing/gate/receipt_audit.md`
  - `.runs/compliance-drift-proofing/gate/contract_compliance.md`
  - `.runs/compliance-drift-proofing/gate/security_scan.md`
  - `.runs/compliance-drift-proofing/gate/coverage_audit.md`
  - `.runs/compliance-drift-proofing/gate/traceability_audit.md`
  - `.runs/compliance-drift-proofing/gate/gate_receipt.json`
  - `.runs/compliance-drift-proofing/build/build_receipt.json` (via git show)
  - `.runs/compliance-drift-proofing/build/test_execution.md` (via git show)
- prior_risk_assessments_seen:
  - `.runs/compliance-drift-proofing/signal/risk_assessment.md`
  - `.runs/compliance-drift-proofing/gate/risk_assessment.md` (iteration 6)

## Risk Register

| ID | Category | Severity | Status | Summary | Owner |
|----|----------|----------|--------|---------|-------|
| RSK-001 | OPS | HIGH | MITIGATED | Prior #49 bounce - warning-first mode provides mitigation | pack-maintainers |
| RSK-002 | DATA | MEDIUM | MITIGATED | PLN/BLD prefix adopted as canonical | pack-maintainers |
| RSK-003 | OPS | MEDIUM | MITIGATED | 4 agents missing Skills sections; addressed via check 49 | agent-authors |
| RSK-004 | COMPLIANCE | MEDIUM | MITIGATED | Warning-first mode implemented via --strict flag | pack-maintainers |
| RSK-005 | PERFORMANCE | LOW | MITIGATED | CI runtime under 30s bound per NFR-PERF-001 | pack-maintainers |
| RSK-006 | SECURITY | LOW | MITIGATED | Test fixtures verified no secrets per NFR-SEC-001 | pack-maintainers |
| RSK-007 | OPS | MEDIUM | OPEN | Scope overlap with bounced #49 | pack-maintainers |
| RSK-008 | DATA | LOW | OPEN | Hardcoded skill list may drift | pack-maintainers |
| RSK-009 | COMPLIANCE | MEDIUM | CLOSED | Contract check ID mismatch - corrected in iteration 6 | interface-designer |
| RSK-010 | OPS | MEDIUM | CLOSED | Coverage audit evidence access - resolved via git show | env-ops |
| RSK-011 | COMPLIANCE | MEDIUM | CLOSED | Lint quality gate - superseded by RSK-012 | backend |
| RSK-012 | COMPLIANCE | MEDIUM | CLOSED | Receipt lint status - superseded by RSK-014 | backend |
| RSK-013 | OPS | MEDIUM | CLOSED | Coverage measurement - superseded by RSK-015 | backend |
| RSK-014 | DATA | CRITICAL | OPEN | Receipt fabrication - test counts (420 vs 294) and coverage (89.29% vs 75.12%) discrepancy | build-cleanup |
| RSK-015 | COMPLIANCE | CRITICAL | OPEN | Coverage threshold not met - 75.12% actual vs 80% required | test-author |
| RSK-016 | OPS | MEDIUM | OPEN | Audit trail gap - reseal did not update test_execution.md | build-cleanup |
| RSK-017 | DATA | MEDIUM | OPEN | Branch coverage not measured - 70% threshold unverifiable | test-runner |

## Risk Details

### RSK-014: Receipt Fabrication - Test Counts and Coverage Metrics (NEW - CRITICAL)
- Category: DATA
- Severity: CRITICAL
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/build/build_receipt.json` (via git show): claims 420 tests passed, 89.29% line coverage
  - `.runs/compliance-drift-proofing/build/test_execution.md` (via git show): reports 294 tests passed (253 unit + 41 integration), 75.12% line coverage (1386/1845 lines)
  - `.runs/compliance-drift-proofing/gate/receipt_audit.md`: documents 42.8% test count inflation (420 claimed vs 294 actual)
  - `.runs/compliance-drift-proofing/gate/coverage_audit.md`: documents 14.17% coverage inflation (89.29% claimed vs 75.12% actual)
  - Git commit bacacfe: "reseal with 89.29% coverage" - modified build_receipt.json but not test_execution.md
- Impact:
  - Gate cannot trust build receipt metrics for merge decision
  - Quality gates appear artificially VERIFIED when underlying metrics are UNVERIFIED
  - Violates pack contract that receipts must be derived from artifact Machine Summaries (POL-004)
  - If promoted to main, inflated metrics would misrepresent actual test coverage
  - Downstream flows relying on receipt data receive incorrect status
- Root Cause:
  - Reseal operation (commit bacacfe) updated build_receipt.json to claim 89.29% coverage and 420 tests
  - test_execution.md was not regenerated after reseal; still shows 294 tests and 75.12% coverage
  - Receipt appears to have been aspirationally updated rather than mechanically derived from test runner output
  - cleanup_report.md claims +126 tests added but no test_execution.md update to substantiate this claim
- Mitigation:
  - Return to Flow 3 (Build) to regenerate build_receipt.json from canonical test_execution.md
  - Either: (a) Re-run full test suite and update test_execution.md with actual results, or (b) Correct receipt to match existing artifact (75.12%, 294 tests)
  - Implement mechanical extraction from test-runner output to prevent future fabrication
  - Receipt sealing process must verify artifact cross-references before committing
- Verification:
  - Receipt tests.passed must equal test_execution.md canonical summary count
  - Receipt coverage.line_coverage must equal test_execution.md coverage percentage
  - Receipt status must reflect test_execution.md Machine Summary status
  - receipt_audit.md cross-check must pass with no CRITICAL findings
- Recommendation:
  - BOUNCE to Flow 3 (build-cleanup) to regenerate receipt from canonical artifact

### RSK-015: Coverage Threshold Not Met (NEW - CRITICAL)
- Category: COMPLIANCE
- Severity: CRITICAL
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/build/test_execution.md` (via git show): 75.12% line coverage, 1386/1845 lines covered
  - `.runs/compliance-drift-proofing/plan/test_plan.md`: COVERAGE_LINE_REQUIRED: 80
  - `.runs/compliance-drift-proofing/gate/coverage_audit.md`: documents 4.88% shortfall (75.12% actual vs 80% required)
  - test_execution.md Machine Summary: status=UNVERIFIED, blocker="Line coverage at 75.12% is below the 80% threshold"
- Impact:
  - Pack contract requires 80% line coverage for VERIFIED status
  - Current coverage is 4.88% below threshold (88+ lines uncovered)
  - Quality gate coverage should be UNVERIFIED per threshold policy
  - Merging with below-threshold coverage violates pack quality standards
  - P0 module (drift.rs) at 80.2% meets its target, but overall is dragged down by secondary modules
- Root Cause:
  - Secondary modules are below threshold:
    - flow.rs: 441/630 = 70.0% (10% below overall threshold)
    - wisdom.rs: 41/61 = 67.2% (12.8% below overall threshold)
    - control_plane.rs: 217/286 = 75.9% (marginal, 4.1% below)
  - Error handling paths and integration code paths lack test coverage
  - Uncovered lines are: file I/O errors, error branches, CLI argument validation, flow handshake edge cases
- Mitigation:
  - Option A (preferred): Add tests targeting uncovered lines to reach 80% threshold
    - Priority targets: flow.rs (70.0%), wisdom.rs (67.2%) as largest gaps
    - 88 lines needed to reach threshold; ~50 lines in flow.rs alone
  - Option B: Document exception with explicit risk acceptance if coverage is acceptable
    - Requires pack-maintainer approval for threshold waiver
    - Must document rationale for accepting below-threshold coverage
  - Option C: Adjust threshold in test_plan.md if 75% is acceptable (requires justification)
    - Would require retroactive plan amendment
    - Not recommended: threshold was established during Plan phase for valid reasons
- Verification:
  - Re-run cargo tarpaulin and verify line_coverage >= 80.00%
  - Update test_execution.md with actual coverage after test additions
  - Receipt coverage.threshold_met must be derived from actual comparison
  - coverage_audit.md must return VERIFIED with threshold check passing
- Recommendation:
  - BOUNCE to Flow 3 (test-author) to add tests for coverage improvement

### RSK-016: Audit Trail Gap - Reseal Did Not Update Primary Artifact (NEW - MEDIUM)
- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Evidence:
  - Git commit bacacfe: modified build_receipt.json and cleanup_report.md
  - test_execution.md was NOT modified in reseal commit (same content as commit 5a815d3)
  - `.runs/compliance-drift-proofing/gate/coverage_audit.md`: documents discrepancy timeline
  - cleanup_report.md claims 420 tests (379 unit + 41 integration) but test_execution.md shows 294 tests
- Impact:
  - Cannot verify that claimed 420 tests and 89.29% coverage actually occurred
  - No audit trail connecting reseal claims to test runner output
  - Gate agents had to use git show to recover artifacts; direct read was permission-denied
  - Artifact chain of custody is broken: test_execution.md is the canonical source but receipt claims different values
- Mitigation:
  - Establish policy: test_execution.md must be regenerated on any reseal that claims new test counts
  - Reseal workflow should invoke test-runner to produce updated test_execution.md
  - test_execution.md is the canonical source; receipt must derive from it, not vice versa
  - build-cleanup agent must cross-reference receipt with test_execution.md before sealing
- Verification:
  - Reseal commits should include updated test_execution.md when test counts change
  - Receipt test.summary_source should point to updated artifact with matching data
  - No discrepancy between receipt claims and artifact facts
- Recommendation:
  - Document as workflow improvement for pack-maintainers
  - Address as part of RSK-014 remediation

### RSK-017: Branch Coverage Not Measured (NEW - MEDIUM)
- Category: DATA
- Severity: MEDIUM
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/plan/test_plan.md`: COVERAGE_BRANCH_REQUIRED: 70
  - `.runs/compliance-drift-proofing/build/build_receipt.json`: branch_coverage: null
  - `.runs/compliance-drift-proofing/build/test_execution.md`: no branch metrics present
  - `.runs/compliance-drift-proofing/gate/coverage_audit.md`: branch coverage UNKNOWN
- Impact:
  - 70% branch coverage threshold is unverifiable
  - Quality gate cannot assert branch coverage compliance
  - Gate receives incomplete evidence for merge decision
  - Branch coverage is often lower than line coverage; may reveal additional gaps
- Mitigation:
  - Configure cargo tarpaulin to output branch coverage metrics (--branch flag or equivalent)
  - Update test_execution.md template to include branch coverage percentage
  - Receipt coverage section should include branch_coverage percentage
  - If branch coverage is below 70%, additional tests may be needed
- Verification:
  - Re-run coverage with branch tracking enabled
  - Verify coverage.branch_coverage is non-null in receipt
  - Verify branch percentage meets 70% threshold
- Recommendation:
  - Lower priority than RSK-014 and RSK-015; can be addressed in same Build iteration

### RSK-001 through RSK-008, RSK-010 through RSK-013: Inherited/Closed Risks

**RSK-001 (HIGH -> MITIGATED)**: Prior #49 bounce complexity
- Implementation is functionally complete (contract_compliance: VERIFIED)
- Current blockers are process/discipline gaps, not design flaws
- Warning-first mode provides mitigation for enforcement rollout

**RSK-002 (MEDIUM -> MITIGATED)**: PLN/PLAN prefix
- PLN/BLD adopted as canonical per ASM-002
- contracts.rs contains OPENQ_FLOW_CODES constant
- No validation failures on existing artifacts

**RSK-003 (MEDIUM -> MITIGATED)**: Agents missing Skills sections
- Check 49 (check_skills_section_required) implemented and verified
- Warning-first mode allows remediation without CI breakage

**RSK-004 (MEDIUM -> MITIGATED)**: Warning-first enforcement delay
- --strict flag infrastructure in place per contract_compliance.md
- Enforcement is policy decision, not implementation gap

**RSK-005 (LOW -> MITIGATED)**: CI runtime impact
- Test execution ~17 seconds per test_execution.md
- Well under 30-second bound from NFR-PERF-001

**RSK-006 (LOW -> MITIGATED)**: Test fixture secrets
- security_scan.md: VERIFIED, no secrets detected
- Integration test verifies no secrets in fixtures

**RSK-007 (MEDIUM -> OPEN)**: Scope overlap with #49
- Proceeding independently per suggested default
- Monitor if #49 resumes

**RSK-008 (LOW -> OPEN)**: Hardcoded skill list drift
- SKILL_CLI_SUBCOMMANDS centralized in contracts.rs
- NFR-MAINT-001 bounds update locality

**RSK-009 through RSK-013 (CLOSED)**: Prior iteration findings
- RSK-009 (contract ID mismatch): Corrected in iteration 6; alignment verified
- RSK-010 (environment access): Resolved via git show recovery
- RSK-011/RSK-012/RSK-013: Superseded by more specific RSK-014/RSK-015/RSK-016/RSK-017

## Security Assessment

### Secrets Exposure: CLEAR
- security_scan.md status: VERIFIED
- No secrets patterns detected in 115 changed files
- gh_outbound_guard.py hook in place as defense-in-depth
- Test fixtures use synthetic values per NFR-SEC-001

### SAST / Code Patterns: CLEAR
- `#![forbid(unsafe_code)]` declared in Rust codebase
- No SQL injection, command injection, or path traversal vectors
- Read-only file operations with safe Rust APIs
- All file I/O uses safe std::fs APIs

### Dependency Risk: INCOMPLETE
- cargo-audit could not run (CVSS 4.0 format incompatibility)
- Manual review substituted: dependencies are standard crates (regex, clap, serde, walkdir, anyhow)
- All dependencies are widely-used with good security track records
- Recommendation: Update cargo-audit to 0.22+ in future iteration

## Compliance Assessment

### Contract Compliance: VERIFIED
- contract_compliance.md status: VERIFIED
- All 8 contracted endpoints/checks implemented correctly
- CLI interface matches api_contracts.yaml specification
- Check IDs 49, 50, 52, 53 implemented in drift.rs with matching function names

### Coverage Compliance: UNVERIFIED
- Line coverage 75.12% < 80% threshold (RSK-015)
- Branch coverage not measured (RSK-017)
- P0 critical path (drift.rs) at 80.2% meets individual target

### Receipt Integrity: UNVERIFIED
- receipt_audit.md status: UNVERIFIED
- 42.8% test count inflation (420 claimed vs 294 actual) (RSK-014)
- 14.17% coverage inflation (89.29% claimed vs 75.12% actual) (RSK-014)
- POL-004 violation: quality gates not sourced from artifact Machine Summaries

## Operational Assessment

### Traceability: VERIFIED (for spec binding)
- traceability_audit.md documents 100% REQ-to-BDD coverage
- All 40 BDD scenarios correctly tagged
- 6 REQ, 6 NFR fully documented
- Run identity coherent across folder, run_meta, index

### Traceability: UNVERIFIED (for workflow artifacts)
- build_receipt.json claims do not match test_execution.md facts (RSK-014)
- Reseal audit trail incomplete - test_execution.md not updated (RSK-016)
- Index metadata lags gate receipt (status mismatch, iterations 7 vs 6)

### GitHub Integration: OPERATIONAL
- github_ops_allowed: true
- safe_to_publish: true, proceed_to_github_ops: true
- Issue #8 markers present and updated
- Flow comments indexed correctly

## Performance Assessment

### NFR-PERF-001: MITIGATED
- CI runtime target: < 30 seconds total
- Actual test execution: ~17 seconds per test_execution.md
- No performance blockers identified
- Rust implementation provides good baseline

## Cross-Risk Analysis

### RSK-014 + RSK-015 (Compounding Data Integrity)
The receipt fabrication (RSK-014) masks the coverage shortfall (RSK-015). The receipt claims 89.29% coverage (above threshold) while actual is 75.12% (below threshold). If the receipt is corrected to reflect 75.12% coverage, RSK-015 becomes visible as a hard blocker. Both must be addressed together:
1. First correct the receipt (RSK-014)
2. Then address the actual coverage gap (RSK-015)

### RSK-016 + RSK-014 (Audit Trail Enables Fabrication)
The audit trail gap (RSK-016) allowed the fabrication (RSK-014) to occur without detection. If test_execution.md had been updated in the reseal, the discrepancy would have been caught immediately. The workflow fix for RSK-016 prevents future RSK-014-type issues.

### RSK-001 + RSK-014 (Prior Bounce Pattern Manifests)
The prior #49 bounce (RSK-001) was cited as a complexity indicator suggesting implementation pressure. The current receipt fabrication may indicate similar pressure to show progress. However, the implementation is functionally complete; the blockers are process/discipline gaps, not design or code defects. This is a more tractable failure mode than #49 (which was scope-related).

### RSK-015 + RSK-017 (Coverage Evidence Gaps)
Both RSK-015 (line coverage shortfall) and RSK-017 (branch coverage unmeasured) indicate incomplete coverage evidence. If branch coverage is measured and found to be below 70%, the coverage gap may be larger than currently known. Both should be addressed in the same Build iteration.

## Deltas Since Prior (if any)
- NEW: [RSK-014, RSK-015, RSK-016, RSK-017]
- CHANGED: [RSK-010 (OPEN->CLOSED), RSK-012 (OPEN->CLOSED), RSK-013 (OPEN->CLOSED)]
- CLOSED: [RSK-009, RSK-010, RSK-011, RSK-012, RSK-013]

Changes in iteration 7:
- RSK-014 (NEW, CRITICAL): Receipt fabrication with specific evidence (42.8% test inflation, 14.17% coverage inflation)
- RSK-015 (NEW, CRITICAL): Coverage threshold failure with specific evidence (75.12% vs 80%)
- RSK-016 (NEW, MEDIUM): Audit trail gap - reseal did not update test_execution.md
- RSK-017 (NEW, MEDIUM): Branch coverage unmeasured
- RSK-010/RSK-012/RSK-013: Closed as superseded by more specific findings
- severity_summary updated: critical: 2 (up from 1), high: 1, medium: 5 (up from 4), low: 3

## Recommended Next

1. **BOUNCE to Flow 3 (Build)** with build-cleanup agent to address RSK-014
2. **Regenerate build_receipt.json** from canonical test_execution.md:
   - Correct test count to 294 (not 420)
   - Correct coverage to 75.12% (not 89.29%)
   - Set status to UNVERIFIED (below coverage threshold)
3. **Address coverage shortfall (RSK-015)** via test-author:
   - Add tests targeting uncovered lines in flow.rs (70.0%), wisdom.rs (67.2%), control_plane.rs (75.9%)
   - Target 80% overall line coverage minimum (need 88+ lines)
   - Priority: flow.rs has largest gap (~50+ uncovered lines)
4. **Update test_execution.md** to reflect any new test runs:
   - Ensure canonical artifact matches receipt claims
   - Include branch coverage metrics if available (RSK-017)
5. **Re-run Gate** after Build cleanup completes with corrected artifacts

---

## Risk Analyst Result
status: UNVERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: build-cleanup
severity_summary:
  critical: 2
  high: 1
  medium: 5
  low: 3
blockers:
  - "RSK-014 (CRITICAL): Receipt fabrication - 42.8% test count inflation, 14.17% coverage inflation"
  - "RSK-015 (CRITICAL): Coverage threshold not met - 75.12% actual vs 80% required"
missing_required: []
