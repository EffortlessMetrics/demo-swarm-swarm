# Observability Specification: DemoSwarm Documentation-Code Alignment Audit

## Overview

This observability spec defines verification signals and drift detection mechanisms for a **documentation alignment** task. Unlike runtime observability (metrics, traces, SLOs), this spec focuses on:

1. **Verification Signals**: How we know documentation changes are correct
2. **Consistency Checks**: How we detect future drift
3. **Test Verification**: How we know test count claims are accurate
4. **Audit Trail**: What evidence we capture for each change

### System Boundary

- **Scope**: Documentation files (README.md, DEMO_RUN.md, CLAUDE.md, architecture.md, CHANGELOG.md, secondary docs)
- **Critical Paths**: Authoritative source updates (CLAUDE.md, architecture.md) flowing to downstream docs
- **Environments**: Local development, CI/CD validation (pack-check)

### Non-Applicability Notice

Traditional runtime observability (latency metrics, trace spans, distributed SLOs) does not apply to this documentation-only change. This spec adapts the observability framework to document-oriented verification.

## Verification Signals (Metrics Equivalent)

Documentation alignment verification is measured through deterministic checks:

### VS-001: Flow Count Consistency Check

- **Purpose**: Verify "six flows" has been replaced with "seven flows"
- **Command**: `grep -ri "six flows" README.md DEMO_RUN.md docs/explanation/architecture.md`
- **Pass Criteria**: Zero matches returned
- **Frequency**: Gate verification (once per phase completion)
- **Traceability**: NFR-DOC-001 MET-1, REQ-001 AC-5

### VS-002: Pack-Check Validation

- **Purpose**: Verify pack structural integrity after documentation changes
- **Command**: `bash .claude/scripts/pack-check.sh --no-color`
- **Pass Criteria**: Exit code 0, no FAIL entries in output
- **Frequency**: After Phase 2 completion, before merge
- **Traceability**: NFR-TRACE-001 MET-1, RSK-001 mitigation

### VS-003: Test Count Source Verification

- **Purpose**: Verify 102 passing test count claim has valid source
- **Command**: `grep -c "102 tests" ./test_output.log` or equivalent
- **Pass Criteria**: Count matches documented claim
- **Actual Source**: `./test_output.log` (repo root; line 109 per requirements)
- **Traceability**: REQ-005 AC-4, RSK-002 mitigation

### VS-004: Security Claim Evidence Verification

- **Purpose**: Verify each security claim references specific code evidence
- **Checks**:
  - ReDoS immunity claim references Rust regex crate usage
  - Path traversal limitation references secrets.rs
- **Pass Criteria**: Each security claim has file:line reference
- **Traceability**: NFR-SEC-001 MET-1, REQ-006 AC-4

## Logs (Audit Trail Equivalent)

For documentation changes, "logs" are the durable audit artifacts:

### LOG-001: Phase Checkpoint Commits

- **Purpose**: Record before/after state for each documentation phase
- **Format**: Git commit with descriptive message per phase
- **Fields**:
  - `phase`: 1|2|3|4
  - `files_modified`: list of paths
  - `verification_passed`: true|false
  - `timestamp`: ISO8601
- **Retention**: Git history (permanent)
- **Traceability**: ADR OPT-003 (granular revert capability)

### LOG-002: Pack-Check Execution Log

- **Purpose**: Record pack-check results for audit
- **Output Path**: `.runs/local-alignment-audit-aba1c6/build/pack_check_output.log`
- **Fields**:
  - `exit_code`: number
  - `check_results`: pass/fail per check
  - `warnings`: list
- **Traceability**: NFR-TRACE-001

### LOG-003: Grep Verification Log

- **Purpose**: Record "six flows" elimination verification
- **Output Path**: `.runs/local-alignment-audit-aba1c6/gate/grep_verification.log`
- **Fields**:
  - `search_pattern`: "six flows"
  - `files_searched`: list
  - `matches_found`: 0 (expected)
  - `timestamp`: ISO8601
- **Traceability**: NFR-DOC-001

## Traces (Derivation Lineage)

For documentation work, "traces" represent the derivation chain from authoritative source to downstream docs:

### TRACE-001: Authority Chain

```
CLAUDE.md (L13: "7 flows")
  -> docs/explanation/architecture.md (flow enumeration)
    -> README.md (flow count reference)
    -> DEMO_RUN.md (flow enumeration)
    -> CHANGELOG.md (v1.0.0 annotation)
```

### TRACE-002: Requirement Coverage Chain

```
REQ-001..REQ-007 (requirements.md)
  -> Phase 1: CLAUDE.md, architecture.md (authoritative)
  -> Phase 2: README.md, DEMO_RUN.md, CHANGELOG.md (public)
  -> Phase 3: glossary.md, CONTRIBUTING.md, etc. (secondary)
  -> Phase 4: structure.rs (pack-check fixtures if needed)
```

### Span Model (Document Update Lineage)

- **Root Span**: ADR decision (OPT-003 chosen)
- **Child Spans**: Phase 1 updates, Phase 2 updates, Phase 3 updates (optional), Phase 4 updates (if needed)
- **Attributes**: `phase_number`, `files_updated`, `verification_result`

## SLOs (Acceptance Criteria as Service Levels)

Documentation alignment has pass/fail criteria rather than percentage targets:

### SLO-001: Flow Count Accuracy

- **SLI**: Count of "six flows" occurrences in public docs
- **Target**: 0 occurrences
- **Window**: Point-in-time at Gate
- **Error Budget**: None (binary pass/fail)
- **Traceability**: NFR-DOC-001, REQ-001

### SLO-002: Pack-Check Continuity

- **SLI**: Pack-check exit code
- **Target**: Exit code 0
- **Window**: Post Phase 2, Pre-merge
- **Error Budget**: None (binary pass/fail)
- **Traceability**: NFR-TRACE-001

### SLO-003: Security Claims Verifiable

- **SLI**: Percentage of security claims with code evidence references
- **Target**: 100%
- **Window**: Gate verification
- **Error Budget**: None (all claims must be evidenced)
- **Traceability**: NFR-SEC-001

### SLO-004: Test Count Accuracy

- **SLI**: Documented test count matches test_output.log
- **Target**: Exact match (102 passing)
- **Window**: Gate verification
- **Error Budget**: None (must match source artifact)
- **Traceability**: REQ-005

## Alerts (Verification Failures)

### ALERT-001: Six-Flow Reference Found

- **Condition**: `grep -ri "six flows"` returns non-zero exit code (matches found)
- **Severity**: BLOCKING
- **Runbook**: Fix remaining "six flows" references in identified files; rerun verification
- **Signal Link**: VS-001
- **Traceability**: REQ-001 AC-5

### ALERT-002: Pack-Check Failure

- **Condition**: `pack-check.sh` returns non-zero exit code
- **Severity**: BLOCKING
- **Runbook**:
  1. Review pack-check output for specific failures
  2. If structure.rs fixtures fail: Proceed to Phase 4
  3. If other failures: Investigate and fix before merge
- **Signal Link**: VS-002
- **Traceability**: NFR-TRACE-001, RSK-001

### ALERT-003: Test Count Mismatch

- **Condition**: Documented count != test_output.log count
- **Severity**: MAJOR
- **Runbook**: Update documentation to match actual test execution results
- **Signal Link**: VS-003
- **Traceability**: REQ-005, RSK-002

### ALERT-004: Security Claim Without Evidence

- **Condition**: Security claim in docs lacks file:line reference
- **Severity**: MAJOR
- **Runbook**: Add specific code evidence reference (e.g., "secrets.rs line 14")
- **Signal Link**: VS-004
- **Traceability**: NFR-SEC-001

## Dashboards (Verification Summary)

For documentation work, the "dashboard" is the Gate verification summary:

### Dashboard: Alignment Audit Gate Summary

| Check             | Status    | Details                     |
| ----------------- | --------- | --------------------------- |
| Six-flow grep     | PASS/FAIL | 0 matches in public docs    |
| Pack-check        | PASS/FAIL | All checks green            |
| Test count        | PASS/FAIL | 102 matches test_output.log |
| Security evidence | PASS/FAIL | All claims have code refs   |

**Why This Matters**: Provides single-view verification that alignment is complete before merge.

## Traceability (REQ/NFR/Risk -> Signals)

### Requirements to Signals Map

| Requirement                  | Signal        | Alert                     |
| ---------------------------- | ------------- | ------------------------- |
| REQ-001 (flow count in docs) | VS-001        | ALERT-001                 |
| REQ-002 (flow overlap docs)  | Manual review | None (no automated check) |
| REQ-003 (Flow 7 docs)        | Manual review | None                      |
| REQ-004 (CLAUDE.md table)    | Manual review | None                      |
| REQ-005 (test count)         | VS-003        | ALERT-003                 |
| REQ-006 (security posture)   | VS-004        | ALERT-004                 |
| REQ-007 (agent colors)       | Manual review | None                      |

### NFRs to Signals Map

| NFR                        | Signal | SLO     |
| -------------------------- | ------ | ------- |
| NFR-DOC-001 (consistency)  | VS-001 | SLO-001 |
| NFR-SEC-001 (evidence)     | VS-004 | SLO-003 |
| NFR-TRACE-001 (pack-check) | VS-002 | SLO-002 |

### Risks to Detection Map

| Risk                          | Detection Mechanism           |
| ----------------------------- | ----------------------------- |
| RSK-001 (pack-check drift)    | VS-002 (pack-check execution) |
| RSK-002 (test count drift)    | VS-003 (count verification)   |
| RSK-003 (path traversal)      | Out of scope (code, not docs) |
| RSK-004 (flow variant drift)  | Manual review (no automation) |
| RSK-005 (color coding)        | Manual review                 |
| RSK-006 (Flow 7 undocumented) | Manual review                 |
| RSK-007 (security claims)     | VS-004                        |

## Drift Detection (Future Observability)

Recommendations for preventing future documentation drift:

### DDR-001: Automated Flow Count Check

- **Proposal**: Add pack-check rule that verifies flow count consistency across docs
- **Implementation**: grep-based check in pack-check.sh
- **Owner**: Tooling maintainers

### DDR-002: Test Count CI Integration

- **Proposal**: CI job that updates test count documentation or fails if mismatch
- **Implementation**: Post-test step that compares documented count to actual
- **Owner**: CI/CD maintainers

### DDR-003: Security Claim Registry

- **Proposal**: Structured file listing all security claims with code evidence pointers
- **Implementation**: YAML or JSON file checked by pack-check
- **Owner**: Security reviewers

## Assumptions Made to Proceed

- **ASM-OBS-001**: Runtime metrics/traces are not applicable to this documentation-only task
  - Impact if wrong: Would need to define actual service observability, but no services are being built

- **ASM-OBS-002**: grep verification is sufficient for "six flows" detection (no complex regex needed)
  - Impact if wrong: Could miss edge cases like "six-flows" or "6 flows"; mitigate with comprehensive pattern

- **ASM-OBS-003**: Pack-check is the authoritative structural validation tool
  - Impact if wrong: Would need alternative validation approach

- **ASM-OBS-004**: test_output.log is the canonical source for test counts
  - Impact if wrong: Would need to identify correct artifact; per requirements.md this is assumed correct

## Questions / Clarifications Needed

- **OQ-OBS-001**: Should pack-check include a flow count consistency rule going forward?
  - Suggested default: Yes, to prevent future drift
  - Impact: Would require tooling enhancement (Phase 4 or follow-up)

- **OQ-OBS-002**: Should there be CI/CD integration for test count verification?
  - Suggested default: Defer to Wisdom flow as enhancement
  - Impact: Without automation, test count may drift again (RSK-002)

## Inventory (machine countable)

- METRIC: vs_001_six_flow_count type=gauge labels=[file_path]
- METRIC: vs_002_pack_check_exit_code type=gauge labels=[run_id]
- METRIC: vs_003_test_count_match type=gauge labels=[documented,actual]
- METRIC: vs_004_security_claims_evidenced type=gauge labels=[claim_id]

- LOG_EVENT: phase_checkpoint level=INFO fields=[phase,files_modified,verification_passed,timestamp]
- LOG_EVENT: pack_check_execution level=INFO fields=[exit_code,check_results,warnings]
- LOG_EVENT: grep_verification level=INFO fields=[pattern,files_searched,matches_found,timestamp]

- TRACE_SPAN: authority_chain parent=null attrs=[source_file,derived_files]
- TRACE_SPAN: phase_update parent=authority_chain attrs=[phase_number,files_updated,verification_result]

- SLO: slo_001_flow_count_accuracy target=0_occurrences window=point_in_time
- SLO: slo_002_pack_check_continuity target=exit_code_0 window=pre_merge
- SLO: slo_003_security_claims_verifiable target=100_percent window=gate
- SLO: slo_004_test_count_accuracy target=exact_match window=gate

- ALERT: alert_001_six_flow_found severity=BLOCKING runbook=TBD
- ALERT: alert_002_pack_check_failure severity=BLOCKING runbook=.runs/local-alignment-audit-aba1c6/runbooks/pack_check_failure.md
- ALERT: alert_003_test_count_mismatch severity=MAJOR runbook=TBD
- ALERT: alert_004_security_claim_no_evidence severity=MAJOR runbook=TBD

## Machine Summary

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Traditional runtime observability (metrics/traces/SLOs) adapted to documentation context
  - Some verification signals require manual review (REQ-002, REQ-003, REQ-007) with no automated checks
  - Drift detection recommendations (DDR-001..003) are future enhancements, not current scope
```
