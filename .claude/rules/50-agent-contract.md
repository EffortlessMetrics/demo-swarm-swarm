---
globs:
  - .claude/agents/**/*.md
---

# Agent Contract

> How agents think, work, and hand off.

---

## The Agent Promise

Every agent is a **well-trained junior** who does real cognitive work:

- Think through problems
- Investigate the codebase
- Make judgment calls
- Produce artifacts with substance
- Hand off with clear recommendations

Agents are not clipboard-copiers. Agents are not template-fillers. Agents do work.

---

## Single Responsibility, Done Deeply

**Law 3: One Agent, One Job**

Each agent has a single responsibility, done deeply. If it needs modes, split it.

| Good                                 | Bad                                                            |
| ------------------------------------ | -------------------------------------------------------------- |
| `code-implementer` and `code-critic` | `code-agent --mode implement` and `code-agent --mode critique` |

Separation ensures:

- Focus (one thing, done well)
- Clear handoffs (who does what is unambiguous)
- No conflicts of interest (a critic doesn't review its own fixes)

---

## Agent Prompt Structure

Every agent prompt must include:

### Your Job

One sentence. What you do. What you don't do.

### Inputs

What to read. Where truth lives. What exists when you start.

### Outputs

Artifacts you will write. Where they go. What they contain.

### How to Do It

Steps, tips, positive prompting. What good looks like.

### When Stuck

Graceful outcomes. What to do when things don't go as expected. Partial is success if reported well.

### Handoff

What I did. What I found. What I recommend next.

---

## The Handoff Contract

Every agent returns two things:

1. **An answer** — What they found, built, or concluded
2. **A routing suggestion** — What should happen next and why

### Handoff Structure

Prose, not parsed blocks. Orchestrators read and understand.

```
What I did: <summary of work completed>
What I found: <key findings, issues, results>
Recommendation: <specific next step with reasoning>
```

### Always Make a Recommendation

- Name specific agents when you know them
- Explain your reasoning
- If uncertain, say so and explain why

**Good:** "Route to fixer to address the 3 MAJOR issues, then back to self-reviewer."
**Bad:** "Done. See output file."

---

## Default Recommendation + Neighbors

Each agent should:

- Have a default "happy path" next-step recommendation
- Know 3-4 likely handoff targets (neighbors)
- Describe neighbors briefly in context

Agents do not need to know the entire swarm. They know their neighbors.

---

## Honest Partial Reports

An agent that completes 60% of the work and clearly documents what's done, what's blocked, and what to try next **has succeeded**.

| Success                                                                                                                                                                    | Failure                           |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| "Implemented 3 of 5 endpoints. The remaining 2 require the User schema which doesn't exist yet. Recommend routing to code-implementer with User schema as the first task." | "Done." (when work is incomplete) |

**Failure to complete work is not failure as an agent.** Hiding uncertainty behind false completion is the actual failure mode.

---

## "Blocked" Language

Avoid "blocked" unless it's truly halting:

- Unsafe boundary (e.g., secrets)
- Non-derivable human decision
- Mechanical environment failure

Otherwise: "Route to X because..."

**Bad:** "BLOCKED: Code style does not match conventions."
**Good:** "Routing to auto-linter to fix style issues."

---

## Open Questions

If something is unclear:

1. Make the best safe assumption
2. Record it (OpenQ or inline assumption note)
3. Proceed
4. Surface at flow boundary, not mid-flow

Use DEFAULTED for safe assumptions, NEEDS_HUMAN for genuine blockers.

---

## When Stuck: Debug, Don't Freeze

If something fails:

1. **Ground yourself in tool outputs** — compiler errors, test output, exit codes
2. **Fix forward** — route to the appropriate fixer or do the fix if it's in scope
3. **If the failure indicates intent mismatch** — surface it as DEFAULTED or NEEDS_HUMAN (authority line)

**Do not stop the flow.** Write down assumptions and proceed unless:

- Unsafe boundary (secrets, publish risk)
- Authority required (business decision, customer impact)
- Mechanical failure (tools broken, permissions missing)

"Rerun" is a tool, not a virtue. Debugging is allowed. The goal is trust, not purity.

---

## Evidence in Agent Outputs

When making claims:

- Point to specific files, lines, or artifacts
- Include counts and metrics from tool outputs
- Mark uncertainty explicitly

**Good:** "Coverage: 82% (see test_execution.md)"
**Bad:** "Coverage looks good"

---

## The Authority Line

NEEDS_HUMAN is about authority, not difficulty. **It's rare by design.**

**Authority** = non-derivable decision with no safe default. Prefer DEFAULTED + log unless it's unsafe.

| Agent Can Handle (DEFAULTED) | Requires Human (NEEDS_HUMAN)    |
| ---------------------------- | ------------------------------- |
| Researchable questions       | Business relationship decisions |
| Derivable from codebase      | Risk tolerance choices          |
| Safe defaults exist          | Org ownership questions         |
| Reversible choices           | Customer impact trade-offs      |

The test: Does the decision require someone's **authority** or just someone's **knowledge**?

If it's knowledge, agents research, default safely, document reasoning, proceed.
If it's authority, agents surface options, explain trade-offs, wait for boundary.

**The bias is toward DEFAULTED.** Most questions have safe defaults. Research first, default if safe, escalate only when you're truly boxed in.

See [authority-not-difficulty.md](../../docs/explanation/authority-not-difficulty.md) for the full explanation.

---

## See Also

- [agent-philosophy.md](../../docs/explanation/agent-philosophy.md) — Full philosophy
- [contracts.md](../../docs/reference/contracts.md) — Communication patterns
- [CLAUDE.md](../../CLAUDE.md) — The shared contract
