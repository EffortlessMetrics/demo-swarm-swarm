---
description: Run Flow 2 (Spec to Design): produce ADR, contracts, observability spec, test/work plans, design validation.
---

# Flow 2: Spec to Design

You are orchestrating Flow 2 of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/plan/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/plan/` exists.

#### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Turn requirements into architecture decisions
- Define API contracts and data models
- Create observability, test, and work plans
- Validate design feasibility

## Before You Begin (Required)

### Two State Machines

Flow 2 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Parallel steps (6-9) are ONE todo.

2. Mirror the same list into `.runs/<run-id>/plan/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- Establish run infrastructure (run-prep)
- Initialize plan flow_plan.md
- Capture plan open questions (clarifier)
- Map impact (impact-analyzer)
- Propose design options (design-optioneer)
- Write ADR (adr-author)
- Define contracts + schema + observability + test plan + work plan (parallel)
- Validate design microloop (design-critic)
- Check policy compliance (policy-analyst)
- Finalize plan receipt (plan-cleanup)
- Sanitize secrets (secrets-sanitizer)
- Checkpoint commit (repo-operator)
- Update issue status board (gh-issue-manager; gated)
- Post Plan summary (gh-reporter; gated)
- Finalize plan flow_plan.md summary
```

### On Rerun

If running `/flow-2-plan` on an existing run-id:
- Read `.runs/<run-id>/plan/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

If you encounter ambiguity or missing information, **document it and continue**. Write assumptions clearly in artifacts.

## Subagents to use

Flow 2 uses infrastructure + domain agents + cross-cutting agents:

### Infrastructure (Step 0)
- run-prep (establish run directory)

### Domain agents (in order)
- impact-analyzer
- design-optioneer
- adr-author
- interface-designer
- observability-designer
- test-strategist
- work-planner
- design-critic

### Cross-cutting agents
- clarifier (Plan-local open questions)
- risk-analyst (if risk patterns identified)
- policy-analyst (policy compliance check)
- plan-cleanup (seal receipt, update index)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit - gated on secrets-sanitizer result)
- gh-issue-manager (update issue status board)
- gh-reporter (one comment per Plan run)

## Upstream Inputs

Read from `.runs/<run-id>/signal/` (if available):
- `problem_statement.md`
- `requirements.md`
- `requirements_critique.md`
- `features/*.feature` (BDD scenarios)
- `example_matrix.md`
- `bdd_critique.md`
- `verification_notes.md` (NFR verification criteria)
- `stakeholders.md`
- `early_risks.md`
- `risk_assessment.md`
- `scope_estimate.md`
- `open_questions.md` (Signal's question register)
- `signal_receipt.json` (optional; provides counts and quality gate status without re-parsing)

**If upstream artifacts are missing**: Flow 2 can start without Flow 1. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue. This enables flexibility for hotfixes or design-first workflows.

## Orchestration outline

### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/plan/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "plan" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/plan/`.

### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/plan/flow_plan.md`:

```markdown
# Flow 2: Plan for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] clarifier (Plan open questions)
- [ ] impact-analyzer (map affected components)
- [ ] design-optioneer (propose 2-3 options)
- [ ] adr-author (write architecture decision)
- [ ] interface-designer / observability-designer / test-strategist / work-planner (parallel)
- [ ] design-critic (validate design, loop if needed)
- [ ] policy-analyst (check compliance)
- [ ] plan-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

### Step 2: Plan Open Questions (Non-blocking)

Call `clarifier` to create the Plan-local questions register. Signal's `open_questions.md` is upstream input; Plan gets its own register for design-phase questions.

### Step 3: Map impact
- Use `impact-analyzer` to map impact and blast radius.

### Step 4: Propose design options
- Use `design-optioneer` to propose design options.

### Step 5: Write ADR
- Use `adr-author` to write the ADR.

### Step 6: Define contracts and schema (can run in parallel with steps 7-9)
- Use `interface-designer` for contracts/schema/migrations (planned migrations live under the run directory; actual migrations move during Build).

### Step 7: Plan observability (parallel)
- Use `observability-designer` to define observability.

### Step 8: Plan testing (parallel)
- Use `test-strategist` to write the test plan (incorporate Signal BDD + verification notes).

### Step 9: Plan work (parallel)
- Use `work-planner` — "produce subtask index + work plan".

### Step 10: Validate design (microloop)
- Use `design-critic` to validate the design.

**Route on the Design Critic Result block** (not by re-reading the file):
- If `status: VERIFIED` → proceed to policy check
- If `status: UNVERIFIED` AND `can_further_iteration_help: yes` → rerun affected steps (options/ADR/contracts/plans), then re-run design-critic
- If `status: UNVERIFIED` AND `can_further_iteration_help: no` → proceed (remaining issues documented)
- If `status: CANNOT_PROCEED` → **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention

**Loop guidance**: The Result block is the control plane; `design_validation.md` is the audit artifact. Agents do not know they are in a loop—they read inputs, write outputs, and set a status. The orchestrator routes on the Result block.

### Step 11: Check policy compliance
- Use `policy-analyst` for policy compliance.

### Step 12: Finalize Plan (receipt + index)
- Use `plan-cleanup` to seal the receipt, verify artifacts, and update index counts mechanically.

### Step 13: Sanitize secrets (publish gate)
- Use `secrets-sanitizer` (publish gate).

**Gate Result block (returned by secrets-sanitizer):**

The agent returns a Gate Result block for orchestrator routing:

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

**Field semantics:**
- `status` is **descriptive** (what happened). **Never infer permissions** from it.
- `safe_to_commit` / `safe_to_publish` are **authoritative permissions**.
- `modified_files` is the **reseal trigger** (if true, rerun cleanup ↔ sanitizer).
- `needs_upstream_fix` means the sanitizer can't make it safe (code/config needs remediation).
- `recommended_action` + `route_to_*` give you a closed-vocab routing signal.

**Control plane vs audit plane:** The Gate Result block is the control plane for orchestrator routing. `secrets_status.json` is the durable audit record. Route on the returned block, not by re-reading the file.

**Gating logic (from Gate Result):**
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 13c)
- If `safe_to_commit: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
- Push + GitHub operations require: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`

### Step 13b: Reseal If Modified (Conditional Loop)

If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(plan-cleanup → secrets-sanitizer)` until either:
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

**Note:** `checkpoint_mode: local_only` is a named parameter to `repo-operator` that mechanically enforces `proceed_to_github_ops: false` regardless of `safe_to_publish`. This ensures safe-bail cannot accidentally push.

### Step 13c: Checkpoint Commit

Checkpoint the audit trail **before** any GitHub operations.

**Call `repo-operator`** in checkpoint mode. The agent handles:
1. Resets staging and stages allowlist only
2. Enforces allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/plan/git_status.md` if anomaly detected
4. Handles no-op gracefully (nothing to commit = success)
5. Returns **Repo Operator Result** (control plane)

**Allowlist for Flow 2:**
- `.runs/<run-id>/plan/`
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

### Step 14: Update GitHub issue status board

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed.

- If `safe_to_publish: false` or `proceed_to_github_ops: false` → skip GH ops; document why.
- If `gh` CLI unauthenticated/unavailable → SKIPPED with evidence (not BLOCKED).
- Otherwise (gates true and gh available) → GitHub ops must run.

**Actions:**
- `gh-issue-manager` updates issue body status board from `.runs/<run-id>/plan/plan_receipt.json`
- **Creates GitHub issue if none exists** (allowed in any flow; includes "Signal pending" banner if created from Flow 2)

### Step 15: Post Plan summary to issue

**Prerequisite (two gates):**
- Gate Result: `safe_to_publish: true`
- Repo Operator Result: `proceed_to_github_ops: true`

Both must be true to proceed.

- If `safe_to_publish: false` or `proceed_to_github_ops: false` → skip GH ops; document why.
- If `gh` CLI unauthenticated/unavailable → SKIPPED with evidence (not BLOCKED).
- Otherwise (gates true and gh available) → GitHub ops must run.

**Actions:**
- `gh-reporter` posts one short, link-heavy comment **to the GitHub issue** (not PR)
- Uses checkmarks for status, sourced from `.runs/<run-id>/plan/plan_receipt.json`
- Writes `.runs/<run-id>/plan/github_report.md` locally as record
- **Issue-first (hard):** All flow logs go to the issue, even if a PR exists. PRs are for PR-review dynamics only.

### Step 16: Finalize flow_plan.md

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **ADR Decision**: <brief summary of chosen approach>
- **Design Concerns**: See `.runs/<run-id>/plan/design_validation.md`
- **Next Flow**: `/flow-3-build` (after human review)

## Human Review Checklist

Before proceeding to Flow 3, humans should review:
- [ ] `.runs/<run-id>/plan/adr.md` - Is this the right architecture decision?
- [ ] `.runs/<run-id>/plan/api_contracts.yaml` - Are the contracts correct?
- [ ] `.runs/<run-id>/plan/work_plan.md` - Is the breakdown reasonable?
- [ ] `.runs/<run-id>/plan/design_validation.md` - Are flagged concerns acceptable?
```

## Downstream Contract

Flow 2 is complete when these exist (even if imperfect):

- `flow_plan.md` - Execution plan and progress
- `plan_receipt.json` - Receipt for downstream consumers
- `impact_map.json` - Services, modules, data, external systems affected
- `design_options.md` - 2-3 architecture options with trade-offs
- `adr.md` - Chosen option with rationale and consequences
- `api_contracts.yaml` - Endpoints, schemas, error shapes
- `schema.md` - Data models, relationships, invariants
- `migrations/*.sql` - Draft migrations (optional, if DB changes needed)
- `observability_spec.md` - Metrics, logs, traces, SLOs, alerts
- `test_plan.md` - BDD to test types mapping, priorities
- `work_plan.md` - Subtasks, ordering, dependencies
- `design_validation.md` - Feasibility assessment, known issues

## Status States

Agents set status in their output artifacts:

- **VERIFIED**: `blockers` empty, `missing_required` empty, and all quality gates passed; artifact complete for its purpose. Set `recommended_action: PROCEED`.
- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR any quality gate UNVERIFIED; artifact created but has issues. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read/write files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.

Use `design-critic` status and `plan_receipt.json` to determine flow outcome.

## Notes

- Steps 6-9 can run in parallel after `adr-author` completes
- `design-critic` reviews ALL artifacts before policy check
- Human gate at end: "Is this the right design?"
- Agents never block; they document concerns and continue

## Artifact Outputs

All written to `.runs/<run-id>/plan/`:

| Artifact | Source Agent | Description |
|----------|--------------|-------------|
| `flow_plan.md` | orchestrator | Execution plan and progress |
| `open_questions.md` | clarifier | Plan-local questions register |
| `impact_map.json` | impact-analyzer | Affected services, modules, data |
| `design_options.md` | design-optioneer | 2-3 architecture options |
| `adr.md` | adr-author | Chosen option with rationale |
| `api_contracts.yaml` | interface-designer | Endpoints, schemas, errors |
| `schema.md` | interface-designer | Data models, relationships |
| `migrations/*.sql` | interface-designer | Draft migrations (if needed) |
| `observability_spec.md` | observability-designer | Metrics, logs, traces, SLOs |
| `test_plan.md` | test-strategist | BDD to test types mapping |
| `work_plan.md` | work-planner | Subtasks, ordering, dependencies |
| `design_validation.md` | design-critic | Feasibility assessment |
| `policy_analysis.md` | policy-analyst | Policy compliance check |
| `plan_receipt.json` | plan-cleanup | Receipt for downstream |
| `cleanup_report.md` | plan-cleanup | Cleanup status and evidence |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan report |
| `secrets_status.json` | secrets-sanitizer | Publish gate status |
| `gh_issue_status.md` | gh-issue-manager | Issue board update status |
| `gh_report_status.md` | gh-reporter | GitHub posting status |
| `github_report.md` | gh-reporter | Report content (local copy) |
| `git_status.md` | repo-operator | Git tree status (if anomaly detected) |

---

## Orchestrator Kickoff

### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator: ensure run/<run-id> branch
- [ ] clarifier (plan open questions)
- [ ] impact-analyzer
- [ ] design-optioneer
- [ ] adr-author
- [ ] interface-designer / observability-designer / test-strategist / work-planner (parallel)
- [ ] design-critic (microloop if needed)
- [ ] policy-analyst
- [ ] plan-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] reseal cycle (plan-cleanup ↔ secrets-sanitizer) if modified_files
- [ ] repo-operator checkpoint (checkpoint mode; capture Repo Operator Result)
- [ ] gh-issue-manager (only if safe_to_publish AND proceed_to_github_ops)
- [ ] gh-reporter (only if safe_to_publish AND proceed_to_github_ops)
- [ ] finalize flow_plan.md summary

### Agent call order
1) run-prep
2) repo-operator (ensure run branch)
3) clarifier
4) impact-analyzer
5) design-optioneer
6) adr-author
7) interface-designer + observability-designer + test-strategist + work-planner (parallel)
8) design-critic (loop if needed)
9) policy-analyst
10) plan-cleanup
11) secrets-sanitizer (read Gate Result)
12) (reseal cycle if needed)
13) repo-operator (checkpoint; read Repo Operator Result)
14) gh-issue-manager (if allowed)
15) gh-reporter (if allowed)
