# Explanation

Understanding-oriented: explore the why behind the pack design.

## Reading Order

**Start here:**
1. [Laws of the Swarm](laws-of-the-swarm.md) — The 10 immutable rules everything derives from
2. [Architecture](architecture.md) — Core philosophy and patterns
3. [What Makes This Different](what-makes-this-different.md) — Assumptions that don't apply here

**Then pick by interest:**

| If you're asking... | Read |
|---------------------|------|
| "What are the immutable rules that govern the pack?" | [Laws of the Swarm](laws-of-the-swarm.md) |
| "Why default-allow for work but gated for publish?" | [Why Ops-First](why-ops-first.md) |
| "How do agents think, decide, and fail gracefully?" | [Agent Philosophy](agent-philosophy.md) |
| "Why do agents read/write disk instead of sharing context?" | [Stateless Execution](stateless-execution.md) |
| "How do agents coordinate without message passing?" | [Coordination by Artifact](coordination-by-artifact.md) |
| "Why route on control blocks, not by re-reading files?" | [Why Two Planes](why-two-planes.md) |
| "Why do GitHub ops need two separate gates?" | [Why Two Gates](why-two-gates.md) |
| "How do publish boundaries make bypassPermissions safe?" | [Boundary Physics](boundary-physics.md) |
| "How does trust emerge from verification, not restrictions?" | [Trust Architecture](trust-architecture.md) |
| "Why does cleanup re-run after sanitization?" | [Why Reseal](why-reseal.md) |
| "What LLM failure modes does this design address?" | [AI Physics](ai-physics.md) |
| "What happens when sources of truth conflict?" | [Truth Hierarchy](truth-hierarchy.md) |
| "Why starve agents of context instead of giving them everything?" | [Context Discipline](context-discipline.md) |
| "How do we prevent agents from making false claims?" | [Claims and Evidence](claims-and-evidence.md) |
| "How does raw LLM output become trusted?" | [Candidates to Artifacts](candidates-to-artifacts.md) |
| "Why is the PR description the primary review interface?" | [PR as Review Surface](pr-as-review-surface.md) |
| "How do critics produce queues that fixers can drain?" | [Worklist Pattern](worklist-pattern.md) |
| "What are the layers of verification and what does each prove?" | [Verification Stack](verification-stack.md) |
| "Why is generation cheap but review expensive?" | [Throughput Inversion](throughput-inversion.md) |
| "What's the ROI math and why does DevLT matter?" | [Economics](economics.md) |
| "What's the mental model for LLMs generating code?" | [Stochastic Compiler](stochastic-compiler.md) |
| "How does AgOps compare to copilots, black boxes, and frameworks?" | [Competitive Positioning](competitive-positioning.md) |
| "How does this repo teach the mental model while operating?" | [Teaching Repo](teaching-repo.md) |
| "When and how should agents involve humans?" | [Human Escalation](human-escalation.md) |

## Terminology Note

The pack uses "planes" in two different contexts:

| Term | Meaning | Explained in |
|------|---------|--------------|
| **Work Plane / Publish Plane** | Where gates engage (publish boundary only) | [Why Ops-First](why-ops-first.md) |
| **Control Plane / Audit Plane** | How routing works (blocks vs files) | [Why Two Planes](why-two-planes.md) |

These are orthogonal concepts—both matter, in different ways.
