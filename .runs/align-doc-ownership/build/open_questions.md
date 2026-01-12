# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run align-doc-ownership

### Resolutions from User Guidance (Plan Questions)

The following Plan-phase questions have been resolved by detailed user guidance provided at Build initiation:

- A: Yes, pack-check boundary rules shall be implemented in Rust (tools/demoswarm-pack-check), not shell scripts. The existing pack-check infrastructure (mod.rs, contracts.rs, flow.rs) provides clear patterns for adding new checks. (resolves OQ-PLAN-001) [RESOLVED]

- A: The specific regex patterns for detecting skill plumbing in flow commands shall match: `demoswarm\.sh`, skill names (runs-derive, runs-index, openq-tools, secrets-tools, test-runner, auto-linter, policy-runner), and CLI flag patterns (--file, --prefix, --run-id, etc.). (resolves OQ-PLAN-002) [RESOLVED]

- A: Archive-over-delete applies only to entire files or sections. Inline CLI examples being replaced with skill references do not require archiving - the skill doc is the authoritative location and the examples were duplicates, not unique content. (resolves OQ-PLAN-003) [RESOLVED]

- A: "Brief inline example" = single-line invocation patterns acceptable (e.g., showing the command structure). "Excessive CLI duplication" = multi-line examples with all flags documented, or the same command pattern shown multiple times. (resolves OQ-PLAN-004) [RESOLVED]

### Questions That Would Change the Implementation

#### Category: Technical

- QID: OQ-BUILD-001
  - Q: Where in pack-check should the new boundary enforcement checks be added - as new CheckSpecs in flow.rs, or as a new boundary.rs module? [OPEN]
  - Suggested default: Add to flow.rs since the existing check_control_plane_agents (check 37) already validates enum consistency; boundary enforcement is logically similar. Create new check IDs in the 45-50 range.
  - Impact if different: A new module (boundary.rs) would be cleaner separation but adds more files; flow.rs consolidation keeps related checks together
  - Needs answer by: Before merge (affects code organization)
  - Evidence: tools/demoswarm-pack-check/src/checks/mod.rs -> flow checks (5, 11, 12, 13, 22, 25, 26, 27, 37, 43, 44)

- QID: OQ-BUILD-002
  - Q: Should the "skill plumbing in flows" check fail (hard error) or warn (soft error) when violations are found? [OPEN]
  - Suggested default: Fail (hard error) per REQ-001 "strictly scanned" and NFR-TEST-001 MET-3 "fail when violations are present"
  - Impact if different: Warn would allow gradual cleanup but would not enforce the boundary; Fail ensures immediate enforcement
  - Needs answer by: Before merge
  - Evidence: requirements.md -> REQ-001 (flow commands strictly scanned), NFR-TEST-001 MET-3 (fail when violations present)

- QID: OQ-BUILD-003
  - Q: The text "via runs-derive skill" in flow commands violates REQ-001/AC-1 (skill-name invocation). Should we replace with "computed mechanically (never estimates)" or a different phrase? [OPEN]
  - Suggested default: Replace with "computed mechanically (never estimates)" as this describes the what, not the how
  - Impact if different: Other phrasings might be clearer; the key is removing skill names from flow orchestration
  - Needs answer by: Before merge
  - Evidence: User guidance explicitly states this wording is non-compliant per REQ-001/AC-1

- QID: OQ-BUILD-004
  - Q: Flow step lists currently show "agent -> output_file.yaml" (e.g., "work-planner -> subtasks.yaml"). Per user guidance, these are drift magnets. Should we remove file paths entirely or keep them somewhere else? [OPEN]
  - Suggested default: Flow step lists show "agent + purpose" only (e.g., "work-planner: breaks ADR decisions into sequenced subtasks"). File-path specifics live in agent docs only.
  - Impact if different: Keeping file paths in flows provides quick reference but creates maintenance burden; agent docs already define their outputs
  - Needs answer by: Before merge
  - Evidence: User guidance states output filenames in flow step lists are drift magnets; agent docs own output definitions

#### Category: Product

- QID: OQ-BUILD-005
  - Q: The 55 agent docs need Skills sections when they use skills. Should we add Skills sections to ALL agents that invoke any skill, or only agents that invoke the 7 canonical skills (test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools)? [OPEN]
  - Suggested default: Only the 7 canonical skills require Skills section declaration. Standard tool usage (Read, Write, Bash, Glob, Grep) does not require declaration.
  - Impact if different: Broader definition would require auditing for any external tool usage; narrower keeps it focused on pack-specific skills
  - Needs answer by: Before merge
  - Evidence: CLAUDE.md -> Skills section lists exactly 7 skills; REQ-002/AC-3 specifies these skill names

### Assumptions Made to Proceed

- Assumption: The existing pack-check check IDs (1-44) are stable and new checks should use IDs 45+.
  - Rationale: mod.rs shows checks sorted by ID; adding new IDs at the end avoids renumbering existing checks.
  - Impact if wrong: Could conflict with other in-flight pack-check changes; would need ID coordination.
  - Linked question: OQ-BUILD-001

- Assumption: Check 37 (check_control_plane_agents) already validates canonical status/action enums in Machine Summary blocks; we do not need a separate enum check.
  - Rationale: flow.rs line 389-394 shows canon_status and canon_action regex matching for agents with Machine Summary.
  - Impact if wrong: May need additional enum check if existing check has gaps.
  - Linked question: null

- Assumption: The contracts.rs file is the canonical location for regex patterns and skill ownership lists; new boundary patterns should be added there.
  - Rationale: Existing patterns (bespoke_pipeline, index_upsert_cmd, secrets_cmd, openq_cmd) follow this pattern.
  - Impact if wrong: Patterns scattered across files would be harder to maintain.
  - Linked question: OQ-BUILD-002

- Assumption: Flow command files are reliably identified via `cx.inv.flow_cmd_files` (glob: `.claude/commands/flow-*.md`).
  - Rationale: flow.rs check functions iterate over flow_cmd_files consistently.
  - Impact if wrong: Some flow commands might be missed by boundary checks.
  - Linked question: null

- Assumption: Agent docs that "use skills" means agents that invoke `bash .claude/scripts/demoswarm.sh <skill-command>` or directly reference skill-specific CLI commands in their operational instructions.
  - Rationale: REQ-002/AC-3 explicitly lists the pattern and skill names.
  - Impact if wrong: May miss agents that use skills indirectly or through other invocation patterns.
  - Linked question: OQ-BUILD-005

### Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 3
route_to_agent: null
output_path: .runs/align-doc-ownership/build/open_questions.md
questions_added: 5
assumptions_added: 5
missing_required: []
blockers: []
concerns:
  - All new Build questions (OQ-BUILD-001 through OQ-BUILD-005) have defaults and do not block
  - Four Plan questions resolved based on user guidance - proceeding with stated defaults
  - OQ-BUILD-002 (fail vs warn) affects enforcement strictness; default is strict per requirements
  - OQ-BUILD-003 and OQ-BUILD-004 are wording/organization concerns from explicit user guidance
```
