# Early Risks

## Risks

- RSK-001 [MEDIUM] [COMPLIANCE]
  - What: Flow count discrepancy may cause pack-check validation to enforce stale "six flow" constraints
  - Trigger: If pack-check derives policy from incorrect documentation (README vs CLAUDE.md) rather than command file enumeration
  - Mitigation hint: Verify pack-check wisdom.rs checks use command file enumeration, not documentation parsing; update any stale checks
  - Evidence: REQ-001, REQ-004; problem_statement.md L61; CLAUDE.md states 7 flows but README states "six flows"

- RSK-002 [MEDIUM] [DATA]
  - What: Test count claims may drift again after documentation update if no automated reconciliation exists
  - Trigger: When test suite changes (tests added/removed) and documentation is not updated synchronously
  - Mitigation hint: Add test count to a generated artifact or CI output that documentation can reference; consider NFR-TRACE-001 MET-1 as ongoing check
  - Evidence: REQ-005; problem_statement.md L57; current test count is 102 passing per test_output.log

- RSK-003 [LOW] [SECURITY]
  - What: Path traversal concern in secrets.rs documented without corresponding fix may create false sense of security
  - Trigger: If documentation states "known limitation" but no threat assessment or mitigation timeline is provided
  - Mitigation hint: Include threat assessment status and mitigation timeline in security posture documentation; link to future hardening work item
  - Evidence: REQ-006 AC-2; problem_statement.md L89; OQ-SIG-004

- RSK-004 [LOW] [OPS]
  - What: Flow variant documentation may become stale if new variants are added without updating overlap semantics section
  - Trigger: When new flow command files are added (e.g., flow-8-\* or additional variants)
  - Mitigation hint: Add flow variant documentation to pack-check scope; consider automating variant enumeration from command files
  - Evidence: REQ-002; 10 command files currently implement 7 flows with variants

- RSK-005 [LOW] [DATA]
  - What: Agent color coding documentation may be inaccurate if color field purpose is misunderstood
  - Trigger: If color field is schema-validated by tooling but documented as advisory, missing colors could cause silent failures
  - Mitigation hint: Verify color field usage in pack tooling before documenting purpose; assume advisory if no consumer found
  - Evidence: REQ-007; OQ-SIG-005; agent frontmatter includes color: field

## Risk Summary (derived)

- Critical: 0
- High: 0
- Medium: 2
- Low: 3

## Notes

- No HIGH or CRITICAL risks identified; this is documentation alignment work with no code changes
- RSK-003 is intentionally rated LOW because path traversal is documented as a known limitation with deferred threat assessment, and the immediate work is documentation-only
- ReDoS is NOT listed as a risk because Rust regex crate is immune by design (finite automata implementation); any prior ReDoS claims are invalid
