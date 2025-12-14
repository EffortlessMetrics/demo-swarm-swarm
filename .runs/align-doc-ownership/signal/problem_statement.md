# Problem Statement

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - ST-004 carries heavier scope (Gate + cross-cutting enforcement + CLAUDE.md cleanup); may require more effort than other subtasks
  - 55 agent files need consistency auditing; risk of inadvertent merge conflicts if subtasks overlap
  - pack-check boundary enforcement rules do not exist yet; Rust development may be required

confidence: High

## The Problem

The DemoSwarm pack documentation currently suffers from blurred ownership boundaries between its three documentation tiers: flow commands, agent docs, and skill docs. Flow commands may contain skill plumbing (e.g., direct `bash demoswarm.sh` invocations) that are not directly tied to flow-routing, agent docs may inconsistently apply status/action enums, and CLAUDE.md may duplicate deep reference material that belongs in skill docs. This structural coupling makes the pack harder to maintain, increases the risk of drift between layers, and obscures where authoritative information lives.

The desired state is a clear separation of concerns: flow commands own orchestration and routing (agent lists, station sequencing, routing skills), agent docs own operational detail (what each agent does, its inputs/outputs, its Machine Summary contract), skill docs own CLI truth (command syntax, flags, examples), and CLAUDE.md serves as a table of contents and quick-start guide rather than a comprehensive reference. Achieving this requires auditing 55 agent files, 6 flow commands, 7 skill docs, and CLAUDE.md, then enforcing boundaries via pack-check and doc-drift validation.

## Who Is Affected

- **Pack maintainers**: Primary stakeholders. Blurred boundaries increase cognitive load when updating docs and risk introducing contradictions.
- **Agent implementations**: Agents that reference skill tooling may have stale or duplicated invocation examples if skill docs are the source of truth.
- **Flow orchestrators**: Flow commands that embed skill plumbing (other than routing-related skills) become coupled to implementation details they should not own.
- **New contributors / onboarding**: Unclear ownership makes it harder to understand where to find or update authoritative information.

## Constraints

- `pack-check` must pass after changes, including any new boundary-enforcement drift checks added as part of this work.
- `doc-drift` (via `scripts/check-doc-drift.sh`) must pass.
- All 55 agent docs must be consistent on: status/recommended_action enums, "write exactly N files" rules, Skills section presence with correct naming, and no contradictory output paths.
- Flow commands must contain no skill plumbing (no direct `runs-derive`, `bash demoswarm.sh ...` invocations) and no agent-internal implementation detail.
- A successful validation run (Toy Run A/B through flows 1-4) must be recorded in `docs/maintainers/validation-log.md`.
- Subtask partitioning (ST-001 through ST-006) must preserve tight `touches` patterns to minimize merge conflict risk.
- Prior alignment commits (be0c81a, 186ea53) established trajectory; this work continues that direction, not contradicts it.
- Archive-over-delete pattern (established by PR #48) should be followed if content is moved or removed.

## Non-Goals

- Changing the functional behavior of any flow, agent, or skill.
- Modifying the pack-check or demoswarm CLI Rust source code beyond what is necessary for boundary enforcement checks.
- Redesigning the three-tier documentation architecture (flow -> agent -> skill); the architecture is assumed correct, only its enforcement is incomplete.
- Upstream repo export or synchronization; this work targets the swarm repo only.
- Addressing any issues outside the 55 agents, 6 flows, 7 skills, and CLAUDE.md scope.
- Adding new features to the pack.

## Success Looks Like

- `pack-check` passes, including any new drift checks for boundary violations.
- `doc-drift` (`scripts/check-doc-drift.sh`) passes.
- Every agent doc uses consistent status/recommended_action enums, states file-write rules clearly, includes a Skills section with correct skill naming, and has no contradictory output paths.
- Every flow command contains zero skill plumbing (no `bash demoswarm.sh`, no `runs-derive` invocations, no CLI flag details).
- CLAUDE.md is a table of contents and quick-start guide; deep CLI reference lives only in skill docs.
- A Toy Run A/B (flows 1-4 end-to-end) succeeds and is recorded in `docs/maintainers/validation-log.md`.
- No regressions: existing flows continue to function; no agent behavior changes; no secrets exposed.

## Known Context

- **Flow commands**: `.claude/commands/flow-{1-6}-*.md` (6 files)
- **Agent docs**: `.claude/agents/*.md` (55 files)
- **Skill docs**: `.claude/skills/*/SKILL.md` (7 files: runs-derive, runs-index, openq-tools, secrets-tools, test-runner, auto-linter, policy-runner)
- **Pack-level**: `CLAUDE.md`, `docs/maintainers/validation-log.md`
- **Validation tooling**: `scripts/check-doc-drift.sh`, `tools/demoswarm-pack-check/`
- **Prior art**: Commits be0c81a, 186ea53 (recent consistency improvements); PR #48 (archive-over-delete pattern); commit b759005 (skill ownership established); commit 963eb64 (agent status sections standardized)

## Assumptions Made to Proceed

- **ASM-1**: The three-tier ownership model (flow commands -> agents -> skills) is the intended architecture.
  - *If wrong*: Entire framing changes; would need to define a different ownership model.

- **ASM-2**: Cleanup agents legitimately need operational detail inline (they execute the work), but should reference skills for CLI truth rather than duplicating examples.
  - *If wrong*: Agent docs may need to embed full CLI examples, increasing duplication tolerance.

- **ASM-3**: CLAUDE.md is meant to be entry-point level and should not duplicate CLI flag details from skill docs.
  - *If wrong*: CLAUDE.md scope expands, skill docs become secondary.

- **ASM-4**: pack-check can be extended with boundary-enforcement drift checks without major refactoring.
  - *If wrong*: Enforcement may need to be deferred or implemented via shell scripts instead.

- **ASM-5**: The subtask partitioning (ST-001 through ST-006 by flow) is the intended decomposition.
  - *If wrong*: Work plan may need restructuring.

- **ASM-6**: No external dependencies or blockers prevent this alignment work.
  - *If wrong*: Scope may need to shrink or defer certain areas.

## Questions / Clarifications Needed

- **Q: Should cleanup agents reference skill docs (e.g., "use runs-derive skill per SKILL.md") rather than embed CLI examples inline?**
  - Suggested default: Yes, agents should reference skills and not duplicate CLI examples. This reduces drift risk.

- **Q: What specific boundary violations should pack-check detect?**
  - Suggested default: (1) Flow commands containing `demoswarm.sh` or skill-name invocations, (2) Agent docs with status/action enums not in the canonical set, (3) Missing Skills sections in agents that use skills.

- **Q: Is Rust development in `tools/demoswarm-pack-check/` acceptable for this run, or should boundary checks use shell scripts?**
  - Suggested default: Rust development is acceptable if the checks are straightforward; fall back to shell scripts if complexity is high.

- **Q: Should the validation run (Toy Run A/B) be a separate subtask or part of ST-006?**
  - Suggested default: Include validation as a final step in ST-006 (Flow 6 / Wisdom), since it validates the entire pass.
