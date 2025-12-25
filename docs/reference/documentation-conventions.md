# Documentation Conventions

Conventions for contributing documentation to DemoSwarm. These exist to keep docs accurate as the pack evolves.

---

## Where to Put Things (Diataxis)

| Type | Purpose | Location | Voice |
|------|---------|----------|-------|
| **Tutorial** | Learning by doing | `docs/tutorials/` | "You run... You see..." |
| **How-to** | Task completion | `docs/how-to/` | "To do X, run Y" |
| **Reference** | Lookup (schemas, tables) | `docs/reference/` | Tables, no prose |
| **Explanation** | Understanding "why" | `docs/explanation/` | Factual, no hype |

Don't mix types. A how-to shouldn't explain architecture; link to the explanation instead.

---

## Terminology

Use these terms consistently:

| Term | Meaning |
|------|---------|
| **Swarm mainline** | `origin/main` — Flow 6 merges here |
| **Upstream** | The human repo (`upstream/main`) — export happens after Wisdom |
| **Deploy** | Merge + verify in swarm, not production rollout |
| **Flow 5 (Gate)** | Decides merge vs bounce |
| **Flow 6 (Deploy)** | Executes the merge |

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

| Flow | Name | Purpose |
|------|------|---------|
| 1 | Signal | Intent → contract |
| 2 | Plan | Contract → blueprint |
| 3 | Build | AC-by-AC implementation |
| 4 | Review | Harvest feedback, batch fixes |
| 5 | Gate | Forensic audit → verdict |
| 6 | Deploy | Merge to swarm mainline |
| 7 | Wisdom | Close loops |

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

## See Also

- [CLAUDE.md](../../CLAUDE.md) — Canonical pack reference
- [Maintainers Style Guide](../maintainers/style-guide.md) — Voice guidance for maintainers
