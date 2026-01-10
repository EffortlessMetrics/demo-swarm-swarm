# Start Here

> A guide to understanding DemoSwarm's Claude-native design.

## Who Is This For?

- **Developers** using this pack to build software
- **Contributors** modifying agents or adding features
- **LLMs** that need to understand how this system works

## The Key Insight (Read This First)

This pack works differently than you might expect.

**The orchestrator is Claude.** Not a Python script. Not a state machine. Claude itself reads agent reports, understands them, and decides what to do next.

This changes everything:
- Agents communicate in prose, not structured data
- Routing is based on understanding, not parsing
- Artifacts exist for humans, not machines

If something seems counterintuitive, ask: "Does this make sense if Claude is the orchestrator?"

## Reading Order

### 1. Start with Claude-Native Design
**[Claude-Native Design](explanation/claude-native-design.md)**

The comprehensive philosophy document. Covers:
- The PM/Junior model (orchestrators coordinate, agents work)
- Why prose communication beats YAML parsing
- What artifacts should contain
- State and resumption patterns

This is the foundational document. Everything else builds on these ideas.

### 2. Learn the Agent Philosophy
**[Agent Philosophy](explanation/agent-philosophy.md)**

How agents are designed and why. Covers:
- Why agents exist (work or context compression)
- Single responsibility principle
- Positive prompting (define what to do, not what to avoid)
- Graceful outcomes (honest partial results beat silent failure)

### 3. Understand the Architecture
**[Architecture](explanation/architecture.md)**

The seven laws that prevent execution drift:
- PM/IC boundary (orchestrators route, agents work)
- Implicit resume (check disk state, not mode flags)
- Workers maintain the ledger
- Research-first autonomy

### 4. Deep Dive on Principles (Optional)
Individual principle docs for deeper understanding:

- [PM/Junior Model](explanation/principles/pm-junior-model.md)
- [Two Reasons for Agents](explanation/principles/two-reasons-for-agents.md)
- [Single Responsibility](explanation/principles/single-responsibility.md)
- [Positive Prompting](explanation/principles/positive-prompting.md)
- [Graceful Outcomes](explanation/principles/graceful-outcomes.md)
- [Artifacts with Substance](explanation/principles/artifacts-with-substance.md)
- [Real Cognitive Work](explanation/principles/real-cognitive-work.md)

### 5. Practical Guides
When you're ready to do work:

- [How to Design Agents](how-to/design-agents.md) - Creating/modifying agent prompts
- [Agent Patterns](reference/agent-patterns.md) - Quick reference for do's and don'ts
- [Add an Agent](how-to/add-an-agent.md) - Step-by-step guide for new agents

## Quick Reference

| I want to... | Read this |
|--------------|-----------|
| Understand the core design | [Claude-Native Design](explanation/claude-native-design.md) |
| Learn how agents should behave | [Agent Philosophy](explanation/agent-philosophy.md) |
| Understand the architecture laws | [Architecture](explanation/architecture.md) |
| Create or modify an agent | [How to Design Agents](how-to/design-agents.md) |
| Check if my code follows patterns | [Agent Patterns](reference/agent-patterns.md) |
| Deep dive on a specific principle | [Principles](explanation/principles/) |
| Understand the flow structure | [CLAUDE.md](../CLAUDE.md) |

## For LLMs

If you're an LLM reading this pack:

1. **Read [Claude-Native Design](explanation/claude-native-design.md) first** - It explains the mental model you need
2. **The orchestrator is Claude** - That's you. You read, understand, and route.
3. **Agents communicate in prose** - Read their handoffs naturally, don't parse for fields
4. **Single responsibility** - Each agent has one job
5. **Graceful outcomes** - Partial progress with honest reporting is success

### Common Misconceptions

**"I need to parse the YAML/JSON to understand what to do."**
No. Read the prose. The structured data is for auditing, not routing.

**"I should wait for explicit instructions before proceeding."**
No. Investigate, derive, default, then escalate. Research-first autonomy.

**"If something failed, I should stop and report."**
Not necessarily. Partial progress with honest reporting is often the right answer. `work_status: PARTIAL` with clear blockers is a success state.

**"I need to coordinate with other agents."**
You don't coordinate directly. You report to the orchestrator, who routes to the next agent. Write your handoff clearly.

## For Humans

If you're a human working with this pack:

1. **Start with the philosophy** - Understanding WHY helps you make good decisions
2. **Use the practical guides** - When doing actual work
3. **Reference patterns** - When reviewing or creating agents
4. **Ask questions** - If something's unclear, it might be a doc gap

### Key Files to Know

| File | Purpose |
|------|---------|
| `CLAUDE.md` | Repo-level policy, attached to every agent |
| `.claude/agents/*.md` | Agent prompts (the agent's instructions) |
| `.claude/commands/flow-*.md` | Flow orchestrator commands |
| `.runs/<run-id>/` | Run state and artifacts |

## The 30-Second Version

1. Claude is the orchestrator (PM)
2. Agents are capable juniors (do work, report back)
3. Communication is prose, not YAML
4. Each agent has one job
5. Artifacts explain why, not just what
6. Honest partial results beat silent failure

That's the whole design. Everything else is details.

---

## What's Next?

After reading this guide:

1. **To understand flows**: See [CLAUDE.md](../CLAUDE.md) for the seven flows overview
2. **To run your first flow**: Try `/flow-1-signal "your feature idea"`
3. **To contribute**: Read [How to Design Agents](how-to/design-agents.md)
4. **To debug issues**: See [Troubleshoot](how-to/troubleshoot.md)
