# Skills vs Agents

> Skills execute. Agents reason.

## The Distinction

**Skills** are deterministic helpers that execute specific tasks:

- Run tests
- Apply formatting
- Count markers
- Scan for secrets

**Agents** are reasoning actors that make judgments:

- Review code quality
- Decide if we should merge
- Design solutions
- Analyze impact

## Why This Matters

### Right Tool for the Job

Some tasks need reasoning:

- "Is this implementation correct?" (judgment)
- "Should we ship this?" (decision)
- "What's the blast radius?" (analysis)

Some tasks need execution:

- "Run the test suite" (command)
- "Format the code" (transformation)
- "Count the requirements" (extraction)

Using an agent for execution is overkill. Using a skill for judgment is insufficient.

### Predictability vs Flexibility

**Skills are predictable:**

- Same input → same output
- No judgment variance
- Fast, mechanical

**Agents are flexible:**

- Handle edge cases
- Apply context
- Make judgment calls

### Cost Considerations

Skills are cheaper:

- Simpler execution
- No reasoning overhead
- Faster completion

Use skills when you can. Use agents when you must.

## When to Use Skills

### The Skill Criteria

Use a skill when:

- Task is mechanical
- Output is deterministic
- No judgment required
- Same input should always give same output

### Skill Examples

| Skill         | What It Does                | Why Skill (not Agent)        |
| ------------- | --------------------------- | ---------------------------- |
| test-runner   | Run tests, capture output   | Mechanical execution         |
| auto-linter   | Format and lint code        | Deterministic transformation |
| runs-derive   | Count markers, extract data | Mechanical counting          |
| runs-index    | Update index.json           | Structured update            |
| secrets-tools | Scan for secrets            | Pattern matching             |

### Skill Invocation

Skills are invoked by agents when needed:

```markdown
## Skills

- **test-runner**: For running the test suite
```

Agent uses the skill, interprets the result.

## When to Use Agents

### The Agent Criteria

Use an agent when:

- Task requires judgment
- Context affects the approach
- Edge cases need handling
- Output depends on understanding

### Agent Examples

| Agent            | What It Does             | Why Agent (not Skill)           |
| ---------------- | ------------------------ | ------------------------------- |
| code-critic      | Review implementation    | Requires understanding intent   |
| merge-decider    | Decide if we should ship | Weighs evidence, makes call     |
| code-implementer | Write code               | Creative, context-dependent     |
| impact-analyzer  | Assess change scope      | Requires codebase understanding |

### Agent Communication

Agents report in prose:

```markdown
## Handoff

Reviewed the implementation. Found two issues...
Recommend fixing before merge.
```

## The Handoff Pattern

**Agent uses skill:**

```
Agent spawned
    ↓
Agent reasons about task
    ↓
Agent invokes skill (e.g., test-runner)
    ↓
Skill returns results
    ↓
Agent interprets results
    ↓
Agent reports findings + recommendation
```

**Example:** test-executor

The test-executor agent:

1. Understands what tests to run
2. Invokes test-runner skill
3. Interprets pass/fail
4. Reports findings with context
5. Recommends next steps

The skill runs tests. The agent reasons about results.

## Common Mistakes

### Using Agent for Mechanical Tasks

"Spawn an agent to count the requirements."

**Fix:** Use runs-derive skill. Counting is mechanical.

### Using Skill for Judgment Tasks

"Use a skill to decide if we should merge."

**Fix:** Use merge-decider agent. This requires judgment.

### Reimplementing Skills in Agents

Agent prompt includes detailed instructions for running tests.

**Fix:** Agent invokes test-runner skill. Skill handles execution.

### Skills Making Decisions

Skill returns "PASS" or "FAIL" as a verdict.

**Fix:** Skill returns data. Agent interprets and decides.

## The Boundary

| Need                      | Use   | Reason          |
| ------------------------- | ----- | --------------- |
| Run tests                 | Skill | Execution       |
| Interpret test results    | Agent | Judgment        |
| Format code               | Skill | Transformation  |
| Review code quality       | Agent | Analysis        |
| Count markers             | Skill | Extraction      |
| Assess completeness       | Agent | Judgment        |
| Scan for secrets          | Skill | Pattern match   |
| Decide if safe to publish | Agent | Risk assessment |

## Skills in This Pack

| Skill         | Purpose                   |
| ------------- | ------------------------- |
| test-runner   | Run tests, capture output |
| auto-linter   | Format and lint           |
| policy-runner | Run policy checks         |
| runs-derive   | Read-only derivations     |
| runs-index    | Update index.json         |
| openq-tools   | Open questions register   |
| secrets-tools | Secrets scanning          |

## Agents Using Skills

Agents declare which skills they use:

```markdown
## Skills

- **test-runner**: For executing tests
- **runs-index**: For updating run state
```

This makes the dependency explicit.

## See Also

- [Two Reasons for Agents](principles/two-reasons-for-agents.md) — When to spawn agents
- [Single Responsibility](principles/single-responsibility.md) — Focused tools
