# The AI Physics of the Pack

> Why the pack is designed this way: sealed stations, microloops, compressors, and mechanical truth.

This document explains the **implicit design constraints** that govern the DemoSwarm architecture. These constraints exist to mitigate specific LLM failure modes (hallucination, context drift, laziness) and leverage their strengths (critique, synthesis, pattern recognition).

---

## Core Constraint

> **Tokens are cheap. Context is finite. Attention is expensive.**

Every pattern in this document exists to maximize engineering output per context window while minimizing the human review burden.

---

## 1. Sealed Stations

**Rule:** Agents do not talk to each other via chat history. They read from disk and write to disk.

Each agent is a **sealed station**: read fresh → do work → write state → die. The next agent starts with zero context and reads from disk. This resets context entropy to zero at every station.

See [Stateless Execution](stateless-execution.md) for the full pattern and why it matters.

---

## 2. The "Compressor" Pattern

**Rule:** Agents that ingest large context must emit small, stable outputs.

### The Problem: Context Explosion

Raw reality is heavy:

- Test logs: 10K+ lines
- GitHub API responses: 100KB+ of JSON
- Git diffs: Hundreds of changed lines

If we carry this through the flow, context exhausts quickly. Worse, LLMs start hallucinating when context is stuffed—they "remember" things from 50K tokens ago that weren't there.

### The Solution

Designate certain agents as **Compressors**:

```
┌────────────────────────────────────────────────────┐
│                   COMPRESSOR                       │
│                                                    │
│   IN:  Heavy context (logs, JSON, diffs)          │
│                                                    │
│   WORK: Parse, filter, triage, summarize          │
│                                                    │
│   OUT: Light artifact (~2KB)                      │
│        + Control block (~200 bytes)               │
└────────────────────────────────────────────────────┘
```

### Compressor Agents

| Agent                   | Heavy Input              | Light Output                                  |
| ----------------------- | ------------------------ | --------------------------------------------- |
| `test-executor`         | 10K lines of test logs   | `test_execution.md` (status + top 5 failures) |
| `pr-feedback-harvester` | 100KB GitHub API JSON    | `pr_feedback.md` + `blockers[]`               |
| `build-cleanup`         | All flow artifacts       | `build_receipt.json` (counts + gates)         |
| `repo-operator`         | `git status`, `git diff` | `Repo Operator Result` block                  |
| `standards-enforcer`    | Full codebase scan       | `standards_report.md` + summary               |

### The Rule

> **Workers may be heavy; their outputs must be light and stable.**

The orchestrator reads the compressed output, not the raw inputs. This keeps flow context clean and prevents hallucination from context stuffing.

---

## 3. The "Context Affinity" Pattern

**Rule:** If an agent has a file open and the token budget to process it, it should do the related work.

### The Problem: Agent Proliferation

It's tempting to create narrow agents for every task:

- `DocstringWriter` for docstrings
- `DebugRemover` for removing `console.log`
- `ImportSorter` for sorting imports

But each agent spawn has cost:

- Context loading overhead
- Round-trip latency
- Loss of local understanding

### The Solution

Group related work by **context loaded**:

| Context Loaded                   | Owner                   | Combined Duties                                        |
| -------------------------------- | ----------------------- | ------------------------------------------------------ |
| `src/*.ts`, ADR, contracts       | `code-implementer`      | Logic, docstrings, local refactor, debug removal       |
| `tests/*.test.ts`, BDD scenarios | `test-author`           | Test writing, fixture updates, spec feedback           |
| `git status`, `git diff`         | `repo-operator`         | Staging, extras detection, security guard, commit/push |
| GitHub API responses             | `pr-feedback-harvester` | Fetching, triage, summarizing                          |

### Efficiency Wins

- We don't have a separate "Anomaly Detector" agent—`repo-operator` sees anomalies while staging
- We don't fetch data in one agent and analyze in another—harvester ingests and emits signal in one pass
- We don't write code in one agent and add docstrings in another—same context, same agent

### The Rule

> **Don't spin up a new agent (and pay the startup cost) just for bureaucratic purity.**

---

## 4. Microloops (Author ↔ Critic)

**Rule:** Never trust an author to critique their own work.

### The Problem: Self-Correction Blindness

LLMs are poor at self-correction in a single pass. They tend to justify their first draft. However, they are **excellent** at critiquing "someone else's" work.

### The Solution

We artificially induce critical distance by splitting the persona:

1. **Author:** Writes the draft (optimistic).
2. **Critic:** Reviews the draft against a checklist (pessimistic).
3. **Loop:** The Author fixes the issues found by the Critic.

**Stop Condition:** The loop ends when the Critic says `can_further_iteration_help: no` or the status is `VERIFIED`.

**Default Cadence:** 2 passes (write → critique → write → critique → proceed).

---

## 5. Mechanical Truth (Null Over Guess)

**Rule:** Agents must not estimate or guess metrics.

### The Problem: Success Pressure → Guessing

Agents under pressure to complete a task will **guess** to finish. If a file is missing, they might try to infer what the count _should_ be (e.g., "I see 0 tests, but maybe they are implied...").

This creates phantom confidence. A guessed `0` looks authoritative but is actually misleading data.

### The Solution

1. **Rust Tooling:** We use `demoswarm` CLI for counting (grep/regex), not LLM vibes.
2. **Explicit Nulls:** We instruct agents to output `null` (not `0`) if a file is missing or unreadable.
3. **Multiple Success Exits:** We give agents `PARTIAL` as a valid success state, so they don't feel forced to guess.

A `null` in a receipt is a signal to the human (missing data). A guessed `0` is misleading.

### The Rule

> **Null over guess. Missing is information; guessing creates phantom confidence.**

---

## 6. Two-Plane Data Model

**Rule:** Separate routing from audit.

| Plane             | Purpose                                                            |
| ----------------- | ------------------------------------------------------------------ |
| **Routing Plane** | Routing ("What happens next?") — prose handoffs returned by agents |
| **Audit Plane**   | Record ("What happened?") — durable files committed to git         |

Orchestrators route on prose handoffs (Claude reads and understands). Files and receipts exist for humans and future review.

See [Why Two Planes](why-two-planes.md) for the full explanation.

---

## 7. Deterministic Tooling

**Rule:** Bash is too creative; use Rust for determinism.

### The Problem: The Bash Tax

- `grep` behaves differently on GNU vs BSD
- `sed` is a minefield of portability issues
- Exit codes vary across platforms
- Quoting rules are arcane

### The Solution

We replaced ad-hoc bash pipelines with the `demoswarm` CLI:

- **The Shim:** `.claude/scripts/demoswarm.sh` ensures we use the same binary logic on Linux, macOS, and Windows
- **Explicit commands:** `demoswarm count pattern` vs `grep -c | wc -l | sed`

### The Rule

> **Trust the shims. Don't bypass `.claude/scripts/demoswarm.sh`.**

---

## Summary for Maintainers

1. **Trust the Shims:** Don't bypass `.claude/scripts/demoswarm.sh`.
2. **Trust the Gates:** If Flow 4 bounces, don't force-merge. Fix the upstream flow.
3. **Respect the Planes:** Orchestrators route on prose handoffs. Receipts are for audit, not routing.
4. **Design for Compression:** Heavy agents must emit light outputs.
5. **Design for Affinity:** Group related work by context loaded, not by task purity.
6. **Null Over Guess:** Missing is information. Guessing creates phantom confidence.

---

## See Also

- [architecture.md](architecture.md) — Overall pack design
- [why-ops-first.md](why-ops-first.md) — The Ops-First philosophy
- [why-two-planes.md](why-two-planes.md) — Control vs audit plane
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
