# Implementation Changes Summary for compliance-drift-proofing

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []
```

## Implementation Facts

```yaml
work_status: COMPLETED
tests_run: yes
tests_passed: yes
scope_manifest_used: no
```

## What Changed

* **contracts.rs**: Added `SKILL_CLI_SUBCOMMANDS` constant (12 subcommands) and `OPENQ_FLOW_CODES` constant (6 canonical codes)
* **contracts.rs**: Extended `Contracts` struct with two new fields for the constants
* **drift.rs**: Added check 52 (`check_flow_boundary_enforcement`) - scans flow commands for demoswarm.sh and skill CLI syntax
* **drift.rs**: Added check 53 (`check_openq_prefix_validation`) - validates QID patterns in open_questions.md files
* **drift.rs**: Added helper function `suggest_canonical_code` for mapping non-canonical to canonical flow codes
* **mod.rs**: Updated comment to reflect new checks range (1..53)

## REQ/NFR to Implementation Map

| ID           | Implementation Pointer | Notes               |
| ------------ | ---------------------- | ------------------- |
| REQ-001      | `drift.rs::check_flow_boundary_enforcement` | Check 52 - scans flow-*.md for demoswarm.sh and skill CLI subcommands |
| REQ-002      | `drift.rs::check_skills_section_required` | Check 49 - already existed and verified adequate |
| REQ-003      | `drift.rs::check_openq_prefix_validation` | Check 53 - validates OQ-<FLOW>-<NNN> patterns |
| REQ-005      | `cli.rs::Cli::strict_warnings` | Flag already exists and works correctly |
| NFR-MAINT-001 | `contracts.rs::SKILL_CLI_SUBCOMMANDS`, `contracts.rs::OPENQ_FLOW_CODES` | Constants centralized for easy maintenance |
| NFR-REL-001  | Both checks use sorted output for determinism | Output is deterministic |
| NFR-OPS-001  | Both checks use `rep.warn()` with file path and line numbers | Diagnostics include location info |

## Contract / Interface Notes

* Check 52 produces warnings (not errors) via `rep.warn()` for boundary violations
* Check 53 produces warnings for non-canonical flow codes (PLAN, BUILD, etc.) with suggestions
* Check 53 produces warnings for invalid numeric padding (not 3 digits)
* Both checks integrate with existing `--strict-warnings` flag for CI enforcement

## Observability Notes

* No new observability hooks added (pack-check is a validation tool, not instrumented)
* Checks produce structured diagnostics via the existing Reporter infrastructure
* JSON output format includes check_id (52, 53) for filtering

## Tests

* Intended tests: `tools/demoswarm-pack-check/tests/check_integration_test.rs`
* Test-runner result: 36 passed, 0 failed, 5 ignored (TDD stubs)
* TDD stubs remain ignored as they were pre-implementation markers

Test coverage:
- Fixture structure tests for flow commands (clean, violation, prose patterns)
- Fixture structure tests for OpenQ patterns (valid, invalid, bad padding, mixed)
- Integration tests for check 49 (Skills section) - verified working
- Integration tests for --strict-warnings flag behavior
- Baseline validation tests (pack runs without false positives)
- Determinism tests (identical output on repeated runs)

## Known Issues / Handoffs

* None - implementation complete and tests pass

## Assumptions Made

* **ASM-001**: Check IDs 52 and 53 are available (verified by reviewing mod.rs - prior highest was 51)
* **ASM-002**: Existing check 49 adequately implements REQ-002 (verified by reading implementation)
* **ASM-003**: --strict_warnings flag already implements REQ-005 (verified in cli.rs and reporter.rs)
* **ASM-004**: PLN/BLD are canonical over PLAN/BUILD (per ADR decision and openq-tools/SKILL.md)

## Inventory (machine countable)

- IMPL_FILE_CHANGED: tools/demoswarm-pack-check/src/contracts.rs
- IMPL_FILE_CHANGED: tools/demoswarm-pack-check/src/checks/drift.rs
- IMPL_FILE_CHANGED: tools/demoswarm-pack-check/src/checks/mod.rs
- IMPL_REQ_IMPLEMENTED: REQ-001
- IMPL_REQ_IMPLEMENTED: REQ-002
- IMPL_REQ_IMPLEMENTED: REQ-003
- IMPL_REQ_IMPLEMENTED: REQ-005
- IMPL_NFR_TOUCHED: NFR-MAINT-001
- IMPL_NFR_TOUCHED: NFR-REL-001
- IMPL_NFR_TOUCHED: NFR-OPS-001
- IMPL_CONTRACT_TOUCHED: none
- IMPL_OBS_HOOK: none
- IMPL_TESTS_RUN: yes
- IMPL_TESTS_PASSED: yes
