# Observability Specification: Compliance Drift-Proofing

## Overview

This document defines the observability contract for the `pack-check` CLI tool extensions supporting compliance drift-proofing (checks 50-51 and related infrastructure).

### System Boundary

- **Component**: `pack-check` Rust CLI tool
- **Location**: `tools/demoswarm-pack-check/`
- **Invocation**: `bash .claude/scripts/pack-check.sh [OPTIONS]`
- **Execution context**: Local developer workstation or CI runner (GitHub Actions)

### Critical Paths

1. **Developer validation loop**: Run pack-check locally to verify pack compliance before commit
2. **CI pipeline validation**: Automated check on PR/push that gates merge
3. **Diagnostic triage**: When validation fails, developers need actionable output to fix issues

### Environments

| Environment | Characteristics | Observability Priority |
|-------------|-----------------|----------------------|
| Local dev | Interactive, frequent runs, human reads output | Human-readable diagnostics |
| CI runner | Automated, non-interactive, machine parses output | Structured JSON, exit codes |
| Debug mode | Troubleshooting failures, verbose context needed | Extended context, file paths |

---

## Diagnostic Output Format (Metrics Equivalent)

For a CLI tool, "metrics" are the structured diagnostic outputs that enable measurement and analysis.

### Standard Output Structure

All diagnostic output follows this format (NFR-OPS-001 MET-1):

```
[SEVERITY] [RULE_ID] path/to/file:LINE - violation description
```

Example:
```
[WARN] CHECK-050 .claude/commands/flow-1-signal.md:42 - Flow command contains demoswarm.sh invocation
[WARN] CHECK-051 .runs/foo/signal/open_questions.md:15 - Invalid OpenQ prefix: OQ-PLAN-001 (expected SIG|PLN|BLD|GAT|DEP|WIS)
```

### Severity Levels

| Level | Usage | Exit Code Impact |
|-------|-------|-----------------|
| ERROR | Rule violation when --strict enabled | Exit 1 |
| WARN | Rule violation (default mode) | Exit 0 (no --strict) |
| INFO | Informational (file scanned, rule applied) | Exit 0 |

### Naming Convention

Rule IDs follow the pattern: `CHECK-NNN` where NNN is the check number in drift.rs.

- `CHECK-050`: Flow boundary enforcement (REQ-001)
- `CHECK-051`: OpenQ prefix validation (REQ-003)
- `CHECK-049`: Skills section enforcement (REQ-002, existing)

### Cardinality Rules

- **Allowed labels**: severity, check_id, file_path (relative), line_number
- **Prohibited labels**: file_contents, user_id, full_absolute_path (security/portability)

---

## Logging (Diagnostic Events)

### Event Taxonomy

| Event | Level | When Emitted | Required Fields |
|-------|-------|--------------|-----------------|
| `SCAN_START` | INFO | Beginning validation run | timestamp, version, flags |
| `RULE_APPLIED` | DEBUG | Each rule execution | check_id, file_count |
| `VIOLATION_FOUND` | WARN/ERROR | Rule violation detected | check_id, file, line, message |
| `SCAN_COMPLETE` | INFO | Validation finished | total_files, total_violations, duration_ms |

### Required Fields (all events)

- `timestamp`: ISO8601 format
- `check_id`: Rule identifier (CHECK-NNN)
- `file`: Repo-relative path
- `line`: Line number (if applicable, 0 otherwise)
- `message`: Human-readable description

### PII/Secrets Guidance (NFR-SEC-001)

- NEVER print file contents in diagnostic output
- ONLY print file paths and rule violation descriptions
- Test fixtures must use obviously synthetic values (e.g., "TEST_VALUE_ONLY")
- Violations reference line numbers, not line contents

---

## Traces (Not Applicable)

As a local CLI tool without distributed execution, traditional distributed tracing is not applicable. The equivalent concept is **execution flow logging**:

### Execution Flow

```
pack-check invocation
  |-- Parse CLI arguments
  |-- Load pack structure
  |-- For each check in drift.rs:
  |     |-- Scan applicable files
  |     |-- Report violations
  |-- Aggregate results
  |-- Emit summary
  |-- Exit with appropriate code
```

### Context Propagation

For CI integration, the following context should be available:

- `CI_JOB_ID`: When running in CI (from environment)
- `GIT_SHA`: Current commit (from git or environment)
- `RUN_ID`: DemoSwarm run context (if available)

---

## SLOs (Performance Contracts)

### SLO-PERF-001: CI Validation Runtime

- **SLI**: Total pack-check execution time
- **Target**: < 30 seconds (NFR-PERF-001 MET-1)
- **Window**: Per invocation (not rolling)
- **Error Budget**: N/A for CLI tool
- **Measurement**: Captured in `SCAN_COMPLETE` event `duration_ms`

### SLO-PERF-002: Incremental Rule Overhead

- **SLI**: Time added by each new rule (CHECK-050, CHECK-051)
- **Target**: < 5 seconds per rule (NFR-PERF-001 MET-2)
- **Window**: Per rule invocation
- **Measurement**: Difference in baseline vs with-rule execution

### SLO-REL-001: Deterministic Output

- **SLI**: Output consistency across identical runs
- **Target**: 100% byte-identical output (NFR-REL-001 MET-1)
- **Window**: Any two consecutive runs on same input
- **Measurement**: Hash comparison of output (CI test)

### SLO-REL-002: Stable Ordering

- **SLI**: Warning/error ordering consistency
- **Target**: Sorted by file path, then rule ID (NFR-REL-001 MET-2)
- **Window**: All output within a run
- **Measurement**: Verify sorted order in output parser

---

## Alerts (CI Integration)

For a CLI tool, "alerts" are CI pipeline failure conditions.

### ALERT-STRICT-001: Strict Mode Failure

- **Condition**: Exit code != 0 when --strict flag enabled
- **Severity**: BLOCKING (PR cannot merge)
- **Primary Signal**: Exit code from pack-check
- **Runbook**: `docs/troubleshooting/pack-check-failures.md` (TBD)

### ALERT-PERF-001: Runtime Bound Exceeded

- **Condition**: `duration_ms` > 30000 in SCAN_COMPLETE event
- **Severity**: WARNING (CI continues but flags regression)
- **Primary Signal**: JSON output `stats.duration_ms`
- **Runbook**: `docs/troubleshooting/pack-check-performance.md` (TBD)

### ALERT-BASELINE-001: Baseline Regression

- **Condition**: Previously passing artifacts now produce warnings
- **Severity**: BLOCKING (when --strict, after migration period)
- **Primary Signal**: Violation count > baseline
- **Runbook**: `docs/troubleshooting/baseline-regression.md` (TBD)

---

## Dashboards (CI Visibility)

### What to Track

1. **Violation Trends**: Count of warnings/errors per rule over time
2. **Performance Trends**: Execution duration over commits
3. **Rule Coverage**: Files scanned vs files with violations
4. **Migration Progress**: Count of known exceptions remaining

### Recommended CI Outputs

- Pack-check summary in PR comment (when enabled)
- JSON artifact for downstream analysis
- Badge showing current compliance status

---

## Exit Code Semantics

### Standard Exit Codes (REQ-005, OQ-PLN-002)

| Exit Code | Meaning | When |
|-----------|---------|------|
| 0 | Success | No violations, or warnings-only without --strict |
| 1 | Failure | Errors present, or warnings present with --strict |
| 2 | Reserved | Future: distinguish warnings-elevated-to-errors from native errors |

### --strict Flag Behavior

```
Without --strict:
  - Warnings logged to stderr
  - Exit 0 (violations do not block)

With --strict:
  - Warnings elevated to errors
  - Exit 1 if any violation present
  - Same output format, different severity label
```

---

## JSON Output Structure (--format json)

For CI consumption, pack-check supports structured JSON output (NFR-OPS-001):

```json
{
  "version": "1.0.0",
  "timestamp": "2025-12-18T12:00:00Z",
  "flags": {
    "strict": false,
    "format": "json"
  },
  "stats": {
    "files_scanned": 42,
    "violations_total": 3,
    "violations_by_severity": {
      "error": 0,
      "warn": 3,
      "info": 0
    },
    "duration_ms": 1250
  },
  "violations": [
    {
      "check_id": "CHECK-050",
      "severity": "warn",
      "file": ".claude/commands/flow-1-signal.md",
      "line": 42,
      "message": "Flow command contains demoswarm.sh invocation",
      "rule_name": "flow_boundary_enforcement",
      "suggestion": "Move CLI invocation to agent layer or skill doc"
    }
  ],
  "checks_applied": [
    {"check_id": "CHECK-049", "files_scanned": 14, "violations": 0},
    {"check_id": "CHECK-050", "files_scanned": 6, "violations": 1},
    {"check_id": "CHECK-051", "files_scanned": 8, "violations": 2}
  ]
}
```

### Required JSON Fields

- `version`: Output schema version
- `stats.violations_total`: Total count for quick assessment
- `stats.duration_ms`: For SLO-PERF-001 verification
- `violations[]`: Array with full diagnostic details
- `violations[].check_id`: For filtering and aggregation

---

## Error Message Guidelines

### Structure (NFR-OPS-001 MET-2)

Every error/warning message must include:

1. **What**: Clear description of the violation
2. **Where**: File path and line number
3. **Why**: Rule being enforced (with ID)
4. **How**: Suggested remediation or documentation link

### Examples

**Good:**
```
[WARN] CHECK-050 .claude/commands/flow-1-signal.md:42 - Flow command contains 'demoswarm.sh' invocation.
       Flow commands should delegate CLI operations to agents/skills.
       See: docs/pack-structure.md#flow-boundaries
```

**Bad:**
```
Warning: Invalid content in file
```

### Remediation Hints

| Check | Standard Remediation |
|-------|---------------------|
| CHECK-049 | Add `## Skills` section to agent file listing required skill invocations |
| CHECK-050 | Remove demoswarm.sh reference; delegate to agent layer or skill doc |
| CHECK-051 | Update QID to use canonical prefix (SIG/PLN/BLD/GAT/DEP/WIS) |

---

## Traceability

### Requirements to Signals

| Requirement | SLI/Signal | Alert |
|-------------|------------|-------|
| REQ-001 (Flow Boundary) | CHECK-050 violations | ALERT-STRICT-001 |
| REQ-002 (Skills Section) | CHECK-049 violations | ALERT-STRICT-001 |
| REQ-003 (OpenQ Prefix) | CHECK-051 violations | ALERT-STRICT-001 |
| REQ-005 (Warning-First) | Exit code behavior | ALERT-STRICT-001 |
| REQ-006 (No False Positives) | Baseline comparison | ALERT-BASELINE-001 |
| NFR-PERF-001 (Runtime) | stats.duration_ms | ALERT-PERF-001 |
| NFR-REL-001 (Deterministic) | Output hash | CI test assertion |
| NFR-OPS-001 (Diagnostics) | JSON structure | Schema validation |

### Risks to Signals

| Risk | Observable Signal | Mitigation Verification |
|------|-------------------|------------------------|
| RSK-001 (Prior Bounce) | Warning count trends | Decreasing over time indicates successful rollout |
| RSK-004 (Enforcement Delay) | --strict adoption rate | CI configs using --strict |
| RSK-005 (CI Runtime) | duration_ms in JSON | Stays under 30s bound |
| RSK-008 (Skill List Drift) | CHECK-050 false negative rate | Manual audit when skills added |

---

## Assumptions Made to Proceed

- ASM-OBS-001: JSON output format from existing pack-check (--format json) can be extended; structure assumed based on common CLI patterns.
- ASM-OBS-002: CI runners have sufficient environment variables (CI_JOB_ID, GIT_SHA) for context correlation.
- ASM-OBS-003: 30-second SLO is appropriate for pack size; baseline measurement needed in Build phase.
- ASM-OBS-004: Exit code 2 reserved for future use; current implementation uses only 0 and 1.

---

## Questions / Clarifications Needed

- Q: OQ-PLN-002 (from ADR) - Should exit code 2 distinguish warnings-elevated-to-errors from native errors? Suggested default: Use exit 1 for both initially; exit 2 as future enhancement.
- Q: Should JSON output include the actual line content for context? Suggested default: No (NFR-SEC-001 prohibits content in output).
- Q: Is there an existing CI dashboard to integrate with? Suggested default: Output JSON artifacts; dashboard integration is downstream concern.

---

## Inventory (machine countable)

- METRIC: pack_check_violations_total type=counter labels=[check_id,severity]
- METRIC: pack_check_duration_ms type=gauge labels=[invocation_type]
- METRIC: pack_check_files_scanned type=counter labels=[check_id]
- LOG_EVENT: SCAN_START level=INFO fields=[timestamp,version,flags]
- LOG_EVENT: RULE_APPLIED level=DEBUG fields=[check_id,file_count]
- LOG_EVENT: VIOLATION_FOUND level=WARN fields=[check_id,file,line,message]
- LOG_EVENT: SCAN_COMPLETE level=INFO fields=[total_files,total_violations,duration_ms]
- SLO: SLO-PERF-001 target=30s window=per-invocation
- SLO: SLO-PERF-002 target=5s window=per-rule
- SLO: SLO-REL-001 target=100% window=consecutive-runs
- SLO: SLO-REL-002 target=sorted window=per-run
- ALERT: ALERT-STRICT-001 severity=BLOCKING runbook=docs/troubleshooting/pack-check-failures.md
- ALERT: ALERT-PERF-001 severity=WARNING runbook=docs/troubleshooting/pack-check-performance.md
- ALERT: ALERT-BASELINE-001 severity=BLOCKING runbook=docs/troubleshooting/baseline-regression.md

---

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
  - Exit code 2 semantics deferred (using 0/1 for now per OQ-PLN-002 suggested default)
  - Runbook paths are TBD; documentation to be created during implementation
  - JSON output schema assumed based on CLI patterns; may need adjustment during Build
```
