# Teaching Repo

> The best documentation is a working system that teaches while it operates.

---

## The Design Intent

This is not just a working system. It is a **teaching system**. The repo teaches the mental model while you use it.

Every layer reinforces the concepts:

- CLAUDE.md provides the contract
- Agent prompts are executable job descriptions
- Docs explain the why behind the what
- Flows demonstrate the patterns in action

The architecture is intentionally **self-documenting and self-teaching**. You learn by doing, and the system reinforces the learning at every step.

---

## The Teaching Layers

### Layer 1: CLAUDE.md (The Contract)

CLAUDE.md is attached to every agent thread. It contains:

- Core philosophy (one paragraph)
- Key principles (bulleted)
- Flow overview (table)
- Reference links

**Teaching function:** Establishes shared context for every interaction. Every agent starts with the same understanding of how the system works.

When you run any flow, CLAUDE.md is already loaded. You never need to explain "what is this swarm" because every agent already knows. This is how coordination becomes implicit rather than explicit.

### Layer 2: Agent Prompts (Executable Examples)

Each prompt in `.claude/agents/` demonstrates:

- Single responsibility in action
- Handoff structure
- Graceful outcome handling
- Evidence requirements

**Teaching function:** Shows what good agent design looks like.

The prompts are not abstract specifications. They are working code. When you read `code-critic.md`, you see how a critic behaves, what it produces, and how it hands off to the next agent. The prompt IS the documentation.

```markdown
# From code-critic.md (example structure)

## Role

You are code-critic. You find issues. You do not fix them.

## Output

Write findings to critique file. Report summary in handoff.

## Handoff

What I found: <issues by severity>
Recommendation: Route to fixer if CRITICAL/MAJOR items exist
```

The prompt teaches by being exactly what it describes.

### Layer 3: Flow Commands (Orchestration Examples)

Each flow in `.claude/commands/` demonstrates:

- Station sequencing
- Microloop iteration
- Routing decisions
- Gate integration

**Teaching function:** Shows how agents compose into workflows.

When you read `flow-3-build.md`, you see:

- Which agents get called and in what order
- What artifacts each station produces
- How the orchestrator routes on outcomes
- When loops terminate

The flow is both documentation and implementation. Running the flow teaches the pattern.

### Layer 4: Explanation Docs (The Why)

`docs/explanation/` provides:

- Principles with rationale
- Patterns with context
- Physics with examples
- Anti-patterns with warnings

**Teaching function:** Explains why things work the way they do.

When you encounter something unexpected (why do critics never fix? why are there two gates for GitHub?), the explanation docs provide the reasoning. They connect the "what" to the "why."

### Layer 5: How-To Docs (The Practice)

`docs/how-to/` provides:

- Step-by-step guides
- Common tasks
- Troubleshooting
- Customization

**Teaching function:** Shows how to apply the concepts.

When you need to do something specific (add a new agent, adapt for a different stack, troubleshoot a failure), the how-to guides provide concrete steps. They bridge from understanding to action.

### Layer 6: Reference Docs (The Details)

`docs/reference/` provides:

- Schemas and contracts
- CLI commands
- Agent index
- Marker definitions

**Teaching function:** Provides precise specifications.

When you need exact details (what fields does a receipt contain? what markers are valid?), the reference docs provide authoritative answers. They are the source of truth for implementation details.

---

## The Self-Reinforcing Loop

The pack teaches itself through a natural learning cycle:

```
Use the system → encounter the patterns
        ↓
Read CLAUDE.md → understand the contract
        ↓
See agent behavior → learn from examples
        ↓
Hit a problem → find the explanation doc
        ↓
Want to modify → find the how-to
        ↓
Need specifics → find the reference
        ↓
Use the system again → reinforce understanding
```

Each interaction reinforces the mental model. You learn by doing, and the system provides contextual explanations when you need them.

---

## Design Principles for Teaching Repos

### Consistency Across Layers

The same concepts appear at every layer, reinforcing each other:

- CLAUDE.md says X
- Agent prompts do X
- Docs explain X
- The system behaves according to X

When layers contradict, understanding breaks. The pack-check validates consistency to prevent drift:

```bash
bash .claude/scripts/pack-check.sh
```

Updates must propagate to all layers. If you change how handoffs work, you update CLAUDE.md, the agent prompts, and the relevant docs.

### Executable Documentation

The distinction between "docs" and "code" is intentionally blurred:

| Artifact      | Documentation? | Executable?               |
| ------------- | -------------- | ------------------------- |
| Agent prompts | Yes            | Yes                       |
| Flow commands | Yes            | Yes                       |
| CLAUDE.md     | Yes            | Yes (shapes all behavior) |
| Skills        | Yes            | Yes                       |

This means:

- Outdated docs fail visibly (the flow breaks)
- Examples are always runnable (they ARE the implementation)
- You can't have working code with wrong documentation

### Graduated Complexity

Information is layered by depth:

| If you need...     | Read...             | Depth        |
| ------------------ | ------------------- | ------------ |
| The basics         | CLAUDE.md           | Summary      |
| The concepts       | `docs/explanation/` | Intermediate |
| The specifications | `docs/reference/`   | Full detail  |

You start simple and add depth when you need it. The system does not force you to understand everything before you can do anything.

### Teaching by Doing

The system teaches by execution, not just explanation:

| Instead of...      | The system...                           |
| ------------------ | --------------------------------------- |
| Explain microloops | Runs them so you see iteration          |
| Describe handoffs  | Produces them so you see the format     |
| Document receipts  | Generates them so you see the structure |

Reading about receipts is helpful. Seeing the receipts the system generates is more helpful. Generating receipts yourself (by running flows) is most helpful.

---

## Why This Matters

### Faster Onboarding

New users learn by using:

1. Run a flow
2. See the patterns
3. Read the docs when curious
4. Modify with understanding

There is no "study the docs first, then try using it" phase. The system is designed to be used immediately, with learning happening along the way.

### Reduced Drift

When docs are executable, they cannot drift silently:

- Outdated docs fail visibly (the flow does not work)
- Pack-check catches mismatches (agent references, flow names)
- The system stays coherent (or it breaks obviously)

Traditional documentation drifts from implementation because they live in separate worlds. Here, they are the same thing.

### Knowledge Transfer

When someone leaves the project:

- The system documents itself
- Patterns are explicit in the prompts
- Mental model is encoded in CLAUDE.md
- Examples exist as working flows

The knowledge lives in the repo, not in people's heads.

---

## The Meta-Insight

Most repos have:

- Code that works
- Docs that explain
- Disconnect between them

This repo has:

- Prompts that are docs
- Flows that are examples
- Docs that match behavior
- Behavior that teaches

**The repo is the curriculum.**

The best way to learn this system is to use it. The best way to understand how agents should behave is to read the agent prompts. The best way to see how flows compose is to run them.

This is not an accident. The design intentionally collapses the distance between "how it works" and "how to understand it."

---

## Practical Implications

### For New Users

Start by running a flow:

```
/flow-1-signal "add a login feature"
```

Watch what happens. Read the artifacts it produces. Then read CLAUDE.md to understand why it happened that way. The sequence is:

```
Do → Observe → Understand → Do again with more context
```

### For Pack Maintainers

Every change should teach correctly:

- Update CLAUDE.md when contracts change
- Update agent prompts when behavior changes
- Update explanation docs when rationale changes
- Run pack-check to verify consistency

If a change only updates the implementation without updating the teaching layer, the next user will learn the wrong thing.

### For Pack Designers

If you create a new pack, consider:

- Can someone learn by using it?
- Do the prompts demonstrate the patterns?
- Is there a graduated path from summary to detail?
- Are the docs executable?

A pack that requires external training is harder to adopt than one that teaches itself.

---

## Influences

This approach was shaped by several domains:

- **ERP systems** — Where data integrity is non-negotiable and audit trails matter. The receipt-based verification and evidence discipline come from here.
- **XDA / FOSS communities** — Where teaching, portability, and helping newcomers are core values. The executable documentation and self-teaching design come from here.
- **Accessibility engineering** — Where building for everyone (not just the common case) is the standard. The PR cockpit as UI and "don't rely on color alone" come from here.
- **Lean manufacturing** — Where small stations, sensors, and throughput-with-quality are the model. The flow structure and fix-forward patterns come from here.

These aren't abstract influences—they show up in specific design choices throughout the pack.

---

## See Also

- [how-claude-md-works.md](how-claude-md-works.md) - CLAUDE.md mechanics
- [architecture.md](architecture.md) - System design principles
- [agent-philosophy.md](agent-philosophy.md) - How agents work
- [CLAUDE.md](../../CLAUDE.md) - The contract itself
