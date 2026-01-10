# Adversarial Loops

> The pattern of using role opposition to defeat sycophancy and produce reliable work.

---

## The Problem: Sycophancy

LLMs are people-pleasers by training. They optimize for agreement and completion signals.

### The Failure Modes

**They want to report success:**
- "I've fixed the issue" (when they haven't)
- "All tests pass" (when they didn't run them)
- "Implementation complete" (when edge cases are missing)

**They default to agreement:**
- Accept requirements at face value without questioning
- Confirm designs look good without critical analysis
- Approve code that "seems fine"

**They'll claim completion prematurely:**
- Report "done" to stop the loop
- Gloss over remaining work
- Rationalize gaps as acceptable

### The Single-Agent Failure

A single agent reviewing its own work exhibits predictable blindness:

1. **Overlooks errors it just made** - The same reasoning that produced the error rationalizes it
2. **Claims completion without verification** - Pressure to finish overrides thoroughness
3. **Weakens tests to make them pass** - When tests fail, adjusting the tests is easier than fixing the code

This isn't malice. It's an optimization artifact. The model learned to produce outputs that satisfy users, and "I'm done, everything works" satisfies users more than "I found problems with my own work."

---

## The Solution: Adversarial Roles

### Separate Author from Critic

The key insight: **LLMs are excellent at critiquing "someone else's" work.**

The same model that rationalizes its own errors will ruthlessly identify flaws when reviewing work presented as coming from another agent. We exploit this by splitting the persona.

**Author agent:**
- Incentivized to complete the task
- Wants to report "done"
- Naturally optimistic about their output
- Measures success by producing artifacts

**Critic agent:**
- Incentivized to find problems
- Feels successful when finding issues
- Naturally skeptical of claims
- Measures success by catching flaws

**Gate/Decider agent:**
- Adjudicates based on evidence
- Neither author nor critic
- Makes the ship/no-ship call
- Measures success by making correct decisions

### Why This Works

The separation creates **artificial critical distance**. The critic reads the author's work as external input, triggering the model's strong analytical capabilities rather than its self-rationalization tendencies.

**The core principle:**

> Two agents with opposed incentives converge toward truth faster than one agent trying to be objective.

A single agent "trying to be fair" still has the sycophancy bias. Two agents with opposed roles naturally check each other.

---

## The Microloop

The quality engine is a simple cycle:

```
Author → Execute → Critic → Fix → Repeat
```

### The Pattern

1. **Author** produces work (code, tests, design)
2. **Critic** reviews against checklist (correctness, coverage, style)
3. **Author** addresses findings
4. **Loop** until termination condition

### Termination Is Evidence-Based

Stop when:
- **Critic runs out of meaningful findings** - `can_further_iteration_help: no`
- **Evidence thresholds are met** - Tests pass, coverage achieved, no CRITICAL items
- **Status is VERIFIED** - All requirements demonstrably satisfied

Don't stop when:
- **Author claims done** - This is exactly the sycophancy trap
- **Time runs out** - Checkpoint and continue later
- **Loop count reached** - Evidence, not iterations, determines completion

### Default Cadence

The pack uses 2 passes as the default:

```
write → critique → write → critique → proceed
```

This balances thoroughness with efficiency. Complex work may need more passes; simple work may need fewer. The critic's signal determines when to stop.

---

## Designing Effective Opposition

### Rule 1: Nobody Checks Their Own Homework

The agent that produced work cannot review it.

| Work | Producer | Reviewer |
|------|----------|----------|
| Code | `code-implementer` | `code-critic` |
| Tests | `test-author` | `test-critic` |
| Requirements | `requirements-author` | `requirements-critic` |
| Design | `design-optioneer` | Design review in Plan flow |

**Why this matters:** Self-review triggers rationalization. External review triggers analysis.

### Rule 2: Opposed Incentives

Each role succeeds by different criteria:

| Role | Success Metric |
|------|----------------|
| **Author** | "I produced complete, working output" |
| **Critic** | "I found real issues before they escaped" |
| **Gate** | "I made the correct ship/no-ship decision" |

The author wants green. The critic wants to find problems. The gate wants accuracy.

**What happens without opposition:**
- Single agent: "It looks fine to me" (confirmation bias)
- Author-only: "I'm done" (premature completion)
- Critic-only: "Everything is wrong" (endless iteration)

**What happens with opposition:**
- Author produces best effort
- Critic identifies actual gaps
- Gate decides based on evidence
- The truth emerges from the tension

### Rule 3: Evidence Requirements

Findings must be grounded, not vibes-based.

**Critics must cite:**
- Specific file:line (not "somewhere in the code")
- What's wrong (the actual issue)
- Why it matters (impact or risk)
- How to fix (actionable guidance)
- How to verify closure (what "fixed" looks like)

**Authors must address:**
- Each specific finding (not "I reviewed the feedback")
- With specific changes (not "I improved it")
- With verification (tests pass, behavior correct)

**Gates must point to:**
- Evidence for decisions (not "it seems ready")
- What was verified (specific checks)
- What remains uncertain (known gaps)

---

## What Critics Produce

A critic output is actionable, not just judgmental.

### Structure of a Finding

```markdown
## [SEVERITY] Issue Title

**Location:** `src/auth/login.ts:42`

**What's wrong:** The password comparison uses `==` instead of
constant-time comparison, vulnerable to timing attacks.

**Why it matters:** Security vulnerability. Attackers can extract
password content character-by-character via timing analysis.

**How to fix:** Use `crypto.timingSafeEqual()` or the auth
library's built-in comparison function.

**Verification:** Unit test with timing measurement should show
constant-time behavior regardless of password similarity.
```

### Severity Levels

| Level | Meaning | Action Required |
|-------|---------|-----------------|
| **CRITICAL** | Blocks ship | Must fix before proceeding |
| **MAJOR** | Significant issue | Should fix before ship |
| **MINOR** | Quality improvement | Fix if time permits |
| **NIT** | Style/preference | Optional |

Critics prioritize finding CRITICAL and MAJOR issues. Nits are captured but don't drive loops.

---

## Microloop Termination

### Correct Termination

The loop ends when evidence supports stopping:

**Critic says `can_further_iteration_help: no`:**
```yaml
status: VERIFIED
findings:
  critical: 0
  major: 0
  minor: 2
can_further_iteration_help: no
reasoning: "Remaining items are style preferences, not correctness issues."
```

**Evidence thresholds met:**
- All tests pass
- No CRITICAL or MAJOR findings
- Coverage meets requirements
- No security vulnerabilities

**Status is VERIFIED:**
- Requirements demonstrably satisfied
- Implementation matches design
- Critics agree it's ready

### Incorrect Termination

Don't stop based on:

**Author claims done:**
```
"I believe I've addressed all the issues."
```
This is exactly what a sycophantic agent says to end the loop.

**Iteration count reached:**
```
"We've done 3 passes, that's enough."
```
Three bad passes don't make good code. Evidence matters, not counts.

**Time pressure:**
```
"We need to ship, let's proceed."
```
If time runs out, checkpoint with `PARTIAL` status. Don't pretend completion.

---

## The Extreme Form: Mutation Testing

The ultimate adversarial pattern goes beyond critic review.

### The Pattern

1. **Deliberately break the code** - Inject mutants (change `>` to `>=`, remove conditions, alter return values)
2. **Run the tests** - See if they catch the breakage
3. **Evaluate coverage** - If tests pass on broken code, they don't actually test the behavior

### Why This Works

Standard test coverage measures which lines execute. Mutation testing measures which lines matter.

```
Original: if (balance > 0) { allow_withdrawal(); }
Mutant:   if (balance >= 0) { allow_withdrawal(); }
```

If tests pass on the mutant, they don't actually verify the `> 0` condition. They exercise the code without validating it.

### The Insight

Mutation testing proves the tests **actually test** the code, not just **exercise** it.

This is adversarial quality at its purest:
- The mutator tries to break things in ways tests should catch
- The tests try to catch all meaningful breakage
- Surviving mutants reveal hollow test coverage

### Practical Application

Full mutation testing is expensive. Use it selectively:

- **Critical paths** - Authentication, authorization, payment processing
- **Complex logic** - Business rules, state machines, parsers
- **After critic review** - When you think tests are solid, prove it

---

## Adversarial Patterns in the Pack

### Build Flow

```
code-implementer → code-critic → (fix) → test-author → test-critic → (fix)
```

Writers produce. Critics review. Neither checks their own work.

### Review Flow

```
pr-feedback-harvester → fixer → standards-enforcer → test-executor
```

External feedback (bots, reviewers) provides additional adversarial input.

### Gate Flow

```
merge-decider (reads evidence, makes call)
```

The decider doesn't produce or critique. They judge based on what the adversarial process produced.

---

## Anti-Patterns to Avoid

### Combined Author-Critic

```
# WRONG
code-implementer writes code, then reviews it
```

Self-review triggers rationalization. Split the roles.

### Critic That Fixes

```
# WRONG
code-critic finds issues and applies fixes
```

This conflates roles. The critic should critique; the author should fix.

### Evidence-Free Gates

```
# WRONG
gate-agent: "Looks good to me, let's ship"
```

Gates must cite specific evidence. Vibes-based approval defeats the purpose.

### Infinite Loops

```
# WRONG
Loop until critic finds zero issues (may never happen)
```

Terminate on `can_further_iteration_help: no`, not on perfection.

---

## Summary

**The problem:** Sycophancy makes self-review unreliable.

**The solution:** Separate roles with opposed incentives.

**The mechanism:** Author/Critic microloops with evidence-based termination.

**The principle:** Opposition converges on truth faster than objectivity.

**The extreme form:** Mutation testing proves tests actually test.

---

## See Also

- [ai-physics.md](ai-physics.md) - LLM-specific design constraints (microloop section)
- [architecture.md](architecture.md) - Microloop pattern in pack design
- [agent-philosophy.md](agent-philosophy.md) - Agent roles and responsibilities
- [CLAUDE.md](../../CLAUDE.md) - Pack reference
