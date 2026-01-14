# Example Artifacts

These are synthetic but realistic examples of swarm-generated artifacts. They demonstrate the expected format and content, not actual outputs.

**Why these examples matter:** You learn this system by seeing the artifacts. The PR cockpit, the merge decision, the critiques—these are what review actually looks like. Read them to understand the posture.

| Example | Shows | What to Notice |
|---------|-------|----------------|
| [pr-cockpit.md](pr-cockpit.md) | PR description format | The cockpit is your primary UI, not the diff |
| [code-critique.md](code-critique.md) | Critique severities | CRITICAL/MAJOR/MINOR/SUGGESTION have clear semantics |
| [open-questions.md](open-questions.md) | Uncertainty tracking | DEFAULTED = safe assumption made; NEEDS_HUMAN = requires authority |
| [merge-decision.md](merge-decision.md) | Gate decision memo | Receipt chain audit shows how verification flows through |

---

## What Good Looks Like

**Honest about gaps.** The PR cockpit explicitly says "Not measured: Load testing under concurrent session expiry." That's not a failure—it's honest acknowledgment. Silent gaps are the failure mode.

**Evidence has pointers.** "Tests pass" isn't good enough. "142/142 passed (evidence: .runs/.../test_execution.md)" is. Claims without pointers are just prose.

**Handoffs are specific.** "Done" isn't a handoff. "Implemented 3 of 5 endpoints. The remaining 2 need the User schema. Recommend routing to code-implementer with User schema as first task." is.

**Completion states are mechanical.** VERIFIED means evidence panel green. UNVERIFIED means checkpointed state. Neither is a feeling.

---

## Key Patterns Demonstrated

All examples demonstrate these patterns:

1. **Handoff structure** — "What I did / What's left / Recommendation" for routing
2. **Evidence pointers** — File paths with line numbers (e.g., `src/auth/session.py:45-89`)
3. **Quality scorecard** — Five surfaces: Correctness, Verification, Boundaries, Maintainability, Explanation
4. **Stable markers** — Countable prefixes like `REQ-`, `OQ-`, `[CRITICAL]`, `[MAJOR]`, `[MINOR]`
5. **"Not Measured" explicit** — Gaps are acknowledged, not hidden

---

## Related Reference Docs

- [pr-review-interface.md](../reference/pr-review-interface.md) — PR Brief template
- [pr-quality-scorecard.md](../reference/pr-quality-scorecard.md) — Scorecard template and status values
- [contracts.md](../reference/contracts.md) — Receipt schemas and handoff contracts
- [stable-markers.md](../reference/stable-markers.md) — Marker patterns for counting

These examples are intentionally small and focused. Real artifacts may be longer but follow the same structure.
