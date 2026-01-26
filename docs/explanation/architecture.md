# Architecture

> How the pack is built and why.

---

## Context

DemoSwarm exists because the bottleneck in AI-assisted development isn't writing code—it's verifying it.

### The Economic Constraint

Model iteration is cheap relative to reviewer attention. So the goal is not "generate code." The goal is to produce **review-ready evidence**:

- Contracts and decisions (`signal/`, `plan/`)
- Tests and diffs (`build/`)
- Feedback closure (`review/`)
- Audit + merge decision (`gate/`)
- Merge + verification on swarm mainline (`deploy/`)
- Learnings that feed the next run (`wisdom/`)

**What we optimize:** DevLT (Developer Lead Time)—the human minutes required to verify a change. The system is allowed to "grind" if it produces better evidence and fewer review surprises.

### The Trust Model

The pack treats generated code as draft until it's backed by evidence (tests, diffs, critiques). Receipts summarize what happened; the git log is the audit trail.

We don't enforce hard coverage ratios or test-to-code formulas. Instead, critics reason about whether the test _strategy_ matches the code's _risk surface_. See [trust-model.md](../reference/trust-model.md) for details.

### The Topology

Runs execute in a **swarm clone/fork** and converge against swarm `origin/main`. Upstream integration happens after the run is stable, and is handled explicitly (sync/rebase → rerun Flows 4–7 if needed → PR upstream).

This isolates high-churn iteration from human development and keeps `.runs/` artifacts in a reviewable audit trail. See [run-topology.md](../how-to/run-topology.md) and [adopt-fork-workflow.md](../how-to/adopt-fork-workflow.md) for setup.

---

## What DemoSwarm Is

- A **Claude Code pack** (`.claude/`) plus deterministic tooling
- A set of **flows** you dispatch manually—no daemon, no always-on agent
- A system where **artifacts are the handoff** and the chat log is not a source of truth
- A **pre-CI manufacturing line** for changes: turns intent into a PR you can skim

## What DemoSwarm Is Not

- Not an autonomous "ship to prod" agent
- Not a replacement for code review—it produces the _input_ to review
- Not a promise that code is "correct because AI said so"
- Not a repo that merges into upstream automatically (by design)

**In one sentence:** DemoSwarm is an artifact-first, rerunnable SDLC workflow for LLM-driven changes that trades machine iteration for lower human verification time—without hiding the evidence.

---

## Core Patterns

Four patterns that separate DemoSwarm from standard LLM scripts.

### 1. Thick Agents, Thin Flows

To optimize token usage and context windows:

- **Thin Flows:** The Orchestrator follows a simple checklist (`flow-3-build.md`). It spends tokens on _routing_, not _instruction_.
- **Thick Agents:** Complex logic lives inside the Agent prompts (`repo-operator.md`). They spin up fresh contexts, perform heavy analysis (diff checks, log parsing), and return small **Result Blocks**.

**Why this matters:** When logic lives in flows, the orchestrator must tokenize and reason about it every step. When logic lives in agents, a fresh sub-agent context handles the work cheaply. Put decisions in flows, put work in agents.

### 2. The Compressor Pattern

We use agents to "compress" reality into signal.

```
┌─────────────────────────────────────────┐
│  pr-feedback-harvester                  │
│                                         │
│  IN:  100KB of GitHub API JSON          │
│       (reviews, comments, checks)       │
│                                         │
│  OUT: pr_feedback.md (~2KB)             │
│       + Result block (~200 bytes)       │
└─────────────────────────────────────────┘
```

Examples:

- `test-executor`: 10K lines of console logs → `test_execution.md` (status + top failures)
- `pr-feedback-harvester`: GitHub API firehose → `pr_feedback.md` + normalized `blockers[]`
- `build-cleanup`: All flow artifacts → `build_receipt.json` (mechanical counts)

**Rule:** Workers may be heavy; their outputs must be light and stable.

### 3. Receipts as Evidence (Not Gatekeepers)

We don't use receipts as permission boundaries. We use them as **Logs**.

- The `*-cleanup` agents verify that the _logical outcome_ (Test Report exists and is Green) is true.
- The `secrets-sanitizer` ensures the _package_ is safe (Redacting logs).
- We accept divergence between the Receipt and the Commit to maintain velocity.

**State-First Verification:** The repo's current state (HEAD + working tree + staged diff) is the thing you're building and shipping. Receipts help investigate what happened—but they're not the primary verification mechanism once the repo has moved.

### 4. Mechanical Guardrails ("Physics")

We enforce safety via tools, not prompts.

| Guardrail               | Implementation                                                                                                                                                              |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Anti-Reward Hacking** | Critics + `standards-enforcer` detect test deletions/weakened assertions during microloops. If detected, route to rework immediately—don't let it reach Gate as a surprise. |
| **Intent + Extras**     | `repo-operator` detects ad-hoc human edits ("Extras") and stages them automatically. It assumes collaboration, not conflict.                                                |
| **Fix-First Security**  | `secrets-sanitizer` runs as a pre-commit hook. It redacts in-place. It only blocks publish when manual remediation is required—work never stops.                            |

---

## Core Philosophy: Ops-First

The DemoSwarm pack is a **build pipeline with guardrails**, not a guardrail pipeline that sometimes builds.

**The shift:** From "Compliance Engine" (policing the robot) to "Code Conveyor Belt" (empowering the robot to ship).

### Objective

Optimize **Dev Lead Time**: minutes of human attention per trusted change.

### Core Constraint

> Tokens are cheap. Context is finite. Attention is expensive.

This constraint shapes every design decision. We maximize engineering output per context window while minimizing human review burden.

---

## The Two Execution Planes

The pack separates **where work happens** from **where gates engage**:

| Plane             | Posture       | Purpose                           | Example                                    |
| ----------------- | ------------- | --------------------------------- | ------------------------------------------ |
| **Work Plane**    | Default-allow | Explore, implement, iterate       | Reading files, writing code, running tests |
| **Publish Plane** | Gated         | Control what leaves the workspace | Commit, push, GitHub post                  |

### Work Plane (Default-Allow)

Everything up to staging runs without friction:

- Read any files, search code, run checks
- Write tests early, iterate on code freely
- Run tests locally, fix issues as discovered
- Push early to get bot feedback (CI, CodeRabbit)
- Security findings here are **advisory**, not throttles

### Publish Plane (Gated)

Gates engage only when crossing the boundary:

- **Commit**: secrets-sanitizer scans staged changes
- **Push**: repo-operator checks for anomalies
- **GitHub post**: content mode restricts what gets posted (not what's analyzed)

If a gate blocks, **keep working locally**. Gates constrain publishing, not thinking.

**Key insight:** This separation prevents "security theater" where agents spend more time proving they're allowed to act than actually acting.

---

## Key Design Patterns

### Context Affinity

**Principle:** If an agent has a file open and the token budget to process it, it should do the related work.

Don't spin up a new agent (and pay the startup cost) just for bureaucratic purity.

| Context Loaded                          | Owner                   | Combined Duties                                        |
| --------------------------------------- | ----------------------- | ------------------------------------------------------ |
| `src/*.ts`, `ac_matrix.md`              | `code-implementer`      | Logic, docstrings, local refactor, debug removal       |
| `features/*.feature`, `tests/*.test.ts` | `test-author`           | Test writing, fixture updates, spec feedback           |
| `git status`, `git diff`                | `repo-operator`         | Staging, extras detection, security guard, commit/push |
| GitHub API JSON                         | `pr-feedback-harvester` | Harvesting, triage, summarizing                        |

**Efficiency wins:**

- We don't have a separate "Anomaly Detector" agent—repo-operator sees anomalies while staging
- We don't fetch data in one agent and analyze in another—harvester ingests and emits signal in one pass

### Critics Never Fix

Critics write harsh assessments; implementers apply fixes.

```
author → artifact → critic → critique → author → improved artifact → ...
```

**Why:** Separation prevents "critic fixes its own issues" loops and maintains clear accountability.

### Microloops

Writer ↔ Critic iteration until:

- `status: VERIFIED`, OR
- `can_further_iteration_help: no`

Default cadence: 2 passes (write → critique → write → critique → proceed).

**Why:** Bounded iteration prevents infinite loops while ensuring quality.

### Intent + Extras (Embrace Ad-Hoc Fixes)

The orchestrator tells agents **what to produce** (intent). Agents figure out **what paths to touch** (execution).

When staging, expect "extras" (files changed outside the expected set):

1. **Stage them** by default (assume the developer did them for a reason)
2. **Record them** in `extra_changes.md`
3. **Do not block** unless they trigger a hard guardrail (test deletion)

**Why:** Developers jump in to fix typos or tweak config while the swarm runs. This is collaboration, not attack.

---

## The Data Model: Two Planes (Routing vs Audit)

Separate from Work/Publish planes, the pack has two **data planes**:

| Plane       | Artifacts                                         | Purpose                 | Lifecycle                             |
| ----------- | ------------------------------------------------- | ----------------------- | ------------------------------------- |
| **Routing** | Prose handoffs, `Gate Result` (for boolean gates) | Routing decisions       | Ephemeral (read once, route, discard) |
| **Audit**   | `*_receipt.json`, `*.md` artifacts, `index.json`  | Record of what happened | Durable (committed to git)            |

**Crucial rule:** Orchestrators route on prose handoffs, not by parsing structured blocks or re-reading files.

```
Agent runs
  ├─→ Writes audit artifacts (files)
  └─→ Returns prose handoff (response)

Orchestrator
  ├─→ Routes on prose handoff (Claude reads and understands)
  └─→ Does NOT parse structured data for routing
```

---

## The Seven Flows

**7 logical flows exposed via 8 slash commands** (7 flow commands + `/customize-pack`).

The flows represent the logical SDLC stages. Each flow has one primary command:

| Flow      | Slash Command      | Input                    | Output                   | Purpose                |
| --------- | ------------------ | ------------------------ | ------------------------ | ---------------------- |
| 1. Signal | `/flow-1-signal`   | Raw request              | Requirements, BDD, risks | Shape the work         |
| 2. Plan   | `/flow-2-plan`     | Signal outputs           | ADR, contracts, plans    | Design the solution    |
| 3. Build  | `/flow-3-build`    | Plan outputs             | Code, tests, reviews     | Implement with tests   |
| 4. Review | `/flow-4-review`   | Build outputs + Draft PR | PR feedback, worklist    | Harvest PR feedback    |
| 5. Gate   | `/flow-5-gate`     | Review outputs           | Merge decision           | Pre-merge verification |
| 6. Deploy | `/flow-6-deploy`   | Gate outputs             | Verification, deployment | Release to mainline    |
| 7. Wisdom | `/flow-7-wisdom`   | All outputs              | Learnings, regressions   | Close feedback loops   |

Additionally, `/customize-pack` provides interactive configuration for adapting the pack to your stack.

### Flows vs Commands vs Skills

| Concept | Count | What It Is | How to Invoke |
| ------- | ----- | ---------- | ------------- |
| **Flow** | 7 | Logical SDLC stage (Signal through Wisdom) | Via its slash command |
| **Command** | 8 | Slash command file in `.claude/commands/` | `/flow-N-name` or `/customize-pack` |
| **Skill** | 7 | Mechanical tool (test-runner, auto-linter, etc.) | `/skill-name` or via agent |

**When to use each:**
- **Flows** are your primary interface. Run `/flow-1-signal` to start, then proceed through the flows in order.
- **Skills** are typically invoked by agents during flows, but you can call them directly for specific tasks (e.g., `/test-runner` to run tests, `/auto-linter` to format code).
- **/customize-pack** is a one-time setup command to configure the pack for your project.

### Flow 3: Build (The Construction Site)

**Posture:** High velocity. Push early, fail fast.

Key stations:

1. **AC Microloops:** Test ↔ Critic ↔ Code ↔ Critic (per acceptance criterion)
2. **Early PR Bootstrap:** After first vertical slice, push + create Draft PR to get bots spinning
3. **Feedback Check:** Harvest PR feedback, route on blockers (CRITICAL only during Build)
4. **Global Hardening:** standards-enforcer (polish) → test-executor (verify)
5. **Ship:** Seal receipt → sanitize → push

### Flow 4: Review (The Inspection Chamber)

**Posture:** High rigor. Drain the worklist.

Key stations:

1. **Harvest:** Full PR feedback (all severities, including nits)
2. **Worklist Loop:** Unbounded iteration until complete or context exhausted
3. **Context Checkpoint:** If context > 80%, checkpoint and exit with `PARTIAL` status
4. **Re-Harvest Cadence:** Push → re-harvest after every N items (capture new bot comments)

### Flow 6: Deploy (Mainline Promotion)

**Two-Axis Verdict:**

- `deploy_action`: COMPLETED | SKIPPED | FAILED
- `governance_enforcement`: VERIFIED | VERIFIED_RULESET | UNVERIFIED_PERMS | NOT_CONFIGURED | UNKNOWN

This separates "what happened" (deploy action) from "can we verify protections" (governance enforcement).

### Flow 7: Wisdom (One-Way by Design)

Flow 7 extracts learnings and proposes actions, but **does not auto-apply** them.

- Outputs `learnings.md` and `feedback_actions.md` with recommendations
- Humans review and decide what to adopt
- No automatic injection into future flows

**Why:** Prevents "learning drift" where the swarm autonomously tightens constraints until shipping becomes impossible. Every policy change has a human decision point.

---

## Agent Taxonomy

| Family         | Color   | Behavior                    |
| -------------- | ------- | --------------------------- |
| Shaping        | Yellow  | Early signal processing     |
| Spec           | Purple  | Write requirements/design   |
| Implementation | Green   | Write code/tests/docs       |
| Critic         | Red     | Harsh review (never fixes)  |
| Verification   | Blue    | Audit and check             |
| Analytics      | Orange  | Analysis and learning       |
| Infra          | Cyan    | Git and run infrastructure  |
| Reporter       | Pink    | GitHub posting              |
| Cleanup        | Various | Seal receipts, update index |

### Key Agents and Their Contexts

| Agent                   | Role          | Context Strategy                                               |
| ----------------------- | ------------- | -------------------------------------------------------------- |
| `repo-operator`         | State Manager | Intent-based staging; embraces extras; guards test deletion    |
| `pr-feedback-harvester` | The Eyes      | Compressor; ingests API JSON, outputs normalized blockers      |
| `secrets-sanitizer`     | The Janitor   | Fix-first pre-commit hook; redacts in-place; doesn't route     |
| `test-executor`         | Verifier      | Compressor; runs suite, outputs pass/fail summary              |
| `standards-enforcer`    | Polisher      | Runs formatters, strips debug artifacts                        |
| `code-implementer`      | Writer        | Writes code + docstrings; focuses on correctness               |
| `*-cleanup`             | Auditors      | Verify logical outcomes; write SKIPPED stubs for missing steps |

---

## Safety Boundaries

### Two-Gate Rule

GitHub operations require BOTH:

1. `safe_to_publish: true` (secrets-sanitizer)
2. `proceed_to_github_ops: true` (repo-operator)

**Why:** No accidental exposure or push of unexpected content.

### Single-Pass Sanitization

The sanitizer runs **once** before push:

1. Scan staged files and allowlist artifacts
2. Auto-fix: redact secrets in-place
3. Do NOT trigger a reseal loop

**Why:** The old behavior created "Compliance Recursion" where redacting triggered receipt regeneration, burning tokens on paperwork instead of engineering.

### Safe-Bail

When publishing can't proceed safely:

- `checkpoint_mode: local_only`
- Never push
- Flow completes UNVERIFIED with evidence

**Why:** Prefer local completion over stuck or exposed state.

---

## Deterministic Tooling

### Why Rust over Bash

We replaced ad-hoc bash pipelines with the `demoswarm` CLI because:

- **The "Bash Tax":** `grep` behaves differently on GNU vs BSD. `sed` is a minefield.
- **The Shim:** `.claude/scripts/demoswarm.sh` ensures consistent behavior across platforms.

### The Shim Pattern

Agents **always** invoke via shims:

```bash
# Never this:
grep -c "pattern" file.md

# Always this:
bash .claude/scripts/demoswarm.sh count pattern --file "file.md" --regex "pattern"
```

---

## What Lives Where

| Content            | Location                           |
| ------------------ | ---------------------------------- |
| Flow behavior      | `.claude/commands/flow-*.md`       |
| Agent behavior     | `.claude/agents/*.md`              |
| Shared invariants  | `CLAUDE.md`                        |
| Mechanical helpers | `.claude/skills/*/SKILL.md`        |
| Validation         | `.claude/scripts/pack-check.sh`    |
| Run artifacts      | `.runs/<run-id>/` (in target repo) |

---

## The "Feel Test"

The system is working when:

1. **You can fix a typo** in `README.md` while the swarm builds, and `repo-operator` just includes it ("Extras")
2. **You delete a test**, and `repo-operator` refuses to push ("Anti-Reward Hacking")
3. **CI fails** on AC-1, and Flow 3 stops immediately to fix it ("Pulse")
4. **CodeRabbit suggests a nit**, and Flow 3 ignores it, but Flow 4 fixes it ("Triage")

---

## Architecture Laws

Seven invariants that prevent execution drift. Violating them creates subtle failures.

### Law 1: PM/IC Boundary

**Orchestrators route. Agents work. Cleanup agents audit.**

| Role                            | Responsibility                                                     | Example                                          |
| ------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------ |
| **Orchestrator** (flow session) | Call agents, route on Result blocks, manage TodoWrite              | "If gate_verdict is MERGE, proceed to merge ops" |
| **Worker** (agent)              | Do the work, update tracking artifacts, report honestly            | `code-implementer`, `test-author`, `fixer`       |
| **Auditor** (cleanup agent)     | Verify on-disk state matches evidence (tests/diffs), seal receipts | `build-cleanup`, `review-cleanup`                |

Workers own their progress updates. Cleanup agents verify that claims match reality.

**Violation:** Orchestrator parsing `ac_status.json` to determine routing.
**Correct:** Orchestrator routes on the Result block returned by `build-cleanup`.

### Law 2: Every Call Is an Implicit Resume

**Agents don't need "resume mode" flags.** They check disk state and determine what's left.

When an agent starts:

1. Check if its tracking artifact exists (e.g., `ac_status.json`, `review_worklist.json`)
2. If yes: read it, determine what's PENDING, continue from there
3. If no: initialize fresh

**Violation:** Flow says "call build-cleanup in resume mode."
**Correct:** Flow says "call build-cleanup." The agent checks disk state and behaves appropriately.

**Corollary:** If an agent needs genuinely different behavior (not just "resume vs fresh"), that's a signal for two separate agents, not a `mode:` parameter.

### Law 3: Workers Maintain the Ledger

**The worker who touches the code is the worker who updates the status.**

Workers (`code-implementer`, `test-author`, `fixer`, `doc-writer`) update their tracking artifacts (`ac_status.json`, `review_worklist.json`) **before** reporting back to the orchestrator. This ensures:

- The "save game" is atomic with the work
- The orchestrator routes on Result blocks, not prose parsing
- State survives context exhaustion

| Artifact Type                                                 | Who Updates                                    | Who Audits                                      |
| ------------------------------------------------------------- | ---------------------------------------------- | ----------------------------------------------- |
| **Progress state** (`ac_status.json`, `review_worklist.json`) | Worker completing the work                     | Cleanup agent cross-checks against test results |
| **Receipts** (`*_receipt.json`)                               | Cleanup agent (mechanical derivation)          | Downstream flows                                |
| **Index** (`.runs/index.json`)                                | Cleanup agents, `run-prep`, `gh-issue-manager` | —                                               |

**Key insight:** Cross-agent coordination happens through artifacts, not prose parsing. The cleanup agent reads `ac_status.json` and cross-references it with `test_execution.md`. If they disagree, it reports a **Forensic Mismatch** — status becomes UNVERIFIED.

### Law 4: AC Termination = Green + Orchestrator Agreement

**"Green tests" is necessary but not sufficient for AC completion.**

An AC is done when:

1. `test-executor` returns Green for that AC's scope
2. The orchestrator agrees there's nothing left worth fixing based on critic feedback

**The loop:** implement → test → critique → (if critic has actionable items) → improve → test again

Even with green tests, if `code-critic` identifies a maintainability risk or clear technical debt, the orchestrator should authorize one improvement pass. The critic's `can_further_iteration_help: no` signal (or orchestrator judgment) terminates the loop.

### Law 5: Research-First Autonomy

**If an agent can't derive an answer, it investigates first, then defaults, then escalates.**

The escalation ladder (in order):

1. **Investigate locally:** Search code, tests, configs, prior runs, existing docs
2. **Investigate remotely (if allowed):** GitHub issues/PRs, web search, library docs
3. **Derive from evidence:** Use patterns in the codebase to infer correct behavior
4. **Default if safe:** Choose a reversible default, document it, continue
5. **Escalate only when boxed in:** All of the above failed AND no safe default exists

**The bar for human escalation is high.** A timeout value? Look at existing timeouts. An error format? Look at existing error handlers. Auth approach? Look at existing auth code.

### Law 6: Foundation-First Sequencing

**Infrastructure subtasks are the root of the dependency tree.**

The `work-planner` designs dependency graphs where:

- Infrastructure/migration subtasks (ST-000, etc.) have no dependencies
- Logic subtasks that consume new state list infrastructure in `depends_on`
- Critics validate that dependencies flow downward (foundations → walls → roof)

**Example:** `ST-001: Create sessions table` has no dependencies. `ST-002: Implement login flow` has `depends_on: ["ST-001"]`.

### Law 7: Local Resolution (The "Zero-Wait" Rule)

**Mismatches between "Plan" and "Reality" are normal. Resolve them locally first.**

When an agent hits a logic gap, design contradiction, or implementation snag:

1. **Don't bail to a previous flow.** Machine time is cheap relative to human interrupt.
2. **Call a reasoning agent within the current flow.** Route to `design-optioneer`, `adr-author`, or `impact-analyzer` to provide a surgical fix.
3. **Re-plan locally.** Have the specialist update `ac_matrix.md` or `work_plan.md` in-place.
4. **Resume.** Hand the micro-fix back to the implementer.

**BOUNCE only when:**

- The specialists agree the entire architecture is invalid
- The fix requires upstream stakeholder decisions
- Multiple flows worth of work needs revisiting

**The bar for flow-level bounces is high.** 2-3 surgical agent calls are always cheaper and faster than a full context switch.

---

## Flow Authoring Rule

**Flows are routing tables. Agents are workers.**

This separation is about **token economics**: Orchestrator context is expensive, Agent execution is cheap.

### Flows contain

- Station order (which agents to call, in what sequence)
- Routing logic (which Result block to read, what to do on PROCEED/RERUN/BOUNCE)
- Artifact expectations (what outputs to expect from each station)
- Termination conditions (when the flow is complete)

### Flows must NOT contain

- Shell snippets (beyond illustrative examples)
- File path lists to stage/check (move to agents)
- Parsing logic or computation
- If/else chains for file existence checks

### Agents contain

- All procedural work (read files, run commands, write outputs)
- Intent-to-paths mapping (the agent figures out what to stage)
- Validation logic (the agent checks if things are correct)
- Prose handoffs for orchestrator routing (Claude reads and understands)

**Why this matters:** When logic lives in flows, the orchestrator must tokenize and reason about it every step. When logic lives in agents, a fresh sub-agent context handles the work cheaply. Put decisions in flows, put work in agents.

---

## See Also

- [why-ops-first.md](why-ops-first.md) — The philosophy behind default-allow engineering
- [agent-philosophy.md](agent-philosophy.md) — Agent intelligence and resiliency
- [ai-physics.md](ai-physics.md) — LLM-specific design constraints
- [why-two-planes.md](why-two-planes.md) — Control vs audit plane separation
- [why-two-gates.md](why-two-gates.md) — GitHub ops gating
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
