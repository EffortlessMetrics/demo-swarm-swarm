# How CLAUDE.md Works

> The mechanics of CLAUDE.md as the repo-level policy contract.

---

## What CLAUDE.md Is

CLAUDE.md is the **repo-level policy file** that Claude Code automatically loads from the repository root. Every Claude Code session in this repository sees this file attached to its context.

Think of it as the **constitution** for the swarm. It defines:
- The operating philosophy (default-allow engineering, gated publishing)
- The coordination contracts (how agents communicate, who owns git)
- The flow overview (what exists, what each flow produces)
- Navigation pointers (where to find detailed documentation)

The opening line of CLAUDE.md captures its purpose:

> **Operational reality:** This file is attached to every agent thread in Claude Code. Treat it as **repo-level policy + shared contracts** (not a marketing doc).

This isn't documentation for humans browsing the repo. It's runtime policy that shapes agent behavior.

---

## How It Propagates

### Automatic Loading

Claude Code loads CLAUDE.md automatically when you open a session in a repository. You don't need to explicitly read it or include it in prompts.

### Inheritance to Spawned Agents

When an orchestrator spawns an agent using the Task tool, the agent inherits the CLAUDE.md context. This is why CLAUDE.md says "This file is attached to every agent thread."

The propagation chain:
1. **User session** - Claude Code loads CLAUDE.md
2. **Orchestrator** - Sees CLAUDE.md in its context
3. **Spawned agent** - Also sees CLAUDE.md (inherited via Task tool)
4. **Nested spawns** - Continue to inherit

Agents don't need to re-read CLAUDE.md. They already have it. This is automatic, not something the pack implements.

### Why This Matters

Every agent in the swarm operates under the same constitutional rules:
- They all know the flows exist
- They all understand the handoff pattern
- They all know repo-operator owns git
- They all speak the same coordination language

This shared context is what makes multi-agent orchestration coherent.

---

## What Belongs in CLAUDE.md vs Elsewhere

CLAUDE.md is loaded in **every** agent context. This has implications for what should live there versus elsewhere.

| Content Type | Location | Why |
|--------------|----------|-----|
| Pack philosophy | CLAUDE.md | Every agent needs the operating model |
| Flow overview | CLAUDE.md | Routing/navigation context |
| Coordination patterns | CLAUDE.md | Shared contracts between agents |
| Reference pointers | CLAUDE.md | "See X for details" links |
| Agent-specific instructions | `.claude/agents/*.md` | Only that agent needs them |
| Flow orchestration logic | `.claude/commands/flow-*.md` | Only that flow needs it |
| Skill invocation details | `.claude/skills/*.md` | Only skill users need it |
| Detailed how-to guides | `docs/how-to/*.md` | Reference when needed, not always |
| Schema specifications | `docs/reference/*.md` | Reference when needed, not always |

**Rule of thumb:** If every agent needs it for coordination, it goes in CLAUDE.md. If only some agents need it, put it elsewhere and link to it.

---

## The Inheritance Model

Context builds up in layers:

### Layer 1: CLAUDE.md (Base)

The repo-level policy. Everyone sees this.

Contains:
- Philosophy and operating model
- Coordination patterns
- Flow overview
- Skill list
- Reference pointers

### Layer 2: Agent Prompt (Specialization)

The agent-specific instructions from `.claude/agents/<agent>.md`.

Contains:
- What this agent does (and doesn't do)
- Input/output specifications
- Success criteria
- Tips for doing the job well
- Handoff template

### Layer 3: Task Description (Instance)

The specific invocation from the orchestrator.

Contains:
- The particular work to do now
- Relevant context for this task
- Run ID and file paths

**Example of the layers in action:**

```
CLAUDE.md says:         "Agents communicate through handoffs"
code-critic.md says:    "You find issues. You do not fix them."
Task description says:  "Review implementation for run feat-auth"
```

The agent knows the coordination model (CLAUDE.md), knows its role (agent prompt), and knows its current task (task description).

---

## Writing for CLAUDE.md

### Conciseness Matters

Every word in CLAUDE.md costs tokens in every agent spawn. A 5000-token CLAUDE.md means 5000 extra tokens for every agent invocation.

**Write tight:**
- State the principle, link to the explanation
- Tables over prose for reference material
- "See X" pointers for depth

**Don't write:**
- Full tutorials (put in `docs/how-to/`)
- Exhaustive explanations (put in `docs/explanation/`)
- Implementation details (put in agent prompts or skill docs)

### Focus on Contracts

CLAUDE.md is where agents learn to coordinate. Focus on:
- Shared vocabulary ("handoff", "receipt", "run")
- Ownership rules ("repo-operator owns git")
- Communication patterns ("natural language routing")
- Boundary rules ("gates engage at publish boundaries")

### Reference, Don't Duplicate

**Good:**
```markdown
## Handoffs

Agents communicate through natural language handoffs.
See: [docs/reference/contracts.md](docs/reference/contracts.md)
```

**Bad:**
```markdown
## Handoffs

Agents communicate through natural language handoffs.

### The Three Questions

Every handoff answers:
1. What did you do?
2. What's left?
3. What do you recommend?

### How to Write a Good Handoff

Start with a summary of completed work...
[500 more words]
```

The first version costs fewer tokens and points to authoritative documentation. The second duplicates content that now needs to be maintained in two places.

---

## What NOT to Put in CLAUDE.md

### Detailed How-To Guides

How-to guides are reference material for specific tasks. Agents read them when they need them, not on every spawn.

**Wrong:** Full guide for adding an agent in CLAUDE.md
**Right:** Link to `docs/how-to/add-an-agent.md`

### Agent-Specific Instructions

Each agent has its own prompt file. Instructions specific to code-critic don't need to be in CLAUDE.md.

**Wrong:** "Code-critic should categorize findings as CRITICAL/MAJOR/MINOR"
**Right:** Put this in `.claude/agents/code-critic.md`

### Verbose Explanations

CLAUDE.md captures the "what" concisely. The "why" in depth belongs in explanation docs.

**Wrong:** 500-word essay on why we use natural language routing
**Right:** One paragraph + "See [claude-native-design.md](docs/explanation/claude-native-design.md)"

### Frequently-Changing Details

If something changes often, it shouldn't be in the file that's loaded everywhere.

**Wrong:** Specific version numbers, dated information, implementation details
**Right:** Stable principles and patterns that evolve slowly

---

## The Token Economics

### Cost Multiplier

CLAUDE.md tokens are multiplied by every agent spawn:

```
Total CLAUDE.md cost = (tokens in CLAUDE.md) x (number of agent spawns)
```

A typical flow might spawn 5-10 agents. A complex build might spawn 20+. Every token in CLAUDE.md is paid many times over.

### The Trade-off

More content in CLAUDE.md means:
- Higher token cost per flow
- Richer shared context
- More consistent agent behavior

Less content means:
- Lower token cost
- Agents may miss important context
- More reliance on agent prompts and task descriptions

### Practical Guidance

1. **Essential coordination goes in CLAUDE.md** - The token cost is worth it for coherent multi-agent behavior
2. **Reference material links out** - Pay the tokens once when an agent needs detail, not on every spawn
3. **Agent-specific content goes in agent prompts** - Targeted, not broadcast
4. **Measure if in doubt** - Track token usage if you're concerned about a specific addition

---

## Updating CLAUDE.md

### Changes Are Immediate

When you modify CLAUDE.md, the next agent spawn sees the new version. There's no deployment, no cache invalidation. The change is live immediately.

This is powerful but requires care. A mistake in CLAUDE.md affects every agent until you fix it.

### Testing Changes

The best way to test CLAUDE.md changes:
1. Run a flow that exercises the changed content
2. Observe agent behavior in the task outputs
3. Check that coordination patterns work as expected

### Pack Consistency

`pack-check.sh` validates that CLAUDE.md is consistent with the pack:
- Flow names match flow commands
- Agent references are valid
- Skill names are correct

Run pack-check after CLAUDE.md updates to catch drift:

```bash
bash .claude/scripts/pack-check.sh
```

### Versioning

CLAUDE.md is version-controlled like any other file. Git history shows what changed and when. If a change causes problems, you can revert.

---

## CLAUDE.md in Practice

### How Agents Use It

Agents don't explicitly "read" CLAUDE.md. It's in their context automatically. They use it implicitly:

- When routing, they know the flow structure
- When handing off, they follow the handoff pattern
- When touching git, they know to describe operations for repo-operator
- When blocked, they know the escalation patterns

### How Orchestrators Use It

Orchestrators use CLAUDE.md to understand the coordination model:

- Which agents exist and what they do
- How flows connect
- What artifacts each flow produces
- Where to look for detailed specifications

### How Humans Use It

Humans use CLAUDE.md as the entry point:

- What commands are available (`/flow-1-signal`, etc.)
- What the philosophy is
- Where to find more information
- How to troubleshoot

---

## See Also

- [claude-native-design.md](claude-native-design.md) - Why the pack works the way it does
- [agent-philosophy.md](agent-philosophy.md) - How agents think and communicate
- [architecture.md](architecture.md) - System design principles
- [contracts.md](../reference/contracts.md) - Communication patterns and schemas
- [CLAUDE.md](../../CLAUDE.md) - The file itself
