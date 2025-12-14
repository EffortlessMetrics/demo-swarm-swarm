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

**Important**: The `signal-run-prep` agent (Step 0) establishes the run directory. Do not skip this step.

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
   - Microloops (e.g., requirements author/critic) are ONE todo.

2. Mirror the same list into `.runs/<run-id>/signal/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- Establish run infrastructure (signal-run-prep)
- Research GitHub context (gh-researcher)
- Normalize signal (signal-normalizer)
- Frame the problem (problem-framer)
- Capture open questions (clarifier)
- Requirements microloop (author/critic)
- BDD microloop (author/critic)
- Assess scope + initial risks (scope-assessor)
- Deep risk analysis (risk-analyst)
- Finalize receipt (signal-cleanup)
- Sanitize secrets (secrets-sanitizer)
- Checkpoint commit (repo-operator)
- Ensure GitHub issue exists (gh-issue-manager; gated)
- Post GitHub summary (gh-reporter; gated)
- Update flow_plan.md summary
```

### On Rerun

If running `/flow-1-signal` on an existing run-id:
- Read `.runs/<run-id>/signal/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

## Agents to Use

### Infrastructure (Step 0)

- **signal-run-prep** - MUST be called first to establish the run directory

### Research (Step 1)

- gh-researcher - research existing GitHub issues/PRs for context

### Domain Agents (Flow 1 Specific)

- signal-normalizer
- problem-framer
- requirements-author / requirements-critic (microloop)
- bdd-author / bdd-critic (microloop)
- scope-assessor

### Cross-Cutting Agents

- clarifier
- risk-analyst

### Cleanup + Reporting (End of Flow)

- signal-cleanup - seal receipt, update index
- secrets-sanitizer - publish gate: scans for secrets, fixes or blocks
- repo-operator - checkpoint commit (gated on secrets-sanitizer result)
- gh-issue-manager - ensure GitHub issue exists and update metadata
- gh-reporter - post summary to GitHub issue when gates allow

## Orchestration Outline

### Step 0: Establish Run Infrastructure

**Call `signal-run-prep` first.**

This agent will:
- Derive or confirm the `run-id` from user input, branch name, or ticket reference
- Create `.runs/<run-id>/signal/` directory structure
- Write `.runs/<run-id>/run_meta.json` with run metadata
- Create artifact stub files

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/signal/`.

### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/signal/flow_plan.md`:

```markdown
# Flow 1: Signal Plan for `run-id`

## Planned Steps

- [ ] signal-run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] gh-researcher (GitHub context)
- [ ] signal-normalizer (parse input)
- [ ] problem-framer (synthesize problem)
- [ ] clarifier (document ambiguities)
- [ ] requirements-author / requirements-critic (microloop)
- [ ] bdd-author / bdd-critic (microloop)
- [ ] scope-assessor (stakeholders, risks, estimate)
- [ ] risk-analyst (enrich risks)
- [ ] signal-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (create issue if missing)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

### Step 2: Research GitHub Context

Use `gh-researcher` to gather related issues/PRs, prior decisions, and constraints.

This context informs problem framing and requirements. If `gh` CLI is not available, proceed without this step (document as assumption).

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
   - If `status: VERIFIED` → proceed to BDD scenarios
   - If `status: UNVERIFIED` AND `can_further_iteration_help: yes` → route feedback to `requirements-author` and loop
   - If `status: UNVERIFIED` AND `can_further_iteration_help: no` → proceed (remaining issues documented; not addressable within scope)
   - If `status: CANNOT_PROCEED` → **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention

**Loop guidance**: The critic's `can_further_iteration_help: no` is the stop signal. Continue while critical/major issues exist AND the critic believes iteration helps. The Result block is the control plane; the critique file is the audit artifact.

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
   - If `status: VERIFIED` → proceed to scope assessment
   - If `status: UNVERIFIED` AND `can_further_iteration_help: yes` → route feedback to `bdd-author` and loop
   - If `status: UNVERIFIED` AND `can_further_iteration_help: no` → proceed (remaining issues documented)
   - If `status: CANNOT_PROCEED` → **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention

**Loop guidance**: Same as requirements—route on the Result block. The critique file is the audit artifact.

### Step 8: Assess Scope

Use `scope-assessor` to capture stakeholders, early risks, and scope estimate.

Identify stakeholders, flag early risks by category, and estimate scope (S/M/L/XL).

### Step 9: Analyze Risks

Use `risk-analyst` for deeper risk assessment.

Add risk patterns (security, compliance, data, performance) and severity ratings. This supplements `.runs/<run-id>/signal/early_risks.md` with deeper analysis.

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

**Gating logic (from Gate Result):**
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 11c)
- If `safe_to_commit: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
- Push + GitHub operations require: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`

### Step 11b: Reseal If Modified (Conditional Loop)

If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(signal-cleanup → secrets-sanitizer)` until either:
- the sanitizer reports `modified_files: false`, or
- the sanitizer indicates no reasonable path to fixing (non-convergent).

If reseal cannot make progress (sanitizer signals no reasonable path):
- Append an evidence note to `secrets_scan.md`:
  - "modified_files remained true; sanitizer reports no viable path to fix; stopping to prevent receipt drift."
- If Gate Result `safe_to_commit: true`: call `repo-operator` with `checkpoint_mode: local_only`
  - Agent commits allowlist locally, does **not** push
  - Agent returns `proceed_to_github_ops: false` (mechanically enforced)
- Skip **all** GitHub ops (issue-manager / reporter).
- Flow outcome: `status: UNVERIFIED`, `recommended_action: ESCALATE`
  - If Gate Result `needs_upstream_fix: true`, use `recommended_action: BOUNCE` and the provided `route_to_*`.
- Exit cleanly (Steps 12/13 will be skipped due to `proceed_to_github_ops: false`)

### Step 11c: Checkpoint Commit

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
anomaly_paths: []
```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null.

**Routing logic (from Repo Operator Result):**
- `status: COMPLETED` + `proceed_to_github_ops: true` → proceed to GitHub ops
- `status: COMPLETED_WITH_ANOMALY` → allowlist committed, anomaly documented in `git_status.md`; `proceed_to_github_ops: false`
- `status: FAILED` or `status: CANNOT_PROCEED` → mechanical failure; stop and require human intervention

**Gating interaction with secrets-sanitizer:**
- `repo-operator` reads `safe_to_commit` and `safe_to_publish` from the prior Gate Result
- If `safe_to_commit: false`: skips commit entirely
- If `safe_to_publish: false`: commits locally but skips push; sets `proceed_to_github_ops: false`

**Why checkpoint before GitHub ops:** The issue comment can reference a stable commit SHA. Also keeps local history clean if the flow is interrupted.

### Step 12: Ensure GitHub Issue Exists

**Call `gh-issue-manager`** -> creates issue if missing, updates metadata

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed. If either gate fails, skip this step.

**Hard requirement:** If both gates are true **and** `gh` is authenticated, failure to create/update the issue is a flow failure (`status: UNVERIFIED` with `recommended_action: BOUNCE` to `gh-issue-manager` or `repo-operator`).

This agent:
- Creates GitHub issue if none exists (preferred in Flow 1; allowed in any flow if missing)
- Sets `issue_number`, `canonical_key`, and `aliases` in `run_meta.json`
- Updates `.runs/index.json` with `issue_number` and `canonical_key`
- Writes `gh_issue_status.md`

**Note:** Issues created in Flows 2-6 include a "Signal pending" banner instructing humans to run `/flow-1-signal` to backfill.

If `gh` CLI is not authenticated, this step is SKIPPED (not blocked).

### Step 13: Report to GitHub

**Call `gh-reporter`** -> posts summary **to the GitHub issue** (not PR)

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed. If either gate fails, skip posting.

The reporter:
- Reads `.runs/<run-id>/signal/signal_receipt.json` (source of truth for counts/status)
- Optionally reads `secrets_status.json` as last-mile safety check (Gate Result already cleared publish)
- Reads `issue_number` from `.runs/<run-id>/run_meta.json`
- Posts summary comment to GitHub issue
- Writes `.runs/<run-id>/signal/github_report.md` locally as a record

**Issue-first (hard):** All flow logs go to the issue, even if a PR exists. PRs are for PR-review dynamics only.

If secrets gate blocked or no issue exists, sets status to SKIPPED.

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
| `signal_receipt.json` | signal-cleanup | Structured summary for downstream flows |
| `cleanup_report.md` | signal-cleanup | Artifact verification and count derivation |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings and actions taken |
| `secrets_status.json` | secrets-sanitizer | Machine-readable publish gate status |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | GitHub issue creation status |
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

### TodoWrite (copy exactly)

- [ ] signal-run-prep
- [ ] repo-operator: ensure run/`run-id` branch
- [ ] gh-researcher
- [ ] signal-normalizer
- [ ] problem-framer
- [ ] clarifier
- [ ] requirements-author ↔ requirements-critic (microloop)
- [ ] bdd-author ↔ bdd-critic (microloop)
- [ ] scope-assessor
- [ ] risk-analyst
- [ ] signal-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] reseal cycle (signal-cleanup ↔ secrets-sanitizer) if modified_files
- [ ] repo-operator checkpoint (checkpoint mode; capture Repo Operator Result)
- [ ] gh-issue-manager (only if safe_to_publish AND proceed_to_github_ops)
- [ ] gh-reporter (only if safe_to_publish AND proceed_to_github_ops)
- [ ] finalize flow_plan.md summary

### Agent call order

1) signal-run-prep
2) repo-operator (ensure run branch)
3) gh-researcher
4) signal-normalizer
5) problem-framer
6) clarifier
7) requirements-author ↔ requirements-critic
8) bdd-author ↔ bdd-critic
9) scope-assessor
10) risk-analyst
11) signal-cleanup
12) secrets-sanitizer (read Gate Result)
13) (reseal cycle if needed)
14) repo-operator (checkpoint; read Repo Operator Result)
15) gh-issue-manager (if allowed)
16) gh-reporter (if allowed)
