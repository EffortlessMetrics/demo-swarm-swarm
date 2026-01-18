# Open Questions - Build Phase

Append-only register for Build-phase clarifications.

## Resolutions from Plan-phase questions

- A: OQ-PLAN-001 resolved - Using single atomic PR with logical commits per phase. Phase 1 commit for authoritative sources, Phase 2 commit for primary docs, Phase 3 commit for secondary docs, Phase 4 commit for pack-check fixes if needed. This provides granular revert while maintaining PR coherence. [RESOLVED]

- A: OQ-PLAN-002 resolved - Flow 7 documentation will explicitly describe "second-cycle wisdom extraction for multi-iteration runs" to distinguish from /flow-6-wisdom. This aligns with the resolved OQ-SIG-002 answer. [RESOLVED]

- A: OQ-PLAN-003 resolved - Flow 7 is documented as distinct from Flow 6 in the flow table (7 flows, 10 command entrypoints). ST-007 compliance partitioning deferred as a follow-up enhancement; current focus is documentation alignment only. [RESOLVED]

## Build-Phase Clarifications

### Assumptions Made to Proceed

- Assumption: "102 unit tests passing" is the current authoritative count per test_output.log analysis.
  - Rationale: Signal/Plan artifacts reference this count; verifiable by running `cargo test`
  - Impact if wrong: Update count to actual current value

- Assumption: CLAUDE.md flow table currently shows correct 7-flow structure but needs variant command documentation.
  - Rationale: L186-196 lists 7 flows; variants need inline or footnote documentation
  - Impact if wrong: Major restructure needed

- Assumption: Pack-check already validates 7-flow structure; test fixtures may need "Seven Flows" instead of "Six Flows" strings.
  - Rationale: Per RSK-001, fixtures may assert old counts
  - Impact if wrong: Pack-check needs deeper investigation

## Initial Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []
questions_resolved: 3
assumptions_added: 3

- QID: OQ-BUILD-001
  - Q: What specific phrasing should be used for the CHANGELOG v1.0.0 annotation to clarify the evolution from 6 to 7 flows? [OPEN]
  - Suggested default: Add parenthetical annotation noting evolution to seven flows with 10 command files
  - Impact if different: If historical accuracy is paramount, may need different phrasing to clarify timing
  - Added: 2025-12-20T11:58:40Z

- QID: OQ-BUILD-002
  - Q: Should pack-check test fixtures be updated proactively (Phase 4) or only reactively if pack-check fails after Phases 1-2? [OPEN]
  - Suggested default: Reactive only - run pack-check after Phase 2 and update structure.rs only if Six Flows assertion fails
  - Impact if different: If proactive, adds scope but ensures no CI breakage; if reactive, minimizes changes but risks temporary CI failure
  - Added: 2025-12-20T11:59:00Z

- QID: OQ-BUILD-003
  - Q: What is the exact line number range in CLAUDE.md that contains the flow table requiring expansion? [OPEN]
  - Suggested default: Lines 186-196 as referenced in work_plan.md ST-001, but verify before editing
  - Impact if different: If line numbers have drifted due to other edits, need to locate correct section before proceeding
  - Added: 2025-12-20T11:59:03Z

- QID: OQ-BUILD-004
  - Q: How should flow variant relationships be structured in architecture.md - as a table, prose sections, or diagram? [OPEN]
  - Suggested default: Prose sections with clear subheadings matching existing architecture.md style
  - Impact if different: If table format preferred, need different markup; if diagram, need ASCII or external tool
  - Added: 2025-12-20T11:59:06Z

- QID: OQ-BUILD-005
  - Q: What is the exact source artifact path for the 102 passing tests claim? [OPEN]
  - Suggested default: Reference as test_output.log per issue_normalized.md; actual path may be ephemeral CI artifact
  - Impact if different: If no persistent artifact exists, test count claim may be unverifiable; may need to document as of specific date
  - Added: 2025-12-20T11:59:09Z

### Additional Assumptions

- Assumption: The build directory will be created by run-prep before subtask implementation begins.
  - Rationale: Standard flow requires run-prep to create flow directories
  - Impact if wrong: File writes would fail; need manual directory creation
  - Linked question: null

- Assumption: All Signal-phase questions (OQ-SIG-001 through OQ-SIG-006) have been resolved or assumed forward per plan/open_questions.md resolutions.
  - Rationale: Plan-phase clarifier resolved OQ-SIG-001, OQ-SIG-002, OQ-SIG-003, OQ-SIG-005; OQ-SIG-004 and OQ-SIG-006 deferred to Build
  - Impact if wrong: May need to revisit Signal assumptions before proceeding
  - Linked question: OQ-SIG-004, OQ-SIG-006 (still open per plan/open_questions.md)

- Assumption: Plan-phase questions (OQ-PLAN-001, OQ-PLAN-002, OQ-PLAN-003) will be resolved using suggested defaults during Build implementation.
  - Rationale: Each question has a reasonable default; no human escalation required
  - Impact if wrong: Implementation may diverge from human expectations on commit strategy, Flow 7 description, or compliance partitioning
  - Linked question: OQ-PLAN-001, OQ-PLAN-002, OQ-PLAN-003

- Assumption: CLAUDE.md and architecture.md have not been modified since plan phase, so line number references remain valid.
  - Rationale: Documentation-only run; no concurrent modifications expected
  - Impact if wrong: Line number references in work_plan.md would be stale; need to re-locate sections before editing
  - Linked question: OQ-BUILD-003

- Assumption: The path traversal concern (OQ-SIG-004) is out of scope for this documentation alignment run.
  - Rationale: ADR explicitly states no code changes to secrets.rs; this is a documentation-only run
  - Impact if wrong: Would need to escalate to security hardening work item before proceeding
  - Linked question: OQ-SIG-004

### Resolutions (if any)

(none yet - Build phase just started)

### Final Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
output_path: .runs/local-alignment-audit-aba1c6/build/open_questions.md
questions_added: 5
assumptions_added: 5
missing_required: []
blockers: []
concerns:

- OQ-SIG-004 (path traversal) and OQ-SIG-006 (test count story) remain open from Signal; assumed forward per defaults
- OQ-PLAN-001/002/003 remain open from Plan; will use suggested defaults during implementation
- Line number references (L186-196) in work_plan.md should be verified before editing
