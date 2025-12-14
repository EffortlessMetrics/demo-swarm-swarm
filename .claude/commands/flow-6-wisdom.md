---
description: Run Flow 6 (Prod -> Wisdom): analyze artifacts, detect regressions, extract learnings, close feedback loops.
---

# Flow 6: Prod -> Wisdom

You are orchestrating Flow 6 of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/wisdom/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/wisdom/` exists.

#### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

## Your Goals

- Verify all flow artifacts exist
- Analyze tests, coverage, and regressions
- Correlate with GitHub issues and git blame
- Compile flow timeline
- Extract learnings from receipts and critiques
- Suggest feedback actions (issues, doc updates)
- Add risk perspective comparing predicted vs actual
- Post learnings and action items to GitHub

## Before You Begin (Required)

### Two State Machines

Flow 6 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.

2. Mirror the same list into `.runs/<run-id>/wisdom/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

### Suggested TodoWrite Items

```
- Establish run infrastructure (run-prep)
- Ensure run branch (repo-operator)
- Verify artifacts (artifact-auditor)
- Analyze regressions (regression-analyst)
- Build history (flow-historian)
- Synthesize learnings (learning-synthesizer)
- Apply feedback (feedback-applier: draft actions only)
- Risk assessment (risk-analyst)
- Finalize receipt (wisdom-cleanup)
- Sanitize secrets (secrets-sanitizer: capture Gate Result)
- Reseal cycle if modified_files
- Checkpoint commit (repo-operator: allowlist interlock)
- Update issue board (gh-issue-manager; gated)
- Report learnings (gh-reporter; gated)
- Update flow_plan.md summary
```

### On Rerun

If running `/flow-6-wisdom` on an existing run-id:
- Read `.runs/<run-id>/wisdom/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

This flow uses **flow artifacts and git/GitHub**. No external observability platform required.

**For production extensions** (metrics, logs, traces, incidents, SLOs): extend this flow with your observability platform.

## Subagents to use

**Infrastructure (Step 0)**:
- **run-prep** -- MUST be called first to establish the run directory and `.runs/<run-id>/wisdom/`

Domain agents (Flow 6):
- artifact-auditor
- regression-analyst
- flow-historian
- learning-synthesizer
- feedback-applier

Cross-cutting agents:
- risk-analyst

Cleanup + Reporting (End of Flow):
- wisdom-cleanup -- writes wisdom_receipt.json, updates index.json status
- secrets-sanitizer -- publish gate (returns Gate Result block)
- repo-operator -- checkpoint commit (gated on Gate Result + anomaly check)
- gh-issue-manager -- updates issue body status board (final update)
- gh-reporter -- posts mini-postmortem summary

## Upstream Inputs

Read from all prior flow directories (if available):
- `.runs/<run-id>/signal/signal_receipt.json`
- `.runs/<run-id>/plan/plan_receipt.json`
- `.runs/<run-id>/build/build_receipt.json`
- `.runs/<run-id>/gate/gate_receipt.json`
- `.runs/<run-id>/deploy/deploy_receipt.json`

**If upstream artifacts are missing**: Flow 6 can start without all prior flows. Proceed best-effort: analyze what's available, document gaps, set status to UNVERIFIED, and continue.

## Orchestration outline

This is a **linear pipeline** except for the reseal convergence cycle (`wisdom-cleanup ↔ secrets-sanitizer`) when `modified_files: true`.

### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/wisdom/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "wisdom" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/wisdom/`.

### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/wisdom/flow_plan.md`:

```markdown
# Flow 6: Wisdom for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch)
- [ ] artifact-auditor (verify all flow artifacts)
- [ ] regression-analyst (analyze test/coverage regressions)
- [ ] flow-historian (build timeline)
- [ ] learning-synthesizer (extract learnings)
- [ ] feedback-applier (draft actions; no gh issue create before secrets gate)
- [ ] risk-analyst (compare predicted vs actual)
- [ ] wisdom-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] reseal cycle (if modified_files)
- [ ] repo-operator (checkpoint commit with allowlist interlock)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

### Step 2: Verify artifacts
- `artifact-auditor` -> walk all `.runs/<run-id>/<flow>/` directories
- Check expected artifacts against flow specs
- Produce `.runs/<run-id>/wisdom/artifact_audit.md` with matrix of flows vs artifacts

### Step 3: Analyze regressions
- `regression-analyst` -> parse test outputs, coverage reports
- Correlate with GitHub issues via `gh issue list`
- Run `git blame` on failing tests to link commits
- Produce `.runs/<run-id>/wisdom/regression_report.md` with findings by type and severity

### Step 4: Build history
- `flow-historian` -> read all artifacts and git history
- Compile `.runs/<run-id>/wisdom/flow_history.json` timeline linking signal -> spec -> design -> build -> gate -> deploy
- Include timestamps, commits, decision points

### Step 5: Synthesize learnings
- `learning-synthesizer` -> read artifact audit, regression report, flow history
- Extract patterns: what worked, what didn't, assumptions that broke
- Produce `.runs/<run-id>/wisdom/learnings.md` narrative with feedback to Flows 1, 2, 3

### Step 6: Apply feedback
- `feedback-applier` -> turn learnings into concrete actions
- Produce `.runs/<run-id>/wisdom/feedback_actions.md` with actionable items
- **Write issue drafts to `feedback_actions.md`** — do NOT call `gh issue create` here
- Actual GitHub issue creation happens **after** secrets gate passes (Step 10)

### Step 7: Risk assessment
- `risk-analyst` (cross-cutting) -> add risk perspective to learnings
- Compare predicted risks (`.runs/<run-id>/signal/early_risks.md`) vs actual outcomes
- Produce `.runs/<run-id>/wisdom/risk_assessment.md` or append to existing artifacts

### Step 8: Finalize and Write Receipt
- `wisdom-cleanup` -> `.runs/<run-id>/wisdom/wisdom_receipt.json`, `.runs/<run-id>/wisdom/cleanup_report.md`
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Updates `.runs/index.json` with status, last_flow, updated_at
- This is the final receipt for the run

### Step 9: Sanitize Secrets (Publish Gate)
- `secrets-sanitizer` -> `.runs/<run-id>/wisdom/secrets_scan.md`, `.runs/<run-id>/wisdom/secrets_status.json`
- Scans all wisdom artifacts before posting
- **Returns a Gate Result block** — this is the control plane for routing decisions

**Status vs flags:**
- `status` is descriptive (CLEAN/FIXED/BLOCKED_PUBLISH)
- `safe_to_commit` / `safe_to_publish` are authoritative

**Control plane:** Route on the **Gate Result block** returned by `secrets-sanitizer`. `secrets_status.json` is audit-only (optional last-mile verification).

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

**Gating logic (from Gate Result block):**
- Proceed only if Gate Result `safe_to_publish: true`
- If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`
- If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention

### Step 9b: Reseal If Modified (Conditional Loop)

If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(wisdom-cleanup → secrets-sanitizer)` until either:
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

### Step 9c: Checkpoint Commit

Checkpoint the audit trail **before** any GitHub operations.

**Call `repo-operator`** with checkpoint mode. The agent:
1. Resets staging and stages only the allowlist (not `git add .`)
2. Enforces the allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/wisdom/git_status.md` if anomaly detected
4. Handles no-op (nothing staged) gracefully—no empty commits

**Allowlist for Flow 6:**
- `.runs/<run-id>/wisdom/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

**Control plane:** `repo-operator` returns a Repo Operator Result block:
```
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
anomaly_paths: []
```
**Note:** `commit_sha` is always populated (current HEAD on no-op), never null.

Orchestrators route on this block, not by re-reading `git_status.md`.

**Anomaly detection:** If anything outside allowlist is dirty (modified/staged/untracked):
- **Anomaly detected** → commit allowlist only
- Set `proceed_to_github_ops: false`
- Write `.runs/<run-id>/wisdom/git_status.md` documenting unexpected paths
- Flow completes locally **UNVERIFIED**

**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
- If `safe_to_commit: false` (from Gate Result): skip commit entirely
- If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`
- If `safe_to_publish: true` AND no anomaly: `repo-operator` pushes the branch, returns `proceed_to_github_ops: true`
- If `safe_to_publish: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → UNVERIFIED; skip external ops

### Step 10: Update Issue Board (Final)

**Prerequisites:**
- Gate Result `safe_to_publish: true`
- Repo Operator Result `proceed_to_github_ops: true`

Both must be true to proceed. If either fails: **SKIP** this step. Flow stays UNVERIFIED.

- `gh-issue-manager` -> final update to issue body status board
- **Creates GitHub issue if none exists** (allowed in any flow; includes "Signal pending" banner if created from Flow 6)
- Marks run as complete

**Note on follow-up issues:** The issues drafted in Step 6 (`feedback_actions.md`) remain as **drafts for human review**. `gh-issue-manager` does NOT auto-create follow-up issues—this keeps the agent's responsibility focused on "run issue + status board". Humans decide which follow-up actions to pursue.

### Step 11: Report Mini-Postmortem

**Prerequisites:**
- Gate Result `safe_to_publish: true`
- Repo Operator Result `proceed_to_github_ops: true`

Both must be true to proceed. If either fails: **SKIP** this step. Flow stays UNVERIFIED.

- If `safe_to_publish: false` or `proceed_to_github_ops: false` → SKIP; document why.
- If `gh` CLI unauthenticated/unavailable → SKIP with evidence (not BLOCKED).
- Otherwise (gates true and gh available) → GitHub ops must run.

- `gh-reporter` -> post mini-postmortem summary **to the GitHub issue** (not PR)
- Include regressions found, learnings extracted, feedback actions
- **Issue-first (hard):** All flow logs go to the issue, even if a PR exists. PRs are for PR-review dynamics only.

### Step 12: Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Regressions Found**: <count>
- **Learnings Extracted**: <count>
- **Feedback Actions Created**: <count>
- **Run Complete**: This run-id is now closed

## Human Review Checklist

- [ ] `.runs/<run-id>/wisdom/learnings.md` - Are learnings actionable?
- [ ] `.runs/<run-id>/wisdom/feedback_actions.md` - Which actions should be prioritized?
- [ ] `.runs/<run-id>/wisdom/regression_report.md` - Are regressions understood?
```

## Closed Feedback Loops

Flow 6 closes the SDLC loop by feeding learnings back (recommendations, not direct calls):

### -> Flow 1 (Signal)
- `learning-synthesizer` extracts problem patterns
- `feedback-applier` suggests updates to requirement templates
- Builds institutional memory of "problems that recur"

### -> Flow 2 (Plan)
- `feedback-applier` suggests architecture doc updates
- Documents patterns that worked/failed
- Improves design templates and ADR prompts

### -> Flow 3 (Build)
- `feedback-applier` drafts GitHub issues for test gaps (for human review)
- Links regression failures to coverage gaps
- Suggests test pattern improvements

These are **recommendations in artifacts**, not direct flow invocations. Humans decide which to act on.

## Expected Outputs

When complete, `.runs/<run-id>/wisdom/` should contain:

- `flow_plan.md` - execution plan and progress
- `artifact_audit.md` - structural sanity check of all flows
- `regression_report.md` - what got worse and where
- `flow_history.json` - timeline linking all flow events
- `learnings.md` - narrative lessons extracted
- `feedback_actions.md` - concrete follow-ups (issues, doc updates)
- `risk_assessment.md` - risk perspective (optional, if risk-analyst invoked)
- `wisdom_receipt.json` - final receipt for the run
- `cleanup_report.md` - cleanup status and evidence
- `secrets_scan.md` - secrets scan report
- `secrets_status.json` - publish gate status
- `git_status.md` - repo state at checkpoint (if anomaly detected)
- `gh_issue_status.md` - issue board update status
- `gh_report_status.md` - GitHub posting status
- `github_report.md` - report content (local copy)

## Completion States

Flow 6 agents report:

- **VERIFIED**: `blockers` empty, `missing_required` empty, and analysis complete with all artifacts processed. Set `recommended_action: PROCEED`.
- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR some data unavailable (GitHub, git, etc.) OR anomaly detected during checkpoint. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.

Any of these are valid outcomes. Document concerns and continue.

## Stable Marker Contract (for mechanical counting)

Flow 6 producers must use these stable markers so `wisdom-cleanup` can derive counts mechanically:

| Agent | Marker Pattern | Artifact | Example |
|-------|----------------|----------|---------|
| regression-analyst | `^### REG-[0-9]{3}:` | regression_report.md | `### REG-001: test_foo::bar — assertion failed` |
| learning-synthesizer | `^## Learning: ` | learnings.md | `## Learning: Requirements` |
| feedback-applier | `^- ISSUE: ` | feedback_actions.md | `- ISSUE: Missing tests for REQ-004` |

**Regression format rule:** Each regression MUST have exactly one `### REG-NNN:` heading section. (You may also include a register table, but headings are the source for counting.)

**Why this matters:** Without stable markers, `wisdom-cleanup` cannot derive counts mechanically and must set them to `null` with reasons. Agents that omit markers degrade receipt quality.

---

## Orchestrator Kickoff

### TodoWrite (copy exactly)

- [ ] run-prep
- [ ] repo-operator: ensure `run/<run-id>` branch
- [ ] artifact-auditor
- [ ] regression-analyst
- [ ] flow-historian
- [ ] learning-synthesizer
- [ ] feedback-applier (draft actions only; no gh issue create before secrets gate)
- [ ] risk-analyst
- [ ] wisdom-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] reseal cycle (wisdom-cleanup ↔ secrets-sanitizer) if modified_files
- [ ] repo-operator: checkpoint commit (allowlist interlock + no-op handling)
- [ ] gh-issue-manager (only if safe_to_publish AND proceed_to_github_ops)
- [ ] gh-reporter (only if safe_to_publish AND proceed_to_github_ops)

### Agent call order

1) run-prep
2) repo-operator (ensure run branch)
3) artifact-auditor → regression-analyst → flow-historian → learning-synthesizer → feedback-applier → risk-analyst
4) wisdom-cleanup
5) secrets-sanitizer (read Gate Result)
6) (reseal cycle if needed)
7) repo-operator (checkpoint commit)
8) gh-issue-manager (if allowed)
9) gh-reporter (if allowed)
