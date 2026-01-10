# Open Questions

This is a test fixture with a mix of valid and invalid QID patterns.

## Questions

- QID: OQ-SIG-001
  - Q: Valid question with proper format [OPEN]
  - A: This is correct

- QID: OQ-PLAN-002
  - Q: Valid question with canonical flow code [OPEN]
  - A: This is correct

- QID: OQ-BLD-003
  - Q: Invalid flow code (BLD should be BUILD) [OPEN]
  - A: This is a violation

- QID: OQ-BUILD-3
  - Q: Invalid padding (should be 003) [OPEN]
  - A: This is a violation

- QID: OQ-GATE-999
  - Q: Valid at upper bound [OPEN]
  - A: This is correct

## Notes

This file tests multi-match validation:
- 3 valid QIDs: OQ-SIG-001, OQ-PLAN-002, OQ-GATE-999
- 2 invalid QIDs: OQ-BLD-003 (bad flow code), OQ-BUILD-3 (bad padding)
