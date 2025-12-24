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
| `style_sweep.md` | Orchestrator | Style sweep result (NOOP if no pending MINOR Markdown items) |
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

- Each item gets a stable `RW-NNN` ID (except the grouped Markdown sweep uses `RW-MD-SWEEP`)
- Items are categorized: CORRECTNESS, TESTS, STYLE, DOCS
- Items are prioritized: CRITICAL, MAJOR, MINOR, INFO
- Items are routed to appropriate agents
- MINOR markdownlint/formatting nits are grouped into `RW-MD-SWEEP` (STYLE, MINOR, route_to: fixer) with files/rules/examples/scope and optional children for traceability

**Route on Review Worklist Writer Result block:**
- Proceed with the worklist regardless of status
- If no items: VERIFIED (nothing to do)
- If items exist: prepare for worklist loop

### Step 6: Worklist Loop (Unbounded)

**This is the core of Flow 4: iteratively resolve worklist items until completion.**

**Termination conditions** (any of):
1. All worklist items resolved (`pending == 0`) → status: VERIFIED
2. Context window exhaustion (approaching limit) → status: PARTIAL (checkpoint & exit)
3. Unrecoverable blocker (mechanical failure, design issue requiring Plan bounce) → status: UNVERIFIED
4. **Stuck detection triggered** → status: PARTIAL (checkpoint & exit with documented stuck state)

### Stuck Detection (delegated to review-worklist-writer)

Stuck detection is handled by the `review-worklist-writer` agent, NOT by the orchestrator.

**How it works:**
- When the worklist is refreshed, `review-worklist-writer` compares the current worklist to the previous version
- The agent detects if the same items keep failing repeatedly (3+ cycles with no status changes)
- The agent emits a `stuck_signal: true | false` field in its Result block
- The agent also emits `stuck_items: []` identifying which items are blocking progress

**Orchestrator routing (pure routing, no computation):**
- Route on Result block: `if stuck_signal == true: exit loop`
- Do NOT maintain counters or compute hashes in the orchestrator

**On stuck detection (when `stuck_signal: true`):**
1. Mark remaining PENDING items as `DEFERRED` with reason: `"STUCK: No progress after 3 iterations"`
2. Write stuck analysis to `review_actions.md` using `stuck_items` from result
3. Checkpoint with `status: PARTIAL`
4. Exit with message: "Review loop stuck. {N} items deferred. Human intervention may be required."

**Why delegated?** The orchestrator should be a pure router. The `review-worklist-writer` agent already has the worklist context and can detect stuck patterns efficiently without the orchestrator maintaining state or computing hashes.

**Context checkpoint behavior (PARTIAL):**
When context is approaching limits, checkpoint immediately:
- Write current worklist state to `review_worklist.json`
- Update `review_receipt.json` with `status: PARTIAL`, `items_resolved`, `items_remaining`
- Commit and push (if gates allow)
- Exit with message: "Resolved {N} items. {M} remain. Context full. Rerun `/flow-4-review` to continue."

This is a **feature, not a failure**. It prevents hallucination from context stuffing and enables incremental progress. The next `/flow-4-review` invocation will resume from the checkpoint.

**Loop structure (pure routing, no computation):**

```
while not terminated:
    1. Read current worklist status from review_worklist.json
    2. If pending == 0: break (complete)
    3. If context exhausted: break (can resume later)

    4. Run Style Sweep station (always):
       - If `RW-MD-SWEEP` is pending: call fixer once to apply all remaining MINOR Markdown formatting fixes in one pass, then run test-executor (pack-check) once, then re-harvest feedback once
       - Update `RW-MD-SWEEP` status and write style_sweep.md (NOOP if no pending MINOR style items)
       - If sweep touched `.runs/<run-id>/build/`, run build-cleanup to reseal build_receipt.json

    5. Pick next pending item (priority order: CRITICAL > MAJOR > MINOR)

    6. Route to appropriate agent based on item.route_to:
       - test-author: for TESTS items
       - code-implementer: for CORRECTNESS/ARCHITECTURE items
       - doc-writer: for DOCS items
       - fixer: for STYLE items

    7. Call fix agent with item context
       - Agent performs stale check FIRST (see agent prompts)
       - If agent returns `worklist_item_status: SKIPPED` with skip_reason: mark item SKIPPED and move to next
       - If agent returns `worklist_item_status: RESOLVED`: run test-executor to verify

    8. Route on agent Result block:
       - If `worklist_item_status: RESOLVED` and tests pass: mark RESOLVED
       - If `worklist_item_status: SKIPPED`: mark SKIPPED (agent already verified staleness)
       - If fix failed: keep PENDING

    9. Append to review_actions.md what was done

    10. If meaningful code changes made:
       - Run build-cleanup to reseal build_receipt.json
       - Run test-executor (full suite) periodically

    11. Re-harvest feedback (pr-feedback-harvester) periodically:
        - New CodeRabbit comments may appear after pushes
        - CI results update after commits
        - Human reviewers may add comments

    12. If new feedback items found:
        - Run review-worklist-writer to refresh worklist
        - **Route on stuck_signal from Result block:**
          - If `stuck_signal: true`: break loop (stuck detection triggered)
          - If `stuck_signal: false`: continue processing
```

**Style Sweep station (standard, always run):**
- Check for a pending `RW-MD-SWEEP` or pending MINOR markdownlint items in the worklist.
- If none: write `.runs/<run-id>/review/style_sweep.md` with `status: NOOP` and "no pending MINOR style items".
- If present: call `fixer` with "apply all remaining MINOR Markdown formatting fixes in one pass" and `scope: mechanical formatting only`, then run `test-executor` once (pack-check), then re-harvest feedback once.
- After re-harvest, call `review-worklist-writer` to refresh `review_worklist.json`, then append to `review_actions.md`. If noise remains, leave `RW-MD-SWEEP` PENDING (non-blocking).
- Do not route markdownlint child items individually; resolve them via the `RW-MD-SWEEP` parent.
- Guardrail: if the sweep touches anything under `.runs/<run-id>/build/`, call `build-cleanup` to reseal `build_receipt.json`.

### Intelligent Summarization (Not File Pointers)

When summarizing feedback for routing or reporting, use JUDGMENT:

**Bad (file pointer dump):**
```
- RW-001: See pr_feedback.md line 45
- RW-002: CodeRabbit comment at src/auth.ts:42
- RW-003: CI failure in tests.yml
```

**Good (intelligent summary):**
```
- RW-001: CodeRabbit found a potential null pointer at auth.ts:42. The code assumes `user` is always defined but the function can be called before login completes. Route to code-implementer to add a guard.
- RW-002: CI tests failing because hashPassword returns undefined for empty strings. This is a valid edge case not covered by current tests. Route to test-author first to add the test, then code-implementer to fix.
- RW-003: Human reviewer asked about error handling strategy. This is a design question, not a code issue. Document in open_questions.md and proceed.
```

**The agent reading the feedback is SMART.** It should understand:
- What the feedback actually means (not just where it is)
- Whether the feedback is valid or a false positive
- What agent is best suited to address it
- Whether the issue is still relevant (stale check)

**Per-item fix process (stale check delegated to agents):**

For each pending worklist item RW-NNN (excluding `RW-MD-SWEEP`, which is handled by the Style Sweep station):

1. Read the item details (category, severity, location, summary)

2. **Route to fix agent with item context:**
   - Item ID and summary
   - File path and line number
   - Evidence from feedback

3. **The fix agent performs the stale check FIRST** (see agent prompts for `code-implementer`, `fixer`, `doc-writer`, `test-author`):
   - Agent verifies the code still exists at HEAD
   - If stale: agent returns `worklist_item_status: SKIPPED` with `skip_reason` and `skip_evidence`
   - If current: agent proceeds with fix

4. **Route on agent Result block:**
   - If `worklist_item_status: SKIPPED`: mark item SKIPPED, log skip_evidence to `review_actions.md`, move to next item
   - If `worklist_item_status: RESOLVED`: run test-executor to verify, then mark RESOLVED
   - If fix failed: keep PENDING (may need different approach)

5. Log action in `review_actions.md`

**Why delegated?** The orchestrator should not read files to check code existence. The fix agent already needs to read the file to apply the fix — it can verify staleness as its first action and return immediately if stale. This keeps the orchestrator as a pure router.

**Reseal after changes:**

When code/test changes are made (or the Style Sweep touches `.runs/<run-id>/build/`):
1. Stage changes (repo-operator)
2. Run build-cleanup to update build_receipt.json
3. Periodically run full test suite (test-executor)

**Re-harvest cadence (gated) - MANDATORY after each push:**

The swarm does **not wait** for CI or bots. It pushes, then immediately re-harvests whatever feedback is available. This is the "Push → Re-harvest" pattern:

**Trigger conditions (any of):**
- After resolving 3-5 worklist items
- After any CRITICAL item is resolved
- After significant code changes (touching core modules)
- When stuck counter increases (to get fresh signal)

**The Reseal → Gate → Push → Re-harvest subroutine:**

1. **Reseal receipts:**
   - Call `build-cleanup` to reseal build_receipt.json (if code/tests changed)
   - Call `review-cleanup` to update worklist state

2. **Secrets gate:**
   - Call `secrets-sanitizer` on staged changes (rescan allowed if new changes staged)

3. **Commit and push (gated):**
   - If `safe_to_commit: true` and `safe_to_publish: true`: call `repo-operator` to commit/push
   - If gates fail: record the blocker and proceed without push (bots won't have new code)

4. **Re-harvest (immediately after push):**
   - Call `pr-feedback-harvester` - captures whatever CI/bot feedback exists *right now*
   - Call `review-worklist-writer` to update worklist with new items
   - **Do not wait** for bots to finish - take what's available and proceed
   - If bots haven't posted yet: that's fine, next iteration will catch it

**Why immediate re-harvest?** The swarm can't "sleep" for CI. By re-harvesting immediately after push, we capture:
- Any CI that finished between iterations
- CodeRabbit comments posted on new code
- Human reviews that arrived while we were fixing

If nothing new appears, the worklist stays the same and we continue. The pattern is: **push early, harvest often, never wait.**

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

<!-- PACK-CONTRACT: GATE_RESULT_V3 START -->
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
<!-- PACK-CONTRACT: GATE_RESULT_V3 END -->

**Gating logic (from Gate Result):**
- The sanitizer is a boolean gate — it says yes/no, not where to route
- If `safe_to_commit: false`: skip commit (blocked by `blocker_kind`)
- If `safe_to_commit: true` but `safe_to_publish: false`: commit locally, skip push
- `modified_files: true`: artifact files were changed (for audit purposes)
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`
- `blocker_kind` explains why blocked: `MECHANICAL` (IO failure), `SECRET_IN_CODE` (needs fix), `SECRET_IN_ARTIFACT` (can't redact)

### Step 10: Commit and Push

**Call `repo-operator`** to commit and push.

Same gating logic as Build:
- Requires `safe_to_commit: true` and `safe_to_publish: true`
- Returns Repo Operator Result block

### Step 11-12: GitHub Reporting

**Call `gh-issue-manager`** then **`gh-reporter`** to update the issue.

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

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

11. `repo-operator` (commit/push)

12. `gh-issue-manager` (if allowed)

13. `gh-reporter` (if allowed)

#### Worklist Loop Template (unbounded resolution, pure routing)

This is the core review loop. Unlike Build's bounded microloops, this runs until completion.

**Entry:** review_worklist.json exists with items

**Loop (pure routing - no computation in orchestrator):**
```
1) Read worklist status (total, pending, resolved)
2) If pending == 0: exit loop (complete)
3) If context exhausted: exit loop (can resume later)

4) Run Style Sweep station (always; NOOP if no pending MINOR Markdown items):
   - If `RW-MD-SWEEP` is pending: call fixer once to apply all remaining MINOR Markdown formatting fixes in one pass, then run test-executor (pack-check) once, then re-harvest feedback once
   - Update `review_worklist.json`, write `style_sweep.md`, and reseal build receipt if `.runs/<run-id>/build/` is touched

5) Pick next pending item by priority:
   - CRITICAL first
   - Then MAJOR
   - Then MINOR (optional; exclude `RW-MD-SWEEP` children)
   - Skip INFO

6) Route to agent:
   - TESTS items → test-author
   - CORRECTNESS items → code-implementer
   - STYLE items → fixer or standards-enforcer
   - DOCS items → doc-writer
   - ARCHITECTURE items → code-implementer

7) Call fix agent with item context
   - Agent performs stale check FIRST
   - Route on agent Result block (worklist_item_status field)

8) Route on Result block:
   - If `worklist_item_status: SKIPPED`: mark SKIPPED, move to next (agent verified staleness)
   - If `worklist_item_status: RESOLVED`: run test-executor, then mark RESOLVED
   - If fix failed: keep PENDING

9) Append to review_actions.md

10) Every N items or after major changes:
    - Apply Reseal → Gate → Push → Re-harvest subroutine (see Re-harvest cadence)
    - Route on review-worklist-writer Result block:
      - If `stuck_signal: true`: exit loop (stuck detection triggered)
      - If `stuck_signal: false`: continue processing
```

**Exit conditions:**
- `pending == 0` (all resolved)
- Context window approaching limit
- `stuck_signal: true` (from review-worklist-writer)
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
- [ ] repo-operator (commit/push; return Repo Operator Result)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

