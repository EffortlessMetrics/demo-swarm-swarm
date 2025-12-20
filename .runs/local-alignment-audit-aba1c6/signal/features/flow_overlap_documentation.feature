Feature: Flow Overlap Semantics Documentation
  As a pack integrator
  I want documentation explaining when to use each flow variant
  So that I can choose the correct entry point for my workflow

  Background:
    Given the DemoSwarm pack has multiple flow variant commands
    And each variant represents a different entry point into a flow phase

  @REQ-002 @smoke
  Scenario: Documentation explains flow-4-gate vs flow-4-review
    When the user reads the flow overlap documentation
    Then the relationship between flow-4-gate and flow-4-review is explained
    And the documentation describes flow-4-review as the entry point after PR feedback
    And the documentation describes flow-4-gate as the entry point before merge decision

  @REQ-002
  Scenario: Documentation explains flow-5-gate vs flow-5-deploy
    When the user reads the flow overlap documentation
    Then the relationship between flow-5-gate and flow-5-deploy is explained
    And the documentation describes flow-5-gate as gate verdict re-entry
    And the documentation describes flow-5-deploy as deployment execution

  @REQ-002
  Scenario: Documentation explains flow-6-deploy vs flow-6-wisdom
    When the user reads the flow overlap documentation
    Then the relationship between flow-6-deploy and flow-6-wisdom is explained
    And the documentation describes flow-6-deploy as deploy-after-gate path
    And the documentation describes flow-6-wisdom as wisdom extraction path

  @REQ-002 @edge
  Scenario: Flow variant guidance is actionable
    When the user reads the flow overlap documentation
    Then specific guidance exists for when to use each variant
    And the guidance includes examples such as "use flow-4-review after PR feedback"
    And the guidance includes examples such as "use flow-4-gate before merge"

  @REQ-002
  Scenario: Flow overlap documentation is discoverable
    Given the user is looking for flow variant guidance
    When the user reads README.md or docs/explanation/architecture.md
    Then the flow overlap documentation is present or linked
    And the documentation location follows pack documentation conventions

  @REQ-003 @smoke
  Scenario: Flow 7 is included in flow enumeration
    When the user reads README.md or architecture documentation
    Then Flow 7 is included in the flow list
    And Flow 7 is named as Wisdom or flow-7-wisdom

  @REQ-003
  Scenario: Flow 7 purpose is documented
    When the user reads the Flow 7 documentation
    Then the purpose of flow-7-wisdom is explained
    And the documentation describes when to invoke flow-7-wisdom

  @REQ-003
  Scenario: Flow 7 vs Flow 6 wisdom difference is explained
    When the user reads the Flow 7 documentation
    Then the difference between flow-7-wisdom and flow-6-wisdom is explained
    And guidance specifies when to use flow-7-wisdom vs flow-6-wisdom

  @REQ-003 @error
  Scenario: Missing Flow 7 documentation is flagged
    Given flow-7-wisdom.md exists as a command file
    When the user searches for Flow 7 in public documentation
    And no documentation for Flow 7 is found
    Then the documentation gap is identified as a defect
