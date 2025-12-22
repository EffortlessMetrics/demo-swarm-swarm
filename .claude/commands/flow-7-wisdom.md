---

description: Run Flow 7 (Prod -> Wisdom): analyze artifacts, detect regressions, extract learnings, close feedback loops.

---



# Flow 7: Prod -> Wisdom



You are orchestrating Flow 7 of the SDLC swarm.



## Working Directory + Paths (Invariant)



- All commands run from **repo root**.

- All paths in this doc are **repo-root-relative**.

- Run artifacts live under: `.runs/<run-id>/`

- Flow artifacts live under: `.runs/<run-id>/wisdom/`

- Do **not** rely on `cd` into any folder to make relative paths work.



**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/wisdom/` exists.



#### Artifact visibility rule



* Do **not** attempt to prove files exist under `.runs/<run-id>/` **before** `signal-run-prep` / `run-prep`.

* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flows verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.

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



Flow 7 uses **two complementary state machines**:



1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)

2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)



### Setup Steps



1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.

   - Track at the behavioral/station level, NOT per agent call.



2. Mirror the same list into `.runs/<run-id>/wisdom/flow_plan.md` as checkboxes.

   - As each station completes: mark TodoWrite done AND tick the checkbox.



### Suggested TodoWrite Items



```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- artifact-auditor (verify artifacts)
- regression-analyst (analyze regressions)
- flow-historian (build history)
- learning-synthesizer (synthesize learnings)
- feedback-applier (draft actions only; no gh issue create before secrets gate)
- traceability-auditor (run-level coherence + spec traceability)
- risk-analyst (compare predicted vs actual)
- wisdom-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate; capture Gate Result block)
- wisdom-cleanup ↔ secrets-sanitizer (reseal cycle; if `modified_files: true`)
- repo-operator (checkpoint commit; allowlist interlock)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report learnings; gated)
```



### On Rerun



If running `/flow-7-wisdom` on an existing run-id:

- Read `.runs/<run-id>/wisdom/flow_plan.md`

- Create TodoWrite from the checklist

- Pre-mark items done if artifacts exist and look current

- Run remaining stations to refine



This flow uses **flow artifacts and git/GitHub**. No external observability platform required.



**For production extensions** (metrics, logs, traces, incidents, SLOs): extend this flow with your observability platform.



## Subagents to use



**Infrastructure (Step 0)**:

- **run-prep** -- MUST be called first to establish the run directory and `.runs/<run-id>/wisdom/`



Domain agents (Flow 7):

- artifact-auditor

- regression-analyst

- flow-historian

- learning-synthesizer

- feedback-applier

- traceability-auditor



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



**If upstream artifacts are missing**: Flow 7 can start without all prior flows. Proceed best-effort: analyze what's available, document gaps, set status to UNVERIFIED, and continue.



## Orchestration outline



This is a **linear pipeline** except for the reseal convergence cycle (`wisdom-cleanup  secrets-sanitizer`) when `modified_files: true`.



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

# Flow 7: Wisdom for <run-id>



## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch)
- [ ] artifact-auditor (verify all flow artifacts)
- [ ] regression-analyst (analyze test/coverage regressions)
- [ ] flow-historian (build timeline)
- [ ] learning-synthesizer (extract learnings)
- [ ] feedback-applier (draft actions; no gh issue create before secrets gate)
- [ ] traceability-auditor (run-level coherence + spec traceability)
- [ ] risk-analyst (compare predicted vs actual)
- [ ] wisdom-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] wisdom-cleanup ↔ secrets-sanitizer (reseal cycle; if `modified_files: true`)
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

- **Calculate Dev Lead Time (DevLT):** Derive `human_attention_time` vs `total_run_time` from timestamps and human interaction markers. Goal: prove that `Human Attention Time << Total Run Time`. This closes the loop on the economic argument that swarm compute is cheaper than human attention.



### Step 5: Synthesize learnings

- `learning-synthesizer` -> read artifact audit, regression report, flow history

- Extract patterns: what worked, what didn't, assumptions that broke

- Produce `.runs/<run-id>/wisdom/learnings.md` narrative with feedback to Flows 1, 2, 3



### Step 6: Apply feedback

- `feedback-applier` -> turn learnings into concrete actions

- Produce `.runs/<run-id>/wisdom/feedback_actions.md` with actionable items

- **Write issue drafts to `feedback_actions.md`** - do NOT call `gh issue create` here

- Actual GitHub issue creation happens **after** secrets gate passes (Step 10)



### Step 6b: Traceability audit

- `traceability-auditor` -> `.runs/<run-id>/wisdom/traceability_audit.md`

- Verifies run identity + receipts + GitHub observability markers (read-only; gated on access + gh auth)



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

- **Returns a Gate Result block**  this is the control plane for routing decisions



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

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV

route_to_flow: 1 | 2 | 3 | 4 | 5 | 6 | 7 | null

route_to_station: <string | null>

route_to_agent: <agent-name | null>

```

<!-- PACK-CONTRACT: GATE_RESULT_V1 END -->



**Gating logic (from Gate Result block):**

- If `safe_to_commit: true` → proceed to checkpoint commit (Step 9c)
- If `safe_to_commit: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`. GitHub reporting ops still run in RESTRICTED mode when publish is blocked or `publish_surface: NOT_PUSHED`.



### Step 9b: Reseal If Modified (Conditional Loop)



If the prior `secrets-sanitizer` reports `modified_files: true`, repeat `(wisdom-cleanup  secrets-sanitizer)` until either:

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



### Step 9c: Checkpoint Commit



Checkpoint the audit trail **before** any GitHub operations.



**Call `repo-operator`** with checkpoint mode. The agent:

1. Resets staging and stages only the allowlist (not `git add .`)

2. Enforces the allowlist/anomaly interlock mechanically

3. Writes `.runs/<run-id>/wisdom/git_status.md` if anomaly detected

4. Handles no-op (nothing staged) gracefullyno empty commits



**Allowlist for Flow 7:**

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

publish_surface: PUSHED | NOT_PUSHED

anomaly_paths: []

```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null.



Orchestrators route on this block, not by re-reading `git_status.md`.



**Anomaly detection:** If anything outside allowlist is dirty (modified/staged/untracked):

- **Anomaly detected**  commit allowlist only

- Set `proceed_to_github_ops: false`

- Write `.runs/<run-id>/wisdom/git_status.md` documenting unexpected paths

- Flow completes locally **UNVERIFIED**



**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**

- If `safe_to_commit: false` (from Gate Result): skip commit entirely

- If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`

- If `safe_to_publish: true` AND no anomaly: `repo-operator` pushes the branch, returns `proceed_to_github_ops: true`

- If `safe_to_publish: false`:

  - If `needs_upstream_fix: true`  **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`

  - If `status: BLOCKED_PUBLISH`  **CANNOT_PROCEED** (mechanical failure); stop and require human intervention

  - Otherwise → UNVERIFIED; skip push (`publish_surface: NOT_PUSHED`). Continue with GitHub Reporting Ops in RESTRICTED mode when access allows.



### Step 10-11: GitHub Reporting (Final)

**Call `gh-issue-manager`** (marks run complete) then **`gh-reporter`** (mini-postmortem).

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

**Content for postmortem:** Learnings, pack/flow observations, feedback actions, meta-notes on the wisdom synthesis.

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



Flow 7 closes the SDLC loop by feeding learnings back (recommendations, not direct calls):



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
- If Build produced hardening worklists (e.g., `build/mutation_report.md`, `build/fuzz_report.md`, `build/flakiness_report.md`), promote the top items into `feedback_actions.md` as issue drafts (with evidence pointers).



These are **recommendations in artifacts**, not direct flow invocations. Humans decide which to act on.



## Expected Outputs



When complete, `.runs/<run-id>/wisdom/` should contain:



- `flow_plan.md` - execution plan and progress

- `artifact_audit.md` - structural sanity check of all flows

- `regression_report.md` - what got worse and where

- `flow_history.json` - timeline linking all flow events + DevLT metrics

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



Flow 7 agents report:



- **VERIFIED**: `blockers` empty, `missing_required` empty, and analysis complete with all artifacts processed. Set `recommended_action: PROCEED`.

- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR some data unavailable (GitHub, git, etc.) OR anomaly detected during checkpoint. Set `recommended_action: RERUN | BOUNCE` depending on fix location.

- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.



**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.



Any of these are valid outcomes. Document concerns and continue.



## Stable Marker Contract (for mechanical counting)



Flow 7 producers must use these stable markers so `wisdom-cleanup` can derive counts mechanically:



| Agent | Marker Pattern | Artifact | Example |

|-------|----------------|----------|---------|

| regression-analyst | `^### REG-[0-9]{3}:` | regression_report.md | `### REG-001: test_foo::bar  assertion failed` |

| learning-synthesizer | `^## Learning: ` | learnings.md | `## Learning: Requirements` |

| feedback-applier | `^- ISSUE: ` | feedback_actions.md | `- ISSUE: Missing tests for REQ-004` |

| flow-historian | `"devlt":` | flow_history.json | `"devlt": {"total_run_minutes": 45, "human_attention_minutes": 8}` |



**Regression format rule:** Each regression MUST have exactly one `### REG-NNN:` heading section. (You may also include a register table, but headings are the source for counting.)



**Why this matters:** Without stable markers, `wisdom-cleanup` cannot derive counts mechanically and must set them to `null` with reasons. Agents that omit markers degrade receipt quality.



---



## Orchestrator Kickoff


### Station order

#### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `artifact-auditor`

4. `regression-analyst`

5. `flow-historian`

6. `learning-synthesizer`

7. `feedback-applier`

8. `traceability-auditor`

9. `risk-analyst`

10. `wisdom-cleanup`

11. `secrets-sanitizer`

12. `wisdom-cleanup` ↔ `secrets-sanitizer` (reseal cycle; if `modified_files: true`)

13. `repo-operator` (checkpoint commit)

14. `gh-issue-manager` (if allowed)

15. `gh-reporter` (if allowed)

### TodoWrite (copy exactly)

- [ ] run-prep

- [ ] repo-operator (ensure `run/<run-id>` branch)

- [ ] artifact-auditor
- [ ] regression-analyst
- [ ] flow-historian
- [ ] learning-synthesizer
- [ ] feedback-applier (draft actions only; no gh issue create before secrets gate)
- [ ] traceability-auditor (run-level coherence + spec traceability)
- [ ] risk-analyst
- [ ] wisdom-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] wisdom-cleanup ↔ secrets-sanitizer (reseal cycle; if `modified_files: true`)
- [ ] repo-operator (checkpoint commit; allowlist interlock + no-op handling)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)

- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.
