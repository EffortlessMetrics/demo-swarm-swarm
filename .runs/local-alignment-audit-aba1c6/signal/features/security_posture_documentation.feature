Feature: Security Posture Documentation Accuracy
  As a security reviewer
  I want security claims to accurately reflect implementation posture
  So that I can assess actual risk without misleading information

  Background:
    Given the DemoSwarm pack includes a secrets scanner
    And security claims must have corresponding code evidence

  @REQ-006 @smoke
  Scenario: Documentation states Rust regex is ReDoS immune
    When the user reads security posture documentation
    Then the documentation states that the Rust regex crate is immune to ReDoS
    And the documentation explains that Rust regex uses finite automata implementation
    And the documentation notes that finite automata do not use backtracking

  @REQ-006
  Scenario: Path traversal is documented as known limitation
    When the user reads security posture documentation
    Then the documentation notes path traversal as a known limitation
    And the limitation is associated with the secrets scanner
    And the documentation states that threat assessment is pending

  @REQ-006 @error
  Scenario: Invalid ReDoS vulnerability claim is corrected
    Given documentation previously claimed ReDoS vulnerability
    When the security documentation is reviewed
    Then the ReDoS vulnerability claim is removed or corrected
    And the correction explains why ReDoS is not possible with Rust regex

  @REQ-006
  Scenario: Security claims reference code evidence
    When the user reads security posture documentation
    Then each security claim references specific code evidence
    And the reference includes the source file name
    And the reference includes the relevant line number

  @REQ-006 @edge
  Scenario: Security claims are verifiable by code inspection
    Given a security claim exists in documentation
    When the claim is traced to its code evidence
    Then the code confirms the claim
    And the verification can be reproduced by inspection
