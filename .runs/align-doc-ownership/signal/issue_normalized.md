# Normalized Issue

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: problem-framer
route_to_flow: 1
blockers: []
missing_required: []
notes:
  - Signal is already structured with clear objective/done-when statements
  - Proposed implementation includes 6 subtasks (ST-001 through ST-006)
  - No sensitive content detected

## Summary

Internal refactoring request to normalize documentation ownership boundaries across the DemoSwarm pack. The work aims to ensure clear separation of concerns: flow commands own orchestration/routing, agent docs own operational detail, skill docs own CLI truth, and CLAUDE.md serves as a "table of contents." Success requires pack-check and doc-drift to pass, plus a recorded validation run.

## Signal Type
- request_type: refactor
- source_type: other (internal work item)
- links:
  - none

## Observed vs Expected
- observed: Documentation responsibilities are blurred; flows may contain skill plumbing (e.g., "bash demoswarm.sh" invocations); agent docs may have inconsistent status/action enums; CLAUDE.md may duplicate deep reference material.
- expected: Clear ownership boundaries where each layer documents only its contract, not implementation details from other layers.

## Impact
- affected_users: Pack maintainers, agents, flow orchestrators
- severity: medium (technical debt / maintainability)
- frequency: always (structural issue)
- environment: all (documentation structure)

## Components Mentioned
- systems/services:
  - pack-check (validation tool)
  - doc-drift (scripts/check-doc-drift.sh)
- endpoints/paths:
  - .claude/commands/flow-*.md (6 flow commands)
  - .claude/agents/*.md (55 agent docs)
  - .claude/skills/*/SKILL.md (7 skill docs)
  - CLAUDE.md
  - docs/maintainers/validation-log.md
- files/modules:
  - ST-001: Flow 1 docs
  - ST-002: Flow 2 docs
  - ST-003: Flow 3 docs
  - ST-004: Flow 4 docs + cross-cutting enforcement + CLAUDE.md cleanup
  - ST-005: Flow 5 docs
  - ST-006: Flow 6 docs

## Constraints / Non-negotiables
- pack-check must pass (including new drift checks for boundary violations)
- doc-drift must pass
- All agents must be consistent on:
  - status/recommended_action enums
  - "write exactly N files" rules
  - "Skills" section presence with correct skill naming
  - no contradictory output paths
- Flows must contain no skill plumbing ("runs-derive", "bash demoswarm.sh ...") and no agent-internal implementation chatter
- A validation run (Toy Run A/B) must succeed and be recorded in docs/maintainers/validation-log.md
- Each subtask has tight `touches` patterns to avoid merge conflicts
- unknowns:
  - Exact new pack-check rules to be added for boundary enforcement
  - Whether existing flows have significant skill plumbing to remove

## Evidence (bounded)

Proposed subtask structure from signal:
> - ST-001: Flow 1 (Signal) docs
> - ST-002: Flow 2 (Plan) docs
> - ST-003: Flow 3 (Build) docs
> - ST-004: Flow 4 (Gate) docs + cross-cutting enforcement + CLAUDE.md cleanup
> - ST-005: Flow 5 (Deploy) docs
> - ST-006: Flow 6 (Wisdom) docs

Definition of Done criteria (from signal):
> - pack-check passes (including new drift checks)
> - doc-drift passes
> - agents consistent on enums, file rules, Skills sections
> - flows contain no skill plumbing
> - validation run recorded in validation-log.md
