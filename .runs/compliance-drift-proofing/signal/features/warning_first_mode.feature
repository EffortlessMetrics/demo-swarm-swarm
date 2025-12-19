Feature: Warning-First Validation Mode
  As a pack maintainer
  I want new compliance rules to warn by default
  So that adoption is incremental and existing artifacts are not broken

  Background:
    Given the pack-check validation tool is available

  @REQ-005 @smoke
  Scenario: Validation completes successfully when only warnings are present
    Given a pack with violations of new compliance rules:
      | rule    | violation_present |
      | REQ-001 | yes               |
      | REQ-002 | yes               |
      | REQ-003 | no                |
    When pack-check validates without the --strict flag
    Then warnings are reported for REQ-001 and REQ-002 violations
    And the validation completes with exit code 0

  @REQ-005 @error
  Scenario: Validation fails with --strict when warnings are present
    Given a pack with violations of new compliance rules:
      | rule    | violation_present |
      | REQ-001 | yes               |
    When pack-check validates with the --strict flag
    Then errors are reported for REQ-001 violations
    And the validation fails with non-zero exit code

  @REQ-005
  Scenario: pack-check --strict flag elevates all new rule warnings
    Given a pack with violations of multiple new rules
    When pack-check validates with the --strict flag
    Then violations of REQ-001 are reported as errors
    And violations of REQ-002 are reported as errors
    And violations of REQ-003 are reported as errors

  @REQ-005
  Scenario: Warning output includes rule identifier
    Given a pack with a REQ-001 violation
    When pack-check validates without the --strict flag
    Then the warning output includes "REQ-001" or equivalent rule identifier
    And the warning identifies the specific rule that was violated

  @REQ-005
  Scenario: Warning output includes file location
    Given a flow command file "flow-test.md" with a REQ-001 violation
    When pack-check validates without the --strict flag
    Then the warning output includes the file path ".claude/commands/flow-test.md"
    And the violation is traceable to the source file

  @REQ-005 @edge
  Scenario: Mixed old and new rule violations with --strict
    Given a pack with:
      | violation_type        | rule_category |
      | invalid status enum   | existing rule |
      | demoswarm.sh in flow  | new rule      |
    When pack-check validates with the --strict flag
    Then both violations are reported as errors
    And the validation fails with non-zero exit code

  @REQ-005
  Scenario: Clean pack passes validation without --strict flag
    Given a pack with no compliance violations
    When pack-check validates without the --strict flag
    Then the validation completes with exit code 0
    And no warnings are reported

  @REQ-005
  Scenario: Clean pack passes validation with --strict flag
    Given a pack with no compliance violations
    When pack-check validates with the --strict flag
    Then the validation completes with exit code 0
    And no errors are reported
