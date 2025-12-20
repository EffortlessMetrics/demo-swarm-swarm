# Requirements

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - OQ-SIG-001 (Six-Flow vs Seven-Flow) remains open; requirements assume Seven-Flow is canonical per CLAUDE.md L13
  - OQ-SIG-004 (path traversal exploitability) deferred to threat assessment; documented as known limitation per ASM-4
  - Test count story (OQ-SIG-006) assumes 102 is the current correct count; if different counts exist elsewhere, they are stale

## Functional Requirements

### REQ-001: Update Flow Count References in Public Documentation
The system shall update all public-facing documentation to reference "seven flows" instead of "six flows."

Priority: HIGH
Traceability: problem_statement.md (L54), github_research.md (L122-129), OQ-SIG-001

- AC-1: README.md references "seven flows" (not "six flows") in section header and body text.
- AC-2: DEMO_RUN.md references "seven flows" and enumerates Signal through Wisdom.
- AC-3: docs/explanation/architecture.md references "seven flows" in section header and flow enumeration.
- AC-4: CHANGELOG.md v1.0.0 is annotated or corrected to clarify actual command count (10 command files implementing 7 flows).
- AC-5: No remaining occurrences of "six flows" appear in public documentation (README.md, DEMO_RUN.md, docs/explanation/architecture.md).

### REQ-002: Document Flow Overlap Semantics
The system shall document the multi-path flow design explaining when to use each flow variant command.

Priority: HIGH
Traceability: problem_statement.md (L55-56), github_research.md (L135-138), OQ-SIG-001, OQ-SIG-002

- AC-1: Documentation explains the relationship between /flow-4-gate and /flow-4-review (different entry points into review/gate cycle).
- AC-2: Documentation explains the relationship between /flow-5-gate and /flow-5-deploy (gate verdict re-entry vs deployment execution).
- AC-3: Documentation explains the relationship between /flow-6-deploy and /flow-6-wisdom (deploy-after-gate vs wisdom extraction paths).
- AC-4: Documentation provides guidance on when users should choose each variant (e.g., "use /flow-4-review after PR feedback, /flow-4-gate before merge").
- AC-5: Flow overlap documentation is placed in a discoverable location (README.md or docs/explanation/architecture.md).

### REQ-003: Document Flow 7 Purpose and Usage
The system shall add documentation for /flow-7-wisdom explaining its purpose, when to invoke it, and how it differs from /flow-6-wisdom.

Priority: HIGH
Traceability: problem_statement.md (L56), github_research.md (L68, L172), OQ-SIG-002

- AC-1: README.md or architecture documentation includes Flow 7 in the flow enumeration.
- AC-2: Documentation explains the purpose of /flow-7-wisdom (e.g., second-cycle wisdom extraction, batch learnings).
- AC-3: Documentation explains how /flow-7-wisdom differs from /flow-6-wisdom.
- AC-4: Usage guidance specifies when users should invoke /flow-7-wisdom vs /flow-6-wisdom.

### REQ-004: Update CLAUDE.md Flow Table
The system shall update the CLAUDE.md flow table to reflect the seven-flow model with all variant commands.

Priority: HIGH
Traceability: problem_statement.md (L60), github_research.md (L147-157)

- AC-1: CLAUDE.md flow table lists all seven flows (Signal, Plan, Build, Review, Gate, Deploy, Wisdom).
- AC-2: Flow table includes or references variant commands (flow-4-review, flow-5-gate, flow-6-deploy, flow-7-wisdom).
- AC-3: Flow table numbering is consistent with the "7 flows" statement in CLAUDE.md L13.
- AC-4: If variants are listed inline, each variant has a brief description of its use case.

### REQ-005: Correct Test Count Documentation
The system shall update test coverage documentation to reflect actual test execution results.

Priority: MEDIUM
Traceability: problem_statement.md (L57), issue_normalized.md (L72-73), OQ-SIG-006

- AC-1: Documentation references "102 unit tests passing" as the current test count (per test_output.log).
- AC-2: Documentation explains that 277 tests are filtered (integration tests requiring manual environment setup).
- AC-3: Any conflicting test count claims (e.g., "374 tests") are corrected or annotated with context.
- AC-4: Test count claims include the source artifact (e.g., "per test_output.log line 109").

### REQ-006: Update Security Posture Documentation
The system shall update security documentation to accurately reflect the implementation posture.

Priority: MEDIUM
Traceability: problem_statement.md (L58), issue_normalized.md (L88-90)

- AC-1: Documentation states that the Rust regex crate is immune to ReDoS (finite automata implementation, not backtracking).
- AC-2: Documentation notes path traversal as a known limitation in secrets scanner (pending threat assessment).
- AC-3: Documentation does not claim ReDoS vulnerability in secrets scanner (invalid per code evidence).
- AC-4: Security claims reference code evidence (e.g., "secrets.rs uses Rust regex crate, line 14").

### REQ-007: Clarify Agent Color Coding Purpose
The system shall clarify that agent color coding in frontmatter is functional metadata, not documentation-only.

Priority: LOW
Traceability: problem_statement.md (L59), issue_normalized.md (L75-83), OQ-SIG-005

- AC-1: Documentation acknowledges that agent frontmatter includes a `color:` field.
- AC-2: Documentation clarifies whether color coding is advisory (human consumption) or schema-validated (tooling dependency).
- AC-3: If used for routing or UI logic, documentation specifies the consumer of color metadata.
- AC-4: Example agent frontmatter in documentation includes the `color:` field.

## Non-Functional Requirements

### Domain Notes
- DOC: Documentation consistency and maintainability
- SEC: Security and privacy
- TRACE: Traceability and test verification

### NFR-DOC-001: Documentation Consistency
The system shall maintain consistent flow count references across all public documentation.

Priority: HIGH
Traceability: problem_statement.md (L54), concern in Machine Summary

- MET-1: Automated search (grep "six flows" in README.md, DEMO_RUN.md, docs/explanation/architecture.md) returns zero matches after update. Verified in: Gate.
- MET-2: All public documentation files agree on flow count (seven flows). Verified in: Gate via cross-file consistency check.

### NFR-SEC-001: Security Claims Evidence
The system shall ensure all security claims in documentation have corresponding code evidence.

Priority: HIGH
Traceability: problem_statement.md (L43), issue_normalized.md (L88-90)

- MET-1: Each security claim (e.g., ReDoS immunity, path traversal limitation) references specific source file and line number. Verified in: Gate via documentation review.
- MET-2: Security claims are testable or verifiable by code inspection. Verified in: Gate.

### NFR-TRACE-001: Pack-Check Test Continuity
The system shall ensure pack-check tests continue to pass after documentation changes.

Priority: HIGH
Traceability: problem_statement.md (L61)

- MET-1: `pack-check` validation passes after all documentation updates. Verified in: Gate via pack-check execution.
- MET-2: wisdom.rs checks in pack-check continue to pass. Verified in: Gate.

## Assumptions Made
- **ASM-001**: The seven-flow model in CLAUDE.md L13 is authoritative, and "six flow" references elsewhere are stale. (why: CLAUDE.md is described as "repo-level policy + shared contracts" and explicitly lists "7 flows: Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom")
  - Impact if wrong: Would need to reconcile by removing Flow 7 or redefining what "flow" means (unlikely given flow-7-wisdom.md exists and is functional)

- **ASM-002**: Flow variants (flow-4-gate vs flow-4-review, etc.) represent intentional alternate entry points, not bugs or duplicates. (why: Each variant represents a different entry point into the review/gate/deploy/wisdom cycle; enables rework flows after gate rejection)
  - Impact if wrong: Would need to deprecate or consolidate command files (unlikely given the re-entry pattern is consistent across multiple flow pairs)

- **ASM-003**: The 102 passing unit tests in test_output.log represents the most recent authoritative test execution. (why: This is the only test artifact referenced in the normalized issue and problem statement)
  - Impact if wrong: Would need to identify correct test execution artifact and reconcile counts

- **ASM-004**: Path traversal in secrets.rs is a documentation/awareness issue, not an immediate exploitable vulnerability. (why: secrets.rs is invoked locally by agents with controlled paths from .runs/ artifacts; no evidence of untrusted external input reaching this code)
  - Impact if wrong: Would escalate to security hardening work item with higher priority

- **ASM-005**: Agent color coding is functional metadata used by tooling or human consumption, not purely decorative. (why: Color field exists in agent frontmatter and is consistently present; unclear if schema-validated)
  - Impact if wrong: Could downgrade to "cosmetic documentation" category

## Questions for Humans
- Q: What is the intended purpose of /flow-7-wisdom and when should users invoke it vs /flow-6-wisdom? Suggested default: Flow 7 is a second-cycle wisdom extraction for multi-iteration runs. Impact if different: If same purpose as /flow-6-wisdom, one should be deprecated; if distinct but undocumented, documentation must explain when to use each. (Linked: OQ-SIG-002)

- Q: Should the documentation present the multi-path design as "7 flows with variant entry points" or "6 main flows with 4 alternate paths"? Suggested default: Present as "7 flows" matching CLAUDE.md, with a subsection explaining the variant pattern. Impact if different: If 6 flows, documentation update is simpler (just explain variants); if 7 flows, public docs must enumerate Flow 7 distinctly. (Linked: OQ-SIG-001)

- Q: Is the path traversal in secrets.rs exploitable given its local-only execution context? Suggested default: Low risk in current context (local execution, no untrusted input paths), but should be documented as known limitation. Impact if different: If exploitable, requires immediate security fix before proceeding with other work. (Linked: OQ-SIG-004)

- Q: Should agent color coding (frontmatter color: field) be schema-validated, or remain advisory? Suggested default: Remain advisory (no schema validation); colors are for human consumption not routing. Impact if different: If used for routing or UI logic, missing color fields would cause failures and require schema enforcement. (Linked: OQ-SIG-005)
