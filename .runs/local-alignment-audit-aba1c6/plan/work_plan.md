# Work Plan for local-alignment-audit-aba1c6

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []
missing_required:
  - .runs/local-alignment-audit-aba1c6/plan/test_plan.md (not found; using verification_notes.md and BDD scenarios as substitute)
  - .runs/local-alignment-audit-aba1c6/plan/observability_spec.md (not found; documentation-only work has no runtime observability needs)

## Scope Snapshot
- **ADR decision**: OPT-003 Layered Approach - Update authoritative sources (CLAUDE.md, architecture.md) first, then derive downstream docs in phases.
- **Primary impacts**:
  - IMP-005: CLAUDE.md flow table needs expansion to 7 flows with variants
  - IMP-003: architecture.md needs flow overlap semantics and Flow 7 documentation
  - IMP-001: README.md section header "six flows" -> "seven flows"
  - IMP-002: DEMO_RUN.md flow count reference update
  - IMP-004: CHANGELOG.md annotation for historical clarity
- **Key constraints**:
  - Seven-flow model is canonical (CLAUDE.md L13 is authoritative)
  - No code logic changes in scope (documentation-only)
  - Pack-check must continue passing (NFR-TRACE-001)
  - Security claims require code evidence (NFR-SEC-001)
  - Per-phase commits for granular revert capability
- **Verification posture**: grep "six flows" returns zero matches in public docs; pack-check exits 0; security claims reference source files

## Subtask Index (parseable)

Write this YAML block verbatim to `.runs/local-alignment-audit-aba1c6/plan/subtasks.yaml`:

```yaml
schema_version: subtasks_v1
subtasks:
  - id: ST-001
    title: "Update CLAUDE.md flow table to seven flows with variants"
    status: TODO
    depends_on: []
    req_ids: ["REQ-004"]
    nfr_ids: ["NFR-DOC-001"]
    acceptance_criteria:
      - "CLAUDE.md flow table lists all seven flows (Signal, Plan, Build, Review, Gate, Deploy, Wisdom)"
      - "Flow table includes variant commands (flow-4-review, flow-5-gate, flow-6-deploy, flow-7-wisdom) with brief descriptions"
      - "Flow table numbering is consistent with '7 flows' statement in CLAUDE.md L13"
      - "Each variant has a brief description of its use case"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["CLAUDE.md"]
      allow_new_files_under: []
    touches: ["CLAUDE.md"]
    tests: ["grep 'seven flows' CLAUDE.md returns match", "pack-check passes"]
    observability: []
    estimate: S
  - id: ST-002
    title: "Document flow overlap semantics and Flow 7 purpose in architecture.md"
    status: TODO
    depends_on: ["ST-001"]
    req_ids: ["REQ-002", "REQ-003"]
    nfr_ids: ["NFR-DOC-001"]
    acceptance_criteria:
      - "Explains relationship between /flow-4-gate and /flow-4-review"
      - "Explains relationship between /flow-5-gate and /flow-5-deploy"
      - "Explains relationship between /flow-6-deploy and /flow-6-wisdom"
      - "Provides guidance on when to use each variant"
      - "Flow 7 purpose documented as second-cycle wisdom extraction"
      - "Section header updated from 'six flows' to 'seven flows'"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["docs/explanation/architecture.md"]
      allow_new_files_under: []
    touches: ["docs/explanation/architecture.md"]
    tests: ["grep 'six flows' docs/explanation/architecture.md returns zero matches"]
    observability: []
    estimate: M
  - id: ST-003
    title: "Update security posture and test count documentation"
    status: TODO
    depends_on: ["ST-001"]
    req_ids: ["REQ-005", "REQ-006"]
    nfr_ids: ["NFR-SEC-001"]
    acceptance_criteria:
      - "Rust regex crate ReDoS immunity documented with code reference"
      - "Path traversal noted as known limitation"
      - "102 unit tests documented with source artifact reference"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["docs/explanation/architecture.md"]
      allow_new_files_under: []
    touches: ["docs/explanation/architecture.md"]
    tests: ["Security claims reference secrets.rs"]
    observability: []
    estimate: S
  - id: ST-004
    title: "Clarify agent color coding purpose"
    status: TODO
    depends_on: ["ST-001"]
    req_ids: ["REQ-007"]
    nfr_ids: []
    acceptance_criteria:
      - "Color coding acknowledged as advisory metadata"
      - "Example frontmatter includes color: field"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["docs/explanation/architecture.md"]
      allow_new_files_under: []
    touches: ["docs/explanation/architecture.md"]
    tests: []
    observability: []
    estimate: S
  - id: ST-005
    title: "Update README.md flow count references"
    status: TODO
    depends_on: ["ST-001", "ST-002"]
    req_ids: ["REQ-001"]
    nfr_ids: ["NFR-DOC-001"]
    acceptance_criteria:
      - "Section header 'seven flows'"
      - "No 'six flows' occurrences"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["README.md"]
      allow_new_files_under: []
    touches: ["README.md"]
    tests: ["grep 'six flows' README.md returns zero"]
    observability: []
    estimate: S
  - id: ST-006
    title: "Update DEMO_RUN.md flow count references"
    status: TODO
    depends_on: ["ST-001"]
    req_ids: ["REQ-001"]
    nfr_ids: ["NFR-DOC-001"]
    acceptance_criteria:
      - "References 'seven flows'"
      - "No 'six flows' occurrences"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["DEMO_RUN.md"]
      allow_new_files_under: []
    touches: ["DEMO_RUN.md"]
    tests: ["grep 'six flows' DEMO_RUN.md returns zero"]
    observability: []
    estimate: S
  - id: ST-007
    title: "Annotate CHANGELOG.md v1.0.0 entry"
    status: TODO
    depends_on: ["ST-001"]
    req_ids: ["REQ-001"]
    nfr_ids: []
    acceptance_criteria:
      - "v1.0.0 annotated to clarify evolution to 7 flows"
      - "Historical accuracy preserved"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["CHANGELOG.md"]
      allow_new_files_under: []
    touches: ["CHANGELOG.md"]
    tests: []
    observability: []
    estimate: S
  - id: ST-008
    title: "Update secondary documentation (glossary, CONTRIBUTING, work-without-github)"
    status: TODO
    depends_on: ["ST-005", "ST-006"]
    req_ids: []
    nfr_ids: ["NFR-DOC-001"]
    acceptance_criteria:
      - "All secondary docs updated to '7 flows'"
      - "No '6 flows' occurrences"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: ["docs/reference/glossary.md", "CONTRIBUTING.md", "docs/how-to/work-without-github.md"]
      allow_new_files_under: []
    touches: ["docs/reference/glossary.md", "CONTRIBUTING.md", "docs/how-to/work-without-github.md"]
    tests: ["grep '6 flows' returns zero in all"]
    observability: []
    estimate: S
  - id: ST-009
    title: "Verify pack-check passes and update test fixtures if needed"
    status: TODO
    depends_on: ["ST-001", "ST-002", "ST-005", "ST-006", "ST-007"]
    req_ids: []
    nfr_ids: ["NFR-TRACE-001"]
    acceptance_criteria:
      - "pack-check exits 0"
      - "If 'Six Flows' fixture fails, update structure.rs"
    scope_hints:
      code_roots: ["tools/demoswarm-pack-check/src/checks/"]
      test_roots: ["tools/demoswarm-pack-check/src/checks/"]
      doc_paths: []
      allow_new_files_under: []
    touches: ["tools/demoswarm-pack-check/src/checks/structure.rs"]
    tests: ["pack-check --no-color exits 0"]
    observability: []
    estimate: S
  - id: ST-010
    title: "Final verification - grep 'six flows' returns zero matches"
    status: TODO
    depends_on: ["ST-005", "ST-006", "ST-007", "ST-008", "ST-009"]
    req_ids: ["REQ-001"]
    nfr_ids: ["NFR-DOC-001"]
    acceptance_criteria:
      - "grep 'six flows' in public docs returns zero"
      - "pack-check passes"
    scope_hints:
      code_roots: []
      test_roots: []
      doc_paths: []
      allow_new_files_under: []
    touches: []
    tests: ["grep -r 'six flows' README.md DEMO_RUN.md docs/explanation/architecture.md returns zero"]
    observability: []
    estimate: S
```

## Subtasks

### ST-001: Update CLAUDE.md flow table to seven flows with variants

* **Objective**: Expand the CLAUDE.md flow table (currently 6 rows at L186-196) to list all 7 flows with their variant commands, aligning with the "7 flows" statement at L13.
* **Status**: TODO
* **Planned touchpoints**: CLAUDE.md (flow table section)
* **REQ/NFR linkage**: REQ-004, NFR-DOC-001
* **Acceptance criteria**:
  * CLAUDE.md flow table lists all seven flows (Signal, Plan, Build, Review, Gate, Deploy, Wisdom)
  * Flow table includes variant commands (flow-4-review, flow-5-gate, flow-6-deploy, flow-7-wisdom) with brief descriptions
  * Flow table numbering is consistent with "7 flows" statement in CLAUDE.md L13
  * Each variant has a brief description of its use case
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "seven flows" CLAUDE.md returns match
  * pack-check passes
* **Observability**: none (documentation-only)
* **Dependencies**: None (root subtask for Phase 1)
* **Risk / blast radius**: Medium - CLAUDE.md is authoritative; changes here propagate to downstream docs. Mitigated by per-phase commits.
* **Estimate**: S

---

### ST-002: Document flow overlap semantics and Flow 7 purpose in architecture.md

* **Objective**: Add documentation explaining the multi-path flow design (when to use each variant command) and document Flow 7 purpose as second-cycle wisdom extraction.
* **Status**: TODO
* **Planned touchpoints**: docs/explanation/architecture.md
* **REQ/NFR linkage**: REQ-002, REQ-003, NFR-DOC-001
* **Acceptance criteria**:
  * Explains relationship between /flow-4-gate and /flow-4-review (different entry points into review/gate cycle)
  * Explains relationship between /flow-5-gate and /flow-5-deploy (gate verdict re-entry vs deployment execution)
  * Explains relationship between /flow-6-deploy and /flow-6-wisdom (deploy-after-gate vs wisdom extraction paths)
  * Provides guidance on when users should choose each variant
  * Flow 7 purpose documented as second-cycle wisdom extraction for multi-iteration runs
  * Documentation explains how /flow-7-wisdom differs from /flow-6-wisdom
  * Section header updated from "six flows" to "seven flows"
  * Flow enumeration updated to include all seven flows
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "six flows" docs/explanation/architecture.md returns zero matches
  * grep "seven flows" docs/explanation/architecture.md returns match
* **Observability**: none
* **Dependencies**: ST-001 (authoritative source must be updated first)
* **Risk / blast radius**: Medium - Core conceptual documentation. Flow overlap semantics require clear explanation.
* **Estimate**: M

---

### ST-003: Update security posture and test count documentation

* **Objective**: Correct security claims (ReDoS immunity, path traversal limitation) with code evidence and update test count documentation to reference 102 passing unit tests.
* **Status**: TODO
* **Planned touchpoints**: docs/explanation/architecture.md
* **REQ/NFR linkage**: REQ-005, REQ-006, NFR-SEC-001
* **Acceptance criteria**:
  * Documentation states Rust regex crate is immune to ReDoS (finite automata implementation)
  * Documentation notes path traversal as known limitation in secrets scanner (pending threat assessment)
  * Security claims reference code evidence (secrets.rs, Rust regex crate)
  * Documentation does not claim ReDoS vulnerability in secrets scanner
  * Documentation references 102 unit tests passing as current test count
  * Documentation explains 277 filtered tests (integration tests requiring manual environment setup)
  * Test count claims include source artifact reference (e.g., "per test_output.log")
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "ReDoS" docs/explanation/architecture.md shows immunity claim (not vulnerability)
  * grep "102" docs/explanation/architecture.md returns match
* **Observability**: none
* **Dependencies**: ST-001 (authoritative source first)
* **Risk / blast radius**: Low - Security documentation only; no code changes. RSK-003 notes path traversal is low-risk in current context.
* **Estimate**: S

---

### ST-004: Clarify agent color coding purpose

* **Objective**: Document that agent color coding in frontmatter is advisory metadata for human/tooling consumption, not schema-enforced.
* **Status**: TODO
* **Planned touchpoints**: docs/explanation/architecture.md (or CLAUDE.md if more appropriate)
* **REQ/NFR linkage**: REQ-007
* **Acceptance criteria**:
  * Documentation acknowledges that agent frontmatter includes a `color:` field
  * Documentation clarifies color coding is advisory (human consumption), not schema-validated
  * If used for UI logic, documentation specifies the consumer of color metadata
  * Example agent frontmatter in documentation includes the `color:` field
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "color:" docs/explanation/architecture.md returns match
* **Observability**: none
* **Dependencies**: ST-001
* **Risk / blast radius**: Low - REQ-007 is low priority; advisory documentation only.
* **Estimate**: S

---

### ST-005: Update README.md flow count references

* **Objective**: Update README.md section header from "six flows" to "seven flows" and ensure body text is consistent.
* **Status**: TODO
* **Planned touchpoints**: README.md
* **REQ/NFR linkage**: REQ-001 AC-1, REQ-001 AC-5, NFR-DOC-001
* **Acceptance criteria**:
  * README.md section header updated from "six flows" to "seven flows" (L67)
  * README.md body text references "seven flows"
  * No remaining occurrences of "six flows" in README.md
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "six flows" README.md returns zero matches
  * grep "seven flows" README.md returns match
* **Observability**: none
* **Dependencies**: ST-001, ST-002 (authoritative sources must be updated first to derive from)
* **Risk / blast radius**: Low - Primary public documentation; highly visible but straightforward change.
* **Estimate**: S

---

### ST-006: Update DEMO_RUN.md flow count references

* **Objective**: Update DEMO_RUN.md to reference "seven flows" instead of "six flows."
* **Status**: TODO
* **Planned touchpoints**: DEMO_RUN.md
* **REQ/NFR linkage**: REQ-001 AC-2, NFR-DOC-001
* **Acceptance criteria**:
  * DEMO_RUN.md references "seven flows" instead of "six flows" (L14)
  * Flow enumeration includes Signal through Wisdom
  * No remaining occurrences of "six flows" in DEMO_RUN.md
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "six flows" DEMO_RUN.md returns zero matches
* **Observability**: none
* **Dependencies**: ST-001
* **Risk / blast radius**: Low - Stub documentation pointing to walkthrough.
* **Estimate**: S

---

### ST-007: Annotate CHANGELOG.md v1.0.0 entry

* **Objective**: Annotate the CHANGELOG.md v1.0.0 entry to clarify that 10 command files now implement 7 flows, while preserving historical accuracy that v1.0.0 shipped with 6 flow commands.
* **Status**: TODO
* **Planned touchpoints**: CHANGELOG.md
* **REQ/NFR linkage**: REQ-001 AC-4
* **Acceptance criteria**:
  * CHANGELOG.md v1.0.0 entry annotated to clarify 10 command files implementing 7 flows
  * Historical accuracy preserved (v1.0.0 did ship with 6 commands initially)
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * CHANGELOG.md contains annotation explaining evolution
* **Observability**: none
* **Dependencies**: ST-001
* **Risk / blast radius**: Low - Historical record; annotation rather than correction.
* **Estimate**: S

---

### ST-008: Update secondary documentation (glossary, CONTRIBUTING, work-without-github)

* **Objective**: Update secondary documentation files to reference "7 flows" for full consistency (Phase 3 per ADR).
* **Status**: TODO
* **Planned touchpoints**: docs/reference/glossary.md, CONTRIBUTING.md, docs/how-to/work-without-github.md
* **REQ/NFR linkage**: NFR-DOC-001
* **Acceptance criteria**:
  * docs/reference/glossary.md updated from "6 flows" to "7 flows" (L8)
  * CONTRIBUTING.md updated from "6 flows" to "7 flows" (L8)
  * docs/how-to/work-without-github.md updated from "6 flows" to "7 flows" (L15)
  * No remaining occurrences of "6 flows" in secondary documentation
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep "6 flows" docs/reference/glossary.md returns zero matches
  * grep "6 flows" CONTRIBUTING.md returns zero matches
  * grep "6 flows" docs/how-to/work-without-github.md returns zero matches
* **Observability**: none
* **Dependencies**: ST-005, ST-006 (primary docs first)
* **Risk / blast radius**: Low - Secondary documentation with lower traffic.
* **Estimate**: S

---

### ST-009: Verify pack-check passes and update test fixtures if needed

* **Objective**: Run pack-check after documentation updates and update structure.rs test fixtures only if pack-check fails on "Six Flows" string assertions.
* **Status**: TODO
* **Planned touchpoints**: tools/demoswarm-pack-check/src/checks/structure.rs (only if pack-check fails)
* **REQ/NFR linkage**: NFR-TRACE-001
* **Acceptance criteria**:
  * pack-check validation passes after all documentation updates (exit code 0)
  * If pack-check fails on "Six Flows" string in structure.rs, update test fixtures to "Seven Flows"
  * wisdom.rs checks continue to pass (Flow 7 validation)
* **Scope hints**:
  * Code roots: tools/demoswarm-pack-check/src/checks/
  * Test roots: tools/demoswarm-pack-check/src/checks/
  * Allow new files under: none
* **Tests**:
  * bash .claude/scripts/pack-check.sh --no-color exits 0
* **Observability**: none
* **Dependencies**: ST-001, ST-002, ST-005, ST-006, ST-007 (run after primary doc updates)
* **Risk / blast radius**: Low - Reactive only; RSK-001 mitigated by running pack-check first.
* **Estimate**: S

---

### ST-010: Final verification - grep "six flows" returns zero matches

* **Objective**: Verify that all public documentation is consistent and no "six flows" references remain.
* **Status**: TODO
* **Planned touchpoints**: none (verification only)
* **REQ/NFR linkage**: REQ-001 AC-5, NFR-DOC-001
* **Acceptance criteria**:
  * grep "six flows" across README.md, DEMO_RUN.md, docs/explanation/architecture.md returns zero matches
  * All public documentation agrees on flow count (seven flows)
  * pack-check passes
* **Scope hints**:
  * Code roots: none
  * Test roots: none
  * Allow new files under: none
* **Tests**:
  * grep -r "six flows" README.md DEMO_RUN.md docs/explanation/architecture.md returns zero matches
  * pack-check exits 0
* **Observability**: none
* **Dependencies**: ST-005, ST-006, ST-007, ST-008, ST-009
* **Risk / blast radius**: Low - Verification only.
* **Estimate**: S

---

## Dependency Graph

```
ST-001 (CLAUDE.md flow table)
   |
   +---> ST-002 (architecture.md flow overlap + Flow 7)
   |        |
   |        +---> ST-005 (README.md)
   |
   +---> ST-003 (security + test count)
   |
   +---> ST-004 (color coding)
   |
   +---> ST-006 (DEMO_RUN.md)
   |
   +---> ST-007 (CHANGELOG.md)

ST-005, ST-006 ---> ST-008 (secondary docs)

ST-001, ST-002, ST-005, ST-006, ST-007 ---> ST-009 (pack-check)

ST-005, ST-006, ST-007, ST-008, ST-009 ---> ST-010 (final verification)
```

## Parallelization Opportunities

Once ST-001 (CLAUDE.md) completes, the following can run in parallel:

| Parallel Group A | Parallel Group B | Parallel Group C |
|------------------|------------------|------------------|
| ST-002 (architecture.md) | ST-003 (security/tests) | ST-006 (DEMO_RUN.md) |
| | ST-004 (color coding) | ST-007 (CHANGELOG.md) |

After ST-002 completes:
- ST-005 (README.md) can proceed

After ST-005 and ST-006 complete:
- ST-008 (secondary docs) can proceed

After primary docs complete:
- ST-009 (pack-check verification) runs
- ST-010 (final verification) runs last

## Rollout Strategy

### Phase 0 (pre-merge) - Contracts + Tests + Observability Hooks

* **Contracts**: CLAUDE.md is the source of truth; flow table must list 7 flows with variants
* **Tests**: BDD scenarios in signal/features/*.feature validate documentation claims
* **Observability**: pack-check execution serves as observability (exit 0 = healthy)
* **Gate criteria**: pack-check passes; grep "six flows" returns zero in public docs

### Phase 1 (merge - authoritative sources)

* **What merges**: ST-001 (CLAUDE.md), ST-002 (architecture.md), ST-003 (security/tests), ST-004 (color coding)
* **What "green" means**: CLAUDE.md flow table shows 7 flows; architecture.md explains overlap semantics and Flow 7
* **Commit message**: "docs: update authoritative sources to seven-flow model (Phase 1)"

### Phase 2 (merge - primary public docs)

* **What merges**: ST-005 (README.md), ST-006 (DEMO_RUN.md), ST-007 (CHANGELOG.md)
* **What "green" means**: grep "six flows" returns zero in README.md, DEMO_RUN.md
* **Gate signal**: NFR-DOC-001 MET-1 passes for primary docs
* **Commit message**: "docs: update primary public docs to seven flows (Phase 2)"

### Phase 3 (merge - secondary docs)

* **What merges**: ST-008 (glossary.md, CONTRIBUTING.md, work-without-github.md)
* **What "green" means**: grep "6 flows" returns zero in all secondary docs
* **Gate signal**: NFR-DOC-001 fully satisfied
* **Commit message**: "docs: update secondary docs to seven flows (Phase 3)"

### Phase 4 (merge - pack tooling, if needed)

* **What merges**: ST-009 (structure.rs test fixtures, only if pack-check fails)
* **What "green" means**: pack-check exits 0
* **Gate signal**: NFR-TRACE-001 satisfied
* **Commit message**: "test: update pack-check fixtures for seven flows (Phase 4)"

## Rollback Plan

### Rollback Lever

Each phase produces a separate commit. Rollback is surgical:

1. **Phase 4 rollback**: `git revert <phase-4-commit>` - Restores test fixtures
2. **Phase 3 rollback**: `git revert <phase-3-commit>` - Restores secondary docs to "6 flows"
3. **Phase 2 rollback**: `git revert <phase-2-commit>` - Restores README.md, DEMO_RUN.md, CHANGELOG.md
4. **Phase 1 rollback**: `git revert <phase-1-commit>` - Restores CLAUDE.md and architecture.md

### Data/Schema Notes

* No database migrations
* No schema changes
* No irreversible data transformations
* All changes are additive documentation updates

### Monitoring for Rollback Decision

* **Trigger**: pack-check fails after merge
* **Signal**: CI reports non-zero exit from pack-check
* **Decision**: If pack-check failure is due to "Six Flows" assertion, proceed to Phase 4; if unexpected failure, investigate before rollback

## Assumptions

* **ASM-001**: CLAUDE.md is the authoritative source for flow architecture (L5 states "repo-level policy + shared contracts")
* **ASM-002**: Flow variants (flow-4-gate, flow-4-review, etc.) are intentional re-entry points, not duplicates
* **ASM-003**: 102 passing unit tests from test_output.log is the current authoritative count
* **ASM-004**: Path traversal in secrets.rs is a documentation/awareness issue, not an immediate exploitable vulnerability
* **ASM-005**: Agent color coding is advisory metadata, not schema-enforced
* **ASM-006**: Pack-check structure.rs test fixtures use "Six Flows" as string literals, not semantic assertions

## Open Questions

Reference: `.runs/local-alignment-audit-aba1c6/plan/open_questions.md`

* **OQ-PLAN-001**: Should the documentation update be structured as a single atomic PR or partitioned into logical commits per file/topic?
  * Suggested default: Single atomic PR with logical commits per phase (adopted in this plan)
  * Impact if different: If single commit preferred, simplifies review but loses granular revert capability

* **OQ-PLAN-002**: Should Flow 7 (/flow-7-wisdom) documentation explicitly reference the "second-cycle" or "iteration" use case?
  * Suggested default: Yes, explicitly describe as "second-cycle wisdom extraction for multi-iteration runs"
  * Impact if different: If generic, loses distinction from /flow-6-wisdom

* **OQ-PLAN-003**: Should the compliance partitioning schema be updated to include ST-007 for Flow 7?
  * Suggested default: Add ST-007 for completeness since Flow 7 is a distinct flow
  * Impact if different: If ST-006 covers both, no schema change but may confuse compliance tracing

---

## Pointers

* ADR: `.runs/local-alignment-audit-aba1c6/plan/adr.md`
* Impact Map: `.runs/local-alignment-audit-aba1c6/plan/impact_map.json`
* Requirements: `.runs/local-alignment-audit-aba1c6/signal/requirements.md`
* Open Questions: `.runs/local-alignment-audit-aba1c6/plan/open_questions.md`
* Design Options: `.runs/local-alignment-audit-aba1c6/plan/design_options.md`
* Early Risks: `.runs/local-alignment-audit-aba1c6/signal/early_risks.md`
* Scope Estimate: `.runs/local-alignment-audit-aba1c6/signal/scope_estimate.md`

## Inventory (machine countable)

- SUBTASK: ST-001
- SUBTASK: ST-002
- SUBTASK: ST-003
- SUBTASK: ST-004
- SUBTASK: ST-005
- SUBTASK: ST-006
- SUBTASK: ST-007
- SUBTASK: ST-008
- SUBTASK: ST-009
- SUBTASK: ST-010
- PHASE: Phase-1 (authoritative)
- PHASE: Phase-2 (primary)
- PHASE: Phase-3 (secondary)
- PHASE: Phase-4 (pack-tooling)
- REQ_COVERED: REQ-001
- REQ_COVERED: REQ-002
- REQ_COVERED: REQ-003
- REQ_COVERED: REQ-004
- REQ_COVERED: REQ-005
- REQ_COVERED: REQ-006
- REQ_COVERED: REQ-007
- NFR_COVERED: NFR-DOC-001
- NFR_COVERED: NFR-SEC-001
- NFR_COVERED: NFR-TRACE-001
