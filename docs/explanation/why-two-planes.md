# Why Two Planes (Routing vs Audit)

> Prose handoffs route work. Receipts preserve history.

**Note:** This doc is about **Routing Plane vs Audit Plane** (how decisions flow). For **Work Plane vs Publish Plane** (where gates engage), see [Why Ops-First](why-ops-first.md).

---

## The insight

Routing and audit serve different purposes:

**Routing:** "What should happen next?" — Real-time decisions during a run
**Audit:** "What happened?" — Records for humans reviewing later

These are different audiences with different needs. Don't conflate them.

---

## Two planes defined

### Routing plane

Natural language handoffs between agents and orchestrator:

```markdown
## Handoff

**What I did:** Reviewed the implementation against REQ-001 through REQ-005.
Found two issues: session timeout doesn't match ADR, REQ-003 has no test coverage.

**What's left:** Timeout fix is mechanical. Missing tests need test-author.

**Recommendation:** Route to **fixer** for the timeout issue, then **test-author**
for coverage. Re-run me after both to verify fixes landed.
```

Properties:

- Natural language, contextual
- Returned in agent response
- The orchestrator (Claude) reads and understands it
- Expressive — handles edge cases and nuance

### Audit plane

Durable artifacts written to `.runs/<run-id>/`:

- `requirements.md`
- `code_critique.md`
- `secrets_scan.md`
- `*_receipt.json`

Properties:

- Rich, detailed, inspectable
- Written to disk
- Used for human review, reruns, debugging
- Receipts include structured fields (derived from handoffs by cleanup agents)

---

## How they interact

```
Agent runs
  ├─→ Writes audit artifacts (files)
  └─→ Returns prose handoff (response)

Orchestrator
  ├─→ Routes on prose handoff (understands it)
  └─→ Does NOT parse structured data for routing

Cleanup agent
  ├─→ Reads artifacts and handoff
  └─→ Writes receipt with structured fields (for audit trail)
```

Example: code-critic

1. Reviews implementation
2. Writes `code_critique.md` (audit artifact)
3. Returns prose handoff: "Found 2 issues. Recommend fixer address both."
4. Orchestrator reads handoff, routes to fixer
5. Later: cleanup agent writes `build_receipt.json` with structured fields
6. Much later: humans inspect receipts to understand run history

---

## Why prose routes and structure audits

### Claude is the orchestrator

Claude doesn't need structured data to make routing decisions. It reads prose:

> "Implementation looks good but the session timeout is wrong. Recommend the fixer address this before we continue."

Claude understands this completely. No parsing needed. No enum matching. Just reading and deciding.

Structured routing blocks are from systems where a Python script made routing decisions. Claude doesn't need training wheels.

### Prose handles edge cases

Real work rarely fits in enums. Consider:

> "Almost done, but the database migration is missing. Either create it as part of this AC, or document the dependency and proceed."

What status enum captures this? `MOSTLY_DONE_BUT_BLOCKED_ON_EXTERNAL_DEPENDENCY_WHICH_MIGHT_BE_IN_SCOPE`?

Prose lets agents tell the truth. Claude understands nuance.

### Structure serves audit

Receipts need structured fields for:

- Mechanical processing (counting, aggregation)
- Consistent audit trail
- Index updates

But these fields are derived AFTER routing happens. Cleanup agents read the prose handoff and produce structured receipts. The structure exists for later review, not for real-time routing.

---

## The derivation flow

```
Agent completes work
    │
    ├─→ Prose handoff: "Found 3 issues. Recommend fixer."
    │
    ▼
Orchestrator routes on handoff
    │
    ▼
Later: Cleanup agent runs
    │
    ├─→ Reads prose handoff
    ├─→ Writes receipt:
    │     {
    │       "status": "UNVERIFIED",
    │       "recommended_action": "BOUNCE",
    │       "route_to_agent": "fixer",
    │       "counts": { "critical": 0, "major": 2, "minor": 1 }
    │     }
    │
    ▼
Audit plane now has structured record
```

The structured fields in receipts are **derived from prose**, not the other way around. They exist for audit trail completeness, not for routing.

---

## Examples

### Critic handoff (routing)

```markdown
## Handoff

**What I did:** Reviewed implementation against the ADR and requirements.

**Findings:**

- [MAJOR] Session timeout uses 30m but ADR specifies 15m
- [MAJOR] Missing error handling in auth refresh path
- [MINOR] Inconsistent naming between UserSession and SessionUser

**Recommendation:** Route to **fixer** for the MAJOR items. The timeout
is a one-line change; the error handling needs about 10 lines. After fixes,
re-run me to verify they landed correctly.
```

The orchestrator reads this and routes to fixer. No parsing of structured blocks.

### Receipt (audit)

```json
{
  "run_id": "feat-auth",
  "flow": "build",
  "status": "UNVERIFIED",
  "recommended_action": "BOUNCE",
  "route_to_agent": "fixer",
  "counts": {
    "critical": 0,
    "major": 2,
    "minor": 1
  },
  "completed_at": "2024-01-15T10:30:00Z"
}
```

The cleanup agent derived this from the prose handoff. It exists so humans reviewing the run in 3 months understand what happened.

---

## Rules

1. **Orchestrators route on prose handoffs** — Claude reads and understands
2. **Receipts are audit artifacts** — Structured fields for later review
3. **Cleanup agents derive structure from prose** — The derivation is one-way
4. **Don't conflate routing and audit** — Different audiences, different needs

---

## See also

- [architecture.md](architecture.md) — Overall pack design
- [what-makes-this-different.md](what-makes-this-different.md) — Contrasts with old patterns
- [contracts.md](../reference/contracts.md) — Handoff patterns and receipt schemas
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
