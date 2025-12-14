Feature: Documentation Ownership Alignment
  As a pack maintainer
  I want clear ownership boundaries for documentation
  So that each layer (flow, agent, skill) has a single source of truth

  Background:
    Given the DemoSwarm pack repository exists
    And the repository follows the three-tier ownership model

  # REQ-001: Flow Command Boundary Enforcement

  @REQ-001 @smoke
  Scenario: Flow commands contain only orchestration content
    Given a flow command file exists at ".claude/commands/flow-1-signal.md"
    When I inspect the file contents
    Then the file does not contain the string "demoswarm.sh"
    And the file does not contain skill-name invocations like "runs-derive" or "runs-index"
    And the file references agents by name rather than implementation details

  @REQ-001 @edge
  Scenario: Flow commands avoid CLI flag syntax
    Given a flow command file exists at ".claude/commands/flow-3-build.md"
    When I inspect the file contents
    Then the file does not contain CLI flag patterns like "--file" or "--prefix" or "--run-id"
    And orchestration references are at the task level not implementation level

  @REQ-001 @error
  Scenario: Detection of skill plumbing in flow command triggers pack-check failure
    Given a flow command file contains "bash .claude/scripts/demoswarm.sh runs-derive"
    When pack-check boundary enforcement runs
    Then the check fails with exit code non-zero
    And the error message identifies the boundary violation

  # REQ-002: Agent Doc Consistency

  @REQ-002 @smoke
  Scenario: Agent docs use canonical status enum values
    Given an agent doc exists at ".claude/agents/requirements-author.md"
    When the agent doc contains a Machine Summary block
    Then the status field uses only values from the set "VERIFIED, UNVERIFIED, CANNOT_PROCEED"

  @REQ-002
  Scenario: Agent docs use canonical recommended_action enum values
    Given an agent doc exists at ".claude/agents/code-critic.md"
    When the agent doc contains a Machine Summary block
    Then the recommended_action field uses only values from the set "PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV"

  @REQ-002
  Scenario: Agent that invokes skills includes Skills section
    Given an agent doc exists at ".claude/agents/signal-cleanup.md"
    And the agent doc invokes "bash .claude/scripts/demoswarm.sh"
    When I inspect the agent doc structure
    Then the agent doc includes a "Skills" section listing the skills by canonical names

  @REQ-002
  Scenario: Agent docs with file-write rules use explicit format
    Given an agent doc exists at ".claude/agents/bdd-author.md"
    When the agent doc specifies output files
    Then the output specification follows the format "write exactly N files"
    And explicit output paths are listed
    And no contradictory output paths exist within the same agent doc

  @REQ-002 @edge
  Scenario: Agent docs reference skill docs for CLI details
    Given an agent doc exists at ".claude/agents/build-cleanup.md"
    And the agent doc uses CLI commands from the runs-derive skill
    When I inspect the CLI invocation sections
    Then the agent doc references "per runs-derive SKILL.md" or similar
    And the agent doc does not duplicate full CLI flag documentation

  @REQ-002 @error
  Scenario: Agent with invalid status enum fails pack-check
    Given an agent doc contains "status: COMPLETE" instead of canonical values
    When pack-check enum validation runs
    Then the check fails with exit code non-zero
    And the error identifies the non-canonical enum value

  # REQ-003: Skill Doc Ownership

  @REQ-003 @smoke
  Scenario: Skill doc contains complete CLI command reference
    Given a skill doc exists at ".claude/skills/runs-derive/SKILL.md"
    When I inspect the skill doc contents
    Then the skill doc contains CLI command documentation
    And the skill doc lists all supported flags with descriptions
    And the skill doc is the authoritative source for CLI syntax

  @REQ-003
  Scenario: Skill doc contains runnable examples
    Given a skill doc exists at ".claude/skills/runs-index/SKILL.md"
    When I inspect the examples section
    Then the skill doc contains at least one runnable example per major command

  @REQ-003 @edge
  Scenario: CLI details migrate from CLAUDE.md to skill docs
    Given CLAUDE.md Skills table contains an entry for "runs-derive"
    When I compare CLAUDE.md to the skill doc at ".claude/skills/runs-derive/SKILL.md"
    Then CLAUDE.md has at most 2 lines for the skill entry
    And detailed flag documentation exists only in the skill doc

  # REQ-004: CLAUDE.md Scope Normalization

  @REQ-004 @smoke
  Scenario: CLAUDE.md Skills table is summary-level only
    Given CLAUDE.md exists at the repository root
    When I inspect the Skills table section
    Then each skill entry is one line with format "command | purpose"
    And no entries contain flag details like "--file" or "--prefix"

  @REQ-004
  Scenario: CLAUDE.md does not duplicate skill doc content
    Given CLAUDE.md contains a Skills section
    When I compare CLAUDE.md to skill docs in ".claude/skills/"
    Then CLAUDE.md does not contain detailed CLI flag documentation
    And flag syntax from skill docs is not duplicated in CLAUDE.md

  @REQ-004
  Scenario: CLAUDE.md references skill docs for detailed usage
    Given CLAUDE.md contains mentions of demoswarm CLI commands
    When I inspect the skill references
    Then CLAUDE.md includes references like "See `.claude/skills/runs-derive/SKILL.md` for complete reference"

  @REQ-004 @error
  Scenario: Detection of duplicated CLI flags in CLAUDE.md triggers doc-drift failure
    Given CLAUDE.md contains "--file X --prefix Y" flag documentation
    And the same flag documentation exists in a skill doc
    When doc-drift check runs
    Then the check fails with exit code non-zero
    And the error identifies the duplication location

  # REQ-005: Subtask Partitioning

  @REQ-005 @smoke
  Scenario: ST-001 covers Flow 1 (Signal) documentation
    Given the subtask partitioning plan exists
    When I inspect the ST-001 scope
    Then ST-001 touches files matching ".claude/agents/*signal*.md"
    And ST-001 touches files matching ".claude/commands/flow-1-signal.md"

  @REQ-005
  Scenario: ST-002 covers Flow 2 (Plan) documentation
    Given the subtask partitioning plan exists
    When I inspect the ST-002 scope
    Then ST-002 touches files matching ".claude/agents/*plan*.md"
    And ST-002 touches files matching ".claude/commands/flow-2-plan.md"

  @REQ-005
  Scenario: ST-003 covers Flow 3 (Build) documentation
    Given the subtask partitioning plan exists
    When I inspect the ST-003 scope
    Then ST-003 touches files matching ".claude/agents/*build*.md"
    And ST-003 touches files matching ".claude/commands/flow-3-build.md"

  @REQ-005
  Scenario: ST-004 covers Flow 4 plus cross-cutting concerns
    Given the subtask partitioning plan exists
    When I inspect the ST-004 scope
    Then ST-004 touches files matching ".claude/agents/*gate*.md"
    And ST-004 touches files matching ".claude/commands/flow-4-gate.md"
    And ST-004 includes pack-check boundary enforcement additions
    And ST-004 includes CLAUDE.md normalization

  @REQ-005
  Scenario: ST-005 covers Flow 5 (Deploy) documentation
    Given the subtask partitioning plan exists
    When I inspect the ST-005 scope
    Then ST-005 touches files matching ".claude/agents/*deploy*.md"
    And ST-005 touches files matching ".claude/commands/flow-5-deploy.md"

  @REQ-005
  Scenario: ST-006 covers Flow 6 (Wisdom) plus validation
    Given the subtask partitioning plan exists
    When I inspect the ST-006 scope
    Then ST-006 touches files matching ".claude/agents/*wisdom*.md"
    And ST-006 touches files matching ".claude/commands/flow-6-wisdom.md"
    And ST-006 includes the validation run execution

  @REQ-005 @edge
  Scenario: Subtasks have distinct touches patterns to minimize conflicts
    Given the subtask partitioning plan exists
    When I compare the touches patterns of ST-001 through ST-006
    Then no two subtasks have overlapping primary file patterns
    And each subtask has a distinct ownership boundary

  # REQ-006: Validation Run Recording

  @REQ-006 @smoke
  Scenario: Validation run is recorded after alignment completion
    Given alignment changes are complete for all subtasks
    And pack-check passes with exit code 0
    And doc-drift passes with exit code 0
    When Toy Run A is executed through flows 1-4
    And Toy Run B is executed through flows 1-4
    Then a validation log entry is recorded in "docs/maintainers/validation-log.md"

  @REQ-006
  Scenario: Validation log entry includes required fields
    Given a validation run has been executed
    When I inspect the validation log entry
    Then the entry includes the date of execution
    And the entry includes run IDs for Toy Run A and Toy Run B
    And the entry includes flows executed (flows 1-4)
    And the entry includes pass/fail status
    And the entry includes any relevant notes

  @REQ-006 @error
  Scenario: Validation run not recorded if pack-check fails
    Given alignment changes are partially complete
    And pack-check fails due to boundary violations
    When I attempt to record a validation run
    Then the validation run is not recorded
    And an error indicates pack-check must pass first

  @REQ-006 @error
  Scenario: Validation run not recorded if doc-drift fails
    Given alignment changes are complete
    And pack-check passes
    But doc-drift fails due to CLI duplication
    When I attempt to record a validation run
    Then the validation run is not recorded
    And an error indicates doc-drift must pass first

  # REQ-007: Archive-Over-Delete Pattern

  @REQ-007 @smoke
  Scenario: Moved content retains reference to new location
    Given documentation content exists in CLAUDE.md Skills section
    When the content is moved to a skill doc at ".claude/skills/runs-derive/SKILL.md"
    Then the original location in CLAUDE.md contains a reference to the new location
    Or the moved content is archived in "docs/archive/"

  @REQ-007
  Scenario: Removed content is archived not deleted
    Given documentation content is identified for removal
    When the content is removed from its source file
    Then the content is preserved in "docs/archive/" directory
    Or the content is preserved as a comment block in the source

  @REQ-007 @edge
  Scenario: Content moves are documented in PR description
    Given documentation content is moved from one file to another
    When the PR is created for the alignment changes
    Then the PR description documents the content move
    And the PR description specifies the archive location or new location

  @REQ-007 @error
  Scenario: Direct deletion without archive is flagged in review
    Given documentation content is removed from a file
    And no archive or reference is created
    When PR review is conducted
    Then the deletion is flagged as a potential issue
    And the reviewer requests an archive or reference
