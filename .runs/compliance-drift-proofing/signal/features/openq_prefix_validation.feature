Feature: OpenQ Prefix Pattern Validation
  As a pack maintainer
  I want QID patterns to follow a consistent format
  So that question tracking is reliable across flows

  Background:
    Given the pack-check validation tool is available
    And open_questions.md files may exist under ".runs/"

  @REQ-003 @smoke
  Scenario: Valid QID with canonical flow code passes validation
    Given an open_questions.md file with content:
      """
      # Open Questions

      - QID: OQ-SIG-001
        - Q: What is the expected behavior? [OPEN]
      """
    When pack-check validates the OpenQ prefix rules
    Then the validation completes with exit code 0
    And no warnings are reported for QID format

  @REQ-003
  Scenario Outline: Valid QIDs for each canonical flow code
    Given an open_questions.md file with QID "<qid>"
    When pack-check validates the OpenQ prefix rules
    Then the validation completes with exit code 0

    Examples:
      | qid          |
      | OQ-SIG-001   |
      | OQ-PLN-042   |
      | OQ-BLD-999   |
      | OQ-GAT-123   |
      | OQ-DEP-007   |
      | OQ-WIS-256   |

  @REQ-003 @error
  Scenario: Non-canonical flow code produces warning
    Given an open_questions.md file with content:
      """
      # Open Questions

      - QID: OQ-PLAN-001
        - Q: Using long-form flow code? [OPEN]
      """
    When pack-check validates the OpenQ prefix rules
    Then a warning is reported for non-canonical flow code "PLAN"
    And the warning suggests using "PLN" instead
    And the validation completes with exit code 0

  @REQ-003 @error
  Scenario Outline: Non-canonical flow codes produce warnings
    Given an open_questions.md file with QID "<invalid_qid>"
    When pack-check validates the OpenQ prefix rules
    Then a warning is reported for non-canonical flow code

    Examples:
      | invalid_qid    |
      | OQ-PLAN-001    |
      | OQ-BUILD-002   |
      | OQ-GATE-003    |
      | OQ-DEPLOY-004  |
      | OQ-WISDOM-005  |

  @REQ-003 @error
  Scenario: QID with non-zero-padded numeric suffix produces warning
    Given an open_questions.md file with content:
      """
      # Open Questions

      - QID: OQ-SIG-1
        - Q: Single digit suffix? [OPEN]
      """
    When pack-check validates the OpenQ prefix rules
    Then a warning is reported for invalid numeric suffix
    And the warning indicates suffix must be zero-padded to 3 digits

  @REQ-003 @error
  Scenario: QID with four-digit numeric suffix produces warning
    Given an open_questions.md file with QID "OQ-SIG-1234"
    When pack-check validates the OpenQ prefix rules
    Then a warning is reported for invalid numeric suffix

  @REQ-003 @edge
  Scenario: Multiple QIDs in same file are all validated
    Given an open_questions.md file with content:
      """
      # Open Questions

      - QID: OQ-SIG-001
        - Q: Valid question [OPEN]

      - QID: OQ-PLAN-002
        - Q: Invalid flow code [OPEN]

      - QID: OQ-BLD-3
        - Q: Invalid padding [OPEN]
      """
    When pack-check validates the OpenQ prefix rules
    Then warnings are reported for exactly 2 QIDs
    And the warnings identify "OQ-PLAN-002" and "OQ-BLD-3"

  @REQ-003 @error
  Scenario: Invalid QID with --strict flag fails validation
    Given an open_questions.md file with QID "OQ-PLAN-001"
    When pack-check validates the OpenQ prefix rules with --strict flag
    Then the validation fails with non-zero exit code
