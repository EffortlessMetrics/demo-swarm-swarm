# Test Execution Report

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: 4
route_to_agent: gate-orchestrator
blockers: []
missing_required: []
concerns: []
test_summary:
mode: verify
command: cargo test --lib && cargo test --test '\*' && cargo tarpaulin --out Html --output-dir coverage --timeout 300
exit_code: 0
passed: 420
failed: 0
skipped: 0
xfailed: 0
xpassed: 0
duration_seconds: 32
coverage_percent: 89.29

## Inputs Used

- `/c/Code/Swarm/demo-swarm-staging/tools/demoswarm-pack-check/Cargo.toml`
- `/c/Code/Swarm/demo-swarm-staging/tools/demoswarm-pack-check/src/` (unit test modules)
- `/c/Code/Swarm/demo-swarm-staging/tools/demoswarm-pack-check/tests/check_integration_test.rs` (41 integration tests)

## Execution

- tool: cargo test
- mode: verify
- command: `cargo test --lib && cargo test --test '*' && cargo tarpaulin --out Html --output-dir coverage --timeout 300`
- exit_code: 0
- duration: 32 seconds (test execution + coverage instrumentation)
- project: tools/demoswarm-pack-check (Rust/Cargo-based)

### Unit Tests (--lib)

```
running 379 tests
test result: ok. 379 passed; 0 failed; 0 ignored; 0 measured
```

Coverage breakdown across modules:

- src/checks/control_plane.rs: 267/269 lines (99.26%)
- src/checks/drift.rs: 357/410 lines (87.07%)
- src/checks/flow.rs: 447/542 lines (82.47%)
- src/checks/structure.rs: 88/88 lines (100%)
- src/checks/wisdom.rs: 45/59 lines (76.27%)
- src/contracts.rs: 54/54 lines (100%)
- src/ctx.rs: 36/43 lines (83.72%)
- src/inventory.rs: 27/28 lines (96.43%)
- src/reporter.rs: 70/71 lines (98.59%)
- src/util.rs: 60/61 lines (98.36%)
- src/runner.rs: 12/12 lines (100%)
- src/main.rs: 4/7 lines (57.14%)

### Integration Tests (--test)

```
running 41 tests
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured
```

Coverage for integration test binary: tracked alongside unit tests in merged profraw.

## Canonical Summary (tool-bound)

Tool output line (tarpaulin):

```
89.29% coverage, 1476/1653 lines covered, +0.00% change in coverage
```

## Test Summary (Canonical): passed=420 failed=0 skipped=0 xfailed=0 xpassed=0

## Coverage Result (Canonical): 89.29% (1476/1653 lines covered)

## Failures (if any)

None. All tests passed.

## Notes

- All 420 tests (379 unit + 41 integration) passed with exit code 0.
- Coverage: 89.29% — stable and consistent with prior measurement (no drift from 89.29% reported in build_receipt.json).
- The discrepancy between build_receipt.json (420 tests) and prior test_execution.md (294 tests) is now resolved: 420 is the authoritative count across both unit and integration test suites.
- Coverage generated via cargo-tarpaulin with HTML report in tools/demoswarm-pack-check/coverage/index.html.
- Integration tests include fixtures for build-receipt validation, OpenQ prefix validation, flow boundary enforcement, skills section enforcement, and multiple baseline/backward-compatibility checks.
