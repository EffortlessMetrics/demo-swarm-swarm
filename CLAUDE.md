# CLAUDE.md

This repository contains an SDLC swarm pack under `.claude/`.

**Operational reality:** This file is attached to every agent thread in Claude Code. Treat it as **repo-level policy + shared contracts** (not a marketing doc). If flow commands, agent prompts, or pack-check drift from what's written here, update the pack so everything agrees.

---

## For Humans

This pack provides:

- **7 flows**: Signal → Plan → Build → Review → Gate → Deploy → Wisdom
- **50+ agents**: narrow specialists (requirements-author, code-critic, test-author, *-cleanup, etc.)
- **7 skills**: test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools

Start here:

```
/flow-1-signal "your feature idea"
```

Then proceed in order (unless you are intentionally running out-of-order):

`/flow-2-plan` → `/flow-3-build` → `/flow-4-review` → `/flow-5-gate` → `/flow-6-deploy` → `/flow-7-wisdom`

---

## Operating Philosophy: Ops-First

**Core principle:** Engineering is default-allow. Publishing is gated.

The pack is a **build pipeline with guardrails**, not a guardrail pipeline that sometimes builds. The default posture is:

> explore → implement → test → push → harvest bot signal → iterate → verify → seal

### Work Plane (default-allow)

Everything up to staging runs without friction:
- Explore aggressively (read any files, search code, run checks)
- Write tests early, iterate on code freely
- Run tests locally, fix issues as you find them
- Push early to get bot feedback (CI, CodeRabbit, etc.)
- Security findings are **advisory** here, not throttles

### Publish Plane (gated)

Gates engage only when crossing the boundary:
- **Commit**: secrets-sanitizer scans staged changes
- **Push**: repo-operator checks for anomalies
- **GitHub post**: content mode restricts what gets posted (not what's analyzed)

If a gate blocks, **keep working locally**. Gates constrain publishing, not thinking.

### Intent Surfaces (Not Allowlists)

Flows express **intent** (what outputs to expect). Agents derive **paths** (what to stage).

This is a map, not a permission boundary:
- "Flow 3 outputs to `.runs/<run-id>/build/` + project code" (intent)
- repo-operator figures out what files to stage (execution)

**Extras are normal:** Ad-hoc fixes (typos, config tweaks) get staged and recorded, not blocked.

**Anomalies are rare:** Only tracked/staged changes outside the intent surface trigger push blocks—and even then, the commit proceeds locally.

### Fix-Forward Default

The sanitizer should behave like a good pre-commit hook:
- Fast scan of staged diff + flow artifacts
- Auto-redact obvious secret shapes
- Only block when remediation requires human judgment

---

## Agent Intelligence Philosophy

### Agents Are Smart, Config Is Dumb

This pack treats agents as intelligent actors, not script executors.

**Config contains only mechanics:**
- What command to run (`npm test`)
- Where files live (`src/`, `tests/`)
- Environment details (`github`, `windows-wsl2`)

**Policies stay in agent prompts:**
- Coverage thresholds
- Quality gates
- Review requirements
- Merge criteria
- What constitutes "good enough"

**Why?** Policies require judgment. "Is 80% coverage acceptable?" depends on context—is this a critical auth module or a CLI helper? Agents can apply judgment; config files cannot.

### Early Detection Over Late Gates

Problems should be caught where the fix is cheapest:
- **Per-AC**: Catch reward hacking during the microloop (before next AC starts)
- **Per-checkpoint**: Catch CI failures during feedback harvest (before flow ends)
- **Per-flow**: Catch format/lint issues in standards-enforcer (before Gate)
- **Gate**: VERIFY earlier findings (discovery belongs in upstream flows)

Gate is a **verification checkpoint**, not a quality filter. If Gate is catching issues that should have been caught earlier, that's a signal the upstream flows need improvement.

### Fix-Forward Within Flows

Small issues should be fixed where they're found:
- Formatting drift: `standards-enforcer` fixes it, doesn't BOUNCE
- Missing imports: `code-implementer` adds them on the next pass
- Stale comments: `fixer` removes them during review worklist

**BOUNCE only when:**
- The fix requires design changes (BOUNCE to Plan)
- The fix spans multiple ACs beyond current scope (BOUNCE to Build start)
- The fix requires human judgment (BOUNCE with `reason: NEEDS_HUMAN_REVIEW`)

### Intelligent Summarization

When summarizing for reports or routing:
- Explain what the issue IS, not just where it is
- Provide your assessment of validity (is this a real issue or bot noise?)
- Route to the agent best suited to fix it
- Synthesize understanding over file path lists

**Agents are smart.** They can read context, understand intent, and make judgment calls. Trust them to summarize intelligently rather than mechanically dumping file pointers.

### Intelligent Conflict Resolution

When conflicts arise (git, semantic, or otherwise):
- **Try to resolve first** - Read both sides, understand intent, merge if possible
- **Only escalate when ambiguous** - When you genuinely cannot determine the right resolution
- **Provide context when escalating** - Explain what you tried and why you couldn't resolve it

Agents should behave like senior engineers who can solve most problems themselves and only escalate the genuinely difficult ones.

---

## Natural Resiliency Philosophy

### Success Pressure → Guessing (PARTIAL is a Win)

Agents under pressure to complete a task will **guess** to finish. The fix is giving them **multiple successful exits**.

**`PARTIAL` is a successful completion** when:
- State is written to disk (`.runs/<run-id>/…`)
- Next steps are documented
- Work is checkpointed so the flow can be rerun cleanly

A `PARTIAL` exit is not failure. It's a save point.

### Honest State Is the Primary Success Metric

Agents are rewarded for **accurate reporting**, not completion theater.

**This is a VERIFIED success:**
```
status: UNVERIFIED
work_status: PARTIAL
what_completed: "Implemented 2/5 ACs"
blockers: ["Missing schema migration for AC-3"]
evidence: "Tests pass for AC-1, AC-2. AC-3 requires DB changes."
```

**This is a HIGH-RISK failure (even though it says "complete"):**
```
status: VERIFIED
work_status: COMPLETED
what_completed: "All 5 ACs implemented"
assumptions: ["Assumed schema exists (didn't verify)"]
```

The first report tells the orchestrator exactly what happened and what to do next. The second report hides uncertainty behind a false completion signal, causing downstream failures.

**Agent rule:** When uncertain, report the uncertainty. A 40% completion with honest blockers is more valuable than a 100% completion with hidden assumptions.

### Write Early, Write Often

Flows are **naturally re-runnable**. Re-running a flow is not "failure recovery"—it's routine:
- Double-check work
- Tighten schema alignment
- Clean up artifacts
- Improve quality incrementally

**Always room for improvement**, even if rerunning something that was already run.

### Forensic Truth: Diff + Test Results

We trust **git diffs and test results** as forensic evidence.
- The diff is the best audit surface for what changed
- Tests are the runtime truth for what works
- Critics do forensic analysis of both

No rigid "coverage ratio" gates—use judgment to assess honesty and risk.

### Intelligence Everywhere

Any agent is authorized to fix an obvious, safe error it sees (typo, lint nit, missing import). We don't silo "fixing" to a specific agent.

If a researcher sees a typo in the README, they should fix it and move on.

### Model Strategy

We intentionally avoid hardcoding model tiers into the pack.

- **Most agents:** `model: inherit` (lets users choose their default)
- **Some operator/librarian agents:** may default to `haiku` for fast search
- **Only force a heavier model** when the task truly needs it (rare)

**Naming rule:** Use model *names* only (Haiku, Sonnet, Opus). No version numbers—they become stale.

---

## Operating Model: Swarm Repo

Recommended: run flows in a dedicated `*-swarm` downstream repo.

```
my-project/           # Human workspace (stays calm)
my-project-swarm/     # Swarm workspace (commits freely)
```

Benefits:
- **Inspectability**: `.runs/` artifacts are committed and reviewable
- **Isolation**: swarm activity doesn't disrupt human development
- **Clean PRs**: open PR from swarm to origin when ready

### `.runs/` is Git Content

`.runs/` is committed by default — **do not gitignore it**.

Size discipline:
- Summaries over raw dumps
- No pasting full issue bodies into artifacts
- Keep artifacts "reviewable diff" sized

### Repo Topology (Invariant)

- **Swarm repo (`*-swarm`) is autonomous**. Flows run here end-to-end.
- **Flow 6 (Deploy) merges a run PR into `*-swarm/main`** (the swarm's mainline).
- This pack does **not** merge into the upstream human repo by default.
  (Upstream export is a customization / separate concern.)

---

## Non-Negotiables

These rules exist to prevent drift and "model invention":

1. **Repo root only**
   All commands run from **repo root**; all paths are **repo-root-relative**. Do not rely on `cd`.

2. **No raw git in flow commands or agent prompts**
   Git operations are owned by `repo-operator`. Orchestrators call `repo-operator` using **task phrasing**.
   If you feel compelled to paste git commands into a flow, that's a pack bug.

3. **Control plane vs audit plane**
   Orchestrators route on returned result blocks (`Gate Result`, `Repo Operator Result`).
   Files like `secrets_status.json` and `git_status.md` are durable audit records, not routing inputs.

4. **Two gates for GitHub operations**
   GitHub operations (`gh-issue-manager`, `gh-reporter`) require BOTH:
   - `safe_to_publish: true` (secrets gate)
   - `proceed_to_github_ops: true` (repo hygiene gate)

5. **`run_id` folders never rename**
   Identity changes happen via `canonical_key` + `aliases[]`, never via renaming directories.

You'll see these repeated in the relevant sections on purpose.

---

## Flow Authoring Rule

**Flows are routing tables. Agents are workers.**

This separation is about **token economics**: Orchestrator context is expensive, Agent execution is cheap.

### Flows contain:
- Station order (which agents to call, in what sequence)
- Routing logic (which Result block to read, what to do on PROCEED/RERUN/BOUNCE)
- Artifact expectations (what outputs to expect from each station)
- Termination conditions (when the flow is complete)

### Flows must NOT contain:
- Shell snippets (beyond illustrative examples)
- File path lists to stage/check (move to agents)
- Parsing logic or computation
- If/else chains for file existence checks

### Agents contain:
- All procedural work (read files, run commands, write outputs)
- Intent-to-paths mapping (the agent figures out what to stage)
- Validation logic (the agent checks if things are correct)
- Machine Summary + Result blocks for orchestrator routing

**Why this matters:** When logic lives in flows, the orchestrator must tokenize and reason about it every step. When logic lives in agents, a fresh sub-agent context handles the work cheaply. Put decisions in flows, put work in agents.

---

## Architecture Laws

These are invariants that prevent execution drift. Violating them creates subtle failures.

### Law 1: PM/IC Boundary

**Orchestrators route. Agents work. State-owners track.**

| Role | Responsibility | Example |
|------|----------------|---------|
| **Orchestrator** (flow session) | Call agents, route on Result blocks, manage TodoWrite | "If gate_verdict is MERGE, proceed to merge ops" |
| **Worker** (agent) | Do the work, report honestly | `code-implementer`, `test-author`, `fixer` |
| **State-Owner** (agent) | Own a state file, update it by reading worker responses | `review-worklist-writer`, `build-cleanup` |

**Violation:** Orchestrator editing `ac_status.json` or `review_worklist.json` directly.
**Correct:** Orchestrator calls `build-cleanup` to update AC status based on worker results.

### Law 2: Every Call Is an Implicit Resume

**Agents don't need "resume mode" flags.** They check disk state and determine what's left.

When an agent starts:
1. Check if its tracking artifact exists (e.g., `ac_status.json`, `review_worklist.json`)
2. If yes: read it, determine what's PENDING, continue from there
3. If no: initialize fresh

**Violation:** Flow says "call build-cleanup in resume mode."
**Correct:** Flow says "call build-cleanup." The agent checks disk state and behaves appropriately.

**Corollary:** If an agent needs genuinely different behavior (not just "resume vs fresh"), that's a signal for two separate agents, not a `mode:` parameter.

### Law 3: Agents Own State Files

Each state file has exactly one owner agent. Only that agent writes to it.

| State File | Owner Agent | Other Agents |
|------------|-------------|--------------|
| `ac_status.json` | `build-cleanup` | Workers report completion; cleanup updates the file |
| `review_worklist.json` | `review-worklist-writer` | Workers report what they did; writer updates status |
| `run_meta.json` | `run-prep`, `*-cleanup`, `gh-issue-manager` | Others may read but not write |
| `index.json` | `*-cleanup`, `run-prep`, `gh-issue-manager` | Others may read but not write |

**Violation:** `code-implementer` updating `ac_status.json` after finishing an AC.
**Correct:** `code-implementer` reports completion in its Result block. Orchestrator calls `build-cleanup` to update `ac_status.json`.

### Law 4: AC Termination = Green + Orchestrator Agreement

**"Green tests" is necessary but not sufficient for AC completion.**

An AC is done when:
1. `test-executor` returns Green for that AC's scope
2. The orchestrator agrees there's nothing left worth fixing based on critic feedback

**The loop:** implement → test → critique → (if critic has actionable items) → improve → test again

Even with green tests, if `code-critic` identifies a maintainability risk or clear technical debt, the orchestrator should authorize one improvement pass. The critic's `can_further_iteration_help: no` signal (or orchestrator judgment) terminates the loop.

### Law 5: True Blockers Surface Immediately

**Non-derivable blockers don't wait for end-of-flow.**

If an agent (especially `clarifier`) hits a genuine NON_DERIVABLE blocker:
- It cannot make a recommendation
- No safe default exists
- Human decision is required

The orchestrator should immediately call `gh-issue-manager` to post a comment with:
- The blocker description
- Evidence searched
- The decision needed

Don't batch these into end-of-flow reporting. The line stops until humans respond.

**Most questions are not blockers.** DEFAULTED (safe reversible default chosen) is the common case. NON_DERIVABLE is rare and requires proof-of-research.

### Law 6: Infrastructure Before Logic

**State-transition work must complete before tasks that assume the new state.**

This is enforced in `work-planner`:
- Infrastructure/migration subtasks (ST-000, etc.) depend on nothing
- Logic subtasks that assume new state must list infrastructure in `depends_on`
- You cannot schedule logic before the state it depends on exists

**Violation:** `ST-002: Implement login flow` with no dependency, when login requires a new `sessions` table from `ST-001`.
**Correct:** `ST-002` has `depends_on: ["ST-001"]`.

---

## Run Identity + State

### Working Directory + Paths Invariant

- All commands run from **repo root**
- All paths are **repo-root-relative**
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/<flow>/`
- `<flow>` is one of: `signal`, `plan`, `build`, `review`, `gate`, `deploy`, `wisdom`

Example: `.runs/feat-auth/build/impl_changes_summary.md`

Code and tests remain in project-defined locations (customize per repo layout).

### `.runs/index.json`

Global run index:

```json
{
  "version": 1,
  "runs": [
    {
      "run_id": "feat-auth",
      "canonical_key": "gh-456",
      "task_key": "feat-auth",
      "task_title": "Add OAuth2 login",
      "issue_number": 456,
      "pr_number": null,
      "updated_at": "2025-12-11T22:15:00Z",
      "status": "VERIFIED",
      "last_flow": "build"
    }
  ]
}
```

Rules:

- One entry per `run_id` (upsert, not append).
- Preserve existing ordering; upsert updates in-place. New runs append.
- Keep fields minimal — counts live in receipts.
- Only these agents may update `.runs/index.json`:
  - `run-prep`, `signal-run-prep`
  - `<flow>-cleanup`
  - `gh-issue-manager`

### `run_meta.json`

Per-run metadata at `.runs/<run-id>/run_meta.json`:

```json
{
  "run_id": "<run-id>",
  "run_id_kind": "GH_ISSUE | LOCAL_ONLY | null",
  "issue_binding": "IMMEDIATE | DEFERRED | null",
  "issue_binding_deferred_reason": "gh_unauth | gh_unavailable | null",
  "canonical_key": "<gh-456 | pr-789 | null>",
  "aliases": ["<run-id>", "<gh-456>", "<branch-name>"],
  "task_key": "<ticket-id | branch-slug | null>",
  "task_title": "<short normalized title>",
  "github_repo": "<owner/repo | null>",
  "github_repo_expected": "<owner/repo | null>",
  "github_repo_actual_at_creation": "<owner/repo | null>",
  "github_ops_allowed": true,
  "repo_mismatch": false,
  "created_at": "<ISO8601>",
  "updated_at": "<ISO8601>",
  "iterations": 1,
  "flows_started": ["signal", "plan"],
  "source": "<branch:name | ticket:id | manual>",
  "issue_number": 456,
  "issue_url": "<url | null>",
  "issue_title": "<string | null>",
  "pr_number": null,
  "supersedes": "<previous-run-id | null>",
  "related_runs": [],
  "base_ref": "<branch-name | null>"
}
```

Identity rules:

- `run_id` is immutable. **No renames.**
- When a GitHub issue/PR exists, set `canonical_key` and add aliases. Do not rename folders.

Stacked run support:

- `base_ref` (optional): The branch this run is based on. Used for diff computation in agents that audit changes (standards-enforcer, coverage-enforcer, etc.). If present, diffs are computed relative to `base_ref`; otherwise agents default to `origin/main`.

---

## Flow Execution Model

### Two State Machines

Every flow uses two complementary state machines:

1. **TodoWrite** = session navigation (ephemeral)
2. **`flow_plan.md`** = durable on-disk state for reruns/handoffs

**Timing rule:** Create TodoWrite immediately. Write/update `flow_plan.md` only **after** `run-prep` / `signal-run-prep` has created `.runs/<run-id>/<flow>/`.

### The Seven Flows

| Flow | Slash Command | Inputs | Key Outputs |
|------|---------------|--------|-------------|
| 1. Signal | `/flow-1-signal` | raw feature request | `requirements.md`, `features/*.feature`, `verification_notes.md`, risks, `signal_receipt.json` |
| 2. Plan | `/flow-2-plan` | Signal outputs (if present) | `adr.md`, `api_contracts.yaml`, `observability_spec.md`, `test_plan.md`, `ac_matrix.md`, `work_plan.md`, `plan_receipt.json` |
| 3. Build | `/flow-3-build` | Plan outputs (if present) | code/tests, critiques, `build/ac_status.json` (created/updated), `build_receipt.json`, Draft PR |
| 4. Review | `/flow-4-review` | Build outputs + Draft PR | `pr_feedback.md`, `review_worklist.md`, `review_actions.md`, `review_receipt.json` |
| 5. Gate | `/flow-5-gate` | Review outputs (if present) | `merge_decision.md` (verdict: MERGE or BOUNCE), `gate_receipt.json` |
| 6. Deploy (Mainline) | `/flow-6-deploy` | Gate outputs (if present) | `deployment_log.md`, `verification_report.md`, `deployment_decision.md`, `deploy_receipt.json` |
| 7. Wisdom | `/flow-7-wisdom` | all prior outputs | `learnings.md`, `feedback_actions.md`, `wisdom_receipt.json` |

**Note on Flow 6:** "Deploy" merges the run PR into `*-swarm/main` (the swarm repo's mainline), not an upstream human repo. This pack treats "mainline promotion" as the Deploy target. Upstream export is a separate concern.

Out-of-order is allowed: proceed best-effort, document assumptions, and expect UNVERIFIED outcomes when upstream artifacts are missing.

---

## Receipts

Receipt naming:

| Flow | Receipt File |
|------|--------------|
| Signal | `.runs/<run-id>/signal/signal_receipt.json` |
| Plan | `.runs/<run-id>/plan/plan_receipt.json` |
| Build | `.runs/<run-id>/build/build_receipt.json` |
| Review | `.runs/<run-id>/review/review_receipt.json` |
| Gate | `.runs/<run-id>/gate/gate_receipt.json` |
| Deploy | `.runs/<run-id>/deploy/deploy_receipt.json` |
| Wisdom | `.runs/<run-id>/wisdom/wisdom_receipt.json` |

Receipt guarantees:

- `counts` are mechanical (grep/wc/parse), never estimated.
- `quality_gates` are sourced from agent Machine Summaries (no recomputation).
- Reporters summarize from receipts, not from raw artifacts.
- `stations` tracks per-station execution:
  - `executed: true|false` — whether the station ran (vs. SKIPPED stub)
  - `result: PASS|FAIL|SKIPPED|UNKNOWN` — what the station produced
- `evidence_sha` is the commit SHA when receipt was generated (for staleness detection)
- `generated_at` is the ISO8601 timestamp for receipt creation

### State-First Verification (Receipts as Logs, Not Gatekeepers)

**Core principle:** The repo's current state (HEAD + working tree + staged diff + actual tool results) is the thing you're building and shipping. Receipts help you investigate what happened, why, and where to look next—but they are not the primary mechanism for verifying and determining outcomes once the repo has moved.

**Trust hierarchy:**

1. **Live repo state + executed evidence** (primary)
   - `git rev-parse HEAD`, `git diff`, `git status`
   - Test/lint/mutation outputs generated *now*
   - CI check runs for the current SHA

2. **Receipts** (cached evidence of prior state)
   - What an agent ran earlier
   - What it saw then
   - Links/paths to logs and artifacts

3. **Narrative summaries** (useful for humans, never a control input)

**Agent invariant:** Validate against current repo state and executed evidence. Use receipts as historical breadcrumbs and summary inputs. Never use receipt presence or receipt fields as permission to proceed.

**Receipts are logs, not locks:** A receipt is a "flight recorder" entry of what happened at a specific station. It is NOT a cryptographic permission slip that must be re-sealed when code changes.

- If `receipt.evidence_sha != git HEAD`, that's normal—ad-hoc fixes, fix-forward, and mid-flow improvements are expected.
- The receipt is still valid as historical evidence of what happened at that station.
- Don't BOUNCE or require regeneration just because the SHA drifted.
- The git log is the audit trail. The receipt is a summary.

**Evidence field convention:** Receipts include these fields for context:
- `evidence_sha`: The commit SHA when this evidence was generated
- `generated_at`: ISO8601 timestamp

These are informational, not gating. If SHA differs from HEAD, the receipt tells you what the world looked like then—not that the work is invalid now.

**Why this matters:** When a developer fixes a typo mid-flow, agents see it (live state). Receipts don't become "paperwork that must be re-sealed." The system adapts forward instead of trying to re-litigate the past.

### Evidence over Format

If verification evidence exists but a receipt is malformed or missing fields:

- Treat it as a **pack/tooling defect**, not an engineering failure.
- Keep the line moving: ship based on the underlying evidence **if gates pass**.
- Record the defect explicitly (e.g., `status: UNVERIFIED`, `blockers: ["receipt_tooling_error: <details>"]`) and open a maintenance issue.

If the evidence itself is missing (tests didn't run, CI unknown, etc.), that's not paperwork drift — that's **unverified work**. Route back to run the missing verification.

**Maintainer mantra:**
> Build the asset. Capture the evidence. State the truth. Ship when the evidence is green.

### DevLT Tracking (Developer Lead Time)

Receipts may include a `devlt` section for retrospective analysis of human vs machine effort:

```json
{
  "devlt": {
    "flow_started_at": "2025-12-22T10:00:00Z",
    "flow_completed_at": "2025-12-22T10:45:00Z",
    "human_checkpoints": [
      {"at": "2025-12-22T10:00:00Z", "action": "flow_start"},
      {"at": "2025-12-22T10:30:00Z", "action": "question_answered"},
      {"at": "2025-12-22T10:45:00Z", "action": "flow_approved"}
    ],
    "machine_duration_sec": 2700,
    "human_checkpoint_count": 3,
    "estimated_human_attention_min": 15,
    "estimation_basis": "checkpoint_count * 5min average"
  }
}
```

**Field semantics:**

- `flow_started_at` / `flow_completed_at`: Observable timestamps (wall clock)
- `human_checkpoints`: Array of human interaction points with timestamps and action types
- `machine_duration_sec`: Derived from timestamps (not execution time, just wall time)
- `human_checkpoint_count`: Count of human interactions (observable)
- `estimated_human_attention_min`: **Inference** - rough estimate based on checkpoint count and typical review times
- `estimation_basis`: Explains how the estimate was derived (transparency)

**Observable vs inferred:**
- Timestamps and counts are **facts** (derived from logs/artifacts)
- `estimated_human_attention_min` is an **inference** (labeled as such)
- Token costs are **not tracked** here (unreliably available)

**Purpose:** DevLT is for retrospective analysis in Flow 7 (Wisdom), not for gating or routing. It helps answer: "How much human attention did this run actually require?"

---

## Machine Summary Contract

Critic and verification agents include a machine-parseable summary block:

```yaml
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED

recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_flow: <1|2|3|4|5|6|7 | null>
route_to_station: <string | null>      # e.g. "test-executor", "lint-executor" — hint, not strict enum
route_to_agent: <agent-name | null>    # strict enum — only when certain the name is valid

blockers: []
missing_required: []
concerns: []

observations: []                       # optional; things noticed worth capturing (friction, cross-cutting, improvements)

can_further_iteration_help: yes | no   # critics only

severity_summary:                      # critics/verifiers
  critical: 0
  major: 0
  minor: 0
```

Semantics:

- `CANNOT_PROCEED` = mechanical failure only (I/O, permissions, tooling unusable). `missing_required` must be non-empty.
- `UNVERIFIED` = gaps/uncertainty/issues documented. `blockers` should explain what prevents VERIFIED.
- `VERIFIED` = adequate for purpose. `blockers` empty.
- `observations` = optional; things the agent noticed that aren't blockers but worth capturing (friction encountered, cross-cutting insights, pack/flow improvements). Feeds into Wisdom flow via `learning-synthesizer`.

Routing:

- Orchestrators route on `recommended_action` + `route_to_*`.
- **Control plane vs audit plane:**
  - The Machine Summary in the agent's **response** is the routing surface (control plane)
  - The artifact **file** (e.g., `design_validation.md`, `test_critique.md`) is the durable audit record
  - Orchestrators route on returned Result blocks, not by re-reading files
  - Downstream agents read artifact files for detailed context
  - Both exist and serve different purposes: fast routing vs durable forensics

Recommended action semantics (closed enum):
- `PROCEED` = default, even when human judgment is required; capture blockers/assumptions. Do **not** use PROCEED as a fallback for "can't name the target."
- `RERUN` = rerun the same station when a deterministic improvement is expected.
- `BOUNCE` = reroute to an upstream flow. Requires `route_to_flow`; optionally set `route_to_station` (hint) and/or `route_to_agent` (strict enum).
- `FIX_ENV` = only with `status: CANNOT_PROCEED` (mechanical/env failure).

Routing field rules:
- **`route_to_agent`** = strict enum. Only set when certain the agent name is valid. Never guess.
- **`route_to_station`** = free-text hint (e.g., "test-executor", "build-cleanup"). Use when you know the station but aren't certain of the exact agent enum.
- **`route_to_flow`** = required for BOUNCE. If you know which phase needs work but not the specific agent/station, set only this.

BOUNCE fallback ladder:
1. Know flow + station + agent → set all three
2. Know flow + station only → set `route_to_flow` + `route_to_station`, leave `route_to_agent: null`
3. Know flow only → set `route_to_flow`, explain in blockers what to rerun
4. Don't know flow → use PROCEED with blockers (rare; document what evidence is missing)

---

## Control-Plane Blocks (Canonical)

Flows and agents should use these blocks **verbatim** (copy/paste) to avoid schema drift.

### Gate Result (emitted by `secrets-sanitizer`)

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

Notes:
- The sanitizer is a **boolean gate**, not a router. It says yes/no.
- If `safe_to_publish: false`, the flow doesn't push. The orchestrator decides next steps.
- `blocker_kind` is the machine-readable category: `NONE` (not blocked), `MECHANICAL` (IO/tooling failure), `SECRET_IN_CODE` (staged code needs fix), `SECRET_IN_ARTIFACT` (artifact can't be redacted).
- `blocker_reason` is the human-readable explanation (if BLOCKED); otherwise null.

### PR Feedback Harvester Result (emitted by `pr-feedback-harvester`)

<!-- PACK-CONTRACT: PR_FEEDBACK_RESULT_V2 START -->
```yaml
## PR Feedback Harvester Result
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
evidence_sha: <sha>                  # commit being evaluated
pr_number: <int | null>

ci_status: PASSING | FAILING | PENDING | NONE
ci_failing_checks: [<check-name>]    # names of failing checks (also appear as blockers)

blockers_count: <int>                # CRITICAL items only (stop-the-line)
blockers:                            # top N blockers (cap at 10)
  - id: FB-CI-<check_run_id> | FB-RC-<review_comment_id> | FB-IC-<issue_comment_id> | FB-RV-<review_id>
    source: CI | CODERABBIT | REVIEW | LINTER | DEPENDABOT | OTHER
    severity: CRITICAL               # blockers are CRITICAL-only
    category: BUILD | TESTS | SECURITY | CORRECTNESS | DOCS | STYLE
    title: <short title>
    route_to_agent: code-implementer | test-author | fixer | doc-writer
    evidence: <check name | file:line | comment id>
    thoughts: <triage-level quick read>

counts:
  total: <n>
  critical: <n>
  major: <n>
  minor: <n>
  info: <n>

sources_harvested: [reviews, review_comments, check_runs, ...]
sources_unavailable: []
```
<!-- PACK-CONTRACT: PR_FEEDBACK_RESULT_V2 END -->

Notes:
- **One routing surface**: CI failures, CodeRabbit, human reviews all become blockers with `source` tag — no separate CI path
- **CRITICAL-only blockers**: Flow 3 interrupts only on stop-the-line issues. MAJOR stays in counts + full `pr_feedback.md`
- **Stable IDs**: Derived from upstream IDs (check_run_id, review_comment_id, etc.) — reruns don't reshuffle
- **Triage, not planning**: `thoughts` is one-line quick read ("valid issue", "outdated suggestion", "bot probably wrong")
- Flow 3 routes on `blockers[]` — routed agent does deep investigation
- Flow 4 drains the complete worklist from `pr_feedback.md` (all severities)
- Per-flow outputs: `build/pr_feedback.md` (Flow 3), `review/pr_feedback.md` (Flow 4)

### Repo Operator Result (emitted by `repo-operator`)

<!-- PACK-CONTRACT: REPO_OPERATOR_RESULT_V2 START -->
```yaml
## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_WARNING | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_classification:
  unexpected_staged_paths: []
  unexpected_unstaged_paths: []
  unexpected_untracked_paths: []
anomaly_paths: []
```
<!-- PACK-CONTRACT: REPO_OPERATOR_RESULT_V2 END -->

Notes:

- `commit_sha` is always populated (current HEAD on no-op).
- `publish_surface` is always present:
  - `PUSHED` only when a push is attempted and succeeds.
  - `NOT_PUSHED` for local-only checkpoints, tracked anomalies, skipped pushes, and push failures.
- `status` values:
  - `COMPLETED` - operation succeeded, no anomalies
  - `COMPLETED_WITH_WARNING` - only untracked anomalies; push allowed
  - `COMPLETED_WITH_ANOMALY` - tracked/staged anomalies; push blocked
- `anomaly_classification` provides breakdown by risk level:
  - `unexpected_staged_paths` - HIGH risk (blocks push)
  - `unexpected_unstaged_paths` - HIGH risk (blocks push)
  - `unexpected_untracked_paths` - LOW risk (warning only, allows push)
- `anomaly_paths` - DEPRECATED; union of classification arrays for backward compatibility
- Orchestrators route on these returned blocks, not by rereading files.

---

## Canonical Status + Verdict Domains

Do not conflate these domains:

1. **Flow/Agent Status** (Machine Summary + receipts)
   `VERIFIED | UNVERIFIED | PARTIAL | CANNOT_PROCEED`

   **VERIFIED requires executed evidence.** A station being "skipped" means the work is unverified, not verified by default. Missing verification artifacts (test execution, critics) result in `UNVERIFIED`, not "concern only."

   Status semantics:
   - `VERIFIED`: Required artifacts exist AND verification stations ran AND passed (executed evidence present)
   - `UNVERIFIED`: Verification incomplete, contradictions, critical failures, or missing core outputs
   - `PARTIAL`: Real progress made, but key verification evidence missing/skipped (valid for unbounded loops like Flow 4 Review)
   - `CANNOT_PROCEED`: Mechanical failure only (IO/permissions/tooling)

   **SKIPPED stubs:** Cleanup agents create explicit SKIPPED stubs for missing station artifacts:
   ```markdown
   # <Artifact Name>
   status: SKIPPED
   reason: <why it wasn't produced>
   evidence_sha: <current HEAD>
   generated_at: <iso8601>
   ```
   This ensures nothing is silently missing. Downstream and Flow 7 (Wisdom) can see what happened.

   **Per-station tracking:** Receipts include a `stations` section with `executed: true|false` and `result: PASS|FAIL|SKIPPED|UNKNOWN` for each station. This is the machine-grade evidence of what ran.

2. **Repo Operator Status** (Repo Operator Result)
   `COMPLETED | COMPLETED_WITH_WARNING | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED`

3. **Secrets Sanitizer Status** (Gate Result)
   `CLEAN | FIXED | BLOCKED`

4. **Gate Merge Verdict** (`merge_decision.md`)
   `MERGE | BOUNCE` (use BOUNCE reason to signal human review)

5. **Deploy Verdict** (`deployment_decision.md`) - Two-Axis Model
   - `deploy_action`: `COMPLETED | SKIPPED | FAILED`
   - `governance_enforcement`: `VERIFIED | VERIFIED_RULESET | UNVERIFIED_PERMS | NOT_CONFIGURED | UNKNOWN`
   - `deployment_verdict` (derived): `STABLE | NOT_DEPLOYED | GOVERNANCE_UNVERIFIABLE | BLOCKED_BY_GATE`

   Note: `GOVERNANCE_UNVERIFIABLE` means deploy action succeeded but governance cannot be verified. This is distinct from `NOT_DEPLOYED` (deploy action failed).

6. **Smoke Signal** (runtime signal inside `verification_report.md`)
   `smoke_signal: STABLE | INVESTIGATE | ROLLBACK`

This separation is intentional: "deploy verdict" is conservative and governance-shaped; "smoke signal" is operational signal.

---

## Publish Surface (Per-Flow)

Publish surface = what secrets-sanitizer scans and what repo-operator checkpoints for that flow.

| Flow | Publish Surface |
|------|-----------------|
| 1 | `.runs/<run-id>/signal/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 2 | `.runs/<run-id>/plan/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 3 | `.runs/<run-id>/build/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json`, plus staged code/test changes |
| 4 | `.runs/<run-id>/review/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json`, plus staged code/test changes |
| 5 | `.runs/<run-id>/gate/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 6 | `.runs/<run-id>/deploy/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |
| 7 | `.runs/<run-id>/wisdom/`, `.runs/<run-id>/run_meta.json`, `.runs/index.json` |

Key invariant: secrets-sanitizer scans only the current flow's publish surface, not the entire `.runs/<run-id>/`.

---

## GitHub Access + Content Mode (Canonical)

GitHub is an **observability pane**, not the work substrate. GitHub operations (`gh-issue-manager`, `gh-reporter`) are governed by:

### 1) Access gate (hard)

- If `run_meta.github_ops_allowed == false`: do **not** call GitHub (even read-only); flows proceed locally.
- If `gh` is unauthenticated/unavailable: SKIP GitHub calls; record the limitation locally; proceed.
- Prefer `run_meta.github_repo` (or `github_repo_actual_at_creation`) for repo scope; do **not** invent a repo when missing.

### 2) Content Mode Ladder (4 levels)

Content mode is derived from **secrets safety** and **push surface**, NOT from workspace hygiene (`proceed_to_github_ops`).

| Mode | Conditions | Allowed Content | Link Style |
|------|------------|-----------------|------------|
| **FULL** | `safe_to_publish: true` AND `publish_surface: PUSHED` | Narrative, links, quotes, open questions | Blob links |
| **FULL_PATHS_ONLY** | `safe_to_publish: true` AND `publish_surface: NOT_PUSHED` AND no tracked anomalies | Narrative, receipts, open questions | Paths only |
| **SUMMARY_ONLY** | `safe_to_publish: true` AND tracked anomalies exist | Counts + concise narrative | Paths only |
| **MACHINE_ONLY** | `safe_to_publish: false` | Counts and paths only | Paths only |

**Key invariants:**
- Secrets gate (`safe_to_publish`) drives MACHINE_ONLY. This is the security gate.
- Push surface (`publish_surface`) drives link style. PUSHED = blob links allowed; NOT_PUSHED = paths only.
- Workspace hygiene (`proceed_to_github_ops`) gates pushing, NOT content mode. Untracked anomalies do not degrade content.
- Only tracked/staged anomalies force SUMMARY_ONLY (uncertain provenance) but NOT MACHINE_ONLY.

**SUMMARY_ONLY semantics (output restriction only):**
- SUMMARY_ONLY restricts **what gets posted to GitHub**, not what the LLM can read or analyze.
- The agent can read **any file** needed to do its job (receipts, requirements, features, ADR, code, etc.).
- The agent must only **post**:
  - Receipts and machine-derived fields (`status`, `counts.*`, `quality_gates.*`)
  - Safe summaries that don't quote verbatim from outside the committed surface
  - Next steps and blockers
- The restriction exists because tracked anomalies mean uncertain provenance for the publish surface — we gate what we expose, not what we think about.

### 3) Anomaly classification

Repo-operator classifies anomalies by type:
- `unexpected_staged_paths` - HIGH risk: staged changes outside allowlist (blocks push, SUMMARY_ONLY)
- `unexpected_unstaged_paths` - HIGH risk: tracked file modifications outside allowlist (blocks push, SUMMARY_ONLY)
- `unexpected_untracked_paths` - LOW risk: new files not yet tracked (warning only, allows push, FULL_PATHS_ONLY)

Only HIGH risk anomalies block `proceed_to_github_ops`. Untracked-only anomalies allow FULL/FULL_PATHS_ONLY.

---

## Git Operations Policy (Repo-Operator Owned)

**Rule (repeat):** do not embed raw git commands in flow commands or agent prompts. All git is executed via `repo-operator` using **task phrasing**.

**Two-step atomic (Flow 3):** Artifacts first, code second. This ensures the audit trail persists even if code changes are reverted.

### Commit Cadence

- **Every flow checkpoints** (main checkpoint): audit commit of the flow's publish surface on the run branch.
- **Flow 3 uses two-step atomic commits**:
  1. Artifacts commit: `.runs/<run-id>/build/` + flow metadata (audit trail)
  2. Code commit: staged code/test changes (work product)

  This separation allows reverting code changes without losing the audit trail of what was attempted and why.
- **Flow 6 additionally merges the PR into swarm mainline**: promotion, plus tags/releases if configured.

GitHub status files (`gh_issue_status.md`, `gh_report_status.md`, `gh_comment_id.txt`) are **gitignored** — they are operational exhaust, not audit trail.

### Required Tasks (Conceptual)

Exact phrasing is standardized in flow docs:

- ensure run branch: `task: "ensure run branch run/<run-id>"`
- checkpoint allowlist: `task: "checkpoint allowlist for flow <flow>"`
- stage intended changes (Build): `task: "stage intended changes for build"`
- commit/push build changes (Build): `task: "commit and push build changes"`
- merge/tag/release (Deploy release ops): `task: "merge and tag release"`

Safe-bail:

- `checkpoint_mode: local_only` is a repo-operator mode that mechanically forces `proceed_to_github_ops: false` and never pushes.

Anomaly handling:

- If **tracked/staged** anomalies exist outside allowlist (or Build's cleanliness interlock fails), repo-operator:
  - commits only the allowlist when safe (`safe_to_commit: true`)
  - sets `status: COMPLETED_WITH_ANOMALY`, `proceed_to_github_ops: false`
  - writes `git_status.md` in the current flow directory
- If **untracked-only** anomalies exist:
  - sets `status: COMPLETED_WITH_WARNING`, `proceed_to_github_ops: true`
  - writes `git_status.md` as a hygiene warning (does not block push)

---

## Secrets Sanitizer (Publish Gate)

The sanitizer is a **fix-first pre-commit hook**, not a behavior throttle.

Execution order in every flow (linear, no reseal loop):

1. `<flow>-cleanup` writes receipt (captures engineering outcome)
2. `repo-operator` stages intended + ad-hoc changes (embrace extras, block test deletion)
3. `secrets-sanitizer` scans publish surface; fixes what it can; returns Gate Result
4. `repo-operator` checkpoint (gated on `safe_to_commit`; push gated on tracked anomalies)
5. `gh-issue-manager` + `gh-reporter` (when access allows; content mode per ladder above)

**No reseal loop:** The sanitizer runs once. If it redacts artifacts, the `secrets_scan.md` is the audit trail — the receipt does not need regeneration. This prevents "Compliance Recursion" where paperwork burns tokens instead of engineering.

---

## Flow 6 Distinction: Release Ops vs Reporting Ops

Flow 6 has two categories with different gating:

| Category | Operations | Gating |
|----------|------------|--------|
| Release Ops | merge PR, tag/release | Gate merge verdict = MERGE + repo-operator mechanics |
| Reporting Ops | gh-issue-manager, gh-reporter | Access gate; content mode per ladder (FULL/FULL_PATHS_ONLY/SUMMARY_ONLY/MACHINE_ONLY) |

This distinction prevents "can we post?" from affecting "can we merge?".

---

## Skills

| Skill | Purpose |
|-------|---------|
| `test-runner` | Run tests, capture output to run artifacts |
| `auto-linter` | Format + lint code |
| `policy-runner` | Run policy-as-code checks |
| `runs-derive` | Read-only .runs derivations (counts, Machine Summary extraction, receipt reading) |
| `runs-index` | Write .runs/index.json updates (status, last_flow, updated_at) |
| `openq-tools` | Open questions register (QID generation, append entries) |
| `secrets-tools` | Secrets scanning/redaction for publish gates (never prints secret content) |

---

## CLI Tooling Surface

Rust-based CLI tools replace ad-hoc bash pipelines for deterministic operations.

### Install location (repo-local)

```
.demoswarm/bin/pack-check      # mac/linux
.demoswarm/bin/pack-check.exe  # windows
.demoswarm/bin/demoswarm       # demoswarm CLI (runs-derive, runs-index, openq-tools, secrets-tools)
.demoswarm/bin/demoswarm.exe   # windows
```

### Install (repo-local)

```bash
# Install both tools
cargo install --path tools/demoswarm-pack-check --root .demoswarm
cargo install --path tools/demoswarm-runs-tools --root .demoswarm
```

### Resolver shims

Agents **always invoke via shims** — never assume PATH or direct binary access:

```bash
# Pack validation
bash .claude/scripts/pack-check.sh [OPTIONS]

# Demoswarm CLI operations
bash .claude/scripts/demoswarm.sh <command> [OPTIONS]
```

Shims handle resolution in order:
1. `.demoswarm/bin/<tool>` (repo-local install, preferred)
2. `<tool>` on PATH (global install)
3. `cargo run` fallback (dev only, if `tools/` exists)
4. Python fallback (legacy, `demoswarm.sh` only)

### pack-check

Validates pack structural + contract consistency.

```bash
# Human-readable output
bash .claude/scripts/pack-check.sh --no-color

# Machine-readable JSON
bash .claude/scripts/pack-check.sh --format json
```

### demoswarm

Deterministic helpers for `.runs/` operations. **Agents must use the shim:**

```bash
bash .claude/scripts/demoswarm.sh <command> [options]
```

| Command | Purpose |
|---------|---------|
| `count pattern --file X --regex Y` | Null-safe grep count |
| `count bdd --dir X` | BDD scenario count |
| `ms get --file X --section Y --key Z` | Extract Machine Summary field |
| `yaml get --file X --key Y` | Extract YAML block field |
| `yaml count-items --file X --item-regex Y` | Count items in YAML array |
| `index upsert-status --index X --run-id Y --status Z` | Update `.runs/index.json` |
| `receipt get --file X --key Y` | Read receipt JSON field |
| `receipts count --run-dir X` | Count receipt files |
| `openapi count-paths --file X` | Count OpenAPI paths |
| `line get --file X --prefix Y` | Extract value from line with prefix |
| `inv get --file X --marker Y` | Extract value from inventory marker line |
| `time now` | ISO8601 timestamp |
| `openq next-id --file X --prefix Y` | Generate next open question ID |
| `openq append --file X --prefix Y --question Z ...` | Append open question entry |
| `secrets scan --path X --output Y` | Scan for secrets (returns status) |
| `secrets redact --file X --type Y` | Redact secrets in-place |

See skill docs for complete reference:
- `.claude/skills/runs-derive/SKILL.md` (read-only derivations)
- `.claude/skills/runs-index/SKILL.md` (index writes)
- `.claude/skills/openq-tools/SKILL.md` (open questions)
- `.claude/skills/secrets-tools/SKILL.md` (secrets scanning)

---

## Customization

See `docs/how-to/customize-pack.md` for:

- prerequisites (bash/jq/grep, Windows/WSL2/Git Bash)
- test/lint command adaptation
- source layout changes
- Git provider adaptation
- policy/security scanner customization

---

## Troubleshooting

### "CANNOT_PROCEED"

CANNOT_PROCEED is mechanical failure only. Fix environment/tooling, then rerun.

### "Microloop won't terminate"

Route on the critic control plane:
- `status: CANNOT_PROCEED` → stop (FIX_ENV)
- `recommended_action: BOUNCE` → follow `route_to_flow` (required), then `route_to_station` (hint), then `route_to_agent` (if present)
- `recommended_action: RERUN` → rerun specified agent
- `recommended_action: PROCEED` → proceed even if UNVERIFIED (capture blockers/limitations)
- If `recommended_action` absent: use `can_further_iteration_help` as tie-breaker (`no` → proceed; `yes` → rerun)

### "No GitHub update happened"

Check the two gates:

- secrets-sanitizer Gate Result: `safe_to_publish`
- repo-operator Result: `proceed_to_github_ops`

If either is false, GH ops must be skipped.

### "Can't find run by issue number"

Alias resolution is via `.runs/index.json` (`issue_number`/`canonical_key`) and `run_meta.json.aliases[]`. Folder names do not change.

### "The swarm is deleting tests / weakening assertions"

This is a common failure mode (reward hacking), not "working as intended." We detect it early via:
- Diff-audits in critic microloops (Flow 3)
- PR feedback harvest (Flow 3/4)
- Standards-enforcer checks (Flow 3)

If detected, route to rework immediately—don't let it reach Gate as a surprise.

### "Context is full"

`PARTIAL` is a win. Checkpoint state + document next steps, then rerun the flow to continue.

Write early, write often. The system uses disk-based state to pick up exactly where it left off.

### "The branch is a mess / upstream moved"

Nuke the **branch**, not the fork. Sync the fork from upstream in the GitHub UI to preserve history, then start a fresh branch/run.

```bash
git branch -D run/<run-id>
rm -rf .runs/<run-id>/
# Sync fork via GitHub UI
# Start fresh run
```
