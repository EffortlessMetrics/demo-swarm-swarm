# The Physics

> The mechanical laws that constrain non-deterministic compute.

These are not suggestions. These are the constraints enforced by the orchestrator to make stochastic generation safe. Violating them breaks the system in predictable ways.

---

## The Core Insight

LLMs are **high entropy** — creative, drifting, hallucinating. Left unconstrained, they expand like gas to fill available context, inventing libraries, changing variable names, rewriting things "because they felt like it."

Engineering requires **low entropy** — deterministic, verifiable, bounded.

The physics are the forces that pull chaotic LLM output into alignment with strict, verifiable structures.

---

## Physics 1: Mechanical Truth

**Trust tool outputs over agent narratives.**

When a tool runs, it returns actual results — exit codes, stdout, structured output. That's what happened. When an agent describes what happened, that's interpretation.

The distinction matters because agents under completion pressure will claim success without evidence. "I ran the tests and they pass" is a narrative. The test runner's actual output is truth.

### The Hierarchy

When sources conflict, trust flows downward:

```
1. Tool outputs (exit codes, stdout)  — What actually happened
2. Derived facts (counts, parses)     — Mechanical extraction from outputs
3. Intent (ADR/BDD)                   — What we meant to build
4. Implementation (code)              — What we actually built
5. Narrative (agent chat)             — Interpretation, useful for reasoning
```

### What It Implies

| Constraint | Why |
|------------|-----|
| Cite tool outputs in receipts | "15 passed, 0 failed" from the actual test run |
| Claims require evidence | Point to the artifact, not just assert |
| "Not measured" is acceptable | Better than false certainty |

### Example

```
# Narrative (don't trust alone):
Agent: "I ran the tests and they all pass."

# Tool output (trust this):
test-runner returned:
  exit_code: 0
  passed: 15
  failed: 0
  output: "tests/test_auth.py ... PASSED"

# Receipt cites the tool output, not the narrative
```

---

## Physics 2: Schema Gravity

**The flow structure itself pulls outputs into alignment.**

Schema Gravity isn't just compilers and linters. It's the swarm's design. Running through the flows naturally shapes outputs toward schema alignment because each step has expectations that constrain the next.

### The Gravity Sources

| Source | Pull |
|--------|------|
| **Flow 1 requires BDD** | Code must satisfy scenarios or gate fails |
| **Flow 2 requires ADR** | Implementation must align with recorded decisions |
| **Flow 3 requires receipts** | Must produce evidence with specific fields |
| **Critics require findings format** | Issues become trackable worklist items |
| **Gates require evidence** | If it wasn't measured, it didn't happen |
| **Compiler/linter** | Code must actually work in the existing codebase |

### How It Works

The swarm's flow is designed so things fall into schema alignment just from running through it.

Flow 1 produces BDD scenarios → Flow 3 must satisfy them or critic catches it → Gate checks coverage.

Flow 2 produces ADR → Implementation must match or design-critic flags drift → Traceability audit verifies alignment.

Each artifact has expected structure → Agents produce what's expected → Cleanup agents verify completeness.

The agent doesn't need to be "told" to align. The flow makes misalignment visible and routes it to fixers.

### Emergent Behavior: Structural Mimicry

Because the agent is constantly pulled toward existing patterns (by the compiler, by critics comparing to codebase conventions, by the existing architecture), it mimics the repo's style.

Not just syntax — **architecture**. If the repo uses a Result pattern, the agent uses it. Not because it was trained on it, but because schema gravity makes it the path of least resistance.

### Why This Enables Scale

- **Low Gravity (Chatbot):** Ask for 100k lines, entropy increases with every line. By line 5,000, it's forgotten line 1. Collapses into noise.
- **High Gravity (Swarm):** Line 100,000 is subject to the same pull as Line 1. Must satisfy BDD. Must pass critics. Must fit receipts. The flow structure maintains coherence.

---

## Physics 3: Shadow Fork (Blast Radius)

**Handcuffs kill velocity. Default-allow inside the sandbox.**

Agents operate in a hermetic `.runs/<run-id>/` directory — the Shadow Fork. Inside, the agent has god-mode. It can delete, refactor, break things. We accept the messiness of creation because the Gate prevents the mess from escaping.

### The Separation

| Zone | Posture | What Happens |
|------|---------|--------------|
| **Inside sandbox** | Default-allow | Read any file, write any code, run any test, iterate freely |
| **At publish boundary** | Gated | Stage, sanitize, persist (only if clean) |

### The Boundary Order

```
1. Stage    — Define the surface (what would be published)
2. Sanitize — Scan the staged surface (secrets, anomalies)
3. Persist  — Commit/push only if safe
```

This order is load-bearing. Scanning an unstaged surface creates a TOCTOU gap.

### What It Implies

| Constraint | Why |
|------------|-----|
| No permission checks inside the sandbox | Permission theater kills throughput |
| Publish boundaries are the only real gates | Commit, push, GitHub post — that's where risk lives |
| Accept iteration messiness | The gate catches problems, not mid-flow checks |

---

## Physics 4: Throughput Inversion

**Generation capacity exceeds human review capacity by orders of magnitude.**

Don't make humans read more. Make the system prove more.

### The Economics

| Resource | Reality |
|----------|---------|
| Tokens | Fractions of a cent. 60+ tokens/second. Effectively free. |
| Attention | Senior dev hours. Most scarce resource. |

**Verification Arbitrage:** Burn infinite cheap compute to buy back expensive human attention.

We don't care if the AI generates 500,000 lines of garbage to produce 100,000 lines of gold. The garbage costs nothing. The gold is verified.

### The Strategy

```
Old: Generate once → Debug if wrong → Review every line → Human runs tests
New: Generate many → Pick best → Review evidence summaries → Machine proves, human reads results
```

### The Glass Cockpit

Reviewing 100,000 lines line-by-line is impossible. We review telemetry:

| Sensor | Threshold |
|--------|-----------|
| Intent | 100% BDD scenario coverage |
| Integrity | 95% mutation score |
| Health | Complexity delta within budget |

If sensors are green and intent (ADR) is valid, we merge. The code is treated as compiled binary.

### What It Implies

| Constraint | Why |
|------------|-----|
| Review cost is O(1), not O(n) | A 2000-line PR takes the same review time as 200 lines if evidence is good |
| The PR cockpit is the product | Most reviewers read only the description |
| Verification loops are investments | More critic passes, mutation testing — cheap machine time buys confidence |

---

## Physics 5: Adversarial Pressure

**Single agents lie to please. Two agents fighting surfaces truth.**

We don't train models to be "honest." We build systems where dishonesty is impossible.

### The Mechanism

The **Author** (who wants to finish) is pitted against the **Critic** (who wants to find bugs). The system doesn't proceed until the Critic runs out of ammunition.

| Role | Incentive | Success |
|------|-----------|---------|
| Author | Complete the task | "I produced working output" |
| Critic | Find problems | "I caught real issues" |
| Gate | Make correct decision | "I made the right call on evidence" |

### What It Implies

| Constraint | Why |
|------------|-----|
| Critics never fix | Conflict of interest. If you fix your own findings, you stop finding. |
| Worklists are drainable | Finite, specific items. Each has file:line and verification condition. |
| "Green CI" is one signal | CI passing means tests ran. Mutation testing proves tests actually test. |

### The Microloop

```
Author → Critic → Fix → Repeat until Critic satisfied
```

Don't stop when:
- Author claims done (sycophancy trap)
- Iteration count reached (evidence matters, not counts)

---

## Physics 6: Scoped Context (Cost Performance)

**Short, focused threads cost less.**

Context is not "knowledge" — context is "cost." Token costs scale with context. Irrelevant context is waste.

### The Economics

```
Bad:  Agent 1 (2K) → Agent 2 (10K) → Agent 3 (25K) → Agent 4 (50K)
      Total: ~87K tokens carried, mostly irrelevant

Good: Agent 1 (2K) → Agent 2 (3K) → Agent 3 (4K) → Agent 4 (3K)
      Total: ~12K tokens, all relevant
```

The savings compound. Over a full run with dozens of agent calls, scoped context is 10-50x cheaper.

### The Mechanism

Every atomic task spins up a fresh context. The Curator generates a manifest that hydrates the worker with *only* the 3-5 files relevant to the task.

### What It Implies

| Constraint | Why |
|------------|-----|
| Artifacts must be self-contained | So you can load just what you need |
| Handoffs should be tight | Pass relevant files and summary, not "read the whole thread" |
| Disk is source of truth | Files are cheaper to re-read than context is to carry |

**Note:** The quality impact of irrelevant upthread context exists but isn't catastrophic. The cost impact is significant and compounds.

---

## When Physics Are Violated

| Physics Violated | What Breaks |
|------------------|-------------|
| Mechanical Truth | Process confabulation, false completions, phantom confidence |
| Schema Gravity | Drift, scope creep, code that "looks right" but doesn't compile |
| Shadow Fork | Permission theater, velocity collapse, unsafe autonomy |
| Throughput Inversion | Review bottleneck, O(n) human cost, unsustainable scaling |
| Adversarial Pressure | Sycophantic completion, hollow tests, unverified claims |
| Scoped Context | Cost explosion, paying for irrelevant tokens |

---

## The Meta-Point

These physics answer: **How do you make LLM-generated code trustworthy?**

Not by hoping. Not by prompting harder. Not by asking "are you sure?"

By architecture:
1. Force mechanical verification (kernel returns integers)
2. Pull output toward schemas (gravity constrains drift)
3. Contain blast radius (safe autonomy inside, gates at boundaries)
4. Invert the bottleneck (machine proves, human reviews evidence)
5. Use opposition (two agents find truth faster than one)
6. Scope context (pay only for relevant tokens)

---

## See Also

- [laws-of-the-swarm.md](laws-of-the-swarm.md) — The ten immutable rules
- [economics.md](economics.md) — Verification arbitrage in detail
- [truth-hierarchy.md](truth-hierarchy.md) — The 5-layer epistemology
- [boundary-physics.md](boundary-physics.md) — Stage, Sanitize, Persist
- [adversarial-loops.md](adversarial-loops.md) — Author/Critic patterns
