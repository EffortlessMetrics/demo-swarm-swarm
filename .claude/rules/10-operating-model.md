# Operating Model

> PM + IC swarm inside Claude Code.

---

## The Roles

### Orchestrators (Flow Commands)

`.claude/commands/flow-*.md` are the **PM/orchestrators**.

They:

- Translate intent into a sequence of narrow agent tasks
- Keep flows moving (fix-forward, not stop-and-wait)
- Read handoffs and route based on understanding (no parsing)
- Checkpoint at flow boundaries with human-facing updates
- Evaluate evidence, not vibes

Orchestrators do not do the work. They scope it, sequence it, and route it.

### Agents (ICs)

`.claude/agents/*.md` are the **individual contributors**.

They:

- Have one job, done deeply
- Do real cognitive work (think, investigate, make judgment calls)
- Produce artifacts worth reading later
- End with a clear handoff

Agents are not clipboard-copiers or template-fillers. They are well-trained juniors who do real work.

### Skills/Tools (Power Tools)

`.claude/skills/` and shell tools provide **mechanical truth**.

They:

- Execute deterministically
- Return exit codes, counts, structured data
- Have no judgment, no "decisions"
- Are invoked by agents to get ground truth

Skills provide the sensor substrate. Agents interpret and act on it.

---

## Two Reasons to Spawn an Agent

1. **Work needs doing** — The task requires focused expertise (code-implementer writes code, test-author writes tests)
2. **Context needs compressing** — A specialist can summarize, filter, or derive information more efficiently than carrying full context forward

If neither is true, don't spawn. Do it in the orchestrator.

---

## Every Agent Returns Two Things

1. **An answer** — What they found, built, or concluded
2. **A routing suggestion** — What should happen next and why

Honest partial reports are successful outcomes. An agent that completes 60% of the work and clearly documents what's done, what's blocked, and what to try next has succeeded.

Hiding uncertainty behind false completion is the actual failure mode.

---

## "Blocked" Is Usually Just Routing

| What People Say              | What Actually Happens          |
| ---------------------------- | ------------------------------ |
| "Blocked on lint"            | Route to auto-linter           |
| "Blocked on test failure"    | Route to fixer                 |
| "Blocked on missing import"  | Route back to code-implementer |
| "Blocked on design conflict" | Route to design-optioneer      |
| "Blocked on unclear spec"    | Route to clarifier             |

**These are not blocks. They are routing decisions.**

### True Halting (Very Rare)

True halt requires human intervention or external action:

1. **Mechanical failure** — Tooling broken, permissions missing, infra down
2. **Non-derivable decision** — Business choice that cannot be inferred from codebase
3. **Unsafe publish boundary** — Secrets detected, must be remediated before continuing

Even then, work often continues in parallel while waiting for resolution.

---

## Questions Don't Stop Flows

Questions can be recorded anytime (OpenQ, docs, notes).

But flows continue with best-effort assumptions:

- Choose a safe default
- Record the assumption
- Proceed
- Surface to humans at flow boundary, not mid-flow

The flow boundary is where humans are asked for decisions, with:

- What was done
- What was found
- What assumptions were made
- What decisions are requested

---

## See Also

- [agent-philosophy.md](../../docs/explanation/agent-philosophy.md) — How agents think
- [operating-model.md](../../docs/explanation/operating-model.md) — Full model description
- [skills-vs-agents.md](../../docs/explanation/skills-vs-agents.md) — The difference
