---
description: Run Flow 3 (Design -> Code): build a working, codebase-aligned implementation.
# argument-hint: [run-id]
---

# Flow 3: Build

You are orchestrating Flow 3 of the SDLC swarm.

**Goal:** Build a working, codebase-aligned implementation. Tests pass. Diff is honest.

## Mental Model

**Flow 3 does not stop until the AC is verifiable and the code is clean.**

This is the Stubborn Loop: implement → test → critique → fix → repeat until the critics are satisfied. The implementer says "I'm done" — the critics verify or reject that claim. If rejected, the implementer goes again.

Flow 3 grabs external feedback (PR, CI, bots) when available to unblock the build. Route CRITICAL blockers immediately. Defer the full worklist to Flow 4.

## Working Directory + Paths

- All commands run from **repo root**
- All paths are **repo-root-relative**
- Run artifacts: `.runs/<run-id>/build/`
- Code/tests: project-defined locations

## Orchestration Model

**You direct agents and route on their responses.**

- Call agents - they do the work
- Listen to responses - agents tell you what happened via Result blocks
- Route on `status`, `recommended_action`, `route_to_*`
- Do not re-read files to make routing decisions

## Before You Begin

### State Machines

1. **TodoWrite** = session navigation (ephemeral)
2. **`flow_plan.md`** = durable on-disk state (enables reruns)

Create TodoWrite immediately. Write `flow_plan.md` after `run-prep` creates the run directory.

### On Rerun

If `.runs/<run-id>/build/` exists:
- Read `flow_plan.md` for navigation state
- **Call `build-cleanup`** to get AC completion status (every call is an implicit resume — the agent checks disk state)
- Route on the returned `Build Cleanup Result` block:
  - `ac_completed` / `ac_total` tells you where to resume
  - Do NOT parse `ac_status.json` directly — the agent owns that file
- Pre-mark completed items as done based on the agent's report

## The Build Loop

### Step 0: Infrastructure

**Call `run-prep`** to establish `.runs/<run-id>/build/`.

### Step 1: Git Prep

**Call `repo-operator`**: "ensure run branch `run/<run-id>`"

### Step 2: Load Context

**Call `context-loader`** to assemble the working set.

### Step 3: Clarify (Non-blocking)

**Call `clarifier`** to capture open questions. Document assumptions and continue.

### Step 4: AC Loop

Read `.runs/<run-id>/plan/ac_matrix.md` for the ordered AC list.

**If `ac_matrix.md` is missing:** Call `test-strategist` to generate it first.

**Note:** `build-cleanup` owns `ac_status.json` — it will create/update it based on test-executor results. The orchestrator does not touch this file directly.

**For each AC in order:**

1. **test-author**: Write tests for this AC
2. **test-critic**: Verify tests are solid
3. **code-implementer**: Implement to pass tests
4. **code-critic**: Verify implementation is honest
5. **test-executor**: Confirm tests pass (AC-scoped); emits `ac_status` in result block
6. **build-cleanup** (or `ac-tracker` if added): Updates `ac_status.json` based on test-executor result

**Note:** The orchestrator routes on `test-executor`'s result block. It does NOT parse `ac_status.json` directly. The cleanup agent owns state file updates.

**Adversarial Microloop (writer ↔ critic):**

The critic's job is to *find the flaw*. The writer's job is to *fix it*. This is not friendly peer review — it's adversarial verification.

```
writer → critic → [if RERUN] → writer → critic → ... → [PROCEED]
```

Route on the critic's Result block:
- `RERUN`: Send the worklist back to the writer
- `PROCEED`: Move forward (even if `status: UNVERIFIED` — blockers are documented)
- `can_further_iteration_help: no`: Stop iterating, proceed with blockers

**AC Termination (Law 4: Green + Orchestrator Agreement):**

An AC is complete when BOTH conditions are met:
1. **test-executor returns Green** for that AC's scope
2. **Orchestrator agrees** there's nothing left worth fixing

**"Green is a floor, not a ceiling."** Passing tests prove functional correctness. But professional code also needs maintainability.

**Post-Green Polish Pass Protocol:**

When `test-executor` returns Green, **read the latest `code-critic` report** before marking the AC complete:

1. **Check `code_critique.md`:** Does the critic identify:
   - Logic debt (fragile patterns, hidden coupling)
   - Maintainability risks (unclear naming, duplicated code)
   - Obvious improvements (missing error handling, unsafe patterns)

2. **Authorize one polish pass** if:
   - Critic identified concrete, fixable issues (not just stylistic preferences)
   - `can_further_iteration_help: yes`
   - The fix is scoped to the current AC (not architectural)

3. **Proceed without polish** if:
   - Critic says `can_further_iteration_help: no`
   - Issues are minor/stylistic (defer to Flow 4)
   - Issues require architectural changes (defer to Flow 4 or future work)

**The single polish pass rule:** One extra iteration to clean up what the critic found. Not an infinite loop of gold-plating. One pass, then proceed.

**After first vertical slice (AC-1 complete):**
1. Call `repo-operator`: checkpoint push
2. Call `pr-creator`: create Draft PR (gets bots spinning early)
3. Call `pr-feedback-harvester`: check for CRITICAL blockers only
   - Route top 1-3 blockers to appropriate agent
   - Continue AC loop (don't drain the full list - Flow 4 owns that)

**Checkpoint cadence:** Push after every 3-5 ACs. Feedback check after each push.

### Step 5: Global Hardening

After all ACs complete:

1. **standards-enforcer**: Format/lint + honest diff check
   - If `HIGH_RISK` (suspicious test deletion): proceed, but flag is visible to Gate
   - If `UNVERIFIED` (coherence issues): route to `code-implementer`

2. **test-executor**: Full suite (not AC-filtered)

3. **flakiness-detector**: If failures, classify deterministic vs flaky

4. **mutation-auditor**: Bounded mutation run on changed files
   - Route survivors to `test-author` or `fixer`

5. **fuzz-triager**: If configured, run bounded fuzz

6. **fixer**: Apply targeted fixes if critiques/worklists require it

### Step 6: Documentation

**doc-writer ↔ doc-critic** microloop

### Step 7: Self-Review

**Call `self-reviewer`** for final consistency check.

### Step 8: Flow Boundary Harvest

**Call `pr-feedback-harvester`** one last time (if PR exists):
- Route CRITICAL blockers only (bounded)
- Record unresolved items for Flow 4

### Step 9: Cleanup + Commit

1. **build-cleanup**: Write `build_receipt.json`, update index
2. **repo-operator**: Stage intended changes
3. **secrets-sanitizer**: Pre-publish sweep
4. **repo-operator**: Commit and push (if gates allow)
5. **gh-issue-manager** + **gh-reporter**: Update GitHub (if allowed)

### Step 10: Finalize

Update `flow_plan.md` with completion status.

## Routing Rules

Route on the Result block returned by each agent:

| `status` | `recommended_action` | What to do |
|----------|---------------------|------------|
| VERIFIED | PROCEED | Continue to next station |
| UNVERIFIED | PROCEED | Continue with blockers documented |
| UNVERIFIED | RERUN | Rerun the producer/writer |
| UNVERIFIED | BOUNCE | Route to `route_to_flow` / `route_to_agent` |
| CANNOT_PROCEED | FIX_ENV | Stop - mechanical failure |

If `recommended_action` is absent: use `can_further_iteration_help` as tie-breaker (`no` → proceed).

## Agents

**Infrastructure:**
- `run-prep` - establish run directory

**Git:**
- `repo-operator` - branch, stage, commit, push

**Context:**
- `context-loader` - curate working set
- `clarifier` - document ambiguities (non-blocking)

**Test loop:**
- `test-author` - write tests
- `test-critic` - verify tests

**Code loop:**
- `code-implementer` - implement code
- `code-critic` - verify implementation

**Hardening:**
- `test-executor` - run tests
- `standards-enforcer` - format/lint + honest diff check
- `flakiness-detector` - classify test failures
- `mutation-auditor` - mutation testing
- `fuzz-triager` - fuzz testing (if configured)
- `fixer` - targeted fixes

**Polish:**
- `doc-writer` - update docs
- `doc-critic` - review docs
- `self-reviewer` - final consistency check

**Cleanup:**
- `build-cleanup` - write receipt, update index
- `secrets-sanitizer` - pre-publish sweep
- `pr-creator` - create Draft PR
- `pr-feedback-harvester` - harvest bot/human feedback
- `gh-issue-manager` - update issue board
- `gh-reporter` - post summary to GitHub

## Upstream Inputs

Read from `.runs/<run-id>/plan/` (if available):
- `adr.md`, `api_contracts.yaml`, `schema.md`
- `test_plan.md`, `ac_matrix.md`, `work_plan.md`

**If upstream artifacts are missing:** Proceed best-effort, document assumptions, set status UNVERIFIED.

## Output Artifacts

After completion, `.runs/<run-id>/build/` contains:
- `flow_plan.md` - execution plan with checkboxes
- `ac_status.json` - AC completion tracker
- `test_changes_summary.md`, `test_critique.md`
- `impl_changes_summary.md`, `code_critique.md`
- `test_execution.md`, `standards_report.md`
- `mutation_report.md`, `flakiness_report.md` (if run)
- `doc_updates.md`, `doc_critique.md`
- `self_review.md`, `build_receipt.json`
- `pr_feedback.md`, `feedback_blockers.md` (if PR exists)

Plus code/test changes in project-defined locations.

## Status States

- **VERIFIED**: Tests pass, diff is honest, ready for Flow 4
- **UNVERIFIED**: Gaps documented, proceed with blockers
- **CANNOT_PROCEED**: Mechanical failure (IO/permissions/tooling)

## TodoWrite (copy exactly)

**These are the agents you call, in order. Do not group. Do not summarize. Execute each line.**

```
- [ ] run-prep
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] context-loader
- [ ] clarifier
- [ ] test-strategist (if ac_matrix.md missing)
- [ ] AC-1: test-author ↔ test-critic microloop
- [ ] AC-1: code-implementer ↔ code-critic microloop
- [ ] AC-1: test-executor (emits ac_status in result)
- [ ] repo-operator (checkpoint push)
- [ ] pr-creator (create Draft PR)
- [ ] pr-feedback-harvester (check CRITICAL only, route blockers)
- [ ] [repeat AC-2..N with same pattern]
- [ ] standards-enforcer (format/lint + suspicious deletion check)
- [ ] test-executor (full suite)
- [ ] flakiness-detector (if failures exist)
- [ ] mutation-auditor
- [ ] fuzz-triager (if configured)
- [ ] fixer (if critiques/mutation have worklist)
- [ ] doc-writer ↔ doc-critic microloop
- [ ] self-reviewer
- [ ] pr-feedback-harvester (flow boundary check)
- [ ] build-cleanup (writes ac_status.json + build_receipt.json)
- [ ] repo-operator (stage intended changes)
- [ ] secrets-sanitizer
- [ ] repo-operator (commit and push)
- [ ] gh-issue-manager
- [ ] gh-reporter
```

**Why explicit?** The orchestrator (you) executes what's in the list. Grouped phases get skipped. Explicit agents get called.

Use explore agents to answer immediate questions, then create the todo list and call agents.
