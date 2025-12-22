# Demo Swarm Cheatsheet

> One-screen reference for using the DemoSwarm **pack-only** repo.

---

## Quick Start

```bash
# Copy pack into your repo
cp -r /path/to/demo-swarm/.claude /path/to/your-repo/

# From your repo root, open in Claude Code and run:
# (run-id is optional; if omitted, run-prep derives one)
/flow-1-signal "Your feature idea"
```

---

## Flow Commands

| Command | Purpose |
|---------|---------|
| `/flow-1-signal [run-id] "description"` | Messy input → requirements, BDD, risks, Signal receipt |
| `/flow-2-plan [run-id]` | Spec → ADR, contracts, observability, plans |
| `/flow-3-build [run-id] [subtask-id]` | Implement → tests/code/docs, Draft PR, Build receipt |
| `/flow-4-review [run-id]` | Drain worklist → harvest PR feedback, fix items, Review receipt |
| `/flow-5-gate [run-id]` | Verify → receipts/contracts/security/coverage/policy, merge decision |
| `/flow-6-deploy [run-id]` | Release → merge/tag (if approved), verify, Deploy receipt |
| `/flow-7-wisdom [run-id]` | Learn → artifact audit, regressions, timeline, feedback drafts |
| `/customize-pack` | Adapt pack to your stack/repo layout |

---

## With Explicit Run ID

```text
/flow-1-signal feat-auth "Add OAuth2 login"
```

Rule of thumb:

* first token = optional `run-id`
* remaining text = the signal

---

## Artifact Locations

All durable artifacts live under:

* `.runs/<run-id>/<flow>/`

| Flow | Key Artifacts (examples) |
|------|--------------------------|
| Signal | `requirements.md`, `features/*.feature`, `verification_notes.md`, `signal_receipt.json` |
| Plan | `adr.md`, `api_contracts.yaml`, `schema.md`, `observability_spec.md`, `plan_receipt.json` |
| Build | `test_changes_summary.md`, `impl_changes_summary.md`, `self_review.md`, `build_receipt.json` |
| Review | `pr_feedback.md`, `review_worklist.md`, `review_actions.md`, `review_receipt.json` |
| Gate | `merge_decision.md`, `contract_compliance.md`, `coverage_audit.md`, `gate_receipt.json` |
| Deploy | `deployment_log.md`, `verification_report.md`, `deployment_decision.md`, `deploy_receipt.json` |
| Wisdom | `artifact_audit.md`, `regression_report.md`, `flow_history.json`, `feedback_actions.md`, `wisdom_receipt.json` |

---

## Skills (mechanical helpers)

| Skill | Purpose |
|-------|---------|
| `test-runner` | Run tests and write a test report artifact (project-defined) |
| `auto-linter` | Run linters/formatters (project-defined) |
| `policy-runner` | Policy-as-code checks |
| `runs-derive` | Read-only `.runs/` operations (counts, Machine Summary extraction, receipt reading) |
| `runs-index` | Write `.runs/index.json` updates (status, last_flow, updated_at) |
| `openq-tools` | Open questions register (QID generation, append entries) |
| `secrets-tools` | Secrets scanning/redaction for publish gates |

---

## Customization (what you actually edit)

| What | Where |
|------|-------|
| Test command | `.claude/skills/test-runner/SKILL.md` |
| Lint command | `.claude/skills/auto-linter/SKILL.md` |
| Repo layout + intended change surface | `/customize-pack` + `subtask_context_manifest.json` (Flow 3) |
| Flow contracts / governance | `.claude/commands/flow-*.md` + `pack-check.sh` |

---

## Validation

```bash
# Canonical pack validation
bash .claude/scripts/pack-check.sh
```

If you have extra repo scripts, run them as optional checks (they must not be required for pack portability).

---

## Key Files

| File | Purpose |
|------|---------|
| `README.md` | Quick start + what this repo is |
| `CLAUDE.md` | Pack reference + contracts |
| `CHEATSHEET.md` | This file |
| `docs/README.md` | Documentation index (Diátaxis) |
| `docs/how-to/customize-pack.md` | Adapting the pack to your stack |
| `docs/tutorials/validation-run.md` | Validation run guide |
| `CHANGELOG.md` | Pack release notes |

---

## Agent Role Families (by color)

| Color | Family | Examples |
|-------|--------|----------|
| Yellow | Shaping | `signal-normalizer`, `problem-framer` |
| Purple | Design/Spec | `requirements-author`, `adr-author`, `interface-designer` |
| Green | Implementation | `code-implementer`, `test-author`, `doc-writer` |
| Red | Critic | `code-critic`, `test-critic`, `design-critic` |
| Blue | Verification | `coverage-enforcer`, `receipt-checker`, `contract-enforcer` |
| Orange | Analytics | `risk-analyst`, `regression-analyst`, `flow-historian` |
| Cyan | Infra / Git | `repo-operator`, `run-prep`, `signal-run-prep` |
| Pink | Reporter | `gh-reporter`, `gh-issue-manager` |

---

## Status Values (pack standard)

| Status | Meaning |
|--------|---------|
| `VERIFIED` | Adequate for purpose; blockers empty |
| `UNVERIFIED` | Gaps/concerns remain; artifacts still written |
| `CANNOT_PROCEED` | Mechanical failure only (IO/perms/tooling) |

Notes:

* "BLOCKED" is **not** a pack-level status. Some agents (e.g., repo-operator) have their own internal status domain.

---

## External Ops Gate (push + GitHub ops)

Before **any external side effects** (push, issue create, issue comment), both must be true:

| Gate | Source | Meaning |
|------|--------|---------|
| `safe_to_publish: true` | `secrets-sanitizer` | publish surface is safe |
| `proceed_to_github_ops: true` | `repo-operator` | allowlist interlock clean; checkpoint/build op succeeded |

If either gate fails:

* write local artifacts
* skip external ops
* finish `UNVERIFIED` (unless mechanical failure)

Safe-bail:

* `repo-operator checkpoint_mode: local_only` → commits locally, never pushes, forces `proceed_to_github_ops: false`

---

## Tips

* Use a sandbox repo first; keep main clean.
* `.runs/` is part of the audit trail and is usually committed.
* GitHub integration is optional (flows still produce local artifacts + receipts).
* Critics never fix; implementers apply fixes; cleanup agents seal receipts.
