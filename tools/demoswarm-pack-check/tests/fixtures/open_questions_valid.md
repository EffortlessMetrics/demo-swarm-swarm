# Open Questions

This is a test fixture with valid QID patterns.

## Questions

- QID: OQ-SIG-001
  - Q: What is the expected behavior for edge case inputs? [OPEN]
  - A: Pending stakeholder confirmation

- QID: OQ-PLAN-002
  - Q: Should the API contract include retry semantics? [RESOLVED]
  - A: Yes, include exponential backoff per ADR-001

- QID: OQ-BUILD-003
  - Q: Which test framework to use? [RESOLVED]
  - A: Use existing project conventions

- QID: OQ-REVIEW-004
  - Q: What review criteria apply? [OPEN]
  - A: Pending

- QID: OQ-GATE-005
  - Q: What merge verdict criteria apply? [OPEN]
  - A: Pending

- QID: OQ-DEPLOY-006
  - Q: Is canary deployment required? [RESOLVED]
  - A: Not for this change

- QID: OQ-WISDOM-007
  - Q: What learnings should be captured? [OPEN]
  - A: Pending flow completion

## Notes

All QIDs above use canonical flow codes:

- SIG (Signal)
- PLAN (Plan)
- BUILD (Build)
- REVIEW (Review)
- GATE (Gate)
- DEPLOY (Deploy)
- WISDOM (Wisdom)

All numeric suffixes are zero-padded to 3 digits.
