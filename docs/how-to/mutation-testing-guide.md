# Mutation Testing Guide

> How to configure, run, and interpret mutation testing in DemoSwarm.

Mutation testing validates that your tests actually catch bugs. It works by injecting small changes (mutations) into your code and checking whether tests fail. Tests that fail correctly "kill" the mutant; tests that still pass reveal gaps in assertion coverage.

---

## Overview

### What Is Mutation Testing?

Traditional test coverage tells you which lines of code were executed. It does not tell you whether the tests actually verified anything meaningful. A test can run through every line of code without asserting a single thing.

Mutation testing answers a different question: **If we break the code, do the tests notice?**

| Metric | What It Measures | Limitation |
|--------|------------------|------------|
| Line coverage | Which lines were executed | Execution is not verification |
| Branch coverage | Which branches were taken | Still does not require assertions |
| Mutation score | Which mutations were detected | Expensive but meaningful |

### Why It Matters

High coverage with a low mutation score indicates "hollow tests" — tests that run but do not assert. These tests provide false confidence.

**Example:**
```python
def calculate_discount(price, rate):
    return price * (1 - rate)

def test_calculate_discount():
    result = calculate_discount(100, 0.1)
    # No assertion! Test passes but verifies nothing.
```

Mutation testing would reveal this gap: changing `1 - rate` to `1 + rate` would not cause the test to fail.

### The Trade-Off

Mutation testing is computationally expensive. Running a full mutation suite on a large codebase can take hours. DemoSwarm addresses this through:

1. **Scoped execution** — Run mutations only on changed files
2. **Time budgets** — Cap execution at a configurable limit
3. **Prioritized worklists** — Focus on the most impactful survivors

---

## Configuring Mutation Testing

The `mutation-auditor` agent handles mutation testing in Flow 3 (Build). Configuration is optional; mutation testing will be skipped if no runner is configured.

### Configuration Options

Add mutation settings to `demo-swarm.config.json`:

```json
{
  "mutation": {
    "command": "npm run mutate -- --since HEAD~1",
    "budget_seconds": 300
  }
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `command` | string | null | The mutation testing command to run |
| `budget_seconds` | integer | 300 | Maximum execution time (5 minutes default) |

### Command Discovery

If `demo-swarm.config.json` does not specify a command, the agent looks for repo-local scripts in this order:

1. `scripts/mutation.sh`
2. `scripts/mutation.ps1`
3. `scripts/mutation.bat`
4. `scripts/mutation.cmd`

If none exist, mutation testing is skipped with an explicit "not configured" status.

### Example Configurations

**JavaScript/TypeScript (Stryker):**
```json
{
  "mutation": {
    "command": "npx stryker run --since HEAD~1",
    "budget_seconds": 600
  }
}
```

**Python (mutmut):**
```json
{
  "mutation": {
    "command": "mutmut run --paths-to-mutate $(git diff --name-only HEAD~1 -- '*.py')",
    "budget_seconds": 300
  }
}
```

**Java (PIT):**
```json
{
  "mutation": {
    "command": "mvn org.pitest:pitest-maven:mutationCoverage",
    "budget_seconds": 900
  }
}
```

### Scoping to Changed Files

Mutation testing should focus on the change surface, not the entire codebase. The agent uses `git diff --name-only` to identify changed files. Your mutation command should accept this scope.

**Stryker (JavaScript):**
```bash
npx stryker run --since HEAD~1
```

**mutmut (Python):**
```bash
mutmut run --paths-to-mutate src/changed_file.py
```

**PIT (Java):**
```bash
mvn pitest:mutationCoverage -DtargetClasses=com.example.ChangedClass
```

---

## Interpreting Mutation Reports

After execution, the `mutation-auditor` writes `.runs/<run-id>/build/mutation_report.md`.

### Report Structure

```markdown
# Mutation Report

## Run Metrics

Mutation command: "npx stryker run --since HEAD~1"
Budget: 300 seconds
Duration: 187 seconds

Results:

- Killed: 45
- Survived: 3
- Errors: 0
- Timeouts: 2

## Run Notes

- Tool/config selection: Used demo-swarm.config.json mutation.command
- Exit status: 0
- Limits: 2 mutants timed out (30s limit per mutant)

## Survivor Worklist (prioritized)

- MUT-SURV-001 [ASSERTION_GAP]
  - Location: src/auth/login.ts:42
  - What it suggests: Return value not verified
  - Next action: Add assertion for JWT expiration
  - Route: test-author

- MUT-SURV-002 [MISSING_EDGE_CASE]
  - Location: src/auth/login.ts:55
  - What it suggests: Empty password not tested
  - Next action: Add test for empty credentials
  - Route: test-author

- MUT-SURV-003 [ORACLE_WEAKNESS]
  - Location: src/utils/format.ts:12
  - What it suggests: Assertion too permissive
  - Next action: Tighten string format assertion
  - Route: test-author

## Inventory (machine countable)

- MUT_SURVIVOR: MUT-SURV-001
- MUT_SURVIVOR: MUT-SURV-002
- MUT_SURVIVOR: MUT-SURV-003

## Handoff

**What I did:** Ran mutation testing on 3 changed files. 45 killed, 3 survived, 2 timeouts.

**What's left:** 3 survivors need test improvements.

**Recommendation:** Route to test-author to address survivor worklist items.
```

### Understanding the Metrics

| Metric | Meaning |
|--------|---------|
| **Killed** | Tests caught the mutation (good) |
| **Survived** | Tests did not catch the mutation (indicates a gap) |
| **Errors** | Mutation caused compilation or runtime failure (not your tests' fault) |
| **Timeouts** | Mutation caused tests to hang (often infinite loops) |

### Mutation Score Calculation

```
Mutation Score = Killed / (Killed + Survived) * 100
```

Example: 45 killed, 3 survived = 45/(45+3) = 93.75%

Timeouts and errors are typically excluded from the denominator since they indicate tool issues, not test quality.

---

## Understanding Survivor Worklists

Survivors are mutations that tests did not catch. Each survivor indicates a potential gap in test coverage or assertion quality.

### Survivor Categories

| Category | Meaning | Typical Fix |
|----------|---------|-------------|
| `ASSERTION_GAP` | Test runs but does not assert the affected value | Add an assertion |
| `ORACLE_WEAKNESS` | Assertion exists but is too permissive | Tighten the assertion |
| `MISSING_EDGE_CASE` | No test for the mutated scenario | Add edge case test |
| `MISSING_NEGATIVE_TEST` | No test for rejection/error paths | Add error path test |
| `UNSAFE_MUTATION_TARGET` | Generated or unstable code | Consider excluding from mutation |

### Example Worklist Item

```markdown
- MUT-SURV-001 [ASSERTION_GAP]
  - Location: src/auth/login.ts:42
  - What it suggests: Return value not verified
  - Next action: Add assertion for JWT expiration
  - Route: test-author
```

**Location:** The file and line where the mutation survived.

**What it suggests:** Why this mutation likely survived.

**Next action:** A concrete step to address the gap.

**Route:** Which agent should handle this (usually `test-author`).

### When to Fix Survivors vs Accept Them

**Fix when:**
- The mutation affects correctness-critical logic
- The gap is in a security-sensitive path
- Adding the test is straightforward

**Accept when:**
- The mutation is in generated or external code
- The cost of testing exceeds the risk
- The code is scheduled for removal

Document accepted survivors with reasoning:

```markdown
- MUT-SURV-005 [UNSAFE_MUTATION_TARGET]
  - Location: src/generated/types.ts:100
  - What it suggests: Generated code; mutations not meaningful
  - Next action: Exclude from mutation scope
  - Route: none (accepted)
```

### Threshold Guidance

| Mutation Score | Typical Interpretation |
|----------------|------------------------|
| 90%+ | Strong test coverage; survivors likely edge cases |
| 80-90% | Good coverage; review survivors for high-risk gaps |
| 70-80% | Moderate coverage; prioritize critical path survivors |
| Below 70% | Coverage gaps likely; review test strategy |

These are guidelines, not gates. Context matters more than numbers.

---

## Integration with Test-Author

When mutation testing reveals survivors, the orchestrator routes to `test-author` with the worklist.

### The Feedback Loop

```
mutation-auditor
      │
      ▼
   Survivors found
      │
      ▼
   test-author
      │
      ▼
   Tests strengthened
      │
      ▼
   test-executor
      │
      ▼
   (loop back to mutation-auditor if needed)
```

### What Test-Author Receives

The test-author agent receives the survivor worklist and addresses items by:

1. Adding missing assertions for `ASSERTION_GAP` items
2. Tightening permissive assertions for `ORACLE_WEAKNESS` items
3. Adding edge case tests for `MISSING_EDGE_CASE` items
4. Adding error path tests for `MISSING_NEGATIVE_TEST` items

### Example: Addressing a Survivor

**Survivor:**
```markdown
- MUT-SURV-001 [ASSERTION_GAP]
  - Location: src/auth/login.ts:42
  - What it suggests: JWT expiration not verified
  - Next action: Add assertion for JWT expiration
```

**Before (hollow test):**
```typescript
test('login returns JWT', async () => {
  const result = await login('user', 'pass');
  expect(result).toBeDefined(); // Too weak
});
```

**After (strengthened test):**
```typescript
test('login returns JWT with correct expiration', async () => {
  const result = await login('user', 'pass');
  expect(result).toBeDefined();
  expect(result.exp - result.iat).toBe(900); // 15 minute expiration
  expect(result.sub).toBe('user');
});
```

---

## Troubleshooting

### Mutation Testing Not Running

**Check:** Is a mutation command configured?

```bash
# Look for config
cat demo-swarm.config.json | jq '.mutation'

# Or check for scripts
ls scripts/mutation.*
```

**Fix:** Add mutation configuration to `demo-swarm.config.json` or create a `scripts/mutation.sh` wrapper.

### Mutation Run Times Out

**Check:** Is the budget too short for your codebase?

**Fix:** Increase `budget_seconds` or scope the mutation command more narrowly.

### Too Many Survivors

**Check:** Are tests asserting anything, or just running code?

**Fix:** Focus on strengthening assertions before expanding test count. One test with strong assertions beats ten tests with weak ones.

### Mutation Tool Errors

**Check:** Does the mutation command work standalone?

```bash
# Run the exact command from config
npx stryker run --since HEAD~1
```

**Fix:** Debug the mutation tool configuration. Common issues:
- Test framework version mismatches
- Missing dependencies
- Incorrect file patterns

---

## See Also

- [`.claude/agents/mutation-auditor.md`](../../.claude/agents/mutation-auditor.md) — Full agent specification
- [`.claude/agents/test-author.md`](../../.claude/agents/test-author.md) — Test authoring agent
- [verification-stack.md](../explanation/verification-stack.md) — Where mutation fits in the verification hierarchy
- [adversarial-loops.md](../explanation/adversarial-loops.md) — Writer-critic patterns
- [working-with-receipts.md](working-with-receipts.md) — Reading build receipts (includes mutation_score)
