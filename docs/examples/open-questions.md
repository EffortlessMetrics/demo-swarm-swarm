# Sample Open Questions Register

This is an example of an open questions register showing how uncertainty is tracked during a run. Questions use the stable marker format `QID: OQ-<FLOW>-NNN` for mechanical counting.

---

## Open Questions: feat-session-timeout

**Run ID:** feat-session-timeout
**Updated:** 2024-01-15T14:30:00Z

---

## DEFAULTED

Questions where an agent chose a reversible default and documented the reasoning. These do not block flow progression.

### OQ-BUILD-001: Session storage backend

- **QID:** OQ-BUILD-001
- **Question:** Use Redis or in-memory for session storage?
- **Context:** No existing Redis infrastructure in repo. ADR doesn't specify backend.
- **Default chosen:** In-memory with documented upgrade path
- **Reasoning:** Lower complexity for MVP; Redis can be added later. Interface is abstracted via `SessionStore` protocol.
- **Risk if wrong:** Low - interface is abstracted, swap is mechanical (estimated 2h refactor)
- **Decided by:** code-implementer during Flow 3
- **Evidence:** `src/auth/session_store.py:1-15` (protocol definition)

### OQ-BUILD-002: Timeout precision

- **QID:** OQ-BUILD-002
- **Question:** Check timeout on every request or use background sweeper?
- **Context:** Trade-off between simplicity and efficiency. ADR-007 prefers simplicity.
- **Default chosen:** Check on every request
- **Reasoning:** Simpler, sufficient for expected load (<1000 concurrent sessions). Aligns with ADR-007.
- **Risk if wrong:** Medium - may need refactor at scale (estimated 4h to add background sweeper)
- **Decided by:** code-implementer during Flow 3
- **Evidence:** `src/auth/middleware.py:45-50` (per-request cleanup call)

### OQ-PLAN-003: Timeout units in config

- **QID:** OQ-PLAN-003
- **Question:** Expose timeout in seconds, minutes, or both?
- **Context:** Other config values use seconds (e.g., `connection_timeout_seconds`).
- **Default chosen:** Seconds only, matching existing convention
- **Reasoning:** Consistency with existing config. Users familiar with the pattern.
- **Risk if wrong:** Low - purely cosmetic, can add alternative format later
- **Decided by:** interface-designer during Flow 2
- **Evidence:** `src/config/schema.yaml:34` (field definition)

---

## NEEDS_HUMAN

Questions requiring human authority — things that genuinely cannot be derived, researched, or safely defaulted. These are rare. If an agent can research it, derive it, or choose a reversible default, it's not NEEDS_HUMAN.

### OQ-PLAN-004: Breaking change affects 3 paying customers

- **QID:** OQ-PLAN-004
- **Question:** New session format breaks API compatibility for Acme Corp, Globex, and Initech. Proceed anyway?
- **Context:** These 3 customers use undocumented session internals. Migration requires their engineering time. Sales says Acme is in renewal negotiations.
- **Why agent cannot decide:** Business relationship decision. Agent has no authority over customer relationships or revenue trade-offs.
- **Options:**
  1. Proceed + notify (faster, risks relationships)
  2. Add compatibility shim (6h work, keeps tech debt)
  3. Delay until customers migrate (unknown timeline)
- **Escalated by:** design-optioneer during Flow 2
- **Needs:** Product owner or account manager decision

### OQ-GATE-001: Ship with known race condition or delay release?

- **QID:** OQ-GATE-001
- **Question:** Race condition in concurrent session handling causes 1-in-10k failures. Release deadline is tomorrow. Ship or fix?
- **Context:** Fix is 8h. Failure is recoverable (client retry works). Monitoring is in place.
- **Why agent cannot decide:** Risk tolerance and release timing are business authority. Both options are technically valid.
- **Options:**
  1. Ship with known issue + documented workaround
  2. Delay release 1 day to fix
- **Escalated by:** merge-decider during Flow 5
- **Needs:** Engineering lead or release manager decision

---

## Format Reference

| Status      | Meaning                                                | Blocks Flow       |
| ----------- | ------------------------------------------------------ | ----------------- |
| DEFAULTED   | Agent chose a reversible default, documented reasoning | No                |
| NEEDS_HUMAN | True ambiguity requiring human decision                | Depends on impact |

**Marker format:** `- QID: OQ-<FLOW>-NNN` where FLOW is SIGNAL, PLAN, BUILD, GATE, DEPLOY, or WISDOM.

---

## Handoff

**What I did:** Registered 4 open questions during this run. 3 were DEFAULTED with documented reasoning; 2 remain NEEDS_HUMAN.

**What's left:** Two NEEDS_HUMAN questions (GDPR retention, encryption at rest) should be resolved before production deployment. They do not block development.

**Recommendation:** Proceed with current flow. Escalate OQ-SIGNAL-001 (GDPR) and OQ-BUILD-004 (encryption) to product owner for decision before Flow 6 (Deploy). Document decisions in this register when resolved.
