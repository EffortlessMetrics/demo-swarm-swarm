# Working with Receipts

> How to read, interpret, and use flow receipts.

Receipts are structured JSON artifacts produced by cleanup agents at the end of each flow. They summarize what happened and provide an audit trail. Routing is based on prose handoffs from agents, not receipt fields.

---

## Receipt Basics

Each flow produces a receipt:

| Flow   | Receipt Path                                |
| ------ | ------------------------------------------- |
| Signal | `.runs/<run-id>/signal/signal_receipt.json` |
| Plan   | `.runs/<run-id>/plan/plan_receipt.json`     |
| Build  | `.runs/<run-id>/build/build_receipt.json`   |
| Review | `.runs/<run-id>/review/review_receipt.json` |
| Gate   | `.runs/<run-id>/gate/gate_receipt.json`     |
| Deploy | `.runs/<run-id>/deploy/deploy_receipt.json` |
| Wisdom | `.runs/<run-id>/wisdom/wisdom_receipt.json` |

---

## Receipt Philosophy

### Receipts Are Logs, Not Gatekeepers

**Core principle:** The repo's current state (HEAD + working tree + staged diff + actual tool results) is what you're shipping. Receipts help you investigate what happened — they are not the primary mechanism for verifying outcomes.

**Trust hierarchy:**

1. **Live repo state + executed evidence** (primary)
2. **Receipts** (cached evidence of prior state)
3. **Narrative summaries** (useful for humans, never a control input)

### Staleness Detection

Receipts include staleness detection fields:

```json
{
  "evidence_sha": "abc123...",
  "generated_at": "2025-12-22T10:30:00Z"
}
```

**Drift rule:** If `receipt.evidence_sha != git HEAD`, treat the receipt as **stale** — use it for investigation, not for pass/fail determination.

---

## Common Receipt Fields

All receipts share a base schema:

```json
{
  "run_id": "<run-id>",
  "flow": "<flow-name>",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | RERUN | BOUNCE | FIX_ENV",
  "route_to_flow": null,
  "route_to_agent": null,
  "missing_required": [],
  "blockers": [],
  "completed_at": "<ISO8601>",
  "evidence_sha": "<commit-sha>",
  "generated_at": "<ISO8601>"
}
```

### DevLT Tracking (Developer Lead Time)

Receipts may include a `devlt` section for retrospective analysis of human vs machine effort:

```json
{
  "devlt": {
    "flow_started_at": "2025-12-22T10:00:00Z",
    "flow_completed_at": "2025-12-22T10:45:00Z",
    "human_checkpoints": [
      { "at": "2025-12-22T10:00:00Z", "action": "flow_start" },
      { "at": "2025-12-22T10:30:00Z", "action": "question_answered" },
      { "at": "2025-12-22T10:45:00Z", "action": "flow_approved" }
    ],
    "machine_duration_sec": 2700,
    "human_checkpoint_count": 3,
    "estimated_human_attention_min": 15,
    "estimation_basis": "checkpoint_count * 5min average"
  }
}
```

**Field semantics:**

| Field                           | Type       | Meaning                                                            |
| ------------------------------- | ---------- | ------------------------------------------------------------------ |
| `flow_started_at`               | Observable | ISO8601 timestamp when flow began                                  |
| `flow_completed_at`             | Observable | ISO8601 timestamp when flow completed                              |
| `human_checkpoints`             | Observable | Array of human interaction points with timestamps and action types |
| `machine_duration_sec`          | Derived    | Wall-clock time (not execution time)                               |
| `human_checkpoint_count`        | Observable | Count of human interactions                                        |
| `estimated_human_attention_min` | Inference  | Rough estimate based on checkpoint count and typical review times  |
| `estimation_basis`              | Metadata   | Explanation of how the estimate was derived                        |

**Observable vs inferred:**

- Timestamps and counts are **facts** (derived from logs/artifacts)
- `estimated_human_attention_min` is an **inference** (labeled as such)
- Token costs are **not tracked** here (unreliably available)

**Purpose:** DevLT is for retrospective analysis in Flow 7 (Wisdom), not for gating or routing. It helps answer: "How much human attention did this run actually require?"

**Common checkpoint actions:**

- `flow_start` — Flow initiated
- `question_answered` — Human responded to a clarifying question
- `decision_made` — Human made a routing or design decision
- `approval_given` — Human approved proceeding
- `flow_approved` — Flow completed and approved

### Status Values

| Status           | Meaning                                                              |
| ---------------- | -------------------------------------------------------------------- |
| `VERIFIED`       | Flow completed successfully; artifacts exist and verification passed |
| `UNVERIFIED`     | Flow completed but with gaps, concerns, or missing verification      |
| `CANNOT_PROCEED` | Mechanical failure only (IO/permissions/tooling)                     |

### Stations Tracking

Receipts include per-station execution status:

```json
{
  "stations": {
    "test-author": { "executed": true, "result": "PASS" },
    "test-critic": { "executed": true, "result": "PASS" },
    "code-implementer": { "executed": true, "result": "PASS" },
    "mutator": { "executed": false, "result": "SKIPPED" }
  }
}
```

| Result    | Meaning                                          |
| --------- | ------------------------------------------------ |
| `PASS`    | Station ran and succeeded                        |
| `FAIL`    | Station ran and failed                           |
| `SKIPPED` | Station was skipped (optional or not configured) |
| `UNKNOWN` | Station status is unclear                        |

---

## Reading Specific Receipts

### signal_receipt.json

```json
{
  "counts": {
    "functional_requirements": 5,
    "non_functional_requirements": 3,
    "bdd_scenarios": 12,
    "open_questions": 2,
    "risks": { "critical": 0, "high": 1, "medium": 2, "low": 1 }
  },
  "quality_gates": {
    "requirements_critic": "VERIFIED",
    "bdd_critic": "VERIFIED"
  },
  "key_artifacts": [
    "requirements.md",
    "features/*.feature",
    "open_questions.md"
  ]
}
```

**Key questions:**

- How many requirements were captured? (`counts.functional_requirements`, `counts.non_functional_requirements`)
- Are the requirements validated? (`quality_gates.requirements_critic`)
- Are there open questions? (`counts.open_questions`)
- What risks were identified? (`counts.risks`)

### build_receipt.json

```json
{
  "schema_version": "build_receipt_v1",
  "counts": {
    "tests_written": 15,
    "files_changed": 8,
    "mutation_score": null,
    "open_questions": 0
  },
  "tests": {
    "summary_source": "build/test_execution.md",
    "passed": 45,
    "failed": 0,
    "skipped": 2,
    "xfailed": 0,
    "xpassed": null
  },
  "quality_gates": {
    "test_critic": "VERIFIED",
    "code_critic": "VERIFIED",
    "self_reviewer": "VERIFIED"
  }
}
```

**Key questions:**

- Did tests pass? (`tests.passed`, `tests.failed`)
- Were tests reviewed? (`quality_gates.test_critic`)
- Was code reviewed? (`quality_gates.code_critic`)
- Is the build self-verified? (`quality_gates.self_reviewer`)

### gate_receipt.json

```json
{
  "merge_verdict": "MERGE",
  "counts": {
    "receipt_checks_total": 5,
    "receipt_checks_passed": 5,
    "contract_violations": 0,
    "security_findings": 0,
    "policy_violations": 0,
    "coverage_line_percent": 85.5,
    "coverage_branch_percent": 72.0
  },
  "quality_gates": {
    "merge_decider": "MERGE",
    "receipt_audit": "VERIFIED",
    "contract_compliance": "VERIFIED",
    "security_scan": "VERIFIED",
    "coverage_audit": "VERIFIED"
  }
}
```

**Key questions:**

- Can this be merged? (`merge_verdict`)
- Are there security issues? (`counts.security_findings`)
- Is coverage adequate? (`counts.coverage_line_percent`, `counts.coverage_branch_percent`)
- Did all gate checks pass? (`quality_gates.*`)

### deploy_receipt.json

```json
{
  "deployment_verdict": "STABLE",
  "deploy_action": "COMPLETED",
  "governance_enforcement": "VERIFIED",
  "gate_verdict": "MERGE",
  "counts": {
    "failed_checks": 0,
    "ci_checks_total": 5,
    "deploy_events_total": 1,
    "verification_checks_total": 3
  },
  "signals": {
    "ci_signal": "GREEN",
    "deploy_signal": "HEALTHY",
    "smoke_signal": "STABLE"
  }
}
```

**Key questions:**

- Is deployment stable? (`deployment_verdict`, `signals.smoke_signal`)
- Did CI pass? (`signals.ci_signal`)
- Is governance verified? (`governance_enforcement`)

---

## Using Receipts

### Receipts Are for Audit, Not Routing

**Key distinction:** Orchestrators route on prose handoffs from agents. Receipts capture what happened for audit and analysis.

The routing fields (`recommended_action`, `route_to_flow`, `route_to_agent`) exist in receipts because cleanup agents derive them from the agent's prose handoff. They are **not** read by orchestrators for live routing decisions.

### Downstream Flows

Later flows read earlier receipts to understand context:

```
Flow 5 (Gate) reads:
  - build_receipt.json (what was built)
  - review_receipt.json (what was reviewed)

Flow 7 (Wisdom) reads:
  - All prior receipts for timeline and analysis
```

### Investigating with Receipts

Use receipts to understand what happened during a run:

```python
# Pseudo-code for investigation
receipt = read_receipt("build_receipt.json")

# What was the outcome?
print(receipt.status)  # VERIFIED, UNVERIFIED, CANNOT_PROCEED

# What were the blockers?
print(receipt.blockers)

# What did tests show?
print(receipt.tests.passed, receipt.tests.failed)
```

For live routing decisions, orchestrators read the prose handoff that agents return.

---

## Mechanical Counting

Receipt counts are **mechanical** — derived from tool output, not estimated:

```bash
# Using demoswarm CLI for counts
bash .claude/scripts/demoswarm.sh count pattern \
  --file .runs/feat-auth/signal/requirements.md \
  --regex "^### REQ-"
```

**Never estimate counts.** If a count can't be derived mechanically, use `null`.

---

## Evidence vs Format

**If verification evidence exists but a receipt is malformed:**

- Treat it as a **pack/tooling defect**, not an engineering failure
- Ship based on the underlying evidence **if gates pass**
- Record the defect: `status: UNVERIFIED`, `blockers: ["receipt_tooling_error: <details>"]`

**If the evidence itself is missing:**

- That's **unverified work**, not paperwork drift
- Route back to run the missing verification

---

## SKIPPED Stubs

When a station is skipped, cleanup agents create explicit SKIPPED stubs:

```markdown
# Mutation Report

status: SKIPPED
reason: Mutation testing not configured for this project
evidence_sha: abc123...
generated_at: 2025-12-22T10:30:00Z
```

This ensures nothing is silently missing. Downstream flows and Flow 7 (Wisdom) can see exactly what happened.

---

## Reporters and Receipts

**Key invariant:** Reporters summarize from receipts; they do not recompute counts or upgrade statuses.

```
build-cleanup writes build_receipt.json
    ↓
gh-reporter reads build_receipt.json
    ↓
gh-reporter posts summary to GitHub issue
```

The reporter trusts the receipt. If the receipt says `status: UNVERIFIED`, the reporter reflects that — it doesn't re-verify.

---

## Reading Receipts Programmatically

Use the demoswarm CLI for mechanical reading:

```bash
# Read a specific field
bash .claude/scripts/demoswarm.sh receipt get \
  --file .runs/feat-auth/build/build_receipt.json \
  --key "tests.passed"

# Count receipts in a run
bash .claude/scripts/demoswarm.sh receipts count \
  --run-dir .runs/feat-auth
```

---

## Common Receipt Patterns

These examples show what receipts look like. Remember: these are **audit records**, not routing inputs. Cleanup agents derive the routing fields from agent prose handoffs.

### Healthy Flow

```json
{
  "status": "VERIFIED",
  "blockers": [],
  "quality_gates": {
    "test_critic": "VERIFIED",
    "code_critic": "VERIFIED"
  }
}
```

### Issues But Proceeding

```json
{
  "status": "UNVERIFIED",
  "recommended_action": "PROCEED",
  "blockers": ["Missing integration tests for edge case"],
  "quality_gates": {
    "test_critic": "UNVERIFIED",
    "code_critic": "VERIFIED"
  }
}
```

The `recommended_action: "PROCEED"` was derived from the agent's handoff which said something like "Ready to move forward despite the gap in integration tests."

### Needs Upstream Fix

```json
{
  "status": "UNVERIFIED",
  "recommended_action": "BOUNCE",
  "route_to_flow": 2,
  "route_to_agent": "interface-designer",
  "blockers": ["API contract doesn't match implementation"]
}
```

The routing fields were derived from the agent's handoff which said something like "This needs to go back to Plan. The interface-designer should update the API contract."

### Mechanical Failure

```json
{
  "status": "CANNOT_PROCEED",
  "recommended_action": "FIX_ENV",
  "missing_required": ["Test runner not installed"],
  "blockers": ["pytest command not found"]
}
```

---

## See Also

- [contracts.md](../reference/contracts.md) — Receipt schemas
- [agent-data-flows.md](../reference/agent-data-flows.md) — Flow dependencies
- [demoswarm-cli.md](../reference/demoswarm-cli.md) — CLI for reading receipts
- [stable-markers.md](../reference/stable-markers.md) — Markers for counting
