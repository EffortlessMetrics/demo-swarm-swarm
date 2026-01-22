---
name: flow-2-plan
description: Run Flow 2 (Spec to Design): produce ADR, contracts, observability spec, test/work plans, design validation.
---

# Flow 2: Spec to Design

You are the PM orchestrating Flow 2 of the SDLC swarm. Your team of specialist agents transforms requirements into architecture decisions, API contracts, and execution plans.

**Your role:** You direct agents, read their reports, and decide what happens next. You do not parse files or extract fields. You understand your agents' prose and route on their recommendations.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/plan/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/plan/` exists.

#### Artifact visibility rule

- Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
- If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
- Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

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
   - Microloops (`design-optioneer` ↔ `option-critic`, `interface-designer` ↔ `contract-critic`, `observability-designer` ↔ `observability-critic`) are ONE todo each.

2. Mirror the same list into `.runs/<run-id>/plan/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure; initialize `flow_plan.md`)
- repo-operator (ensure run branch)
- clarifier (plan open questions)
- impact-analyzer (map impact)
- design-optioneer ↔ option-critic (microloop; signal-based termination)
- adr-author (write ADR)
- interface-designer / observability-designer / test-strategist / work-planner (lanes; parallel)
- interface-designer ↔ contract-critic (microloop; signal-based termination; recommended)
- observability-designer ↔ observability-critic (microloop; signal-based termination; recommended)
- design-critic (integrative validation; may return worklist)
- policy-analyst (policy compliance)
- plan-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (update issue status board; gated)
- gh-reporter (post Plan summary; gated)
```

### Critic choreography (default behavior)

Think in **worklists**, not "who wins".

- **Signal-based microloop:** writer → critic → route on handoff. If critic recommends improvements and says further iteration will help: call writer with critique worklist, then call critic again. Otherwise proceed (carry blockers honestly).
- **Option critique (early):** Apply microloop pattern between `design-optioneer` and `option-critic`.
- **Lane worklists:** If `contract-critic` or `observability-critic` recommends fixes or changes, treat that as the active worklist for its lane unless you resolve it or explicitly defer it (Decision Log entry).
- **Integration read (late):** `design-critic` is integrative across artifacts. Run it after lane worklists are resolved/deferred. A later `design-critic` `PROCEED` does not clear an open lane worklist.

### Decision log (only when you defer a critic worklist)

If you intentionally proceed while a critic still has an open worklist (e.g., you choose not to rerun/bounce), record a short entry in `.runs/<run-id>/plan/flow_plan.md` capturing what you deferred, why, evidence, and what you will re-check before sealing `plan_receipt.json`.

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
- option-critic
- adr-author
- interface-designer
- contract-critic
- observability-designer
- observability-critic
- test-strategist
- work-planner
- design-critic

### Cross-cutting agents

- clarifier (Plan-local open questions)
- intent-auditor (audit ADR/BDD/REQ coherence; optional, when coherence issues suspected)
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
- [ ] design-optioneer ↔ option-critic (microloop; apply Microloop Template)
- [ ] adr-author (write architecture decision)
- [ ] interface-designer (contracts/schema; lane; parallel)
- [ ] interface-designer ↔ contract-critic (microloop; apply Microloop Template)
- [ ] observability-designer (observability; lane; parallel)
- [ ] observability-designer ↔ observability-critic (microloop; apply Microloop Template)
- [ ] test-strategist (test plan; lane; parallel)
- [ ] work-planner (work plan; lane; parallel)
- [ ] design-critic (integrative validation; may return worklist)
- [ ] policy-analyst (check compliance)
- [ ] plan-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>

## Decision Log (only when you defer a critic worklist)

- Deferred: <critic-name> requested <RERUN|BOUNCE|FIX_ENV> on <artifact> -> proceeding with <action>
  - Why: <short>
  - Evidence: <artifact/path pointers>
  - Re-check before seal: <what you will re-verify before plan-cleanup>
```

### Step 2: Plan Open Questions (Non-blocking)

Call `clarifier` to create the Plan-local questions register. Signal's `open_questions.md` is upstream input; Plan gets its own register for design-phase questions.

### Step 3: Map impact

- Use `impact-analyzer` to map impact and blast radius.

### Step 4: Propose design options

- Use `design-optioneer` to propose design options.

### Step 4b: Critique design options (microloop; recommended)

- Use `option-critic` to critique `design_options.md` and write `option_critique.md`.

**Route on the critic's handoff:**
Read the critic's report. They will tell you what they found and what they recommend:

- If the critic recommends "proceed" or says options are ready → move forward
- If the critic recommends fixes → run the optioneer with their feedback, then ask the critic again
- If the critic reports a mechanical failure → stop and address the environment issue

When you defer issues the critic raised, record why in the Decision Log.

### Step 5: Write ADR

- Use `adr-author` to write the ADR.

### Step 6: Define contracts and schema (FIRST - others depend on this)

- Use `interface-designer` for contracts/schema/migrations (planned migrations live under the run directory; actual migrations move during Build).
- **This must complete before Steps 8-9** because:
  - `test-strategist` reads `schema.md` to plan test data/fixture updates
  - `test-strategist` reads `api_contracts.yaml` to generate contract-bound ACs
  - `work-planner` reads `migrations/` to schedule infrastructure subtasks (ST-000)

### Step 6b: Validate contracts (microloop; recommended)

- Use `contract-critic` to validate `api_contracts.yaml` + `schema.md` and write `contract_critique.md`.

**Route on the critic's handoff:**
Read the critic's report. They will tell you what they found and what they recommend:

- If the critic recommends "proceed" or says contracts are ready → move forward
- If the critic recommends fixes → run the designer with their feedback, then ask the critic again
- If the critic reports a mechanical failure → stop and address the environment issue

When the critic recommends changes, that becomes your active worklist for this lane. Resolve it or defer it (with a Decision Log entry explaining why).

### Step 7: Plan observability (parallel)

- Use `observability-designer` to define observability.

### Step 7b: Validate observability (microloop; recommended)

- Use `observability-critic` to validate `observability_spec.md` and write `observability_critique.md`.

**Route on the critic's handoff:**
Read the critic's report. They will tell you what they found and what they recommend:

- If the critic recommends "proceed" or says observability spec is ready → move forward
- If the critic recommends fixes → run the designer with their feedback, then ask the critic again
- If the critic reports a mechanical failure → stop and address the environment issue

When the critic recommends changes, that becomes your active worklist for this lane. Resolve it or defer it (with a Decision Log entry explaining why).

### Step 8: Plan testing (after interface-designer)

- Use `test-strategist` to write the test plan (incorporate Signal BDD + verification notes).
- **Requires:** `schema.md` (for fixture planning) and `api_contracts.yaml` (for contract-to-AC binding)

### Step 9: Plan work (after interface-designer)

- Use `work-planner` — "produce subtask index + work plan".
- **Requires:** `migrations/` (to sequence infrastructure subtasks as ST-000 prerequisites)

### Step 10: Validate design (microloop)

- Use `design-critic` to validate the design.

**Conflict handling (default):**

- If a targeted critic still recommends fixes or changes, keep that lane's worklist open until resolved or explicitly deferred (Decision Log entry in `flow_plan.md`). You can still run `design-critic` for an integration read.

**Route on the design-critic's handoff:**
Read the critic's integrative report. This is the final quality check before policy. Trust their assessment:

- If the critic recommends "proceed" → move to policy check
- If the critic recommends improvements and believes another iteration will help → address the specific areas they name
- If the critic says "no further improvement possible" → proceed with documented issues

Your agents produce reports and recommendations. You decide what happens next based on their guidance.

**Reseal-if-modified:** If the design-critic's feedback loop causes artifact modifications (interface-designer rerun, adr-author amendment, etc.), you must call `plan-cleanup` again to regenerate `plan_receipt.json` before the final seal. The receipt must reflect the final state of artifacts, not an intermediate state.

### Step 11: Check policy compliance

- Use `policy-analyst` for policy compliance.

### Step 12: Finalize Plan (receipt + index)

- Use `plan-cleanup` to seal the receipt, verify artifacts, and update index counts mechanically.

### Step 13: Sanitize secrets (publish gate)

- Use `secrets-sanitizer` (publish gate).

**Secrets-sanitizer reports status in its handoff.** Example:

> Secrets scan complete. Status: CLEAN. No findings. Safe to commit and publish.

For audit purposes, it also writes `secrets_status.json` with fields:

- `status`: CLEAN, FIXED, or BLOCKED (descriptive — never infer permissions from it)
- `safe_to_commit` / `safe_to_publish`: authoritative permissions
- `modified_files`: whether artifact files were changed
- `findings_count`: number of issues found
- `blocker_kind`: NONE, MECHANICAL, SECRET_IN_CODE, or SECRET_IN_ARTIFACT

The handoff is the routing signal. `secrets_status.json` is the durable audit record.

**Gating logic (boolean gate — the sanitizer says yes/no, orchestrator decides next steps):**

- The sanitizer is a fix-first pre-commit hook, not a router
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 13c)
- If `safe_to_commit: false`:
  - `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO failure)
  - `blocker_kind: SECRET_IN_CODE` → route to appropriate agent (orchestrator decides)
  - `blocker_kind: SECRET_IN_ARTIFACT` → investigate manually
- Publish mode gating: `FULL` only when `safe_to_publish: true`, Repo Operator Result `proceed_to_github_ops: true`, **and** `publish_surface: PUSHED`. Otherwise, GitHub ops (when access is allowed) run in `RESTRICTED` mode. Publish blocked implies RESTRICTED, **not skip**.

### Step 13b: Checkpoint Commit

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
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null. `publish_surface` must always be present (PUSHED or NOT_PUSHED), even on no-op commits, anomalies, `safe_to_commit: false`, push skipped, or push failure.

**Routing logic (from Repo Operator Result):**

- `status: COMPLETED` + `proceed_to_github_ops: true` → proceed to GitHub ops
- `status: COMPLETED_WITH_ANOMALY` → allowlist committed, anomaly documented in `git_status.md`; `proceed_to_github_ops: false`
- `status: FAILED` or `status: CANNOT_PROCEED` → mechanical failure; stop and require human intervention

**Gating interaction with secrets-sanitizer:**

- `repo-operator` reads `safe_to_commit` and `safe_to_publish` from the prior Gate Result
- If `safe_to_commit: false`: skips commit entirely
- If `safe_to_publish: false`: commits locally but skips push; sets `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`

**Why checkpoint before GitHub ops:** The issue comment can reference a stable commit SHA. Also keeps local history clean if the flow is interrupted.

### Step 14-15: GitHub Reporting

**Call `gh-issue-manager`** then **`gh-reporter`** to update the issue.

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:

- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

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
- `option_critique.md` - Options critique + worklist (decision readiness)
- `adr.md` - Chosen option with rationale and consequences
- `api_contracts.yaml` - Endpoints, schemas, error shapes
- `schema.md` - Data models, relationships, invariants
- `migrations/*.sql` - Draft migrations (optional, if DB changes needed)
- `observability_spec.md` - Metrics, logs, traces, SLOs, alerts
- `test_plan.md` - BDD to test types mapping, priorities
- `ac_matrix.md` - AC-driven build contract (Flow 3 iterates per AC; Build creates `build/ac_status.json` at runtime)
- `work_plan.md` - Subtasks, ordering, dependencies
- `design_validation.md` - Feasibility assessment, known issues

## Understanding Agent Reports

Your agents report what they did and what they recommend. Read their prose and follow their guidance:

- When an agent says **"ready" or "proceed"** → they are satisfied and you should move forward
- When an agent says **"needs X" or "fix Y"** → they identified work and are telling you what to do next
- When an agent says **"blocked"** → something mechanical failed (IO, permissions, tooling). Address the environment issue.

**PARTIAL is a success.** If a flow ends PARTIAL with honest documentation of what's done and what remains, that's a valid checkpoint. The flow is resumable; state is on disk.

**Key rule**: "Blocked" means mechanical failure only. Missing upstream artifacts are "incomplete/unverified" with documented assumptions, not "blocked".

## Notes

- **Lane dependencies (Materials-First sequencing):**
  - `interface-designer` (Step 6) must complete first — produces `schema.md`, `api_contracts.yaml`, `migrations/`
  - `observability-designer` (Step 7) can run in parallel with Step 6
  - `test-strategist` (Step 8) depends on Step 6 outputs (contract-to-AC binding, fixture planning)
  - `work-planner` (Step 9) depends on Step 6 outputs (infrastructure subtask sequencing)
- `design-critic` reviews ALL artifacts before policy check
- `option-critic` critiques options before ADR authoring
- Human gate at end: "Is this the right design?"
- Agents never block; they document concerns and continue

## Artifact Outputs

All written to `.runs/<run-id>/plan/`:

| Artifact                    | Source Agent           | Description                                                     |
| --------------------------- | ---------------------- | --------------------------------------------------------------- |
| `flow_plan.md`              | orchestrator           | Execution plan and progress                                     |
| `open_questions.md`         | clarifier              | Plan-local questions register                                   |
| `impact_map.json`           | impact-analyzer        | Affected services, modules, data                                |
| `design_options.md`         | design-optioneer       | 2-3 architecture options                                        |
| `option_critique.md`        | option-critic          | Options critique + worklist                                     |
| `adr.md`                    | adr-author             | Chosen option with rationale                                    |
| `api_contracts.yaml`        | interface-designer     | Endpoints, schemas, errors                                      |
| `schema.md`                 | interface-designer     | Data models, relationships                                      |
| `migrations/*.sql`          | interface-designer     | Draft migrations (if needed)                                    |
| `contract_critique.md`      | contract-critic        | Contract validation critique (optional)                         |
| `observability_spec.md`     | observability-designer | Metrics, logs, traces, SLOs                                     |
| `observability_critique.md` | observability-critic   | Observability validation critique (optional)                    |
| `test_plan.md`              | test-strategist        | BDD to test types mapping                                       |
| `ac_matrix.md`              | test-strategist        | AC-driven build contract (Build creates `build/ac_status.json`) |
| `work_plan.md`              | work-planner           | Subtasks, ordering, dependencies                                |
| `design_validation.md`      | design-critic          | Feasibility assessment                                          |
| `policy_analysis.md`        | policy-analyst         | Policy compliance check                                         |
| `plan_receipt.json`         | plan-cleanup           | Receipt for downstream                                          |
| `cleanup_report.md`         | plan-cleanup           | Cleanup status and evidence                                     |
| `secrets_scan.md`           | secrets-sanitizer      | Secrets scan report                                             |
| `secrets_status.json`       | secrets-sanitizer      | Publish gate status                                             |
| `gh_issue_status.md`        | gh-issue-manager       | Issue board update status                                       |
| `gh_report_status.md`       | gh-reporter            | GitHub posting status                                           |
| `github_report.md`          | gh-reporter            | Report content (local copy)                                     |
| `git_status.md`             | repo-operator          | Git tree status (if anomaly detected)                           |

---

## Orchestrator Kickoff

### Station order + templates

#### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `clarifier`

4. `impact-analyzer`

5. `design-optioneer` ↔ `option-critic` (microloop; apply Microloop Template)

6. `adr-author`

7. `interface-designer` / `observability-designer` / `test-strategist` / `work-planner` (parallel)

8. `interface-designer` ↔ `contract-critic` (microloop; apply Microloop Template; recommended)

9. `observability-designer` ↔ `observability-critic` (microloop; apply Microloop Template; recommended)

10. `design-critic` (integrative validation; route to options/contracts/observability/plans as returned; rerun once to confirm the top worklist moved)

11. `policy-analyst`

12. `plan-cleanup`

13. `secrets-sanitizer`

14. `repo-operator` (checkpoint; read Repo Operator Result)

15. `gh-issue-manager` (if allowed)

16. `gh-reporter` (if allowed)

#### Microloop Template (writer ↔ critic)

Run this template for: tests, code, docs, requirements, BDD, options, contracts, observability.

1. **Writer pass:** call the writer agent
2. **Critique pass:** call the critic agent, read their handoff
3. **Route on the critic's recommendation:**
   - If the critic says "ready" or "proceed" → move forward
   - If the critic recommends improvements → run the writer with their feedback, then ask the critic again
   - If the critic says "no further improvement possible" → proceed with documented blockers

**Termination:** Trust the critic's judgment. They will tell you when to proceed. If context is exhausted, exit PARTIAL with an honest checkpoint.

### TodoWrite (copy exactly)

- [ ] run-prep
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] clarifier (plan open questions)
- [ ] impact-analyzer
- [ ] design-optioneer ↔ option-critic (microloop; signal-based termination)
- [ ] adr-author
- [ ] interface-designer / observability-designer / test-strategist / work-planner (parallel)
- [ ] interface-designer ↔ contract-critic (microloop; signal-based termination; recommended)
- [ ] observability-designer ↔ observability-critic (microloop; signal-based termination; recommended)
- [ ] design-critic (microloop if needed)
- [ ] policy-analyst
- [ ] plan-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint; capture Repo Operator Result)
- [ ] gh-issue-manager (skip when github_ops_allowed: false; FULL/RESTRICTED based on gates/publish_surface)
- [ ] gh-reporter (skip when github_ops_allowed: false; FULL/RESTRICTED based on gates/publish_surface)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.
