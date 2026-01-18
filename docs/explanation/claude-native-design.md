# Claude-Native Design

> How the DemoSwarm pack works with Claude Code's architecture.

This document explains the fundamental principles that make this swarm pack effective with Claude Code. Understanding these patterns will help you modify the pack, debug issues, and design new agents that fit naturally.

---

## The Core Mental Model

### Orchestrator = PM, Agents = Well-Trained Juniors

The central metaphor: **the orchestrator is a project manager coordinating a team of well-trained junior engineers.**

The PM:

- Knows the overall goal and the order of steps
- Assigns work to the right people
- Routes on status updates (not by doing the work themselves)
- Makes decisions when workers disagree or get stuck

The juniors:

- Have deep expertise in their specific area
- Investigate problems before asking for help
- Report what they did, what they found, and what they recommend
- Communicate naturally, not in rigid data formats

This metaphor shapes everything. When you design an agent, ask: "How would a senior engineer explain this task to a capable junior?" When you design a flow, ask: "What does a PM need to know to route work?"

### Claude Reads and Understands, Doesn't Parse

Claude is a language model, not a JSON parser. Its strength is understanding intent and context from natural language.

**The anti-pattern:**

```yaml
action: PROCEED
next_agent: code-implementer
reason_code: REQ_COMPLETE
```

**The Claude-native pattern:**

```markdown
**What I did:** Reviewed the requirements against the acceptance criteria.
Found all five requirements are testable and unambiguous.

**What's left:** Nothing for this stage. Ready for implementation.

**Recommendation:** Run code-implementer next to start building.
```

The second version gives Claude the context it needs. A PM reading the first version would need to decode `reason_code: REQ_COMPLETE`. A PM reading the second version knows exactly what happened.

### Natural Language Communication Throughout

The pack uses prose handoffs between agents, not structured data exchanges.

Why:

- **Claude is good at understanding prose.** It can infer routing from "run the implementer next" as easily as from `next_agent: code-implementer`.
- **Prose captures nuance.** "The timeout logic is wrong, but it's a minor fix that code-implementer can handle" is richer than `severity: MINOR, agent: code-implementer`.
- **Humans can audit it.** When something goes wrong, you read a story, not decode a schema.

The only structured data in the pack is for **mechanical operations**: receipts (for counting), control blocks (for gates), and status files (for resumption).

---

## Why Agents Exist

Agents exist for exactly two reasons: **to do work** or **to compress context**.

### Reason 1: Do Work

The primary reason to spawn an agent is that work needs doing.

A fresh agent context gives you:

- Clean token budget (no accumulated context bloat)
- Focused attention (single responsibility)
- Cheaper execution (sub-agents are cheaper than orchestrator tokens)

Examples:

- `code-implementer`: Writes the implementation
- `test-author`: Writes tests
- `code-critic`: Reviews code quality
- `repo-operator`: Manages git operations

### Reason 2: Compress Context

Some agents exist primarily to digest heavy inputs and emit light outputs.

Raw reality is often too heavy to carry through a flow:

- Test logs: 10K+ lines
- GitHub API responses: 100KB+ of JSON
- Git diffs: Hundreds of changed lines

A compressor agent ingests this heavy context and emits a concise summary:

```
test-executor:
  IN:  10,000 lines of pytest output
  OUT: test_execution.md (200 lines: status, top failures, evidence)

pr-feedback-harvester:
  IN:  100KB of GitHub API JSON
  OUT: pr_feedback.md (2KB: categorized feedback, blockers)
```

The orchestrator reads the compressed output. The raw inputs never pollute the flow context.

### The Spawn Decision Framework

Before creating a new agent, ask:

1. **Is there work to do?** If yes, does an existing agent cover this work?
2. **Is there context to compress?** If yes, which agent should digest it?
3. **Would this work be cheaper in the current context?** Sometimes adding a few lines to an existing agent is better than spawning a new one.

### When NOT to Spawn

Don't create a new agent when:

- **The work is trivial.** Adding a docstring doesn't need a `DocstringWriter` agent. The implementer does it.
- **The context is already loaded.** If `code-implementer` has the file open, it should fix the import, not hand off to `ImportFixer`.
- **The handoff cost exceeds the work.** Spawning an agent has overhead: context loading, round-trip latency, summary writing. If the work is smaller than the overhead, don't spawn.

This is the **context affinity** principle: if an agent has a file open and the token budget to process it, it should do the related work.

---

## How Agents Communicate

### Agent to Orchestrator: The Handoff

Every agent ends with a **handoff**: a natural language summary that tells the orchestrator what happened and what to do next.

The handoff answers three questions:

1. **What did you do?** Summary of work completed.
2. **What's left?** Remaining work, blockers, open questions.
3. **What do you recommend?** Specific next step with reasoning.

**Example:**

```markdown
## Handoff

**What I did:** Implemented REQ-001 through REQ-003. Added unit tests for the
login flow. The session timeout logic matches the ADR specification (15 minutes).

**What's left:** REQ-004 (password reset) requires email service integration
that isn't configured yet. REQ-005 is blocked on the database migration.

**Recommendation:** Run test-executor to verify the implementation, then
pause on REQ-004/005 until infrastructure is ready. Document the blockers
in open_questions.md for the human reviewer.
```

The orchestrator reads this and knows:

- Three requirements are done
- Two are blocked (with specific reasons)
- Next step is test execution
- There's a clear handoff point for human decision

### Orchestrator to Agent: Natural Language Tasks

The orchestrator assigns work using natural language, not configuration objects.

**The anti-pattern:**

```yaml
agent: code-implementer
config:
  target_files: ["src/auth/login.ts", "src/auth/session.ts"]
  requirements: ["REQ-001", "REQ-002", "REQ-003"]
  mode: implement
  strict: true
```

**The Claude-native pattern:**

```markdown
Implement REQ-001, REQ-002, and REQ-003 from the requirements document.
The ADR specifies JWT-based authentication with 15-minute session timeout.
Write tests alongside the implementation. Update the AC status when done.
```

The second version tells the agent what to accomplish and provides context. The agent figures out which files to touch.

### Positive Framing

Agent prompts emphasize **what to do**, not **what not to do**.

**The anti-pattern:**

```
DO NOT:
- Modify files outside the manifest
- Skip writing tests
- Commit directly to main
- Use deprecated APIs
- Ignore the ADR constraints
```

**The Claude-native pattern:**

```
Your goal is to implement the acceptance criteria with full test coverage.

Read the ADR to understand the design constraints. Write tests alongside
your implementation. Update the AC status file when you complete each item.

If you need to modify files not listed in the manifest, do so. The critic
will review your changes.
```

Negative lists create defensive behavior. Positive framing creates capable behavior.

---

## The Handoff Contract

The handoff is the **primary communication interface** between agents and the orchestrator.

### The Three Questions

Every handoff answers:

1. **What did you do?**
   - Summary of completed work
   - What changed (areas, not exhaustive file lists)
   - Evidence (test results, validation outcomes)

2. **What's left?**
   - Remaining work in scope
   - Blockers (things you couldn't do and why)
   - Open questions (things you couldn't answer)

3. **What do you recommend?**
   - Specific next step
   - Named agent if you know it
   - Reasoning for your recommendation

### Always Make a Recommendation

Even when uncertain, take a stance.

**Weak:**

```
I'm not sure what to do next. The tests pass but there might be issues.
```

**Strong:**

```
Tests pass. Code-critic should review for maintainability before we proceed.
The session logic is complex enough that a second set of eyes would help.
```

The orchestrator can override your recommendation. But if you don't make one, the orchestrator has to do your thinking for you.

### Graceful Outcomes

**Honest partial reports are successful outcomes.**

A handoff that says "I completed 2/5 ACs, blocked on missing schema" is a verified success. The orchestrator knows exactly what happened and what to do next.

A handoff that says "All 5 ACs complete (assuming schema exists)" is a high-risk failure. It hides uncertainty behind false completion.

**PARTIAL is a win.** If you:

- Made real progress
- Documented what's done and what's blocked
- Left the codebase in a runnable state

...then reporting partial completion with honest blockers is correct. The flow will rerun and pick up where you left off.

---

## Artifacts with Substance

Artifacts are the durable record of what happened. They serve two audiences: **future agents** (who read them for context) and **humans** (who review them for decisions).

### The Artifact Test

Before writing an artifact, ask: **Would this be worth reading in 3 months?**

If you're debugging a failure in 3 months, would this artifact help you understand what happened? If yes, it has substance. If no, it's probably routing noise.

### Good Artifacts Explain the Why

**Stubby artifact (bad):**

```markdown
# Requirements Critique

Status: VERIFIED
Issues: 0
```

**Substantive artifact (good):**

```markdown
# Requirements Critique

All five requirements pass the testability check.

REQ-001 through REQ-003 have clear acceptance criteria with measurable outcomes.
REQ-004 references "appropriate security measures" which I interpret as the
OWASP guidelines mentioned in NFR-SEC-001. REQ-005's timeout value aligns with
the ADR's session management constraints.

No blocking issues. Recommend proceeding to implementation.
```

The second version tells a future reader what was checked and how the critic reasoned. The first version is a checkbox.

### Bad Artifacts Are Stubby Routing Gates

Some artifacts exist only to satisfy a "did you produce X?" check. They look like:

```markdown
# Security Review

Passed.
```

This artifact has no substance. It doesn't explain what was checked, what was found, or why it passed. A human reviewer learns nothing. A future agent gets no context.

If an artifact would be this stubby, either:

1. **Make it substantive.** Explain what you checked and why it passed.
2. **Don't produce it.** Let the handoff convey the status.

---

## Agent Design Principles

### Single Responsibility

Each agent has one job, done deeply.

**Critics critique.** They never fix. They produce assessments with severity ratings and evidence.

**Workers implement.** They write code, tests, or documentation. They maintain the ledger (update status files).

**Cleanup agents audit.** They verify claims against evidence and write receipts.

Mixing responsibilities creates confusion. If a critic fixes issues it finds, you lose the independent verification. If a worker writes its own receipt, you lose the audit trail.

### Positive Prompting

Tell agents what to do, not what not to do.

- "Write comprehensive tests for the login flow" (positive)
- "Don't skip edge cases" (negative)

Positive prompts activate Claude's constructive capabilities. Negative prompts activate defensive behaviors that can limit exploration.

### Graceful Outcomes

Design agents with multiple successful exit states.

- **COMPLETED**: Did everything, no blockers
- **PARTIAL**: Did some things, documented blockers
- **CANNOT_PROCEED**: Mechanical failure (missing file, broken tool)

PARTIAL is a success. It means the agent did real work and honestly reported what's left. The flow can rerun and pick up where it stopped.

Only CANNOT_PROCEED is a failure, and only because it indicates environmental problems, not agent problems.

### Real Cognitive Work

Agents do thinking, not copying.

**Mechanical work (avoid):**

```
Read the test file. Count the test functions. Write the count to a file.
```

**Cognitive work (prefer):**

```
Analyze the test strategy. Does it cover the risk surface of the implementation?
Are there gaps where untested code could fail? What's your assessment?
```

Mechanical work should be done by tools (the `demoswarm` CLI, bash scripts). Agents should do the work that requires judgment.

---

## What's Still Needed

The pack isn't pure prose. Some mechanical elements are necessary for reliability.

### Receipts (Audit Trail)

Receipts are structured JSON files that record what happened in each flow.

```json
{
  "run_id": "feat-auth",
  "flow": "build",
  "status": "VERIFIED",
  "counts": {
    "tests_written": 12,
    "files_changed": 8
  },
  "completed_at": "2025-01-10T14:32:00Z"
}
```

Receipts serve humans reviewing later, not routing. The orchestrator routes on handoffs; receipts are the durable evidence.

**Rule:** Receipt data must be mechanical (derived by tools), not estimated by agents. A count of 12 tests means 12 tests actually exist. If the tool can't count, the value is `null`, not a guess.

### Markers (Counting)

Some artifacts contain **stable markers** that enable mechanical counting:

```markdown
### REQ-001: User login

### REQ-002: Session management

### REQ-003: Password reset
```

The pattern `^### REQ-` is a stable marker. The CLI can count occurrences without parsing prose:

```bash
bash .claude/scripts/demoswarm.sh count --file requirements.md --pattern "^### REQ-"
```

Markers enable receipts to contain accurate counts without agent estimation.

### Skills (Deterministic Work)

Skills are mechanical helpers that do work too deterministic for agent judgment:

- `test-runner`: Runs the test suite, captures output
- `auto-linter`: Formats code, runs linters
- `runs-derive`: Counts markers in artifacts
- `secrets-tools`: Scans for secrets before publish

Skills are invoked by agents but produce deterministic outputs. The test runner doesn't decide if tests pass; it runs them and reports results.

---

## State and Resumption

### State Lives on Disk

Flows don't carry context between invocations. State transfers through artifacts on disk.

```
.runs/feat-auth/
  signal/
    requirements.md
    signal_receipt.json
  plan/
    adr.md
    plan_receipt.json
  build/
    impl_changes_summary.md
    ac_status.json
    build_receipt.json
```

When Flow 3 (Build) starts, it reads Flow 1 and Flow 2 outputs from disk. It doesn't inherit context from prior flows.

### Every Call Is an Implicit Resume

Agents don't need "resume mode" flags. They check disk state and determine what's left.

When an agent starts:

1. Check if its tracking artifact exists (e.g., `ac_status.json`)
2. If yes: read it, determine what's PENDING, continue from there
3. If no: initialize fresh

This makes flows **naturally resumable**. If a flow crashes mid-execution, rerunning it picks up from the last checkpoint.

### Naturally Recoverable

Because state lives on disk and agents check before acting, the pack is resilient to interruption.

- Context exhaustion? Write checkpoint, exit with PARTIAL. Rerun continues.
- Tool failure? Report CANNOT_PROCEED with evidence. Human fixes environment, reruns.
- Human intervention? Edit the artifact on disk. Next agent sees the change.

No special recovery logic needed. The disk is the truth.

---

## What to Avoid

### Harness-Era Patterns

The pack replaces patterns from traditional automation harnesses. These patterns don't fit Claude Code.

| Old Pattern                | Problem                                         | Claude-Native Alternative          |
| -------------------------- | ----------------------------------------------- | ---------------------------------- |
| Structured command objects | Claude understands prose; parsing adds overhead | Natural language task descriptions |
| Allowlists/denylists       | Assumes omniscient planning; limits exploration | Role focus + critic guardrails     |
| Mode flags                 | Agents should determine mode from disk state    | Every call is implicit resume      |
| Routing tables in YAML     | Claude can route from prose recommendations     | Handoffs with natural language     |
| Hardcoded file paths       | Brittle; assumes static structure               | Agent discovers paths from context |
| Permission prompts         | Token waste; friction cascade                   | Default-allow work, gate publish   |

### Specific Anti-Patterns

**Don't:** Pass structured routing objects between agents.
**Do:** Write prose handoffs that explain intent.

**Don't:** Create narrow agents for every small task.
**Do:** Group work by context affinity.

**Don't:** Use negative prompts ("don't do X").
**Do:** Use positive prompts ("do Y").

**Don't:** Force completion signals when uncertain.
**Do:** Report PARTIAL with honest blockers.

**Don't:** Have agents estimate metrics.
**Do:** Use tools for mechanical counting; null if unknown.

**Don't:** Route on file reads.
**Do:** Route on agent handoffs.

---

## See Also

- [agent-philosophy.md](agent-philosophy.md) - Agent intelligence and resiliency
- [architecture.md](architecture.md) - System design and patterns
- [why-ops-first.md](why-ops-first.md) - Default-allow philosophy
- [stateless-execution.md](stateless-execution.md) - State transfer between flows
- [ai-physics.md](ai-physics.md) - LLM-specific design constraints
- [contracts.md](../reference/contracts.md) - Handoff patterns and receipt schemas
- [CLAUDE.md](../../CLAUDE.md) - Pack reference
