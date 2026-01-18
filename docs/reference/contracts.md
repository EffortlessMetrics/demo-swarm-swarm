# Contracts Reference

> Control-plane blocks, enums, and schemas.

This document indexes canonical contracts. The source of truth is `CLAUDE.md` and `.claude/commands/*`.

---

## Canonical Enums (Audit Vocabulary)

These enums define the **audit vocabulary** used in receipts and machine summaries. They are not routing contracts for agents.

**Agents express these concepts in prose handoffs**, not enum values. The orchestrator reads handoffs; cleanup agents write receipts with these values for audit purposes.

### Status axis

```
VERIFIED | UNVERIFIED | CANNOT_PROCEED
```

| Value            | Meaning                                       |
| ---------------- | --------------------------------------------- |
| `VERIFIED`       | Adequate for purpose; blockers empty          |
| `UNVERIFIED`     | Gaps/concerns remain; artifacts still written |
| `CANNOT_PROCEED` | Mechanical failure only (IO/perms/tooling)    |

**Rule:** `CANNOT_PROCEED` requires `missing_required` to be non-empty.

### Recommended action

```
PROCEED | RERUN | BOUNCE | FIX_ENV
```

| Value     | Meaning                                                                         |
| --------- | ------------------------------------------------------------------------------- |
| `PROCEED` | Default: continue even when open questions exist (capture blockers/assumptions) |
| `RERUN`   | Same station again with a deterministic improvement expected                    |
| `BOUNCE`  | Route to a specific flow/agent for an actionable fix                            |
| `FIX_ENV` | Environment/tooling issue (paired with `status: CANNOT_PROCEED`)                |

**Note:** Agents don't emit these values directly. They write prose like "run code-implementer next" or "this needs to go back to Plan." Cleanup agents translate prose into enum values when writing receipts.

---

## Control-Plane Blocks

These specialized blocks are returned by specific agents for **boolean gate decisions**, not general routing. They answer yes/no questions at publish boundaries (can we commit? can we push?).

For general routing, orchestrators read **prose handoffs** from agents. Control-plane blocks are distinct: they're structured responses from specialized agents (secrets-sanitizer, repo-operator) that need deterministic parsing for safety gates.

### Gate Result (secrets-sanitizer)

```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
findings_count: <int>
blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
blocker_reason: <string | null>
```

Notes:

- The sanitizer is a **boolean gate**, not a router. It says yes/no.
- If `safe_to_publish: false`, the flow doesn't push. The orchestrator decides next steps.
- `blocker_kind` is the machine-readable category: `NONE` (not blocked), `MECHANICAL` (IO/tooling failure), `SECRET_IN_CODE` (staged code needs fix), `SECRET_IN_ARTIFACT` (artifact can't be redacted).
- `blocker_reason` is the human-readable explanation (if BLOCKED); otherwise null.

**Canonical location:** `CLAUDE.md` (with PACK-CONTRACT markers)

### Repo Operator Result

```yaml
## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_WARNING | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_classification:
  unexpected_staged_paths: []
  unexpected_unstaged_paths: []
  unexpected_untracked_paths: []
anomaly_paths: []
```

Notes:

- `commit_sha` is always populated (current HEAD on no-op).
- `publish_surface` is always present: `PUSHED` only when push succeeds; `NOT_PUSHED` for local-only checkpoints.
- `status` values:
  - `COMPLETED` - operation succeeded, no anomalies
  - `COMPLETED_WITH_WARNING` - only untracked anomalies; push allowed
  - `COMPLETED_WITH_ANOMALY` - tracked/staged anomalies; push blocked
- `anomaly_classification` breaks down by risk level (HIGH blocks push, LOW warns only).
- `anomaly_paths` - DEPRECATED; union of classification arrays for backward compatibility.

**Canonical location:** `CLAUDE.md` (with PACK-CONTRACT markers)

---

## Machine Summary (Receipts Only)

Machine Summary is an **audit format** used by cleanup agents when writing receipts. It is not a communication format between agents.

**Who uses Machine Summary:**

- Cleanup agents (signal-cleanup, plan-cleanup, build-cleanup, etc.) write these when producing receipts
- The format enables mechanical processing of receipt data

**Who does NOT use Machine Summary:**

- Critics communicate via prose critiques with severity markers
- Workers communicate via prose handoffs
- Orchestrators route on prose handoffs, not Machine Summary

```yaml
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_agent: <agent-name | null>
route_to_flow: <1|2|3|4|5|6 | null>

blockers: []
missing_required: []
concerns: []

can_further_iteration_help: yes | no # critics only

severity_summary: # critics/verifiers
  critical: 0
  major: 0
  minor: 0
```

**Note:** The `route_to_agent` and `route_to_flow` fields exist for audit trail completeness. Cleanup agents derive these from the agent's prose handoff, not from structured routing blocks.

---

## Verdict Domains (don't conflate)

| Domain                    | Values                                                                                                                                                                                                                                                               | Used by                   |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| Flow/Agent Status         | `VERIFIED \| UNVERIFIED \| PARTIAL \| CANNOT_PROCEED`                                                                                                                                                                                                                | Machine Summary, receipts |
| Repo Operator Status      | `COMPLETED \| COMPLETED_WITH_WARNING \| COMPLETED_WITH_ANOMALY \| FAILED \| CANNOT_PROCEED`                                                                                                                                                                          | Repo Operator Result      |
| Secrets Sanitizer Status  | `CLEAN \| FIXED \| BLOCKED`                                                                                                                                                                                                                                          | Gate Result               |
| Gate Merge Verdict        | `MERGE \| BOUNCE` (include reason when bouncing)                                                                                                                                                                                                                     | `merge_decision.md`       |
| Deploy Verdict (two-axis) | `deploy_action`: `COMPLETED \| SKIPPED \| FAILED`; `governance_enforcement`: `VERIFIED \| VERIFIED_RULESET \| UNVERIFIED_PERMS \| NOT_CONFIGURED \| UNKNOWN`; `deployment_verdict` (derived): `STABLE \| NOT_DEPLOYED \| GOVERNANCE_UNVERIFIABLE \| BLOCKED_BY_GATE` | `deployment_decision.md`  |
| Smoke Signal              | `STABLE \| INVESTIGATE \| ROLLBACK`                                                                                                                                                                                                                                  | `verification_report.md`  |

Note: `GOVERNANCE_UNVERIFIABLE` means deploy action succeeded but governance cannot be verified. This is distinct from `NOT_DEPLOYED` (deploy action failed).

---

## Receipt Schema (minimum fields)

```json
{
  "run_id": "<run-id>",
  "flow": "<flow-name>",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | RERUN | BOUNCE | FIX_ENV",
  "route_to_flow": null,
  "route_to_agent": null,
  "missing_required": [],
  "counts": {},
  "completed_at": "<ISO8601>"
}
```

Receipts are mechanical; counts are derived via `demoswarm` CLI (not estimated).

Receipt writers may include a `schema_version` field for compatibility (e.g., `build_receipt_v1`).

### Test count definitions

`xpassed` counts tests marked expected-to-fail (xfail) that actually passed. If unknown, keep as `null`.

---

## Handoff Contract (Primary Agent Communication)

The handoff is **the primary way agents communicate** with the orchestrator. All agent-to-orchestrator communication flows through prose handoffs, not structured YAML blocks.

This is how intelligent actors report to their PM: natural language that conveys intent, context, and reasoning.

### The Pattern

Every agent ends with a handoff that answers three questions:

1. **What was done?** — Summary of work completed
2. **What still needs to be done?** — Remaining work, blockers, open questions
3. **My recommendation** — Specific next step with reasoning

### Example Handoff

```markdown
## Handoff

**What I did:** Reviewed the implementation against REQ-001 through REQ-005.
Found two issues: the session timeout logic doesn't match the ADR (uses 30m instead of 15m),
and REQ-003 has no test coverage.

**What's left:** The timeout fix is mechanical. The missing tests need the test-author.

**Recommendation:** Route to **fixer** for the timeout issue (mechanical fix, no design input needed),
then route to **test-author** to add coverage for REQ-003. Re-run me after both are done to verify
the fixes landed correctly.
```

Note how the recommendation names specific agents and explains why each is appropriate.

### Rules

- **Always make a recommendation.** Even when uncertain, take a stance.
- **Name specific agents when you know them.** "Run test-author" is better than "run tests."
- **Explain your reasoning.** "Because X" helps the orchestrator override intelligently.
- **Alternatives are for real tradeoffs only.** Don't hedge unnecessarily.

---

## Handoff Recommendations

Agents only need to know their immediate neighbors, not the entire swarm. Each agent prompt includes a short list of agents it might reasonably hand off to.

### The Pattern

**In the agent prompt**, include a "Handoff Targets" section with 3-4 likely neighbors:

```markdown
## Handoff Targets

When you complete your work, recommend one of these agents to the orchestrator:

- **fixer**: Applies targeted fixes for issues you've identified
- **test-author**: Writes or updates tests when test coverage is needed
- **code-critic**: Reviews implementation when you want a second opinion on code quality

Tell the orchestrator which agent you recommend and why.
```

**In the agent's response**, make a specific recommendation with reasoning:

```markdown
## My Recommendation

Route to **fixer** - I found 3 MINOR issues that are mechanical fixes.
The fixer can address these without needing design input.
```

### Why This Works

- **Agents only know their neighbors, not the whole swarm.** Each agent is responsible for knowing a handful of agents it commonly routes to. No agent needs to understand all 50+ agents.
- **Recommendations are reasoned, not just "done."** The agent explains why this routing makes sense, giving the orchestrator context to override if needed.
- **Orchestrator gets context for routing decisions.** The recommendation plus reasoning lets the orchestrator make an informed choice.
- **No central routing table to maintain.** Routing knowledge is distributed across agent prompts. Adding a new agent only requires updating its direct neighbors.

### What This Replaces

| Old Pattern               | Problem                                     | New Pattern                           |
| ------------------------- | ------------------------------------------- | ------------------------------------- |
| Rigid routing rules       | Brittle, requires updates across the system | Agent recommends, orchestrator routes |
| Agents knowing all agents | Impossible to maintain, prompts bloat       | Agents know 3-4 neighbors             |
| Orchestrator guessing     | Orchestrator lacks context                  | Agent provides reasoning              |

### Guidelines for Neighbor Descriptions

Keep descriptions to **one line** with two parts:

1. **What the agent does** (verb phrase)
2. **When to route there** (condition)

**Good examples:**

```
- **fixer**: Applies targeted fixes when issues are mechanical and well-defined
- **test-author**: Writes or updates tests when test coverage gaps are identified
- **code-implementer**: Implements features when new code is needed beyond fixes
- **clarifier**: Queues questions when human input is needed to proceed
```

**Anti-patterns to avoid:**

```
# Too vague - doesn't say when to route
- **fixer**: Fixes things

# Too long - loses scanability
- **test-author**: This agent is responsible for writing comprehensive test suites
  including unit tests, integration tests, and end-to-end tests when the code
  reviewer or critic identifies gaps in test coverage
```

### Choosing Which Neighbors to Include

Include agents that handle:

1. **The happy path** - Where does work normally go next after you?
2. **Common issues** - What problems do you often discover that someone else should fix?
3. **Quality gates** - Who reviews your work before it moves forward?

Do not include agents for:

- Edge cases that rarely happen
- Agents many hops away in the flow
- Mechanical skills (use skill invocations instead)

### Example: Full Agent Handoff Section

```markdown
## Handoff Targets

When you complete your work, recommend one of these agents:

- **fixer**: Applies targeted fixes for MINOR/MAJOR issues that are mechanical
- **test-author**: Writes tests when you identify coverage gaps
- **code-implementer**: Implements new features when scope expands beyond fixes
- **clarifier**: Queues questions when requirements are ambiguous

## My Recommendation

Route to **test-author** - Implementation is complete and passes existing tests,
but I identified two untested edge cases in the error handling path. The
test-author should add coverage for:

1. Network timeout during auth refresh
2. Malformed token response from OAuth provider

After tests are added, route to code-critic for final review.
```

See [agent-philosophy.md](../explanation/agent-philosophy.md) for the broader philosophy on agent autonomy and communication, and [routing-table.md](routing-table.md) for the full routing reference.

### Graceful Outcomes

**Honest partial reports are successful outcomes.** A handoff that says "I completed 2/5 ACs, blocked on missing schema" is a verified success. A report saying "All 5 ACs complete (assuming schema exists)" is a high-risk failure.

The orchestrator routes on your signals. Hiding uncertainty behind false completion causes downstream failures.

**PARTIAL is a win.** If you:

- Made real progress
- Documented what's done and what's blocked
- Left the codebase in a runnable state

...then reporting partial completion with honest blockers is correct. The flow will rerun and pick up where you left off.

### Status Concepts (Natural Language)

Use these concepts in your handoff prose:

- **Complete / verified** — Work is done, evidence exists, no blockers
- **Incomplete / unverified** — Gaps exist, document what's missing
- **Blocked** — Cannot proceed without external input (human decision, missing access, etc.)
- **Mechanical failure** — IO/permissions/tooling broken; environment needs fixing

### Routing Intent

Express routing naturally:

- "Run X next" — You know the agent
- "This needs to go back to Plan" — Flow-level routing
- "The implementer should fix this" — Station-level hint
- "I need another pass after they fix the schema" — Rerun self
- "This is blocked until the user decides on auth approach" — Human escalation needed

---

## Agent Philosophy

Agents are **intelligent actors**, not mechanical extractors. They do real cognitive work.

### Core Principles

**Agents are like well-trained juniors reporting to a PM.** They:

- Investigate autonomously before asking for help
- Make reasonable assumptions and document them
- Report what they found, what they did, and what they recommend
- Communicate in natural language, not structured data formats

**Single responsibility per agent.** Each agent has one job:

- Critics critique (they never fix)
- Workers implement (they maintain the ledger)
- Cleanup agents audit (they write receipts)

**Positive prompting.** Agent prompts emphasize what to do, not what not to do. This produces more capable, less defensive behavior.

**Graceful outcomes.** An honest partial report is a successful outcome. The system routes on truth, not on completeness. Agents that admit uncertainty enable better decisions than agents that hide it.

### Research-First Autonomy

When agents encounter ambiguity, they follow this sequence:

1. **Investigate** — Search the codebase, read existing implementations
2. **Derive** — Use existing patterns to infer correct behavior
3. **Default** — Choose reversible defaults and document them
4. **Escalate** — Only flag as blocked if research failed AND no safe default exists

---

## Stable Marker Contracts

Stable markers enable mechanical counting without parsing prose. Agents emit these prefixes; cleanup agents count them via the `demoswarm` CLI.

**Key marker families:**

| Family       | Pattern                                           | Used in                                |
| ------------ | ------------------------------------------------- | -------------------------------------- |
| Requirements | `^### REQ-`, `^### NFR-`                          | `requirements.md`                      |
| Risks        | `^- RSK-[0-9]+ \[SEVERITY\]`                      | `early_risks.md`, `risk_assessment.md` |
| Questions    | `^- QID: OQ-<FLOW>-[0-9]{3}`                      | `open_questions.md`                    |
| Severity     | `^- \[CRITICAL\]`, `^- \[MAJOR\]`, `^- \[MINOR\]` | Critiques                              |
| Build        | `^- IMPL_FILE_*:`, `^- TEST_FILE_*:`              | `*_changes_summary.md`                 |
| Wisdom       | `^## Learning:`, `^### REG-[0-9]{3}:`             | `learnings.md`, `regression_report.md` |

See [stable-markers.md](stable-markers.md) for complete patterns, examples, and counting commands.

---

## Per-Flow Receipt Schemas

Each flow produces a receipt with flow-specific fields. All receipts share a common base:

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
  "completed_at": "<ISO8601>"
}
```

### signal_receipt.json

```json
{
  "counts": {
    "functional_requirements": null,
    "non_functional_requirements": null,
    "bdd_scenarios": null,
    "open_questions": null,
    "risks": { "critical": null, "high": null, "medium": null, "low": null }
  },
  "quality_gates": {
    "requirements_critic": null,
    "bdd_critic": null
  },
  "key_artifacts": [
    "requirements.md",
    "features/*.feature",
    "open_questions.md"
  ]
}
```

### plan_receipt.json

```json
{
  "counts": {
    "design_options": null,
    "subtasks_total": null,
    "open_questions": null,
    "contract_endpoints": null,
    "test_plan_entries": null
  },
  "quality_gates": {
    "design_critic": null,
    "policy_analyst": null
  },
  "decision_spine": {
    "status": null,
    "design_options": {
      "status": null,
      "recommendation": null,
      "confidence": null
    },
    "adr": { "status": null, "chosen_option": null, "drivers_total": null }
  },
  "key_artifacts": [
    "design_options.md",
    "adr.md",
    "design_validation.md",
    "work_plan.md",
    "test_plan.md"
  ]
}
```

### build_receipt.json

```json
{
  "schema_version": "build_receipt_v1",
  "counts": {
    "tests_written": null,
    "files_changed": null,
    "mutation_score": null,
    "open_questions": null
  },
  "tests": {
    "summary_source": "build/test_execution.md",
    "canonical_summary": null,
    "passed": null,
    "failed": null,
    "skipped": null,
    "xfailed": null,
    "xpassed": null,
    "metrics_binding": "test_execution:test-runner"
  },
  "critic_verdicts": {
    "test_critic": null,
    "code_critic": null
  },
  "quality_gates": {
    "test_critic": null,
    "code_critic": null,
    "self_reviewer": null
  },
  "key_artifacts": [
    "self_review.md",
    "test_changes_summary.md",
    "impl_changes_summary.md",
    "test_execution.md",
    "test_critique.md",
    "code_critique.md"
  ]
}
```

### gate_receipt.json

```json
{
  "merge_verdict": "MERGE | BOUNCE | null",
  "counts": {
    "receipt_checks_total": null,
    "receipt_checks_passed": null,
    "contract_violations": null,
    "security_findings": null,
    "policy_violations": null,
    "coverage_line_percent": null,
    "coverage_branch_percent": null
  },
  "quality_gates": {
    "merge_decider": null,
    "receipt_audit": null,
    "contract_compliance": null,
    "security_scan": null,
    "coverage_audit": null
  },
  "key_artifacts": [
    "merge_decision.md",
    "receipt_audit.md",
    "contract_compliance.md",
    "security_scan.md",
    "coverage_audit.md"
  ]
}
```

### deploy_receipt.json

```json
{
  "deployment_verdict": "STABLE | NOT_DEPLOYED | BLOCKED_BY_GATE | null",
  "gate_verdict": "MERGE | BOUNCE | null",
  "counts": {
    "failed_checks": null,
    "ci_checks_total": null,
    "deploy_events_total": null,
    "verification_checks_total": null
  },
  "signals": {
    "ci_signal": null,
    "deploy_signal": null,
    "not_deployed": null
  },
  "quality_gates": {
    "deploy_decider": null,
    "verification_report": null
  },
  "key_artifacts": [
    "deployment_decision.md",
    "deployment_log.md",
    "verification_report.md"
  ]
}
```

### wisdom_receipt.json

```json
{
  "counts": {
    "learnings_extracted": null,
    "feedback_actions_created": null,
    "regressions_found": null,
    "flows_completed": null,
    "followup_issue_drafts": null
  },
  "flow_summary": {
    "signal": null,
    "plan": null,
    "build": null,
    "gate": null,
    "deploy": null
  },
  "final_outcomes": {
    "merge_decision": null,
    "deployment_verdict": null
  },
  "run_complete": true,
  "key_artifacts": ["learnings.md", "feedback_actions.md"]
}
```

---

## Index Schema

`.runs/index.json`:

```json
{
  "version": 1,
  "runs": [
    {
      "run_id": "<run-id>",
      "canonical_key": "gh-456",
      "task_key": "<task-key>",
      "task_title": "<title>",
      "issue_number": 456,
      "pr_number": null,
      "updated_at": "<ISO8601>",
      "status": "VERIFIED",
      "last_flow": "build"
    }
  ]
}
```

---

## Run Meta Schema

`.runs/<run-id>/run_meta.json`:

```json
{
  "run_id": "<run-id>",
  "run_id_kind": "GH_ISSUE | LOCAL_ONLY | null",
  "issue_binding": "IMMEDIATE | DEFERRED | null",
  "issue_binding_deferred_reason": "gh_unauth | gh_unavailable | null",
  "canonical_key": "<gh-456 | pr-789 | null>",
  "aliases": ["<run-id>", "<gh-456>", "<branch-name>"],
  "task_key": "<ticket-id | branch-slug | null>",
  "task_title": "<short normalized title>",
  "github_repo": "<owner/repo | null>",
  "github_repo_expected": "<owner/repo | null>",
  "github_repo_actual_at_creation": "<owner/repo | null>",
  "github_ops_allowed": true,
  "repo_mismatch": false,
  "created_at": "<ISO8601>",
  "updated_at": "<ISO8601>",
  "iterations": 1,
  "flows_started": ["signal", "plan"],
  "source": "<branch:name | ticket:id | manual>",
  "issue_number": 456,
  "issue_url": "<url | null>",
  "issue_title": "<string | null>",
  "pr_number": null,
  "supersedes": "<previous-run-id | null>",
  "related_runs": []
}
```

---

## Sources of Truth

| What                                | Where                           |
| ----------------------------------- | ------------------------------- |
| Flow sequences, gating              | `.claude/commands/flow-*.md`    |
| Agent behavior, outputs             | `.claude/agents/*.md`           |
| Shared invariants, canonical blocks | `CLAUDE.md`                     |
| Drift validation                    | `.claude/scripts/pack-check.sh` |

---

## See also

- [CLAUDE.md](../../CLAUDE.md) — Pack reference
- [run-state.md](run-state.md) — Run identity and state schemas
- [stable-markers.md](stable-markers.md) — Marker prefixes for counting
- [trust-model.md](trust-model.md) — Evidence hierarchy
- [glossary.md](glossary.md) — Term definitions
