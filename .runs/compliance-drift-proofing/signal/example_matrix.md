# Example Matrix

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

## Coverage Summary

| Requirement | Happy Path | Edge Cases | Error Cases | Scenario Count | Notes                                                                 |
| ----------- | ---------- | ---------- | ----------- | -------------- | --------------------------------------------------------------------- |
| REQ-001     | Yes        | Yes        | Yes         | 6              | Flow boundary enforcement                                             |
| REQ-002     | Yes        | Yes        | Yes         | 6              | Skills section enforcement                                            |
| REQ-003     | Yes        | Yes        | Yes         | 8              | OpenQ prefix validation (includes Scenario Outline with examples)     |
| REQ-004     | Yes        | Yes        | Yes         | 5              | Build-to-Gate handshake test fixtures                                 |
| REQ-005     | Yes        | Yes        | Yes         | 8              | Warning-first validation mode (split clean pack scenario)             |
| REQ-006     | Yes        | Yes        | N/A         | 7              | No false positives (no error scenarios; focus is baseline validation) |

## Scenario Index

| REQ     | Scenario                                                           | Feature File                                | Tags            |
| ------- | ------------------------------------------------------------------ | ------------------------------------------- | --------------- |
| REQ-001 | Flow command file without skill references passes validation       | features/flow_boundary_enforcement.feature  | @REQ-001 @smoke |
| REQ-001 | Flow command file containing demoswarm.sh produces warning         | features/flow_boundary_enforcement.feature  | @REQ-001 @error |
| REQ-001 | Flow command file containing skill CLI subcommand produces warning | features/flow_boundary_enforcement.feature  | @REQ-001 @error |
| REQ-001 | Flow command with skill name in prose context is not flagged       | features/flow_boundary_enforcement.feature  | @REQ-001 @edge  |
| REQ-001 | Flow command file with demoswarm.sh and --strict flag fails        | features/flow_boundary_enforcement.feature  | @REQ-001 @error |
| REQ-001 | Validation scans all flow command files matching pattern           | features/flow_boundary_enforcement.feature  | @REQ-001        |
| REQ-002 | Agent with demoswarm.sh and Skills section passes validation       | features/skills_section_enforcement.feature | @REQ-002 @smoke |
| REQ-002 | Agent with demoswarm.sh but no Skills section produces warning     | features/skills_section_enforcement.feature | @REQ-002 @error |
| REQ-002 | Agent without demoswarm.sh is not required to have Skills section  | features/skills_section_enforcement.feature | @REQ-002        |
| REQ-002 | Agent invoking skill via Skill tool only is not flagged            | features/skills_section_enforcement.feature | @REQ-002 @edge  |
| REQ-002 | Multiple agents missing Skills sections are all identified         | features/skills_section_enforcement.feature | @REQ-002 @error |
| REQ-002 | Missing Skills section with --strict flag fails validation         | features/skills_section_enforcement.feature | @REQ-002 @error |
| REQ-003 | Valid QID with canonical flow code passes validation               | features/openq_prefix_validation.feature    | @REQ-003 @smoke |
| REQ-003 | Valid QIDs for each canonical flow code (6 examples)               | features/openq_prefix_validation.feature    | @REQ-003        |
| REQ-003 | Non-canonical flow code produces warning                           | features/openq_prefix_validation.feature    | @REQ-003 @error |
| REQ-003 | Non-canonical flow codes produce warnings (5 examples)             | features/openq_prefix_validation.feature    | @REQ-003 @error |
| REQ-003 | QID with non-zero-padded numeric suffix produces warning           | features/openq_prefix_validation.feature    | @REQ-003 @error |
| REQ-003 | QID with four-digit numeric suffix produces warning                | features/openq_prefix_validation.feature    | @REQ-003 @error |
| REQ-003 | Multiple QIDs in same file are all validated                       | features/openq_prefix_validation.feature    | @REQ-003 @edge  |
| REQ-003 | Invalid QID with --strict flag fails validation                    | features/openq_prefix_validation.feature    | @REQ-003 @error |
| REQ-004 | Valid build_receipt.json fixture passes receipt validation         | features/build_gate_handshake.feature       | @REQ-004 @smoke |
| REQ-004 | Invalid build_receipt.json fixture fails receipt validation        | features/build_gate_handshake.feature       | @REQ-004 @error |
| REQ-004 | Test suite includes Build receipt validation test case             | features/build_gate_handshake.feature       | @REQ-004        |
| REQ-004 | Invalid fixture with missing required field fails validation       | features/build_gate_handshake.feature       | @REQ-004 @edge  |
| REQ-004 | Handshake contract documentation exists                            | features/build_gate_handshake.feature       | @REQ-004        |
| REQ-005 | Validation completes successfully when only warnings are present   | features/warning_first_mode.feature         | @REQ-005 @smoke |
| REQ-005 | Validation fails with --strict when warnings are present           | features/warning_first_mode.feature         | @REQ-005 @error |
| REQ-005 | pack-check --strict flag elevates all new rule warnings            | features/warning_first_mode.feature         | @REQ-005        |
| REQ-005 | Warning output includes rule identifier                            | features/warning_first_mode.feature         | @REQ-005        |
| REQ-005 | Warning output includes file location                              | features/warning_first_mode.feature         | @REQ-005        |
| REQ-005 | Mixed old and new rule violations with --strict                    | features/warning_first_mode.feature         | @REQ-005 @edge  |
| REQ-005 | Clean pack passes validation without --strict flag                 | features/warning_first_mode.feature         | @REQ-005        |
| REQ-005 | Clean pack passes validation with --strict flag                    | features/warning_first_mode.feature         | @REQ-005        |
| REQ-006 | Existing flow command files pass REQ-001 validation                | features/no_false_positives.feature         | @REQ-006 @smoke |
| REQ-006 | Existing agent files pass REQ-002 validation                       | features/no_false_positives.feature         | @REQ-006 @smoke |
| REQ-006 | Existing open_questions.md files pass REQ-003 validation           | features/no_false_positives.feature         | @REQ-006 @smoke |
| REQ-006 | Validation baseline is established before introducing rules        | features/no_false_positives.feature         | @REQ-006        |
| REQ-006 | Known exception agents are documented not flagged as errors        | features/no_false_positives.feature         | @REQ-006 @edge  |
| REQ-006 | New rule introduction does not change existing rule behavior       | features/no_false_positives.feature         | @REQ-006        |
| REQ-006 | Edge case prose that resembles violations is not flagged           | features/no_false_positives.feature         | @REQ-006 @edge  |

## Gaps

None. All functional requirements (REQ-001 through REQ-006) have BDD coverage with happy path, edge cases, and error cases (where applicable).

## Notes

- Counts are derived mechanically by signal-cleanup; this matrix is for human navigation.
- REQ-003 uses Scenario Outline with Examples tables, resulting in multiple test cases from fewer scenario definitions.
- REQ-006 focuses on baseline validation (no false positives), so error scenarios are N/A -- the requirement is about _not_ producing errors on valid artifacts.
- NFRs (NFR-PERF-001, NFR-REL-001, NFR-OPS-001, NFR-COMP-001, NFR-SEC-001, NFR-MAINT-001) are non-behavioral and documented in verification_notes.md.

## Iteration 2 Changes

- Split "Clean pack passes validation in both modes" into two separate scenarios per BDD best practice (one When-Then per scenario)
- Made documentation assertions in build_gate_handshake.feature more concrete (README.md with specific section titles)
- Replaced abstract "validation result is PASS" with concrete exit code and warning/error assertions throughout
- Total scenario count increased from 39 to 40
