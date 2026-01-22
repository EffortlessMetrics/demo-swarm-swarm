# Documentation Conventions

Conventions for contributing documentation to DemoSwarm. These exist to keep docs accurate as the pack evolves.

---

## Voice

**One sentence:** Write like an incident report or ADR—factual, scoped, reproducible.

The pack trades machine iteration for human verification time. Docs should do the same: spend words on precision, not persuasion.

### Principles

- **Constraint-first:** Open with the problem, not the philosophy
- **Evidence-centered:** Point to specific file paths
- **Precise claims:** "may block" beats "blocks"
- **Positive framing:** "Prefer X" beats "You are forbidden from Y"

### Anti-Patterns

| Pattern                                            | Problem                                 |
| -------------------------------------------------- | --------------------------------------- |
| Hype language ("revolutionary", "magical")         | Ages poorly, invites skepticism         |
| Persona language ("rogue auditor")                 | Requires context outsiders don't have   |
| Proof-of-work flexing (commit counts, token spend) | Distracts from utility                  |
| Dramatic terminology ("catastrophic failure")      | Makes routine operations sound alarming |
| Emojis                                             | Undercuts operator-grade tone           |
| Dialogue scripts ("Start by stating X")            | Over-constrains agent behavior          |

### Litmus Tests

Before committing doc changes:

1. **Would this sound weird in an ADR or postmortem?** If yes, cut it.
2. **Does it point to a file path?** If it describes something, point to where it lives.
3. **Does it handle failure without drama?** "Rerun the flow" not "catastrophic failure".
4. **Is there a hardcoded count?** Remove it and point to the source directory.

---

## Where to Put Things (Diataxis)

| Type            | Purpose                  | Location            | Voice                   |
| --------------- | ------------------------ | ------------------- | ----------------------- |
| **Tutorial**    | Learning by doing        | `docs/tutorials/`   | "You run... You see..." |
| **How-to**      | Task completion          | `docs/how-to/`      | "To do X, run Y"        |
| **Reference**   | Lookup (schemas, tables) | `docs/reference/`   | Tables, no prose        |
| **Explanation** | Understanding "why"      | `docs/explanation/` | Factual, no hype        |

Don't mix types. A how-to shouldn't explain architecture; link to the explanation instead.

---

## Terminology

Use these terms consistently:

| Term                | Meaning                                                        |
| ------------------- | -------------------------------------------------------------- |
| **Swarm mainline**  | `origin/main` — Flow 6 merges here                             |
| **Upstream**        | The human repo (`upstream/main`) — export happens after Wisdom |
| **Deploy**          | Merge + verify in swarm, not production rollout                |
| **Flow 5 (Gate)**   | Decides merge vs bounce                                        |
| **Flow 6 (Deploy)** | Executes the merge                                             |

Common mistake: Flow 5 decides, Flow 6 executes. Don't confuse them.

---

## Evidence and Drift

### Point to specific file paths

```markdown
<!-- Good -->

Open `.runs/<run-id>/signal/requirements.md` to review the contract.

<!-- Bad -->

Check the requirements file in the signal directory.
```

### Use symbols, not line numbers

Line numbers drift. Prefer file paths + function/class names + artifact names.

Exception: quoting compiler or test output that already includes line numbers.

### No hardcoded counts

```markdown
<!-- Good -->

See `.claude/agents/` for the current agent set.

<!-- Bad -->

The pack includes 73 agents across 8 families.
```

### No embedded prices or costs

Link to sources instead of embedding values that drift:

```markdown
<!-- Good -->

Check [Anthropic pricing](https://www.anthropic.com/pricing) for current rates.
Sonnet is ~10-15x Haiku cost.

<!-- Bad -->

Sonnet costs ~$3.00 per million tokens.
Each run costs $5-10 in API calls.
```

Use relative comparisons ("cheap to rerun", "most agents") not absolutes ("$5", "96%").

---

## Claims Discipline

### Policy vs mechanism

- "Policy may block publish" — governance choice
- "Blocked due to prerequisite failure" — mechanical fact

### Avoid absolutes

```markdown
<!-- Good -->

Test deletions may block push depending on policy.

<!-- Bad -->

Test deletions block push.
```

### Don't promise mechanics

Describe outcomes, not implementation details:

- ✓ "surfaced", "gated", "routed", "may block depending on policy"
- ✗ "staged automatically", "physically blocked"

Exception: contract definitions in CLAUDE.md or reference docs.

---

## Flow Reference

| Flow | Name   | Purpose                       |
| ---- | ------ | ----------------------------- |
| 1    | Signal | Intent → contract             |
| 2    | Plan   | Contract → blueprint          |
| 3    | Build  | AC-by-AC implementation       |
| 4    | Review | Harvest feedback, batch fixes |
| 5    | Gate   | Forensic audit → verdict      |
| 6    | Deploy | Merge to swarm mainline       |
| 7    | Wisdom | Close loops                   |

---

## PR Checklist

Before submitting documentation changes:

- [ ] Points to file paths where things live (not vague references)
- [ ] No hardcoded counts or prices (links to sources, uses relative terms)
- [ ] Uses conditional language for policy outcomes ("may block")
- [ ] Placed in correct Diataxis category (tutorials don't explain "why")
- [ ] Uses stable terminology (swarm mainline, upstream, deploy)
- [ ] Would sound reasonable in an ADR or postmortem

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

## Agent Prompts

Agent prompts (`.claude/agents/*.md`) follow the same voice principles:

- **Factual, not theatrical:** "You critique. You do not fix." not "You are a harsh auditor who..."
- **Constraint-first:** Open with what the agent does and doesn't do
- **Evidence-centered:** Point to specific file paths
- **No scripts:** Describe behavior, don't prescribe dialogue

For agent structure and mechanics, see [Adding an Agent](../how-to/add-an-agent.md).

---

## See Also

- [CLAUDE.md](../../CLAUDE.md) — Canonical pack reference
- [Maintainer Handover](../maintainers/handover.md) — Pack maintenance onboarding
