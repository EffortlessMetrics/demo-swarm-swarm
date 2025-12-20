Feature: Agent Color Coding Documentation
  As a documentation maintainer
  I want agent color coding to be acknowledged as functional metadata
  So that users understand its purpose and usage

  Background:
    Given agent frontmatter includes a color field
    And color coding exists in agent Markdown files

  @REQ-007 @smoke
  Scenario: Documentation acknowledges color field existence
    When the user reads agent documentation
    Then the documentation acknowledges that agent frontmatter includes a color field
    And the color field is described as part of agent metadata

  @REQ-007
  Scenario: Documentation clarifies color field purpose
    When the user reads agent documentation
    Then the documentation clarifies whether color coding is advisory or schema-validated
    And the clarification states if colors are for human consumption
    And the clarification states if colors are used by tooling

  @REQ-007 @edge
  Scenario: Color consumer is documented if functional
    Given color coding is used for routing or UI logic
    When the user reads agent documentation
    Then the documentation specifies the consumer of color metadata
    And the consumer is identified as tooling, UI, or other system

  @REQ-007
  Scenario: Example agent frontmatter includes color field
    When the user reads agent documentation examples
    Then example agent frontmatter includes the color field
    And the example shows a valid color value

  @REQ-007 @error
  Scenario: Incorrect documentation-only claim is corrected
    Given documentation previously claimed color is documentation-only
    When the color field is functional metadata
    Then the incorrect claim is corrected
    And the correction describes actual color field usage
