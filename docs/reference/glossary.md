# Glossary

> Key terms used throughout the demo-swarm pack.

---

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

### Skill
A tool or script that provides mechanical truth to agents. Skills are deterministic, return exit codes and structured data, and have no judgment or decision-making. Agents invoke skills to get ground truth. The pack provides 7 skills: `test-runner`, `auto-linter`, `policy-runner`, `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`.

See [CLAUDE.md Skills section](../../CLAUDE.md#skills) for the full list.

### Orchestrator
A flow command (`.claude/commands/flow-*.md`) that acts as PM. Orchestrators translate intent into agent task sequences, keep flows moving, read prose handoffs and route based on understanding (not parsing), checkpoint at flow boundaries, and evaluate evidence. Orchestrators do not do the work themselves; they scope, sequence, and route it.

See [60-flow-orchestrators.md](../../.claude/rules/60-flow-orchestrators.md) for orchestrator behavior.

---

## Agent Taxonomy

### Domain Agent
An agent defined in `.claude/agents/*.md` with YAML frontmatter + prompt. Responsible for a specific role within flows (e.g., `requirements-author`, `design-optioneer`).

### Built-in Agent
An agent provided by Claude Code (no local definition). Names and capabilities may vary by Claude Code version.

### Cross-cutting Agent
A domain agent used across multiple flows (e.g., `clarifier`, `risk-analyst`, `repo-operator`, `gh-reporter`).

### Critic
An agent that reviews work but never fixes it. Critics produce prose critiques with severity markers and end with a prose handoff recommending next steps. Cleanup agents later derive Machine Summary fields from the prose for audit purposes. (e.g., `requirements-critic`, `bdd-critic`, `test-critic`, `code-critic`, `design-critic`).

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

### Handoff
Natural language communication from an agent to the orchestrator. Contains: what was done, what was found, and a recommendation for next steps. Orchestrators read handoffs and route accordingly. Handoffs are prose, not parsed blocks.

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

## The Physics

These are the six constraints that make stochastic generation safe. See [the-physics.md](../explanation/the-physics.md) for full explanations.

### Mechanical Truth (Physics 1)
Trust tool outputs over agent narratives. Exit codes and counts are what actually happened; agent claims are interpretation. When sources conflict, trust flows downward through the [Truth Hierarchy](#truth-hierarchy).

### Schema Gravity (Physics 2)
The flow structure itself pulls outputs into alignment. Running through flows naturally shapes outputs toward schema alignment because each step has expectations that constrain the next. Existing codebase patterns, conventions, and structure act as a mold that shapes what gets generated.

See [codebase-as-mold.md](../explanation/codebase-as-mold.md).

### Shadow Fork (Physics 3)
The blast-radius principle. Agents operate with default-allow inside the `.runs/<run-id>/` sandbox; gates engage only at publish boundaries (commit, push, GitHub post). Handcuffs kill velocity. Accept iteration messiness because the Gate prevents mess from escaping.

### Throughput Inversion (Physics 4)
Generation capacity exceeds human review capacity by orders of magnitude. Don't make humans read more; make the system prove more. **Verification arbitrage:** burn cheap compute to buy back expensive human attention.

### Adversarial Pressure (Physics 5)
Single agents lie to please. Two agents fighting surfaces truth. The Author (who wants to finish) is pitted against the Critic (who wants to find bugs). The system doesn't proceed until the Critic runs out of ammunition.

### Scoped Context (Physics 6)
Short, focused threads cost less. Context is cost, not knowledge. Every atomic task spins up fresh context with only the 3-5 files relevant to the task. Token costs scale with context; irrelevant context is waste.

---

## Planes and Contracts

### Audit plane
Durable files under `.runs/<run-id>/...` that exist for inspection, handoffs, and reruns (e.g., `*_receipt.json`, critiques, reports).

### Control plane
Machine-parseable **Result blocks** returned by specialized agents for **boolean gate decisions** at publish boundaries. These are distinct from general routing, which uses prose handoffs.

Examples:
- **Gate Result** (from `secrets-sanitizer`) — answers "safe to publish?"
- **Repo Operator Result** (from `repo-operator`) — answers "safe to proceed with GitHub ops?"

Note: General routing uses prose handoffs, not control-plane blocks. Orchestrators read prose recommendations to decide what to run next.

### Two-gate rule
GitHub operations require both:
- `safe_to_publish: true` (from secrets Gate Result)
- `proceed_to_github_ops: true` (from Repo Operator Result)

If either is false, flows still write artifacts/receipts but skip external operations.

### Reseal
If `secrets-sanitizer` modifies files (`modified_files: true`), the orchestrator reruns `(cleanup <-> secrets-sanitizer)` until `modified_files: false` so receipts match the final tree.

---

## The Laws

The eleven immutable rules that govern everything. See [laws-of-the-swarm.md](../explanation/laws-of-the-swarm.md) for full explanations.

| Law | Name | Core Principle |
|-----|------|----------------|
| 1 | Disk Is Memory | State lives on disk, not in chat. Every call is an implicit resume. |
| 2 | Prose Routes Work | Orchestrators read handoffs and decide. No parsing. Claude understands language. |
| 3 | One Agent, One Job | Single responsibility, done deeply. If it needs modes, split it. |
| 4 | Evidence Over Trust | Claims require pointers. "Not measured" is acceptable; false certainty is not. |
| 5 | Fix Forward by Default | "Blocked" is almost always routing. True halting is very rare. |
| 6 | Gate at Boundaries | Default-allow inside workspace; gates engage at publish boundaries only. |
| 7 | Local Resolution First | Try 2-3 targeted specialist calls before bouncing flows. |
| 8 | Truth Flows Downward | Tool outputs > derived facts > intent > implementation > narrative. |
| 9 | Artifacts Reduce Future Work | If an artifact has no future reader, do not create it. |
| 10 | The System Improves | Wisdom feeds back into templates. Failures make the factory smarter. |
| 11 | Keep Going | Flows run to completion. Counts are not exit criteria. |

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
Minutes of human attention time required. The key economic ratio is **Quality:DevLT** (PR quality achieved per minute of developer attention). The pack optimizes for high quality with minimal DevLT by using machine verification to compress review time.

Also: an optional section in receipts that captures timing and human interaction data for retrospective analysis. Includes observable timestamps (`flow_started_at`, `flow_completed_at`), human checkpoint events, and inferred estimates of human attention time. Used in Flow 7 (Wisdom) for understanding how much human attention a run required. Not used for gating or routing.

### PR Cockpit (Glass Cockpit)
The PR description treated as a UI/dashboard. Most reviewers only read the PR description, so it must provide: what changed (summary), why it changed (intent link), what was verified (evidence table), where to spot-check (hotspots), and what's unknown (explicit gaps). The cockpit enables decisions without reading the entire diff.

See [pr-quality-scorecard.md](pr-quality-scorecard.md).

### Critique
A structured review artifact produced by a critic (Markdown). Contains human-readable analysis with severity markers, plus a prose handoff. Cleanup agents derive Machine Summary fields for receipts.

### Bounce
Routing from one station/flow back to another due to issues. Agents express this in prose handoffs (e.g., "This needs to go back to Plan to resolve the contract conflict"). Cleanup agents translate this to `recommended_action: BOUNCE` and `route_to_flow`/`route_to_agent` fields in receipts for audit purposes.

### Mechanical fix
A fix requiring minimal judgment (formatting, small correctness fixes, mechanical hygiene). In this pack, "mechanical" fixes should still respect lanes and scope boundaries.

### Anomaly
Unexpected repository state (e.g., dirty tree outside allowlist) detected by `repo-operator`. Anomalies result in:
- allowlist committed (audit trail preserved),
- external ops skipped (`proceed_to_github_ops: false`),
- flow outcome typically `UNVERIFIED` (not mechanical failure).

### Stagnation
No new signal: same failure signature, same evidence, no meaningful diff change. Stagnation is evidence-based, not count-based. Response is to reroute (try a different agent, change approach), not to stop.

### Oscillation
Toggling between states without convergence. Response is to break the cycle by routing to a different specialist or reframing the problem.

---

## Status and Routing

### Natural Language Routing
The primary way agents communicate routing intent. Agents write prose handoffs that explain what they did, what's left, and their recommendation. Orchestrators (which are Claude threads) read and understand these recommendations to make routing decisions.

**Agents say things like:**
- "Run code-implementer to fix the timeout, then back to me for verification"
- "This needs to go back to Plan flow to resolve the contract conflict"
- "Ready for test-author to add coverage for the error paths"

### Audit Fields (Derived, Not Routing Input)
Receipt fields like `recommended_action`, `route_to_flow`, and `route_to_agent` exist for **audit purposes only**. Cleanup agents derive these values from the agent's prose handoff when writing receipts. Orchestrators do not parse these fields for routing decisions.

### status
Pack-standard status axis:
- **VERIFIED**: adequate for purpose
- **UNVERIFIED**: work exists but has gaps, missing artifacts, or blockers
- **CANNOT_PROCEED**: mechanical failure only (IO/permissions/tooling prevents doing the job)

### Completion Discipline
"Done" is a mechanical state, not a feeling. Flows run to completion; they never stop mid-execution. Evidence panel must be green (or external constraint forces checkpoint). Counts are not exit criteria.

See [60-flow-orchestrators.md](../../.claude/rules/60-flow-orchestrators.md) for completion semantics.

### DEFAULTED
Status for questions that agents resolved by making a safe assumption. The agent researched, derived from patterns, or chose a reversible default, documented reasoning, and proceeded. Contrast with [NEEDS_HUMAN](#needs_human).

See [authority-not-difficulty.md](../explanation/authority-not-difficulty.md).

### NEEDS_HUMAN
Status for questions requiring human authority (not just knowledge). Used only when:
- The decision affects external parties or business relationships
- No safe default exists
- The choice cannot be derived from research or codebase patterns

NEEDS_HUMAN is about authority, not difficulty. Most "blocked" situations are actually DEFAULTED.

See [authority-not-difficulty.md](../explanation/authority-not-difficulty.md).

### The Authority Line
The distinction between DEFAULTED and NEEDS_HUMAN. Ask: does this require someone's **authority** or just someone's **knowledge**? Knowledge problems are DEFAULTED; authority problems are NEEDS_HUMAN. Most questions resolve with knowledge.

### blockers
A list of concrete items preventing `VERIFIED`. This is the sanctioned "blocked on X" outlet.

### recommended_action
Closed enum (pack-wide):
`PROCEED | RERUN | BOUNCE | FIX_ENV`

These values are written to receipts for audit trail. Cleanup agents derive them from prose handoffs.

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

---

## Key Concepts

### Truth Hierarchy
The 5-layer epistemology for resolving conflicts between sources. When sources conflict, trust flows downward:
1. **Tool outputs** (exit codes, stdout) -- what actually happened
2. **Derived facts** (counts, parses) -- mechanical extraction from outputs
3. **Intent** (ADR, BDD, contracts) -- what we meant to build
4. **Implementation** (code) -- what we actually built
5. **Narrative** (agent chat) -- interpretation, useful for reasoning but not truth

An agent's claim does not override a tool output.

See [truth-hierarchy.md](../explanation/truth-hierarchy.md).

### Stochastic Compiler
The mental model for LLMs: not a chatbot, but a **non-deterministic compiler**. Input is natural language specs (BDD, requirements, ADRs); output is implementation code. The process is probabilistic, fallible, and cheap to re-run. Trust the pipeline (generate -> verify -> critique -> refine), not individual generations.

See [stochastic-compiler.md](../explanation/stochastic-compiler.md).

### Verification Arbitrage
The economic strategy: burn cheap compute (tokens, machine time) to buy back expensive human attention. We don't care if the AI generates 500,000 lines of garbage to produce 100,000 lines of gold; the garbage costs nothing and the gold is verified.

### Fix Forward
The default response to problems: route to a fixer and continue. "Blocked" is almost always just routing to another agent. True halting (mechanical failure, authority gap, unsafe boundary) is rare. Prefer DEFAULTED + log over stopping.

See Law 5 in [laws-of-the-swarm.md](../explanation/laws-of-the-swarm.md).

### Two-Pass Minimum
When fixing an issue, re-review at least once to confirm stability. "Two passes" is a minimum observation window, not a maximum retry count. If re-review finds the same issue, you're not stable.

