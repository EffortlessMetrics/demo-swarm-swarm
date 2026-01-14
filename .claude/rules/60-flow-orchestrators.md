---
globs:
  - .claude/commands/flow-*.md
---

# Flow Orchestrators

> How orchestrators scope, route, and checkpoint.

---

## The Orchestrator Role

Flow commands (`.claude/commands/flow-*.md`) are **PM/orchestrators**.

They:
- Translate intent into a sequence of narrow agent tasks
- Keep flows moving (fix-forward, not stop-and-wait)
- Read handoffs and route based on understanding
- Checkpoint at flow boundaries with human-facing updates
- Evaluate evidence, not vibes

Orchestrators **do not do the work**. They scope it, sequence it, route it.

---

## Routing by Understanding

**Law 2: Prose Routes Work**

The orchestrator reads handoffs and decides. No parsing. No structured routing blocks. Claude understands language.

| How It Works | How It Doesn't Work |
|--------------|---------------------|
| Agent says "recommend routing to fixer" → orchestrator routes to fixer | Parse `{ "next_agent": "fixer" }` from JSON block |
| Agent explains reasoning → orchestrator makes informed decision | Follow rigid routing rules regardless of context |

The communication channel is natural language throughout.

---

## Keep Flows Moving

**Law 5: Fix Forward by Default**

"Blocked" is almost always just routing to another agent. Keep the flow moving.

### The Fix-Forward Pattern

```
Problem detected → Route to specialist → Get result → Continue
```

Not:
```
Problem detected → Stop → Wait for human → Resume
```

### When to Actually Stop

True halt is rare:
- Mechanical failure (tooling broken, permissions missing)
- Non-derivable decision (human must choose business direction)
- Unsafe boundary (secrets detected, must remediate)

Even then, scope the halt narrowly. Other work may continue.

---

## Flow Boundary Checkpoints

At the end of each flow, provide:

### Progress Update
What was done in this flow. Summary, not dump.

### Findings Summary
Key results, evidence pointers, quality events.

### Assumptions + Open Questions
What was assumed and why. What needs human input.

### Decision Requests
If human input is needed, ask clearly:
- What decision is needed
- What options exist
- What you recommend and why

### Next-Flow Preview
What the next flow will do. What it needs.

### Artifact Links
Pointers to receipts, evidence, cockpit surfaces.

---

## The PR as Primary UI

The PR description is what most reviewers will read. Treat it as the cockpit display.

### Prefer Outputs That Make Review Fast

| Good | Bad |
|------|-----|
| Short summary tables | Long prose paragraphs |
| Mermaid diagrams (where helpful) | ASCII art |
| Links to evidence artifacts | Inline raw output |
| Hotspot pointers | Comprehensive file lists |
| Explicit "not measured" | Silent gaps |

### Compression Is Kindness

Every unnecessary line increases DevLT. Compress ruthlessly while keeping all necessary information.

---

## No Mid-Flow Stoppage

Questions can be logged anytime. But flows complete the phase and surface questions at the boundary.

### The Pattern

```
Encounter uncertainty → Record assumption → Proceed → Ask at boundary
```

Not:
```
Encounter uncertainty → Stop → Ask user → Wait → Resume
```

Humans are asked at flow boundaries, with context, with options, with recommendations.

---

## Completion Discipline

**Flows run to completion. They never stop mid-execution.**

A flow ends with one of two statuses:

| Status | When | What It Means |
|--------|------|---------------|
| **VERIFIED** | Evidence says done | Panel green, evidence fresh, blockers empty |
| **UNVERIFIED** | External constraint hit | Artifacts written, state captured, resumable |

Everything else is "keep grinding."

### No Early Exit

Orchestrators do not accept "DONE" prose as completion. Completion requires:
- **Evidence panel green** (all required sensors pass), OR
- **External constraint** forces checkpoint (complete UNVERIFIED with honest state)

"Feels done" is not done. "Agent said done" is not done. **Evidence says done.**

**Any agent claiming completion without producing evidence is automatically treated as UNKNOWN and routed to the evidence producer** (test-runner, linter, mutation, etc.). This is the stop-hook that prevents early exit.

### The Loop Body

The microloop that produces convergence:

```
Author → Critic → Fixer → Verifier → [if not converged] → repeat
```

For each station:
1. Run the producer (author, implementer, etc.)
2. Run the critic (attacks the output)
3. If critic finds issues → run fixer → run verifier
4. Repeat until critic says "proceed"

### Two-Pass Minimum for Stability

When fixing an issue, **re-review at least once** to confirm stability. "Two passes" is a minimum observation window, not a maximum retry count.

| Good | Bad |
|------|-----|
| Fix → re-review → stable → proceed | Fix → assume stable → proceed |
| Critic found nothing twice → proceed | Critic found nothing once → proceed |

If re-review finds the same issue, you're not stable. Route to a different approach or escalate.

### Routing to Unstick

**Counts are not conditions. Signal is.**

"We've run 3 times" → run it again. A count alone justifies nothing.

**Stagnation** (same failure, no new signal) → orchestrator routes to unstick. Try a different agent, change the approach, get new signal. This is normal routing, not a special mechanism.

**Oscillation** (toggling between states) → orchestrator breaks the cycle. Route to a different specialist, reframe the problem, get out of the loop.

The orchestrator's job is to keep things moving. When progress stalls, route to unstick. That's orchestration.

| Wrong | Right |
|-------|-------|
| "3 tries, moving on" | "3 tries, running again" |
| "Stagnation detected, stopping" | "Stagnation detected, routing to different agent" |
| "Max iterations, proceeding as done" | "Still not converged, change approach" |
| "Timeout, assuming success" | "External constraint hit, checkpointing UNVERIFIED" |

### "Clean" Means Panel Clean

"Clean" is defined by the evidence panel, not by any single sensor:

- **Intent fidelity**: REQ/BDD/ADR satisfied (or explicit DEFAULTED/NEEDS_HUMAN logged)
- **Verification depth**: Tests + mutation (where required)
- **Maintainability**: Deltas acceptable / hotspots checked
- **Boundaries**: Stage→sanitize→persist, secrets scan on staged surface
- **Explainability**: Cockpit + pointers
- **Freshness**: Evidence SHA matches HEAD

If any of these are red/unknown AND it matters for this change, you're not clean.

### Checkpoint Report

When an external constraint forces UNVERIFIED completion, document:

```markdown
## Checkpoint

**Constraint:** <budget | access | authority>
**Detail:** <what specifically: tokens exhausted, tooling broken, decision needed>

**Current state:**
- <what's done>
- <what's not done>

**Evidence (if any):**
- <artifact path>: <what it shows>

**Recommended next step:** <what to do when constraint clears>
```

**Checkpoint means: save state, publish artifacts, recommend next route. Work continues when constraint clears.**

This makes the flow resumable. External constraints are checkpoints, not failures. UNVERIFIED is not failure—it's unmerged state.

---

## Local Resolution Before Bouncing

**Law 7: Local Resolution First**

Before bouncing to a previous flow:
- Try 2-3 targeted specialist calls
- Route to design-optioneer, adr-author, or impact-analyzer
- Re-plan locally
- Resume

Bounce only when specialists agree the issue cannot be resolved locally.

### The Economics

```
Microloop: ~10 minutes, focused context
Flow bounce: ~60+ minutes, full context rebuild
```

Try local resolution first. Bounce only when truly necessary.

---

## See Also

- [flow-composition.md](../../docs/explanation/flow-composition.md) — How flows compose
- [operating-model.md](../../docs/explanation/operating-model.md) — Full operating model
- [laws-of-the-swarm.md](../../docs/explanation/laws-of-the-swarm.md) — The immutable rules
- [90-voice-and-tone.md](90-voice-and-tone.md) — How to communicate findings
