# Acceptance Criteria Matrix

## Machine Summary
ac_count: 32
requirements_covered: 10
scenarios_covered: 32

## AC Inventory

| AC-ID | Source | Description | Priority | Test Types | Impl Hints | Verification |
|-------|--------|-------------|----------|------------|------------|--------------|
| AC-001-001 | @REQ-001, flow_count_alignment.feature:11 | README.md references "seven flows" in section header and body text | P0 | Grep | README.md L67 section header | `grep "seven flows" README.md` returns match |
| AC-001-002 | @REQ-001, flow_count_alignment.feature:18 | DEMO_RUN.md references "seven flows" and enumerates Signal through Wisdom | P1 | Grep | DEMO_RUN.md L14 | `grep "seven flows" DEMO_RUN.md` returns match |
| AC-001-003 | @REQ-001, flow_count_alignment.feature:24 | docs/explanation/architecture.md references "seven flows" in section header and enumeration | P1 | Grep | architecture.md L11, L62 | `grep "seven flows" docs/explanation/architecture.md` returns match |
| AC-001-004 | @REQ-001, flow_count_alignment.feature:31 | CHANGELOG.md v1.0.0 annotated to clarify "10 command files implementing 7 flows" | P2 | Manual | CHANGELOG.md v1.0.0 section | Review annotation text |
| AC-001-005 | @REQ-001, flow_count_alignment.feature:38 | No occurrences of "six flows" in README.md, DEMO_RUN.md, architecture.md | P0 | Grep | All public doc files | `grep -r "six flows" [files]` returns 0 |
| AC-002-001 | @REQ-002, flow_overlap_documentation.feature:11 | Documentation explains flow-4-gate vs flow-4-review relationship | P0 | Manual | architecture.md flow overlap section | Review explains different entry points |
| AC-002-002 | @REQ-002, flow_overlap_documentation.feature:17 | Documentation explains flow-5-gate vs flow-5-deploy relationship | P1 | Manual | architecture.md flow overlap section | Review explains gate re-entry vs deployment |
| AC-002-003 | @REQ-002, flow_overlap_documentation.feature:24 | Documentation explains flow-6-deploy vs flow-6-wisdom relationship | P1 | Manual | architecture.md flow overlap section | Review explains deploy vs wisdom paths |
| AC-002-004 | @REQ-002, flow_overlap_documentation.feature:31 | Flow variant guidance includes specific examples ("use flow-4-review after PR feedback") | P1 | Manual | architecture.md flow overlap section | Review for actionable guidance |
| AC-002-005 | @REQ-002, flow_overlap_documentation.feature:38 | Flow overlap documentation is present in README.md or architecture.md | P2 | Grep | README.md or architecture.md | Verify section exists or is linked |
| AC-003-001 | @REQ-003, flow_overlap_documentation.feature:45 | Flow 7 is included in flow enumeration in README or architecture docs | P0 | Grep | README.md, architecture.md | `grep -E "Flow 7|flow-7-wisdom" [files]` returns match |
| AC-003-002 | @REQ-003, flow_overlap_documentation.feature:52 | Flow 7 purpose (second-cycle wisdom extraction) is documented | P1 | Manual | architecture.md Flow 7 section | Review explains when to invoke |
| AC-003-003 | @REQ-003, flow_overlap_documentation.feature:57 | Difference between flow-7-wisdom and flow-6-wisdom is explained | P1 | Manual | architecture.md | Review differentiation guidance |
| AC-003-004 | @REQ-003, flow_overlap_documentation.feature:63 | Missing Flow 7 documentation gap is identified (pre-condition) | P2 | Grep | Pre-implementation check | Confirm gap exists before fix |
| AC-004-001 | @REQ-004, flow_count_alignment.feature:43 | CLAUDE.md flow table lists all seven flows (Signal through Wisdom) | P0 | pack-check | CLAUDE.md flow table L186-196 | pack-check validates CLAUDE.md structure |
| AC-004-002 | @REQ-004, flow_count_alignment.feature:54 | CLAUDE.md flow table includes variant commands (flow-4-review, flow-5-gate, flow-6-deploy, flow-7-wisdom) | P1 | pack-check, Manual | CLAUDE.md flow table | Verify variants listed inline or referenced |
| AC-004-003 | @REQ-004, flow_count_alignment.feature:62 | CLAUDE.md flow table numbering matches "7 flows" statement at L13 | P1 | pack-check | CLAUDE.md L13 and flow table | No skipped/duplicated flow numbers |
| AC-005-001 | @REQ-005, test_count_documentation.feature:11 | Documentation references "102 unit tests passing" as current count | P1 | Grep | architecture.md or CLAUDE.md | `grep "102" [target-file]` returns match |
| AC-005-002 | @REQ-005, test_count_documentation.feature:17 | Documentation explains 277 tests are filtered (integration tests requiring setup) | P2 | Manual | architecture.md | Review filtered tests explanation |
| AC-005-003 | @REQ-005, test_count_documentation.feature:24 | Conflicting test count claims (e.g., "374 tests") are corrected or annotated | P1 | Grep | All docs | `grep -E "374|test.*pass" [docs]` returns no conflicts |
| AC-005-004 | @REQ-005, test_count_documentation.feature:31 | Test count claims include source artifact reference (test_output.log) | P2 | Manual | architecture.md | Verify source reference present |
| AC-005-005 | @REQ-005, test_count_documentation.feature:37 | Undocumented test count source gap identified (pre-condition) | P2 | Manual | Pre-implementation check | Confirm gap exists before fix |
| AC-006-001 | @REQ-006, security_posture_documentation.feature:11 | Documentation states Rust regex crate is immune to ReDoS (finite automata) | P0 | Grep | architecture.md security section | `grep -i "redos\|immune" [file]` returns immunity claim |
| AC-006-002 | @REQ-006, security_posture_documentation.feature:17 | Path traversal documented as known limitation pending threat assessment | P1 | Manual | architecture.md security section | Review limitation documentation |
| AC-006-003 | @REQ-006, security_posture_documentation.feature:24 | Invalid ReDoS vulnerability claim is removed/corrected | P0 | Grep | All docs | `grep -i "redos.*vuln" [docs]` returns 0 |
| AC-006-004 | @REQ-006, security_posture_documentation.feature:31 | Security claims reference code evidence (file:line) | P1 | Manual | architecture.md | Verify secrets.rs L14 reference |
| AC-006-005 | @REQ-006, security_posture_documentation.feature:38 | Security claims are verifiable by code inspection | P1 | Manual | Cross-reference with secrets.rs | Confirm claim matches code |
| AC-007-001 | @REQ-007, agent_color_coding.feature:11 | Documentation acknowledges agent frontmatter includes color field | P2 | Manual | architecture.md or agent docs | Review acknowledgment |
| AC-007-002 | @REQ-007, agent_color_coding.feature:17 | Documentation clarifies color coding is advisory (human consumption) not schema-validated | P2 | Manual | architecture.md or agent docs | Review purpose clarification |
| AC-007-003 | @REQ-007, agent_color_coding.feature:24 | If color is functional, documentation specifies consumer (tooling/UI) | P2 | Manual | Conditional | Review consumer specification |
| AC-007-004 | @REQ-007, agent_color_coding.feature:31 | Example agent frontmatter in documentation includes color field | P2 | Manual | architecture.md | Review frontmatter example |
| AC-007-005 | @REQ-007, agent_color_coding.feature:36 | Incorrect "documentation-only" claim corrected if color is functional | P2 | Manual | Conditional | Review correction if applicable |

## Column Definitions

- **AC-ID**: Stable identifier (AC-{REQ}-{sequence}). Flow 3 references these.
- **Source**: Traceability back to REQ tags and feature file:line.
- **Description**: One-sentence statement of what "done" looks like.
- **Priority**: P0 (must not fail) / P1 (primary path) / P2 (secondary).
- **Test Types**: From test_plan.md mapping (Grep, pack-check, Manual).
- **Impl Hints**: Which file/section is likely affected.
- **Verification**: How Flow 3 confirms this AC is satisfied.

## NFR Acceptance Criteria

| AC-ID | Source | Description | Priority | Test Types | Impl Hints | Verification |
|-------|--------|-------------|----------|------------|------------|--------------|
| AC-NFR-DOC-001 | NFR-DOC-001 MET-1, MET-2 | All public docs agree on "seven flows"; no "six flows" matches | P0 | Grep | README, DEMO_RUN, architecture.md | `grep -r "six flows" [files]` returns 0 |
| AC-NFR-SEC-001 | NFR-SEC-001 MET-1, MET-2 | Security claims have code evidence references | P0 | Manual | architecture.md security section | Each claim has file:line reference |
| AC-NFR-TRACE-001 | NFR-TRACE-001 MET-1, MET-2 | pack-check passes after all documentation updates | P0 | pack-check | N/A | `bash .claude/scripts/pack-check.sh` exits 0 |

## Implementation Order

Recommended sequence for Flow 3 (respects dependencies per ADR OPT-003 phases):

### Phase 1: Authoritative Sources (CLAUDE.md + architecture.md)

Dependencies: None (foundational)

1. **AC-004-001** - CLAUDE.md flow table lists all seven flows
2. **AC-004-002** - CLAUDE.md flow table includes variant commands
3. **AC-004-003** - CLAUDE.md flow table numbering is consistent
4. **AC-002-001** - Flow overlap: flow-4-gate vs flow-4-review
5. **AC-002-002** - Flow overlap: flow-5-gate vs flow-5-deploy
6. **AC-002-003** - Flow overlap: flow-6-deploy vs flow-6-wisdom
7. **AC-002-004** - Flow variant guidance is actionable
8. **AC-003-001** - Flow 7 included in enumeration
9. **AC-003-002** - Flow 7 purpose documented
10. **AC-003-003** - Flow 7 vs Flow 6 difference explained
11. **AC-006-001** - ReDoS immunity documentation
12. **AC-006-002** - Path traversal known limitation
13. **AC-006-004** - Security claims reference code evidence
14. **AC-006-005** - Security claims verifiable by inspection
15. **AC-005-001** - Test count (102 passing)
16. **AC-005-002** - Filtered tests explanation
17. **AC-005-004** - Test count source reference

**Checkpoint**: Phase 1 commit

### Phase 2: Primary Public Docs (README.md, DEMO_RUN.md, CHANGELOG.md)

Dependencies: Phase 1 complete (authoritative sources establish content)

18. **AC-001-001** - README references seven flows
19. **AC-001-002** - DEMO_RUN references seven flows
20. **AC-001-003** - Architecture documentation references seven flows
21. **AC-001-004** - CHANGELOG annotation
22. **AC-001-005** - No stale "six flows" references
23. **AC-002-005** - Flow overlap documentation discoverable
24. **AC-NFR-DOC-001** - Cross-file consistency verified

**Checkpoint**: Phase 2 commit

### Phase 3: Secondary Docs (optional, time-gated)

Dependencies: Phase 2 complete

25. **AC-007-001** - Color field acknowledgment
26. **AC-007-002** - Color purpose clarification
27. **AC-007-003** - Color consumer documentation (conditional)
28. **AC-007-004** - Example frontmatter with color
29. Secondary doc updates: glossary.md, CONTRIBUTING.md, work-without-github.md, walkthrough.md

**Checkpoint**: Phase 3 commit (if executed)

### Phase 4: Pack Tooling (reactive, only if needed)

Dependencies: Phase 2 complete; triggered only if pack-check fails

30. **AC-NFR-TRACE-001** - pack-check validation
31. Structure.rs test fixtures (if pack-check fails on "Six Flows" assertion)
32. Control_plane.rs updates (if needed)

**Checkpoint**: Phase 4 commit (if executed)

## Verification Checklist (Gate)

Flow 5 uses this checklist to audit completion:

### P0 Acceptance Criteria (must pass for MERGE)

- [ ] AC-001-001: `grep "seven flows" README.md` returns match
- [ ] AC-001-005: `grep -r "six flows" README.md DEMO_RUN.md docs/explanation/architecture.md` returns 0
- [ ] AC-002-001: Flow overlap section explains flow-4-gate vs flow-4-review
- [ ] AC-003-001: `grep -E "Flow 7|flow-7-wisdom" README.md docs/explanation/architecture.md` returns match
- [ ] AC-004-001: pack-check validates CLAUDE.md flow table structure
- [ ] AC-006-001: `grep -i "redos.*immune\|finite automata" docs/explanation/architecture.md` returns match
- [ ] AC-006-003: `grep -i "redos.*vuln" README.md docs/explanation/architecture.md` returns 0
- [ ] AC-NFR-DOC-001: All public docs agree on "seven flows"
- [ ] AC-NFR-SEC-001: Security claims have code evidence references
- [ ] AC-NFR-TRACE-001: `bash .claude/scripts/pack-check.sh` exits 0

### P1 Acceptance Criteria (should pass for MERGE)

- [ ] AC-001-002: DEMO_RUN references seven flows
- [ ] AC-001-003: architecture.md references seven flows
- [ ] AC-002-002: Flow overlap explains flow-5-gate vs flow-5-deploy
- [ ] AC-002-003: Flow overlap explains flow-6-deploy vs flow-6-wisdom
- [ ] AC-002-004: Flow variant guidance is actionable
- [ ] AC-003-002: Flow 7 purpose documented
- [ ] AC-003-003: Flow 7 vs Flow 6 difference explained
- [ ] AC-004-002: CLAUDE.md includes variant commands
- [ ] AC-004-003: CLAUDE.md numbering consistent
- [ ] AC-005-001: Test count (102) documented
- [ ] AC-005-003: No conflicting test count claims
- [ ] AC-006-002: Path traversal documented as limitation
- [ ] AC-006-004: Security claims reference code evidence
- [ ] AC-006-005: Security claims verifiable

### P2 Acceptance Criteria (nice to have)

- [ ] AC-001-004: CHANGELOG annotated
- [ ] AC-002-005: Flow overlap discoverable
- [ ] AC-005-002: Filtered tests explained
- [ ] AC-005-004: Test count source referenced
- [ ] AC-007-001 through AC-007-005: Color coding documented

## Notes

- Each AC should be completable in one documentation edit microloop.
- If an AC requires multiple file changes, complete all changes for that AC before marking complete.
- Flow 3 creates `.runs/local-alignment-audit-aba1c6/build/ac_status.json` and updates it as it completes each AC.
- Pre-condition ACs (AC-003-004, AC-005-005, AC-007-005) confirm gaps exist before fixes are applied; these pass automatically once the fix ACs are complete.
- Phase 3 and Phase 4 ACs may be deferred per ADR OPT-003; track as follow-up if not completed.

## Cross-References

- Test Plan: `.runs/local-alignment-audit-aba1c6/plan/test_plan.md`
- Requirements: `.runs/local-alignment-audit-aba1c6/signal/requirements.md`
- Feature Files: `.runs/local-alignment-audit-aba1c6/signal/features/*.feature`
- ADR: `.runs/local-alignment-audit-aba1c6/plan/adr.md`
- Impact Map: `.runs/local-alignment-audit-aba1c6/plan/impact_map.json`
