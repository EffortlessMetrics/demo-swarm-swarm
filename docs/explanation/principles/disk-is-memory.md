# Principle: Disk Is Memory

> State lives on disk, not in chat. Every call is an implicit resume.

## The Principle

The filesystem is the system's memory. Chat context is debugging information. When the session ends, disk state is all that survives.

Agents read from `.runs/` to understand what happened. They write to `.runs/` to record what they did. Artifacts are the work product.

**Corollary:** If you cannot recover from a session reset, the system is broken.

---

## Why This Matters

### Sessions Are Ephemeral

Claude sessions end. Context windows fill up. Conversations reset. These are not edge cases — they are the normal operating condition.

| If state lives in chat         | If state lives on disk                    |
| ------------------------------ | ----------------------------------------- |
| Session timeout = work lost    | Session timeout = resume from disk        |
| Context overflow = work lost   | Context overflow = compress and continue  |
| User closes window = work lost | User returns tomorrow = artifacts waiting |

### Context Windows Have Limits

Context accumulates: requirements (50K), design (80K), implementation (200K), review (150K). Carrying all of this forward is expensive and degrades quality.

The solution is not "fit it all in context" — it is "write the important parts to disk and read them back fresh."

### Every Call Is a Resume

When you invoke `/flow-3-build`, the orchestrator does not remember `/flow-2-plan`. It reads artifacts from disk. This enables:

- No telephone game (context drift through paraphrasing)
- No compound token tax (reading artifacts is cheaper than carrying context)
- No hidden state (everything is inspectable)

---

## How Agents Implement This

### Read Before Acting

Every agent starts by checking disk state:

```
Agent starts → Reads .runs/<run-id>/ → Sees what exists → Decides what to do
```

The agent does not ask "what's my assignment?" It looks at the artifacts:

- Does `ac_status.json` exist? What does it say?
- Does `requirements.md` exist? What constraints does it define?
- Does `build_receipt.json` exist? (Flow complete or not?)

### Write to Record

Every agent writes its work to known locations:

| Agent               | Writes                                                |
| ------------------- | ----------------------------------------------------- |
| requirements-author | `signal/requirements.md`, `signal/features/*.feature` |
| adr-author          | `plan/adr.md`                                         |
| code-implementer    | Source files, `build/impl_changes_summary.md`         |
| code-critic         | `build/code_critique.md`                              |
| \*-cleanup          | `*_receipt.json`                                      |

Artifacts persist. Chat history does not.

### Die and Resume

Each agent follows: `Read Fresh -> Do Work -> Write State -> Die`

No shared memory. No context inheritance. Filesystem is the only communication channel.

---

## Violation vs Correct Behavior

### Session Memory (Violation)

```
User: "We were working on the auth feature"
Agent: "Yes, I remember. Let me continue from where we left off..."
```

The agent relies on session memory. After a reset, this fails.

### Disk Memory (Correct)

```
User: "Continue the work"
Agent: [Reads .runs/feat-auth/build/ac_status.json]
       "AC-001 is complete. AC-002 shows 'pending'. Continuing with AC-002."
```

The agent reads disk state. Works identically after a session reset.

### Context-Only Handoff (Violation)

```
Agent A: "Tests pass. Agent B, please deploy."
Agent B: "Okay, deploying based on what Agent A said."
```

Agent B trusted Agent A's claim. No evidence on disk.

### Artifact Handoff (Correct)

```
Agent A: [Writes test_execution.md: "47 passed, 0 failed, exit code 0"]
Agent B: [Reads test_execution.md] "Tests verified. Proceeding with deploy."
```

Agent B verified from the artifact, not from trust.

---

## What This Enables

### Stateless Execution

Flows can run in any order. Each flow reads inputs from disk, does work, writes outputs. No dependency on prior session state.

### Crash Recovery

If a flow crashes mid-execution: some artifacts exist, some are missing. Next run reads what exists, continues from there. No complex checkpoint logic. Just files.

### Auditability

Every decision is on disk. You can:

- **Review it** — open the file
- **Audit it** — git blame, git log
- **Reproduce it** — rerun from the same inputs

### Collaboration

Multiple humans and agents can work on the same run. Human edits `requirements.md` after Flow 1. Flow 2 reads the edited version. No coordination protocol needed.

---

## The Coordination Connection

Agents coordinate through artifacts, not messages:

```
Agent A writes src/auth.rs, impl_changes_summary.md
Orchestrator routes to Agent B
Agent B reads impl_changes_summary.md, src/auth.rs
Agent B writes tests/auth_test.rs
```

The filesystem is the coordination layer. No message broker. No shared memory. Just files appearing in known locations.

See: [Coordination by Artifact](../coordination-by-artifact.md)

---

## Summary

**The problem:** Chat sessions are ephemeral. Context windows are limited. Memory is unreliable.

**The solution:** Write state to disk. Read state from disk. Every call is an implicit resume.

**The test:** If you cannot recover from a session reset, the system is broken.

---

## See Also

- [Coordination by Artifact](../coordination-by-artifact.md) — How agents coordinate through disk state
- [Stateless Execution](../stateless-execution.md) — Why each flow is a fresh context window
- [Run State](../../reference/run-state.md) — Directory structure and schemas
- [Laws of the Swarm](../laws-of-the-swarm.md) — Law 1 and the other nine laws
