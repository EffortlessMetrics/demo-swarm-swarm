# Design Options: DemoSwarm Documentation-Code Alignment Audit

## Context

Run ID: `local-alignment-audit-aba1c6`

This document proposes 2-3 distinct architecture options for implementing the documentation alignment audit. The ADR author will select among these options; this agent does not make the final decision.

## Enumerated Requirements

From `.runs/local-alignment-audit-aba1c6/signal/requirements.md`:

### Functional Requirements

- REQ-001: Update flow count references (six -> seven) in public documentation
- REQ-002: Document flow overlap semantics (when to use each variant command)
- REQ-003: Document Flow 7 purpose and usage
- REQ-004: Update CLAUDE.md flow table to reflect seven-flow model
- REQ-005: Correct test count documentation (102 passing unit tests)
- REQ-006: Update security posture documentation (ReDoS immunity, path traversal limitation)
- REQ-007: Clarify agent color coding purpose (functional metadata vs documentation-only)

### Non-Functional Requirements

- NFR-DOC-001: Documentation consistency across all public documentation
- NFR-SEC-001: Security claims have corresponding code evidence
- NFR-TRACE-001: Pack-check tests continue to pass after documentation changes

---

## OPT-001: Minimal Touch - Requirements-Scoped Updates Only

### Description

This option updates only the files explicitly called out in REQ-001 through REQ-007 acceptance criteria, leaving secondary documentation files for a separate follow-up pass. The approach prioritizes a tight scope that can be validated quickly and merged without extensive cross-file coordination.

Primary files updated:

- README.md (REQ-001 AC-1, REQ-001 AC-5)
- DEMO_RUN.md (REQ-001 AC-2)
- docs/explanation/architecture.md (REQ-001 AC-3, REQ-002, REQ-003)
- CHANGELOG.md (REQ-001 AC-4 - annotation only)
- CLAUDE.md (REQ-004)

Security and test documentation updates (REQ-005, REQ-006) would be placed in architecture.md or a dedicated section rather than touching multiple files.

Secondary files NOT updated in this pass:

- docs/reference/glossary.md
- CONTRIBUTING.md
- docs/how-to/work-without-github.md
- docs/tutorials/walkthrough.md
- pack-check test fixtures (structure.rs)

The secondary files would be tracked as a follow-up work item with explicit blockers in the Machine Summary.

### Requirements Fit

| Requirement   | Fit       | Notes                                                                            |
| ------------- | --------- | -------------------------------------------------------------------------------- |
| REQ-001       | SATISFIED | Updates the 4 explicitly named files (README, DEMO_RUN, architecture, CHANGELOG) |
| REQ-002       | SATISFIED | Flow overlap semantics documented in architecture.md                             |
| REQ-003       | SATISFIED | Flow 7 purpose documented in architecture.md                                     |
| REQ-004       | SATISFIED | CLAUDE.md flow table expanded to 7 flows with variants                           |
| REQ-005       | SATISFIED | Test count corrected in architecture.md or README                                |
| REQ-006       | SATISFIED | Security posture updated in architecture.md                                      |
| REQ-007       | SATISFIED | Color coding clarified in architecture.md or CLAUDE.md                           |
| NFR-DOC-001   | PARTIAL   | Primary docs consistent; secondary docs still say "6 flows"                      |
| NFR-SEC-001   | SATISFIED | Security claims reference code evidence                                          |
| NFR-TRACE-001 | SATISFIED | Pack-check tests pass (no fixture changes required)                              |

### Trade-offs

| Dimension                                        | Impact | Rationale                                                       |
| ------------------------------------------------ | ------ | --------------------------------------------------------------- |
| Structure (coupling, components)                 | Low    | Changes isolated to 5 primary files; no cross-cutting refactors |
| Velocity (time-to-first-change)                  | Low    | Fastest path to merge; minimal coordination                     |
| Governance (auditability, determinism)           | Med    | NFR-DOC-001 PARTIAL creates audit gap in secondary docs         |
| Operability (on-call, monitoring, failure modes) | Low    | No operational changes; documentation only                      |
| Cost (compute, complexity tax)                   | Low    | Minimal review burden; small diff                               |

### Reversibility

- Rating: Easy
- Switch effort: Can add secondary file updates in subsequent PRs
- Blast radius if wrong: Pack integrators may encounter "6 flows" in secondary docs and be confused; no functional impact

### Risks

| Risk                                  | Likelihood | Impact | Mitigation (if chosen)                           |
| ------------------------------------- | ---------- | ------ | ------------------------------------------------ |
| Secondary docs remain inconsistent    | High       | Low    | Create explicit follow-up issue; add to blockers |
| Reviewers expect comprehensive update | Medium     | Low    | Clear PR description explaining scope            |
| grep "six flows" still returns hits   | High       | Low    | NFR-DOC-001 MET-1 would fail for secondary files |

### Assumptions

- **Secondary doc inconsistency is acceptable short-term** - impact if wrong: Would need to expand scope to satisfy NFR-DOC-001 fully
- **No external links point to secondary docs for flow count** - impact if wrong: External references would see stale "6 flows" claims

### When to Choose This

Choose this option when time-to-merge is the priority and follow-up work can be reliably tracked. Best for situations where the primary documentation is the most-read surface and secondary docs have low traffic.

---

## OPT-002: Comprehensive Sweep - All Files in One Pass

### Description

This option updates ALL files that reference "six flows" or contain stale flow count claims, including secondary documentation and pack-check test fixtures. The approach prioritizes full consistency over speed, ensuring NFR-DOC-001 is fully satisfied before merge.

Files updated:

- README.md (primary)
- DEMO_RUN.md (primary)
- docs/explanation/architecture.md (primary + flow overlap + Flow 7)
- CHANGELOG.md (annotation)
- CLAUDE.md (flow table expansion)
- docs/reference/glossary.md (secondary)
- CONTRIBUTING.md (secondary)
- docs/how-to/work-without-github.md (secondary)
- docs/tutorials/walkthrough.md (secondary - add Flow 7 step if appropriate)
- tools/demoswarm-pack-check/src/checks/structure.rs (test fixtures)

The comprehensive sweep ensures that `grep "six flows"` returns zero matches across the entire repository after the update.

### Requirements Fit

| Requirement   | Fit       | Notes                                                          |
| ------------- | --------- | -------------------------------------------------------------- |
| REQ-001       | SATISFIED | All files updated to "seven flows"                             |
| REQ-002       | SATISFIED | Flow overlap semantics documented in architecture.md           |
| REQ-003       | SATISFIED | Flow 7 purpose documented; walkthrough may include Flow 7 step |
| REQ-004       | SATISFIED | CLAUDE.md flow table expanded                                  |
| REQ-005       | SATISFIED | Test count corrected                                           |
| REQ-006       | SATISFIED | Security posture updated with code evidence                    |
| REQ-007       | SATISFIED | Color coding clarified                                         |
| NFR-DOC-001   | SATISFIED | Zero matches for "six flows" in any documentation              |
| NFR-SEC-001   | SATISFIED | Security claims reference code evidence                        |
| NFR-TRACE-001 | TRADE_OFF | Pack-check tests may need fixture updates; risk of test churn  |

### Trade-offs

| Dimension                                        | Impact | Rationale                                              |
| ------------------------------------------------ | ------ | ------------------------------------------------------ |
| Structure (coupling, components)                 | Med    | Changes span 10+ files including code (test fixtures)  |
| Velocity (time-to-first-change)                  | High   | Larger PR; more files to review; coordination overhead |
| Governance (auditability, determinism)           | Low    | Full consistency; grep verification passes             |
| Operability (on-call, monitoring, failure modes) | Low    | No operational changes                                 |
| Cost (compute, complexity tax)                   | Med    | Larger diff; more review cycles; test fixture changes  |

### Reversibility

- Rating: Easy
- Switch effort: Individual file changes can be reverted independently
- Blast radius if wrong: Larger PR means more potential for merge conflicts if other work touches same files

### Risks

| Risk                                              | Likelihood | Impact | Mitigation (if chosen)                                         |
| ------------------------------------------------- | ---------- | ------ | -------------------------------------------------------------- |
| Test fixture changes break pack-check             | Medium     | Med    | Run pack-check as gate; verify all 102 tests pass              |
| Merge conflicts with concurrent work              | Medium     | Low    | Coordinate timing; merge quickly after approval                |
| Walkthrough Flow 7 step adds scope creep          | Low        | Low    | Make walkthrough update optional; note in PR if skipped        |
| structure.rs "Six Flows" string is test assertion | Medium     | Med    | Verify whether "Six Flows" vs "Seven Flows" affects test logic |

### Assumptions

- **Pack-check test fixtures use string literals, not semantic assertions** - impact if wrong: Changing "Six Flows" to "Seven Flows" in test fixtures could break tests that expect specific strings
- **No files outside the impact_map.json scope reference flow count** - impact if wrong: Additional files would need updates

### When to Choose This

Choose this option when full consistency is mandatory and the team can absorb a larger review scope. Best for situations where the run will be the single source of truth and no follow-up is guaranteed.

---

## OPT-003: Layered Approach - Authoritative First, Generate/Validate Downstream

### Description

This option updates authoritative sources (CLAUDE.md) first, then uses the updated authoritative source to validate and update downstream docs in a controlled sequence. This mirrors the existing pack hierarchy where CLAUDE.md is "repo-level policy + shared contracts" and other docs should derive from it.

**Phase 1: Authoritative Sources**

- CLAUDE.md (expand flow table, confirm "7 flows" statement)
- docs/explanation/architecture.md (flow overlap semantics, Flow 7 purpose)

**Phase 2: Primary Public Docs (validated against Phase 1)**

- README.md
- DEMO_RUN.md
- CHANGELOG.md (annotation)

**Phase 3: Secondary Docs (optional, gated on time)**

- docs/reference/glossary.md
- CONTRIBUTING.md
- docs/how-to/work-without-github.md
- docs/tutorials/walkthrough.md

**Phase 4: Pack Tooling (if needed)**

- tools/demoswarm-pack-check/src/checks/structure.rs (only if pack-check fails)

Each phase produces a checkpoint commit, allowing the work to be merged after Phase 2 if time is constrained. Phases 3 and 4 can be follow-up commits or a separate PR.

### Requirements Fit

| Requirement   | Fit       | Notes                                                             |
| ------------- | --------- | ----------------------------------------------------------------- |
| REQ-001       | SATISFIED | Phases 1-2 cover all AC; Phase 3 extends to secondary             |
| REQ-002       | SATISFIED | Flow overlap in architecture.md (Phase 1)                         |
| REQ-003       | SATISFIED | Flow 7 purpose in architecture.md (Phase 1)                       |
| REQ-004       | SATISFIED | CLAUDE.md flow table in Phase 1                                   |
| REQ-005       | SATISFIED | Test count in Phase 1 or 2                                        |
| REQ-006       | SATISFIED | Security posture in Phase 1                                       |
| REQ-007       | SATISFIED | Color coding in Phase 1                                           |
| NFR-DOC-001   | PARTIAL   | Full satisfaction requires Phase 3; Phases 1-2 cover primary docs |
| NFR-SEC-001   | SATISFIED | Security claims in Phase 1 reference code                         |
| NFR-TRACE-001 | SATISFIED | Phase 4 addresses pack-check only if needed                       |

### Trade-offs

| Dimension                                        | Impact | Rationale                                                          |
| ------------------------------------------------ | ------ | ------------------------------------------------------------------ |
| Structure (coupling, components)                 | Low    | Phased approach isolates concerns; each phase is reviewable        |
| Velocity (time-to-first-change)                  | Med    | Faster than OPT-002 (can merge after Phase 2); slower than OPT-001 |
| Governance (auditability, determinism)           | Low    | Explicit derivation from authoritative source; clear lineage       |
| Operability (on-call, monitoring, failure modes) | Low    | No operational changes                                             |
| Cost (compute, complexity tax)                   | Med    | Multiple commits; but each phase is small and focused              |

### Reversibility

- Rating: Easy
- Switch effort: Each phase can be reverted independently; later phases depend on earlier
- Blast radius if wrong: Phase 1 errors would propagate to later phases; catch early

### Risks

| Risk                                               | Likelihood | Impact | Mitigation (if chosen)                                            |
| -------------------------------------------------- | ---------- | ------ | ----------------------------------------------------------------- |
| Phase 3/4 never completed                          | Medium     | Low    | Track as explicit follow-up; include in blockers if mandatory     |
| Phase 1 CLAUDE.md changes conflict with other work | Low        | Med    | CLAUDE.md rarely edited; coordinate if needed                     |
| Derivation logic is manual, not automated          | Medium     | Low    | Document the derivation in PR; consider automation in Wisdom flow |

### Assumptions

- **CLAUDE.md is the authoritative source for flow architecture** - impact if wrong: Would need to identify correct authoritative source and adjust Phase 1 target
- **Downstream docs can be mechanically derived from CLAUDE.md** - impact if wrong: Manual edits needed for each downstream file (still feasible, just more work)

### When to Choose This

Choose this option when the team values explicit derivation from authoritative sources and wants the option to merge incrementally. Best for situations where the pack hierarchy should be respected and future drift can be prevented by clear lineage.

---

## Comparison Matrix

| Dimension             | OPT-001                     | OPT-002            | OPT-003              |
| --------------------- | --------------------------- | ------------------ | -------------------- |
| REQ coverage (count)  | 7/7                         | 7/7                | 7/7                  |
| NFR coverage (count)  | 2/3                         | 3/3                | 2/3                  |
| Implementation effort | Low                         | High               | Medium               |
| Reversibility         | Easy                        | Easy               | Easy                 |
| Ops burden            | Low                         | Low                | Low                  |
| Primary risk          | Secondary docs inconsistent | Test fixture churn | Phase 3/4 incomplete |

## Suggested Default (non-binding)

suggested_default: OPT-003
confidence: Medium

Rationale (tie to IDs):

- Satisfies all 7 REQs (REQ-001 through REQ-007) in Phases 1-2
- Respects the pack hierarchy where CLAUDE.md is authoritative (per problem_statement.md L68)
- Provides clear derivation lineage from authoritative to downstream (supports NFR-DOC-001 MET-2)
- Allows incremental merge after Phase 2 if time is constrained
- Explicitly addresses NFR-TRACE-001 in Phase 4 only if pack-check fails (avoids unnecessary test churn)

What would change this:

- If NFR-DOC-001 is strictly mandatory and no follow-up is acceptable, prefer OPT-002
- If time is extremely constrained and secondary doc inconsistency is acceptable, prefer OPT-001
- If pack-check test fixtures are known to use "Six Flows" as semantic assertions (not just string literals), prefer OPT-001 to avoid code changes

## Open Questions Affecting Choice

- Q: Is NFR-DOC-001 ("zero matches for six flows in public documentation") strictly mandatory for merge, or can secondary docs be addressed in follow-up? - default if unanswered: Allow follow-up (choose OPT-003 or OPT-001)
- Q: Does the "Six Flows" string in structure.rs test fixtures affect test logic, or is it just a string literal in test data? - default if unanswered: Assume string literal; no code change needed unless pack-check fails (choose OPT-003)
- Q: Is there a deadline that would preclude the comprehensive sweep (OPT-002)? - default if unanswered: No hard deadline; OPT-003 provides flexibility

## Shared Assumptions

- Seven-flow model in CLAUDE.md L13 is canonical; "six flow" references elsewhere are stale (ASM-001 from requirements.md)
- Flow variants (flow-4-gate vs flow-4-review, etc.) are intentional alternate entry points (ASM-002 from requirements.md)
- 102 passing unit tests from test_output.log is the authoritative count (ASM-003 from requirements.md)
- Path traversal in secrets.rs is a documentation concern, not an immediate security fix (ASM-004 from requirements.md)
- Agent color coding is functional metadata, not purely decorative (ASM-005 from requirements.md)
- Pack-check tests (wisdom.rs) will continue to pass as they validate wisdom markers, not flow count strings

---

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

missing_required: []

blockers: []

options_proposed: 3
suggested_default: OPT-003
confidence: Medium
