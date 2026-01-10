# The Verification Stack

> The product isn't code. The product is verified artifacts. Code is just what got verified.

---

## The Core Insight

**Verification is the product. Code is just the artifact that gets verified.**

The system doesn't produce "code that hopefully works." It produces "verified artifacts with evidence."

Traditional development:
```
Developer writes code -> Reviewer reads code -> Ship
```

The product is "code someone wrote."

AgOps development:
```
Spec -> Generate -> Verify -> Verify -> Verify -> Evidence -> Ship
```

The product is "verified artifact with proof."

This distinction matters. When you sell a bridge, you don't sell "steel that a welder assembled." You sell "a structure certified to bear load." The certification is the value; the steel is the substrate.

---

## The Stack

Verification is layered. Each layer catches different failure modes.

### Layer 1: Syntax (Immediate)

**What it catches:** Code that doesn't compile/parse

**How:** Language tooling, formatters, linters

**Trust gained:** "This is valid code"

This is table stakes. It's not verification—it's just "not obviously broken."

A file that doesn't parse can't do anything. Passing this layer means the code could theoretically execute. It says nothing about whether it should.

### Layer 2: Tests (Behavioral)

**What it catches:** Code that doesn't behave as specified

**How:** Unit tests, integration tests, BDD scenarios

**Trust gained:** "This does what the spec says"

Tests prove behavior matches intent. They encode expectations as executable checks. When tests pass, we have evidence that specific behaviors work.

But tests can be hollow. A test that exercises code without actually checking its behavior provides false confidence. "Coverage" without meaningful assertions is theater.

### Layer 3: Mutation (Test Quality)

**What it catches:** Tests that don't actually test the code

**How:** Inject mutations, verify tests fail

**Trust gained:** "The tests are real, not theater"

Mutation testing is adversarial quality at its purest:

```
Original: if (balance > 0) { allow_withdrawal(); }
Mutant:   if (balance >= 0) { allow_withdrawal(); }
```

If tests pass on the mutant, they exercise the code path but don't verify the `> 0` condition. The tests are hollow.

Mutation testing proves the tests would catch bugs. Without it, tests might pass on broken code.

### Layer 4: Critics (Quality)

**What it catches:** Issues that tests don't cover

**How:** Adversarial review by critic agents

**Trust gained:** "Another perspective found no issues"

Critics find what automated testing misses:

- **Design problems** — Architecture that will cause pain later
- **Security concerns** — Patterns that introduce vulnerabilities
- **Maintainability issues** — Code that's correct but opaque
- **Spec mismatches** — Implementation drift from requirements

The author/critic separation defeats sycophancy. A model reviewing its own work rationalizes errors. A model reviewing "someone else's" work finds problems.

### Layer 5: Evidence (Claims)

**What it catches:** Claims without backing

**How:** Require pointers for all assertions

**Trust gained:** "What they said happened, actually happened"

Evidence discipline prevents confabulation:

| Claim | Without Evidence | With Evidence |
|-------|------------------|---------------|
| "Tests pass" | Maybe they ran, maybe not | `npm test` exit 0, 47 passed (artifact: test_execution.md) |
| "Secure" | Someone thought about it | secrets-tools scan on 12 files, 0 findings (artifact: secrets_scan.md) |
| "Complete" | Author says so | All ACs verified, critic says `can_further_iteration_help: no` |

"Not measured" is honest. "Passed" without evidence is dangerous.

### Layer 6: Gates (Boundaries)

**What it catches:** Unsafe publish operations

**How:** Secrets scanning, anomaly detection

**Trust gained:** "Nothing dangerous escapes"

Gates are the last line. They catch what slipped through everything else:

- Credentials that would be exposed
- Artifacts that shouldn't be public
- State that isn't ready to publish

Gates are cheap but critical. They're not where quality comes from—they're where disasters get prevented.

---

## What Each Layer Proves

| Layer | Proves | Doesn't Prove |
|-------|--------|---------------|
| Syntax | Valid code | Correct code |
| Tests | Matches spec | Spec is right |
| Mutation | Tests are real | Tests are complete |
| Critics | No obvious issues | No subtle issues |
| Evidence | Claims are backed | Claims are correct |
| Gates | Safe to publish | Good to publish |

Each layer is necessary. None is sufficient alone.

**Syntax** without tests: The code runs but does the wrong thing.

**Tests** without mutation: The tests pass but don't catch bugs.

**Mutation** without critics: The tests work but miss design issues.

**Critics** without evidence: Issues were discussed but maybe not fixed.

**Evidence** without gates: Everything was verified but secrets leaked.

The stack is cumulative. Higher layers assume lower layers passed.

---

## The Verification Pyramid

```
        +----------+
        |  Gates   |  <- Catches escapes
       +------------+
       | Evidence  |  <- Backs claims
      +--------------+
      |   Critics   |  <- Adversarial review
     +----------------+
     |   Mutation    |  <- Proves tests work
    +------------------+
    |     Tests       |  <- Proves behavior
   +--------------------+
   |      Syntax       |  <- Valid code
   +--------------------+
```

Higher layers are more expensive but catch subtler issues.

**Cheap layers (bottom):** Run on every change. Fast feedback. Catch obvious problems.

**Expensive layers (top):** Run selectively. Catch what cheap layers miss. Provide the evidence that enables trust.

---

## Verification as Product

### What Reviewers Review

The traditional model: **Review every line of code**

The reviewer reads each change, tries to spot issues, hopes they catch problems. Quality depends on attention span and expertise. Large changes are terrifying.

The AgOps model: **Review evidence that verification happened**

The reviewer checks the evidence table, spots-checks hotspots, verifies the process worked. Quality depends on the verification stack. Large changes are manageable.

### The Shift

| Aspect | Old Model | New Model |
|--------|-----------|-----------|
| Review target | Lines of code | Evidence summary |
| Quality source | Human attention | Verification layers |
| Scaling limit | Reviewer capacity | Machine time |
| Large changes | Terrifying | Manageable |

The code is the binary. The verification is the product.

---

## Cost of Verification

| Layer | Cost | Value |
|-------|------|-------|
| Syntax | Milliseconds | Low (necessary, not valuable) |
| Tests | Seconds-minutes | Medium (proves behavior) |
| Mutation | Minutes-hours | High (proves test quality) |
| Critics | Minutes | High (finds issues early) |
| Evidence | Minimal | High (enables trust) |
| Gates | Seconds | Critical (prevents disasters) |

### Investment Strategy

**Automate cheap layers:** Syntax and tests run on every change. No human involvement.

**Invest in valuable layers:** Mutation and critics are where quality comes from. Spend here.

**Never skip critical layers:** Gates are cheap. The cost of not running them is unbounded.

### The Economics

A change that costs 100K tokens in verification but requires 5 minutes of human review beats a change that costs 10K tokens but requires 60 minutes of human review.

Machine iteration is cheap. Human attention is expensive. Verification converts cheap iteration into expensive confidence.

---

## Verification Debt

Skipping verification layers creates debt:

| Skipped Layer | Debt Created |
|---------------|--------------|
| Tests | Behavioral uncertainty — does it actually work? |
| Mutation | Test theater — tests pass but don't verify |
| Critics | Missed issues — problems escape to production |
| Evidence | Unverifiable claims — "trust me" culture |
| Gates | Unsafe publishes — credentials in repo, broken deploys |

Each skip increases risk. Debt compounds.

### How Debt Accumulates

Skip tests once: "We'll add them later."
Skip tests twice: "We never have time."
Skip tests always: "Testing is too expensive."

The cost of adding tests goes up as the codebase grows. The debt compounds. Eventually, "add tests" becomes a multi-sprint project instead of a few hours.

### Paying Down Debt

The only way out is through:

1. **Stop accruing** — No new code without tests
2. **Invest strategically** — Add tests to high-risk areas first
3. **Use critics** — Identify where debt hurts most
4. **Track evidence** — Know what's verified and what isn't

---

## The Verification Flywheel

Better verification enables a virtuous cycle:

```
Better verification
      |
      v
More trust in changes
      |
      v
More autonomy for agents
      |
      v
Faster iteration
      |
      v
More time for verification
      |
      +---> (loop)
```

**Invest in verification infrastructure. It pays dividends.**

The inverse is also true:

```
Weak verification
      |
      v
Less trust in changes
      |
      v
More human review required
      |
      v
Slower iteration
      |
      v
Less time for verification
      |
      +---> (death spiral)
```

The system either gets better at verification over time, or it collapses back to manual review.

---

## Practical Application

### Every Change

1. **Syntax**: Auto-format, lint (milliseconds)
2. **Tests**: Run relevant test suite (seconds-minutes)
3. **Evidence**: Capture results with pointers (automatic)

### Complex Changes

Add:
4. **Critics**: Author/critic loop until verified (minutes)
5. **Mutation**: Prove tests actually test (optional, targeted)

### Publish Boundaries

Always:
6. **Gates**: Secrets scan, safety checks (seconds)

### The Stack in Receipts

A well-formed receipt shows the stack:

```yaml
## Verification
syntax:
  tool: "eslint + prettier"
  result: "clean"

tests:
  command: "npm test"
  result: "47 passed, 0 failed"
  artifact: ".runs/feat-auth/build/test_execution.md"

critics:
  passes: 2
  final_status: "VERIFIED"
  remaining: "0 CRITICAL, 0 MAJOR, 2 MINOR"

gate:
  secrets_scan: "0 findings"
  status: "PASS"

## Not Measured
- mutation_testing: "not configured"
- integration_tests: "requires external service"
```

Honest about what was verified. Explicit about what wasn't.

---

## The Core Message

**Code is a means, not an end.**

The goal isn't "write code." The goal is "deliver verified capability."

Code that isn't verified is liability, not asset. It might work. It might not. You don't know.

Verified code is an asset. It works. You know. You have evidence.

**The product isn't code. The product is verified artifacts. Code is just what got verified.**

---

## See Also

- [claims-and-evidence.md](claims-and-evidence.md) — Evidence discipline in detail
- [adversarial-loops.md](adversarial-loops.md) — Author/critic opposition
- [code-as-binary.md](code-as-binary.md) — Why we don't read every line
- [economics.md](economics.md) — The ROI math
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
