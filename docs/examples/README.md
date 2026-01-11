# Example Artifacts

These are synthetic but realistic examples of swarm-generated artifacts. They demonstrate the expected format and content, not actual outputs.

| Example | Shows |
|---------|-------|
| [pr-cockpit.md](pr-cockpit.md) | PR description format (the primary review UI) |
| [code-critique.md](code-critique.md) | Critique with CRITICAL/MAJOR/MINOR/SUGGESTION severity levels |
| [open-questions.md](open-questions.md) | How uncertainty is tracked (DEFAULTED vs NEEDS_HUMAN) |
| [merge-decision.md](merge-decision.md) | Gate decision memo with receipt chain audit |

## Key Patterns Demonstrated

All examples demonstrate these patterns:

1. **Handoff structure** - "What I did / What's left / Recommendation" for routing
2. **Evidence pointers** - File paths with line numbers (e.g., `src/auth/session.py:45-89`)
3. **Quality scorecard** - Five surfaces: Correctness, Verification, Boundaries, Maintainability, Explanation
4. **Stable markers** - Countable prefixes like `REQ-`, `OQ-`, `[CRITICAL]`, `[MAJOR]`, `[MINOR]`

## Related Reference Docs

- [pr-review-interface.md](../reference/pr-review-interface.md) - PR Brief template
- [pr-quality-scorecard.md](../reference/pr-quality-scorecard.md) - Scorecard template and status values
- [contracts.md](../reference/contracts.md) - Receipt schemas and handoff contracts
- [stable-markers.md](../reference/stable-markers.md) - Marker patterns for counting

These examples are intentionally small and focused. Real artifacts may be longer but follow the same structure.
