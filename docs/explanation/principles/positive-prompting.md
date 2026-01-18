# Principle: Positive Prompting

> Tell agents what TO do, not what NOT to do.

## The Principle

Agent prompts should focus on what the agent should accomplish and how to do it well, rather than listing constraints, forbidden actions, and things to avoid.

## Why This Matters

### Clarity of Purpose

Positive framing tells the agent what success looks like:

> "Focus on correctness and security first, style second"

Negative framing tells them what failure looks like:

> "Do NOT focus on style issues"

The first gives direction. The second just removes options.

### Cognitive Load

A list of "don'ts" requires the agent to:

1. Remember all the forbidden things
2. Check each action against the list
3. Figure out what's left

A positive guide says: "Here's what to do."

### Better Outcomes

Agents prompted positively produce better work because they understand the goal, not just the boundaries.

### Maintainability

Positive prompts are easier to update. Adding guidance is clearer than adding more constraints.

## How It Works

### Converting Constraints to Guidance

| Constraint (Bad)                    | Guidance (Good)                                 |
| ----------------------------------- | ----------------------------------------------- |
| "Do NOT modify files outside scope" | "Focus on files related to your task"           |
| "NEVER skip the summary"            | "End with a clear summary"                      |
| "Do NOT emit invalid status"        | "Report your findings clearly"                  |
| "Must NOT guess at requirements"    | "If something's unclear, note it as a question" |

### Structure of Positive Prompts

```markdown
## Your Job

[Clear statement of what to accomplish]

## What Success Looks Like

[Description of good outcomes]

## Tips

[Helpful guidance on how to do it well]
```

Not:

```markdown
## Constraints

- Do NOT do X
- NEVER do Y
- Must NOT do Z

## Forbidden Actions

- X is not allowed
- Y is prohibited
```

### The Tips Section

Tips are positive guidance:

```markdown
## Tips

- Focus on correctness and security first, style second
- Cite specific locations (file:line) so fixes are easy to find
- If something's ambiguous in the spec, note it as a question
- Group related issues together in your report
```

## Anti-Patterns

### Constraint Lists

```markdown
## Constraints

- Do NOT modify files outside your scope
- Do NOT emit fields not in the schema
- NEVER skip the summary section
- Must NOT use arbitrary status values
- Do NOT include code in your response
- NEVER make assumptions about requirements
```

### Fear-Based Prompting

```markdown
WARNING: If you emit invalid status, the system will fail.
CRITICAL: Never skip the summary or downstream will break.
```

### Double Negatives

```markdown
Do NOT fail to include the summary.
Never skip NOT checking the tests.
```

## Examples

### Good: Positive Guidance

```markdown
## Your Job

Review the implementation against the spec. Find issues that matter.

## Tips

- Focus on correctness and security first
- Cite specific locations for easy fixes
- Group related issues together
- Note ambiguities as questions rather than guessing
```

### Bad: Constraint-Heavy

```markdown
## Your Job

Review the implementation.

## Constraints

- Do NOT review style unless critical
- Do NOT make changes yourself
- NEVER skip the handoff section
- Must NOT report issues without locations
- Do NOT group unrelated issues
- NEVER guess at spec intent
```

## Conversion Exercise

Take any constraint and convert it:

**Constraint:** "Do NOT include implementation details in the summary"
**Guidance:** "Keep the summary focused on findings and recommendations"

**Constraint:** "NEVER emit more than 10 issues"
**Guidance:** "Focus on the most important issues; group minor ones"

**Constraint:** "Must NOT skip the handoff section"
**Guidance:** "End with a handoff summarizing findings and next steps"

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [How to Design Agents](../../how-to/design-agents.md)
