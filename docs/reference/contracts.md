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
PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV
```

| Value | Meaning |
|-------|---------|
| `PROCEED` | Continue to next step/flow |
| `RERUN` | Same flow again |
| `BOUNCE` | Route to different flow/agent |
| `ESCALATE` | Human decision needed |
| `FIX_ENV` | Environment/tooling issue |

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
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
needs_upstream_fix: true | false
recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | null
route_to_agent: <agent-name> | null
```

**Canonical location:** `CLAUDE.md` (with PACK-CONTRACT markers)

### Repo Operator Result

```yaml
## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
anomaly_paths: []
```

**Canonical location:** `CLAUDE.md` (with PACK-CONTRACT markers)

---

## Machine Summary (critics/verifiers)

```yaml
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV
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
| Flow/Agent Status | `VERIFIED \| UNVERIFIED \| CANNOT_PROCEED` | Machine Summary, receipts |
| Repo Operator Status | `COMPLETED \| COMPLETED_WITH_ANOMALY \| FAILED \| CANNOT_PROCEED` | Repo Operator Result |
| Secrets Sanitizer Status | `CLEAN \| FIXED \| BLOCKED_PUBLISH` | Gate Result |
| Gate Merge Verdict | `MERGE \| BOUNCE \| ESCALATE` | `merge_decision.md` |
| Deploy Verdict | `STABLE \| NOT_DEPLOYED \| BLOCKED_BY_GATE` | `deployment_decision.md` |
| Smoke Signal | `STABLE \| INVESTIGATE \| ROLLBACK` | `verification_report.md` |

---

## Receipt Schema (minimum fields)

```json
{
  "run_id": "<run-id>",
  "flow": "<flow-name>",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV",
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

## Stable Marker Contracts

Stable markers enable mechanical counting without parsing prose. Agents emit these prefixes; cleanup agents count them.

### Severity markers

Used in critiques and audits:

| Pattern | Regex | Example |
|---------|-------|---------|
| Critical finding | `^- \[CRITICAL\]` | `- [CRITICAL] Missing auth check` |
| Major finding | `^- \[MAJOR\]` | `- [MAJOR] No input validation` |
| Minor finding | `^- \[MINOR\]` | `- [MINOR] Style inconsistency` |

### Requirement markers

Used in `requirements.md`:

| Pattern | Regex | Example |
|---------|-------|---------|
| Functional requirement | `^### REQ-` | `### REQ-001` |
| Non-functional requirement | `^### NFR-` | `### NFR-SECURITY-001` |

### Risk markers

Used in `early_risks.md`, `risk_assessment.md`:

| Pattern | Regex | Example |
|---------|-------|---------|
| Risk item | `^- RSK-[0-9]+ \[(CRITICAL\|HIGH\|MEDIUM\|LOW)\]` | `- RSK-001 [HIGH] Data exposure risk` |

### Open question markers

Used in `open_questions.md`:

| Pattern | Regex | Example |
|---------|-------|---------|
| Question ID | `^- QID: OQ-.*-[0-9]{3}` | `- QID: OQ-SIG-001` |

Flow prefixes: `SIG`, `PLAN`, `BUILD`, `GAT`, `DEP`, `WIS`

### Test markers

Used in `test_changes_summary.md`:

| Pattern | Regex | Example |
|---------|-------|---------|
| Test file changed | `^- TEST_FILE_CHANGED:` | `- TEST_FILE_CHANGED: tests/auth_test.py` |
| Test file added | `^- TEST_FILE_ADDED:` | `- TEST_FILE_ADDED: tests/test_auth_flow.py` |

### File markers

Used in `impl_changes_summary.md`:

| Pattern | Regex | Example |
|---------|-------|---------|
| File changed | `^- IMPL_FILE_CHANGED:` | `- IMPL_FILE_CHANGED: src/auth.rs` |
| File added | `^- IMPL_FILE_ADDED:` | `- IMPL_FILE_ADDED: src/new_module.rs` |

### Inventory markers

Used in various flow artifacts:

| Pattern | Regex | Example |
|---------|-------|---------|
| Issue draft | `^- ISSUE_DRAFT:` | `- ISSUE_DRAFT: Add rate limiting` |
| Suggestion | `^- SUGGESTION:` | `- SUGGESTION: Consider caching` |
| Issue (feedback) | `^- ISSUE:` | `- ISSUE: Create follow-up ticket` |

### Wisdom markers

Used in Flow 6 artifacts:

| Pattern | Regex | Example |
|---------|-------|---------|
| Learning | `^## Learning: ` | `## Learning: Caching improves latency` |
| Regression | `^### REG-[0-9]{3}:` | `### REG-001: Performance degradation` |

See also: [stable-markers.md](stable-markers.md) for complete reference.

---

## Per-Flow Receipt Schemas

Each flow produces a receipt with flow-specific fields. All receipts share a common base:

```json
{
  "run_id": "<run-id>",
  "flow": "<flow-name>",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV",
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
  "merge_verdict": "MERGE | BOUNCE | ESCALATE | null",
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
  "gate_verdict": "MERGE | BOUNCE | ESCALATE | null",
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
  "canonical_key": "<gh-456 | pr-789 | null>",
  "aliases": ["<run-id>", "<gh-456>", "<branch-name>"],
  "task_key": "<ticket-id | branch-slug | null>",
  "task_title": "<short normalized title>",
  "created_at": "<ISO8601>",
  "updated_at": "<ISO8601>",
  "iterations": 1,
  "flows_started": ["signal", "plan"],
  "source": "<branch:name | ticket:id | manual>",
  "issue_number": 456,
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

- [CLAUDE.md](../../CLAUDE.md) — Canonical blocks with PACK-CONTRACT markers
- [stable-markers.md](stable-markers.md) — Marker prefixes for counting
- [glossary.md](glossary.md) — Term definitions
