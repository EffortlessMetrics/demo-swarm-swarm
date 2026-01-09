# Contracts Reference

> Control-plane blocks, enums, and schemas.

This document indexes canonical contracts. The source of truth is `CLAUDE.md` and `.claude/commands/*`.

---

## Canonical Enums

### Status axis (most agents)

```
VERIFIED | UNVERIFIED | CANNOT_PROCEED
```

| Value | Meaning |
|-------|---------|
| `VERIFIED` | Adequate for purpose; blockers empty |
| `UNVERIFIED` | Gaps/concerns remain; artifacts still written |
| `CANNOT_PROCEED` | Mechanical failure only (IO/perms/tooling) |

**Rule:** `CANNOT_PROCEED` requires `missing_required` to be non-empty.

### Recommended action (routing)

```
PROCEED | RERUN | BOUNCE | FIX_ENV
```

| Value | Meaning |
|-------|---------|
| `PROCEED` | Default: continue even when open questions exist (capture blockers/assumptions) |
| `RERUN` | Same station again with a deterministic improvement expected |
| `BOUNCE` | Route to a specific flow/agent for an actionable fix |
| `FIX_ENV` | Environment/tooling issue (paired with `status: CANNOT_PROCEED`) |

### Route fields

```yaml
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | null
route_to_agent: <agent-name> | null
```

---

## Control-Plane Blocks

These blocks are returned by agents and used for routing. Orchestrators route on these; they don't reread files.

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

## Machine Summary (critics/verifiers)

```yaml
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_agent: <agent-name | null>
route_to_flow: <1|2|3|4|5|6 | null>

blockers: []
missing_required: []
concerns: []

can_further_iteration_help: yes | no   # critics only

severity_summary:                      # critics/verifiers
  critical: 0
  major: 0
  minor: 0
```

---

## Verdict Domains (don't conflate)

| Domain | Values | Used by |
|--------|--------|---------|
| Flow/Agent Status | `VERIFIED \| UNVERIFIED \| PARTIAL \| CANNOT_PROCEED` | Machine Summary, receipts |
| Repo Operator Status | `COMPLETED \| COMPLETED_WITH_WARNING \| COMPLETED_WITH_ANOMALY \| FAILED \| CANNOT_PROCEED` | Repo Operator Result |
| Secrets Sanitizer Status | `CLEAN \| FIXED \| BLOCKED` | Gate Result |
| Gate Merge Verdict | `MERGE \| BOUNCE` (include reason when bouncing) | `merge_decision.md` |
| Deploy Verdict (two-axis) | `deploy_action`: `COMPLETED \| SKIPPED \| FAILED`; `governance_enforcement`: `VERIFIED \| VERIFIED_RULESET \| UNVERIFIED_PERMS \| NOT_CONFIGURED \| UNKNOWN`; `deployment_verdict` (derived): `STABLE \| NOT_DEPLOYED \| GOVERNANCE_UNVERIFIABLE \| BLOCKED_BY_GATE` | `deployment_decision.md` |
| Smoke Signal | `STABLE \| INVESTIGATE \| ROLLBACK` | `verification_report.md` |

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

## Handoff Contract

Agents communicate routing through natural language handoffs, not structured YAML blocks.

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

**Recommendation:** Run test-author to add coverage for REQ-003, then re-run me to verify the fixes.
The timeout issue is minor enough that code-implementer can fix it in the same pass.
```

### Rules

- **Always make a recommendation.** Even when uncertain, take a stance.
- **Name specific agents when you know them.** "Run test-author" is better than "run tests."
- **Explain your reasoning.** "Because X" helps the orchestrator override intelligently.
- **Alternatives are for real tradeoffs only.** Don't hedge unnecessarily.

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

## Stable Marker Contracts

Stable markers enable mechanical counting without parsing prose. Agents emit these prefixes; cleanup agents count them via the `demoswarm` CLI.

**Key marker families:**

| Family | Pattern | Used in |
|--------|---------|---------|
| Requirements | `^### REQ-`, `^### NFR-` | `requirements.md` |
| Risks | `^- RSK-[0-9]+ \[SEVERITY\]` | `early_risks.md`, `risk_assessment.md` |
| Questions | `^- QID: OQ-<FLOW>-[0-9]{3}` | `open_questions.md` |
| Severity | `^- \[CRITICAL\]`, `^- \[MAJOR\]`, `^- \[MINOR\]` | Critiques |
| Build | `^- IMPL_FILE_*:`, `^- TEST_FILE_*:` | `*_changes_summary.md` |
| Wisdom | `^## Learning:`, `^### REG-[0-9]{3}:` | `learnings.md`, `regression_report.md` |

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
  "key_artifacts": ["requirements.md", "features/*.feature", "open_questions.md"]
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
    "design_options": { "status": null, "recommendation": null, "confidence": null },
    "adr": { "status": null, "chosen_option": null, "drivers_total": null }
  },
  "key_artifacts": ["design_options.md", "adr.md", "design_validation.md", "work_plan.md", "test_plan.md"]
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
  "key_artifacts": ["merge_decision.md", "receipt_audit.md", "contract_compliance.md", "security_scan.md", "coverage_audit.md"]
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
  "key_artifacts": ["deployment_decision.md", "deployment_log.md", "verification_report.md"]
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
    "signal": null, "plan": null, "build": null, "gate": null, "deploy": null
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

| What | Where |
|------|-------|
| Flow sequences, gating | `.claude/commands/flow-*.md` |
| Agent behavior, outputs | `.claude/agents/*.md` |
| Shared invariants, canonical blocks | `CLAUDE.md` |
| Drift validation | `.claude/scripts/pack-check.sh` |

---

## See also

- [CLAUDE.md](../../CLAUDE.md) — Pack reference
- [run-state.md](run-state.md) — Run identity and state schemas
- [stable-markers.md](stable-markers.md) — Marker prefixes for counting
- [trust-model.md](trust-model.md) — Evidence hierarchy
- [glossary.md](glossary.md) — Term definitions
