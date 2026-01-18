# Implementation Changes Summary for docs-routing-fix

## Handoff

**What I did:** Updated four reference docs to align with natural language routing: removed language implying orchestrators parse structured routing fields, and clarified that routing fields in receipts are audit-only (derived from prose handoffs by cleanup agents).

**What's left:** Nothing - all identified docs are updated.

**Recommendation:** Route to **code-critic** for review. The changes are documentation-only; no code or tests to run.

## What Changed

- **contracts.md**: Clarified that Control-Plane Blocks (Gate Result, Repo Operator Result) are for boolean gate decisions at publish boundaries, not general routing. General routing uses prose handoffs.

- **glossary.md**:
  - Added "Natural Language Routing" definition explaining how agents communicate routing via prose
  - Added "Audit Fields (Derived, Not Routing Input)" clarifying that route*to*\* fields are derived from prose for audit
  - Updated "Control plane" definition to clarify it is for boolean gate decisions, not routing
  - Updated "Critic" to mention prose handoffs instead of "structured Machine Summary fields and routing guidance"
  - Updated "Critique" to reflect prose handoffs plus derived Machine Summary
  - Updated "Bounce" to explain it is expressed in prose, not routing fields

- **pack-check.md**: Added note that Machine Summary is an audit format, not a routing mechanism. Orchestrators route on prose handoffs.

- **trust-model.md**: Rewrote "Verification Agent Pattern" section. Now shows prose handoff as primary routing communication, with Machine Summary as audit plane. Changed "Orchestrators route on the control plane" to "Orchestrators route on prose handoffs. Machine Summary is for audit trails, not routing."

## REQ/NFR -> Implementation Map

| ID  | Implementation Pointer        | Notes                                   |
| --- | ----------------------------- | --------------------------------------- |
| N/A | docs/reference/contracts.md   | Documentation alignment - no formal REQ |
| N/A | docs/reference/glossary.md    | Documentation alignment - no formal REQ |
| N/A | docs/reference/pack-check.md  | Documentation alignment - no formal REQ |
| N/A | docs/reference/trust-model.md | Documentation alignment - no formal REQ |

## Tests

- Test-runner result: N/A (documentation changes only)
- Remaining failures: None

## Known Issues / Handoffs

- None

## Assumptions Made

- Assumed agent-patterns.md was already sufficiently aligned (it has good "prose wins" content at lines 9-30 and 159-176).
- Assumed the routing-table.md file was already aligned (it shows prose handoff examples and says "No parsing required").
