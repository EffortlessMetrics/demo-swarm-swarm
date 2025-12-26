---
description: Run Flow 1 (Signal -> Spec): shape the problem, identify stakeholders, flag early risks, estimate scope.
argument-hint: "[optional-run-id] <feature request or signal>"
---

# Flow 1: Signal -> Spec

You are orchestrating Flow 1 of the SDLC swarm. This flow transforms messy input into testable requirements, BDD features, early risks, and a GitHub-ready summary.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/signal/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Run identity now comes from `gh-issue-resolver` (Step 0). The `repo-operator` ensures the run branch (Step 0b) and `signal-run-prep` (Step 0c) establishes the run directory using that run-id. Do not skip these steps.
- `run_id_kind: LOCAL_ONLY` means the run-id is a local slug (`local-...`) and the issue is not bound yet (`issue_number: null`).
  - If `github_ops_allowed: false` → repo mismatch / trust block (never bind/create issues in this repo).
  - If `github_ops_allowed: true` + `issue_number: null` → GitHub binding is deferred (bind later when GitHub works; handled by `gh-issue-manager`).
- Only repo mismatch sets `github_ops_allowed: false`. If GitHub is temporarily unavailable/unauthenticated, `github_ops_allowed` remains `true` and binding is deferred (`issue_binding: DEFERRED`; later handled by `gh-issue-manager` when access allows).

#### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Turn messy input into testable requirements
- Identify affected stakeholders (teams, systems, users)
- Flag early security/compliance/performance risks
- Estimate scope (S/M/L/XL t-shirt size)
- Produce BDD scenarios
- Post summary to GitHub issue

## Before You Begin (Required)

### Two State Machines

Flow 1 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Microloops (`requirements-author` ↔ `requirements-critic`, `bdd-author` ↔ `bdd-critic`) are ONE todo each.

2. Mirror the same list into `.runs/<run-id>/signal/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- gh-issue-resolver (resolve/bind GitHub issue; may defer when GH unavailable)
- repo-operator (ensure run branch)
- signal-run-prep (establish run infrastructure)
- gh-researcher (research GitHub context + wisdom scent trail)
- signal-normalizer (normalize signal)
- problem-framer (frame the problem; check for state/migration implications)
- clarifier (capture open questions; non-blocking)
- requirements-author ↔ requirements-critic (microloop; signal-based termination)
- bdd-author ↔ bdd-critic (microloop; signal-based termination; enforce sad paths)
- scope-assessor (assess scope + initial risks)
- risk-analyst (deep risk analysis)
- spec-auditor (integrative audit; may bounce for fixes)
- signal-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (sync GitHub issue; skip when `github_ops_allowed: false`; restricted mode when publish is blocked or artifacts are not pushed)
- gh-reporter (post GitHub summary; skip when `github_ops_allowed: false`; restricted handoff when publish is blocked or artifacts are not pushed)
```

### On Rerun

If running `/flow-1-signal` on an existing run-id:
- Read `.runs/<run-id>/signal/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

## Agents to Use

### Issue binding (Step 0)

- **gh-issue-resolver** - MUST be called first to resolve/create the GitHub issue (or mark repo mismatch / defer binding) and emit `run_id` (`gh-<issue_number>` or `local-<slug>-<hash6>`)

### Infrastructure (Step 0b/0c)

- **repo-operator** - Ensure run branch `run/<run-id>` exists
- **signal-run-prep** - Establish the run directory using the issue-derived run-id

### Research (Step 1)

- gh-researcher - research existing GitHub issues/PRs for context

### Domain Agents (Flow 1 Specific)

- signal-normalizer
- problem-framer
- requirements-author ↔ requirements-critic (microloop; signal-based termination)
- bdd-author ↔ bdd-critic (microloop; signal-based termination)
- scope-assessor

### Cross-Cutting Agents

- clarifier
- risk-analyst

### Integrative Audit (Before Cleanup)

- spec-auditor - final holistic audit of all Flow 1 artifacts; routes back if critical gaps

### Cleanup + Reporting (End of Flow)

- signal-cleanup - seal receipt, update index
- secrets-sanitizer - publish gate: scans for secrets, fixes or blocks
- repo-operator - checkpoint commit (gated on secrets-sanitizer result)
- gh-issue-manager - sync GitHub issue metadata (always attempt when `gh` auth is available; full vs restricted mode based on publish gates and publish_surface)
- gh-reporter - post summary to GitHub issue (full vs restricted handoff based on publish gates)

## Orchestration Outline

### Step 0: Resolve or Create GitHub Issue

**Call `gh-issue-resolver` first.**

This agent will:
- Resolve an explicit issue reference **or** create a new GitHub issue from the signal text
- Return `run_id` (gh-<issue_number> or local-<slug>-<hash6> when repo mismatch prevents GitHub ops) plus issue metadata in a control-plane block
- Perform no filesystem writes (runs before `.runs/<run-id>/` exists)

Use the returned `run_id` for all subsequent steps.

### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main so run artifacts land on the run branch.

### Step 0c: Establish Run Infrastructure

**Call `signal-run-prep`** using the issue-derived `run_id` while on the run branch.

This agent will:
- Confirm the provided `run-id` (should already be `gh-<issue_number>`)
- Create `.runs/<run-id>/signal/` directory structure
- Write `.runs/<run-id>/run_meta.json` with run metadata (binding `issue_number` when the run-id matches `gh-<n>`)
- Create artifact stub files

After this step, you will have a confirmed run directory on the run branch. All subsequent agents write to `.runs/<run-id>/signal/`.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/signal/flow_plan.md`:

```markdown
# Flow 1: Signal Plan for `run-id`

## Planned Steps

- [ ] gh-issue-resolver (resolve/create issue, emit run_id)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] signal-run-prep (establish run directory)
- [ ] gh-researcher (GitHub context + wisdom scent trail)
- [ ] signal-normalizer (parse input)
- [ ] problem-framer (synthesize problem; check state/migration)
- [ ] clarifier (document ambiguities)
- [ ] requirements-author ↔ requirements-critic (microloop; signal-based termination)
- [ ] bdd-author ↔ bdd-critic (microloop; signal-based termination; enforce sad paths)
- [ ] scope-assessor (stakeholders, risks, estimate)
- [ ] risk-analyst (enrich risks)
- [ ] spec-auditor (integrative audit; may bounce for fixes)
- [ ] signal-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (sync issue metadata; restricted issue updates when publish is blocked or not pushed)
- [ ] gh-reporter (post summary; restricted handoff when publish is blocked or not pushed)

## Progress Notes

<Update as each step completes>
```

### Step 2: Research Context

**a) GitHub context:** Use `gh-researcher` to gather related issues/PRs, prior decisions, and constraints.

This context informs problem framing and requirements. If `gh` CLI is not available, proceed without this step (document as assumption).

**b) Wisdom scent trail (optional):** Check `.runs/_wisdom/latest.md` if it exists. This file contains top learnings from the most recent wisdom flow — insights that may inform this run's approach.

If present, extract relevant learnings (especially any that relate to similar feature areas or common pitfalls) and pass them to `problem-framer` as additional context. This enables the pack to learn from itself across runs.

### Step 3: Normalize Signal

Use `signal-normalizer` to parse the raw input into structured form.

### Step 4: Frame the Problem

Use `problem-framer` to synthesize the normalized signal into a clear problem statement with goals, non-goals, and constraints.

### Step 5: Clarify Ambiguities (Non-Blocking)

Use `clarifier` to document ambiguities and assumptions. This step is non-blocking—it produces questions for humans to review later, not gates for the flow.

### Step 6: Refine Requirements (Microloop)

Alternate between `requirements-author` and `requirements-critic`:

1. Call `requirements-author` to draft requirements.
   - Writes functional requirements (REQ-001, REQ-002, etc.)
   - Writes non-functional requirements (NFR-SEC-001, NFR-PERF-001, etc.)
   - Includes acceptance criteria for each

2. Call `requirements-critic` to critique requirements.
   - Reviews testability, consistency, completeness, traceability
   - Sets `Status: VERIFIED | UNVERIFIED`
   - Sets `can_further_iteration_help: yes | no`
   - Lists issues by severity (critical, major, minor)

 3. **Route on the Requirements Critic Result block** (not by re-reading the file):
    - If `status: CANNOT_PROCEED` -> **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention
    - If `recommended_action: BOUNCE` -> follow `route_to_flow/route_to_agent`
    - If `recommended_action: RERUN` -> do the apply pass: rerun `requirements-author` once with the critique worklist, then rerun `requirements-critic` once; proceed after the second critique even if still UNVERIFIED (carry blockers honestly)
    - If `recommended_action: PROCEED` -> proceed after the re-critique pass (even if UNVERIFIED)

**Loop guidance (Signal-Based Termination)**:
- Route on critic's Result block, not pass counts.
- If critic returns `recommended_action: RERUN` AND `can_further_iteration_help: yes`: call writer again with critique worklist, then call critic again.
- Exit conditions (in priority order):
  1. `status: VERIFIED` → proceed (success)
  2. `recommended_action: PROCEED` → proceed (even if UNVERIFIED; carry blockers honestly)
  3. `can_further_iteration_help: no` → proceed (no improvement possible)
  4. Context exhausted → checkpoint and exit `PARTIAL`
- The Result block is the control plane; the critique file is the audit artifact.

### Step 7: BDD Scenarios (Microloop)

Alternate between `bdd-author` and `bdd-critic`:

1. Call `bdd-author` to draft scenarios and examples.
   - Creates Gherkin scenarios for each functional requirement
   - Covers happy paths, edge cases, error scenarios
   - Tags scenarios with requirement IDs (@REQ-001, etc.)
   - Writes `verification_notes.md` for NFRs not expressible as BDD (always present; richer when NFRs exist)

2. Call `bdd-critic` to critique scenarios.
   - Reviews traceability (every REQ-* has scenarios)
   - Checks testability (concrete, not vibes)
   - Assesses coverage (edge cases, errors)
   - Sets `Status: VERIFIED | UNVERIFIED`
   - Sets `can_further_iteration_help: yes | no`
   - Lists issues by severity (critical, major, minor)

 3. **Route on the BDD Critic Result block** (not by re-reading the file):
    - If `status: CANNOT_PROCEED` -> **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention
    - If `recommended_action: BOUNCE` -> follow `route_to_flow/route_to_agent`
    - If `recommended_action: RERUN` -> do the apply pass: rerun `bdd-author` once with the critique worklist, then rerun `bdd-critic` once; proceed after the second critique even if still UNVERIFIED (carry blockers honestly)
    - If `recommended_action: PROCEED` -> proceed after the re-critique pass (even if UNVERIFIED)

**Loop guidance (Signal-Based Termination)**:
- Route on critic's Result block, not pass counts.
- If critic returns `recommended_action: RERUN` AND `can_further_iteration_help: yes`: call writer again with critique worklist, then call critic again.
- **Sad Path enforcement:** The critic will flag REQs missing negative scenarios. Ensure the author addresses these before proceeding.
- Exit conditions (in priority order):
  1. `status: VERIFIED` → proceed (success)
  2. `recommended_action: PROCEED` → proceed (even if UNVERIFIED; carry blockers honestly)
  3. `can_further_iteration_help: no` → proceed (no improvement possible)
  4. Context exhausted → checkpoint and exit `PARTIAL`
- The Result block is the control plane; the critique file is the audit artifact.

### Step 8: Assess Scope

Use `scope-assessor` to capture stakeholders, early risks, and scope estimate.

Identify stakeholders, flag early risks by category, and estimate scope (S/M/L/XL).

### Step 9: Analyze Risks

Use `risk-analyst` for deeper risk assessment.

Add risk patterns (security, compliance, data, performance) and severity ratings. This supplements `.runs/<run-id>/signal/early_risks.md` with deeper analysis.

### Step 9b: Final Spec Audit (Integrative)

**Call `spec-auditor`** to perform an integrative audit of all Flow 1 artifacts.

This is the "Staff Engineer" check before handoff to Flow 2. The auditor reviews:
- Problem → Requirements alignment
- Requirements → BDD coverage
- Risk coverage completeness
- Cross-artifact consistency
- Unresolved critic findings

**Route on its Result block:**
- If `status: VERIFIED` → proceed to cleanup
- If `status: UNVERIFIED` with `recommended_action: BOUNCE` → route back to the specified agent (e.g., `requirements-author`, `bdd-author`) for rework, then re-run the auditor
- If `status: UNVERIFIED` with `recommended_action: PROCEED` → proceed with blockers documented (human judgment needed)
- If `status: CANNOT_PROCEED` → `FIX_ENV` (mechanical failure)

**Loop limit:** Re-run the auditor at most twice after routing to fix agents. If still UNVERIFIED after 2 fix attempts, proceed with blockers and let Flow 2/human handle.

### Step 10: Finalize and Write Receipt

Use `signal-cleanup` to seal the receipt and update index.

This agent:
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Reads quality gate status from critic outputs
- Writes the definitive `signal_receipt.json`
- Updates `.runs/index.json` with status, last_flow, updated_at

**This step MUST complete before secrets-sanitizer and gh-issue-manager.**

### Step 11: Sanitize Secrets (Publish Gate)

Use `secrets-sanitizer` (publish gate).

This agent is a **publish gate** that ensures no secrets are accidentally committed or posted:
- Scans the commit surface (`.runs/<run-id>/`, staged changes)
- **Fixes** what it can: redacts artifacts, externalizes code secrets
- Writes `secrets_status.json` with `safe_to_commit` and `safe_to_publish` flags

**Status semantics** (status describes what the sanitizer *did*; flags tell you what you're allowed to do):
- `CLEAN`: No secrets found; flags typically true (but always read flags, not status)
- `FIXED`: Secrets found and remediated; flags typically true **unless** `needs_upstream_fix` forced gating
- `BLOCKED_PUBLISH`: Sanitizer couldn't complete (mechanical); `safe_to_publish: false`

**Control plane:** The sanitizer returns a **Gate Result block** for orchestrator routing. `secrets_status.json` is the durable audit record. Route on the Gate Result, not by re-reading the file.

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
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 11c)
- If `safe_to_commit: false`:
  - `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO failure)
  - `blocker_kind: SECRET_IN_CODE` → route to appropriate agent (orchestrator decides)
  - `blocker_kind: SECRET_IN_ARTIFACT` → investigate manually
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`
- GitHub issue/comment updates still run in restricted mode when publish is blocked or `publish_surface: NOT_PUSHED`

### Step 11b: Checkpoint Commit

Checkpoint the audit trail **before** any GitHub operations.

**Call `repo-operator`** in checkpoint mode. The agent handles:
1. Resets staging and stages allowlist only
2. Enforces allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/signal/git_status.md` if anomaly detected
4. Handles no-op gracefully (nothing to commit = success)
5. Returns **Repo Operator Result** (control plane)

**Allowlist for Flow 1:**
- `.runs/<run-id>/signal/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

**Control plane:** The `repo-operator` returns a **Repo Operator Result block** for orchestrator routing:

```md
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null. `publish_surface` must always be present (PUSHED or NOT_PUSHED), even on no-op commits, anomalies, `safe_to_commit: false`, or push failures.

**Routing logic (from Repo Operator Result):**
- `status: COMPLETED` + `proceed_to_github_ops: true` → proceed to GitHub ops
- `status: COMPLETED_WITH_ANOMALY` → allowlist committed, anomaly documented in `git_status.md`; `proceed_to_github_ops: false`
- `status: FAILED` or `status: CANNOT_PROCEED` → mechanical failure; stop and require human intervention

**Gating interaction with secrets-sanitizer:**
- `repo-operator` reads `safe_to_commit` and `safe_to_publish` from the prior Gate Result
- If `safe_to_commit: false`: skips commit entirely
- If `safe_to_publish: false`: commits locally but skips push; sets `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`

**Why checkpoint before GitHub ops:** The issue comment can reference a stable commit SHA. Also keeps local history clean if the flow is interrupted.

### Step 12-13: GitHub Reporting

**Call `gh-issue-manager`** (sync/update/bind issue) then **`gh-reporter`** (post summary).

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

If `issue_number` is missing and `gh` is available, `gh-issue-manager` may attempt to create/bind.

### Step 14: Finalize Flow

**Note:** Receipt derivation is handled by `signal-cleanup` (Step 10). See the `signal-cleanup` agent documentation for derivation rules.

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Open Questions**: See `open_questions.md`
- **Assumptions Made**: See individual artifacts
- **Next Flow**: `/flow-2-plan` (after human review)

## Human Review Checklist

Before proceeding to Flow 2, humans should review:
- [ ] `.runs/<run-id>/signal/requirements.md` - Are these the right requirements?
- [ ] `.runs/<run-id>/signal/features/*.feature` - Do these scenarios cover the expected behavior?
- [ ] `.runs/<run-id>/signal/verification_notes.md` - Are NFR verification criteria adequate?
- [ ] `.runs/<run-id>/signal/early_risks.md` and `.runs/<run-id>/signal/risk_assessment.md` - Are risks acceptable?
- [ ] `.runs/<run-id>/signal/open_questions.md` - Can any questions be answered now?
```

## Artifact Outputs

All written to `.runs/<run-id>/signal/`:

| Artifact | Source Agent | Description |
|----------|--------------|-------------|
| `run_meta.json` | signal-run-prep, gh-issue-manager | Run metadata (in `.runs/<run-id>/`) |
| `flow_plan.md` | orchestrator | Execution plan and progress |
| `github_research.md` | gh-researcher | Related issues/PRs and constraints |
| `issue_normalized.md` | signal-normalizer | Structured summary of raw signal |
| `context_brief.md` | signal-normalizer | Related history and context |
| `problem_statement.md` | problem-framer | Goals, non-goals, constraints |
| `open_questions.md` | clarifier | Open questions and assumptions |
| `requirements.md` | requirements-author | Functional + non-functional requirements |
| `requirements_critique.md` | requirements-critic | Critique and iteration guidance |
| `features/*.feature` | bdd-author | BDD scenarios (Gherkin) |
| `example_matrix.md` | bdd-author | Example mapping for BDD |
| `verification_notes.md` | bdd-author | NFR verification criteria (non-BDD) |
| `bdd_critique.md` | bdd-critic | Critique of BDD scenarios |
| `stakeholders.md` | scope-assessor | Teams, systems, users affected |
| `early_risks.md` | scope-assessor | Initial risk identification by category |
| `risk_assessment.md` | risk-analyst | Deep risk analysis with severity ratings |
| `scope_estimate.md` | scope-assessor | S/M/L/XL estimate with rationale |
| `spec_audit.md` | spec-auditor | Integrative audit verdict and cross-artifact consistency |
| `signal_receipt.json` | signal-cleanup | Structured summary for downstream flows |
| `cleanup_report.md` | signal-cleanup | Artifact verification and count derivation |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings and actions taken |
| `secrets_status.json` | secrets-sanitizer | Machine-readable publish gate status |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | GitHub issue sync status |
| `gh_report_status.md` | gh-reporter | GitHub posting status |
| `github_report.md` | gh-reporter | Record of GitHub post |

## Assumptions + Questions Contract

All Flow 1 agents must emit:
- **Assumptions Made to Proceed**: What was assumed, why, and impact if wrong
- **Questions / Clarifications Needed**: Questions that would change the spec, with defaults

These sections enable humans to review what was assumed at the flow boundary, and to re-run with better inputs if needed.

**Flow 1 is designed to be re-run.** If you run `/flow-1-signal` on an existing run-id:
- `signal-run-prep` will lock onto the existing directory
- Agents will read and refine existing artifacts
- Each run improves the output based on newly resolved ambiguity

## Status States

Agents set status in their output artifacts:

- **VERIFIED** - `blockers` empty, `missing_required` empty, and all quality gates passed; assumptions documented. Set `recommended_action: PROCEED`.
- **UNVERIFIED** - `blockers` non-empty OR `missing_required` non-empty OR any quality gate UNVERIFIED; contains concrete concerns and assumptions. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED** - IO/permissions/tool failure only (exceptional); cannot read/write files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED. If agents can read inputs and form an opinion, status is VERIFIED or UNVERIFIED with assumptions, never CANNOT_PROCEED. Ambiguity uses documented assumptions + UNVERIFIED status.

## Human Collaboration

**Humans do not intervene mid-flow.** Flow 1 runs from start to finish without stopping for human input. At the end:

1. All artifacts are written to `.runs/<run-id>/signal/`
2. Summary is posted to GitHub
3. Human reviews:
   - `requirements.md`
   - `features/*.feature`
   - `early_risks.md`
   - `open_questions.md`
4. Human answers questions and updates assumptions as needed
5. Human runs `/flow-2-plan` when ready

## Completion

Flow 1 is complete when:
1. All artifacts exist under `.runs/<run-id>/signal/` (even if imperfect)
2. `flow_plan.md` is updated with final status
3. GitHub summary is posted (or `github_report.md` written if gh unavailable)

Human gate at end: **"Is this the right problem to solve?"**

If yes, proceed to `/flow-2-plan`.

---

## Orchestrator Kickoff

### Station order + templates

#### Station order

1. `gh-issue-resolver`

2. `repo-operator` (ensure run branch)

3. `signal-run-prep`

4. `gh-researcher`

5. `signal-normalizer`

6. `problem-framer`

7. `clarifier`

8. `requirements-author` ↔ `requirements-critic` (microloop; apply Microloop Template)

9. `bdd-author` ↔ `bdd-critic` (microloop; apply Microloop Template)

10. `scope-assessor`

11. `risk-analyst`

12. `spec-auditor` (integrative audit; may route back for fixes)

13. `signal-cleanup`

14. `secrets-sanitizer`

15. `repo-operator` (checkpoint; read Repo Operator Result)

16. `gh-issue-manager` (if allowed)

17. `gh-reporter` (if allowed)

#### Microloop Template (writer ↔ critic)

Run this template for: tests, code, docs, requirements, BDD, options, contracts, observability.

1) Writer pass: call `<writer>`
2) Critique pass: call `<critic>` and read its control-plane Result
3) Route on critic Result:
   - If `recommended_action: PROCEED` → proceed (no apply pass needed)
   - If `recommended_action: RERUN` AND `can_further_iteration_help: yes` → continue to step 4
   - Otherwise → proceed with `UNVERIFIED` + blockers recorded
4) Apply pass: call `<writer>` with the critique worklist
5) Re-critique: call `<critic>` again, return to step 3

**Termination:** Signal-based, not count-based. Loop continues while critic says RERUN + can_further_iteration_help: yes. Exit when signal says stop or context exhausted.

### TodoWrite (copy exactly)

- [ ] gh-issue-resolver (issue binding -> run_id)
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] signal-run-prep
- [ ] gh-researcher
- [ ] signal-normalizer
- [ ] problem-framer
- [ ] clarifier
- [ ] requirements-author ↔ requirements-critic (microloop; signal-based termination)
- [ ] bdd-author ↔ bdd-critic (microloop; signal-based termination)
- [ ] scope-assessor
- [ ] risk-analyst
- [ ] spec-auditor (integrative audit; may bounce for fixes)
- [ ] signal-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint; capture Repo Operator Result)
- [ ] gh-issue-manager (skip when `github_ops_allowed: false`; full when `safe_to_publish` + `proceed_to_github_ops` + `publish_surface: PUSHED`; restricted updates otherwise when gh auth is available)
- [ ] gh-reporter (skip when `github_ops_allowed: false`; full only when publish gates are clear and artifacts pushed; restricted handoff otherwise)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.
