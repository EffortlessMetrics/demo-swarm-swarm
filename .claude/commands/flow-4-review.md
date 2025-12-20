---
description: "Run Flow 4 (Review): harvest PR feedback, apply fixes, flip Draft to Ready when complete."
---

# Flow 4: PR Review + Improvement

You are orchestrating Flow 4 of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/review/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/review/` exists.

#### Artifact visibility rule

* Do **not** attempt to prove files exist under `.runs/<run-id>/` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on verification agents to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Ensure a PR exists (create Draft if missing)
- Harvest all available PR feedback (CodeRabbit, GitHub Actions, Dependabot, human reviews)
- Convert feedback into an actionable worklist
- Apply fixes until completion (unbounded loop)
- Reseal build receipt after changes
- Flip Draft PR to Ready when review is complete
- Update issue and PR with progress

## Before You Begin (Required)

### Two State Machines

Flow 4 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - The worklist loop is ONE todo (unbounded iterations).

2. Mirror the same list into `.runs/<run-id>/review/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- pr-creator (create Draft PR if none exists)
- pr-feedback-harvester (pull all bot/human feedback)
- review-worklist-writer (cluster into actionable items)
- worklist loop (unbounded: resolve items until completion/context/unrecoverable)
- pr-commenter (post/update PR summary comment)
- pr-status-manager (flip Draft to Ready if review complete)
- review-cleanup (finalize receipt; update index; update flow_plan.md)
- secrets-sanitizer (publish gate)
- repo-operator (commit/push)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report to GitHub; gated)
```

### On Rerun

If running `/flow-4-review` on an existing run-id:
- Read `.runs/<run-id>/review/flow_plan.md`
- Read `.runs/<run-id>/review/review_worklist.json` for current item statuses
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Resume the worklist loop from pending items

If you encounter missing PR or unclear state, **document it and continue**. Create the PR if possible.

## Subagents to use

**Infrastructure (Step 0)**:
- **run-prep** -- establish the run directory and `.runs/<run-id>/review/`

**Git operations (cross-cutting)**:
- repo-operator -- branch at start, commit at end

**PR lifecycle**:
- pr-creator -- create Draft PR if none exists
- pr-feedback-harvester -- read all PR feedback sources
- review-worklist-writer -- convert feedback to actionable worklist
- pr-commenter -- post idempotent PR summary comment (after worklist loop)
- pr-status-manager -- flip Draft to Ready when review complete

**Fix loop agents (reused from Build)**:
- test-author -- fix test-related items
- code-implementer -- fix code-related items
- doc-writer -- fix documentation items
- fixer -- apply targeted fixes
- test-executor -- verify fixes

**Polish and wrap-up**:
- build-cleanup -- reseal build receipt after code changes
- review-cleanup -- write review_receipt.json, update index

**Cleanup + Reporting (End of Flow)**:
- secrets-sanitizer -- publish gate
- repo-operator -- commit/push (gated on secrets)
- gh-issue-manager -- update issue board
- gh-reporter -- post summary to GitHub

## Upstream Inputs

Read from `.runs/<run-id>/build/` (if available):
- `build_receipt.json`
- `pr_creation_status.md`

Read from `.runs/<run-id>/run_meta.json`:
- `pr_number` (from pr-creator in Flow 3)
- `issue_number`
- `github_repo`

**If PR does not exist**: Call `pr-creator` to create a Draft PR first.

**If upstream artifacts are missing**: Flow 4 can start without Flows 1-3. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue.

## Artifact Outputs

| Artifact | Producer | Description |
|----------|----------|-------------|
| `flow_plan.md` | Orchestrator | Flow progress tracking |
| `pr_feedback.md` | pr-feedback-harvester | Summarized bot + human feedback |
| `pr_feedback_raw.json` | pr-feedback-harvester | Raw API responses (optional) |
| `review_worklist.md` | review-worklist-writer | Actionable items with stable markers |
| `review_worklist.json` | review-worklist-writer | Machine-readable worklist |
| `review_actions.md` | Orchestrator | Cumulative log of changes made |
| `cleanup_report.md` | review-cleanup | Cleanup summary |
| `review_receipt.json` | review-cleanup | Machine-readable receipt |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings |
| `secrets_status.json` | secrets-sanitizer | Gate status (audit record) |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | Issue operation status |
| `github_report.md` | gh-reporter | Local copy of GitHub post |
| `gh_report_status.md` | gh-reporter | GitHub posting status |

All artifacts live under `.runs/<run-id>/review/`.

## Orchestration outline

### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/review/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "review" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/review/`.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/review/flow_plan.md`:

```markdown
# Flow 4: Review for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] pr-creator (create Draft PR if none exists)
- [ ] pr-feedback-harvester (pull all feedback)
- [ ] review-worklist-writer (create actionable worklist)
- [ ] worklist loop (unbounded: resolve items)
- [ ] PR status management (inline: flip Draft to Ready if complete)
- [ ] review-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (commit/push)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Worklist Progress

| Item | Category | Severity | Status |
|------|----------|----------|--------|
| (populated by worklist loop) |

## Progress Notes

<Update as each step completes>
```

### Step 2: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely.

### Step 3: Create Draft PR (if needed)

**Call `pr-creator`** to ensure a Draft PR exists.

- If PR already exists: capture `pr_number` from result
- If no PR: create Draft PR targeting `main`
- Write `pr_number` to `run_meta.json`

**Route on PR Creator Result block:**
- If `operation_status: CREATED` or `EXISTING`: proceed
- If `operation_status: SKIPPED`: note the reason, proceed anyway (can retry later)
- If `operation_status: FAILED`: note in concerns, proceed with available feedback

### Step 4: Harvest PR Feedback

**Call `pr-feedback-harvester`** to pull all available feedback sources.

Sources:
- PR reviews (human + bot like CodeRabbit)
- PR line comments
- Issue comments on PR
- CI check runs (GitHub Actions, etc.)
- Check suites summary

**Route on PR Feedback Harvester Result block:**
- If `status: VERIFIED`: proceed with full feedback
- If `status: UNVERIFIED` (no PR, auth issue): proceed with whatever was captured
- If `status: CANNOT_PROCEED`: note mechanical failure, proceed without external feedback

### Step 5: Create Review Worklist

**Call `review-worklist-writer`** to convert feedback into actionable items.

- Each item gets a stable `RW-NNN` ID
- Items are categorized: CORRECTNESS, TESTS, STYLE, DOCS
- Items are prioritized: CRITICAL, MAJOR, MINOR, INFO
- Items are routed to appropriate agents

**Route on Review Worklist Writer Result block:**
- Proceed with the worklist regardless of status
- If no items: VERIFIED (nothing to do)
- If items exist: prepare for worklist loop

### Step 6: Worklist Loop (Unbounded)

**This is the core of Flow 4: iteratively resolve worklist items until completion.**

**Termination conditions** (any of):
1. All worklist items resolved (`pending == 0`)
2. Context window exhaustion (approaching limit)
3. Unrecoverable blocker (mechanical failure, design issue requiring Plan bounce)

**Loop structure:**

```
while not terminated:
    1. Read current worklist status from review_worklist.json
    2. If pending == 0: break (complete)
    3. If context exhausted: break (can resume later)

    4. Pick next pending item (priority order: CRITICAL > MAJOR > MINOR)

    5. Route to appropriate agent based on item.route_to:
       - test-author: for TESTS items
       - code-implementer: for CORRECTNESS/ARCHITECTURE items
       - doc-writer: for DOCS items
       - fixer: for STYLE items

    6. Run test-executor to verify the fix (fast confirm)

    7. Update item status in review_worklist.json:
       - RESOLVED: if fix verified
       - PENDING: if fix failed (will retry)
       - SKIPPED: if out of scope (document reason)

    8. Append to review_actions.md what was done

    9. If meaningful code changes made:
       - Run build-cleanup to reseal build_receipt.json
       - Run test-executor (full suite) periodically

    10. Re-harvest feedback (pr-feedback-harvester) periodically:
        - New CodeRabbit comments may appear after pushes
        - CI results update after commits
        - Human reviewers may add comments

    11. If new feedback items found:
        - Run review-worklist-writer to update worklist
        - New items get appended (IDs continue from last)
```

**Per-item fix process:**

For each pending worklist item RW-NNN:

1. Read the item details (category, severity, location, summary)
2. Call the routed agent with context:
   - Item ID and summary
   - File path and line number
   - Evidence from feedback
3. If agent succeeds: mark RESOLVED
4. If agent fails: keep PENDING (may need different approach)
5. Log action in `review_actions.md`

**Reseal after changes:**

When code/test changes are made:
1. Stage changes (repo-operator)
2. Run build-cleanup to update build_receipt.json
3. Periodically run full test suite (test-executor)

**Re-harvest cadence (gated):**

Every N items resolved (or after significant changes), apply the **Reseal → Gate → Push → Re-harvest** subroutine:

1. **Reseal receipts:**
   - Call `build-cleanup` to reseal build_receipt.json (if code/tests changed)
   - Call `review-cleanup` to update worklist state
2. **Secrets gate:**
   - Call `secrets-sanitizer` on staged changes
   - If `modified_files: true`: repeat reseal until stable
3. **Commit and push (gated):**
   - If `safe_to_commit: true` and `safe_to_publish: true`: call `repo-operator` to commit/push
   - If gates fail: record the blocker and proceed without push (bots won't have new code)
4. **Re-harvest:**
   - Call `pr-feedback-harvester` (new bot comments may appear after push)
   - Call `review-worklist-writer` to update worklist with new items
   - If bots haven't posted yet: record "pending feedback" and proceed (can re-harvest on next iteration)

**Note:** Agents cannot "wait" - they re-harvest and check. If new feedback appears, it gets added to the worklist for subsequent iterations.

### Step 7: PR Status Management

After worklist loop completes, manage PR status via dedicated agents.

**Call `pr-commenter`** to post/update the PR summary comment:
- Summarizes worklist progress
- Lists recent changes from `review_actions.md`
- Idempotent (updates existing comment with marker)

**Call `pr-status-manager`** to manage PR state:
- If review is complete: flip Draft PR to Ready for Review
- If review is incomplete: keep as Draft, document what's remaining

**Route on PR Status Manager Result block:**
- If `operation_status: TRANSITIONED`: PR is now ready for human review
- If `operation_status: UNCHANGED`: state kept as-is (review incomplete or already ready)
- If `operation_status: SKIPPED`: note reason and continue

### Step 8: Finalize and Write Receipt

**Call `review-cleanup`** to:
- Verify all required artifacts exist
- Compute counts mechanically
- Write `review_receipt.json`
- Update `.runs/index.json` with status, last_flow, updated_at

### Step 9: Sanitize Secrets (Publish Gate)

**Call `secrets-sanitizer`** to scan staged changes and audit artifacts.

**secrets-sanitizer** returns a **Gate Result** block:

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

**Gating logic (from Gate Result):**
- If `safe_to_commit: false`: apply route-and-fix triage
- If `modified_files: true`: reseal loop (review-cleanup → secrets-sanitizer)
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`

### Step 10: Commit and Push

**Call `repo-operator`** to commit and push.

Same gating logic as Build:
- Requires `safe_to_commit: true` and `safe_to_publish: true`
- Returns Repo Operator Result block

### GitHub Access + Content Mode (canonical)

See `CLAUDE.md` → **GitHub Access + Content Mode (Canonical)**.

- Publish blocked → `RESTRICTED` (never skip when access is allowed)
- `FULL` only when `safe_to_publish: true` AND `proceed_to_github_ops: true` AND `publish_surface: PUSHED`

### Step 11: Update Issue Board

Apply Access + Content Mode rules:
- Skip GitHub calls if `github_ops_allowed: false` or `gh` unauthenticated (record SKIPPED/UNVERIFIED).
- Otherwise derive `FULL` vs `RESTRICTED` from gates + publish surface. Publish blocked reasons must be explicit; RESTRICTED uses paths only and the receipt allowlist.

`gh-issue-manager` updates issue body status board from receipt.

### Step 12: Report to GitHub

Apply Access + Content Mode rules:
- Skip only when `github_ops_allowed: false` or `gh` unauthenticated (record SKIPPED/UNVERIFIED).
- Otherwise post in `FULL` only when `safe_to_publish: true`, `proceed_to_github_ops: true`, and `publish_surface: PUSHED`; use `RESTRICTED` for all other cases.

`gh-reporter` writes `.runs/<run-id>/review/github_report.md` locally and posts to the issue (never PR). Issue-first (hard): flow logs go to the issue even if a PR exists.

### Step 13: Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Worklist Items**: <resolved>/<total> resolved
- **PR State**: draft | ready
- **Next Flow**: `/flow-5-gate` (after human review)

## Human Review Checklist

Before proceeding to Flow 5, humans should review:
- [ ] PR is ready for review (not draft)
- [ ] All critical worklist items are resolved
- [ ] CI checks are passing
- [ ] CodeRabbit concerns addressed
```

## Status States

Agents report one of:
- **VERIFIED**: All critical items resolved, review complete.
- **UNVERIFIED**: Items still pending or incomplete feedback.
- **CANNOT_PROCEED**: IO/permissions/tool failure only.

## Review Completion Criteria

Flow 4 is VERIFIED when:
- All CRITICAL worklist items are resolved
- All MAJOR worklist items are resolved (or explicitly deferred with reason)
- CI checks are passing
- No blocking review requests

MINOR and INFO items may remain pending without blocking.

---

## Orchestrator Kickoff


### Station order + templates

#### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `pr-creator` (create Draft PR if none exists)

4. `pr-feedback-harvester`

5. `review-worklist-writer`

6. **Worklist loop** (unbounded; apply Worklist Loop Template below)

7. `pr-commenter` (post/update PR summary comment)

8. `pr-status-manager` (flip Draft to Ready if review complete)

9. `review-cleanup`

10. `secrets-sanitizer`

11. `review-cleanup` ↔ `secrets-sanitizer` (reseal cycle; if `modified_files: true`)

12. `repo-operator` (commit/push)

13. `gh-issue-manager` (if allowed)

14. `gh-reporter` (if allowed)

#### Worklist Loop Template (unbounded resolution)

This is the core review loop. Unlike Build's bounded microloops, this runs until completion.

**Entry:** review_worklist.json exists with items

**Loop:**
```
1) Read worklist status (total, pending, resolved)
2) If pending == 0: exit loop (complete)
3) If context exhausted: exit loop (can resume later)

4) Pick next pending item by priority:
   - CRITICAL first
   - Then MAJOR
   - Then MINOR (optional)
   - Skip INFO

5) Route to agent:
   - TESTS items → test-author
   - CORRECTNESS items → code-implementer
   - STYLE items → fixer or lint-executor
   - DOCS items → doc-writer
   - ARCHITECTURE items → code-implementer

6) Run fix agent with item context

7) Run test-executor (fast confirm: relevant tests)

8) Update worklist item status:
   - If fix verified: RESOLVED
   - If fix failed: keep PENDING
   - If out of scope: SKIPPED with reason

9) Append to review_actions.md

10) Every N items or after major changes:
    - Apply Reseal → Gate → Push → Re-harvest subroutine (see Re-harvest cadence)
    - This ensures proper gating before push and captures new bot feedback
```

**Exit conditions:**
- `pending == 0` (all resolved)
- Context window approaching limit
- Unrecoverable blocker

#### Microloop Template (writer ↔ critic)

Reused from Build when needed within the worklist loop.

1) Writer pass: call `<writer>`
2) Critique pass: call `<critic>` and read its control-plane Result
3) Apply pass: call `<writer>` once using the critic's worklist
4) Re-critique: call `<critic>` again

Continue beyond default two passes only when critic returns `recommended_action: RERUN` and `can_further_iteration_help: yes`.

### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] pr-creator (create Draft PR if needed)
- [ ] pr-feedback-harvester
- [ ] review-worklist-writer
- [ ] worklist loop (unbounded: resolve items until completion/context/unrecoverable)
- [ ] pr-commenter (post/update PR summary comment)
- [ ] pr-status-manager (flip Draft to Ready if review complete)
- [ ] review-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] review-cleanup ↔ secrets-sanitizer (reseal cycle; if `modified_files: true`)
- [ ] repo-operator (commit/push; return Repo Operator Result)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.
