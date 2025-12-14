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
   - Microloops (test author/critic, code implementer/critic) are ONE todo each.

2. Mirror the same list into `.runs/<run-id>/build/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- Establish run infrastructure (run-prep)
- Git prep (repo-operator branch)
- Load context (context-loader)
- Clarify ambiguities (clarifier)
- Tighten tests microloop (test-author/test-critic)
- Tighten code microloop (code-implementer/code-critic)
- Harden via mutation (mutator/fixer)
- Format/lint (lint-executor)
- Re-verify tests (test-executor)
- Polish docs (doc-writer)
- Self-review (self-reviewer)
- Finalize receipt (build-cleanup)
- Stage changes + sanitize secrets (repo-operator stage, secrets-sanitizer)
- Reseal + restage (if sanitizer modified files)
- Commit changes (repo-operator commit - only if secrets gate passes)
- Update issue board (gh-issue-manager; gated—see Orchestrator Kickoff)
- Report to GitHub (gh-reporter; gated—see Orchestrator Kickoff)
- Update flow_plan.md summary
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
- mutator -- run mutation tests
- fixer -- apply targeted fixes from critiques
- lint-executor -- format/lint codebase
- test-executor -- rerun tests to confirm clean state

**Polish and wrap-up**:
- doc-writer -- update documentation
- self-reviewer -- final review (`self_review.md`)

**Cross-cutting agents**:
- clarifier -- detect ambiguities in specs/design, document assumptions

**Cleanup + Reporting (End of Flow)**:
- build-cleanup -- writes build_receipt.json, updates index.json status
- secrets-sanitizer -- publish gate (scans staged changes before commit)
- repo-operator -- commit only after secrets gate passes
- gh-issue-manager -- updates issue body status board
- gh-reporter -- post summary to GitHub

## Upstream Inputs

Read from `.runs/<run-id>/plan/` (if available):
- `adr.md`
- `api_contracts.yaml`
- `schema.md`
- `test_plan.md`
- `work_plan.md`

**If upstream artifacts are missing**: Flow 3 can start without Flows 1-2. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue. This enables flexibility for hotfixes or code-first workflows.

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
- [ ] test-author / test-critic (microloop)
- [ ] code-implementer / code-critic (microloop)
- [ ] mutator / fixer (hardening)
- [ ] lint-executor (format/lint)
- [ ] test-executor (re-verify tests)
- [ ] doc-writer (polish)
- [ ] self-reviewer (review)
- [ ] build-cleanup (write receipt, update index)
- [ ] repo-operator (stage changes)
- [ ] secrets-sanitizer (publish gate)
- [ ] build-cleanup reseal + repo-operator restage (if modified)
- [ ] repo-operator (commit - if gate passes)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

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

### Step 5: Tighten tests (microloop)
Loop between `test-author` and `test-critic`:
- Call `test-author` to write/update tests
- Call `test-critic` to review them

**Route on the Test Critic Result block** (not by re-reading the file):
- If `status: VERIFIED` → proceed to code
- If `status: UNVERIFIED` AND `can_further_iteration_help: yes` → route back to `test-author` with specific feedback
- If `status: UNVERIFIED` AND `can_further_iteration_help: no` → proceed (remaining issues not addressable within scope)
- If `status: CANNOT_PROCEED` → **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention

**Loop guidance**: The Result block is the control plane; `test_critique.md` is the audit artifact.

### Step 6: Tighten code (microloop)
Loop between `code-implementer` and `code-critic`:
- Call `code-implementer` to implement behavior
- Call `code-critic` to review code

**Route on the Code Critic Result block** (not by re-reading the file):
- If `status: VERIFIED` → proceed to hardening
- If `status: UNVERIFIED` AND `can_further_iteration_help: yes` → route back to `code-implementer` with specific feedback
- If `status: UNVERIFIED` AND `can_further_iteration_help: no` → proceed (remaining issues not addressable within scope)
- If `status: CANNOT_PROCEED` → **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention

**Loop guidance**: The Result block is the control plane; `code_critique.md` is the audit artifact.

### Step 7: Harden
- Use `mutator` to run mutation tests.
- Apply targeted fixes with `fixer`.
- Run `lint-executor` to format/lint the codebase (capture a lint report).
- Run `test-executor` to re-verify tests deterministically (capture a test report).

Gate may perform one fix-forward pass for deterministic mechanical hygiene; Build remains the owner of semantic fixes.

### Step 8: Polish
- Use `doc-writer` to polish docs.

### Step 9: Self-review
- Use `self-reviewer` for final review.

### Step 10: Finalize and Write Receipt
- `build-cleanup` -> `build_receipt.json`, `cleanup_report.md`
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Updates `.runs/index.json` with status, last_flow, updated_at

### Step 11: Stage Changes

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

### Step 12: Sanitize Secrets (Commit Gate)
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
recommended_action: PROCEED | RERUN | BOUNCE | ESCALATE | FIX_ENV
route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | null
route_to_agent: <agent-name> | null
```
<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->

**Gating logic (route-and-fix triage):**
- If `safe_to_commit: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`), usually `code-implementer`, with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → UNVERIFIED; write evidence, skip external ops
- If `safe_to_publish: false` → Do NOT push or post to GitHub (apply same triage)

### Step 12b: Reseal If Modified (Conditional Loop)

If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(build-cleanup → secrets-sanitizer)` until either:
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

**Note:** `checkpoint_mode: local_only` mechanically enforces `proceed_to_github_ops: false`, ensuring safe-bail cannot accidentally push even if `safe_to_publish` is true.

### Step 12c: Restage After Reseal (Conditional)

**If any reseal occurred (Step 12b ran at least once), restage all changes.**

This is critical: `secrets-sanitizer` may modify tracked files (code, config, `.runs/` artifacts) and `build-cleanup` reseal writes new receipt + updates `index.json`. These edits are **not staged** unless you stage again.

**Call `repo-operator`** with task: "restage intended changes" — the agent handles staging per project layout.

**Why this matters:** Without restage, "receipt says X, commit contains Y" — the committed receipt won't match the actual staged state.

### Step 13: Commit and Push (Only if Secrets Gate Passes)

**Call `repo-operator`** to commit code/test changes + audit trail. The agent generates an appropriate commit message from `impl_changes_summary.md`.

**No-op commit guard:** If `git diff --cached --quiet` → commit SKIPPED (not an error), do not push. `repo-operator` handles this gracefully.

**Control plane:** `repo-operator` returns a Repo Operator Result block:
```
## Repo Operator Result
operation: build
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
anomaly_paths: []
```
**Note:** `commit_sha` is always populated (current HEAD on no-op), never null. Flow 3 uses `operation: build` (not `checkpoint`) because it commits code/tests alongside audit artifacts.

Orchestrators route on this block, not by re-reading `git_status.md`.

**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
- If `safe_to_commit: false` (from Gate Result): skip commit (apply route-and-fix triage from Step 12)
- If `safe_to_commit: true`: commit
- If anomaly detected (dirty tree after staging): commits staged changes, returns `proceed_to_github_ops: false`

**Push logic:**
- If `safe_to_publish: true` AND `proceed_to_github_ops: true`: repo-operator pushes the branch
- If `safe_to_publish: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`), usually `code-implementer`, with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → UNVERIFIED; skip external ops, write evidence

### Step 14: Update Issue Board

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed.

- If `safe_to_publish: false` or `proceed_to_github_ops: false` → skip GH ops; document why.
- If `gh auth` unavailable → SKIPPED with evidence (not BLOCKED).
- Otherwise (gates true and gh available) → GitHub ops must run.

**Actions:**
- `gh-issue-manager` -> updates issue body status board from receipt
- **Creates GitHub issue if none exists** (allowed in any flow; includes "Signal pending" banner if created from Flow 3)

### Step 15: Report to GitHub

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed.

- If `safe_to_publish: false` or `proceed_to_github_ops: false` → skip GH ops; document why.
- If `gh auth` unavailable → SKIPPED with evidence (not BLOCKED).
- Otherwise (gates true and gh available) → GitHub ops must run.

**Actions:**
- `gh-reporter` -> post summary **to the GitHub issue** (not PR)
- Writes `.runs/<run-id>/build/github_report.md` locally as record

**Issue-first (hard):** All flow logs go to the issue, even if a PR exists. PRs are for PR-review dynamics only.

### Step 16: Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Tests**: <pass/fail counts from build_receipt.json>
- **Mutation Score**: <score from mutation_report.md>
- **Next Flow**: `/flow-4-gate` (after human review)

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
- Stop when `status: VERIFIED`, or
- Stop when `status: UNVERIFIED` and `can_further_iteration_help: no`, or
- Stop immediately on `status: CANNOT_PROCEED` (mechanical failure)

The critic's explicit `can_further_iteration_help` judgment is the stop signal.
Do not hand-wave. Continue while critical/major issues exist AND the critic
believes iteration helps.

## Output Artifacts

After this flow completes, `.runs/<run-id>/build/` should contain:
- `flow_plan.md`
- `subtask_context_manifest.json`
- `open_questions.md`
- `test_changes_summary.md`
- `test_critique.md`
- `impl_changes_summary.md`
- `code_critique.md`
- `mutation_report.md`
- `fix_summary.md`
- `doc_updates.md`
- `self_review.md`
- `build_receipt.json`
- `cleanup_report.md`
- `secrets_scan.md`
- `secrets_status.json`
- `gh_issue_status.md`
- `gh_report_status.md`
- `github_report.md`
- `git_status.md` (if anomaly detected)

Code/test changes in project-defined locations.

---

## Orchestrator Kickoff

### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator: ensure run/<run-id> branch
- [ ] context-loader
- [ ] clarifier
- [ ] test-author ↔ test-critic (microloop)
- [ ] code-implementer ↔ code-critic (microloop)
- [ ] mutator → fixer
- [ ] doc-writer
- [ ] self-reviewer
- [ ] build-cleanup
- [ ] repo-operator: stage intended changes (project-defined)
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] reseal cycle (build-cleanup ↔ secrets-sanitizer) if modified_files
- [ ] repo-operator: restage intended changes (if reseal occurred)
- [ ] repo-operator: commit/push (return Repo Operator Result)
- [ ] gh-issue-manager (only if safe_to_publish AND proceed_to_github_ops)
- [ ] gh-reporter (only if safe_to_publish AND proceed_to_github_ops)
- [ ] finalize flow_plan.md summary

### Agent call order
1) run-prep
2) repo-operator (ensure run branch)
3) context-loader
4) clarifier
5) test-author ↔ test-critic
6) code-implementer ↔ code-critic
7) mutator → fixer
8) doc-writer
9) self-reviewer
10) build-cleanup
11) repo-operator (stage)
12) secrets-sanitizer (Gate Result)
13) (reseal cycle if needed)
14) repo-operator (restage if needed)
15) repo-operator (commit/push; Repo Operator Result)
16) gh-issue-manager (if allowed)
17) gh-reporter (if allowed)
