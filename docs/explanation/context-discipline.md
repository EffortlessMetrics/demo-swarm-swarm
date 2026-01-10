# Context Discipline

> The pattern of managing context to reduce hallucination and improve reliability.

This is not about Claude Code's context window limits. It's about the architectural decision to **starve agents of unnecessary context**.

---

## The Problem: Context Rot

LLM performance degrades non-linearly with context length. More context does not mean better understanding—it means more noise to hallucinate from.

### Non-Linear Degradation

```
Context Size    Effective Attention
   10K tokens    High precision, focused work
   50K tokens    Some drift, but manageable
  100K tokens    Attention spreading, details blur
  200K tokens    Competing signals, hallucination risk spikes
```

The relationship isn't linear. Doubling context doesn't halve precision—it can collapse it entirely. An agent with 200K tokens of "helpful context" performs worse than one with 10K tokens of the right context.

### Noise to Hallucinate From

Every piece of context is a potential source of confusion:
- Old requirements that were superseded
- Similar but different code patterns
- Tangential discussions that feel relevant
- Prior agent outputs that contained assumptions

LLMs don't distinguish between "important context" and "background noise." They weight everything by attention, and attention is easily captured by irrelevant details.

### Long Conversations Drift

Conversations accumulate baggage:
```
Turn 1: "Add OAuth login"
Turn 10: "...so we're using OAuth with Google and GitHub..."
Turn 20: "...the OAuth providers including Apple and Microsoft..."
Turn 30: "...the multi-provider OAuth system with SSO..."
```

Intent gets confused. Details accrete that were never requested. By turn 30, the agent is solving a different problem than turn 1 defined.

### Cross-Contamination

When agents see multiple unrelated tasks in the same context:
- Patterns from Task A leak into Task B
- Naming conventions get confused
- Architectural decisions from one feature affect another
- Testing approaches blend inappropriately

Unrelated context is not neutral—it's actively harmful.

---

## The Solution: Context Discipline

### 1. Disk as Memory

**State lives on disk, not in chat.**

The conversation is a console; the repo is the work product.

```
WRONG:
  Agent A → tells Agent B in chat → Agent B remembers

RIGHT:
  Agent A → writes to disk → Agent B reads from disk
```

This is the "sealed stations" pattern from [ai-physics.md](ai-physics.md). Each agent:
1. Reads from disk (fresh start)
2. Does work
3. Writes to disk
4. Dies

The next agent has no memory of the previous agent's conversation. It only sees what was written to disk.

**Every call is implicitly a resume:**
- Check what exists
- Update what's missing
- Report what changed

If an agent crashes or times out, the next invocation picks up from the last checkpoint on disk—not from a corrupted conversation history.

### 2. Targeted Hydration

**Instead of "here's the whole repo":**
- Load only what's needed for the current task
- Prefer narrow manifests over broad context
- Explain expansion when more is needed (not ask permission)

Think of it as **handing a junior dev 3 files and a mission, not the whole codebase and good luck.**

A senior engineer giving a task to a junior doesn't say "here's our 500-file monorepo, figure out what you need." They say "here's the auth module, the user model, and the test file—add email verification."

The context-loader pattern:
1. **Curator identifies the minimal file set** (typically 3-5 files)
2. **Worker receives only the manifest + required artifacts**
3. **Worker has no memory of previous steps**

The worker can expand beyond the manifest if genuinely needed—but they start focused. This is an accelerator, not a gate.

### 3. Short Threads

**Avoid accumulating drift across many turns.**

Long conversations are toxic to precision:
- Early context gets diluted by later turns
- Corrections and clarifications pile up
- The agent "remembers" things from 50 turns ago that weren't said

The remedy:
- **Checkpoint to disk frequently** (after each meaningful unit of work)
- **New station = fresh start from artifacts**
- **Treat thread death as a feature**, not a failure

When you spawn a new agent, you're not "losing context"—you're resetting context entropy to zero. The new agent sees only the compressed, curated output from the previous station.

---

## This is an Accelerator, Not a Gate

The goal is **focus**, not **restriction**.

### The Pattern

1. **Start with small context** (recommended manifest from curator)
2. **Expand when genuinely needed** (search, read, explore)
3. **Explain why expansion was necessary** (not ask permission)

### What This Is Not

This is not about creating:
- Allowlists of permitted files
- Permission systems for reading context
- Approval workflows for accessing information

Agents are intelligent. They can determine what they need. The discipline is about **where they start**, not where they're allowed to go.

### The Autonomy Balance

```
WRONG:
  Agent: "May I read utils.ts?"
  Orchestrator: "Yes, you may."
  Agent: "May I also read config.ts?"
  Orchestrator: "Yes, you may."
  (Token-burning permission loops)

RIGHT:
  Agent: "Manifest gave me 3 files. Needed utils.ts for helper patterns.
          Expanded to config.ts for environment handling. Implemented the feature."
  (Explain expansion, don't ask permission)
```

Critics evaluate quality afterward. That's the guardrail—not preventative restrictions on what agents can touch.

---

## Practical Effects

### Hallucination Drops

With less noise in context, there's less material to confuse the model:
- Fewer irrelevant patterns to blend in
- Fewer superseded requirements to resurrect
- Fewer tangential discussions to misinterpret

The agent sees what it needs and nothing else. Precision increases.

### Agents Behave Consistently

When every invocation starts from the same artifacts:
- Same inputs produce same outputs
- Behavior doesn't drift based on conversation history
- Testing and debugging become tractable

You can rerun a flow and expect similar results because the inputs are stable.

### Resumption Works

State on disk means:
- Agent crashes don't lose progress
- Timeout recovery is automatic
- Partial work is preserved
- Multiple attempts compound rather than restart

The conversation can die, but the work survives.

### Context Budget Stays Healthy

By not stuffing context with "might be useful" material:
- More room for the actual task
- Better attention on what matters
- Faster response times
- Lower token costs

---

## Implementation

### For Orchestrators

1. **Spawn agents with focused manifests**, not entire repo context
2. **Let agents expand** if they explain why
3. **Trust artifacts**, not conversation memory
4. **Use cleanup agents** to compress flow outputs before the next flow

### For Agents

1. **Start from the manifest** (the curator chose those files for a reason)
2. **Explore when blocked** (search and read, don't guess)
3. **Document expansion** (mention what you read beyond the manifest)
4. **Write state to disk** (don't rely on the orchestrator remembering)

### For Flow Design

1. **Short flows** with clear inputs and outputs
2. **Receipts as compression** (summarize, don't pass through)
3. **Checkpoint frequently** (disk is durable, chat is not)
4. **Fresh context per station** (reset entropy at each handoff)

---

## The Metaphor

**Handing a junior dev 3 files and a mission, not the whole codebase and good luck.**

The senior engineer (orchestrator) has read the codebase. They know what's relevant. They give the junior (agent) exactly what's needed to do the job, with permission to explore further if stuck.

The junior doesn't need to understand the whole system. They need to understand their task. Context discipline makes this possible at scale—across many agents, many flows, many tasks.

---

## See Also

- [ai-physics.md](ai-physics.md) — Sealed stations, compressors, context affinity
- [stateless-execution.md](stateless-execution.md) — Why each flow is a fresh context window
- [agent-philosophy.md](agent-philosophy.md) — How agents think and act
- [architecture.md](architecture.md) — System design principles
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
