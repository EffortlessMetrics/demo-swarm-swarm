# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run local-alignment-audit-aba1c6

### Questions That Would Change the Spec

#### Category: Product

(No new Plan-phase product questions; Signal questions resolved below)

#### Category: Technical

- QID: OQ-PLAN-001
  - Q: Should the documentation update be structured as a single atomic PR or partitioned into logical commits per file/topic? [OPEN]
  - Suggested default: Single atomic PR with logical commits (one commit per file group: README+DEMO_RUN, architecture.md, CLAUDE.md, CHANGELOG)
  - Impact if different: If single commit preferred, simplifies review but loses granular revert capability; if separate PRs, increases review overhead but enables selective merge
  - Needs answer by: Flow 3 (Build)
  - Evidence: flow_plan.md -> "work-planner" step will need commit strategy guidance

- QID: OQ-PLAN-002
  - Q: Should Flow 7 (/flow-7-wisdom) documentation explicitly reference the "second-cycle" or "iteration" use case, or remain generic? [OPEN]
  - Suggested default: Explicitly describe as "second-cycle wisdom extraction for multi-iteration runs" since this distinguishes it from /flow-6-wisdom
  - Impact if different: If generic, loses the distinction from /flow-6-wisdom; if too specific, may not cover all valid use cases
  - Needs answer by: Flow 3 (Build)
  - Evidence: OQ-SIG-002 resolution assumes this interpretation; requirements.md REQ-003 AC-2 requires explaining purpose

#### Category: Data

(No new Plan-phase data questions)

#### Category: Ops

- QID: OQ-PLAN-003
  - Q: Should the compliance partitioning schema (ST-001 through ST-006) be updated to include ST-007 for Flow 7, or should Flow 7 be covered under ST-006 (Wisdom)? [OPEN]
  - Suggested default: Add ST-007 for completeness, since Flow 7 has its own command file and distinct entry point
  - Impact if different: If ST-006 covers both, no schema change needed but may confuse compliance tracing; if ST-007 added, compliance runs need update
  - Needs answer by: Flow 3 (Build)
  - Evidence: OQ-SIG-003 raised this; CLAUDE.md L186-196 shows Flow 7 as distinct from Flow 6

### Assumptions Made to Proceed

- Assumption: CLAUDE.md is the authoritative source of truth for flow count and flow semantics.
  - Rationale: CLAUDE.md states "Treat it as repo-level policy + shared contracts" (L5) and explicitly lists "7 flows" (L13)
  - Impact if wrong: Would need to reconcile by updating CLAUDE.md instead of public docs
  - Linked question: OQ-SIG-001 (resolved below)

- Assumption: Flow variants (flow-4-gate, flow-4-review, etc.) are intentional re-entry points, not duplicates.
  - Rationale: CLAUDE.md L25 shows the canonical flow sequence including variants; each represents different entry into the same phase
  - Impact if wrong: Would need to deprecate variant commands
  - Linked question: OQ-SIG-002 (resolved below)

- Assumption: Agent color coding is advisory metadata for human/tooling consumption, not a schema-enforced contract.
  - Rationale: No evidence of routing logic consuming color fields; pack-check does not validate colors
  - Impact if wrong: Would need to add schema validation and populate missing color fields
  - Linked question: OQ-SIG-005 (resolved below)

### Resolutions (from upstream Signal questions)

- A: Seven flows are canonical. CLAUDE.md L13 states "7 flows: Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom" and this is authoritative per L5 "repo-level policy + shared contracts". Public documentation claiming "six flows" is stale and should be updated. (resolves OQ-SIG-001) [RESOLVED]

- A: /flow-7-wisdom is for second-cycle wisdom extraction. It follows iteration runs where /flow-6-wisdom has already extracted initial learnings, enabling batch or cross-run wisdom synthesis. CLAUDE.md L196 lists it as Flow 7 with distinct command /flow-7-wisdom. (resolves OQ-SIG-002) [RESOLVED]

- A: Yes, Flow 7 warrants ST-007 in compliance partitioning for completeness. Since Flow 7 is a distinct flow (not a variant), compliance coverage should include it. Implementation deferred to OQ-PLAN-003 for Build-phase decision on schema update. (resolves OQ-SIG-003) [RESOLVED]

- A: Agent color coding remains advisory. No routing logic consumes color fields; they are for human/tooling visualization. No schema enforcement needed. (resolves OQ-SIG-005) [RESOLVED]

### Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 3
route_to_agent: null
output_path: .runs/local-alignment-audit-aba1c6/plan/open_questions.md
questions_added: 3
assumptions_added: 3
missing_required: []
blockers: []
concerns:
  - Resolved 4 Signal questions based on CLAUDE.md authority
  - 3 new Plan questions identified for Build-phase decisions
  - OQ-SIG-004 and OQ-SIG-006 remain open (deferred to Build and before-merge respectively)
