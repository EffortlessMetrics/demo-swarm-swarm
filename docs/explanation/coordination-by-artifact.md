# Coordination by Artifact

> Agents don't talk to each other. They talk to disk. The orchestrator reads the results and decides what's next.

---

## The Coordination Problem

Multiple agents working on related tasks could:

- Overwrite each other's work
- Duplicate effort
- Make conflicting decisions
- Lose track of state

Traditional distributed systems solve this with:

- **Locks** — Acquire before acting, release after
- **Queues** — Serialize operations through a broker
- **Message passing** — Agents coordinate via protocols
- **Shared memory** — Concurrent access to data structures

These work, but they're complex. They require connection management, failure handling, protocol negotiation, and state synchronization.

**Swarm solution: Coordination by artifact.**

Agents coordinate through disk state. No messages between agents. No shared memory. No locks.

---

## The Pattern

Agents coordinate through disk state:

- Read what exists before starting
- Write results to known locations
- Let artifacts define boundaries
- Trust the filesystem as arbiter

The filesystem provides everything coordination needs:

- **Atomicity** — A write completes or doesn't
- **Visibility** — Everyone can read
- **Durability** — State survives restarts
- **Ordering** — Modification times establish sequence
- **Naming** — Paths serve as identifiers

It's simple, reliable, and universal.

---

## How It Works

### 1. Implicit Resume

Every agent starts by reading disk state:

- What artifacts exist?
- What work is complete?
- What's pending?

```
Agent starts → Reads .runs/ → Sees AC-001 done, AC-002 pending → Works on AC-002
```

No one tells the agent what to do. The state tells the agent what to do.

The agent doesn't ask "what's my assignment?" It checks:

- Does `ac_status.json` exist? What does it say?
- Does `build_receipt.json` exist? (Flow complete or not?)
- What artifacts are present in the run directory?

State IS the disk contents. There's no separate "mode" or "assignment" — just files.

### 2. Artifact Boundaries

Each agent owns specific artifacts:

| Agent            | Owns                                    |
| ---------------- | --------------------------------------- |
| code-implementer | Source files, `impl_changes_summary.md` |
| test-author      | Test files, `test_strategy.md`          |
| code-critic      | `code_critique.md`                      |
| test-critic      | `test_critique.md`                      |
| \*-cleanup       | `*_receipt.json`                        |
| repo-operator    | Git operations, staging                 |

Agents don't touch each other's artifacts. A critic never modifies source code. An implementer never writes receipts. This creates natural separation — no collisions possible.

### 3. Handoff via Completion

When an agent finishes:

1. Writes its artifacts to known locations
2. Reports what was done (natural language)
3. Recommends what should happen next

The next agent reads those artifacts and continues. There's no handshake, no acknowledgment protocol — just files appearing on disk.

```
code-implementer writes src/auth.rs
code-implementer writes impl_changes_summary.md
code-implementer reports: "Implemented login flow. Ready for test-author."

test-author reads impl_changes_summary.md
test-author reads src/auth.rs
test-author writes tests/auth_test.rs
test-author reports: "Tests written. Ready for test-executor."
```

Each agent picks up where the last left off.

### 4. The Orchestrator Routes

The orchestrator (Claude) reads handoffs and decides:

- Which agent runs next?
- Is work complete?
- Are there blockers?

Routing is by understanding, not by message protocol. The orchestrator reads natural language responses, not structured control blocks.

---

## Coordination Patterns

### The Handoff Chain

Sequential work flows through artifact handoffs:

```
Agent A writes artifact → Agent B reads artifact → Agent B writes → Agent C reads
```

### Example: Build flow

```
requirements-author → requirements.md
    ↓
test-author reads requirements.md → test files
    ↓
code-implementer reads requirements.md, tests → source files
    ↓
code-critic reads source files → code_critique.md
    ↓
code-implementer reads critique → improved source files
```

Each agent picks up where the last left off. No coordination messages needed.

### The Critic Loop

Author-critic iteration through a worklist artifact:

```
Author writes code → Critic reads code → Critic writes issues → Author reads issues → Author fixes
```

The worklist artifact (`code_critique.md`) is the coordination point. The author doesn't wait for a "please fix" message — they read the worklist and see what needs fixing.

### The Status Ledger

Shared state through a known file:

```
Worker updates ac_status.json → Orchestrator reads ac_status.json → Routes next work
```

The status file is the coordination point. Workers update their section. The orchestrator reads the whole picture.

### The Cleanup Seal

Cleanup agents verify and seal:

```
Workers write artifacts → Cleanup reads everything → Cleanup writes receipt
```

The receipt is the coordination point. Downstream flows check "does receipt exist?" to know if upstream is complete.

---

## Why This Works

### No Race Conditions

Agents run sequentially (one at a time in most flows). No concurrent writes to the same artifact. Even when agents could theoretically run in parallel, artifact ownership prevents conflicts.

### No Lost State

State is on disk. Session resets don't lose work. Context exhaustion doesn't lose progress. Artifacts survive any interruption.

Compare to message-based coordination:

- Message in flight when agent crashes? Lost.
- Shared memory when process dies? Lost.
- Disk file when session resets? Still there.

### No Coordination Overhead

No need to:

- Establish connections between agents
- Negotiate protocols or versions
- Handle message delivery failures
- Manage shared memory access
- Implement leader election

Just read and write files. The filesystem handles the rest.

### Natural Boundaries

File ownership creates natural separation:

- **Clear responsibility** — Each agent knows exactly what it writes
- **No stepping on toes** — Agents can't accidentally overwrite each other
- **Easy debugging** — Check who owns the file, check what they wrote

---

## Anti-Patterns

### Message Passing Between Agents

**Wrong:** Agents send messages to each other

```
code-implementer → message → code-critic: "Please review auth.rs"
code-critic → message → code-implementer: "Found 3 issues"
```

**Right:** Agents write artifacts, orchestrator routes

```
code-implementer writes src/auth.rs
orchestrator calls code-critic
code-critic reads src/auth.rs, writes code_critique.md
orchestrator calls code-implementer with critique context
```

### Shared Memory Access

**Wrong:** Agents access shared data structure

```
shared_state["ac_001"] = "in_progress"
other_agent reads shared_state["ac_001"]
```

**Right:** Agents access files with clear ownership

```
code-implementer updates ac_status.json (its section)
cleanup reads ac_status.json (everyone's sections)
```

### Implicit Dependencies

**Wrong:** Agent B assumes Agent A ran

```
# code-implementer assumes requirements.md exists
requirements = read("requirements.md")  # crashes if missing
```

**Right:** Agent B checks if A's artifacts exist

```
# code-implementer checks for required input
if not exists("requirements.md"):
    report("Cannot proceed: requirements.md missing")
    return
requirements = read("requirements.md")
```

### Direct Agent-to-Agent Calls

**Wrong:** One agent spawns another

```
code-implementer spawns test-author
```

**Right:** Orchestrator routes to next agent

```
code-implementer reports: "Implementation complete. Recommend test-author."
orchestrator calls test-author
```

---

## The Filesystem as Coordination Layer

The filesystem provides everything distributed coordination needs:

| Need       | Filesystem Provides                |
| ---------- | ---------------------------------- |
| Atomicity  | Write completes or doesn't         |
| Visibility | Any process can read any file      |
| Durability | Files survive process death        |
| Ordering   | Modification times, creation order |
| Identity   | Paths as unique identifiers        |
| Isolation  | Directory structure as namespacing |

No message broker. No consensus protocol. No distributed lock manager. Just files.

### Run Directories as Namespaces

```
.runs/
├── feat-auth/           # One run's namespace
│   ├── signal/
│   │   └── requirements.md
│   ├── plan/
│   │   └── adr.md
│   └── build/
│       ├── ac_status.json
│       └── impl_changes_summary.md
└── feat-search/         # Another run's namespace
    └── ...
```

Each run has isolated state. Agents working on different runs can't interfere. Agents working on the same run coordinate through shared files.

---

## Scaling

This pattern scales because:

**No central coordinator bottleneck.** There's no message broker, no leader, no single point of failure. Each agent reads and writes independently.

**No message broker to manage.** Nothing to deploy, scale, or maintain. The filesystem is already there.

**State is always recoverable.** Crash recovery is trivial — read the files, continue from where they indicate.

**New agents just need to know artifact locations.** Adding a new agent type doesn't require protocol changes. Just document what files it reads and writes.

### Horizontal Extension

Adding a new flow or agent:

1. Define what artifacts it reads (inputs)
2. Define what artifacts it writes (outputs)
3. Document artifact ownership
4. Add to the routing table

No coordination protocol changes. No version negotiation. Just files.

---

## Implementation

Coordination by artifact is implemented through:

### 1. Directory Structure (Known Locations)

`.runs/<run-id>/<flow>/` provides predictable paths. Agents know where to look.

### 2. Artifact Schemas (Known Formats)

`ac_status.json`, `*_receipt.json`, etc. have documented schemas. Agents know how to parse what they find.

### 3. Agent Prompts (Read-Before-Write)

Agent prompts include instructions to check disk state first:

```markdown
## On Start

Check if your tracking artifacts exist.
If partially complete, resume from there.
```

### 4. Orchestrator Routing (Handoff-Based)

Orchestrators route on agent responses, not by parsing files. Agents report what happened; orchestrators decide what's next.

---

## The Contract

Each agent has an implicit contract:

**Inputs:** What artifacts must exist before I can run
**Outputs:** What artifacts I produce
**Ownership:** What files only I can modify
**Resume:** How I detect and continue partial work

This contract is documented in agent prompts and understood by orchestrators. No formal protocol negotiation — just shared understanding of file locations and formats.

---

## Comparison: Traditional vs Artifact Coordination

| Aspect           | Traditional (Messages)     | Artifact Coordination |
| ---------------- | -------------------------- | --------------------- |
| State location   | In flight, in memory       | On disk               |
| Failure recovery | Complex (replay, acks)     | Trivial (read files)  |
| Debugging        | Trace logs, message dumps  | Read files directly   |
| Adding agents    | Protocol updates           | Document file paths   |
| Scaling          | Broker capacity            | Filesystem capacity   |
| Complexity       | High (protocols, failures) | Low (files)           |

The tradeoff: Artifact coordination is simpler but requires sequential execution within a flow. For swarm workflows, this is the right tradeoff — we want simplicity and recoverability over parallelism.

---

## Summary

**The problem:** Multiple agents could collide, duplicate, or lose state.

**The solution:** Coordinate through disk state, not messages.

**The mechanism:**

1. Read disk to understand current state
2. Write to owned artifacts
3. Report completion to orchestrator
4. Next agent reads and continues

**The principle:** Agents don't talk to each other. They talk to disk. The orchestrator reads the results and decides what's next.

---

## See Also

- [state-and-resumption.md](state-and-resumption.md) — How disk state enables resume
- [architecture.md](architecture.md) — The seven architecture laws
- [worklist-pattern.md](worklist-pattern.md) — How critics coordinate through queues
- [agent-philosophy.md](agent-philosophy.md) — Agent roles and responsibilities
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
