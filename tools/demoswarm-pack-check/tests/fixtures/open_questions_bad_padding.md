# Open Questions

This is a test fixture with invalid numeric suffix padding.

## Questions

- QID: OQ-SIG-1
  - Q: Single digit suffix? [OPEN]
  - A: Should be OQ-SIG-001

- QID: OQ-PLN-12
  - Q: Two digit suffix? [OPEN]
  - A: Should be OQ-PLN-012

- QID: OQ-BLD-1234
  - Q: Four digit suffix? [OPEN]
  - A: Should be OQ-BLD-999 max (3 digits)

## Notes

Numeric suffixes must be:

- Exactly 3 digits
- Zero-padded (001, not 1)
- Range: 001-999
