# Principle: Single Responsibility

> Each agent has one job. They do it deeply.

## The Principle

Every agent should have exactly one clear responsibility. When you can describe an agent's job in one sentence, you've got it right.

## Why This Matters

### Depth Over Breadth

An agent with one job can:

- Understand the task deeply
- Apply full attention to it
- Produce quality output
- Be expert at that thing

An agent with seven jobs:

- Does everything shallowly
- Misses nuances
- Produces mediocre output across the board

### Clear Ownership

When something goes wrong, you know which agent owns it.

When something needs improvement, you know which prompt to update.

### Composability

Single-responsibility agents compose well. The orchestrator chains them:

> test-author → test-critic → code-implementer → code-critic → test-executor

Each does one thing. Together they build quality software.

### Easier Prompts

A focused prompt is easier to write, understand, and maintain than a sprawling one.

## How It Works

### One Sentence Test

If you can't describe the agent's job in one sentence, split it.

| Good                                 | Bad                                                                          |
| ------------------------------------ | ---------------------------------------------------------------------------- |
| "Write tests for the implementation" | "Write tests, check coverage, update docs, fix lint"                         |
| "Find issues in the code"            | "Review code, validate security, check contracts, update receipt"            |
| "Decide if we should merge"          | "Check tests, scan security, verify contracts, decide merge, post to GitHub" |

### Agent Categories

| Category    | Job                                |
| ----------- | ---------------------------------- |
| **Workers** | Do one type of implementation work |
| **Critics** | Review one aspect of quality       |
| **Cleanup** | Summarize one flow                 |
| **Gate**    | Make one decision                  |

### Splitting Agents

If an agent is doing too much:

1. Identify the distinct responsibilities
2. Create separate agents for each
3. Let the orchestrator sequence them

**Before:** One agent reviews code AND checks contracts AND validates security

**After:** code-critic + contract-enforcer + security-scanner, orchestrated in sequence

## Anti-Patterns

### ❌ The Kitchen Sink Agent

```markdown
## Your Responsibilities

1. Review the implementation
2. Check contract compliance
3. Validate security
4. Verify test coverage
5. Update the receipt
6. Maintain the index
7. Report to GitHub
```

This agent does too much. Split it.

### ❌ "While You're There..."

Don't add "also do X" to an agent. If X is important, make it a separate agent.

### ❌ Coupling Unrelated Work

Updating the index and reviewing code are unrelated. Don't combine them.

## Examples

### Good: Focused Critic

```markdown
## Your Job

Review the implementation against the spec. Find issues that matter.
```

### Good: Focused Worker

```markdown
## Your Job

Write tests for the acceptance criteria. Cover happy path and error cases.
```

### Bad: Sprawling Agent

```markdown
## Your Job

Review the implementation, then update the receipt with your findings,
then check if the index needs updating, then post a summary to GitHub.
```

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [How to Design Agents](../../how-to/design-agents.md)
