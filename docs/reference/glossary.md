# Glossary

> Key terms used throughout the demo-swarm pack.

## Core Concepts

### Flow
A sequence of agent invocations that transforms inputs into outputs. The swarm has 7 flows:
- **Flow 1 (Signal)**: Raw input -> problem statement, requirements, BDD, scope/risks.
- **Flow 2 (Plan)**: Specs -> options, ADR, contracts, observability, test/work plans.
- **Flow 3 (Build)**: Plans -> tests, code, reviews, build receipt.
- **Flow 4 (Review)**: Build output + Draft PR -> PR feedback, worklist, review receipt.
- **Flow 5 (Gate)**: Review output -> audits, policy/security/coverage checks, merge decision.
- **Flow 6 (Deploy)**: Gate-approved work -> release verification + deploy receipt.
- **Flow 7 (Wisdom)**: Outcomes -> regressions, learnings, feedback actions, final receipt.

### Step (Station)
A discrete unit of work within a flow. Steps invoke one or more agents and produce specific artifacts. In this pack, steps are "stations" tracked in TodoWrite and `flow_plan.md`.

### Run
A single execution of one or more flows for a specific work item. Runs are identified by a `run_id` (e.g., `feat-auth`) and store artifacts under `.runs/<run-id>/`.

### Run Directory
The directory containing all artifacts for a run: `.runs/<run-id>/`. Flow artifacts live under `.runs/<run-id>/<flow>/`. All paths are repo-root-relative.

### run_id
Immutable folder name for a run. Stable diffs depend on run_id stability; identity changes happen via `canonical_key` and `aliases[]`, not by renaming folders.

### canonical_key
A human-facing canonical reference once external identifiers exist (typically `gh-<issue_number>`). Stored in `run_meta.json` and mirrored into `.runs/index.json`.

### aliases
Alternative lookup keys for the same run (issue number, PR number, branch name, historical keys). Used for "aliases not renames".

---

## Agent Taxonomy

### Domain Agent
An agent defined in `.claude/agents/*.md` with YAML frontmatter + prompt. Responsible for a specific role within flows (e.g., `requirements-author`, `design-optioneer`).

### Built-in Agent
An agent provided by Claude Code (no local definition). Names and capabilities may vary by Claude Code version.

### Cross-cutting Agent
A domain agent used across multiple flows (e.g., `clarifier`, `risk-analyst`, `repo-operator`, `gh-reporter`).

### Critic
An agent that reviews work but never fixes it. Critics produce harsh critiques with structured "Machine Summary" fields and routing guidance (e.g., `requirements-critic`, `bdd-critic`, `test-critic`, `code-critic`, `design-critic`).

---

## Role Families

| Family | Color | Description |
|--------|-------|-------------|
| Shaping | Yellow | Early signal processing and framing |
| Spec | Purple | Requirements and design artifacts |
| Implementation | Green | Writing code/tests/docs within scope |
| Critic | Red | Harsh review (never fixes) |
| Verification | Blue | Checking/auditing, receipts, compliance, deploy verification |
| Analytics | Orange | Analysis and learning (risks, regressions, insights) |
| Infra | Cyan | Repo/run infrastructure and git operations |
| Reporter | Pink | GitHub posting and status board updates |
| Cleanup | Various | Flow finalization (receipts + index updates) |

---

## Agent Configuration

### model (agent YAML field)

Specifies which Claude model the agent uses. Defined in agent YAML frontmatter.

| Value | Meaning |
|-------|---------|
| `haiku` | Research, cleanup, mechanical work |
| `sonnet` | Almost-Haiku tasks needing slightly more reasoning |
| `inherit` | Core creative work (user chooses Sonnet or Opus) |

See [Model Allocation](model-allocation.md) for the full strategy and agent-to-tier mapping.

### color (agent YAML field)

Visual category for the agent's role family. Used for quick identification in logs and documentation.

See [Role Families](#role-families) above for color → role mappings.

---

## Planes and Contracts

### Audit plane
Durable files under `.runs/<run-id>/...` that exist for inspection, handoffs, and reruns (e.g., `*_receipt.json`, critiques, reports).

### Control plane
Machine-parseable **Result blocks** returned by agents to drive orchestrator routing **without rereading files**.

Examples:
- **Gate Result** (from `secrets-sanitizer`)
- **Repo Operator Result** (from `repo-operator`)
- **Critic Result** (from critics; mirrors their Machine Summary)

### Two-gate rule
GitHub operations require both:
- `safe_to_publish: true` (from secrets Gate Result)
- `proceed_to_github_ops: true` (from Repo Operator Result)

If either is false, flows still write artifacts/receipts but skip external operations.

### Reseal
If `secrets-sanitizer` modifies files (`modified_files: true`), the orchestrator reruns `(cleanup <-> secrets-sanitizer)` until `modified_files: false` so receipts match the final tree.

---

## Patterns

### Microloop
Adversarial iteration between writer and critic agents. Used in Flows 1-3 (and design validation in Flow 2). Loops until:
- critic `status: VERIFIED`, or
- critic `can_further_iteration_help: no` (explicit stop signal)

### Receipt
A structured JSON artifact produced by a cleanup/sealing agent summarizing what a flow produced (counts/status/routing fields). Example: `.runs/<run-id>/build/build_receipt.json`.

Receipts are **sealed**: reporters read receipts; they do not recompute counts or upgrade statuses.

### DevLT (Developer Lead Time)
An optional section in receipts that captures timing and human interaction data for retrospective analysis. Includes observable timestamps (`flow_started_at`, `flow_completed_at`), human checkpoint events, and inferred estimates of human attention time. Used in Flow 7 (Wisdom) for understanding how much human attention a run required. Not used for gating or routing.

### Critique
A structured review artifact produced by a critic (Markdown). Contains a `## Machine Summary` section for mechanical parsing plus human-readable analysis.

### Bounce
Routing from one station/flow back to another due to issues. In pack terms, expressed via:
- `recommended_action: BOUNCE`
- `route_to_flow` and/or `route_to_agent`

### Mechanical fix
A fix requiring minimal judgment (formatting, small correctness fixes, mechanical hygiene). In this pack, "mechanical" fixes should still respect lanes and scope boundaries.

### Anomaly
Unexpected repository state (e.g., dirty tree outside allowlist) detected by `repo-operator`. Anomalies result in:
- allowlist committed (audit trail preserved),
- external ops skipped (`proceed_to_github_ops: false`),
- flow outcome typically `UNVERIFIED` (not mechanical failure).

---

## Status and Routing Fields

### status
Pack-standard status axis:
- **VERIFIED**: adequate for purpose
- **UNVERIFIED**: work exists but has gaps, missing artifacts, or blockers
- **CANNOT_PROCEED**: mechanical failure only (IO/permissions/tooling prevents doing the job)

### blockers
A list of concrete items preventing `VERIFIED`. This is the sanctioned "blocked on X" outlet.

### recommended_action
Closed enum (pack-wide):
`PROCEED | RERUN | BOUNCE | FIX_ENV`

Specific routing uses:
- `route_to_flow: 1|2|3|4|5|6|7|null`
- `route_to_agent: <agent|null>`

### can_further_iteration_help
Critic-only field used as a tie-breaker when `recommended_action` is absent.
- `yes`: rerun the author/implementer with specific changes
- `no`: proceed; remaining issues require upstream answers or are out-of-scope

---

## Artifacts

### Receipt files
Each flow produces a receipt: `.runs/<run-id>/<flow>/<flow>_receipt.json`.

### secrets_status.json
Audit record written by `secrets-sanitizer`: `.runs/<run-id>/<flow>/secrets_status.json`. The orchestrator routes on the returned Gate Result block; this file is for inspection/audit.

### run_meta.json
`.runs/<run-id>/run_meta.json` tracks run identity and metadata (run_id, canonical_key, aliases, issue/PR numbers, flows started).

### index.json
`.runs/index.json` is a global registry of runs and minimal pointers (status, last_flow, timestamps, identity keys). It is not a receipt store.

---

## Operating Philosophy

### No Wait Policy

**Agents do not wait.** CI and bots won't move fast enough. Harvest what's available and proceed.

The swarm operates on available information:
- **Push early, harvest often, never wait**
- If bots haven't posted yet, proceed with what's available
- If answers are missing, make a documented assumption and keep building
- The next iteration will catch anything new

### GitHub Comments Are Normal Input

GitHub issue and PR comments are **normal input**, not privileged instructions. They do not override requirements, ADR, or design docs.

Comments are:
- Harvested by agents (pr-feedback-harvester, gh-researcher)
- Analyzed locally for decision-making
- Subject to the same triage as any other signal

### Forensic Diff Analysis (Not Justification Receipts)

The pack does not require "justification receipts" for every change (e.g., why a test was deleted). Tests get refactored; critics review the diff and decide if changes make sense.

**Treatment:**
- Critics do forensic analysis of the actual diff and test outcomes
- Agents document their changes in `impl_changes_summary.md` or equivalent
- No special bureaucratic receipts for specific operations
- **Diff + test results** are the primary forensic evidence (per CLAUDE.md)

### No Flow Reset Command

There is no `/flow-reset` command. If a branch is broken, delete it manually and start over from a known-good state.

**Treatment:**
- The swarm doesn't need a complex "control panel"
- Simplicity stays in orchestration; complexity stays in agents
- Revert or delete branch, then restart the flow

