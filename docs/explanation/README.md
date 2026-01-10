# Explanation

Understanding-oriented: explore the why behind the pack design.

## Reading Order

**Start here:**
1. [Architecture](architecture.md) — Core philosophy and patterns
2. [What Makes This Different](what-makes-this-different.md) — Assumptions that don't apply here

**Then pick by interest:**

| If you're asking... | Read |
|---------------------|------|
| "Why default-allow for work but gated for publish?" | [Why Ops-First](why-ops-first.md) |
| "How do agents think, decide, and fail gracefully?" | [Agent Philosophy](agent-philosophy.md) |
| "Why do agents read/write disk instead of sharing context?" | [Stateless Execution](stateless-execution.md) |
| "Why route on control blocks, not by re-reading files?" | [Why Two Planes](why-two-planes.md) |
| "Why do GitHub ops need two separate gates?" | [Why Two Gates](why-two-gates.md) |
| "Why does cleanup re-run after sanitization?" | [Why Reseal](why-reseal.md) |
| "What LLM failure modes does this design address?" | [AI Physics](ai-physics.md) |

## Terminology Note

The pack uses "planes" in two different contexts:

| Term | Meaning | Explained in |
|------|---------|--------------|
| **Work Plane / Publish Plane** | Where gates engage (publish boundary only) | [Why Ops-First](why-ops-first.md) |
| **Control Plane / Audit Plane** | How routing works (blocks vs files) | [Why Two Planes](why-two-planes.md) |

These are orthogonal concepts—both matter, in different ways.
