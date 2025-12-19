Feature: Skills Section Enforcement
  As a pack maintainer
  I want agents using demoswarm.sh to declare their skill dependencies
  So that skill usage is transparent and documented

  Background:
    Given the pack-check validation tool is available
    And agent files are located at ".claude/agents/"

  @REQ-002 @smoke
  Scenario: Agent with demoswarm.sh and Skills section passes validation
    Given an agent file "build-cleanup.md" with content:
      """
      # Build Cleanup Agent

      ## Skills
      - runs-derive: count markers
      - runs-index: update status

      ## Behavior
      Run: bash .claude/scripts/demoswarm.sh count pattern --file receipt.json
      """
    When pack-check validates the skills section rules
    Then the validation completes with exit code 0
    And no warnings are reported for missing Skills section

  @REQ-002 @error
  Scenario: Agent with demoswarm.sh but no Skills section produces warning
    Given an agent file "test-agent.md" with content:
      """
      # Test Agent

      ## Behavior
      Run: bash .claude/scripts/demoswarm.sh ms get --file summary.md
      """
    When pack-check validates the skills section rules
    Then a warning is reported for missing Skills section
    And the warning identifies file ".claude/agents/test-agent.md"
    And the validation completes with exit code 0

  @REQ-002
  Scenario: Agent without demoswarm.sh is not required to have Skills section
    Given an agent file "simple-critic.md" with content:
      """
      # Simple Critic Agent

      ## Behavior
      Review the code and provide feedback.
      """
    When pack-check validates the skills section rules
    Then the validation completes with exit code 0
    And no warnings are reported for this agent

  @REQ-002 @edge
  Scenario: Agent invoking skill via Skill tool only is not flagged
    Given an agent file "skill-invoker.md" with content:
      """
      # Skill Invoker Agent

      ## Behavior
      Invoke the runs-derive skill to count markers.
      Use: Skill(skill: "runs-derive")
      """
    When pack-check validates the skills section rules
    Then the validation completes with exit code 0
    And no warnings are reported for missing Skills section

  @REQ-002 @error
  Scenario: Multiple agents missing Skills sections are all identified
    Given agent files exist:
      | file               | has_demoswarm_sh | has_skills_section |
      | agent-a.md         | yes              | no                 |
      | agent-b.md         | yes              | no                 |
      | agent-c.md         | yes              | yes                |
    When pack-check validates the skills section rules
    Then warnings are reported for exactly 2 agents
    And the warnings identify "agent-a.md" and "agent-b.md"

  @REQ-002 @error
  Scenario: Missing Skills section with --strict flag fails validation
    Given an agent file "test-agent.md" with content:
      """
      # Test Agent

      ## Behavior
      Run: bash .claude/scripts/demoswarm.sh receipt get --file r.json
      """
    When pack-check validates the skills section rules with --strict flag
    Then the validation fails with non-zero exit code
    And an error is reported for missing Skills section
