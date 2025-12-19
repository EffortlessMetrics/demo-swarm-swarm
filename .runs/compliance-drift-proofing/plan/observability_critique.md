# Observability Critique for compliance-drift-proofing

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Runbook paths are TBD; should be created during Build phase
  - SLO-REL-002 measurement method is vague; should specify CI assertion
  - RSK-008 traceability references unmeasurable "false negative rate"
```

## Iteration Control

```yaml
can_further_iteration_help: no
rationale: "All issues are MINOR and informational. The spec is coherent, measurable, and actionable. Remaining gaps are documentation/polish concerns that do not block implementation."
```

## Metrics

```yaml
severity_summary:
  critical: 0
  major: 0
  minor: 3
```

## Summary (1-5 bullets)

- The observability spec is well-structured for a CLI tool context, correctly adapting traditional observability concepts (metrics, logs, traces, SLOs, alerts) to CLI diagnostic output patterns.
- All four SLOs have clear targets and measurement methods tied to specific output fields (e.g., `stats.duration_ms` for SLO-PERF-001).
- Traceability is comprehensive: all REQs, NFRs, and identified risks are mapped to observable signals and alerts.
- PII/secrets guidance is explicit and appropriate (NFR-SEC-001: never print file contents, only paths and rule violations).
- The inventory section is complete with 3 metrics, 4 log events, 4 SLOs, and 3 alerts - adequate coverage for the defined scope.

## Critical Issues

(none)

## Major Issues

(none)

## Minor Issues

- [MINOR] OC-MIN-001: TBD runbook paths - ALERT-STRICT-001, ALERT-PERF-001, and ALERT-BASELINE-001 all reference runbook paths ending in "(TBD)". While this is acceptable at Plan phase, these should be created during Build/Gate. Evidence: lines 169, 176, 183 of `observability_spec.md`.

- [MINOR] OC-MIN-002: Vague SLO-REL-002 measurement method - The measurement "Verify sorted order in output parser" does not specify how this will be asserted in CI. Suggested improvement: "Assert sorted order in CI integration test via output parser comparison." Evidence: line 156-157 of `observability_spec.md`.

- [MINOR] OC-MIN-003: Unmeasurable RSK-008 signal - The Risks to Signals table references "CHECK-050 false negative rate" for RSK-008, but no metric or test method is defined to measure false negatives. This is appropriate as manual audit is the mitigation, but the traceability table is misleading. Evidence: line 337 of `observability_spec.md`.

## Traceability Gaps

(none - all REQ and NFR identifiers have observability coverage)

The following requirements are covered by signals:
- REQ-001: CHECK-050 violations + ALERT-STRICT-001
- REQ-002: CHECK-049 violations + ALERT-STRICT-001
- REQ-003: CHECK-051 violations + ALERT-STRICT-001
- REQ-005: Exit code behavior + ALERT-STRICT-001
- REQ-006: Baseline comparison + ALERT-BASELINE-001
- NFR-PERF-001: stats.duration_ms + ALERT-PERF-001 + SLO-PERF-001
- NFR-REL-001: Output hash + SLO-REL-001, SLO-REL-002
- NFR-OPS-001: JSON structure + schema validation
- NFR-SEC-001: Covered by policy (no metric needed; code review verification)

## Questions for Humans

(none - all open questions in the spec have suggested defaults that are reasonable)

## Inventory (machine countable)

- OC_MINOR: OC-MIN-001
- OC_MINOR: OC-MIN-002
- OC_MINOR: OC-MIN-003

---

## Detailed Analysis

### 1) Handshake Validity

**PASS**: The observability spec includes a properly formatted `## Inventory (machine countable)` section (lines 358-373) with all required marker prefixes:
- `METRIC`: 3 entries (pack_check_violations_total, pack_check_duration_ms, pack_check_files_scanned)
- `LOG_EVENT`: 4 entries (SCAN_START, RULE_APPLIED, VIOLATION_FOUND, SCAN_COMPLETE)
- `SLO`: 4 entries (SLO-PERF-001, SLO-PERF-002, SLO-REL-001, SLO-REL-002)
- `ALERT`: 3 entries (ALERT-STRICT-001, ALERT-PERF-001, ALERT-BASELINE-001)

All alerts include runbook pointers (though marked TBD).

### 2) Measurability of Critical Journeys

**PASS**: The three critical paths identified (developer validation loop, CI pipeline validation, diagnostic triage) are all measurable:

| Journey | Rate/Errors/Duration Signal | Debug Anchor |
|---------|---------------------------|--------------|
| Developer validation | stats.violations_total, exit code | VIOLATION_FOUND log event with check_id, file, line |
| CI pipeline | exit code, stats.duration_ms | JSON output with full diagnostics array |
| Diagnostic triage | CheckDiagnostic with message, suggestion | File path + line number in output format |

### 3) Safety: PII/Secrets + Cardinality

**PASS**: Section "PII/Secrets Guidance (NFR-SEC-001)" at lines 90-95 explicitly addresses:
- NEVER print file contents in diagnostic output
- ONLY print file paths and rule violation descriptions
- Test fixtures must use obviously synthetic values
- Violations reference line numbers, not line contents

Cardinality rules at lines 66-68:
- Allowed labels: severity, check_id, file_path (relative), line_number
- Prohibited labels: file_contents, user_id, full_absolute_path

### 4) SLOs + Alerts are Actionable

**PASS**: All four SLOs have explicit targets and measurement methods:

| SLO | Target | Window | Measurement |
|-----|--------|--------|-------------|
| SLO-PERF-001 | < 30 seconds | per invocation | SCAN_COMPLETE.duration_ms |
| SLO-PERF-002 | < 5 seconds per rule | per rule | baseline vs with-rule delta |
| SLO-REL-001 | 100% byte-identical | consecutive runs | hash comparison |
| SLO-REL-002 | sorted by file, rule ID | per run | output parser verification |

All three alerts specify severity (BLOCKING or WARNING) and runbook pointers.

### 5) Traceability + Verification Hooks

**PASS**: The "Traceability" section (lines 315-338) maps:
- REQ-001 through REQ-006 to specific CHECK-NNN IDs and alerts
- NFR-PERF-001, NFR-REL-001, NFR-OPS-001 to measurement methods
- RSK-001, RSK-004, RSK-005, RSK-008 to observable signals

The test_plan.md "Non-Behavioral Verification" section (lines 251-259) includes verification strategies for:
- NFR-PERF-001: CI timing wrapper
- NFR-REL-001: Determinism test (run twice, diff output)
- NFR-OPS-001: Manual review of diagnostic output format
- NFR-SEC-001: Code review of reporter.rs output functions

This cross-reference is adequate for verification.

---

## Verdict Rationale

The observability spec is **VERIFIED** because:

1. All required structural elements are present (inventory with proper markers).
2. All SLOs are measurable with explicit targets and measurement methods.
3. Critical journeys (developer loop, CI validation, diagnostic triage) have adequate coverage.
4. PII/secrets posture is explicit and appropriate.
5. Traceability is comprehensive (no REQ/NFR gaps).
6. Test plan includes verification hooks for observability assertions.

The three MINOR issues identified are polish concerns:
- TBD runbooks are normal at Plan phase
- Vague measurement method for SLO-REL-002 is clarifiable during implementation
- RSK-008 "false negative rate" is correctly positioned as manual audit (not automated metric)

No further Plan-phase iteration can meaningfully improve these - they are implementation-phase concerns.
