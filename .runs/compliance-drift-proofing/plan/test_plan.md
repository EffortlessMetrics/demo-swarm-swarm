# Test Plan

## Machine Summary
status: VERIFIED
missing_required: []
blockers: []
concerns:
  - OQ-SIG-002 (PLN vs PLAN prefix) should be resolved before implementing REQ-003 validation regex
  - 4 agents potentially missing Skills sections not enumerated; REQ-006 baseline may surface them as expected warnings
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

counts:
  scenarios_total: 40
  requirements_total: 12
  requirements_with_scenarios: 6

severity_summary:
  critical: 0
  major: 0
  minor: 2

## Scope

This test plan covers:
- Unit tests for new pack-check validation checks (50, 51)
- Integration tests with fixture files for all new validation rules
- CLI behavior tests (--strict flag, exit codes)
- Regression tests ensuring existing checks continue to function
- Build-to-Gate handshake validation test fixtures

This test plan does NOT cover:
- Signal-to-Plan or Plan-to-Build handshake validation (deferred per OQ-SIG-003)
- Semantic/behavioral compliance validation (non-goal per ADR)
- Dynamic skill discovery (hardcoded list per OQ-SIG-006 default)
- check-doc-drift.sh testing (separate tool per OQ-SIG-007 default)

## Coverage Thresholds

Stable markers (required for coverage-enforcer to parse mechanically):
- COVERAGE_LINE_REQUIRED: 80
- COVERAGE_BRANCH_REQUIRED: 70
- COVERAGE_CRITICAL_PATH: tools/demoswarm-pack-check/src/checks/drift.rs, tools/demoswarm-pack-check/src/cli.rs

Additional notes:
- measurement_notes: Use Rust's built-in coverage tooling (cargo-llvm-cov or similar); parse summary from test-runner output
- Critical path modules: drift.rs (contains new checks 50, 51) and cli.rs (--strict flag behavior)
- P0 coverage target: 90% for checks 50, 51 in drift.rs

## Mutation Testing
- mutation_required: false
- mutation_threshold: null
- mutation_scope:
  - tools/demoswarm-pack-check/src/checks/drift.rs
- mutation_tool_hint: cargo-mutants
- rationale: This change is validation tooling, not security/payment/auth code. Mutation testing is recommended but not required. If available, target check 50 and 51 implementations to verify boundary conditions are properly tested.

## Scenario Inventory (Feature Files)

### flow_boundary_enforcement.feature (6 scenarios)

| # | Scenario | Tags | REQ |
|---|----------|------|-----|
| 1 | Flow command file without skill references passes validation | @REQ-001, @smoke | REQ-001 |
| 2 | Flow command file containing demoswarm.sh produces warning | @REQ-001, @error | REQ-001 |
| 3 | Flow command file containing skill CLI subcommand produces warning | @REQ-001, @error | REQ-001 |
| 4 | Flow command with skill name in prose context is not flagged | @REQ-001, @edge | REQ-001 |
| 5 | Flow command file with demoswarm.sh and --strict flag fails | @REQ-001, @error | REQ-001 |
| 6 | Validation scans all flow command files matching pattern | @REQ-001 | REQ-001 |

### skills_section_enforcement.feature (6 scenarios)

| # | Scenario | Tags | REQ |
|---|----------|------|-----|
| 1 | Agent with demoswarm.sh and Skills section passes validation | @REQ-002, @smoke | REQ-002 |
| 2 | Agent with demoswarm.sh but no Skills section produces warning | @REQ-002, @error | REQ-002 |
| 3 | Agent without demoswarm.sh is not required to have Skills section | @REQ-002 | REQ-002 |
| 4 | Agent invoking skill via Skill tool only is not flagged | @REQ-002, @edge | REQ-002 |
| 5 | Multiple agents missing Skills sections are all identified | @REQ-002, @error | REQ-002 |
| 6 | Missing Skills section with --strict flag fails validation | @REQ-002, @error | REQ-002 |

### openq_prefix_validation.feature (8 scenarios)

| # | Scenario | Tags | REQ |
|---|----------|------|-----|
| 1 | Valid QID with canonical flow code passes validation | @REQ-003, @smoke | REQ-003 |
| 2 | Valid QIDs for each canonical flow code (Scenario Outline, 6 examples) | @REQ-003 | REQ-003 |
| 3 | Non-canonical flow code produces warning | @REQ-003, @error | REQ-003 |
| 4 | Non-canonical flow codes produce warnings (Scenario Outline, 5 examples) | @REQ-003, @error | REQ-003 |
| 5 | QID with non-zero-padded numeric suffix produces warning | @REQ-003, @error | REQ-003 |
| 6 | QID with four-digit numeric suffix produces warning | @REQ-003, @error | REQ-003 |
| 7 | Multiple QIDs in same file are all validated | @REQ-003, @edge | REQ-003 |
| 8 | Invalid QID with --strict flag fails validation | @REQ-003, @error | REQ-003 |

### build_gate_handshake.feature (5 scenarios)

| # | Scenario | Tags | REQ |
|---|----------|------|-----|
| 1 | Valid build_receipt.json fixture passes receipt validation | @REQ-004, @smoke | REQ-004 |
| 2 | Invalid build_receipt.json fixture fails receipt validation | @REQ-004, @error | REQ-004 |
| 3 | Test suite includes Build receipt validation test case | @REQ-004 | REQ-004 |
| 4 | Invalid fixture with missing required field fails validation | @REQ-004, @edge | REQ-004 |
| 5 | Handshake contract documentation exists | @REQ-004 | REQ-004 |

### warning_first_mode.feature (8 scenarios)

| # | Scenario | Tags | REQ |
|---|----------|------|-----|
| 1 | Validation completes successfully when only warnings are present | @REQ-005, @smoke | REQ-005 |
| 2 | Validation fails with --strict when warnings are present | @REQ-005, @error | REQ-005 |
| 3 | pack-check --strict flag elevates all new rule warnings | @REQ-005 | REQ-005 |
| 4 | Warning output includes rule identifier | @REQ-005 | REQ-005 |
| 5 | Warning output includes file location | @REQ-005 | REQ-005 |
| 6 | Mixed old and new rule violations with --strict | @REQ-005, @edge | REQ-005 |
| 7 | Clean pack passes validation without --strict flag | @REQ-005 | REQ-005 |
| 8 | Clean pack passes validation with --strict flag | @REQ-005 | REQ-005 |

### no_false_positives.feature (7 scenarios)

| # | Scenario | Tags | REQ |
|---|----------|------|-----|
| 1 | Existing flow command files pass REQ-001 validation | @REQ-006, @smoke | REQ-006 |
| 2 | Existing agent files pass REQ-002 validation | @REQ-006, @smoke | REQ-006 |
| 3 | Existing open_questions.md files pass REQ-003 validation | @REQ-006, @smoke | REQ-006 |
| 4 | Validation baseline is established before introducing rules | @REQ-006 | REQ-006 |
| 5 | Known exception agents are documented not flagged as errors | @REQ-006, @edge | REQ-006 |
| 6 | New rule introduction does not change existing rule behavior | @REQ-006 | REQ-006 |
| 7 | Edge case prose that resembles violations is not flagged | @REQ-006, @edge | REQ-006 |

## Scenario to Test Type Matrix

| REQ | Feature File | Scenario | Priority | Unit | Integration | Contract | E2E | Fuzz | Perf/Obs | Notes |
|-----|--------------|----------|----------|------|-------------|----------|-----|------|----------|-------|
| REQ-001 | flow_boundary_enforcement.feature | Flow command without skill references passes | P1 | x | x | | | | | Happy path; check 50 logic |
| REQ-001 | flow_boundary_enforcement.feature | demoswarm.sh produces warning | P1 | x | x | | | | | Core violation detection |
| REQ-001 | flow_boundary_enforcement.feature | skill CLI subcommand produces warning | P1 | x | x | | | | | Skill subcommand regex |
| REQ-001 | flow_boundary_enforcement.feature | prose context not flagged | P1 | x | | | | x | | False positive prevention; fuzz prose variants |
| REQ-001 | flow_boundary_enforcement.feature | --strict fails | P1 | | x | | | | | CLI integration |
| REQ-001 | flow_boundary_enforcement.feature | Scans all flow-*.md | P2 | | x | | | | | Glob pattern coverage |
| REQ-002 | skills_section_enforcement.feature | demoswarm.sh + Skills passes | P1 | x | x | | | | | Happy path; check 49 logic |
| REQ-002 | skills_section_enforcement.feature | demoswarm.sh no Skills warns | P1 | x | x | | | | | Core violation detection |
| REQ-002 | skills_section_enforcement.feature | No demoswarm.sh no Skills OK | P1 | x | | | | | | Negative case |
| REQ-002 | skills_section_enforcement.feature | Skill tool only not flagged | P1 | x | | | | | | Edge case; literal matching |
| REQ-002 | skills_section_enforcement.feature | Multiple agents identified | P2 | | x | | | | | Multi-file detection |
| REQ-002 | skills_section_enforcement.feature | --strict fails | P1 | | x | | | | | CLI integration |
| REQ-003 | openq_prefix_validation.feature | Valid QID passes | P1 | x | x | | | | | Happy path; check 51 regex |
| REQ-003 | openq_prefix_validation.feature | All canonical codes (Outline) | P1 | x | | | | | | Exhaust valid codes |
| REQ-003 | openq_prefix_validation.feature | Non-canonical warns | P1 | x | x | | | | | PLAN vs PLN detection |
| REQ-003 | openq_prefix_validation.feature | Non-canonical codes (Outline) | P1 | x | | | | | | Exhaust invalid codes |
| REQ-003 | openq_prefix_validation.feature | Non-padded suffix warns | P1 | x | | | | x | | Boundary; fuzz padding |
| REQ-003 | openq_prefix_validation.feature | Four-digit suffix warns | P1 | x | | | | x | | Boundary; fuzz length |
| REQ-003 | openq_prefix_validation.feature | Multiple QIDs validated | P2 | | x | | | | | Multi-match in file |
| REQ-003 | openq_prefix_validation.feature | --strict fails | P1 | | x | | | | | CLI integration |
| REQ-004 | build_gate_handshake.feature | Valid receipt passes | P0 | x | x | x | | | | Core handshake contract |
| REQ-004 | build_gate_handshake.feature | Invalid receipt fails | P0 | x | x | x | | | | Contract violation |
| REQ-004 | build_gate_handshake.feature | Test case exists | P1 | | | | | | | Meta-test; verify fixture usage |
| REQ-004 | build_gate_handshake.feature | Missing field fails | P0 | x | | x | | | | Schema enforcement |
| REQ-004 | build_gate_handshake.feature | Documentation exists | P2 | | | | | | | Non-code artifact |
| REQ-005 | warning_first_mode.feature | Warnings exit 0 | P0 | | x | | | | | Exit code contract |
| REQ-005 | warning_first_mode.feature | --strict exit non-zero | P0 | | x | | | | | Exit code contract |
| REQ-005 | warning_first_mode.feature | --strict elevates all | P1 | | x | | | | | Behavior uniformity |
| REQ-005 | warning_first_mode.feature | Output has rule ID | P1 | | x | | | | x | Diagnostics; log check |
| REQ-005 | warning_first_mode.feature | Output has file path | P1 | | x | | | | x | Diagnostics; log check |
| REQ-005 | warning_first_mode.feature | Mixed violations --strict | P1 | | x | | | | | Old + new rules together |
| REQ-005 | warning_first_mode.feature | Clean no --strict | P1 | | x | | | | | Regression baseline |
| REQ-005 | warning_first_mode.feature | Clean with --strict | P1 | | x | | | | | Regression baseline |
| REQ-006 | no_false_positives.feature | Existing flow commands pass | P0 | | x | | | | | Regression gate |
| REQ-006 | no_false_positives.feature | Existing agents pass | P0 | | x | | | | | Regression gate |
| REQ-006 | no_false_positives.feature | Existing open_questions pass | P0 | | x | | | | | Regression gate |
| REQ-006 | no_false_positives.feature | Baseline established | P1 | | x | | | | | Pre-rule capture |
| REQ-006 | no_false_positives.feature | Known exceptions warned | P2 | | x | | | | | Document vs error |
| REQ-006 | no_false_positives.feature | Existing rules unchanged | P1 | | x | | | | | Regression gate |
| REQ-006 | no_false_positives.feature | Prose not flagged | P1 | x | | | | x | | False positive prevention |

## Requirement Coverage Summary

| Requirement | Scenarios | Priority | Required Test Types | Notes |
|-------------|-----------|----------|---------------------|-------|
| REQ-001 | 6 | P1 | Unit, Integration, Fuzz | Check 50; flow boundary enforcement |
| REQ-002 | 6 | P1 | Unit, Integration | Check 49 (existing); Skills section enforcement |
| REQ-003 | 8 | P1 | Unit, Integration, Fuzz | Check 51; OpenQ prefix validation |
| REQ-004 | 5 | P0 | Unit, Integration, Contract | Build-to-Gate handshake fixtures |
| REQ-005 | 8 | P0 | Integration, Observability | CLI --strict flag and exit codes |
| REQ-006 | 7 | P0 | Integration | Regression gate; false positive prevention |
| NFR-PERF-001 | 0 (non-behavioral) | P2 | Performance | CI timing assertions |
| NFR-REL-001 | 0 (non-behavioral) | P1 | Integration | Determinism; byte-identical output |
| NFR-OPS-001 | 0 (non-behavioral) | P1 | Observability | Diagnostic format; --help docs |
| NFR-COMP-001 | 0 (non-behavioral) | P0 | Integration | Backward compat; exit code preservation |
| NFR-SEC-001 | 0 (non-behavioral) | P0 | Code Review | No secrets in output |
| NFR-MAINT-001 | 0 (non-behavioral) | P2 | Code Review | Constants in contracts.rs |

## Test Fixtures Needed

All fixtures should be located at: `tools/demoswarm-pack-check/tests/fixtures/`

### Build Receipt Fixtures (REQ-004)

| Fixture | Purpose | Contents |
|---------|---------|----------|
| `build_receipt_valid.json` | Valid Build receipt passes validation | Complete receipt with run_id, flow, status: VERIFIED, counts, quality_gates, timestamp |
| `build_receipt_invalid_status.json` | Invalid status value fails validation | Receipt with status: "INVALID_STATUS" |
| `build_receipt_missing_run_id.json` | Missing required field fails | Receipt without run_id field |
| `build_receipt_empty_counts.json` | Minimal valid receipt | Receipt with empty counts object (valid structure) |
| `README.md` | Handshake contract documentation | Required Fields, Valid Status Values, Cross-Flow Expectations sections |

### Flow Command Fixtures (REQ-001)

| Fixture | Purpose | Contents |
|---------|---------|----------|
| `flow_command_clean.md` | No violations | Standard flow command prose only |
| `flow_command_demoswarm.md` | Contains demoswarm.sh | Flow command with embedded demoswarm.sh call |
| `flow_command_skill_subcommand.md` | Contains skill CLI subcommand | Flow command with "count", "ms get", etc. |
| `flow_command_prose_count.md` | Prose mentions "count" naturally | For false positive testing |

### Agent Fixtures (REQ-002)

| Fixture | Purpose | Contents |
|---------|---------|----------|
| `agent_with_skills.md` | Has demoswarm.sh and ## Skills | Compliant agent |
| `agent_without_skills.md` | Has demoswarm.sh, no ## Skills | Violation case |
| `agent_no_demoswarm.md` | No demoswarm.sh | Not required to have Skills |
| `agent_skill_tool_only.md` | Uses Skill() tool, no demoswarm.sh literal | Edge case |

### OpenQ Fixtures (REQ-003)

| Fixture | Purpose | Contents |
|---------|---------|----------|
| `open_questions_valid.md` | Valid QIDs | OQ-SIG-001, OQ-PLAN-002, etc. |
| `open_questions_invalid_code.md` | Non-canonical flow code | OQ-PLAN-001, OQ-BUILD-002 |
| `open_questions_bad_padding.md` | Invalid numeric suffix | OQ-SIG-1, OQ-SIG-1234 |
| `open_questions_mixed.md` | Valid and invalid QIDs | For multi-match testing |

## Contract Test Plan

No api_contracts.yaml present for this run.

Contract testing is implicitly covered by REQ-004 (Build-to-Gate handshake):
- Receipt schema assertions (status enum values, required fields)
- Error shapes (missing field detection, invalid value messages)

Exit code contract (REQ-005):
- Exit 0: success, no errors (warnings allowed without --strict)
- Exit non-zero: errors present (or warnings with --strict)

Diagnostic output contract (NFR-OPS-001):
- Each warning/error includes: rule ID, file path, violation description
- Line number included where applicable

## Non-Behavioral Verification (from verification_notes.md)

| NFR | Verification Strategy | When | How |
|-----|----------------------|------|-----|
| NFR-PERF-001 | Measure pack-check runtime; assert < 30s total, < 5s incremental | Gate / CI | CI timing wrapper; compare against baseline |
| NFR-REL-001 | Run pack-check twice; diff output; assert byte-identical | Gate / CI | Determinism test in CI |
| NFR-OPS-001 | Manual review of diagnostic output format | Gate / Manual | Code review; sample output inspection |
| NFR-COMP-001 | Regression test on existing valid artifacts | Gate / CI | Run pack-check on known-good pack state |
| NFR-SEC-001 | Code review; verify no file contents printed for potential secrets | Gate / Code Review | Review reporter.rs output functions |
| NFR-MAINT-001 | Code review; verify constants in contracts.rs | Plan / Code Review | Verify SKILL_CLI_SUBCOMMANDS and OPENQ_FLOW_CODES are constants |

## Risk-Based Priority Assignment

Based on `early_risks.md`:

### P0 (Critical Path - Must Not Fail)

- REQ-004: Build-to-Gate handshake (core handoff integrity)
- REQ-005: Warning-first mode (backward compatibility gate; RSK-004)
- REQ-006: No false positives (regression prevention; RSK-001)
- NFR-COMP-001: Backward compatibility (CI pipeline stability)
- NFR-SEC-001: No secrets in output (security baseline)

### P1 (Primary User Path)

- REQ-001: Flow boundary enforcement (ownership model integrity)
- REQ-002: Skills section enforcement (documentation completeness)
- REQ-003: OpenQ prefix validation (data consistency)
- NFR-REL-001: Deterministic output (CI reliability)
- NFR-OPS-001: Diagnostic clarity (developer experience)

### P2 (Secondary Behavior)

- NFR-PERF-001: CI runtime (bounded by 30s; low risk per RSK-005)
- NFR-MAINT-001: Pattern maintainability (long-term concern)

## Gaps and Questions

- Q: Which 4 agents are missing Skills sections? Suggested default: Enumerate via audit during build (grep demoswarm.sh minus grep "## Skills"). Impact: If intentionally exempt, need exemption list in validation rule rather than adding sections. (OQ-SIG-004, OQ-PLAN-009)

- Q: Is PLN or PLAN canonical for OpenQ flow codes? Suggested default: PLN/BLD per openq-tools (implementation). Impact: REQ-003 regex pattern depends on this resolution. (OQ-SIG-002, OQ-PLAN-004)

- Q: Should test fixtures be committed or generated dynamically? Suggested default: Committed to repo for reviewability. Impact: Dynamic generation adds CI complexity. (OQ-SIG-010)

## Recommended Next (Flow 3 Implementation Order)

1. **Resolve OQ-SIG-002**: Confirm PLN/BLD is canonical; update stable-markers.md line 60 and contracts.md line 184 if needed
2. **Add constants to contracts.rs**: SKILL_CLI_SUBCOMMANDS list and OPENQ_FLOW_CODES (SIG/PLN/BLD/GAT/DEP/WIS)
3. **Create test fixtures directory**: `tools/demoswarm-pack-check/tests/fixtures/`
4. **Create Build receipt fixtures**: Valid/invalid JSON files for REQ-004
5. **Verify check 49 coverage**: Run pack-check on current pack; enumerate any warnings for REQ-002
6. **Implement check 50 (REQ-001)**: Flow boundary enforcement in drift.rs
7. **Implement check 51 (REQ-003)**: OpenQ prefix validation in drift.rs
8. **Verify --strict flag behavior**: Ensure REQ-005 AC-1 through AC-4 are satisfied
9. **Create flow command/agent/openq fixtures**: For comprehensive testing
10. **Run baseline validation (REQ-006)**: Capture current pack state before enabling new checks
11. **Update documentation**: pack-check.md with new rules; README.md for fixtures
