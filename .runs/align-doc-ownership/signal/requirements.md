# Requirements

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - ST-004 carries heavier scope than other subtasks; may require proportionally more effort
  - 55 agent files to audit increases risk of merge conflicts if subtasks overlap
  - Open questions (OQ-SIG-001 through OQ-SIG-006) have defaults but remain open; Plan phase should confirm
  - NFR-MAINT-001 MET-3 relies on maintainer survey which is non-deterministic; manual inspection is alternative

## Functional Requirements

### REQ-001: Flow Command Boundary Enforcement
The system shall ensure that all flow command files (`.claude/commands/flow-*.md`) contain only orchestration and routing content, with no skill plumbing or CLI invocation details.
- AC-1: No flow command file contains the string `demoswarm.sh` or direct skill-name invocations (e.g., `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`).
- AC-2: No flow command file contains CLI flag syntax (e.g., `--file`, `--prefix`, `--run-id`).
- AC-3: Flow commands reference agents by name and routing, not by implementation details of how agents invoke skills.

*Source: problem_statement.md (Constraints: "Flow commands must contain no skill plumbing"), issue_normalized.md (Constraints: "Flows must contain no skill plumbing").*

### REQ-002: Agent Doc Consistency
The system shall ensure that all agent documentation files (`.claude/agents/*.md`) use consistent structure and terminology.
- AC-1: Every agent doc uses only the canonical status enum values: `VERIFIED`, `UNVERIFIED`, `CANNOT_PROCEED`.
- AC-2: Every agent doc uses only the canonical recommended_action enum values: `PROCEED`, `RERUN`, `BOUNCE`, `ESCALATE`, `FIX_ENV`.
- AC-3: Every agent doc that invokes `bash .claude/scripts/demoswarm.sh` or references skill-specific CLI commands (runs-derive, runs-index, openq-tools, secrets-tools, test-runner, auto-linter, policy-runner) includes a "Skills" section listing the skills by their canonical names.
- AC-4: Every agent doc with file-write rules uses the format "write exactly N files" with explicit output paths, and no contradictory output paths exist within a single agent doc.
- AC-5: Agent docs that invoke CLI commands reference the skill doc (e.g., "per runs-derive SKILL.md") rather than duplicating full CLI examples with flags.

*Source: problem_statement.md (Constraints: "All 55 agent docs must be consistent on: status/recommended_action enums, 'write exactly N files' rules, Skills section presence"), open_questions.md (OQ-SIG-001, OQ-SIG-005).*

### REQ-003: Skill Doc Ownership
The system shall ensure that skill documentation files (`.claude/skills/*/SKILL.md`) serve as the single source of truth for CLI syntax, flags, and invocation examples.
- AC-1: Each skill doc contains the complete CLI command reference including all supported flags and their descriptions.
- AC-2: Each skill doc contains at least one runnable example per major command.
- AC-3: CLI invocation details with flags (e.g., `--file`, `--prefix`, `--run-id`) that previously appeared in CLAUDE.md or agent docs are present in the corresponding skill doc; CLAUDE.md Skills table has at most 2 lines per skill (command | purpose).

*Source: problem_statement.md (The Problem: "skill docs own CLI truth"), issue_normalized.md (Components: 7 skill docs).*

### REQ-004: CLAUDE.md Scope Normalization
The system shall ensure that CLAUDE.md serves as a table of contents and quick-start guide, not a comprehensive CLI reference.
- AC-1: CLAUDE.md Skills table contains only summary-level entries (one line per command, no flag details).
- AC-2: CLAUDE.md does not duplicate detailed CLI flag documentation that exists in skill docs.
- AC-3: CLAUDE.md links or references skill docs for detailed usage (e.g., "See `.claude/skills/runs-derive/SKILL.md` for complete reference").

*Source: problem_statement.md (Success Looks Like: "CLAUDE.md is a table of contents and quick-start guide; deep CLI reference lives only in skill docs"), open_questions.md (OQ-SIG-002).*

### REQ-005: Subtask Partitioning by Flow
The system shall organize the alignment work into six subtasks (ST-001 through ST-006), each scoped to a single flow's documentation set, with ST-004 additionally handling cross-cutting enforcement and CLAUDE.md cleanup.
- AC-1: ST-001 covers Flow 1 (Signal) documentation: agents, flow command, and related files.
- AC-2: ST-002 covers Flow 2 (Plan) documentation: agents, flow command, and related files.
- AC-3: ST-003 covers Flow 3 (Build) documentation: agents, flow command, and related files.
- AC-4: ST-004 covers Flow 4 (Gate) documentation, cross-cutting boundary enforcement rules (pack-check additions), and CLAUDE.md normalization.
- AC-5: ST-005 covers Flow 5 (Deploy) documentation: agents, flow command, and related files.
- AC-6: ST-006 covers Flow 6 (Wisdom) documentation: agents, flow command, and related files, plus the validation run.
- AC-7: Each subtask has a distinct `touches` pattern to minimize merge conflict risk.

*Source: problem_statement.md (Constraints: "Subtask partitioning (ST-001 through ST-006) must preserve tight touches patterns"), issue_normalized.md (Components Mentioned).*

### REQ-006: Validation Run Recording
The system shall record a successful validation run (Toy Run A/B through flows 1-4) in `docs/maintainers/validation-log.md` upon completion of alignment work.
- AC-1: A Toy Run A and Toy Run B are executed through flows 1-4 after alignment changes are complete.
- AC-2: The validation log entry includes: date, run IDs, flows executed, pass/fail status, and any notes.
- AC-3: Both pack-check and doc-drift (`scripts/check-doc-drift.sh`) pass before the validation run is recorded.

*Source: problem_statement.md (Constraints: "A successful validation run (Toy Run A/B through flows 1-4) must be recorded"), issue_normalized.md (Constraints: "A validation run must succeed and be recorded").*

### REQ-007: Archive-Over-Delete Pattern
The system shall follow the archive-over-delete pattern (established by PR #48) when content is moved or removed during alignment work.
- AC-1: When documentation content is moved from one file to another, the original location is either archived or contains a reference to the new location.
- AC-2: When documentation content is removed, it is archived (e.g., in a `docs/archive/` directory or as a comment block) rather than permanently deleted.
- AC-3: The PR description or commit message documents any content moves or removals with their archive locations.

*Source: problem_statement.md (Constraints: "Archive-over-delete pattern (established by PR #48) should be followed if content is moved or removed").*

## Non-Functional Requirements

### Domain Notes
This run declares three additional NFR domains beyond the canonical set (SEC, PERF, REL, OPS, COMP):
- **MAINT**: Documentation maintainability and ownership clarity
- **TEST**: Validation tooling and automated verification
- **REGR**: Regression prevention and functional stability

These domains are appropriate for a documentation-alignment run where security, performance, and compliance are not primary concerns.

### NFR-MAINT-001: Documentation Maintainability
The system shall establish clear ownership boundaries to reduce cognitive load and drift risk during future maintenance.
- MET-1: After alignment, each documentation layer (flow, agent, skill) has a single source of truth for its domain, verifiable by manual inspection during PR review.
- MET-2: No duplicate CLI flag documentation exists between CLAUDE.md and skill docs (verified by doc-drift check in CI).
- MET-3: Pack maintainers can determine the authoritative location for any documentation concern without consulting multiple files (verified by onboarding feedback or maintainer survey).

### NFR-TEST-001: Validation Tooling Compliance
The system shall pass all automated validation checks after alignment work is complete.
- MET-1: `pack-check` passes with exit code 0, including any new boundary-enforcement rules added (verified in CI gate).
- MET-2: `doc-drift` (`scripts/check-doc-drift.sh`) passes with exit code 0 (verified in CI gate).
- MET-3: New pack-check rules detect boundary violations (skill plumbing in flows, enum inconsistencies) and fail when violations are present (verified by negative test).

### NFR-REGR-001: No Functional Regression
The system shall preserve all existing flow, agent, and skill behavior after alignment work.
- MET-1: Toy Run A/B (flows 1-4) completes successfully with no errors attributable to alignment changes (verified by validation run).
- MET-2: No agent output formats, file paths, or machine-parseable contracts change (verified by diff review during PR).
- MET-3: No secrets are exposed in committed artifacts (verified by secrets-sanitizer in gate flow).

## Assumptions Made
- **ASM-001**: The three-tier ownership model (flow commands -> agents -> skills) is the intended architecture. (why: Recent commits be0c81a, 186ea53 show alignment work toward this structure; CLAUDE.md describes this hierarchy)
  - Impact if wrong: The entire boundary definition and REQ-001 through REQ-004 would need rethinking.
- **ASM-002**: Cleanup agents legitimately need operational detail inline (they execute work, not just orchestrate), but should reference skill docs for CLI truth rather than duplicating full examples. (why: Cleanup agents perform counts, derive summaries, write receipts - they need to know tool invocation patterns)
  - Impact if wrong: REQ-002 AC-5 would need revision; agents might embed full CLI examples.
- **ASM-003**: CLAUDE.md is meant to be entry-point level, not deep reference. (why: CLAUDE.md states it is "repo-level policy + shared contracts" and the Skills section currently has one-line entries)
  - Impact if wrong: REQ-004 would be invalid; CLAUDE.md would expand to serve as primary reference.
- **ASM-004**: pack-check can be extended with boundary-enforcement drift checks without major refactoring. (why: pack-check already validates pack structure; boundary checks are similar pattern)
  - Impact if wrong: NFR-TEST-001 MET-3 may need to use shell scripts instead of Rust.
- **ASM-005**: The subtask partitioning (ST-001 through ST-006 by flow) is the intended decomposition. (why: Stated in problem_statement.md and issue_normalized.md)
  - Impact if wrong: REQ-005 would need restructuring.
- **ASM-006**: This run continues alignment work from prior commits (be0c81a, 186ea53), not a new direction. (why: Explicit mention in problem_statement.md Known Context)
  - Impact if wrong: We might contradict prior architectural decisions.

## Questions for Humans
- Q: Should cleanup agents reference skill docs (e.g., "use runs-derive skill per SKILL.md") rather than embed CLI examples inline? Suggested default: Yes, agents should reference skills and not duplicate CLI examples. Impact if different: Agents would need to maintain their own CLI examples, increasing drift risk (linked: OQ-SIG-001).
- Q: What specific boundary violations should pack-check detect? Suggested default: (1) Flow commands containing `demoswarm.sh` or skill-name invocations, (2) Agent docs with status/action enums not in the canonical set, (3) Missing Skills sections in agents that use skills. Impact if different: Manual review burden increases if not automated.
- Q: Is Rust development in `tools/demoswarm-pack-check/` acceptable for this run, or should boundary checks use shell scripts? Suggested default: Rust development is acceptable if checks are straightforward. Impact if different: Shell script fallback may be less maintainable but faster to implement.
- Q: Should the validation run (Toy Run A/B) be a separate subtask or part of ST-006? Suggested default: Include validation as a final step in ST-006. Impact if different: If separate, adds coordination overhead; if in ST-006, ties Wisdom flow to validation completion.
