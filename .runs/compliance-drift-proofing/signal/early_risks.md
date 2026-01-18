# Early Risks

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:

- RSK-001 (HIGH) remains highest risk due to empirical evidence from #49 bounce
- 10 open questions indicate some design uncertainty, but all have suggested defaults

## Risks

- RSK-001 [HIGH] [OPS]
  - What: Prior related work (issue #49 align-doc-ownership) bounced at Gate, indicating implementation complexity may be underestimated.
  - Trigger: When implementing similar boundary enforcement rules, may encounter same blockers that caused #49 to bounce.
  - Mitigation hint: Review #49 Gate bounce reasons before implementation; design for warning-first mode (REQ-005) to allow incremental rollout; limit scope to syntactic checks (non-goal: semantic validation).
  - Evidence: problem_statement.md -> Prior Art (issue #49 bounced at Gate); OQ-SIG-008 (relationship to #49); ASM-001 (three-tier ownership is authoritative)

- RSK-002 [MEDIUM] [DATA]
  - What: PLN vs PLAN prefix inconsistency in existing documentation may cause validation confusion or require mass migration.
  - Trigger: When REQ-003 validation enforces canonical prefixes, existing QIDs using non-canonical forms will produce warnings.
  - Mitigation hint: Resolve OQ-SIG-002 definitively before implementation; update stable-markers.md line 60 if PLN/BLD is canonical. ASM-003 assumes PLN/BLD is canonical.
  - Evidence: open_questions.md OQ-SIG-002; stable-markers.md line 60 vs openq-tools/SKILL.md; requirements.md ASM-002

- RSK-003 [MEDIUM] [OPS]
  - What: 4 agents using demoswarm.sh may lack required Skills sections, requiring content remediation alongside validation code.
  - Trigger: When REQ-002 validation runs, these agents will produce warnings (or errors with --strict).
  - Mitigation hint: Enumerate specific agents via audit (grep for demoswarm.sh minus grep for ## Skills); remediate in parallel with validation rule development.
  - Evidence: problem_statement.md -> Who Is Affected; OQ-SIG-004 (exemption policy); ASM-004 (assumes gaps not exceptions)

- RSK-004 [MEDIUM] [COMPLIANCE]
  - What: Warning-first validation mode (REQ-005) may delay compliance enforcement if teams never opt into --strict.
  - Trigger: When new rules produce warnings but are never enforced, drift continues without consequence.
  - Mitigation hint: Document clear timeline for warning-to-error promotion; consider CI gate that requires --strict in future release. REQ-005 provides the infrastructure; policy is separate.
  - Evidence: REQ-005; OQ-SIG-001 (warnings vs failures open question); NFR-COMP-001 MET-2 (warning-only default)

- RSK-005 [LOW] [PERFORMANCE]
  - What: Adding 3-5 new validation rules to pack-check may impact CI runtime (NFR-PERF-001 bounds).
  - Trigger: If rules involve scanning many files or complex pattern matching, may exceed 30-second total or 5-second incremental bounds.
  - Mitigation hint: Profile pack-check after each rule addition; optimize regex patterns; consider file caching. Current baseline is well under 30 seconds.
  - Evidence: NFR-PERF-001 MET-1 (< 30s), MET-2 (< 5s incremental); requirements_critique.md severity_summary (no CRITICAL/MAJOR)

- RSK-006 [LOW] [SECURITY]
  - What: Test fixtures for Build receipt validation (REQ-004) could accidentally include sensitive data patterns.
  - Trigger: If fixture JSON contains realistic-looking secrets (even fake ones), may confuse secrets-sanitizer or be flagged in scans.
  - Mitigation hint: Use obviously synthetic values in test fixtures (e.g., "FAKE_TOKEN_12345"); review via NFR-SEC-001 MET-2 during code review.
  - Evidence: NFR-SEC-001 MET-2 (test fixture safety); REQ-004 AC-1/AC-2 (fixture requirements); OQ-SIG-010 (committed vs dynamic fixtures)

- RSK-007 [MEDIUM] [OPS]
  - What: Scope overlaps with bounced #49 may cause merge conflicts or duplicated effort if #49 resumes.
  - Trigger: If issue #49 work resumes before or during this implementation, changes may conflict in drift.rs or contracts.rs.
  - Mitigation hint: Proceed independently per OQ-SIG-008 suggested default; communicate with #49 stakeholders if work resumes. Explicit non-goal: not superseding #49.
  - Evidence: OQ-SIG-008 (relationship to #49); problem_statement.md Non-Goals (not superseding #49); stakeholders.md Consulted (Issue #49 Stakeholders)

- RSK-008 [LOW] [DATA]
  - What: Hardcoded skill CLI subcommand list (REQ-001) may drift as new skills are added.
  - Trigger: When a new skill is added to .claude/skills/, the hardcoded list in contracts.rs becomes stale.
  - Mitigation hint: NFR-MAINT-001 requires constant definition in contracts.rs; document update process; OQ-SIG-006 remains open on dynamic vs hardcoded.
  - Evidence: REQ-001 AC-2 (skill CLI subcommands); NFR-MAINT-001 MET-1 (constant definition); OQ-SIG-006 (dynamic vs hardcoded)

## Risk Summary (derived)

- Critical: 0
- High: 1
- Medium: 4
- Low: 3

## Notes

- RSK-001 is rated HIGH because prior work (issue #49) bounced at Gate, which is empirical evidence of implementation difficulty. However, this work has explicit mitigations (warning-first mode, narrower scope to syntactic checks) that may avoid the same fate.

- RSK-008 added in iteration 2 to capture skill list drift risk flagged by OQ-SIG-006.

- Security risks (RSK-006) are rated LOW because validation tools print paths not contents, and NFR-SEC-001 explicitly addresses fixture safety.

- No CRITICAL risks identified because:
  1. All issues have documented mitigations
  2. The work is additive (new validation rules) rather than breaking (no removal of existing functionality)
  3. Warning-first mode (REQ-005) provides escape valve for unexpected issues

- Risks intentionally not included:
  - "New Rust code may have bugs" -- covered by standard code review and test processes, not project-specific
  - "CI may be slow" -- covered by NFR-PERF-001 with explicit bounds

## Iteration 2 Changes

- Added RSK-008 [LOW] [DATA] for skill CLI subcommand list drift (OQ-SIG-006 connection).
- Updated RSK-001 mitigation to reference non-goal (semantic validation out of scope).
- Updated RSK-002 to reference ASM-003 (PLN/BLD canonical assumption).
- Updated RSK-004 to clarify REQ-005 provides infrastructure; enforcement policy is separate.
- Updated risk summary: Critical: 0, High: 1, Medium: 4, Low: 3 (was 2).
