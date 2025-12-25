# Maintainers Style Guide

Voice and reasoning behind DemoSwarm documentation standards.

For enforceable conventions (terminology, Diataxis, drift rules), see [Documentation Conventions](../reference/documentation-conventions.md).

---

## The Voice

**One sentence:** Write like an incident report or ADR — factual, scoped, reproducible.

The goal is operator-grade documentation: a reader should be able to follow it without decoding rhetoric or inferring context.

### Why this voice?

The pack trades machine iteration for human verification time. Docs should do the same: spend words on precision, not persuasion.

- **Constraint-first:** Open with the problem, not the philosophy
- **Evidence-centered:** Point to specific file paths
- **Precise claims:** "may block" beats "blocks"
- **Positive framing:** "Prefer X" beats "You are forbidden from Y"

---

## Economic Framing

The economic thesis is core but should appear **once per entry doc**, not everywhere:

> **Core constraint:** Tokens are cheap; reviewer attention is the bottleneck.

Define it in README and architecture docs. Elsewhere, link to it. Don't expand into manifesto territory.

---

## Anti-Patterns

Patterns that signal the wrong voice:

| Pattern | Examples |
|---------|----------|
| Hype language | "Terrifyingly effective", "revolutionary", "magical" |
| Persona language | "Rogue auditor", "factory foreman" |
| Proof-of-work flexing | Commit count bragging, monthly token spend, magnitude-over-evidence claims |
| Dramatic terminology | "Scars" for friction, "catastrophic" for fixable errors |
| Emojis | Not part of the voice |
| Dialogue scripts | "Start by stating X" — agents derive behavior from context |

### Why these are problems

- **Hype** ages poorly and invites skepticism
- **Personas** require context outsiders don't have
- **Flexing** distracts from utility
- **Drama** makes routine operations sound alarming
- **Emojis** undercut the operator-grade tone
- **Scripts** over-constrain agent behavior and break when context changes

---

## Patterns That Work

| Pattern | Example |
|---------|---------|
| Constraint + mechanism | "Tokens are cheap; reviewer attention is the bottleneck." |
| Evidence surface | "Open `signal_receipt.json` to verify status." |
| Conditional language | "may block", "surfaced", "depending on policy" |
| Rerun guidance | "If the contract is wrong, rerun Flow 1." |
| Positive framing | "Prefer verified pointers (read files before citing)" |

---

## Litmus Tests

Before committing doc changes:

1. **Would this sound weird in an ADR or postmortem?** If yes, cut it.
2. **Does it point to a file path?** If it describes something, it should point to where that thing lives.
3. **Does it handle failure without drama?** "Rerun the flow" not "catastrophic failure requires intervention".
4. **Is there a hardcoded count?** Remove it and point to the source directory.
5. **Does it use absolute claims?** Add "may" or "depending on policy" where appropriate.

---

## Drift Prevention

Values that change over time create maintenance burden and erode trust when stale.

### Hardcoded values to avoid

| Type | Problem | Fix |
|------|---------|-----|
| Prices | API pricing changes | Link to pricing page, use relative ratios |
| Counts | Agent/flow counts change | Point to source directory |
| Costs | Per-run costs vary | Describe as "cheap to rerun" not "$5-10" |
| Percentages | Allocations shift | Use "most" or "minimal" not "96%" |

### Link to sources, don't embed

```markdown
<!-- Bad: will drift -->
Sonnet costs ~$3.00 per million tokens.

<!-- Good: stays current -->
Check [Anthropic pricing](https://www.anthropic.com/pricing) for current rates.
Sonnet is ~10-15x Haiku cost.
```

### Tutorials stay tight

Tutorials are step-by-step. Cut:
- "Why this demo goal?" sections (explanation, not tutorial)
- Verbose verification examples (show one command, not five)
- Architecture context (link to explanation docs)

If you're explaining *why*, you're in explanation territory — link there instead.

---

## Reusable Blocks

Copy-paste patterns for consistency:

```markdown
<!-- Economic anchor (README, architecture docs only) -->
**Core constraint:** Tokens are cheap; reviewer attention is the bottleneck.

<!-- What to skim pattern -->
Then open:
- `.runs/<run-id>/signal/requirements.md` — the contract
- `.runs/<run-id>/signal/open_questions.md` — assumptions needing validation

<!-- Rerun guidance -->
If the contract is wrong, rerun Flow 1. Fixing the spec is cheaper than fixing a bad build.

<!-- Gate language -->
Gates engage at publish boundaries. If a gate blocks, keep working locally.

<!-- Receipt philosophy -->
Receipts are logs, not locks. The git log is the audit trail.
```

---

## README Scope

The root README is **orientation + first success + navigation**.

It should:
1. State the constraint (one line)
2. Show the first command
3. Point to what to open after
4. Link to full docs

It should NOT:
- Explain full architecture (link to explanation docs)
- List all agents (link to agents index)
- Be a manifesto

---

## Agent Prompts

Agent prompts (`.claude/agents/*.md`) follow the same voice principles:

- **Factual, not theatrical:** "You critique. You do not fix." not "You are a harsh auditor who..."
- **Constraint-first:** Open with what the agent does and doesn't do
- **Evidence-centered:** Point to specific file paths
- **No scripts:** Describe behavior, don't prescribe dialogue

For agent structure and mechanics, see [Adding an Agent](../how-to/add-an-agent.md).

---

## See Also

- [Documentation Conventions](../reference/documentation-conventions.md) — Public, enforceable rules
- [Adding an Agent](../how-to/add-an-agent.md) — Agent structure and mechanics
- [CLAUDE.md](../../CLAUDE.md) — Canonical pack reference
