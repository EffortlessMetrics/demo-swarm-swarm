---
description: Run Flow 3 (Design -> Code): implement via adversarial microloops, self-verify, produce receipts.
# argument-hint: [subtask-id]
---

# Flow 3: Design -> Code

You are orchestrating Flow 3 of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/build/`
- Code/tests remain in project-defined locations (customize per repo layout).
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/build/` exists.

#### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Implement via adversarial microloops (test <-> critic, code <-> critic)
- Strengthen tests (unit/integration/mutation)
- Update docs
- Produce `build_receipt.json` and `self_review.md`

## Before You Begin (Required)

### Two State Machines

Flow 3 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Microloops (`test-author` ↔ `test-critic`, `code-implementer` ↔ `code-critic`, `doc-writer` ↔ `doc-critic`) are ONE todo each.

2. Mirror the same list into `.runs/<run-id>/build/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure)
- repo-operator (git prep; ensure run branch)
- context-loader (load context)
- clarifier (clarify ambiguities; non-blocking)
- test-strategist (if ac_matrix.md missing; generate before proceeding)
- AC loop: for each AC in ac_matrix.md (see AC Loop Template)
  - test-author (scope: current AC)
  - test-critic (scope: current AC)
  - code-implementer (scope: current AC)
  - code-critic (scope: current AC)
  - test-executor (fast confirm: AC-scoped tests only)
  - update build/ac_status.json
- lint-executor (format/lint; global)
- test-executor (full suite; global)
- flakiness-detector (if failures; apply Worklist Loop Template)
- mutation-auditor (mutation worklist; apply Worklist Loop Template)
- fuzz-triager (if configured; apply Worklist Loop Template)
- fixer (only if critiques/worklists require it)
- doc-writer ↔ doc-critic (microloop; 2 passes default)
- self-reviewer (self-review)
- build-cleanup (finalize receipt; update index; update `flow_plan.md`)
- repo-operator (stage intended changes)
- secrets-sanitizer (publish gate)
- build-cleanup ↔ secrets-sanitizer (reseal cycle; if `modified_files: true`)
- repo-operator (restage intended changes; if reseal happened)
- repo-operator (commit/push; only if secrets gate passes)
- pr-creator (create Draft PR; gated on push)
- secrets-sanitizer + repo-operator (commit PR metadata; if PR created)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report to GitHub; gated)
```

### On Rerun

If running `/flow-3-build` on an existing run-id:
- Read `.runs/<run-id>/build/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

If you encounter ambiguity, **document it and continue**. Write assumptions in artifacts.

## Subagents to use

**Infrastructure (Step 0)**:
- **run-prep** -- MUST be called first to establish the run directory and `.runs/<run-id>/build/`

**Git operations (cross-cutting)**:
- repo-operator -- branch at start, commit at end

**Context loading**:
- context-loader -- load relevant files for subtask

**Test microloop**:
- test-author -- write/update tests
- test-critic -- harsh review of tests (never fixes)

**Code microloop**:
- code-implementer -- implement code to pass tests
- code-critic -- harsh review of code (never fixes)

**Hardening**:
- flakiness-detector -- rerun failing tests; tag deterministic vs flaky vs env/tooling (conditional)
- mutation-auditor -- bounded mutation run + prioritized survivor worklist (routes to test-author/fixer)
- fuzz-triager -- optional fuzz run + crash triage (config-present ⇒ run)
- fixer -- apply targeted fixes from critiques
- lint-executor -- format/lint codebase
- test-executor -- rerun tests to confirm clean state

**Polish and wrap-up**:
- doc-writer -- update documentation
- doc-critic -- critique docs (no edits; routable worklist)
- self-reviewer -- final review (`self_review.md`)

**Cross-cutting agents**:
- clarifier -- detect ambiguities in specs/design, document assumptions
- test-strategist -- generate AC matrix if missing (called before AC loop starts)

**Cleanup + Reporting (End of Flow)**:
- build-cleanup -- writes build_receipt.json, updates index.json status
- secrets-sanitizer -- publish gate (scans staged changes before commit)
- repo-operator -- commit only after secrets gate passes
- pr-creator -- create Draft PR after push (gets bots spinning early)
- gh-issue-manager -- updates issue body status board
- gh-reporter -- post summary to GitHub

## Upstream Inputs

Read from `.runs/<run-id>/plan/` (if available):
- `adr.md`
- `api_contracts.yaml`
- `schema.md`
- `test_plan.md`
- `ac_matrix.md` (AC-driven build contract - Flow 3 iterates per AC; read-only)
- `work_plan.md`

**If upstream artifacts are missing**: Flow 3 can start without Flows 1-2. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue. This enables flexibility for hotfixes or code-first workflows.

**If `ac_matrix.md` is missing**: Investigate why. If Flow 2 ran but didn't produce it, call `test-strategist` directly to generate it from available inputs (requirements, BDD scenarios). If no upstream inputs exist, call `test-strategist` to produce a minimal AC matrix from the task description. Do not proceed without an AC matrix.

## Orchestration outline

### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/build/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "build" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/build/`.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/build/flow_plan.md`:

```markdown
# Flow 3: Build for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (git prep)
- [ ] context-loader (load context)
- [ ] clarifier (document ambiguities)
- [ ] test-strategist (if ac_matrix.md missing)
- [ ] AC loop (for each AC in ac_matrix.md):
  - [ ] AC-001: test-author → test-critic → code-implementer → code-critic → fast confirm
  - [ ] AC-002: ...
  - [ ] (add rows per AC from ac_matrix.md)
- [ ] lint-executor (format/lint; global)
- [ ] test-executor (full suite; global)
- [ ] flakiness-detector (if failures)
- [ ] mutation-auditor (mutation worklist)
- [ ] fuzz-triager (optional; config-present ⇒ run)
- [ ] fixer (targeted; only if critiques/worklists require it)
- [ ] doc-writer ↔ doc-critic (microloop; 2 passes default)
- [ ] self-reviewer (review)
- [ ] build-cleanup (write receipt, update index)
- [ ] repo-operator (stage changes)
- [ ] secrets-sanitizer (publish gate)
- [ ] build-cleanup reseal + repo-operator restage (if modified)
- [ ] repo-operator (commit - if gate passes)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## AC Progress

| AC-ID | Tests Written | Tests Pass | Code Done | Code Reviewed | Status |
|-------|---------------|------------|-----------|---------------|--------|
| AC-001 | | | | | pending |
| AC-002 | | | | | pending |

## Progress Notes

<Update as each step completes>
```

### Step 2: Git prep

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent ensures clean tree and handles branch creation/switching safely. This keeps checkpoint commits off main.

### Step 3: Load context
- Use `context-loader` to assemble the working set.

### Step 4: Clarify
- Use `clarifier` to capture open questions (non-blocking).
- Scan ADR, contracts, and loaded context for ambiguities.
- Document assumptions being made to proceed.
- Continue regardless of questions found.

### Step 5: AC Loop (per-AC microloops)

**Flow 3 executes test/code microloops per AC (within a single build run).**

Read `.runs/<run-id>/plan/ac_matrix.md` to get the ordered list of ACs. Initialize `.runs/<run-id>/build/ac_status.json` before starting the loop.

**If `ac_matrix.md` is missing:** Call `test-strategist` to generate it before proceeding (see Upstream Inputs).

**Initialize `build/ac_status.json` (before first AC):**

If `build/ac_status.json` does not exist, create it from `ac_matrix.md`:

```json
{
  "schema_version": "ac_status_v1",
  "run_id": "<run-id>",
  "ac_count": <number of ACs in ac_matrix.md>,
  "completed": 0,
  "in_progress": null,
  "items": [
    {
      "ac_id": "AC-001",
      "status": "pending",
      "tests_written": false,
      "code_implemented": false,
      "code_reviewed": false,
      "tests_passing": false,
      "files_touched": [],
      "evidence": []
    }
  ]
}
```

If `build/ac_status.json` exists (rerun scenario), read it and resume from the first non-completed AC.

#### AC Loop Iteration (for each AC in order)

For each AC (e.g., AC-001):

1. **Scope context**: Pass the AC-ID, description, test types, and impl hints to each agent

2. **test-author ↔ test-critic microloop** (scope: this AC only; apply Microloop Template):
   - test-author writes tests tagged/named with AC-ID
   - test-critic confirms tests exercise this AC
   - Apply pass if critic returns `recommended_action: RERUN`
   - Re-critique, then proceed (2 passes default)

3. **code-implementer ↔ code-critic microloop** (scope: this AC only; apply Microloop Template):
   - code-implementer implements for this AC
   - code-critic reviews against ADR/contracts
   - Apply pass if critic returns `recommended_action: RERUN`
   - Re-critique, then proceed (2 passes default)

4. **test-executor** (fast confirm: run only tests for this AC, not full suite)

5. **Update build/ac_status.json**: Mark AC as `completed` if all checks pass, `blocked` if critic issues remain

**Route on critic Result blocks** (same rules as Microloop Template):
- If `status: CANNOT_PROCEED` -> **FIX_ENV**; stop AC loop
- If `recommended_action: BOUNCE` -> follow `route_to_flow/route_to_agent`; stop AC loop
- If `recommended_action: RERUN` -> apply pass within this AC's microloop
- If `recommended_action: PROCEED` -> proceed to next step/AC

**Termination per AC:** Each microloop follows the 2-pass default. Continue beyond that only when critic returns `recommended_action: RERUN` and `can_further_iteration_help: yes`.

**After all ACs complete:** Proceed to global hardening (Step 6).

### Step 6: Global Hardening

After all ACs complete, run global hardening:

- Run `lint-executor` to format/lint the codebase (capture a lint report).
- Run `test-executor` to run the **full test suite** (not per-AC filtered; captures coverage).
- If tests fail or look unstable, run `flakiness-detector` and route on its Result block.
- If tests are green, run `mutation-auditor` (bounded) to produce a prioritized survivor worklist (route to `test-author` or `fixer`).
- If `demo-swarm.config.json` defines a fuzz harness, run `fuzz-triager` (bounded) and route crashes.
- Apply targeted fixes with `fixer` only when a critique/worklist calls for it, then rerun `test-executor`.

Treat each hardening station as a **bounded worklist loop**: hardener -> fix lane -> hardener (confirm once, budgeted).

Gate may perform one fix-forward pass for deterministic mechanical hygiene; Build remains the owner of semantic fixes.

### Step 7: doc-writer ↔ doc-critic (microloop)
Loop between `doc-writer` and `doc-critic` (2 passes default):
- Call `doc-writer` to update documentation/docstrings (no behavior changes)
- Call `doc-critic` to critique docs for staleness and verification realism (no edits)

**Route on the Doc Critic Result block** (not by re-reading the file):
- If `status: CANNOT_PROCEED` -> **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention
- If `recommended_action: BOUNCE` -> follow `route_to_flow/route_to_agent`
- If `recommended_action: RERUN` -> do the apply pass: rerun `doc-writer` once with the critique worklist, then rerun `doc-critic` once; proceed after the second critique even if still UNVERIFIED (carry blockers honestly)
- If `recommended_action: PROCEED` -> proceed after the re-critique pass (even if UNVERIFIED)
- If `recommended_action` is absent: use `can_further_iteration_help` as a tie-breaker (`yes` -> rerun; `no` -> proceed)

### Step 8: Self-review
- Use `self-reviewer` for final review.

### Step 9: Finalize and Write Receipt
- `build-cleanup` -> `build_receipt.json`, `cleanup_report.md`
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Updates `.runs/index.json` with status, last_flow, updated_at

### Step 10: Stage Changes

**Call `repo-operator`** with task: "stage intended changes for build"

**Commit surface for Build:**
- Code/test/doc changes (project-defined locations)
- `.runs/<run-id>/build/` (all flow artifacts)
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

The repo-operator handles staging; customize the intended changes set via the project customizer.

**Dirty-tree interlock:** The repo-operator enforces "no unstaged + no untracked" after staging. If the interlock fails → **anomaly**:
- **Call `repo-operator`** with task: "reconcile staging anomaly"
  - Produces `.runs/<run-id>/build/git_status.md` listing unexpected paths
  - Classifies each path: agent wrote outside lane | leftover from prior flow | temp/debug file | legitimate for this flow
  - Applies safe mechanical actions only (delete temp files, add OS junk to .gitignore)
  - Returns recommended action for paths it cannot safely handle
- If safe actions resolve the anomaly → retry staging
- If unsafe changes remain → flow ends UNVERIFIED with `proceed_to_github_ops: false`
- Do **not** proceed to secrets-sanitizer, commit, push, or GitHub ops until clean

**IMPORTANT: Do NOT commit yet. Must pass secrets gate first.**

### Step 11: Sanitize Secrets (Commit Gate)
- Use `secrets-sanitizer` to scan staged changes and audit artifacts.
- Scans staged changes, .runs/ artifacts
- Fixes what it can (redacts artifacts, externalizes secrets)

**Status vs Flags:**
- `status` = what happened:
  - `CLEAN`: No secrets found
  - `FIXED`: Secrets found and remediated
  - `BLOCKED_PUBLISH`: Mechanical failure (IO/permissions)
- `safe_to_commit/safe_to_publish` = what you're allowed to do (authoritative)
- `needs_upstream_fix` + `route_to_agent` (and optionally `route_to_flow`) = where to bounce

Typically `safe_to_*` are true for CLEAN/FIXED, but **the orchestrator must use the Gate Result booleans, not infer from status**.

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

**Gating logic (route-and-fix triage):**
- If `safe_to_commit: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`), usually `code-implementer`, with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → UNVERIFIED; write evidence, do not commit/push
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`. GitHub reporting ops still run in RESTRICTED mode when publish is blocked or `publish_surface: NOT_PUSHED`.

### Step 11b: Reseal If Modified (Conditional Loop)

If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(build-cleanup → secrets-sanitizer)` until either:
- the sanitizer reports `modified_files: false`, or
- the sanitizer indicates no reasonable path to fixing (non-convergent).

If reseal cannot make progress (sanitizer signals no reasonable path):
- Append an evidence note to `secrets_scan.md`:
  - "modified_files remained true; sanitizer reports no viable path to fix; stopping to prevent receipt drift."
- If Gate Result `safe_to_commit: true`: call `repo-operator` with `checkpoint_mode: local_only`
  - it must return `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`
- GitHub ops: obey the access gate. If `github_ops_allowed: false` or `gh` is unauthenticated, **skip** and write local status. Otherwise run in **RESTRICTED** mode (paths only) and use only receipt-derived machine fields (`status`, `recommended_action`, `counts.*`, `quality_gates.*`). Publish block reason must be explicit.
- Flow outcome: `status: UNVERIFIED`, `recommended_action: PROCEED`
  - If Gate Result `needs_upstream_fix: true`, use `recommended_action: BOUNCE` and the provided `route_to_*`.

**Note:** `checkpoint_mode: local_only` mechanically enforces `proceed_to_github_ops: false`, ensuring safe-bail cannot accidentally push even if `safe_to_publish` is true.

### Step 11c: Restage After Reseal (Conditional)

**If any reseal occurred (Step 11b ran at least once), restage all changes.**

This is critical: `secrets-sanitizer` may modify tracked files (code, config, `.runs/` artifacts) and `build-cleanup` reseal writes new receipt + updates `index.json`. These edits are **not staged** unless you stage again.

**Call `repo-operator`** with task: "restage intended changes" — the agent handles staging per project layout.

**Why this matters:** Without restage, "receipt says X, commit contains Y" — the committed receipt won't match the actual staged state.

### Step 12: Commit and Push (Only if Secrets Gate Passes)

**Call `repo-operator`** to commit code/test changes + audit trail. The agent generates an appropriate commit message from `impl_changes_summary.md`.

**No-op commit guard:** If `git diff --cached --quiet` → commit SKIPPED (not an error), do not push. `repo-operator` handles this gracefully.

**Control plane:** `repo-operator` returns a Repo Operator Result block:
```
## Repo Operator Result
operation: build
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```
**Note:** `commit_sha` is always populated (current HEAD on no-op), never null. Flow 3 uses `operation: build` (not `checkpoint`) because it commits code/tests alongside audit artifacts.

Orchestrators route on this block, not by re-reading `git_status.md`.

**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
- If `safe_to_commit: false` (from Gate Result): skip commit (apply route-and-fix triage from Step 11)
- If `safe_to_commit: true`: commit
- If anomaly detected (dirty tree after staging): commits staged changes, returns `proceed_to_github_ops: false`

**Push logic:**
- If `safe_to_publish: true` AND `proceed_to_github_ops: true`: repo-operator pushes the branch
- If `safe_to_publish: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`), usually `code-implementer`, with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → UNVERIFIED; skip push (`publish_surface: NOT_PUSHED`), write evidence

### Step 12b: Create Draft PR

**Call `pr-creator`** to create a Draft PR after the branch is pushed.

This gets bots (CodeRabbit, CI) spinning early before Flow 4 (Review) harvests their feedback.

**Prerequisites (checked by pr-creator):**
- `github_ops_allowed: true` in run_meta
- `gh` authenticated
- `publish_surface: PUSHED` (from Repo Operator Result)

**Call `pr-creator`** with context:
- run-id
- github_repo from run_meta
- Repo Operator Result (for commit_sha and publish_surface)

**Route on PR Creator Result block:**
- If `operation_status: CREATED`: PR created successfully, `pr_number` available
- If `operation_status: EXISTING`: PR already existed, `pr_number` captured
- If `operation_status: SKIPPED`: Prerequisites not met (branch not pushed, auth issue), note reason and continue
- If `operation_status: FAILED`: Creation failed, note in concerns and continue

**Metadata updates (handled by pr-creator):**
- `run_meta.json`: `pr_number`, `pr_url`
- `index.json`: `pr_number`

**Artifact output:**
- `.runs/<run-id>/build/pr_creation_status.md`

**Important:** PR creation failure does not block the flow. Flow 4 (Review) can create the PR if needed.

### Step 12c: Commit PR Metadata (Conditional)

**If PR was created or found (operation_status: CREATED or EXISTING):**

The `run_meta.json` and `index.json` now contain `pr_number` but this isn't on the branch yet. Commit this metadata so the run state is complete.

1. **Stage metadata only:**
   - `.runs/<run-id>/run_meta.json`
   - `.runs/<run-id>/build/pr_creation_status.md`
   - `.runs/index.json`

2. **Call `secrets-sanitizer`** on the staged surface (likely CLEAN, but required for gate consistency).

3. **Call `repo-operator`** with task: "commit PR metadata"
   - Commit message: "chore(<run-id>): add PR metadata"
   - Push if `safe_to_publish: true`

**If PR was skipped or failed:** Skip this step (nothing to commit).

### GitHub Access + Content Mode (canonical)

See `CLAUDE.md` → **GitHub Access + Content Mode (Canonical)**.

- Publish blocked → `RESTRICTED` (never skip when access is allowed)
- `FULL` only when `safe_to_publish: true` AND `proceed_to_github_ops: true` AND `publish_surface: PUSHED`

### Step 13: Update Issue Board

Apply Access + Content Mode rules:
- Skip GitHub calls if `github_ops_allowed: false` or `gh` unauthenticated (record SKIPPED/UNVERIFIED).
- Otherwise derive `FULL` vs `RESTRICTED` from gates + publish surface. Publish blocked reasons must be explicit; RESTRICTED uses paths only and the receipt allowlist.

`gh-issue-manager` updates issue body status board from receipt. If the issue is missing and gh is available, it may create it (with a Signal-pending banner when created from Flow 3).

### Step 14: Report to GitHub

Apply Access + Content Mode rules:
- Skip only when `github_ops_allowed: false` or `gh` unauthenticated (record SKIPPED/UNVERIFIED).
- Otherwise post in `FULL` only when `safe_to_publish: true`, `proceed_to_github_ops: true`, and `publish_surface: PUSHED`; use `RESTRICTED` for all other cases (paths only, receipt allowlist, no human-authored markdown).

`gh-reporter` writes `.runs/<run-id>/build/github_report.md` locally and posts to the issue (never PR). Issue-first (hard): flow logs go to the issue even if a PR exists. PRs are for PR-review dynamics only.

**Content expectations:** The gh-reporter comment should include:
- Decisions Needed (unanswered open questions requiring human input)
- Concerns for Review (critic findings, test failures, HIGH risks)
- Agent Notes (substantive observations: friction noticed, cross-cutting insights, pack improvements)

These make the GitHub update actionable - humans can make decisions without leaving GitHub.

### Step 15: Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Tests**: <pass/fail counts from build_receipt.json>
- **Mutation Score**: <score from mutation_report.md>
- **Next Flow**: `/flow-4-review` (after human review)

## Human Review Checklist

Before proceeding to Flow 4, humans should review:
- [ ] `test_critique.md` - Are test concerns addressed?
- [ ] `code_critique.md` - Are code concerns addressed?
- [ ] `self_review.md` - Is the implementation complete?
- [ ] Git diff - Are the changes what you expected?
```

## Status States

Agents report one of:
- **VERIFIED**: `blockers` empty, `missing_required` empty, and all quality gates passed; work is adequate for its purpose. Set `recommended_action: PROCEED`.
- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR any quality gate UNVERIFIED; concrete concerns documented. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read/write files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.

Use critic status to decide whether to loop or proceed.

## Microloop Termination

Termination is driven by the critic control plane:
- `status: CANNOT_PROCEED` -> stop (FIX_ENV)
- `recommended_action: BOUNCE` -> bounce to `route_to_*`
- **Default cadence (2 passes):** producer -> critic -> producer -> critic
- `recommended_action: RERUN` -> run the apply pass, then re-run the critic once; proceed after the second critique even if still UNVERIFIED (carry blockers honestly)
- `recommended_action: PROCEED` or no actionable rerun/bounce -> proceed (log blockers/assumptions if UNVERIFIED)
- If no `recommended_action`, use `can_further_iteration_help` as tie-breaker (`no` -> proceed; `yes` -> rerun)

## Output Artifacts

After this flow completes, `.runs/<run-id>/build/` should contain:
- `flow_plan.md`
- `subtask_context_manifest.json`
- `open_questions.md`
- `test_changes_summary.md`
- `test_critique.md`
- `lint_report.md`
- `test_execution.md`
- `flakiness_report.md` (if run)
- `impl_changes_summary.md`
- `code_critique.md`
- `mutation_report.md`
- `fuzz_report.md` (if run)
- `fix_summary.md`
- `doc_updates.md`
- `doc_critique.md`
- `self_review.md`
- `build_receipt.json`
- `cleanup_report.md`
- `secrets_scan.md`
- `secrets_status.json`
- `gh_issue_status.md`
- `gh_report_status.md`
- `github_report.md`
- `git_status.md` (if anomaly detected)
- `pr_creation_status.md` (from pr-creator)

Also creates in `.runs/<run-id>/build/`:
- `ac_status.json` (runtime AC completion tracker - created by Build, updated per AC)

Code/test changes in project-defined locations.

---

## Orchestrator Kickoff

### Station order + templates

#### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `context-loader`

4. `clarifier`

5. `test-strategist` (if `ac_matrix.md` missing; generate before proceeding)

6. **AC loop** (for each AC in `ac_matrix.md`; apply AC Loop Template below)

7. `lint-executor` (global)

8. `test-executor` (full suite; global)

9. `flakiness-detector` (if failures; routes to `test-author`/`code-implementer`/`fixer`; confirm via `test-executor`; apply Worklist Loop Template)

10. `mutation-auditor` (if tests green; routes to `test-author`/`fixer`; confirm via `test-executor`; apply Worklist Loop Template)

11. `fuzz-triager` (if configured; routes to `code-implementer`/`fixer`; confirm via `test-executor`; apply Worklist Loop Template)

12. `doc-writer` ↔ `doc-critic` (microloop; apply Microloop Template)

13. `self-reviewer`

14. `build-cleanup`

15. `repo-operator` (stage)

16. `secrets-sanitizer`

17. `build-cleanup` ↔ `secrets-sanitizer` (reseal cycle; if `modified_files: true`)

18. `repo-operator` (restage if reseal happened)

19. `repo-operator` (commit/push)

20. `pr-creator` (create Draft PR; gated on publish_surface: PUSHED)

21. `secrets-sanitizer` + `repo-operator` (commit PR metadata; if PR created/found)

22. `gh-issue-manager` (if allowed)

23. `gh-reporter` (if allowed)

#### Microloop Template (writer ↔ critic)

Run this template for: tests, code, docs, requirements, BDD, options, contracts, observability.

1) Writer pass: call `<writer>`
2) Critique pass: call `<critic>` and read its control-plane Result
3) Apply pass (default second writer pass): call `<writer>` once using the critic's worklist (no-op if the critic returned `recommended_action: PROCEED`)
4) Re-critique: call `<critic>` again

Continue looping beyond the default two passes only when:
- critic returns `recommended_action: RERUN`, and
- `can_further_iteration_help: yes`, and
- the remaining items are concrete and writer-addressable (a new writer pass can plausibly clear them).

Otherwise proceed with `UNVERIFIED` + blockers recorded.

#### AC Loop Template (per-AC microloops)

**Purpose:** Execute test/code microloops scoped to each Acceptance Criterion, ensuring complete coverage.

**Prerequisites:**
- Read `.runs/<run-id>/plan/ac_matrix.md` for the ordered AC list
- Initialize or read `.runs/<run-id>/build/ac_status.json`

**Initialization (before first AC):**

If `build/ac_status.json` does not exist:
- Parse `ac_matrix.md` to extract AC count and AC IDs
- Create `build/ac_status.json` with schema:
  - `schema_version: "ac_status_v1"`
  - `run_id`, `ac_count`, `completed: 0`, `in_progress: null`
  - `items[]` with one entry per AC (all `status: "pending"`)

If `build/ac_status.json` exists (rerun):
- Load it and resume from the first non-completed AC
- Preserve prior `completed` tally and item states

**For each AC (in Implementation Order from ac_matrix.md):**

1. **Mark in-progress:** Update `build/ac_status.json` with `"status": "in_progress"` and `"in_progress": "<AC-ID>"`

2. **Scope context:** Extract from ac_matrix.md for this AC:
   - AC-ID, Description, Priority
   - Test Types (what kinds of tests to write)
   - Impl Hints (which modules/files to modify)
   - Verification (what assertions confirm success)

3. **test-author ↔ test-critic microloop** (scope: this AC only; apply Microloop Template):
   - test-author: Pass AC-ID, Description, Test Types, Verification; tests tagged with AC-ID
   - test-critic: Confirms tests actually exercise this specific AC
   - Apply pass if critic returns `recommended_action: RERUN`
   - Re-critique, then proceed (2 passes default)
   - Update `build/ac_status.json`: `"tests_written": true`

4. **code-implementer ↔ code-critic microloop** (scope: this AC only; apply Microloop Template):
   - code-implementer: Pass AC-ID, Description, Impl Hints, test file locations
   - code-critic: Reviews code for this AC against ADR/contracts
   - Apply pass if critic returns `recommended_action: RERUN`
   - Re-critique, then proceed (2 passes default)
   - Update `build/ac_status.json`: `"code_implemented": true`, `"code_reviewed": true`, `"files_touched": [...]`

5. **test-executor** (fast confirm: AC-scoped tests only)
   - Run filtered test subset (by AC-ID tag/name)
   - Update `build/ac_status.json`: `"tests_passing": true/false`

6. **Finalize AC:**
   - If all checks pass: `"status": "completed"`, increment `"completed"` count
   - If critic issues remain: `"status": "blocked"`, document in `"evidence"`
   - Clear `"in_progress": null`

**Termination per AC:** Each microloop follows the 2-pass default. Continue beyond that only when critic returns `recommended_action: RERUN` and `can_further_iteration_help: yes`.

**After all ACs:** Proceed to global hardening (lint-executor, full test-executor, mutation, etc.)

#### Worklist Loop Template (producer → fix lane → confirm)

1) Run the producer (`mutation-auditor` / `fuzz-triager` / `flakiness-detector`)
2) If it returns `recommended_action: RERUN` or a worklist that routes to an agent:
   - call the routed agent once (`test-author` / `code-implementer` / `fixer`)
3) Confirm once: rerun the producer one time to verify the top items moved.
4) If still UNVERIFIED, proceed with blockers unless the producer says another pass will help and the fix lane can actually address it.

### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] context-loader
- [ ] clarifier
- [ ] test-strategist (if ac_matrix.md missing)
- [ ] AC loop (for each AC in ac_matrix.md; apply AC Loop Template)
- [ ] lint-executor (global)
- [ ] test-executor (full suite; global)
- [ ] flakiness-detector (if failures)
- [ ] mutation-auditor
- [ ] fuzz-triager (if configured)
- [ ] fixer (only if critiques/worklists require it)
- [ ] doc-writer ↔ doc-critic (microloop; 2 passes default)
- [ ] self-reviewer
- [ ] build-cleanup
- [ ] repo-operator (stage intended changes; project-defined)
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] build-cleanup ↔ secrets-sanitizer (reseal cycle; if `modified_files: true`)
- [ ] repo-operator (restage intended changes; if reseal occurred)
- [ ] repo-operator (commit/push; return Repo Operator Result)
- [ ] pr-creator (create Draft PR; gated on publish_surface: PUSHED)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
