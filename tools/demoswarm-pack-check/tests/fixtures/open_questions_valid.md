# Open Questions

This is a test fixture with valid QID patterns.

## Questions

- QID: OQ-SIG-001
  - Q: What is the expected behavior for edge case inputs? [OPEN]
  - A: Pending stakeholder confirmation

- QID: OQ-PLN-002
  - Q: Should the API contract include retry semantics? [RESOLVED]
  - A: Yes, include exponential backoff per ADR-001

- QID: OQ-BLD-003
  - Q: Which test framework to use? [RESOLVED]
  - A: Use existing project conventions

- QID: OQ-GAT-004
  - Q: What merge verdict criteria apply? [OPEN]
  - A: Pending

- QID: OQ-DEP-005
  - Q: Is canary deployment required? [RESOLVED]
  - A: Not for this change

- QID: OQ-WIS-006
  - Q: What learnings should be captured? [OPEN]
  - A: Pending flow completion

## Notes

All QIDs above use canonical flow codes:
- SIG (Signal)
- PLN (Plan)
- BLD (Build)
- GAT (Gate)
- DEP (Deploy)
- WIS (Wisdom)

All numeric suffixes are zero-padded to 3 digits.
