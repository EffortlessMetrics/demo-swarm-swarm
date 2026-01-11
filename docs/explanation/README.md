# Explanation

These documents explain *why* things work the way they do. Understanding-oriented: explore the reasoning behind the pack design.

---

## Start Here

| Question | Document |
|----------|----------|
| "What is this? Why does it exist?" | [The Thesis](the-thesis.md) |
| "What's the core philosophy?" | [Doctrine](../../.claude/rules/00-doctrine.md) |
| "What are the constraints that make it work?" | [The Physics](the-five-physics.md) |
| "What are the immutable rules?" | [Laws of the Swarm](laws-of-the-swarm.md) |
| "How does the swarm make decisions?" | [Agent Philosophy](agent-philosophy.md) |
| "What makes this different?" | [What Makes This Different](what-makes-this-different.md) |

---

## How It Works

| Question | Document |
|----------|----------|
| "How do agents think, decide, and fail gracefully?" | [Agent Philosophy](agent-philosophy.md) |
| "How is the swarm organized?" | [Org Design as Code](org-design-as-code.md) |
| "How do flows compose?" | [Flow Composition](flow-composition.md) |
| "What's the operating model?" | [Operating Model](operating-model.md) |
| "Why do agents read/write disk instead of sharing context?" | [Stateless Execution](stateless-execution.md) |
| "How do agents coordinate without message passing?" | [Coordination by Artifact](coordination-by-artifact.md) |
| "Why route on prose handoffs, not by parsing structured blocks?" | [Why Two Planes](why-two-planes.md) |
| "How do critics produce queues that fixers can drain?" | [Worklist Pattern](worklist-pattern.md) |
| "Can I skip flows or mix human work with swarm work?" | [Flow Flexibility](flow-flexibility.md) |

---

## Key Concepts

| Question | Document |
|----------|----------|
| "When should agents escalate?" | [Authority, Not Difficulty](authority-not-difficulty.md) |
| "When and how should agents involve humans?" | [Human Escalation](human-escalation.md) |
| "Why do we verify so much?" | [Economics](economics.md) |
| "What happens at scale?" | [Emergent Phenomena](emergent-phenomena.md) |
| "How does truth arbitration work?" | [Truth Hierarchy](truth-hierarchy.md) |
| "How does trust emerge from verification, not restrictions?" | [Trust Architecture](trust-architecture.md) |
| "What are the layers of verification and what does each prove?" | [Verification Stack](verification-stack.md) |
| "Why is generation cheap but review expensive?" | [Throughput Inversion](throughput-inversion.md) |
| "What's the mental model for LLMs generating code?" | [Stochastic Compiler](stochastic-compiler.md) |

---

## Deep Dives

| Question | Document |
|----------|----------|
| "Why default-allow?" | [Why Ops First](why-ops-first.md) |
| "How do boundaries work?" | [Boundary Physics](boundary-physics.md) |
| "Why two gates?" | [Why Two Gates](why-two-gates.md) |
| "How do claims work?" | [Claims and Evidence](claims-and-evidence.md) |
| "How does raw LLM output become trusted?" | [Candidates to Artifacts](candidates-to-artifacts.md) |
| "Why is the PR description the primary review interface?" | [PR as Review Surface](pr-as-review-surface.md) |
| "What LLM failure modes does this design address?" | [AI Physics](ai-physics.md) |
| "Why starve agents of context instead of giving them everything?" | [Context Discipline](context-discipline.md) |
| "Why does cleanup re-run after sanitization?" | [Why Reseal](why-reseal.md) |
| "How does AgOps compare to copilots, black boxes, and frameworks?" | [Competitive Positioning](competitive-positioning.md) |
| "How does this repo teach the mental model while operating?" | [Teaching Repo](teaching-repo.md) |

---

## Architecture and Philosophy

| Question | Document |
|----------|----------|
| "What are the core architecture principles?" | [Architecture](architecture.md) |
| "What's the operational philosophy?" | [Operational Philosophy](operational-philosophy.md) |
| "What's the difference between skills and agents?" | [Skills vs Agents](skills-vs-agents.md) |
| "How does state and resumption work?" | [State and Resumption](state-and-resumption.md) |
| "How does CLAUDE.md shape behavior?" | [How CLAUDE.md Works](how-claude-md-works.md) |
| "What is Claude-native design?" | [Claude-Native Design](claude-native-design.md) |
| "How does the flywheel work?" | [The Flywheel](the-flywheel.md) |
| "Why seven flows?" | [Why Seven Flows](why-seven-flows.md) |
| "How does traceability work?" | [Traceability Spine](traceability-spine.md) |
| "What are common anti-patterns?" | [Anti-Patterns](anti-patterns.md) |
| "What role do adversarial loops play?" | [Adversarial Loops](adversarial-loops.md) |
| "Why is code treated as binary?" | [Code as Binary](code-as-binary.md) |

## Terminology Note

The pack uses "planes" in two different contexts:

| Term | Meaning | Explained in |
|------|---------|--------------|
| **Work Plane / Publish Plane** | Where gates engage (publish boundary only) | [Why Ops-First](why-ops-first.md) |
| **Routing Plane / Audit Plane** | How decisions flow (prose handoffs vs files) | [Why Two Planes](why-two-planes.md) |

These are orthogonal concepts—both matter, in different ways.
