# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run align-doc-ownership

### Resolutions from User Guidance (Signal Questions)

The following Signal-phase questions have been resolved by detailed user guidance provided at Plan initiation:

- A: Agents should reference skill docs (e.g., "per runs-derive SKILL.md") rather than embed CLI examples inline. Flow commands own orchestration only; agents own operational detail but reference skills for CLI truth; skill docs own CLI flag truth. (resolves OQ-SIG-001) [RESOLVED]

- A: CLAUDE.md stays summary-level (one-line per command in Skills table). It is entry point + pointers only. Detailed CLI reference lives exclusively in skill docs. (resolves OQ-SIG-002) [RESOLVED]

- A: Flow commands must contain zero skill plumbing. No mention of demoswarm.sh, skill names (runs-derive, runs-index, etc.), or CLI flags. Flow commands are orchestration only - they route to agents, which in turn use skills. (resolves OQ-SIG-003) [RESOLVED]

- A: Validation run (Toy Run A/B) is part of ST-006, not a separate subtask. ST-006 covers Flow 6 (Wisdom) documentation plus the validation run. (resolves OQ-SIG-004) [RESOLVED]

- A: Agents should reference skills for CLI truth, not embed full CLI examples. Brief inline examples are acceptable only when the skill doc does not yet document that pattern. Otherwise, use "per <skill> SKILL.md" references. (resolves OQ-SIG-005) [RESOLVED]

- A: Yes, add pack-check rules for boundary enforcement in ST-004. Rules should detect: (1) skill plumbing in flow commands, (2) enum inconsistencies in agent docs, (3) missing Skills sections in agents that use skills. (resolves OQ-SIG-006) [RESOLVED]

### Questions That Would Change the Spec

#### Category: Technical

- QID: OQ-PLAN-001
  - Q: Should pack-check boundary rules be implemented in Rust (tools/demoswarm-pack-check) or as shell script checks (scripts/check-*.sh)? [OPEN]
  - Suggested default: Rust implementation in pack-check, following the established pattern for structural validation
  - Impact if different: Shell scripts would be faster to implement but harder to maintain; Rust keeps all pack validation in one tool
  - Needs answer by: Flow 3 (Build)
  - Evidence: requirements.md -> NFR-TEST-001 MET-3 (new pack-check rules), ASM-004 (pack-check extensibility assumed)

- QID: OQ-PLAN-002
  - Q: What specific regex patterns should pack-check use to detect skill plumbing in flow commands? [OPEN]
  - Suggested default: Match patterns: `demoswarm\.sh`, `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`, `test-runner`, `auto-linter`, `policy-runner`, plus CLI flag patterns `--file`, `--prefix`, `--run-id`
  - Impact if different: Broader patterns catch more drift but may false-positive on legitimate documentation; narrower patterns may miss edge cases
  - Needs answer by: Flow 3 (Build)
  - Evidence: REQ-001 AC-1, AC-2 (defines what constitutes skill plumbing in flows)

- QID: OQ-PLAN-003
  - Q: Should the archive-over-delete pattern apply to inline CLI examples being removed from agent docs, or only to entire files/sections? [OPEN]
  - Suggested default: Only entire files/sections need archiving. Inline CLI examples can be replaced with skill references without archiving the original examples.
  - Impact if different: If all CLI examples need archiving, creates significant archive overhead; if not, we lose the audit trail of what was removed
  - Needs answer by: Flow 3 (Build)
  - Evidence: REQ-007 (archive-over-delete pattern), REQ-002 AC-5 (agents reference skills)

#### Category: Product

- QID: OQ-PLAN-004
  - Q: What is the definition of "brief inline example" that is acceptable in agent docs vs "excessive CLI duplication" that must be moved to skill docs? [OPEN]
  - Suggested default: Acceptable: single-line invocations showing the command pattern (e.g., `bash .claude/scripts/demoswarm.sh count pattern ...`). Excessive: multi-line examples with all flags documented, or the same command shown in multiple places within one agent doc.
  - Impact if different: Stricter definition removes more content from agents (more DRY but more indirection); looser definition keeps agents more self-contained
  - Needs answer by: Flow 3 (Build)
  - Evidence: OQ-SIG-005 resolution (reference skills, do not duplicate)

### Assumptions Made to Proceed

- Assumption: The subtask partitioning (ST-001 through ST-006) as defined in requirements.md is final and will not change during Plan phase.
  - Rationale: User provided explicit subtask breakdown in plan direction; requirements.md REQ-005 codifies this.
  - Impact if wrong: Work plan and ADR would need restructuring.
  - Linked question: null

- Assumption: pack-check can be extended without major refactoring because boundary checks are similar to existing structural validation.
  - Rationale: Stated in ASM-004; pack-check already validates pack structure using pattern matching.
  - Impact if wrong: May need shell script fallback for boundary checks, affecting OQ-PLAN-001.
  - Linked question: OQ-PLAN-001

- Assumption: The flow code for Plan in QID format is "PLN" (not "PLAN" or "PL").
  - Rationale: Per openq-tools SKILL.md, flow codes are 3-letter abbreviations: SIG, PLN, BLD, GAT, DEP, WIS.
  - Impact if wrong: QID format would be inconsistent with skill documentation.
  - Linked question: null

- Assumption: All six Signal open questions are considered resolved by user guidance provided at Plan initiation.
  - Rationale: User provided detailed direction addressing each question's underlying concern.
  - Impact if wrong: Some Signal questions may need reopening if guidance was misinterpreted.
  - Linked question: null

### Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 2
route_to_agent: null
output_path: .runs/align-doc-ownership/plan/open_questions.md
questions_added: 4
assumptions_added: 4
missing_required: []
blockers: []
concerns:
  - All new Plan questions (OQ-PLAN-001 through OQ-PLAN-004) have defaults and do not block
  - Six Signal questions resolved based on user guidance - verify interpretation is correct
  - OQ-PLAN-001 (Rust vs shell) may need user confirmation before Build phase
```
