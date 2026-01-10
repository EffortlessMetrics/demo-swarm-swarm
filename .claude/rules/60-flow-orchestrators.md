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
