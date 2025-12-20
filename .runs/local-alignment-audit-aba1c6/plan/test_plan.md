# Test Plan

## Machine Summary
status: VERIFIED
missing_required: []
blockers: []
concerns:
  - Mutation testing not applicable for documentation-only work
  - "Tests" are verification commands and manual review checklists, not executable code tests
  - Phase 3/4 scenarios depend on time-gated scope per ADR OPT-003
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

counts:
  scenarios_total: 32
  requirements_total: 10
  requirements_with_scenarios: 7
  ac_count: 32

severity_summary:
  critical: 0
  major: 0
  minor: 0

## Scope

### What this plan covers
- All 32 BDD scenarios from 5 feature files in Signal
- 7 functional requirements (REQ-001 through REQ-007)
- 3 non-functional requirements (NFR-DOC-001, NFR-SEC-001, NFR-TRACE-001)
- Verification strategy for documentation alignment audit
- Test types mapped to documentation verification (grep, pack-check, manual review)

### What this plan explicitly does NOT cover
- Executable unit tests (no code changes in scope)
- Integration tests requiring test harness
- E2E automation (documentation work, not behavioral)
- Mutation testing (no code logic to mutate)
- Performance testing (not applicable)

## Coverage Thresholds

Stable markers (required for coverage-enforcer to parse mechanically):
- COVERAGE_LINE_REQUIRED: null
- COVERAGE_BRANCH_REQUIRED: null
- COVERAGE_CRITICAL_PATH: null

Additional notes:
- measurement_notes: Coverage thresholds not applicable for documentation-only work. This run produces documentation changes, not code changes. Verification is via grep searches, pack-check validation, and manual review checklists.

## Mutation Testing
- mutation_required: false
- mutation_threshold: null
- mutation_scope: []
- mutation_tool_hint: null
- rationale: Documentation-only work; no code logic to mutate. All changes are markdown content updates, not executable code.

## Scenario to Test Type Matrix

| REQ | Feature File | Scenario | Priority | Unit | Integration | Contract | E2E | Fuzz | Perf/Obs | Notes |
|-----|--------------|----------|----------|------|-------------|----------|-----|------|----------|-------|
| REQ-001 | flow_count_alignment.feature | README references seven flows | P0 | - | - | - | - | - | Grep | `grep "seven flows" README.md`; smoke |
| REQ-001 | flow_count_alignment.feature | DEMO_RUN references seven flows with enumeration | P1 | - | - | - | - | - | Grep | `grep "seven flows" DEMO_RUN.md` |
| REQ-001 | flow_count_alignment.feature | Architecture documentation references seven flows | P1 | - | - | - | - | - | Grep | `grep "seven flows" docs/explanation/architecture.md` |
| REQ-001 | flow_count_alignment.feature | CHANGELOG clarifies actual command count | P2 | - | - | - | - | - | Manual | Review annotation for "10 command files implementing 7 flows" |
| REQ-001 | flow_count_alignment.feature | No stale flow count references remain | P0 | - | - | - | - | - | Grep | `grep -r "six flows" README.md DEMO_RUN.md docs/explanation/architecture.md` returns 0; edge |
| REQ-004 | flow_count_alignment.feature | CLAUDE.md flow table lists all seven flows | P0 | - | - | pack-check | - | - | - | pack-check validates CLAUDE.md structure; smoke |
| REQ-004 | flow_count_alignment.feature | CLAUDE.md flow table includes variant commands | P1 | - | - | pack-check | - | - | - | Verify variants in table |
| REQ-004 | flow_count_alignment.feature | CLAUDE.md flow table numbering is consistent | P1 | - | - | pack-check | - | - | - | No skipped/duplicated flow numbers |
| REQ-002 | flow_overlap_documentation.feature | Documentation explains flow-4-gate vs flow-4-review | P0 | - | - | - | - | - | Manual | Review flow overlap section; smoke |
| REQ-002 | flow_overlap_documentation.feature | Documentation explains flow-5-gate vs flow-5-deploy | P1 | - | - | - | - | - | Manual | Review flow overlap section |
| REQ-002 | flow_overlap_documentation.feature | Documentation explains flow-6-deploy vs flow-6-wisdom | P1 | - | - | - | - | - | Manual | Review flow overlap section |
| REQ-002 | flow_overlap_documentation.feature | Flow variant guidance is actionable | P1 | - | - | - | - | - | Manual | Review for specific "when to use" examples; edge |
| REQ-002 | flow_overlap_documentation.feature | Flow overlap documentation is discoverable | P2 | - | - | - | - | - | Grep | Verify link/presence in README or architecture.md |
| REQ-003 | flow_overlap_documentation.feature | Flow 7 is included in flow enumeration | P0 | - | - | - | - | - | Grep | `grep -E "Flow 7|flow-7-wisdom" README.md docs/explanation/architecture.md`; smoke |
| REQ-003 | flow_overlap_documentation.feature | Flow 7 purpose is documented | P1 | - | - | - | - | - | Manual | Review Flow 7 documentation section |
| REQ-003 | flow_overlap_documentation.feature | Flow 7 vs Flow 6 wisdom difference is explained | P1 | - | - | - | - | - | Manual | Review differentiation guidance |
| REQ-003 | flow_overlap_documentation.feature | Missing Flow 7 documentation is flagged | P2 | - | - | - | - | - | Grep | Pre-condition check; error case |
| REQ-005 | test_count_documentation.feature | Documentation references correct passing test count | P1 | - | - | - | - | - | Grep | `grep "102" [target-file]`; smoke |
| REQ-005 | test_count_documentation.feature | Documentation explains filtered tests | P2 | - | - | - | - | - | Manual | Review explanation of 277 filtered tests |
| REQ-005 | test_count_documentation.feature | Conflicting test count claims are corrected | P1 | - | - | - | - | - | Grep | `grep -E "374|test.*pass" [docs]` returns no conflicts; edge |
| REQ-005 | test_count_documentation.feature | Test count claims include source reference | P2 | - | - | - | - | - | Manual | Verify test_output.log reference |
| REQ-005 | test_count_documentation.feature | Undocumented test count source is flagged | P2 | - | - | - | - | - | Manual | Pre-condition check; error case |
| REQ-006 | security_posture_documentation.feature | Documentation states Rust regex is ReDoS immune | P0 | - | - | - | - | - | Grep | `grep -i "redos\|immune\|finite automata" [security-doc]`; smoke |
| REQ-006 | security_posture_documentation.feature | Path traversal is documented as known limitation | P1 | - | - | - | - | - | Manual | Review security posture section |
| REQ-006 | security_posture_documentation.feature | Invalid ReDoS vulnerability claim is corrected | P0 | - | - | - | - | - | Grep | `grep -i "redos.*vuln" [docs]` returns 0; error case |
| REQ-006 | security_posture_documentation.feature | Security claims reference code evidence | P1 | - | - | - | - | - | Manual | Verify file:line references |
| REQ-006 | security_posture_documentation.feature | Security claims are verifiable by code inspection | P1 | - | - | - | - | - | Manual | Cross-reference with secrets.rs; edge |
| REQ-007 | agent_color_coding.feature | Documentation acknowledges color field existence | P2 | - | - | - | - | - | Manual | Review agent documentation; smoke |
| REQ-007 | agent_color_coding.feature | Documentation clarifies color field purpose | P2 | - | - | - | - | - | Manual | Verify advisory vs schema-validated statement |
| REQ-007 | agent_color_coding.feature | Color consumer is documented if functional | P2 | - | - | - | - | - | Manual | Conditional verification; edge |
| REQ-007 | agent_color_coding.feature | Example agent frontmatter includes color field | P2 | - | - | - | - | - | Manual | Review frontmatter examples |
| REQ-007 | agent_color_coding.feature | Incorrect documentation-only claim is corrected | P2 | - | - | - | - | - | Manual | Pre-condition check; error case |

## Requirement Coverage Summary

| Requirement | Scenarios | Priority | Required Test Types | Notes |
|-------------|-----------|----------|---------------------|-------|
| REQ-001 | 5 | P0-P2 | Grep, Manual | Flow count updates across public docs |
| REQ-002 | 5 | P0-P2 | Manual, Grep | Flow overlap semantics documentation |
| REQ-003 | 4 | P0-P2 | Grep, Manual | Flow 7 purpose and usage |
| REQ-004 | 3 | P0-P1 | pack-check | CLAUDE.md flow table validation |
| REQ-005 | 5 | P1-P2 | Grep, Manual | Test count documentation accuracy |
| REQ-006 | 5 | P0-P1 | Grep, Manual | Security posture documentation |
| REQ-007 | 5 | P2 | Manual | Agent color coding clarification |
| NFR-DOC-001 | - | P0 | Grep | Cross-file consistency: `grep "six flows"` returns 0 |
| NFR-SEC-001 | - | P0 | Manual | Security claims have code evidence |
| NFR-TRACE-001 | - | P0 | pack-check | `pack-check` validation passes |

## Contract Test Plan

No API contracts in scope (documentation-only work). The structural contract is CLAUDE.md as validated by pack-check.

### pack-check Validation (NFR-TRACE-001)
- Command: `bash .claude/scripts/pack-check.sh --no-color`
- Pass criteria: Exit code 0
- Specific checks:
  - wisdom.rs checks continue to pass (Flow 7 validation)
  - contracts.rs "Seven Flows" constant alignment
  - CLAUDE.md structural validation

## Non-Behavioral Verification (from verification_notes.md)

### NFR-DOC-001: Documentation Consistency
- Verification: `grep -r "six flows" README.md DEMO_RUN.md docs/explanation/architecture.md` returns 0 matches
- When: Gate
- Method: Automated grep

### NFR-SEC-001: Security Claims Evidence
- Verification: Each security claim references specific source file and line number
- When: Gate
- Method: Manual review
- Pass criteria:
  - ReDoS immunity claim references secrets.rs and Rust regex crate usage
  - Path traversal limitation claim references secrets.rs path handling

### NFR-TRACE-001: Pack-Check Test Continuity
- Verification: `bash .claude/scripts/pack-check.sh --no-color`
- When: Gate
- Method: Automated execution
- Pass criteria: Exit code 0, wisdom.rs checks pass

## Test Type Definitions (Documentation Context)

For this documentation alignment audit, test types map to verification methods:

| Test Type | Verification Method | Tooling | When Run |
|-----------|--------------------|---------|----|
| Grep | Automated text search | `grep`, `rg`, or demoswarm CLI | Build, Gate |
| pack-check | Structural validation | `bash .claude/scripts/pack-check.sh` | Gate |
| Manual | Human review checklist | Documentation review | Gate |

### Priority Definitions

| Priority | Definition | Examples |
|----------|------------|----------|
| P0 | Must not fail; blocks merge | REQ-001 "no stale six flows", REQ-006 "no ReDoS vulnerability claim" |
| P1 | Primary user path | Flow enumeration, security posture documentation |
| P2 | Secondary behavior | CHANGELOG annotation, color coding clarification |

## Verification Commands Reference

### Flow Count Verification (REQ-001, NFR-DOC-001)
```bash
# Verify no "six flows" in public docs
grep -r "six flows" README.md DEMO_RUN.md docs/explanation/architecture.md
# Expected: No matches

# Verify "seven flows" in README
grep "seven flows" README.md
# Expected: Match in section header and/or body

# Verify "seven flows" in architecture.md
grep "seven flows" docs/explanation/architecture.md
# Expected: Match in section header and flow enumeration
```

### Flow 7 Verification (REQ-003)
```bash
# Verify Flow 7 mentioned in public docs
grep -E "Flow 7|flow-7-wisdom" README.md docs/explanation/architecture.md CLAUDE.md
# Expected: At least one match per file
```

### Security Posture Verification (REQ-006)
```bash
# Verify ReDoS immunity is documented
grep -i "redos" docs/explanation/architecture.md
# Expected: Match explaining immunity (not vulnerability)

# Verify no ReDoS vulnerability claim remains
grep -i "redos.*vulnerab" README.md docs/explanation/architecture.md
# Expected: No matches
```

### Pack-Check Verification (NFR-TRACE-001)
```bash
# Run pack-check
bash .claude/scripts/pack-check.sh --no-color
# Expected: Exit code 0, all checks pass
```

## Gaps and Questions

### Open Questions Affecting Test Strategy

- Q: OQ-PLAN-002 - Should Flow 7 documentation explicitly reference "second-cycle" use case?
  - Suggested default: Yes, explicitly describe as "second-cycle wisdom extraction for multi-iteration runs"
  - Impact: If generic, test for "second-cycle" terminology would need adjustment

- Q: OQ-PLAN-003 - Should compliance partitioning include ST-007 for Flow 7?
  - Suggested default: Add ST-007 for completeness
  - Impact: If ST-006 covers both, no separate Flow 7 compliance check needed

### Assumptions in Test Strategy

- ASM-001: Seven-flow model is canonical (CLAUDE.md authoritative)
- ASM-003: 102 passing tests from test_output.log is authoritative count
- ASM-005: Agent color coding is advisory metadata, not schema-enforced

## Recommended Next

Flow 3 should implement in this order (per ADR OPT-003 phases):

### Phase 1: Authoritative Sources (P0)
1. AC-004-001: CLAUDE.md flow table lists all seven flows
2. AC-004-002: CLAUDE.md flow table includes variant commands
3. AC-004-003: CLAUDE.md flow table numbering is consistent
4. AC-002-001: Flow overlap semantics (flow-4-gate vs flow-4-review)
5. AC-002-002: Flow overlap semantics (flow-5-gate vs flow-5-deploy)
6. AC-002-003: Flow overlap semantics (flow-6-deploy vs flow-6-wisdom)
7. AC-003-001: Flow 7 is included in enumeration
8. AC-003-002: Flow 7 purpose is documented
9. AC-003-003: Flow 7 vs Flow 6 difference explained
10. AC-006-001: ReDoS immunity documentation
11. AC-006-002: Path traversal known limitation
12. AC-005-001: Test count (102 passing)
13. AC-005-002: Filtered tests explanation

### Phase 2: Primary Public Docs (P1)
14. AC-001-001: README references seven flows
15. AC-001-002: DEMO_RUN references seven flows
16. AC-001-003: Architecture documentation references seven flows
17. AC-001-004: CHANGELOG annotation
18. AC-001-005: No stale "six flows" references

### Phase 3: Secondary Docs (P2, time-gated)
19-24. Secondary documentation updates (glossary.md, CONTRIBUTING.md, etc.)

### Phase 4: Pack Tooling (reactive)
25-32. Pack-check fixtures (only if pack-check fails after Phase 2)

## Cross-References

- Requirements: `.runs/local-alignment-audit-aba1c6/signal/requirements.md`
- Feature Files: `.runs/local-alignment-audit-aba1c6/signal/features/*.feature`
- Verification Notes: `.runs/local-alignment-audit-aba1c6/signal/verification_notes.md`
- Example Matrix: `.runs/local-alignment-audit-aba1c6/signal/example_matrix.md`
- Early Risks: `.runs/local-alignment-audit-aba1c6/signal/early_risks.md`
- Risk Assessment: `.runs/local-alignment-audit-aba1c6/signal/risk_assessment.md`
- ADR: `.runs/local-alignment-audit-aba1c6/plan/adr.md`
- Impact Map: `.runs/local-alignment-audit-aba1c6/plan/impact_map.json`
- AC Matrix: `.runs/local-alignment-audit-aba1c6/plan/ac_matrix.md`
