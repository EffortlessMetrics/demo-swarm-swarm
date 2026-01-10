# Principle: Two Reasons for Agents

> Spawn an agent to do work OR to compress context. Nothing else.

## The Principle

There are exactly two valid reasons to spawn an agent:

1. **Work needs to be done** — implementation, testing, fixing, reviewing
2. **Context needs to be compressed** — summarizing, analyzing, deciding

If neither applies, the orchestrator should handle it directly.

## Why This Matters

### Agents Have Overhead

Spawning an agent:
- Adds a round trip
- Requires context transfer
- Needs result interpretation

Only do it when the benefit exceeds the overhead.

### Clarity of Purpose

When you know WHY you're spawning an agent, you can:
- Choose the right agent
- Give clear instructions
- Interpret results correctly

### Avoiding Agent Soup

Without this discipline, you end up spawning agents for everything, creating a confusing mess of handoffs for simple tasks.

## The Two Reasons

### Reason 1: Work Needs Doing

The orchestrator needs something built, fixed, or changed.

| Agent Type | Work They Do |
|------------|--------------|
| code-implementer | Write/modify code |
| test-author | Write tests |
| fixer | Fix identified issues |
| doc-writer | Update documentation |
| repo-operator | Handle git operations |
| standards-enforcer | Apply formatting/linting |

**Spawn signal:** "I need someone to make changes."

### Reason 2: Context Needs Compressing

The orchestrator has too much to read, or needs specialized analysis.

| Agent Type | Context They Compress |
|------------|----------------------|
| cleanup | All artifacts from a flow → summary + receipt |
| code-critic | Implementation + spec → quality assessment |
| merge-decider | All evidence → ship/no-ship decision |
| context-loader | Large codebase → relevant working set |
| impact-analyzer | Change scope → blast radius map |

**Spawn signal:** "I have a lot to review" or "I need expert analysis."

## The Decision Framework

```
Need something done?
├── Yes: Can I do it in a few lines?
│   ├── Yes → Do it directly
│   └── No → Spawn a worker agent
└── No: Do I have a lot to read/analyze?
    ├── Yes → Spawn a summarizer/analyst agent
    └── No → Handle it directly
```

### Examples

| Situation | Decision |
|-----------|----------|
| Need to implement a feature | Spawn code-implementer (work) |
| Need to run tests | Spawn test-executor (work) |
| Need to review 15 build artifacts | Spawn cleanup (context compression) |
| Need to decide if we should merge | Spawn merge-decider (context compression) |
| Need to update one config value | Do it directly (trivial work) |
| Need to read one short file | Do it directly (no compression needed) |

## When NOT to Spawn

### ❌ Trivial Tasks
Don't spawn an agent to change one line. Just change it.

### ❌ Simple Reads
Don't spawn an agent to read one file. Just read it.

### ❌ Routing Decisions
The orchestrator makes routing decisions. Don't spawn an agent to decide what agent to call.

### ❌ Status Checks
Don't spawn an agent just to check if something exists or has a value.

## Agent Categories by Reason

### Work Agents
- code-implementer
- test-author
- fixer
- doc-writer
- repo-operator
- standards-enforcer
- test-executor

### Context Compression Agents
- All critics (code-critic, test-critic, etc.)
- All cleanup agents
- Gate agents (merge-decider, deploy-decider)
- context-loader
- impact-analyzer
- clarifier

### Hybrid (Both)
- Some agents do work AND produce compressed context
- Example: test-executor runs tests (work) and summarizes results (compression)

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [PM/Junior Model](pm-junior-model.md)
