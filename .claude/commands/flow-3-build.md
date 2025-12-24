---
description: Run Flow 3 (Design -> Code): build a working, codebase-aligned implementation.
# argument-hint: [run-id]
---

# Flow 3: Build

You are orchestrating Flow 3 of the SDLC swarm.

**Goal:** Build a working, codebase-aligned implementation. Tests pass. Diff is honest.

## Mental Model

Flow 3 builds. Flow 4 polishes.

- **Flow 3**: Create working code that satisfies the acceptance criteria
- **Flow 4**: Handle PR feedback, refine maintainability, respond to reviewers

The implementer can decide "I'm done" - we verify that claim via tests and critics. This isn't paranoid supervision; it's normal engineering: build → verify → ship.

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
- Read `flow_plan.md` and `ac_status.json`
- Resume from the first incomplete item
- Pre-mark completed items as done

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

**Initialize `ac_status.json`** before starting (or read existing on rerun).

**For each AC in order:**

1. **test-author**: Write tests for this AC
2. **test-critic**: Verify tests are solid
3. **code-implementer**: Implement to pass tests
4. **code-critic**: Verify implementation is honest
5. **test-executor**: Confirm tests pass (AC-scoped)
6. **Update `ac_status.json`**: Mark AC complete or blocked

**Microloop pattern (writer ↔ critic):**
- Writer pass → Critic pass → Apply pass (if critic says RERUN) → Re-critique
- 2 passes default. Continue only if critic says `recommended_action: RERUN` and `can_further_iteration_help: yes`
- Proceed with blockers documented if critic says PROCEED

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

**doc-writer ↔ doc-critic** microloop (2 passes default)

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

## TodoWrite Template

```
- [ ] run-prep
- [ ] repo-operator (ensure run branch)
- [ ] context-loader
- [ ] clarifier
- [ ] test-strategist (if ac_matrix.md missing)
- [ ] AC loop (for each AC)
  - [ ] (after first slice) checkpoint + pr-creator + feedback check
- [ ] standards-enforcer
- [ ] test-executor (full suite)
- [ ] flakiness-detector (if failures)
- [ ] mutation-auditor
- [ ] fuzz-triager (if configured)
- [ ] fixer (if needed)
- [ ] doc-writer ↔ doc-critic
- [ ] self-reviewer
- [ ] pr-feedback-harvester (boundary)
- [ ] build-cleanup
- [ ] repo-operator (stage)
- [ ] secrets-sanitizer
- [ ] repo-operator (commit/push)
- [ ] gh-issue-manager
- [ ] gh-reporter
```

Use explore agents to answer immediate questions, then create the todo list and call agents.
