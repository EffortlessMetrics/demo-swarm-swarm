# Sample Open Questions Register

This is an example of an open questions register showing how uncertainty is tracked during a run.

---

# Open Questions: feat-session-timeout

## DEFAULTED

### OQ-001: Session storage backend

**Question:** Use Redis or in-memory for session storage?

**Context:** No existing Redis infrastructure in repo

**Default chosen:** In-memory with documented upgrade path

**Reasoning:** Lower complexity for MVP; Redis can be added later

**Risk if wrong:** Low - interface is abstracted, swap is mechanical

**Decided by:** design-optioneer during Flow 2

### OQ-002: Timeout precision

**Question:** Check timeout on every request or use background sweeper?

**Context:** Trade-off between simplicity and efficiency

**Default chosen:** Check on every request

**Reasoning:** Simpler, sufficient for expected load (<1000 concurrent sessions)

**Risk if wrong:** Medium - may need refactor at scale

**Decided by:** code-implementer during Flow 3

## NEEDS_HUMAN

(None for this run)

---

## Format Reference

- **DEFAULTED**: Agent chose a reversible default, documented reasoning
- **NEEDS_HUMAN**: True ambiguity requiring human decision at flow boundary
