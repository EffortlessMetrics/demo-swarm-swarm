# Doctrine

> The immutable beliefs that shape everything else.

---

## The Thesis

**Humans author intent. Swarms manufacture verified change. Reviewers audit the cockpit, not the diff.**

This is not "AI helps you code faster." This is an industrial control loop that makes stochastic generation safe by producing **telemetry you can trust**.

---

## The Triangle (No Compromises)

Three things matter. Optimize all three together:

1. **Minimize human review time** per PR
2. **Maximize PR quality + maintainability**
3. **Maximize wall-clock velocity**

The ratio that matters: **Quality per DevLT** (PR quality achieved per minute of developer attention).

Compute is cheap. Architect attention is expensive. Trade accordingly.

---

## Trust AND Verify

High autonomy is enabled by high verification. Not "trust later." Not "verify forever then trust." **Both, always.**

The system earns trust through evidence, not assertions. An agent's claim does not override an exit code. A receipt without pointers is hope, not proof.

---

## Handcuffs vs Errors

In the work plane, **handcuffs are worse than errors**.

| Mode | What Happens |
|------|--------------|
| **Errors** | Route to fixers, continue, learn |
| **Handcuffs** | Stop-and-ask mid-flow, rigid routing schemas, kill throughput |

Errors are expected. The pipeline handles them. Handcuffs prevent the pipeline from running.

---

## The Stochastic Compiler

The LLM is not a chatbot. It is a **stochastic compiler**.

- **Input:** Natural language specs (BDD, requirements, ADRs)
- **Output:** Implementation code (candidate artifacts)
- **Process:** Non-deterministic, iterative, refinable

Like a traditional compiler, but:
- Probabilistic (same input may vary)
- Fallible (output needs verification)
- Cheap to re-run (iteration is the normal path)

**Trust the pipeline, not the generation.** Individual generations are unreliable. Generate → Verify → Critique → Refine is reliable.

---

## Anti-Austerity (When Value Is Real)

Spend compute when it buys:
- **Clarity** (better evidence, clearer receipts)
- **Maintainability** (well-structured code, useful documentation)
- **Verification depth** (more critic passes, mutation testing)
- **Accessibility** (better PR cockpit, clearer human interfaces)

Reject austerity when there is clear value. Optimize for **Quality : Dev Attention**, not for cheap.

The flywheel: every hour of work moved from human to machine becomes an hour we can make faster tomorrow. Human workflows are hard to optimize. Machine workflows are just code.

---

## The Lineage

This approach draws from:

- **Lean manufacturing / Toyota** — Small stations, sensors, throughput + quality together
- **DevOps** — Automation, observability, tight feedback loops
- **Teal / modern org design** — Autonomy inside clear boundaries; responsibility is local
- **Audit/controls mindset** — Receipts, traceability, evidence hierarchy
- **UX + accessibility engineering** — The PR cockpit is a UI; humans should understand quickly
- **Open source sensibility** — Make it teachable, portable, and kind to the next contributor

---

## The Laws Are Physics

The ten laws in `docs/explanation/laws-of-the-swarm.md` are not preferences. They are the physics that makes the system work. Violating them breaks the system in predictable ways.

When proposing a change, ask:
- Does this violate any law?
- Does this support any law?
- Is the violation justified and documented?

---

## See Also

- [laws-of-the-swarm.md](../../docs/explanation/laws-of-the-swarm.md) — The ten immutable rules
- [economics.md](../../docs/explanation/economics.md) — The math that makes this work
- [stochastic-compiler.md](../../docs/explanation/stochastic-compiler.md) — The compiler mental model
- [why-ops-first.md](../../docs/explanation/why-ops-first.md) — Default-allow philosophy
