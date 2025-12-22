Feature: Test Count Documentation Accuracy
  As a test reviewer or compliance auditor
  I want test coverage claims to match actual test execution results
  So that I can trust the documented test posture

  Background:
    Given the DemoSwarm pack has test execution artifacts
    And test_output.log contains the authoritative test results

  @REQ-005 @smoke
  Scenario: Documentation references correct passing test count
    When the user reads test coverage documentation
    Then the documentation references "102 unit tests passing"
    And the count matches the test_output.log artifact

  @REQ-005
  Scenario: Documentation explains filtered tests
    When the user reads test coverage documentation
    Then the documentation explains that 277 tests are filtered
    And the filtered tests are described as integration tests
    And the documentation notes that integration tests require manual environment setup

  @REQ-005 @edge
  Scenario: Conflicting test count claims are corrected
    Given documentation previously claimed a different test count
    When the test count documentation is reviewed
    Then conflicting claims such as "374 tests" are corrected or annotated
    And the annotation provides context for the difference

  @REQ-005
  Scenario: Test count claims include source reference
    When the user reads test coverage documentation
    Then the test count claim includes a source artifact reference
    And the reference specifies test_output.log as the source
    And the reference includes the relevant line number if available

  @REQ-005 @error
  Scenario: Undocumented test count source is flagged
    Given test count claims exist in documentation
    When the documentation does not reference the source artifact
    Then the missing source reference is identified as a documentation gap
