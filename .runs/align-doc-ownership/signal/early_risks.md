# Early Risks

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
blockers: []
missing_required: []

## Risks

- RSK-001 [HIGH] [OPS]
  - What: Merge conflicts across 6 parallel subtasks (ST-001 through ST-006)
  - Trigger: Multiple subtasks touch overlapping agent or skill doc files simultaneously; concurrent PRs create conflicts
  - Mitigation hint: Execute subtasks sequentially or ensure distinct file ownership per subtask; each subtask has tight `touches` pattern per REQ-005 AC-7
  - Evidence: REQ-005 (Subtask Partitioning), problem_statement.md (Constraints: tight touches patterns), context_brief.md (Scope creep risk)

- RSK-002 [MEDIUM] [OPS]
  - What: ST-004 scope concentration makes it heavier than other subtasks
  - Trigger: ST-004 owns Flow 4 (Gate), cross-cutting boundary enforcement (pack-check additions), and CLAUDE.md normalization; more effort than ST-001 through ST-003
  - Mitigation hint: Plan ST-004 with explicit buffer time; consider splitting CLAUDE.md work from pack-check additions if timeline pressure exists
  - Evidence: problem_statement.md (Concerns), requirements.md (REQ-005 AC-4), context_brief.md (Cross-cutting enforcement concentration)

- RSK-003 [MEDIUM] [OPS]
  - What: pack-check boundary enforcement may require Rust development
  - Trigger: Adding new drift checks (skill plumbing in flows, enum inconsistencies, missing Skills sections) to pack-check requires modifying `tools/demoswarm-pack-check/`
  - Mitigation hint: If Rust development is complex, fall back to shell script implementation; open_questions.md (OQ-SIG-006) suggests Rust is acceptable if straightforward
  - Evidence: OQ-SIG-006 (pack-check rule additions), problem_statement.md (Constraints: pack-check must pass), context_brief.md (pack-check extension risk)

- RSK-004 [MEDIUM] [DATA]
  - What: False positives in pack-check boundary rules
  - Trigger: New boundary enforcement rules may incorrectly flag legitimate patterns (e.g., flow commands that mention skill names for reference, not invocation)
  - Mitigation hint: Define precise patterns for violations (e.g., `bash .claude/scripts/demoswarm.sh` + skill-name is violation; mentioning skill name in prose is not); test with negative cases per NFR-TEST-001 MET-3
  - Evidence: REQ-001 (AC-1, AC-2), NFR-TEST-001 (MET-3: negative test requirement)

- RSK-005 [LOW] [OPS]
  - What: Validation run dependency delays completion
  - Trigger: Toy Run A/B (flows 1-4) must succeed before work is recorded as complete; any failure in flows 1-4 blocks validation log entry
  - Mitigation hint: Run Toy Run A/B incrementally during development; do not wait until all subtasks are complete to start validation
  - Evidence: REQ-006 (Validation Run Recording), problem_statement.md (Constraints: validation run required), context_brief.md (Validation dependency)

- RSK-006 [LOW] [OPS]
  - What: NFR-MAINT-001 MET-3 relies on maintainer survey which is non-deterministic
  - Trigger: Verifying "maintainers can determine authoritative location without consulting multiple files" requires human judgment
  - Mitigation hint: Use manual inspection by 2+ reviewers as acceptable alternative; automated doc-drift check (MET-2) provides partial assurance
  - Evidence: requirements.md (NFR-MAINT-001 MET-3), requirements_critique.md (Concerns), verification_notes.md (Note on MET-3)

## Risk Summary (derived)

- Critical: 0
- High: 1
- Medium: 3
- Low: 2

## Notes

- **Security risks not applicable**: This work is documentation alignment; no code execution paths change, no secrets handling changes, no external integrations added. Security risks are inherently low.
- **Compliance risks not applicable**: No regulatory or policy concerns with documentation restructuring.
- **Performance risks not applicable**: No runtime behavior changes.
- **Intentionally excluded**: Risks related to upstream repo export (out of scope per problem_statement.md Non-Goals), risks related to changing agent/skill behavior (out of scope per Non-Goals).
