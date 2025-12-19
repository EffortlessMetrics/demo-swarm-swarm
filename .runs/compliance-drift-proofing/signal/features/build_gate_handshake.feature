Feature: Build-to-Gate Handshake Test Scenario
  As a pack maintainer
  I want Build receipt validation to have test coverage
  So that cross-flow handshake integrity is verified

  Background:
    Given the pack-check test suite is available
    And test fixtures exist at "tools/demoswarm-pack-check/tests/fixtures/"

  @REQ-004 @smoke
  Scenario: Valid build_receipt.json fixture passes receipt validation
    Given a test fixture file "valid_build_receipt.json" with content:
      """
      {
        "run_id": "test-run",
        "flow": "build",
        "status": "VERIFIED",
        "counts": {
          "scenarios_written": 5,
          "tests_passed": 10,
          "tests_failed": 0
        },
        "quality_gates": {
          "tests_pass": true,
          "lint_clean": true
        },
        "timestamp": "2025-01-15T10:00:00Z"
      }
      """
    When the receipt-checker validation logic processes this fixture
    Then the validation completes with exit code 0
    And no validation errors are reported for the fixture

  @REQ-004 @error
  Scenario: Invalid build_receipt.json fixture fails receipt validation
    Given a test fixture file "invalid_build_receipt.json" with content:
      """
      {
        "run_id": "test-run",
        "flow": "build",
        "status": "INVALID_STATUS",
        "counts": {}
      }
      """
    When the receipt-checker validation logic processes this fixture
    Then the validation fails with non-zero exit code
    And the failure output identifies invalid status value "INVALID_STATUS"

  @REQ-004
  Scenario: Test suite includes Build receipt validation test case
    Given the pack-check test suite at "tools/demoswarm-pack-check/tests/"
    When examining the test cases
    Then a test case exists that validates Build receipt structure
    And the test case exercises both valid and invalid fixtures

  @REQ-004 @edge
  Scenario: Invalid fixture with missing required field fails validation
    Given a test fixture file "missing_field_receipt.json" with content:
      """
      {
        "flow": "build",
        "status": "VERIFIED"
      }
      """
    When the receipt-checker validation logic processes this fixture
    Then the validation fails with non-zero exit code
    And the failure output identifies missing "run_id" field

  @REQ-004
  Scenario: Handshake contract documentation exists
    Given test fixtures for Build receipt validation exist
    Then a README.md file exists in the fixtures directory
    And the README.md contains a section titled "Required Fields"
    And the README.md contains a section titled "Valid Status Values"
    And the README.md contains a section titled "Cross-Flow Expectations"
