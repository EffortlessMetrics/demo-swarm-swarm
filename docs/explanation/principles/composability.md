# Principle: Composability

> Simple agents compose into powerful workflows.

## The Principle

Each agent does one thing well. The orchestrator combines them into sophisticated workflows. Power comes from composition, not complexity.

## Why This Matters

### Simple Parts, Complex Whole

A complex agent is:
- Hard to understand
- Hard to debug
- Hard to modify
- Brittle

Simple agents composed:
- Each part is clear
- Issues are isolated
- Changes are local
- System is robust

### Reusability

A focused agent can be reused:
- code-critic works in Build AND Review
- test-executor works in Build AND Gate
- clarifier works in Signal AND Build

A kitchen-sink agent is one-off.

### Parallel Execution

Independent agents can run in parallel:
- code-critic AND test-critic (different domains)
- security-scanner AND coverage-enforcer (different checks)

Composed workflows naturally parallelize.

### Substitutability

Don't like how an agent works? Replace just that agent:
- Swap test-critic for a stricter version
- Use a different security-scanner
- Add a new critic to the chain

The rest of the workflow stays the same.

## How It Works

### The Orchestrator Composes

The orchestrator (Claude as PM) sequences agents:

```
For each acceptance criterion:
    test-author → writes tests
    test-critic → reviews tests
    code-implementer → writes code
    code-critic → reviews code
    test-executor → runs tests
```

Each agent does one job. The orchestrator creates the workflow.

### Composition Patterns

**Sequential:**
```
A → B → C
```
Each depends on the previous.

**Parallel:**
```
    ┌→ B ─┐
A ──┤     ├→ D
    └→ C ─┘
```
B and C run simultaneously after A.

**Iterative:**
```
A → B → (feedback) → A → B → ... → C
```
Loop until condition met.

**Conditional:**
```
A → (if X) → B
    (else) → C
```
Branch based on outcome.

### Example: Build Workflow

```
context-loader (load relevant files)
    ↓
For each AC:
    test-author → test-critic ─┐
                               ├→ (iterate until critic satisfied)
    code-implementer → code-critic ─┘
    ↓
    test-executor
    ↓
    (if issues) → fixer → test-executor
    ↓
build-cleanup (summarize)
```

15 lines of composition creates a sophisticated build process.

### Agent Interfaces

Agents compose because they share a common interface:
- **Input:** Context + task description
- **Output:** Artifacts + handoff with recommendation
- **Contract:** Single responsibility, clear handoff

Any agent can be replaced with another that follows this interface.

## Composition Examples

### Quality Chain
```
code-implementer
    ↓
code-critic (correctness)
    ↓
security-scanner (security)
    ↓
standards-enforcer (style)
```

Each adds a quality dimension. Together: comprehensive review.

### Verification Chain
```
test-executor (unit tests)
    ↓
contract-enforcer (API contracts)
    ↓
coverage-enforcer (coverage thresholds)
```

Each verifies a different property.

### Decision Chain
```
receipt-checker (artifacts valid)
    ↓
merge-decider (should ship?)
    ↓
secrets-sanitizer (safe to publish?)
```

Each contributes to the final decision.

## Building New Workflows

### Step 1: Identify Responsibilities
What distinct jobs need doing?
- "Review code" — code-critic
- "Check security" — security-scanner
- "Run tests" — test-executor

### Step 2: Define Sequence
What order? What depends on what?
- Tests first (need passing tests to review)
- Security after code exists
- Critic after implementation

### Step 3: Identify Iteration
Where might we loop?
- Writer/critic loops
- Test/fix loops

### Step 4: Compose
Write the orchestration that sequences agents.

## Anti-Patterns

### The God Agent
One agent that does everything.

**Fix:** Split into focused agents. Compose them.

### Tight Coupling
Agent A directly calls Agent B.

**Fix:** Orchestrator manages all agent calls. Agents are independent.

### Hidden Dependencies
Agent assumes another agent ran without checking.

**Fix:** Check artifacts. Evidence over trust.

### Non-Standard Interface
Agent returns data in a unique format.

**Fix:** All agents use handoff pattern. Standard interface.

## The Power of Composition

### Small Agents
- code-implementer: ~200 lines of prompt
- code-critic: ~200 lines of prompt
- test-author: ~150 lines of prompt

### Large Workflows
Build flow composes 10+ agents into a sophisticated process that:
- Implements features
- Reviews quality
- Runs tests
- Fixes issues
- Documents changes
- Creates PRs

The whole is greater than the sum of parts.

## See Also

- [Single Responsibility](single-responsibility.md) — Why agents are focused
- [Two Reasons for Agents](two-reasons-for-agents.md) — When to create agents
- [Agent Philosophy](../agent-philosophy.md) — How agents behave
