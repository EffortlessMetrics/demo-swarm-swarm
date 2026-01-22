# Principle: Real Cognitive Work

> Agents think, they don't copy.

## The Principle

When agents produce output, they should be doing real cognitive work — reading, understanding, reasoning, synthesizing. Not mechanically extracting fields from one place and copying them to another.

## Why This Matters

### The Value of Agents

Agents add value through understanding:

- Reading implementation AND spec to find mismatches
- Analyzing test results AND requirements to assess coverage
- Reviewing evidence AND applying judgment to make decisions

If an agent is just copying fields, a shell script could do it.

### Quality of Output

An agent that understands what it's reading produces better output:

- More accurate assessments
- Better explanations
- Useful recommendations

An agent that mechanically extracts fields can only produce what's already there.

### Handling Edge Cases

Real understanding handles edge cases naturally:

- "The tests pass but they don't actually test the requirement"
- "The status says verified but there's clearly a problem"
- "The numbers look good but the context suggests an issue"

Mechanical extraction misses these.

## How It Works

### The Thinking Test

Ask: Is the agent thinking or copying?

| Thinking (Good)                                | Copying (Bad)                              |
| ---------------------------------------------- | ------------------------------------------ |
| "Read the critique and summarize key findings" | "Extract the severity_summary field"       |
| "Analyze test results and assess readiness"    | "Copy passed and failed counts to receipt" |
| "Review evidence and make a merge decision"    | "Check if blockers array is empty"         |
| "Understand what happened and write summary"   | "Run ms get on 5 files and aggregate"      |

### Cognitive Work Examples

**Reading and Understanding:**

> "Read code_critique.md. What did the critic find? Were there critical issues? Is this blocking or advisory?"

**Analyzing and Synthesizing:**

> "Look at the test results, the critic findings, and the coverage report. What's the overall quality picture?"

**Judging and Deciding:**

> "Given all the evidence, should we ship this? What's your reasoning?"

### Mechanical Work Examples (Bad)

**Field Extraction:**

```bash
bash .claude/scripts/demoswarm.sh ms get --file "code_critique.md" --key "status"
```

**Copy and Paste:**

> "Copy the status field from the critique to the receipt"

**Boolean Logic:**

> "If blockers is empty and status is VERIFIED, set verdict to MERGE"

## Receipt Example

### Mechanical Approach (Bad)

```
1. Extract status fields from critique artifacts
2. Parse structured blocks for counts
3. Copy fields to receipt
4. If all fields are "good", receipt status = VERIFIED
```

The agent is just a data pipeline.

### Cognitive Approach (Good)

```
1. Read code_critique.md — what did the critic find?
2. Read test_critique.md — any gaps in coverage?
3. Read test_execution.md — did tests actually pass?
4. Understand what happened in this build
5. Write a receipt that tells the story
```

The agent understands and synthesizes.

### Same Output, Different Process

Both approaches might produce similar receipts. But the cognitive approach:

- Catches edge cases
- Produces better explanations
- Handles unexpected situations
- Adds real value

## Anti-Patterns

### Field Extraction Pipelines

```bash
status=$(ms get --file X --key status)
count=$(ms get --file Y --key count)
echo "{\"status\": \"$status\", \"count\": $count}"
```

### Field-by-Field Instructions

```markdown
1. Extract field A from file X
2. Extract field B from file Y
3. Put A in receipt.fieldA
4. Put B in receipt.fieldB
```

### Boolean Routing

```markdown
If status == VERIFIED and blockers.empty():
verdict = MERGE
else:
verdict = BOUNCE
```

## Rewriting Mechanical to Cognitive

### Before (Mechanical)

```markdown
### Step 3: Extract quality gate status

Parse structured blocks from critiques:

- code_critique.md → quality_gates.code_critic
- test_critique.md → quality_gates.test_critic
```

### After (Cognitive)

```markdown
### Step 3: Understand what the critics found

Read the critique artifacts:

- code_critique.md — What issues did the code critic find? Blocking or advisory?
- test_critique.md — Any gaps in test coverage? Quality concerns?

Summarize the critic findings for the receipt.
```

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [Artifacts with Substance](artifacts-with-substance.md)
