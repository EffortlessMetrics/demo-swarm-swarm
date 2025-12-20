Feature: Flow Count Documentation Alignment
  As a pack integrator
  I want documentation to consistently reference the correct flow count
  So that I understand the actual flow architecture without confusion

  Background:
    Given the DemoSwarm pack implements seven flows
    And CLAUDE.md is the authoritative source for flow count

  @REQ-001 @smoke
  Scenario: README references seven flows
    When the user reads README.md
    Then the flow section header references "seven flows"
    And the flow section body text references "seven flows"
    And no occurrences of "six flows" appear in the file

  @REQ-001
  Scenario: DEMO_RUN references seven flows with enumeration
    When the user reads DEMO_RUN.md
    Then the document references "seven flows"
    And the flows are enumerated as Signal through Wisdom
    And no occurrences of "six flows" appear in the file

  @REQ-001
  Scenario: Architecture documentation references seven flows
    When the user reads docs/explanation/architecture.md
    Then the flow section header references "seven flows"
    And the flow enumeration lists all seven flows
    And no occurrences of "six flows" appear in the file

  @REQ-001
  Scenario: CHANGELOG clarifies actual command count
    When the user reads CHANGELOG.md v1.0.0 section
    Then the flow command claim is annotated or corrected
    And the annotation clarifies "10 command files implementing 7 flows"

  @REQ-001 @edge
  Scenario: No stale flow count references remain in public documentation
    Given the public documentation files are README.md, DEMO_RUN.md, and docs/explanation/architecture.md
    When a search is performed for "six flows" across all public documentation
    Then zero matches are returned

  @REQ-004 @smoke
  Scenario: CLAUDE.md flow table lists all seven flows
    When the user reads the CLAUDE.md flow table
    Then the table lists Signal as Flow 1
    And the table lists Plan as Flow 2
    And the table lists Build as Flow 3
    And the table lists Review as Flow 4
    And the table lists Gate as Flow 5
    And the table lists Deploy as Flow 6
    And the table lists Wisdom as Flow 7

  @REQ-004
  Scenario: CLAUDE.md flow table includes variant commands
    When the user reads the CLAUDE.md flow table
    Then the table includes or references flow-4-review as a variant
    And the table includes or references flow-5-gate as a variant
    And the table includes or references flow-6-deploy as a variant
    And the table includes or references flow-7-wisdom as a variant

  @REQ-004
  Scenario: CLAUDE.md flow table numbering is consistent
    When the user reads CLAUDE.md
    Then the flow table numbering matches the "7 flows" statement in line 13
    And no flow number is skipped or duplicated
