# Work Plan for compliance-drift-proofing

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []
missing_required: []

## Scope Snapshot
- **ADR decision**: Inline Extension of Existing Modules (OPT-001) - add checks 50-51 to drift.rs, constants to contracts.rs, verify check 49 and --strict_warnings behavior
- **Primary impacts**:
  - HIGH: contracts.rs (new constants for skill CLI subcommands and OpenQ flow codes)
  - HIGH: drift.rs (add checks 50-51 for flow boundary and OpenQ prefix validation)
  - HIGH: cli.rs (verify --strict_warnings flag behavior)
  - MEDIUM: test fixtures directory (new fixtures for Build-to-Gate handshake)
  - MEDIUM: stable-markers.md and contracts.md (normalize PLN/BLD prefixes)
- **Key constraints**:
  - Canonical status and action enums are frozen
  - Three-tier ownership model is authoritative (flow commands -> agent docs -> skill docs)
  - New checks must not break existing valid artifacts without migration path (NFR-COMP-001)
  - CI runtime must complete under 30 seconds (NFR-PERF-001)
  - pack-check (Rust) is the preferred venue for structural validation
- **Verification posture**:
  - Check 50 passes on all existing flow commands (REQ-001 AC-3)
  - Check 49 passes on all agents with Skills sections (REQ-002 AC-4)
  - Check 51 passes on existing open_questions.md files or exceptions documented (REQ-003 AC-4)
  - --strict_warnings elevates warnings to errors with non-zero exit code (REQ-005)
  - Validation baseline established before new rules introduced (REQ-006)

## Subtask Index (parseable)

See `.runs/compliance-drift-proofing/plan/subtasks.yaml` for machine-canonical YAML.

## Subtasks

### ST-001: Add skill CLI subcommands and OpenQ flow codes constants to contracts.rs

* **Objective**: Add the foundational constants that checks 50 and 51 will reference
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/src/contracts.rs`
* **REQ/NFR linkage**: REQ-001, REQ-003, NFR-MAINT-001
* **Acceptance criteria**:
  * contracts.rs contains SKILL_CLI_SUBCOMMANDS constant with all demoswarm CLI subcommands (count, ms, yaml, index, receipt, receipts, openapi, line, inv, time, openq, secrets)
  * contracts.rs contains OPENQ_FLOW_CODES constant with SIG, PLN, BLD, GAT, DEP, WIS
  * Constants are referenced from Contracts struct and exposed to checks
  * cargo check passes with no errors
* **Scope hints**:
  * Code roots: `tools/demoswarm-pack-check/src/`
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: (none)
* **Tests**:
  * cargo test contracts
* **Observability**: (none)
* **Dependencies**: None
* **Risk / blast radius**: Low - additive changes to existing constants file; no behavior changes
* **Estimate**: S

---

### ST-002: Add check 50 for flow boundary enforcement (demoswarm.sh in flow commands)

* **Objective**: Enforce that flow commands do not contain skill-layer CLI syntax
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/src/checks/drift.rs`
  - `tools/demoswarm-pack-check/src/checks/mod.rs`
* **REQ/NFR linkage**: REQ-001, NFR-PERF-001, NFR-REL-001, NFR-OPS-001
* **Acceptance criteria**:
  * Check 50 scans all .claude/commands/flow-*.md files
  * Check 50 reports warning when flow command contains "demoswarm.sh"
  * Check 50 reports warning when flow command contains skill CLI subcommands
  * Existing flow command files pass validation without warnings
  * Diagnostic output includes rule ID, file path, and line number
* **Scope hints**:
  * Code roots: `tools/demoswarm-pack-check/src/checks/`
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: (none)
* **Tests**:
  * @check-50, cargo test check_flow_boundary
* **Observability**: (none)
* **Dependencies**: ST-001 (needs constants)
* **Risk / blast radius**: Medium - new check could produce false positives if pattern too broad; mitigated by baseline verification (ST-011) and warning-first mode
* **Estimate**: M

---

### ST-003: Verify check 49 adequacy for REQ-002 (Skills section enforcement)

* **Objective**: Confirm existing check 49 meets all REQ-002 acceptance criteria
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/src/checks/drift.rs` (read/analyze)
* **REQ/NFR linkage**: REQ-002
* **Acceptance criteria**:
  * Check 49 correctly identifies agents using demoswarm.sh without ## Skills section
  * Check 49 diagnostic output includes specific agent file paths
  * Agents using demoswarm.sh indirectly are not flagged (per REQ-002 AC-3)
  * All agents currently having Skills sections pass validation
  * Gap analysis documented: list any agents needing Skills sections added
* **Scope hints**:
  * Code roots: `tools/demoswarm-pack-check/src/checks/`
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: (none)
* **Tests**:
  * @check-49, cargo test check_skills_section
* **Observability**: (none)
* **Dependencies**: None
* **Risk / blast radius**: Low - verification only; existing check 49 already implemented
* **Estimate**: S

---

### ST-004: Remediate agents missing Skills sections (if any)

* **Objective**: Add ## Skills sections to any agents using demoswarm.sh that lack them
* **Status**: TODO
* **Planned touchpoints**:
  - `.claude/agents/*.md` (agents identified in ST-003)
* **REQ/NFR linkage**: REQ-002, REQ-006
* **Acceptance criteria**:
  * All agents using demoswarm.sh have ## Skills section
  * Skills sections list the specific skills used (runs-derive, runs-index, etc.)
  * pack-check reports no warnings for check 49 after remediation
* **Scope hints**:
  * Code roots: `.claude/agents/`
  * Test roots: (none)
  * Allow new files under: (none)
* **Tests**:
  * pack-check --no-color | grep 'check 49'
* **Observability**: (none)
* **Dependencies**: ST-003 (gap analysis identifies agents)
* **Risk / blast radius**: Low - adding documentation sections; no behavior changes
* **Estimate**: S

---

### ST-005: Add check 51 for OpenQ prefix pattern validation

* **Objective**: Validate QID patterns in open_questions.md files
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/src/checks/drift.rs`
  - `tools/demoswarm-pack-check/src/checks/mod.rs`
  - `tools/demoswarm-pack-check/src/contracts.rs`
* **REQ/NFR linkage**: REQ-003, NFR-PERF-001, NFR-REL-001, NFR-OPS-001
* **Acceptance criteria**:
  * Check 51 scans .runs/**/open_questions.md files for QID patterns
  * Check 51 validates QID format: OQ-<FLOW>-<NNN> where FLOW is SIG/PLN/BLD/GAT/DEP/WIS
  * Check 51 reports warning for non-canonical flow codes (PLAN instead of PLN)
  * Check 51 validates three-digit zero-padded numeric suffix
  * Diagnostic output includes rule ID, file path, and line number
* **Scope hints**:
  * Code roots: `tools/demoswarm-pack-check/src/checks/`
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: (none)
* **Tests**:
  * @check-51, cargo test check_openq_prefix
* **Observability**: (none)
* **Dependencies**: ST-001 (needs OPENQ_FLOW_CODES constant)
* **Risk / blast radius**: Medium - existing QIDs using PLN/BLD are canonical; check must not flag those. RSK-002 mitigated by using canonical PLN/BLD from openq-tools.
* **Estimate**: M

---

### ST-006: Resolve PLN vs PLAN prefix in documentation (normalize to PLN/BLD)

* **Objective**: Update documentation to use canonical PLN/BLD abbreviations
* **Status**: TODO
* **Planned touchpoints**:
  - `docs/reference/stable-markers.md`
  - `docs/reference/contracts.md`
* **REQ/NFR linkage**: REQ-003, NFR-MAINT-001
* **Acceptance criteria**:
  * stable-markers.md uses PLN/BLD (not PLAN/BUILD)
  * contracts.md uses PLN/BLD (not PLAN/BUILD)
  * openq-tools/SKILL.md remains canonical (PLN/BLD already correct)
  * Documentation is internally consistent
* **Scope hints**:
  * Code roots: (none)
  * Doc paths: `docs/reference/stable-markers.md`, `docs/reference/contracts.md`
  * Allow new files under: (none)
* **Tests**:
  * grep -E 'PLAN|BUILD' docs/reference/stable-markers.md docs/reference/contracts.md
* **Observability**: (none)
* **Dependencies**: None (can proceed in parallel)
* **Risk / blast radius**: Low - documentation changes only; resolves OQ-SIG-002 and OQ-PLN-004
* **Estimate**: S

---

### ST-007: Verify --strict_warnings flag behavior matches REQ-005

* **Objective**: Confirm existing --strict_warnings flag meets REQ-005 requirements
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/src/cli.rs`
  - `tools/demoswarm-pack-check/src/main.rs`
  - `tools/demoswarm-pack-check/src/reporter.rs`
* **REQ/NFR linkage**: REQ-005, NFR-COMP-001
* **Acceptance criteria**:
  * --strict_warnings elevates warnings to errors for all new compliance rules
  * Without --strict_warnings, validation completes with exit code 0 when only warnings present
  * With --strict_warnings, validation exits non-zero when any warning present
  * Warning output includes rule identifier and file location
  * Existing pack-check exit codes and output format preserved
* **Scope hints**:
  * Code roots: `tools/demoswarm-pack-check/src/`
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: (none)
* **Tests**:
  * cargo test strict_warnings, @exit-codes
* **Observability**: (none)
* **Dependencies**: ST-002, ST-005 (need new checks to test warning behavior)
* **Risk / blast radius**: Low - verification primarily; any CLI changes are small (exit code handling). RSK-004 mitigated by warning-first design.
* **Estimate**: S

---

### ST-008: Create Build-to-Gate test fixtures directory and valid receipt fixture

* **Objective**: Create test infrastructure for Build receipt validation
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/tests/fixtures/build_receipt_valid.json`
* **REQ/NFR linkage**: REQ-004, NFR-SEC-001
* **Acceptance criteria**:
  * tools/demoswarm-pack-check/tests/fixtures/ directory exists
  * build_receipt_valid.json contains all required fields per receipt schema
  * Fixture uses obviously synthetic values (no real secrets or credentials)
  * Valid fixture passes receipt-checker validation logic
* **Scope hints**:
  * Code roots: (none)
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: `tools/demoswarm-pack-check/tests/fixtures/`
* **Tests**:
  * cargo test fixtures
* **Observability**: (none)
* **Dependencies**: None (can start early)
* **Risk / blast radius**: Low - new test files; no production code changes. RSK-006 mitigated by using obviously synthetic values.
* **Estimate**: S

---

### ST-009: Create invalid Build receipt fixture for failure path testing

* **Objective**: Create fixture that exercises validation failure paths
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/tests/fixtures/build_receipt_invalid.json`
* **REQ/NFR linkage**: REQ-004, NFR-SEC-001
* **Acceptance criteria**:
  * build_receipt_invalid.json has missing required field or invalid value
  * Invalid fixture fails receipt-checker validation logic
  * Fixture uses obviously synthetic values
* **Scope hints**:
  * Code roots: (none)
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: `tools/demoswarm-pack-check/tests/fixtures/`
* **Tests**:
  * cargo test fixtures_invalid
* **Observability**: (none)
* **Dependencies**: ST-008 (fixture directory exists)
* **Risk / blast radius**: Low - test fixture only
* **Estimate**: S

---

### ST-010: Add Rust test case exercising Build receipt validation

* **Objective**: Create test that validates Build-to-Gate handshake contract
* **Status**: TODO
* **Planned touchpoints**:
  - `tools/demoswarm-pack-check/tests/receipt_validation_test.rs`
  - `tools/demoswarm-pack-check/src/lib.rs`
* **REQ/NFR linkage**: REQ-004, NFR-REL-001
* **Acceptance criteria**:
  * Test case loads valid fixture and verifies it passes validation
  * Test case loads invalid fixture and verifies it fails validation
  * Test documents handshake contract being validated (required fields)
  * Tests are deterministic (same result on repeated runs)
* **Scope hints**:
  * Code roots: `tools/demoswarm-pack-check/src/`
  * Test roots: `tools/demoswarm-pack-check/tests/`
  * Allow new files under: `tools/demoswarm-pack-check/tests/`
* **Tests**:
  * cargo test receipt_validation
* **Observability**: (none)
* **Dependencies**: ST-008, ST-009 (fixtures exist)
* **Risk / blast radius**: Low - new test code; validates existing contract
* **Estimate**: M

---

### ST-011: Establish validation baseline (REQ-006)

* **Objective**: Run pack-check and document baseline before new rules go live
* **Status**: TODO
* **Planned touchpoints**:
  - `.runs/compliance-drift-proofing/build/validation_baseline.md`
* **REQ/NFR linkage**: REQ-006
* **Acceptance criteria**:
  * pack-check runs on current pack state before new rules introduced
  * Baseline output captured and documented
  * No false positives on existing valid artifacts
  * Any divergent patterns enumerated as known exceptions or remediated
* **Scope hints**:
  * Code roots: (none)
  * Test roots: (none)
  * Allow new files under: `.runs/compliance-drift-proofing/build/`
* **Tests**:
  * bash .claude/scripts/pack-check.sh --no-color
* **Observability**: (none)
* **Dependencies**: ST-002, ST-003, ST-005 (new checks implemented)
* **Risk / blast radius**: Low - documentation/verification only
* **Estimate**: S

---

### ST-012: Update pack-check.md documentation with new validation rules

* **Objective**: Document new checks for discoverability and remediation guidance
* **Status**: TODO
* **Planned touchpoints**:
  - `docs/reference/pack-check.md`
* **REQ/NFR linkage**: NFR-OPS-001
* **Acceptance criteria**:
  * pack-check.md documents check 50 (flow boundary enforcement)
  * pack-check.md documents check 51 (OpenQ prefix validation)
  * pack-check.md documents --strict_warnings flag behavior
  * Suggested remediation for each rule is documented
* **Scope hints**:
  * Code roots: (none)
  * Doc paths: `docs/reference/pack-check.md`
  * Allow new files under: (none)
* **Tests**: (none)
* **Observability**: (none)
* **Dependencies**: ST-002, ST-005, ST-007 (checks and flag behavior finalized)
* **Risk / blast radius**: Low - documentation only
* **Estimate**: S

---

## Dependency Graph

```
ST-001 (contracts.rs constants)
   |
   +---> ST-002 (check 50: flow boundary) ---+
   |                                          |
   +---> ST-005 (check 51: OpenQ prefix) -----+---> ST-007 (--strict_warnings) ---> ST-012 (docs)
                                              |
                                              +---> ST-011 (baseline)

ST-003 (verify check 49) ---> ST-004 (remediate agents)

ST-006 (PLN/BLD docs) [parallel]

ST-008 (valid fixture) ---> ST-009 (invalid fixture) ---> ST-010 (test case)
```

## Parallelization Opportunities

The following subtasks can run concurrently once their prerequisites are met:

**Wave 1 (no dependencies):**
- ST-001 (contracts.rs constants)
- ST-003 (verify check 49)
- ST-006 (PLN/BLD documentation)
- ST-008 (valid receipt fixture)

**Wave 2 (after Wave 1):**
- ST-002 (check 50) after ST-001
- ST-005 (check 51) after ST-001
- ST-004 (remediate agents) after ST-003
- ST-009 (invalid fixture) after ST-008

**Wave 3 (after Wave 2):**
- ST-007 (--strict_warnings verification) after ST-002 and ST-005
- ST-010 (receipt test case) after ST-008 and ST-009
- ST-011 (baseline) after ST-002, ST-003, ST-005

**Wave 4 (finalization):**
- ST-012 (documentation) after ST-002, ST-005, ST-007

## Rollout Strategy

* **Phase 0 (pre-merge)**: All checks implemented as warnings (default mode). Test fixtures and validation baseline established. Pack-check runs green on existing artifacts.

* **Phase 1 (merge)**: PR merged when:
  - All 12 subtasks complete
  - `cargo test` passes
  - `pack-check --no-color` reports no errors (warnings allowed)
  - CI runtime under 30 seconds (NFR-PERF-001)

* **Phase 2 (limited exposure)**: Teams run pack-check with new rules. Warning output identifies remediation needs without blocking. Monitor for false positives via GitHub issues.

* **Phase 3 (enforcement)**: After burn-in period (1-2 weeks), enable `--strict_warnings` in CI to enforce compliance. Document in release notes.

**Observability signals for phase gates:**
- Check pass/fail counts in pack-check JSON output
- CI runtime (target: <30s total, <5s incremental per NFR-PERF-001)
- GitHub issues reporting false positives

## Rollback Plan

* **Rollback lever**: Remove checks 50 and 51 from drift.rs checks() vector. Constants in contracts.rs can remain (harmless).

* **Data/schema notes**: No data migrations. Test fixtures can be left in place. Documentation changes (ST-006, ST-012) can be reverted independently.

* **Reversibility**: All changes are additive. No existing behavior is modified. Rollback is a simple revert of the drift.rs changes.

* **What we monitor to decide rollback**:
  - False positive rate (>5% of pack files flagged incorrectly)
  - CI runtime exceeding 30s
  - GitHub issues indicating blocking problems

* **Irreversible steps**: None. All changes are additive and can be reverted without data loss.

## Assumptions

* **ASM-001**: Check 49 (check_skills_section_required) is already implemented in drift.rs and adequately addresses REQ-002 for agents with literal "demoswarm.sh" references.
  - Impact if wrong: Would need to enhance check 49 logic (minor effort)

* **ASM-002**: PLN/BLD abbreviations (per openq-tools/SKILL.md) are canonical over PLAN/BUILD (per stable-markers.md line 60).
  - Impact if wrong: Would need to update openq-tools Rust code and existing QIDs

* **ASM-003**: The existing --strict_warnings flag provides the warning-to-error elevation mechanism needed for REQ-005.
  - Impact if wrong: Minor CLI changes to exit code handling

* **ASM-004**: Two new checks (50, 51) will not exceed 30-second CI budget.
  - Impact if wrong: Would need regex optimization (pattern matching is inherently fast)

* **ASM-005**: The 4 agents using demoswarm.sh without Skills sections (if any) are gaps to fix, not intentional exceptions.
  - Impact if wrong: Would need exemption list mechanism

## Open Questions

Reference: `.runs/compliance-drift-proofing/plan/open_questions.md`

Questions that materially affect sequencing/rollout:

* **OQ-PLN-004** (PLN vs PLAN): Resolved by assumption - PLN/BLD is canonical per openq-tools. ST-006 updates docs to match.

* **OQ-PLN-009** (Missing Skills sections): Addressed by ST-003 gap analysis and ST-004 remediation.

* **OQ-PLN-001** (Module location for new checks): Resolved by ADR - checks go in drift.rs per existing pattern (checks 38-49).

* **OQ-PLN-002** (--strict exit codes): Default assumed - exit 0 success, exit 1 errors/warnings+strict. Verified in ST-007.

---

## Inventory (machine countable)

- SUBTASK_COUNT: 12
- TOTAL_ESTIMATE: 5S + 3M = approximately 8 story points
- HIGH_RISK_SUBTASKS: 0
- MEDIUM_RISK_SUBTASKS: 2 (ST-002, ST-005)
- LOW_RISK_SUBTASKS: 10
- PARALLEL_WAVES: 4
- DEPENDENCIES_TOTAL: 14

## Machine Summary Block

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - OQ-PLN-009: 4 agents missing Skills sections not yet enumerated (deferred to ST-003/ST-004)
  - RSK-001: Prior #49 bounce - mitigated by warning-first mode and narrower scope

subtask_count: 12
estimates:
  S: 9
  M: 3
  L: 0
  XL: 0
```
