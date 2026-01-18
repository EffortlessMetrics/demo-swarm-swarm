# Principle: PM/Junior Model

> The orchestrator is a PM. Agents are well-trained juniors.

## The Principle

In Claude Code, the orchestrator (Claude itself) acts as a project manager directing a team of specialist agents. Each agent is like a well-trained junior developer who:

- Has a specific job they're good at
- Does real work
- Reports back clearly
- Suggests what should happen next

## Why This Matters

### Claude Is Capable

The orchestrator doesn't need training wheels. It can:

- Read natural language reports
- Understand context and nuance
- Make judgment calls
- Route based on understanding, not field parsing

Treating Claude like a parser that needs structured YAML wastes its capabilities.

### Natural Communication

A PM doesn't ask their team for JSON status reports. They read updates:

> "Finished the auth implementation. Tests pass. Found one issue with session timeout — it's 30m but spec says 15m. I can fix it, or we can ship and address later. What do you think?"

The PM reads this, understands it, decides. No parsing needed.

### Trust and Autonomy

Well-trained juniors don't need micromanagement. They:

- Understand their job
- Do it competently
- Ask for help when stuck
- Report honestly

The same applies to agents.

## How It Works

### Orchestrator Responsibilities

- Read flow command (the project plan)
- Spawn agents for specific tasks
- Read agent handoffs
- Make routing decisions
- Keep the flow moving

### Agent Responsibilities

- Do the assigned work
- Write meaningful artifacts
- Report back with findings
- Recommend next steps

### Communication Pattern

**Orchestrator → Agent:**

> "Review the implementation against the spec. Find issues. Report back."

**Agent → Orchestrator:**

> "Found 2 issues. Session timeout wrong, missing error handling. Recommend code-implementer fix both, then re-run me."

**Orchestrator decision:**

> "Okay, code-implementer — fix those issues."

## Anti-Patterns

### ❌ Treating Claude Like a Parser

```yaml
status: UNVERIFIED
recommended_action: RERUN
route_to_agent: code-implementer
```

The orchestrator doesn't need structured fields to route.

### ❌ Micromanaging Agents

Don't specify every step. Give them the job and let them figure out how.

### ❌ Removing Agent Judgment

Agents should make recommendations, not just report facts.

## Examples

### Good PM Behavior

Orchestrator reads: "Critic found 3 issues, recommends fixes before proceeding"
Orchestrator decides: Route to implementer with the issues

### Good Junior Behavior

Agent finds a problem, explains it clearly, suggests a solution, recommends next steps

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [Claude-Native Design](../claude-native-design.md)
