Feature: Flow Boundary Enforcement
  As a pack maintainer
  I want flow command files to remain free of skill-layer CLI syntax
  So that the three-tier ownership model is preserved

  Background:
    Given the pack-check validation tool is available
    And the pack contains flow command files at ".claude/commands/flow-*.md"

  @REQ-001 @smoke
  Scenario: Flow command file without skill references passes validation
    Given a flow command file "flow-1-signal.md" with content:
      """
      # Flow 1 - Signal
      This flow uses agents to process requirements.
      Call the requirements-author agent to proceed.
      """
    When pack-check validates the flow boundary rules
    Then the validation completes with exit code 0
    And no warnings are reported for flow boundary violations

  @REQ-001 @error
  Scenario: Flow command file containing demoswarm.sh produces warning
    Given a flow command file "flow-test.md" with content:
      """
      # Test Flow
      Run: bash .claude/scripts/demoswarm.sh count pattern --file foo.md
      """
    When pack-check validates the flow boundary rules
    Then a warning is reported for "demoswarm.sh" in flow command
    And the warning identifies file ".claude/commands/flow-test.md"
    And the validation completes with exit code 0

  @REQ-001 @error
  Scenario: Flow command file containing skill CLI subcommand produces warning
    Given a flow command file "flow-test.md" with content:
      """
      # Test Flow
      Use the `count` command to tally markers.
      Use `ms get` to extract Machine Summary fields.
      """
    When pack-check validates the flow boundary rules
    Then a warning is reported for skill CLI subcommand in flow command
    And the warning identifies the specific subcommands found
    And the validation completes with exit code 0

  @REQ-001 @edge
  Scenario: Flow command with skill name in prose context is not flagged
    Given a flow command file "flow-1-signal.md" with content:
      """
      # Flow 1 - Signal
      The count of requirements should be documented.
      Use the index to find related runs.
      """
    When pack-check validates the flow boundary rules
    Then the validation completes with exit code 0
    And no warnings are reported for flow boundary violations

  @REQ-001 @error
  Scenario: Flow command file with demoswarm.sh and --strict flag fails
    Given a flow command file "flow-test.md" with content:
      """
      # Test Flow
      Run: bash .claude/scripts/demoswarm.sh index upsert-status
      """
    When pack-check validates the flow boundary rules with --strict flag
    Then the validation fails with non-zero exit code
    And an error is reported for "demoswarm.sh" in flow command

  @REQ-001
  Scenario: Validation scans all flow command files matching pattern
    Given flow command files exist:
      | file                           | contains_violation |
      | flow-1-signal.md               | no                 |
      | flow-2-plan.md                 | no                 |
      | flow-3-build.md                | yes                |
    When pack-check validates the flow boundary rules
    Then the validation scans exactly 3 files matching "flow-*.md"
    And only "flow-3-build.md" produces a warning
