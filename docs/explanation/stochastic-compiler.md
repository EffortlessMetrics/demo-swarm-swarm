# Stochastic Compiler

> The LLM is a compiler. A stochastic, fallible, tweakable compiler. Treat it like one.

---

## The Mental Model

An LLM is not a chatbot. It's a **stochastic compiler**.

- **Input:** Natural language specs (BDD, requirements, ADRs)
- **Output:** Implementation code (Rust, Python, TypeScript)
- **Process:** Non-deterministic, tweakable, iterative

Like a traditional compiler, but:
- Probabilistic (same input may produce different output)
- Refinable (you can adjust the output through iteration)
- Fallible (output may be wrong and needs verification)

This mental model changes everything about how you use LLMs for development.

---

## Traditional vs Stochastic Compilation

### Traditional Compiler

```
Source (C) --> Compiler (gcc) --> Binary (x86)
```

Properties:
- **Deterministic:** Same input always produces same output
- **Correct by construction:** If it compiles, syntax is valid
- **No refinement:** Output is what it is

The contract is simple: give it valid C, get valid x86. The compiler doesn't guess. The compiler doesn't hallucinate. The compiler doesn't have good days and bad days.

### Stochastic Compiler

```
Spec (BDD) --> Compiler (Swarm) --> Implementation (TS)
```

Properties:
- **Probabilistic:** Same input may vary
- **Possibly wrong:** Must verify
- **Refinable:** Iterate until correct

The contract is different: give it specs, get a candidate implementation. The implementation might be wrong. The implementation might be different next time. But you can iterate until it's right.

---

## The Refinement Loop

Because the compiler is stochastic, we add a refinement loop:

```
Spec --> Generate --> Verify --> Critique --> Refine --> Verify --> ... --> Ship
```

Each iteration:
1. **Generate:** Produce candidate implementation
2. **Verify:** Run tests, check constraints
3. **Critique:** Find issues, suggest fixes
4. **Refine:** Apply fixes, regenerate

Continue until:
- Verification passes
- Critics run out of issues
- Evidence threshold met

This is the fundamental difference from traditional compilation. You don't expect the first output to be correct. You expect to iterate.

---

## Why "Stochastic" Matters

### Embrace Non-Determinism

The same spec might produce:
- Different variable names
- Different helper functions
- Different implementation strategies

This is fine. What matters is:
- Does it pass the tests?
- Does it meet the spec?
- Is it maintainable?

Implementation diversity is a feature, not a bug. Multiple valid solutions exist for any spec. The compiler picks one.

### Plan for Failure

Every generation might be wrong:
- Hallucinated imports
- Incorrect API usage
- Missing edge cases

That's why verification is mandatory, not optional. The compiler is not trusted. The pipeline is trusted.

### Iteration Is Cheap

If the first attempt is wrong, regenerate. Tokens are cheap. The refinement loop catches errors that would otherwise reach humans.

| Resource | Cost |
|----------|------|
| Token generation | Cheap (fractions of a cent) |
| Machine verification | Cheap (seconds) |
| Human review of bad code | Expensive (hours) |
| Bug in production | Very expensive |

Spend machine cycles to save human cycles. That's the trade.

---

## The Compiler Pipeline

The swarm implements a complete compilation pipeline:

### Stage 1: Frontend (Signal)

Parse the input:
- Problem statement
- Requirements (REQ/NFR markers)
- BDD scenarios

**Output:** Validated, structured intent

The frontend catches ambiguity early. Vague requirements fail here, not in implementation.

### Stage 2: Middle-End (Plan)

Optimize and design:
- Architecture decisions (ADR)
- API contracts
- Work breakdown

**Output:** Implementation plan

The middle-end makes structural decisions. How will components interact? What patterns apply? What order of implementation?

### Stage 3: Backend (Build)

Generate code:
- Implementation
- Tests
- Documentation

**Output:** Candidate artifact

The backend is where generation happens. But generation alone is not enough.

### Stage 4: Linker (Gate)

Combine and verify:
- Run tests
- Check contracts
- Verify evidence

**Output:** Verified artifact (or feedback for refinement)

The linker is where trust is earned. Without verification, generation is just speculation.

---

## Tuning the Compiler

Unlike traditional compilers, you can tune stochastic compilers:

### Prompt Tuning

Adjust how the compiler interprets specs:

| Adjustment | Effect |
|------------|--------|
| More explicit examples | More consistent output |
| Clearer constraints | Fewer hallucinations |
| Better context | More accurate generation |

The prompt is your compiler flags. `-O3` for optimization becomes "ensure minimal allocations" in your context.

### Temperature/Sampling

Control randomness:

| Setting | Effect |
|---------|--------|
| Lower temperature | More deterministic, less creative |
| Higher temperature | More varied, potentially more novel |

For implementation tasks, lower is usually better. For design exploration, higher can help.

### Iteration Budget

Control refinement:

| Budget | Trade-off |
|--------|-----------|
| More iterations | Higher quality, diminishing returns |
| Fewer iterations | Faster, possibly rougher |

The default cadence (write, critique, write, critique) balances speed and quality.

### Critic Strength

Control the quality bar:

| Critic | Effect |
|--------|--------|
| Stricter critics | Fewer issues escape, more iterations |
| Lighter critics | Faster iteration, more issues may pass |

Tune based on risk. Security-critical code gets stricter critics. Formatting changes get lighter critics.

---

## The Verification Imperative

Traditional compilers guarantee syntactic correctness. Stochastic compilers guarantee nothing.

So verification is mandatory:

| Verification Type | What It Proves |
|-------------------|----------------|
| Tests | Behavior matches expectations |
| Critics | Code meets quality standards |
| Mutation testing | Tests actually verify behavior |
| Evidence | Claims about verification are true |

**No verification = no trust.**

The compiler output is a candidate until verified. Verification is what transforms speculation into trusted artifact.

---

## Mental Model Implications

### Don't Read the Assembly

You don't read x86 to understand C code. You don't read generated TypeScript to understand a BDD spec.

Read:
- The spec (what you intended)
- The evidence (what was verified)
- The hotspots (where to spot-check)

The implementation is the binary. The spec is the source. Focus on source.

### Treat Output as Provisional

Generated code is a candidate, not a finished product. It becomes trusted only after:
- Tests pass
- Critics approve
- Gates clear

Until then, it's speculation. Plausible speculation, but speculation.

### Embrace Regeneration

Bad output? Regenerate. Different approach? Regenerate. The compiler is cheap to run.

Traditional development has high switching costs. You wrote 500 lines; you're invested. Stochastic compilation has low switching costs. The machine wrote 500 lines; regenerate.

### Trust the Pipeline, Not the Generation

Individual generations are unreliable. The pipeline (generate, verify, critique, refine) is reliable.

```
Generation: Unreliable (any single output may be wrong)
     +
Verification: Reliable (tests catch failures)
     +
Iteration: Reliable (refinement converges)
     =
Pipeline: Reliable (verified output or explicit failure)
```

Trust emerges from the process, not the model.

---

## The Paradigm Shift

### Old Mental Model

"AI writes code for me to review"

- Human reviews every line
- Human catches AI mistakes
- Human is the verification layer
- AI is a typing assistant

### New Mental Model

"AI compiles specs into verified artifacts"

- Specs define intent
- Machine generates candidates
- Machine verifies candidates
- Human reviews evidence

The human role shifts from:
- Reviewing generated code (expensive, error-prone)
- To: Writing good specs and reviewing evidence (high-leverage, focused)

### What This Means in Practice

| Old Approach | New Approach |
|--------------|--------------|
| Read 500 lines of generated code | Read 5-line spec, check evidence table |
| Hope to spot bugs by reading | Run tests to prove absence of bugs |
| Trust because you reviewed | Trust because evidence exists |
| Slow, exhausting, incomplete | Fast, sustainable, thorough |

---

## The Compiler Analogy Applied

### When Things Go Wrong

**Compiler error in C:**
```
error: undeclared identifier 'foo'
    |
 12 |     printf(foo);
    |            ^^^
```

You don't debug the x86 output. You fix the C source.

**Verification failure in swarm:**
```
Test failed: login_with_valid_credentials
Expected: JWT token returned
Actual: 401 Unauthorized
```

You don't debug the generated TypeScript. You check:
1. Is the spec correct?
2. Is the test correct?
3. Does the implementation need regeneration?

### When Things Are Ambiguous

**C compiler:** Flags with warnings. You clarify.

**Stochastic compiler:** Generates something plausible. You verify. If wrong, refine the spec.

The difference: C compilers refuse ambiguity. Stochastic compilers try to resolve it. Verification catches when they resolve it wrong.

### When Output Is Wrong

**C compiler:** Bug in compiler. Report it.

**Stochastic compiler:** Normal operation. Iterate.

Wrong output is not failure. Wrong output with no iteration path is failure.

---

## Practical Application

### Writing Good Specs

Good compiler input produces good compiler output.

**Bad spec:**
```
Make the login work better.
```

**Good spec:**
```
REQ-001: Login endpoint accepts email/password, returns JWT with 24h expiry
REQ-002: Invalid credentials return 401 with error message
REQ-003: Rate limit: 5 failed attempts per email per 15 minutes
```

The more precise the spec, the more constrained the generation, the more likely a correct first output.

### Interpreting Failures

When verification fails:

1. **Check the spec** — Is the requirement clear?
2. **Check the test** — Is the test correct?
3. **Check the generation** — Did the compiler misinterpret?
4. **Iterate** — Refine and regenerate

Failures are information. They narrow the search space.

### Knowing When to Stop

Stop when:
- All tests pass
- Critics have no major findings
- Evidence threshold is met

Don't stop because:
- "It looks right" (verification beats intuition)
- "I'm tired of iterating" (machine doesn't tire)
- "It's probably fine" (probably is not evidence)

---

## The Core Insight

> The LLM is a compiler. A stochastic, fallible, tweakable compiler. Treat it like one.

This means:
- **Accept non-determinism** — Different runs may differ
- **Require verification** — Compilation doesn't imply correctness
- **Embrace iteration** — Refinement is the normal path
- **Trust the pipeline** — Not the individual generation

The stochastic compiler is not magic. It's machinery. Understood as machinery, it becomes powerful. Misunderstood as magic, it disappoints.

---

## See Also

- [code-as-binary.md](code-as-binary.md) — Implementation as output, specs as source
- [adversarial-loops.md](adversarial-loops.md) — Author/critic refinement pattern
- [claims-and-evidence.md](claims-and-evidence.md) — Verification discipline
- [ai-physics.md](ai-physics.md) — Design constraints for LLM systems
- [what-makes-this-different.md](what-makes-this-different.md) — Assumptions that don't apply here
