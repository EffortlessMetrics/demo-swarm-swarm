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
  - (after first vertical slice) checkpoint push + pr-creator (early; once)
  - (after checkpoint push) feedback check (pr-feedback-harvester; route on blockers[])
  - (checkpoint push every 3-5 ACs; feedback check after each)
- standards-enforcer (format/lint/hygiene; global)
- test-executor (full suite; global)
- flakiness-detector (if failures; apply Worklist Loop Template)
- mutation-auditor (mutation worklist; apply Worklist Loop Template)
- fuzz-triager (if configured; apply Worklist Loop Template)
- fixer (only if critiques/worklists require it)
- doc-writer ↔ doc-critic (microloop; 2 passes default)
- self-reviewer (self-review)
- build-cleanup (finalize receipt; update index; update `flow_plan.md`)
- repo-operator (stage + classify changes)
- secrets-sanitizer (pre-publish sweep)
- repo-operator (commit/push)
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
- standards-enforcer -- format/lint/hygiene codebase
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
  - [ ] (after first vertical slice) checkpoint push + pr-creator (early; once)
  - [ ] (after checkpoint push) feedback check (pr-feedback-harvester; route on blockers[])
  - [ ] AC-002: ...
  - [ ] (add rows per AC from ac_matrix.md)
  - [ ] (checkpoint push every 3-5 ACs; feedback check after each)
- [ ] standards-enforcer (format/lint/hygiene; global)
- [ ] test-executor (full suite; global)
- [ ] flakiness-detector (if failures)
- [ ] mutation-auditor (mutation worklist)
- [ ] fuzz-triager (optional; config-present ⇒ run)
- [ ] fixer (targeted; only if critiques/worklists require it)
- [ ] doc-writer ↔ doc-critic (microloop; 2 passes default)
- [ ] self-reviewer (review)
- [ ] build-cleanup (write receipt, update index)
- [ ] repo-operator (stage + classify)
- [ ] secrets-sanitizer (pre-publish sweep)
- [ ] repo-operator (commit/push)
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

**Anti-Reward-Hacking Guard:** After each code-implementer pass, check `git diff` for deleted tests. If existing tests were removed (not just updated), flag as **HIGH risk** in the code-critic review unless explicitly justified. Deleting tests to make code "pass" is a reward-hacking pattern that degrades quality.

**After all ACs complete:** Proceed to global hardening (Step 6).

### Step 5b: Early PR Bootstrap (Once, After First Vertical Slice)

**After completing the first AC (or first meaningful vertical slice), bootstrap bot feedback:**

1. **Call `repo-operator`** with task: "checkpoint push after first slice"
   - Stage code/test changes + `.runs/<run-id>/build/` artifacts
   - Run secrets-sanitizer (light sweep)
   - Commit and push if clean

2. **Call `pr-creator`** (once per run):
   - Create Draft PR immediately
   - Gets CodeRabbit + GitHub Actions spinning early
   - CI feedback runs continuously in parallel while you continue the AC loop

**Why early?** This is the cheapest way to find integration failures. Bots review while you implement. Flow 4 harvests their feedback formally, but you can iterate on findings during the AC loop.

**Checkpoint cadence for remaining ACs:**
- Push after every 3-5 ACs, or when you touch core modules
- Each checkpoint: stage → sanitizer → commit/push (same pattern as Step 5b)
- CI/bot feedback runs continuously; don't wait for it

**If push fails or is blocked:**
- Continue the AC loop locally
- Note the blocker in `flow_plan.md`
- Retry push after remediation

This turns "push at end" into "push early, push often" — continuous feedback, not post-hoc review.

### Step 5c: Feedback Check (Harvest → Route on Blockers)

**After each checkpoint push (and after the first Draft PR exists), harvest full PR feedback and route on blockers.**

This uses the **same `pr-feedback-harvester`** as Flow 4 — no separate "pulse mode." The difference is how Flow 3 consumes the Result block.

**What it does:**
1. **Call `pr-feedback-harvester`** with output directory `build/` (full harvest — CI status, all comments, all reviews)
2. **Route on the PR Feedback Harvester Result block** (not by re-parsing the file)

**Routing logic (Route on Result block's `blockers[]`):**

```yaml
# Route on PR Feedback Harvester Result — one routing surface (CI + comments all become blockers)
# blockers[] is CRITICAL-only. MAJOR/MINOR/INFO stay in counts + full pr_feedback.md for Flow 4.

if blockers_count > 0:
  # CRITICAL items — route top 1-3 for the agent to investigate and fix
  for blocker in blockers[:3]:
    call blocker.route_to_agent with:
      - blocker.id                # "FB-CI-987654321" or "FB-RC-123456789" (stable)
      - blocker.evidence          # "check:test → auth.test.ts:45" or "src/auth.ts:42"
      - blocker.thoughts          # "Looks like hashPassword returns undefined for empty input"
    run relevant verification (test-executor for CI, targeted tests for code fixes)
  # do NOT drain the full list (Flow 4 owns that)

else:
  # No CRITICAL blockers — continue AC loop
  # MAJOR/MINOR/INFO will be handled in Flow 4
  continue_ac_loop: true
```

**Key invariant:** One routing surface. CI failures are blockers with `source: CI`. CodeRabbit/human CRITICAL items are blockers with their source. No separate CI vs comment path.

**Key shift:** The harvester triaged and added quick-read thoughts. The routed agent:
- Receives the evidence (file:line, check name)
- Sees the harvester's triage thoughts ("looks like real issue", "bot probably wrong")
- Does the deep investigation and fix

The harvester gets feedback back fast. The routed agents do the deep work.

**Record unresolved blockers:**
After the bounded interrupt (top 1-3), record remaining blockers in `.runs/<run-id>/build/feedback_blockers.md` as IDs only (e.g., `FB-CI-987654321`, `FB-RC-123456789`). Flow 4 will harvest and drain everything systematically.

**Why this matters:**
- Flow 3 was blind to blocker **content** (only saw CI red + counts)
- Now it sees "CodeRabbit says security issue at auth.ts:42" and routes to code-implementer
- The point of early PR bootstrap is to catch these during build, not post-hoc

**Optional station (skip conditions):**
- No PR exists yet (first push hasn't happened)
- GitHub access is unavailable
- Time pressure requires continuing without the check

When skipped, continue the AC loop — Flow 4 will harvest feedback formally.

### Step 6: Global Hardening

After all ACs complete, run global hardening:

- Run `standards-enforcer` to format/lint the codebase and remove debug artifacts (capture a standards report).
- Run `test-executor` to run the **full test suite** (not per-AC filtered; captures coverage).
- If tests fail or look unstable, run `flakiness-detector` and route on its Result block.
- If tests are green, run `mutation-auditor` (bounded, on changed files) to produce a prioritized survivor worklist (route to `test-author` or `fixer`).
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

### Step 10: Stage + Classify Changes

**Call `repo-operator`** with task: "stage intended changes for build"

**Intended commit surface for Build:**
- Code/test/doc changes (project-defined locations)
- `.runs/<run-id>/build/` (all flow artifacts)
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

The repo-operator stages the intended set and **classifies** any out-of-scope changes:

**Anomaly classification:**
- **HIGH risk (blocks push):** staged/tracked changes outside intended set
  - Uncertain provenance → cannot safely publish
  - Commit the intended set locally, skip push
  - Write `.runs/<run-id>/build/extra_changes.md` with classification:
    - Incidental (include next time) vs scope creep (split / new run)
- **LOW risk (warning only):** untracked files outside intended set
  - Allows push (new files haven't been `git add`ed, so provenance is clear)
  - Write warning in `git_status.md`

**Repo Operator Result for staging:**
```yaml
## Repo Operator Result
operation: stage
status: COMPLETED | COMPLETED_WITH_WARNING | COMPLETED_WITH_ANOMALY
proceed_to_github_ops: true | false  # false if HIGH risk anomalies
anomaly_classification:
  unexpected_staged_paths: []    # HIGH risk
  unexpected_unstaged_paths: []  # HIGH risk
  unexpected_untracked_paths: [] # LOW risk (warning)
```

**Key shift:** This replaces "stop the world until clean" with "stage, classify, proceed."
- Engineering continues locally even when push is blocked
- The system captures and labels extra changes rather than punishing them
- Ad-hoc fixes happen; record them, don't fight them

**IMPORTANT: Do NOT commit yet. Must pass pre-publish sweep first.**

### Step 11: Pre-Publish Sweep (Secrets Sanitizer)

**The sanitizer is a fix-first pre-publish hook, not a behavioral throttle.**

Call `secrets-sanitizer` with publish surface:
- Staged code/test changes
- `.runs/<run-id>/build/` artifacts

**What it does:**
- Fast scan for obvious secret patterns
- Auto-redact what it can (artifact secrets, placeholder tokens)
- Report findings only when remediation requires human judgment

**Status vs Flags:**
- `status` = what happened:
  - `CLEAN`: No secrets found
  - `FIXED`: Secrets found and auto-remediated
  - `BLOCKED`: Cannot safely remediate (requires human judgment or upstream fix)
- `safe_to_commit/safe_to_publish` = what you're allowed to do (authoritative)
- `blocker_kind` = why blocked (machine-readable category): `NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT`

**Key posture:** Publishing can be blocked, but work never stops.
- If `blocker_kind: SECRET_IN_CODE`: route to `code-implementer` or `fixer` (orchestrator decides)
- If `blocker_kind: MECHANICAL`: **FIX_ENV** — tool/IO issue, not a code problem
- Either way: continue engineering locally while remediation proceeds

Typically `safe_to_*` are true for CLEAN/FIXED, but **the orchestrator must use the Gate Result booleans, not infer from status**.

**Gate Result block (returned by secrets-sanitizer):**

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

**Gating logic (boolean gate — the sanitizer says yes/no, orchestrator decides next steps):**
- The sanitizer is a fix-first pre-commit hook, not a router
- `blocker_kind` explains why blocked (machine-readable category):
  - `NONE`: not blocked
  - `MECHANICAL`: IO/permissions/tooling failure → **FIX_ENV**
  - `SECRET_IN_CODE`: secret in staged code → route to `code-implementer` or `fixer` (orchestrator decides)
  - `SECRET_IN_ARTIFACT`: secret in `.runs/` artifact that can't be redacted → investigate manually
- If `safe_to_commit: false`: skip commit, continue engineering locally
- If `safe_to_commit: true` but `safe_to_publish: false`: commit locally (audit trail preserved), skip push
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`
- GitHub reporting ops still run in RESTRICTED mode when publish is blocked or `publish_surface: NOT_PUSHED`

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
  - If `blocker_kind: SECRET_IN_CODE` → route to `code-implementer` or `fixer` (orchestrator decides)
  - If `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO issue)
  - Otherwise → skip push (`publish_surface: NOT_PUSHED`), write evidence
- **Keep engineering locally** while remediation proceeds. Publishing is gated; work is not.

### Step 12b: PR Status Check (Conditional)

**Note:** PR creation now happens early (Step 5b) to get bots spinning during the AC loop.

This step handles edge cases where the early PR wasn't created:
- If `pr_number` exists in run_meta: skip (PR already exists from Step 5b)
- If `publish_surface: NOT_PUSHED`: skip (can't create PR without push)
- Otherwise: call `pr-creator` as fallback

**Route on PR Creator Result block:**
- If `operation_status: CREATED`: PR created successfully, `pr_number` available
- If `operation_status: EXISTING`: PR already existed, `pr_number` captured
- If `operation_status: SKIPPED`: Prerequisites not met, note reason and continue
- If `operation_status: FAILED`: Creation failed, note in concerns and continue

**Metadata updates (handled by pr-creator):**
- `run_meta.json`: `pr_number`, `pr_url`
- `index.json`: `pr_number`

**Artifact output:**
- `.runs/<run-id>/build/pr_creation_status.md`

**Important:** PR creation failure does not block the flow. Flow 4 (Review) can create the PR if needed.

### Step 12c: Stage PR Metadata (Conditional)

**If PR was created or found (operation_status: CREATED or EXISTING):**

The `run_meta.json` and `index.json` now contain `pr_number`. The next checkpoint will include this metadata.

**If PR was skipped or failed:** Nothing to stage.

### Step 13-14: GitHub Reporting

**Call `gh-issue-manager`** then **`gh-reporter`** to update the issue.

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

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
- `standards_report.md`
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
- `pr_feedback.md` (full PR feedback harvest; Flow 3's own copy)
- `feedback_blockers.md` (unresolved blocker IDs for Flow 4; optional)

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
   - After first vertical slice: `repo-operator` (checkpoint push) + `pr-creator` (early; once)
   - After checkpoint push: `pr-feedback-harvester` (full harvest; route on `blockers[]`)
   - Checkpoint push every 3-5 ACs or when touching core modules (feedback check after each)

7. `standards-enforcer` (global — format/lint + hygiene sweep)

8. `test-executor` (full suite; global)

9. `flakiness-detector` (if failures; routes to `test-author`/`code-implementer`/`fixer`; confirm via `test-executor`; apply Worklist Loop Template)

10. `mutation-auditor` (if tests green; routes to `test-author`/`fixer`; confirm via `test-executor`; apply Worklist Loop Template)

11. `fuzz-triager` (if configured; routes to `code-implementer`/`fixer`; confirm via `test-executor`; apply Worklist Loop Template)

12. `doc-writer` ↔ `doc-critic` (microloop; apply Microloop Template)

13. `self-reviewer`

14. `build-cleanup`

15. `repo-operator` (stage + classify)

16. `secrets-sanitizer` (pre-publish sweep)

17. `repo-operator` (commit/push)

18. `gh-issue-manager` (if allowed)

19. `gh-reporter` (if allowed)

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
   - **Anti-Reward-Hacking:** code-critic checks `git diff` for deleted tests; flags as HIGH risk if tests removed without justification
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

**After all ACs:** Proceed to global hardening (standards-enforcer, full test-executor, mutation, etc.)

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
  - (after first vertical slice) checkpoint push + pr-creator (early; once)
  - (after checkpoint push) feedback check (pr-feedback-harvester; route on `blockers[]`)
  - (checkpoint push every 3-5 ACs; feedback check after each)
- [ ] standards-enforcer (global)
- [ ] test-executor (full suite; global)
- [ ] flakiness-detector (if failures)
- [ ] mutation-auditor
- [ ] fuzz-triager (if configured)
- [ ] fixer (only if critiques/worklists require it)
- [ ] doc-writer ↔ doc-critic (microloop; 2 passes default)
- [ ] self-reviewer
- [ ] build-cleanup
- [ ] repo-operator (stage + classify; capture Repo Operator Result)
- [ ] secrets-sanitizer (pre-publish sweep; capture Gate Result block)
- [ ] repo-operator (commit/push; return Repo Operator Result)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.
