# Doctrine

> The immutable beliefs that shape everything else.

---

## The Thesis

**Humans author intent. Swarms manufacture verified change. Reviewers audit the cockpit, not the diff.**

This is not "AI helps you code faster." This is an industrial control loop that makes stochastic generation safe by producing **telemetry you can trust**.

---

## The Assembly Moment

We've reached a transition like 1970s (binary → assembly) or 1990s (assembly → C).

**Now:** Stop writing code. Implementation languages are intermediate bytecode.

The bottleneck has moved. Code generation is solved. **Trust is the bottleneck.** The system exists to manufacture trust, not code.

Everything we learned about software engineering still applies—tests, reviews, architecture, CI/CD. We just have compute that can run the mechanical parts. Your starting point is no longer a blank file. It's a review-ready artifact that's already been through the SDLC.

---

## The New Roles

| Role | What They Do |
|------|--------------|
| **Architect** | Write intent (specs, ADRs, BDD). Define contracts. |
| **PM** | Make judgment calls. Handle NEEDS_HUMAN. Decide scope. |
| **Auditor** | Review evidence. Spot-check hotspots. Authorize merge. |

What humans don't do: write implementation code, debug syntax, review diffs line-by-line.

---

## The Vibe

**We are not Silicon Valley Hype. We are Rust Belt Industrialism.**

- Cold, industrial, deterministic, high-trust
- We don't "chat" with the bot—we operate the refinery
- Not "Copilots" (assistants)—**AgOps** (infrastructure)

See [the-thesis.md](../../docs/explanation/the-thesis.md) for the full story.

---

## The Triangle (No Compromises)

Three things matter. Optimize all three together:

1. **Minimize human review time** per PR
2. **Maximize PR quality + maintainability**
3. **Maximize wall-clock velocity**

The ratio that matters: **Quality per DevLT** (PR quality achieved per minute of developer attention).

Compute is cheap. Architect attention is expensive. Trade accordingly.

---

## The Manufacturing Thesis

**The factory produces trust, not code.**

Code is a byproduct. The actual product is a PR where the review surface is self-evidently trustworthy. The verification panel answers every question the reviewer would have asked.

| Traditional | Swarm |
|-------------|-------|
| Write code | Define intent |
| Hope it's correct | Generate candidates |
| Review catches problems | Verify exhaustively |
| | Ship what passes |

The verification IS the product.

---

## Trust AND Verify

High autonomy is enabled by high verification. Not "trust later." Not "verify forever then trust." **Both, always.**

The system earns trust through evidence, not assertions. An agent's claim does not override an exit code. A receipt without pointers is hope, not proof.

---

## Intent Starts. Evidence Decides.

The causal flow starts from intent:

**Intent → Candidates → Evidence → Decision → Narrative**

When sources conflict, evidence arbitrates:

**Tool outputs → Derived facts → Intent → Implementation → Narrative**

These are two different orderings for two different purposes:
- The first is **how work moves** (forward flow)
- The second is **how disagreements resolve** (arbitration)

Intent artifacts (REQ/BDD/ADR) are the primary change surface. We still edit code when needed, but the system's leverage comes from making intent + evidence the default review surface.

---

## Debugging Is Allowed

"Rerun" is a tool, not a virtue.

Choose the cheapest path to trust:
1. **Tool-output grounded debugging** — compiler errors, failing tests, diff-based blame
2. **Intent edits** — if the bug is spec/contract-level
3. **Surgical code edits** — if it's implementation-level

Agents can and should debug. The goal is trust, not purity.

---

## Completion Semantics

"Done" is a mechanical state, not a feeling.

| Status | Meaning |
|--------|---------|
| `VERIFIED` | Converged. Evidence panel green, evidence fresh, blockers empty. |
| `UNVERIFIED` | Not converged, but checkpointed. Artifacts written, state captured, resumable. |
| `CANNOT_PROCEED` | Mechanical failure. Tooling broken, permissions missing, infra down. |

**UNVERIFIED is not failure. It's honest state.**

### Flows Run to Completion

Flows never stop mid-execution. They run to the boundary and checkpoint.

- **VERIFIED** = Evidence says done (panel green, evidence fresh, blockers empty)
- **UNVERIFIED** = External constraint hit (artifacts written, state captured, resumable)

The only external constraints (all rare):

| Constraint | What It Means | How Rare |
|------------|---------------|----------|
| **Budget** | Tokens, time, or CI minutes exhausted | Occasional |
| **Access** | Tooling broken, permissions missing, infra down | Rare |
| **Authority** | Business decision requiring human judgment | Very rare |

**Everything else is routing.** Stagnation → route differently. Oscillation → break the cycle. Failure → route to fixer. "Blocked" is almost always just "route to the right agent."

Counts are not exit criteria. "3 tries" means run it again. Stagnation triggers rerouting, not stopping.

---

## The Physics

These are non-negotiable invariants. Violating them breaks the system.

1. **Mechanical Truth** — Trust tool outputs over agent narratives. Exit codes and counts, not claims.
2. **Schema Gravity** — The flow structure pulls outputs into alignment. Each step has expectations that constrain the next.
3. **Shadow Fork** — .runs/ is a sandbox. Nothing graduates until gate passes.
4. **Throughput Inversion** — Verification arbitrage: burn cheap compute to buy back expensive attention.
5. **Adversarial Pressure** — Single agents lie to please. Two agents fighting surfaces truth.
6. **Scoped Context** — Short focused threads cost less. Pay only for relevant tokens.

See [the-physics.md](../../docs/explanation/the-physics.md) for full explanations.

---

## Handcuffs vs Errors

In the work plane, **handcuffs are worse than errors**.

| Mode | What Happens |
|------|--------------|
| **Errors** | Route to fixers, continue, learn |
| **Handcuffs** | Stop-and-ask mid-flow, rigid routing schemas, kill throughput |

Errors are expected. The pipeline handles them. Handcuffs prevent the pipeline from running.

---

## Smooth is Fast

**Don't stop mid-flow to ask permission. Let the swarm run to completion.**

- Don't ask "is this plan okay?" — run the plan, gate catches problems
- Don't ask "should I proceed?" — proceed, evidence shows if it worked
- Don't wait for approval between steps — flow boundaries are where humans engage

Velocity comes from smooth execution, not from being clever. Long, simple chains of atomic operations beat short, complex ones.

---

## The Flywheel

**Machine workflows are code. Human workflows are not.**

Every hour of work moved from human to machine is an hour we can optimize:
- Profile it
- Parallelize it
- Cache it
- Make it faster

Human workflows resist optimization. Machine workflows are just code.

```
Today: 8 hrs machine time
Optimize critic loop → 6 hrs
Parallelize testing → 4 hrs
Smarter caching → 2 hrs
```

The more we push to machine, the faster everything gets. Human time doesn't compress like that.

---

## What Changes at Scale

When trust is manufactured (not hoped for), the shape of work changes:

| Old Constraint | New Reality |
|----------------|-------------|
| PRs must be small to be reviewable | PR size is independent of review time |
| Large changes require many review cycles | Evidence quality determines review time |
| Refactors are risky and avoided | Refactors are verified like any other change |
| "Too big to review" is a blocker | Review the cockpit, spot-check hotspots |

This isn't about doing more. It's about the relationship between change size and review burden no longer being linear.

---

## The Skill Shift

Implementation code is generated output. Your attention shifts:

| Less | More |
|------|------|
| Reading implementation line-by-line | Reading evidence panels |
| Debugging syntax | Refining intent |
| Code ownership ("my code") | Intent ownership ("my spec") |
| Reviewing diffs | Reviewing cockpits |

**No one "wrote" the implementation.** Just like no one wrote the assembly under your C. The work you take pride in is the spec, the architecture, the decisions—not the syntax.

**The codebase is a mold.** Existing patterns shape new generation via schema gravity. Good codebase = good generations. Refactoring the mold improves all future output.

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

## Anti-Austerity (The Laziness Tax Is Gone)

We always knew we should write comprehensive tests, document everything, run mutation testing. We didn't because human time was expensive. We cut corners out of economic rationality.

Now the cost is immaterial. **Do it right.**

Spend compute on:
- **Verification depth** (mutation testing, multiple critic passes)
- **Documentation** (generated alongside code, always current)
- **Evidence quality** (clear receipts, comprehensive coverage)
- **Review surfaces** (better cockpit, clearer hotspots)

The discipline was always correct. We just couldn't afford it. Now we can.

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

The eleven laws in `docs/explanation/laws-of-the-swarm.md` are not preferences. They are the physics that makes the system work. Violating them breaks the system in predictable ways.

When proposing a change, ask:
- Does this violate any law?
- Does this support any law?
- Is the violation justified and documented?

---

## See Also

- [laws-of-the-swarm.md](../../docs/explanation/laws-of-the-swarm.md) — The eleven immutable rules
- [economics.md](../../docs/explanation/economics.md) — The math that makes this work
- [stochastic-compiler.md](../../docs/explanation/stochastic-compiler.md) — The compiler mental model
- [why-ops-first.md](../../docs/explanation/why-ops-first.md) — Default-allow philosophy
