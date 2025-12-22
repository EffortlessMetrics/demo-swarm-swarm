---
description: Run Flow 6 (Artifact -> Prod): execute GitHub-native deployment, monitor CI, verify, create audit trail.
---

# Flow 6: Artifact -> Prod (Deploy)

You are orchestrating Flow 6 of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/deploy/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/deploy/` exists.

#### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

Move an approved artifact from "ready to merge" to "deployed"—execute deployment, verify health, create audit trail.

**Flow 6 is always callable.** Its behavior depends on Gate's decision:
- If Gate said MERGE: merge, verify, report.
- If Gate said BOUNCE (including NEEDS_HUMAN_REVIEW): don't merge, write receipts explaining why.

## Before You Begin (Required)

### Two State Machines

Flow 6 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.

2. Mirror the same list into `.runs/<run-id>/deploy/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- repo-operator (merge + tag + release; only if Gate verdict MERGE)
- deploy-monitor (monitor CI; only if Gate verdict MERGE)
- smoke-verifier (smoke tests; only if Gate verdict MERGE)
- deploy-decider (deployment decision)
- deploy-cleanup (finalize receipt)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report deployment status; gated)
```

### On Rerun

If running `/flow-6-deploy` on an existing run-id:
- Read `.runs/<run-id>/deploy/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

This flow uses **git and GitHub** (via `gh` CLI). No external deployment platform required.

**For production extensions** (k8s, canary, metrics): extend this flow with your deployment platform.

## Agents to Use

| Agent | Responsibility |
|-------|----------------|
| **run-prep** | MUST be called first to establish the run directory and `.runs/<run-id>/deploy/` |
| repo-operator | Merge PR, create git tag/release (only if Gate approved MERGE) |
| deploy-monitor | Watch CI and deployment events, write verification report |
| smoke-verifier | Health checks, artifact verification, append to verification report |
| deploy-decider | Synthesize verification into deployment decision |
| **deploy-cleanup** | Write deploy receipt, update index.json status |
| **secrets-sanitizer** | Publish gate before GitHub posting |
| **gh-issue-manager** | Update issue body status board |
| **gh-reporter** | Post deployment summary to issue |

## Upstream Inputs

Read from `.runs/<run-id>/gate/` (if available):
- `merge_decision.md`
- `gate_receipt.json`

**If upstream artifacts are missing**: Flow 6 can start without Flow 4. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue.

## Orchestration Outline

### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/deploy/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "deploy" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/deploy/`.

### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/deploy/flow_plan.md`:

```markdown
# Flow 6: Deploy for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] repo-operator (merge + tag + release; only if Gate verdict MERGE)
- [ ] deploy-monitor (only if Gate verdict MERGE)
- [ ] smoke-verifier (only if Gate verdict MERGE)
- [ ] deploy-decider (deployment decision)
- [ ] deploy-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

### Step 2: Read Gate Decision

Read `.runs/<run-id>/gate/merge_decision.md` (if available):
- Parse the `verdict:` field from `## Machine Summary` (preferred) or `## Verdict` section (MERGE or BOUNCE (with reason))
- This determines the entire flow path
- If missing, default to Path B (NOT_DEPLOYED)

### Path A: Gate Decision = MERGE

**Two operation types in Flow 6:**

| Category | Operations | Gating |
|----------|------------|--------|
| **Release Ops** | Merge PR, create tag, create release | Gate decision = MERGE + repo-operator mechanics |
| **Reporting Ops** | `gh-issue-manager`, `gh-reporter` | Two-gate prerequisites (secrets + repo hygiene) |

Release Ops execute only when Gate's `merge_decision.md` says MERGE. They are gated by the Gate flow's decision, not by the two-gate prerequisites. Reporting Ops use the same two-gate system as all other flows.

**Note on secrets governance:** The code being merged was already sanitized in Build (Flow 3). Deploy's secrets-sanitizer (Step 6) is specifically for `.runs/deploy/` artifacts and GitHub posting—not for code merge governance.

1. **Merge & Tag** (repo-operator) - **Release Op**
   - **Prerequisite:** Gate decision = MERGE
   - Execute `gh pr merge`, create git tag + GitHub release
   - Write `.runs/<run-id>/deploy/deployment_log.md` with merge details
   - **If `gh` CLI not authenticated or PR not found:** Write `deployment_log.md` noting failure, set status NOT_DEPLOYED, `recommended_action: RERUN` (deterministic fix: auth/PR ref). Do not silently skip-this is a failed release op, not a skipped reporting op.

2. **Monitor CI** (deploy-monitor)
   - Watch GitHub Actions status on main branch
   - Write `.runs/<run-id>/deploy/verification_report.md` with CI status

3. **Smoke Tests** (smoke-verifier)
   - If URL available, curl health endpoints; else verify artifacts
   - Append results to `.runs/<run-id>/deploy/verification_report.md`

4. **Decide** (deploy-decider)
   - Synthesize CI + smoke results
   - Write `.runs/<run-id>/deploy/deployment_decision.md` with `verdict`:
     - STABLE: Governance enforced and runtime verification passes
     - NOT_DEPLOYED: Governance not enforced or verification failed
     - BLOCKED_BY_GATE: Gate verdict was not MERGE

5. **Finalize Receipt** (deploy-cleanup)
   - Write `.runs/<run-id>/deploy/deploy_receipt.json`, `.runs/<run-id>/deploy/cleanup_report.md`
   - Update `.runs/index.json` with status, last_flow, updated_at

6. **Sanitize Secrets** (secrets-sanitizer)
   - Scan artifacts before GitHub posting
   - Write `.runs/<run-id>/deploy/secrets_scan.md`, `.runs/<run-id>/deploy/secrets_status.json`
   - **Returns a Gate Result block** for orchestrator routing (control plane)
   - **Status vs flags:** `status` is descriptive (CLEAN/FIXED/BLOCKED_PUBLISH); `safe_to_commit`/`safe_to_publish` are authoritative permissions; `needs_upstream_fix` + `route_to_agent` (and optionally `route_to_flow`) drive routing
   - The JSON file is an audit record; orchestrator routes on the Gate Result block, not by re-reading the file

   **Gate Result block (returned by secrets-sanitizer):**

   <!-- PACK-CONTRACT: GATE_RESULT_V1 START -->
   ```
   ## Gate Result
   status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
needs_upstream_fix: true | false
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | 7 | null
route_to_station: <string | null>
route_to_agent: <agent-name | null>
```
<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->

6b. **Checkpoint Commit** (repo-operator)

   Checkpoint the audit trail **before** any GitHub operations.

   **Allowlist for Flow 6:**
   - `.runs/<run-id>/deploy/`
   - `.runs/<run-id>/run_meta.json`
   - `.runs/index.json`

   **Call `repo-operator`** with `checkpoint_mode: normal` (default). The agent:
   1. Resets staging and stages only the allowlist (not `git add .`)
   2. Enforces the allowlist/anomaly interlock mechanically
   3. Writes `.runs/<run-id>/deploy/git_status.md` if anomaly detected
   4. Handles no-op (nothing staged) gracefully—no empty commits

   **Control plane:** `repo-operator` returns a Repo Operator Result block:
   ```
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```
   **Note:** `commit_sha` is always populated (current HEAD on no-op), never null.

   Orchestrators route on this block, not by re-reading `git_status.md`.

   **Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
   - If `safe_to_commit: false` (from Gate Result): `repo-operator` skips commit entirely
   - If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`
   - If `safe_to_publish: true` and no anomaly: `repo-operator` commits and pushes, returns `proceed_to_github_ops: true`
   - If `safe_to_publish: false`:
     - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`; flow ends UNVERIFIED
     - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
     - Otherwise → UNVERIFIED; commit locally but skip push; returns `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`.

7. **GitHub Reporting** (gh-issue-manager → gh-reporter) - **Reporting Ops**

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR
- Reporting Ops are distinct from Release Ops (merge/tag) above

### Path B: Gate Decision = BOUNCE (including human-review reasons)

1. **Skip Merge** (no repo-operator merge)
   - Write `.runs/<run-id>/deploy/deployment_log.md` noting: "No merge performed; Gate decision = <verdict>"

2. **Minimal Monitoring** (deploy-monitor)
   - Write `.runs/<run-id>/deploy/verification_report.md` noting: "No deployment to verify; Gate decision = <verdict>"
   - Status: NOT_DEPLOYED

3. **Decision** (deploy-decider)
   - Write `.runs/<run-id>/deploy/deployment_decision.md` with:
     - Verdict: NOT_DEPLOYED
     - Explanation of why deployment did not occur
     - Reference to Gate's concerns

4. **Finalize Receipt** (deploy-cleanup)
   - Write receipt with NOT_DEPLOYED status
   - Update index.json

5. **Sanitize + Checkpoint + Report** (secrets-sanitizer → repo-operator → gh-issue-manager → gh-reporter)
   - Same as Path A (steps 6, 6b, 6c, 7, 8)

## Output Artifacts

| Artifact | Description |
|----------|-------------|
| `flow_plan.md` | Execution plan and progress |
| `deployment_log.md` | Record of merge, tag, release actions (or why skipped) |
| `verification_report.md` | CI status + smoke check results |
| `deployment_decision.md` | Final verdict: STABLE / NOT_DEPLOYED / BLOCKED_BY_GATE |
| `deploy_receipt.json` | Receipt for downstream |
| `cleanup_report.md` | Cleanup status and evidence |
| `secrets_scan.md` | Secrets scan report |
| `secrets_status.json` | Publish gate status (audit record) |
| `git_status.md` | Repository status and anomaly documentation (if anomaly detected) |
| `gh_issue_status.md` | Issue board update status |
| `gh_report_status.md` | Log of GitHub posting |
| `github_report.md` | Report content (local copy) |

## deploy-decider Verdicts

| `Verdict` | Meaning |
|---------|---------|
| STABLE | Governance enforced and runtime verification (if present) passes |
| NOT_DEPLOYED | Governance not enforced, or critical check FAIL/UNKNOWN, or runtime verification failed |
| BLOCKED_BY_GATE | Gate verdict was not MERGE; no deployment attempted |

### Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Deployment Verdict**: STABLE | NOT_DEPLOYED | BLOCKED_BY_GATE
- **Next Flow**: `/flow-7-wisdom` (post-deployment analysis)

- [ ] `.runs/<run-id>/deploy/deployment_decision.md` - Is the verdict correct?
- [ ] `.runs/<run-id>/deploy/verification_report.md` - Are checks passing?
- [ ] If NOT_DEPLOYED/BLOCKED_BY_GATE - What action is needed?
```

## Completion

Flow 6 is complete when:
- `deployment_log.md` exists (even if minimal for BOUNCE (including NEEDS_HUMAN_REVIEW))
- `verification_report.md` exists
- `deployment_decision.md` exists with valid verdict

Human gate at end: "Did deployment succeed?" (or "Why didn't we deploy?")

---

## Orchestrator Kickoff

### Station order

#### Station order

- `run-prep`
- `repo-operator` (ensure run branch)
- `repo-operator` (merge + tag + release; only if Gate verdict MERGE)
- `deploy-monitor` (only if Gate verdict MERGE)
- `smoke-verifier` (only if Gate verdict MERGE)
- `deploy-decider`
- `deploy-cleanup`
- `secrets-sanitizer`
- `repo-operator` (checkpoint; read Repo Operator Result)
- `gh-issue-manager` (if allowed)
- `gh-reporter` (if allowed)

### TodoWrite (copy exactly)

```
- [ ] run-prep
- [ ] repo-operator (ensure run/<run-id> branch)
- [ ] repo-operator (merge + tag + release; only if Gate verdict MERGE)
- [ ] deploy-monitor (only if Gate verdict MERGE)
- [ ] smoke-verifier (only if Gate verdict MERGE)
- [ ] deploy-decider
- [ ] deploy-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint commit; allowlist interlock + no-op handling)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
```

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.
