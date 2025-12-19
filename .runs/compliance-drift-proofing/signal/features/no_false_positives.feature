Feature: No False Positives on Existing Artifacts
  As a pack maintainer
  I want new validation rules to not flag valid existing artifacts
  So that the pack does not regress when rules are introduced

  Background:
    Given the pack-check validation tool is available
    And the current pack state is the validation baseline

  @REQ-006 @smoke
  Scenario: Existing flow command files pass REQ-001 validation
    Given the current pack flow command files at ".claude/commands/flow-*.md"
    When pack-check validates the flow boundary rules
    Then all existing flow command files pass validation
    And no false positive warnings are produced

  @REQ-006 @smoke
  Scenario: Existing agent files pass REQ-002 validation
    Given the current pack agent files at ".claude/agents/"
    When pack-check validates the skills section rules
    Then agents with demoswarm.sh and Skills sections pass
    And agents without demoswarm.sh are not flagged
    And gaps are documented as exceptions requiring remediation

  @REQ-006 @smoke
  Scenario: Existing open_questions.md files pass REQ-003 validation
    Given open_questions.md files in the current pack
    When pack-check validates the OpenQ prefix rules
    Then existing valid QIDs pass validation
    And any divergent QIDs are enumerated as known exceptions

  @REQ-006
  Scenario: Validation baseline is established before introducing rules
    Given the pack-check rules REQ-001, REQ-002, and REQ-003 are being introduced
    When pack-check is run on the current pack state
    Then a validation baseline is recorded
    And the baseline documents:
      | category                  | count  |
      | flow commands scanned     | N      |
      | agents scanned            | N      |
      | open_questions.md scanned | N      |
      | violations found          | N      |

  @REQ-006 @edge
  Scenario: Known exception agents are documented not flagged as errors
    Given agent files that use demoswarm.sh without Skills sections exist
    And these agents are documented as requiring remediation
    When pack-check validates the skills section rules
    Then warnings are produced for these agents
    And the warnings indicate remediation is needed
    And no false positive errors claim these are compliant

  @REQ-006
  Scenario: New rule introduction does not change existing rule behavior
    Given pack-check existing rules for status enum validation
    When REQ-001, REQ-002, and REQ-003 rules are added
    Then existing status enum validation continues to work
    And existing rule violations continue to be reported as before
    And no regression in existing validation behavior occurs

  @REQ-006 @edge
  Scenario: Edge case prose that resembles violations is not flagged
    Given a flow command file discussing validation concepts:
      """
      # Flow Overview
      This flow validates that agents do not contain hardcoded counts.
      The index is used to track run status.
      Use the receipt to verify completion.
      """
    When pack-check validates the flow boundary rules
    Then the validation completes with exit code 0
    And prose usage of "count", "index", "receipt" is not flagged
