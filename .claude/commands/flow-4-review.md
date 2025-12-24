---
description: "Run Flow 4 (Review): harvest PR feedback, apply fixes, flip Draft to Ready when complete."
---

# Flow 4: PR Review + Improvement

You are orchestrating Flow 4 of the SDLC swarm.

## The Mental Model: "The Finishing School"

Flow 3 built the house. Flow 4 does the punch list.

**Mentality:** Feedback is noisy, time is linear, code rots instantly. Grab what's available, fix it, report it, move on. Don't wait for perfect signal.

**Three Phases:**
1. **Harvest & Cluster** — Pull all feedback, cluster into actionable Work Items
2. **Execute** — Route Work Items to agents, fix what's current, skip what's stale
3. **Close the Loop** — Update the PR, show humans what was addressed

**Key principle:** Agents are smart. They read the file, see if the code is there, fix it or report "context changed." No separate stale-check ceremony.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/review/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Setup (run-prep) establishes the run directory and ensures `.runs/<run-id>/review/` exists.

#### Artifact visibility rule

* Do **not** attempt to prove files exist under `.runs/<run-id>/` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on verification agents to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Ensure a PR exists (create Draft if missing)
- Harvest all available PR feedback (grab partials from CI if already failing)
- Convert feedback into clustered Work Items (by file/theme, not individual comments)
- Apply fixes until completion (agents handle staleness naturally)
- Flip Draft PR to Ready when review is complete
- Post a closure checklist so humans see feedback was addressed

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

## Orchestration Outline

Flow 4 follows the 3-phase model with setup and seal bookends:

```
[Setup] → [Phase 1: Harvest & Cluster] → [Phase 2: Execute] → [Phase 3: Close] → [Seal]
```

---

### Setup: Infrastructure

**run-prep** → **repo-operator** (branch) → **pr-creator** (if needed)

1. **Call `run-prep`** to establish `.runs/<run-id>/review/`
2. **Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"
3. **Call `pr-creator`** to ensure a Draft PR exists

After setup, you have a run directory and a PR to harvest feedback from.

---

### Phase 1: Harvest & Cluster

**pr-feedback-harvester** → **review-worklist-writer**

**Call `pr-feedback-harvester`:**
- Grabs all available feedback (bots, humans, CI)
- Grabs partial CI failures if jobs are still running but already failing
- Doesn't wait for pending checks

**Call `review-worklist-writer`:**
- Clusters feedback into Work Items (by file/theme, not individual comments)
- 50 comments → 5-10 Work Items
- Items get stable `RW-NNN` IDs
- Markdown nits grouped into single `RW-MD-SWEEP`

**Route on worklist:** If no items, proceed to Close. Otherwise, enter Execute loop.

---

### Phase 2: Execute (Unbounded Loop)

**The core of Flow 4: iteratively resolve Work Items.**

**Loop until done:**

```
while pending > 0 and not exhausted:
    1. Pick next batch by priority + affinity:
       - CRITICAL first, then MAJOR, then MINOR
       - Batch by file (3 issues in auth.ts = one agent call)
       - Batch by theme (security issue + related test gap)

    2. Route batch to agent:
       - TESTS → test-author
       - CORRECTNESS → code-implementer
       - STYLE → fixer
       - DOCS → doc-writer

    3. Agent fixes naturally:
       - Reads the file, sees what's there
       - Fixes if current, reports "context changed" if stale
       - No separate stale-check ceremony

    4. Update worklist:
       - Fixed → RESOLVED
       - Stale/moved → SKIPPED
       - Failed → PENDING (retry later)

    5. Log action in review_actions.md

    6. Periodically: push → re-harvest → refresh worklist
       - If stuck_signal: true → exit loop
```

**Exit conditions:**
- `pending == 0` → complete
- Context exhausted → PARTIAL (checkpoint, rerun to continue)
- `stuck_signal: true` → PARTIAL (human may need to intervene)

**Style Sweep:** If `RW-MD-SWEEP` is pending, call `fixer` once to apply all markdown fixes in one pass.

---

### Phase 3: Close the Loop

**pr-commenter** → **pr-status-manager**

**Call `pr-commenter`:**
- Posts resolved items checklist (closure signal)
- Shows what was fixed, skipped, or pending
- Idempotent (updates existing comment)

**Call `pr-status-manager`:**
- If review complete: flip Draft → Ready for Review
- If incomplete: keep Draft, document remaining items

---

### Seal: Receipt + Publish

**review-cleanup** → **secrets-sanitizer** → **repo-operator** → **gh-issue-manager** → **gh-reporter**

1. **`review-cleanup`** — Write `review_receipt.json`, update index
2. **`secrets-sanitizer`** — Publish gate (returns Gate Result)
3. **`repo-operator`** — Commit/push (gated on secrets + hygiene)
4. **`gh-issue-manager`** + **`gh-reporter`** — Update issue (if allowed)

**Gate Result semantics:**
- `safe_to_commit: false` → skip commit
- `safe_to_publish: false` → commit locally, skip push
- `proceed_to_github_ops: false` → skip GitHub updates

---

### flow_plan.md Template

```markdown
# Flow 4: Review for <run-id>

## Phases

- [ ] Setup (run-prep, branch, PR)
- [ ] Harvest & Cluster
- [ ] Execute (worklist loop)
- [ ] Close the Loop (comment, status)
- [ ] Seal (receipt, publish)

## Worklist Progress

| Item | Category | Severity | Status |
|------|----------|----------|--------|
| (populated by worklist loop) |

## Summary

- **Final Status**: VERIFIED | PARTIAL | UNVERIFIED
- **Worklist Items**: <resolved>/<total> resolved
- **PR State**: draft | ready
- **Next Flow**: `/flow-5-gate`
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

### Station Order (5 groups)

```
SETUP          run-prep → repo-operator (branch) → pr-creator
HARVEST        pr-feedback-harvester → review-worklist-writer
EXECUTE        worklist loop (unbounded)
CLOSE          pr-commenter → pr-status-manager
SEAL           review-cleanup → secrets-sanitizer → repo-operator → gh-issue-manager → gh-reporter
```

### Execute Loop (Detailed)

**Entry:** `review_worklist.json` exists with pending items

**Loop:**
```
1) Check worklist: pending count
2) If pending == 0: exit (complete)
3) If context exhausted: checkpoint and exit (PARTIAL)

4) Style Sweep (if `RW-MD-SWEEP` pending):
   - Call fixer once for all markdown fixes
   - Re-harvest feedback

5) Pick next batch by priority + affinity:
   - CRITICAL → MAJOR → MINOR (skip INFO)
   - Batch by file (3 issues in auth.ts = one call)
   - Batch by theme (security + related test gap)
   - Up to 3-5 items per batch

6) Route to agent:
   - TESTS → test-author
   - CORRECTNESS → code-implementer
   - STYLE → fixer
   - DOCS → doc-writer

7) Agent fixes naturally:
   - Reads file, fixes what's there
   - Reports "context changed" if stale
   - No separate stale-check step

8) Update worklist:
   - Fixed → RESOLVED
   - Stale → SKIPPED
   - Failed → PENDING

9) Log in review_actions.md

10) Periodically: push → re-harvest → refresh worklist
    - If stuck_signal: true → exit loop
```

**Exit conditions:**
- `pending == 0` (all resolved) → VERIFIED
- Context exhausted → PARTIAL
- `stuck_signal: true` → PARTIAL
- Unrecoverable blocker → UNVERIFIED

### TodoWrite (3-phase model)

```
- [ ] Setup (run-prep, branch, pr-creator)
- [ ] Harvest & Cluster (pr-feedback-harvester, review-worklist-writer)
- [ ] Execute loop (resolve items until completion/context/stuck)
- [ ] Close the Loop (pr-commenter, pr-status-manager)
- [ ] Seal (review-cleanup, secrets-sanitizer, repo-operator, gh-issue-manager, gh-reporter)
```

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

