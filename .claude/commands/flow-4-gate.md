---
description: Run Flow 4 (Code → Artifact): verify receipts, contracts, security, policies; recommend merge or bounce.
---

# Flow 4: Code → Artifact (Gate)

You are orchestrating Flow 4 of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/gate/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/gate/` exists.

#### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Verify build receipts exist and are complete
- Check API/schema contracts
- Scan security and coverage
- Enforce policies
- Decide: MERGE / BOUNCE / ESCALATE
- **Optional, single-lane fix-forward** for deterministic mechanical drift (fmt/import order/whitespace) when `gate-fixer` says it is safe and resealable

## Before You Begin (Required)

### Two State Machines

Flow 4 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Parallel checks (contracts/security/coverage) are ONE todo.

2. Mirror the same list into `.runs/<run-id>/gate/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- Establish run infrastructure (run-prep)
- Verify receipts (receipt-checker first; route on Result)
- Check contracts + security + coverage (parallel)
- Mechanical issues report (gate-fixer)
- Fix-forward lane (conditional; runner-bounded)
- Risk assessment (risk-analyst)
- Policy compliance (policy-analyst)
- Merge decision (merge-decider)
- Finalize receipt (gate-cleanup)
- Sanitize secrets (secrets-sanitizer)
- Checkpoint commit (repo-operator)
- Update issue board (gh-issue-manager; gated)
- Report gate verdict (gh-reporter; gated)
- Update flow_plan.md summary
```

### On Rerun

If running `/flow-4-gate` on an existing run-id:
- Read `.runs/<run-id>/gate/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

If you encounter missing receipts or unclear state, **document it and continue with available information**. Gate agents should note gaps in their reports rather than blocking.

## Subagents to use

**Infrastructure (Step 0)**:
- **run-prep** -- MUST be called first to establish the run directory and `.runs/<run-id>/gate/`

Domain agents (Flow 4 specific):
- receipt-checker
- contract-enforcer
- security-scanner
- coverage-enforcer
- gate-fixer (reports mechanical issues; no repo mutations)
- merge-decider

Cross-cutting agents:
- risk-analyst
- policy-analyst

Fix-forward lane (conditional; runner-bounded):
- lint-executor (apply deterministic format/lint)
- test-executor (verify tests)
- build-cleanup (reseal Build receipt after code changes)
- repo-operator (stage/commit/push; owns git side effects)

Cleanup + Reporting (End of Flow):
- gate-cleanup -- writes gate_receipt.json, updates index.json status
- secrets-sanitizer -- publish gate
- repo-operator -- checkpoint commit (gated on secrets-sanitizer result); writes git_status.md if anomaly
- gh-issue-manager -- updates issue body status board
- gh-reporter -- posts gate verdict to issue

## Upstream Inputs

Read from `.runs/<run-id>/build/` (if available):
- build receipt and supporting critiques (tests, code, self-review)

If these files are not visible locally but may exist in committed state, do **not** block Gate. Proceed and let `receipt-checker` pull evidence from the committed snapshot; workspace visibility alone is not a missing-artifact signal.

**If upstream artifacts are missing**: Flow 4 can start without Flows 1-3. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue. This enables flexibility for gate-only checks.

## Artifact Outputs

| Artifact | Producer | Description |
|----------|----------|-------------|
| `flow_plan.md` | Orchestrator | Flow progress tracking |
| `receipt_audit.md` | receipt-checker | Build receipt verification |
| `contract_compliance.md` | contract-enforcer | API contract check results |
| `security_scan.md` | security-scanner | Security scan findings |
| `coverage_audit.md` | coverage-enforcer | Coverage threshold check |
| `gate_fix_summary.md` | gate-fixer | Mechanical issues report (no fixes) + fix-forward plan |
| `fix_forward_report.md` | fix-forward routine (conditional) | What ran, files changed, test summary, commit SHA |
| `risk_assessment.md` | risk-analyst | Risk analysis |
| `policy_analysis.md` | policy-analyst | Policy compliance check |
| `merge_decision.md` | merge-decider | MERGE / BOUNCE / ESCALATE decision |
| `cleanup_report.md` | gate-cleanup | Cleanup summary |
| `gate_receipt.json` | gate-cleanup | Machine-readable receipt |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings |
| `secrets_status.json` | secrets-sanitizer | Gate status (audit record) |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | Issue operation status |
| `github_report.md` | gh-reporter | Local copy of GitHub post |
| `gh_report_status.md` | gh-reporter | GitHub posting status |

All artifacts live under `.runs/<run-id>/gate/`.

**Fix-forward contract:** `gate_fix_summary.md` must contain the `## Fix-forward Plan (machine readable)` block (`PACK-CONTRACT: FIX_FORWARD_PLAN_V1`). The optional `fix_forward_report.md` records what the fix-forward routine actually ran (commands, touched files, test summary, commit SHA).

## Orchestration outline

### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/gate/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "gate" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/gate/`.

### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

**Do not** read `.runs/` artifacts before run-prep. After run-prep, call `receipt-checker` first and route on its Result block before running contracts/security/coverage.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/gate/flow_plan.md`:

```markdown
# Flow 4: Gate for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] receipt-checker (verify receipts first; route on Result)
- [ ] contract-enforcer / security-scanner / coverage-enforcer (parallel)
- [ ] gate-fixer (mechanical issues report)
- [ ] fix-forward lane (conditional: lint-executor apply → test-executor verify → build-cleanup reseal → repo-operator commit/push → rerun receipt-checker + gate-fixer)
- [ ] risk-analyst (risk assessment)
- [ ] policy-analyst (policy compliance)
- [ ] merge-decider (decide: MERGE/BOUNCE/ESCALATE)
- [ ] gate-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

### Step 2: Verify receipts
- `receipt-checker` -> `.runs/<run-id>/gate/receipt_audit.md`
- Run this before contracts/security/coverage; route on its Result block.

### Step 3: Check contracts (can run in parallel with security/coverage)
- `contract-enforcer` -> `.runs/<run-id>/gate/contract_compliance.md`

### Step 4: Security scan (can run in parallel with contracts/coverage)
- `security-scanner` -> `.runs/<run-id>/gate/security_scan.md`

### Step 5: Coverage (can run in parallel with contracts/security)
- `coverage-enforcer` -> `.runs/<run-id>/gate/coverage_audit.md`

### Step 6: Mechanical issues report (after verification agents complete)
- `gate-fixer` -> `.runs/<run-id>/gate/gate_fix_summary.md` (recommendations only; **no repo mutations in Gate**)
- Identifies lint, format, and doc issues that would be fixed in Build
- Does NOT apply fixes—Gate has no commit governance

### Step 7: Fix-forward lane (conditional; runner-bounded)
- Read the `FIX_FORWARD_PLAN_V1` block from `gate_fix_summary.md`.
- If `fix_forward_eligible: false` → skip fix-forward; proceed to risk/policy.
- If `fix_forward_eligible: true` → **execute the Fix-forward Plan** (as emitted by gate-fixer) using existing agents (lint-executor, test-executor, build-cleanup, repo-operator, secrets-sanitizer); do not re-diagnose or rescan beyond the plan.
- After executing the plan (success or failure), re-run only:
  - `receipt-checker` (attestation coherence after reseal)
  - `gate-fixer` (confirm mechanical blockers are resolved; still report-only)
- If the plan execution fails or remains unclean, proceed with the remaining Gate stations and expect merge-decider to bounce to Flow 3.

### Step 8: Risk assessment
- `risk-analyst` -> `.runs/<run-id>/gate/risk_assessment.md`

### Step 9: Policy compliance
- `policy-analyst` -> `.runs/<run-id>/gate/policy_analysis.md`

### Step 10: Merge decision
- `merge-decider` -> `.runs/<run-id>/gate/merge_decision.md` (MERGE/BOUNCE/ESCALATE)

### Step 11: Finalize and Write Receipt
- `gate-cleanup` -> `.runs/<run-id>/gate/gate_receipt.json`, `.runs/<run-id>/gate/cleanup_report.md`
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Updates `.runs/index.json` with status, last_flow, updated_at

### Step 12: Sanitize Secrets (Publish Gate)
- `secrets-sanitizer` -> `.runs/<run-id>/gate/secrets_scan.md`, `.runs/<run-id>/gate/secrets_status.json`
- Scans .runs/ artifacts before GitHub posting
- Fixes what it can (redacts artifacts)

**Status vs flags:** `status` is descriptive (CLEAN/FIXED/BLOCKED_PUBLISH).
`safe_to_commit` / `safe_to_publish` are authoritative. Route on Gate Result booleans, not status.

**Audit-only file:** `secrets_status.json` is audit-only; optional last-mile verification is allowed, but routing uses the Gate Result block.

**Control plane:** `secrets-sanitizer` returns a Gate Result block for orchestrator routing; `secrets_status.json` is the durable audit record. Orchestrators route on the returned block, not by re-reading the file.

**Gate Result block (returned by secrets-sanitizer):**

<!-- PACK-CONTRACT: GATE_RESULT_V1 START -->
```
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
<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->

**Gating logic (from Gate Result):**
- Proceed only if `safe_to_publish: true`
- If `needs_upstream_fix: true` → **BOUNCE** (typically to Flow 3) with pointer to `secrets_scan.md`
- If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention

### Step 12b: Reseal If Modified (Conditional Loop)

If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(gate-cleanup → secrets-sanitizer)` until either:
- the sanitizer reports `modified_files: false`, or
- the sanitizer indicates no reasonable path to fixing (non-convergent).

If reseal cannot make progress (sanitizer signals no reasonable path):
- Append an evidence note to `secrets_scan.md`:
  - "modified_files remained true; sanitizer reports no viable path to fix; stopping to prevent receipt drift."
- If Gate Result `safe_to_commit: true`: call `repo-operator` with `checkpoint_mode: local_only`
  - it must return `proceed_to_github_ops: false`
- Skip **all** GitHub ops (issue-manager / reporter).
- Flow outcome: `status: UNVERIFIED`, `recommended_action: ESCALATE`
  - If Gate Result `needs_upstream_fix: true`, use `recommended_action: BOUNCE` and the provided `route_to_*`.

**Note:** `checkpoint_mode: local_only` is a named parameter to `repo-operator` that mechanically enforces `proceed_to_github_ops: false` regardless of `safe_to_publish`. This ensures safe-bail cannot accidentally push.

### Step 12c: Checkpoint Commit

- `repo-operator` -> `.runs/<run-id>/gate/git_status.md` (if anomaly detected)

Checkpoint the audit trail **before** any GitHub operations.

**Allowlist for Flow 4:**
- `.runs/<run-id>/gate/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

**Call `repo-operator`** with `checkpoint_mode: normal` (default). The agent:
1. Resets staging and stages only the allowlist (not `git add .`)
2. Enforces the allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/gate/git_status.md` if anomaly detected
4. Handles no-op (nothing staged) gracefully—no empty commits

**Control plane:** `repo-operator` returns a Repo Operator Result block:
```
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
anomaly_paths: []
```
**Note:** `commit_sha` is always populated (current HEAD on no-op), never null.

Orchestrators route on this block, not by re-reading `git_status.md`.

**Safe-bail override:** If this checkpoint was invoked due to safe-bail (Step 12b), `repo-operator` must set `proceed_to_github_ops: false` even if `safe_to_publish: true`.

**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
- If `safe_to_commit: false` (from Gate Result): `repo-operator` skips commit entirely
- If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`
- If `safe_to_publish: true` and no anomaly: `repo-operator` commits and pushes, returns `proceed_to_github_ops: true`
- If `safe_to_publish: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → **UNVERIFIED**; commit locally, skip push

### Step 13: Update Issue Board

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed.

- `gh-issue-manager` -> updates issue body status board from receipt
- **Creates GitHub issue if none exists** (allowed in any flow; includes "Signal pending" banner if created from Flow 4)

**Route-and-fix behavior:**
- If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) (already handled in 12c; should not reach here)
- If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure; stop)
- If `gh` CLI unauthenticated/unavailable → **SKIPPED** with evidence (not blocked)
- If `proceed_to_github_ops: false` → **SKIPPED**; anomaly documented in `git_status.md`

### Step 14: Report Gate Verdict

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed.

- `gh-reporter` -> post verdict **to the GitHub issue** (not PR)
- Writes `.runs/<run-id>/gate/github_report.md` locally as record
- **Issue-first (hard):** All flow logs go to the issue, even if a PR exists. PRs are for PR-review dynamics only.

**Route-and-fix behavior:**
- If `safe_to_publish: false` → **SKIPPED**; write local report only
- If `proceed_to_github_ops: false` → **SKIPPED**; anomaly documented in `git_status.md`
- If `gh` CLI unauthenticated → **SKIPPED** (not blocked)

### Step 15: Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Merge Decision**: MERGE | BOUNCE | ESCALATE
- **Blockers**: <list if any>
- **Next Flow**: `/flow-5-deploy` (if MERGE) or bounce target

## Human Review Checklist

Before proceeding:
- [ ] `.runs/<run-id>/gate/merge_decision.md` - Is the decision correct?
- [ ] `.runs/<run-id>/gate/security_scan.md` - Are security findings acceptable?
- [ ] `.runs/<run-id>/gate/policy_analysis.md` - Are policy concerns addressed?
```

## Bounce Semantics

Gate-fixer **remains report-only**. It emits the fix-forward plan; the **fix-forward lane** applies deterministic hygiene once (fmt/import order/docs) and reseals before merge-decision. Formatting/import-only drift should be fixed-forward when `fix_forward_eligible: true`; bounce only if ineligible or the fix-forward attempt failed.

**BOUNCE to Build (Flow 3)**:
- Logic errors
- Test failures
- API contract violations
- Security vulnerabilities
- Coverage below threshold
- Mechanical drift that is **not** eligible for fix-forward or failed within the runner-bounded fix-forward lane

**BOUNCE to Plan (Flow 2)**:
- Design flaws
- Architecture issues
- Missing requirements

## Status States

Agents set status in their output artifacts:

- **VERIFIED**: `blockers` empty, `missing_required` empty, and check passed; here's why. Set `recommended_action: PROCEED`.
- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR check has concerns; here are the issues. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.

`merge-decider` synthesizes all statuses into a merge decision.

## Merge Decision States

`merge-decider` outputs one of:

- **MERGE**: All checks pass or concerns are acceptable; ready to deploy.
- **BOUNCE**: Issues found; specifies target flow (Build or Plan) and reasons.
- **ESCALATE**: Needs human judgment; explains why automated decision isn't sufficient.

---

## Orchestrator Kickoff

### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator: ensure run/<run-id> branch
- [ ] receipt-checker
- [ ] contract-enforcer / security-scanner / coverage-enforcer (parallel)
- [ ] gate-fixer (report + fix-forward plan)
- [ ] fix-forward lane (if eligible: lint-executor apply → test-executor verify → build-cleanup reseal → repo-operator commit/push → rerun receipt-checker + gate-fixer)
- [ ] risk-analyst
- [ ] policy-analyst
- [ ] merge-decider
- [ ] gate-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] reseal cycle (gate-cleanup ↔ secrets-sanitizer) if modified_files
- [ ] repo-operator checkpoint (checkpoint mode; capture Repo Operator Result)
- [ ] gh-issue-manager (only if safe_to_publish AND proceed_to_github_ops)
- [ ] gh-reporter (only if safe_to_publish AND proceed_to_github_ops)
- [ ] finalize flow_plan.md summary

### Agent call order
1) run-prep
2) repo-operator (ensure run branch)
3) receipt-checker
4) contract-enforcer + security-scanner + coverage-enforcer (parallel)
5) gate-fixer (report + fix-forward plan)
6) fix-forward lane (if `fix_forward_eligible: true`: lint-executor apply → test-executor verify → build-cleanup reseal → repo-operator commit/push; then rerun receipt-checker + gate-fixer)
7) risk-analyst
8) policy-analyst
9) merge-decider
10) gate-cleanup
11) secrets-sanitizer (read Gate Result)
12) (reseal cycle if needed)
13) repo-operator (checkpoint; read Repo Operator Result)
14) gh-issue-manager (if gates true and gh available; otherwise SKIP with evidence)
15) gh-reporter (if gates true and gh available; otherwise SKIP with evidence)
