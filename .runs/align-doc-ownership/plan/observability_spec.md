# Observability Spec: Documentation Ownership Alignment

## Overview

This run (`align-doc-ownership`) is a **documentation-only refactoring** implementing OPT-002 (Pragmatic Enforcement) from the ADR. There is no runtime component; therefore, traditional observability (metrics, logs, traces, SLOs) does not apply.

Instead, this spec defines:
1. **Build-time observability** -- counts of artifacts changed
2. **Verification observability** -- pass/fail criteria for gates
3. **Drift detection observability** -- CI gates for ongoing enforcement

### System Boundary

- **In scope**: `.claude/commands/flow-*.md` (6 files), `.claude/agents/*.md` (55 files), `.claude/skills/*/SKILL.md` (7 files), `CLAUDE.md` (1 file)
- **Out of scope**: Code execution paths, runtime behavior, external integrations

### Critical Paths

There are no runtime critical paths. The critical verification path is:

1. Subtask execution (ST-001 through ST-006)
2. pack-check validation (boundary enforcement rules)
3. doc-drift validation (duplication detection)
4. Toy Run A/B (flows 1-4) validation

### Environments

- **Development**: Local agent execution, pack-check runs
- **CI**: pack-check + doc-drift gates (future, post-merge)
- **No production environment**: Documentation-only; no deployment target

---

## Metrics (Build-Time Counts)

Since this is not runtime code, metrics are **artifact counts** captured in the build receipt.

### Count Categories

| Metric Name | Description | Expected Range |
|-------------|-------------|----------------|
| `agent_docs_changed` | Number of agent docs modified for consistency | 10-55 |
| `flow_commands_changed` | Number of flow commands cleaned of skill plumbing | 0-6 |
| `skill_docs_changed` | Number of skill docs updated with CLI truth | 0-7 |
| `claude_md_changed` | Whether CLAUDE.md was normalized (0 or 1) | 0-1 |
| `pack_check_rules_added` | Number of new boundary enforcement rules | 2-5 |
| `validation_runs_completed` | Number of Toy Run A/B cycles completed | 1 |

### Naming Convention

Counts follow: `<domain>_<artifact_type>_<action>` (e.g., `agent_docs_changed`).

### Cardinality

Not applicable (no labels; these are scalar counts).

---

## Logs (Build-Time Events)

No runtime logs. Build events are recorded in:

| Event | Location | Purpose |
|-------|----------|---------|
| Subtask start/complete | `flow_plan.md` (TodoWrite) | Session navigation |
| pack-check results | Terminal output + receipt | Validation gate |
| doc-drift results | Terminal output + receipt | Validation gate |
| Validation run outcome | `docs/maintainers/validation-log.md` | Audit trail |

### Required Fields for Validation Log

Per REQ-006 AC-2, validation log entries must include:
- `date`: ISO8601 timestamp
- `run_ids`: Toy Run A and B identifiers
- `flows_executed`: 1, 2, 3, 4
- `status`: PASS or FAIL
- `notes`: Any observations

### PII/Secrets Guidance

Not applicable. Documentation files should not contain secrets. secrets-sanitizer gate verifies this.

---

## Traces (Not Applicable)

No distributed tracing. This is documentation refactoring with no runtime component.

---

## SLOs (Verification Criteria)

Instead of runtime SLOs, we define **verification criteria** that gate completion.

### SLO-001: pack-check Passes

- **SLI**: pack-check exit code
- **Target**: Exit code 0 (all checks pass)
- **Window**: At gate (Flow 4)
- **Error Budget**: None; must pass

### SLO-002: doc-drift Passes

- **SLI**: `scripts/check-doc-drift.sh` exit code
- **Target**: Exit code 0 (no duplicate documentation detected)
- **Window**: At gate (Flow 4)
- **Error Budget**: None; must pass

### SLO-003: Boundary Enforcement Rules Fire on Violations

- **SLI**: pack-check correctly detects seeded violations (negative test)
- **Target**: 100% of seeded violations detected
- **Window**: During ST-004 development
- **Error Budget**: None; rules must work

### SLO-004: Validation Run Succeeds

- **SLI**: Toy Run A/B completion status (flows 1-4)
- **Target**: Both runs complete without errors attributable to alignment changes
- **Window**: After ST-006 completion
- **Error Budget**: None; must pass

---

## Alerts (Gate Failures)

No runtime alerts. Gate failures are detected synchronously during flow execution.

### ALERT-001: pack-check Failure

- **Condition**: pack-check exit code != 0
- **Severity**: BLOCKING (cannot proceed to merge)
- **Signal**: pack-check terminal output
- **Runbook**: Review pack-check output; fix violations; rerun

### ALERT-002: doc-drift Failure

- **Condition**: `scripts/check-doc-drift.sh` exit code != 0
- **Severity**: BLOCKING (cannot proceed to merge)
- **Signal**: doc-drift terminal output
- **Runbook**: Review duplicated content; move to single source of truth; rerun

### ALERT-003: Validation Run Failure

- **Condition**: Toy Run A/B fails in any flow (1-4)
- **Severity**: BLOCKING (cannot record completion)
- **Signal**: Flow error output
- **Runbook**: Identify regression; fix alignment changes; rerun validation

---

## Dashboards (Not Applicable)

No runtime dashboards. Progress visibility is via:

1. **TodoWrite**: Session navigation (ephemeral)
2. **flow_plan.md**: Durable flow state
3. **Receipts**: Artifact counts and gate results
4. **GitHub PR**: Diff review and discussion

---

## Traceability

### Requirements to Verification Mapping

| Requirement | Verification Criteria | Alert |
|-------------|----------------------|-------|
| REQ-001 (Flow Command Boundary) | SLO-001 + SLO-003 | ALERT-001 |
| REQ-002 (Agent Doc Consistency) | SLO-001 | ALERT-001 |
| REQ-003 (Skill Doc Ownership) | SLO-002 | ALERT-002 |
| REQ-004 (CLAUDE.md Scope) | SLO-002 | ALERT-002 |
| REQ-005 (Subtask Partitioning) | Receipt counts | N/A |
| REQ-006 (Validation Run) | SLO-004 | ALERT-003 |
| REQ-007 (Archive-Over-Delete) | PR review (manual) | N/A |

### NFR to Verification Mapping

| NFR | Verification Criteria | Alert |
|-----|----------------------|-------|
| NFR-MAINT-001 (Maintainability) | SLO-002 + PR review | ALERT-002 |
| NFR-TEST-001 (Validation Tooling) | SLO-001 + SLO-003 | ALERT-001 |
| NFR-REGR-001 (No Regression) | SLO-004 + secrets gate | ALERT-003 |

### Key Risks to Verification Mapping

| Risk | Verification Criteria | Notes |
|------|----------------------|-------|
| RSK-001 (Merge Conflicts) | PR review | Mitigated by distinct touches patterns |
| RSK-002 (ST-004 Scope) | Receipt counts by subtask | Monitor if ST-004 dominates |
| RSK-003 (False Positives) | SLO-003 (negative test) | Rules must not over-match |
| RSK-004 (Validation Delay) | SLO-004 | Expected gate, not true risk |

---

## Drift Detection (Ongoing CI)

After this run merges, drift detection operates via:

### CI Gate 1: pack-check

- **Trigger**: Every PR, every push to main
- **Checks**: Boundary enforcement rules (flow skill plumbing, enum consistency, Skills section presence)
- **Failure mode**: Block merge until violations fixed

### CI Gate 2: doc-drift

- **Trigger**: Every PR, every push to main
- **Checks**: Duplicate documentation detection (CLAUDE.md vs skill docs, agent docs vs skill docs)
- **Failure mode**: Block merge until duplicates removed

### CI Gate 3: secrets-sanitizer

- **Trigger**: Every flow gate
- **Checks**: No secrets in publish surface
- **Failure mode**: Block publish until secrets removed

---

## Assumptions Made to Proceed

- ASM-OBS-001: pack-check boundary rules can be extended without major Rust refactoring (per ASM-004 in ADR)
- ASM-OBS-002: doc-drift script (`scripts/check-doc-drift.sh`) exists and is functional
- ASM-OBS-003: Toy Run A/B is sufficient validation coverage for documentation alignment
- ASM-OBS-004: CI integration for pack-check and doc-drift will be configured post-merge (not in this run scope)

---

## Questions / Clarifications Needed

- Q-OBS-001: Should CI gates be configured as part of this run, or deferred to a follow-on run? **Suggested default**: Deferred; this run focuses on documentation alignment and local validation.
- Q-OBS-002: Are there existing pack-check boundary rules to build upon, or is this greenfield? **Suggested default**: Check `tools/demoswarm-pack-check/` for existing patterns.
- Q-OBS-003: Should validation log entries include machine-parseable markers for automated extraction? **Suggested default**: No; validation log is human-oriented audit trail.

---

## Inventory (machine countable)

- METRIC: agent_docs_changed type=counter labels=[]
- METRIC: flow_commands_changed type=counter labels=[]
- METRIC: skill_docs_changed type=counter labels=[]
- METRIC: claude_md_changed type=gauge labels=[]
- METRIC: pack_check_rules_added type=counter labels=[]
- METRIC: validation_runs_completed type=counter labels=[]
- LOG_EVENT: subtask_start level=INFO fields=[subtask_id, timestamp]
- LOG_EVENT: subtask_complete level=INFO fields=[subtask_id, timestamp, status]
- LOG_EVENT: validation_run level=INFO fields=[run_id, flows, status, notes]
- SLO: pack_check_passes target=exit_code_0 window=gate
- SLO: doc_drift_passes target=exit_code_0 window=gate
- SLO: boundary_rules_fire target=100pct_detection window=development
- SLO: validation_run_succeeds target=both_runs_pass window=post_alignment
- ALERT: pack_check_failure severity=BLOCKING runbook=fix-violations
- ALERT: doc_drift_failure severity=BLOCKING runbook=remove-duplicates
- ALERT: validation_run_failure severity=BLOCKING runbook=fix-regression

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
  - No runtime component means traditional observability (metrics/logs/traces) is not applicable
  - CI gate configuration is deferred to post-merge; ongoing drift detection depends on CI setup
  - Negative test for boundary rules (SLO-003) requires careful design to avoid false positives
```
