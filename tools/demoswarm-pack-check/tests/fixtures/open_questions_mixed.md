# Open Questions

This is a test fixture with a mix of valid and invalid QID patterns.

## Questions

- QID: OQ-SIG-001
  - Q: Valid question with proper format [OPEN]
  - A: This uses canonical SIG code

- QID: OQ-PLAN-002
  - Q: Invalid flow code (PLAN should be PLN) [OPEN]
  - A: This is a violation

- QID: OQ-BLD-003
  - Q: Valid question with canonical BLD code [OPEN]
  - A: This is correct

- QID: OQ-BLD-3
  - Q: Invalid padding (should be 003) [OPEN]
  - A: This is a violation

- QID: OQ-GAT-999
  - Q: Valid at upper bound [OPEN]
  - A: This uses canonical GAT code

## Notes

This file tests multi-match validation:

- 3 valid QIDs: OQ-SIG-001, OQ-BLD-003, OQ-GAT-999
- 2 invalid QIDs: OQ-PLAN-002 (non-canonical PLAN), OQ-BLD-3 (bad padding)
