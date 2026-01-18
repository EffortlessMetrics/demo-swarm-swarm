# Schemas Reference

> Canonical data structures that carry meaning between agents.

---

## Purpose

This document defines the schemas used throughout the swarm. These structures are the shared vocabulary for communicating state, decisions, and evidence between agents. They are not routing contracts (see [contracts.md](contracts.md) for control-plane blocks); they are data shapes that agents read and write.

---

## Quality Panel

The Quality Panel is the multi-surface view of PR quality. It appears in PR cockpits, receipts, and gate decisions.

```yaml
surface: Correctness | Verification | Boundaries | Maintainability | Explanation
status: measured | partial | estimated | clean | unknown
method: automated | derived | attested
evidence: [file paths or commands]
notes: string (1-3 lines)
```

### Field Definitions

| Field      | Values      | Meaning                                    |
| ---------- | ----------- | ------------------------------------------ |
| `surface`  | See list    | One of the five quality surfaces           |
| `status`   | `measured`  | Automated verification ran and reported    |
|            | `partial`   | Some aspects measured, others not          |
|            | `estimated` | Derived from patterns/precedent, not run   |
|            | `clean`     | No changes in this surface                 |
|            | `unknown`   | Not measured, no estimate available        |
| `method`   | `automated` | Tool output (test runner, linter, scanner) |
|            | `derived`   | Calculated from other evidence             |
|            | `attested`  | Agent observation (lower confidence)       |
| `evidence` | array       | File paths, commands, or artifact links    |
| `notes`    | string      | Brief context (1-3 lines max)              |

### Example

```yaml
- surface: Correctness
  status: measured
  method: automated
  evidence:
    - .runs/feat-auth/build/test_execution.md
    - pytest tests/ --verbose
  notes: 12 BDD scenarios pass. 1 edge case pending.

- surface: Verification
  status: partial
  method: automated
  evidence:
    - .runs/feat-auth/build/mutation_report.md
  notes: Mutation score 87%. 3 survivors in error handling.

- surface: Boundaries
  status: clean
  method: derived
  evidence:
    - git diff origin/main -- api_contracts.yaml
  notes: No API/schema changes.
```

### Usage

| Context        | Location                                  |
| -------------- | ----------------------------------------- |
| PR cockpit     | PR description, Quality Scorecard section |
| Receipts       | `quality_gates` field in `*_receipt.json` |
| Gate decisions | `merge_decision.md` evidence section      |

See: [pr-quality-scorecard.md](pr-quality-scorecard.md) for the full quality surface philosophy.

---

## Open Question

Open questions track uncertainty during a run. The schema has two variants based on status.

```yaml
qid: OQ-<FLOW>-NNN
status: DEFAULTED | NEEDS_HUMAN

# If DEFAULTED:
question: string
context: string
default_chosen: string
reasoning: string
risk_if_wrong: string
decided_by: agent + flow
evidence: file:line

# If NEEDS_HUMAN:
question: string
context: string
why_agent_cannot_decide: string  # Must be about AUTHORITY, not difficulty
options: [list of choices with trade-offs]
escalated_by: agent + flow
needs: who has authority (role, not person)
```

### Key Principle

**NEEDS_HUMAN is about authority, not difficulty.**

If an agent can research it, derive it, or choose a reversible default, it's DEFAULTED. NEEDS_HUMAN is reserved for decisions requiring human authority:

- Business relationship trade-offs
- Risk tolerance choices
- Release timing decisions
- Policy exceptions

### DEFAULTED Example

```yaml
qid: OQ-BUILD-001
status: DEFAULTED
question: Use Redis or in-memory for session storage?
context: No existing Redis infrastructure. ADR doesn't specify backend.
default_chosen: In-memory with documented upgrade path
reasoning: Lower complexity for MVP; interface is abstracted via SessionStore protocol.
risk_if_wrong: Low - swap is mechanical (estimated 2h refactor)
decided_by: code-implementer during Flow 3
evidence: src/auth/session_store.py:1-15
```

### NEEDS_HUMAN Example

```yaml
qid: OQ-PLAN-004
status: NEEDS_HUMAN
question: Breaking change affects 3 paying customers. Proceed anyway?
context: Acme Corp, Globex, and Initech use undocumented session internals.
why_agent_cannot_decide: Business relationship decision. Agent has no authority over customer relationships or revenue trade-offs.
options:
  - Proceed + notify (faster, risks relationships)
  - Add compatibility shim (6h work, keeps tech debt)
  - Delay until customers migrate (unknown timeline)
escalated_by: design-optioneer during Flow 2
needs: Product owner or account manager
```

### Usage

| Context        | Location                             |
| -------------- | ------------------------------------ |
| Run artifacts  | `.runs/<run-id>/*/open_questions.md` |
| Receipt counts | `open_questions` field in receipts   |
| PR Brief       | Open questions summary section       |

See: [docs/examples/open-questions.md](../examples/open-questions.md) for full examples.

---

## Evidence Freshness

Evidence has a timestamp; repo state is "now." This schema tracks the relationship between them.

```yaml
evidence_sha: git SHA when evidence was generated
generated_at: ISO timestamp
scope: what was verified (e.g., "all tests", "changed files only")
status: FRESH | ACCEPTABLE_STALE | STALE | UNKNOWN
divergence_note: string (if stale, what might have changed)
```

### Status Values

| Status             | Meaning                                                        | Action                      |
| ------------------ | -------------------------------------------------------------- | --------------------------- |
| `FRESH`            | SHA matches HEAD                                               | Trust as current            |
| `ACCEPTABLE_STALE` | SHA differs but changes are irrelevant (e.g., formatting only) | Accept with note            |
| `STALE`            | SHA differs and changes may affect validity                    | Re-verify or note staleness |
| `UNKNOWN`          | Missing timestamp or SHA                                       | Treat as unverified         |

### Example

```yaml
evidence_sha: abc123def456
generated_at: 2025-12-22T10:45:00Z
scope: all tests in tests/ directory
status: ACCEPTABLE_STALE
divergence_note: 2 commits since evidence (formatting and comments only)
```

### Usage

| Context        | Location                                 |
| -------------- | ---------------------------------------- |
| Receipts       | `evidence_sha` and `generated_at` fields |
| Gate decisions | Freshness check before merge             |
| PR Brief       | Quality Scorecard staleness notes        |

See: [evidence-freshness.md](evidence-freshness.md) for staleness detection patterns.

---

## Handoff

The handoff is how agents communicate with orchestrators. It is prose, not parsed.

```yaml
what_i_did: string (1-2 sentences summarizing work completed)
whats_left: string (remaining work, blockers, open items)
recommendation: string (specific next step + reasoning, naming agents)
```

### The Pattern

Every agent ends with a handoff that answers three questions:

1. What was done?
2. What still needs to be done?
3. What do I recommend?

### Example

```markdown
## Handoff

**What I did:** Implemented the session timeout feature. Added middleware check
on every request. Wrote 4 unit tests covering normal flow and edge cases.

**What's left:** One test is failing on concurrent access edge case. The timeout
configuration format is documented but not validated at startup.

**Recommendation:** Route to **fixer** for the failing test (appears to be a race
condition in the test setup, not the code). Then route to **self-reviewer** for
final check before proceeding to review.
```

### Rules

- **Always make a recommendation.** Even when uncertain, take a stance.
- **Name specific agents.** "Route to fixer" is better than "fix the issue."
- **Explain reasoning.** "Because X" helps the orchestrator override if needed.
- **Partial is success.** "Completed 3/5 items" is a valid handoff.

### Usage

| Context           | Location                             |
| ----------------- | ------------------------------------ |
| Agent responses   | End of every agent output            |
| Cleanup summaries | Derived from agent handoffs          |
| Receipts          | `recommended_action` field (derived) |

See: [contracts.md](contracts.md) for the full handoff contract.

---

## Routing Decision

When an agent recommends routing, this is the implicit structure.

```yaml
symptom: string (what triggered the routing need)
first_choice: agent name
second_choice: agent name (optional fallback)
why: string (reasoning for this routing)
```

### Example

```yaml
symptom: 3 test failures in authentication module
first_choice: fixer
second_choice: test-author
why: Failures appear to be test setup issues, not code bugs. Fixer can address mechanical fixes; if it turns out tests need rewriting, route to test-author.
```

### Usage

| Context                | Location                            |
| ---------------------- | ----------------------------------- |
| Agent handoffs         | Recommendation section (prose form) |
| Routing table          | Reference patterns                  |
| Orchestrator decisions | Input for routing logic             |

See: [routing-table.md](routing-table.md) for common routing patterns.

---

## Trust Signals

Trust signals are the evidence patterns that inform merge decisions.

```yaml
trust_signals:
  - clean receipts (VERIFIED status)
  - green mutation (>90%)
  - hotspots called out
  - critiques addressed
  - ADR alignment
  - BDD scenarios pass

red_flags:
  - missing verification on security surfaces
  - "not measured" on risky areas
  - UNVERIFIED without explanation
  - contract violations
  - unaddressed CRITICAL critiques
```

### Green Signals (Trust Builders)

| Signal              | What It Means                                  |
| ------------------- | ---------------------------------------------- |
| Clean receipts      | All flows completed with VERIFIED status       |
| Green mutation      | Mutation score > 90%, survivors are documented |
| Hotspots called out | Reviewer knows where to focus                  |
| Critiques addressed | No unresolved CRITICAL or MAJOR issues         |
| ADR alignment       | Implementation matches design decisions        |
| BDD scenarios pass  | Acceptance criteria are verified               |

### Red Flags (Trust Breakers)

| Flag                           | Why It Matters                        |
| ------------------------------ | ------------------------------------- |
| Missing security verification  | High-risk surface unverified          |
| "Not measured" on risky areas  | Unknown state in critical path        |
| UNVERIFIED without explanation | Gap not acknowledged                  |
| Contract violations            | API/schema drift without version bump |
| Unaddressed CRITICAL critiques | Known serious issues remain           |

### Usage

| Context           | Location                                |
| ----------------- | --------------------------------------- |
| Gate decisions    | `merge_decision.md` evidence evaluation |
| PR Brief          | Quality events section                  |
| Reviewer guidance | Trust/risk assessment                   |

See: [trust-model.md](trust-model.md) for the full evidence hierarchy.

---

## Flow Boundary Update

When a flow completes, the cleanup agent produces this summary.

```yaml
flow: Signal | Plan | Build | Review | Gate | Deploy | Wisdom
status: complete | partial | blocked
what_was_done: string
findings: [list of key results]
assumptions_made: [DEFAULTED items]
decisions_needed: [NEEDS_HUMAN items]
recommendation: next flow + reasoning
```

### Example

```yaml
flow: Build
status: complete
what_was_done: Implemented session timeout feature with tests
findings:
  - 4 new tests added (all pass)
  - Mutation score 87%
  - 2 MINOR issues addressed from code critic
  - 1 MAJOR issue deferred (requires design input)
assumptions_made:
  - OQ-BUILD-001: Session storage uses in-memory (upgrade path documented)
  - OQ-BUILD-002: Timeout check per-request (simplest approach)
decisions_needed:
  - OQ-PLAN-004: Breaking change notification (needs product owner)
recommendation: Proceed to Review. The deferred MAJOR issue does not block review; it should be addressed in the review worklist.
```

### Usage

| Context      | Location                      |
| ------------ | ----------------------------- |
| Flow cleanup | Written by `*-cleanup` agents |
| Receipts     | Populates `*_receipt.json`    |
| Orchestrator | Input for next-flow decision  |

---

## Schema Index

Quick reference for where each schema appears in the codebase.

| Schema             | Primary Producer | Primary Consumer       | Artifact Location        |
| ------------------ | ---------------- | ---------------------- | ------------------------ |
| Quality Panel      | cleanup agents   | reviewers, gate agents | PR description, receipts |
| Open Question      | any agent        | cleanup, reviewers     | `open_questions.md`      |
| Evidence Freshness | cleanup agents   | gate agents            | receipts                 |
| Handoff            | all agents       | orchestrators          | agent output             |
| Routing Decision   | agents           | orchestrators          | agent handoffs           |
| Trust Signals      | gate agents      | merge-decider          | `merge_decision.md`      |
| Flow Boundary      | cleanup agents   | orchestrators          | receipts                 |

---

## Schema Versioning

Schemas evolve. When updating:

1. **Add fields** - Prefer additive changes (new optional fields)
2. **Version receipts** - Use `schema_version` field (e.g., `build_receipt_v1`)
3. **Document migration** - Note what changed and why
4. **Update consumers** - Ensure agents reading schemas handle new/missing fields

---

## See Also

- [contracts.md](contracts.md) - Control-plane blocks and receipt schemas
- [stable-markers.md](stable-markers.md) - Marker patterns for counting
- [run-state.md](run-state.md) - Run identity and artifact organization
- [trust-model.md](trust-model.md) - Evidence hierarchy
- [evidence-freshness.md](evidence-freshness.md) - Staleness detection
- [pr-quality-scorecard.md](pr-quality-scorecard.md) - Quality surfaces
- [routing-table.md](routing-table.md) - Routing patterns
