# Risk Assessment

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_flow: 2
route_to_agent: null

blockers: []

missing_required: []

concerns:
  - RSK-001 (path traversal) requires threat modeling to confirm LOW severity assumption
  - RSK-003 (pack-check drift) depends on OQ-SIG-001 resolution for Seven-Flow vs Six-Flow

severity_summary:
  critical: 0
  high: 0
  medium: 4
  low: 3

## Context
- flow: signal
- run_id: local-alignment-audit-aba1c6
- inputs_used:
  - `.runs/local-alignment-audit-aba1c6/run_meta.json`
  - `.runs/local-alignment-audit-aba1c6/signal/problem_statement.md`
  - `.runs/local-alignment-audit-aba1c6/signal/requirements.md`
  - `.runs/local-alignment-audit-aba1c6/signal/early_risks.md`
  - `.runs/local-alignment-audit-aba1c6/signal/open_questions.md`
- prior_risk_assessments_seen:
  - none

## Risk Register

| ID | Category | Severity | Status | Summary | Owner |
|----|----------|----------|--------|---------|-------|
| RSK-001 | SECURITY | MEDIUM | OPEN | Path traversal in secrets.rs due to missing canonicalization | security |
| RSK-002 | SECURITY | LOW | MITIGATED | ReDoS misconception in documentation (Rust regex is immune) | documentation |
| RSK-003 | COMPLIANCE | MEDIUM | OPEN | pack-check may enforce stale Six-Flow constraints | tooling |
| RSK-004 | DOCUMENTATION | MEDIUM | OPEN | Stale flow count claims causing user confusion | documentation |
| RSK-005 | DOCUMENTATION | LOW | OPEN | Flow overlap semantics undocumented (variant commands) | documentation |
| RSK-006 | OPS | MEDIUM | OPEN | Flow 7 completely undocumented leading to wrong flow selection | documentation |
| RSK-007 | DOCUMENTATION | LOW | OPEN | Agent color coding purpose unclear (decorative vs functional) | documentation |

## Risk Details

### RSK-001: Path Traversal in secrets.rs
- Category: SECURITY
- Severity: MEDIUM
- Status: OPEN
- Likelihood: Low
- Impact: Medium
- Evidence:
  - `tools/demoswarm-runs-tools/src/commands/secrets.rs` (lines 76-124) accepts raw paths without canonicalization
  - No `realpath` or `canonicalize` calls observed in path handling
  - `problem_statement.md` (L22) identifies this as "unaddressed valid concern"
  - `open_questions.md` (OQ-SIG-004) tracks exploitability question
- Impact:
  - If untrusted input reaches path argument, attacker could scan or redact files outside `.runs/` boundary
  - Could expose secrets in unintended locations if symlinks or `../` sequences are crafted
  - In current context (local agent-controlled paths from `.runs/`), exploitation requires agent compromise or malicious `.runs/` content
- Mitigation:
  - Document as known limitation pending threat assessment (per ASM-004 in problem_statement.md)
  - Recommended: Add path canonicalization in future security hardening work item (non-goal for this run per problem_statement.md L47)
  - Defense-in-depth: Validate paths start with expected `.runs/` prefix before processing
- Verification:
  - Threat model confirms paths are agent-controlled (no untrusted external input)
  - Future: Unit tests confirming path traversal attempts are rejected
- Recommendation:
  - PROCEED with documentation work; escalate to separate security hardening run if threat model reveals higher risk

### RSK-002: ReDoS Misconception in Documentation
- Category: SECURITY
- Severity: LOW
- Status: MITIGATED
- Likelihood: None (false positive)
- Impact: Low (documentation confusion only)
- Evidence:
  - `secrets.rs` uses Rust `regex` crate (line 14 per problem_statement.md)
  - Rust regex crate uses finite automata implementation, not backtracking
  - Finite automata regex engines are immune to ReDoS by design
  - `open_questions.md` assumption confirms "Rust regex crate used (not vulnerable to ReDoS)"
- Impact:
  - If documented as vulnerable, security reviewers waste time on false positive
  - Users may perceive system as less secure than it is
  - No actual exploitability exists
- Mitigation:
  - REQ-006 (AC-1, AC-3) addresses this: Documentation must state Rust regex is immune to ReDoS
  - Remove any existing ReDoS claims from security documentation
- Verification:
  - Code inspection confirms `regex` crate usage
  - Gate check: No ReDoS vulnerability claims in updated documentation
- Recommendation:
  - PROCEED; this is a documentation correction, not a code fix

### RSK-003: Pack-Check Drift from Stale Documentation
- Category: COMPLIANCE
- Severity: MEDIUM
- Status: OPEN
- Likelihood: Medium
- Impact: Medium
- Evidence:
  - `problem_statement.md` (L34): "Downstream tooling (pack-check) May enforce stale six flow constraints if policy is derived from incorrect documentation"
  - `tools/demoswarm-pack-check/src/checks/wisdom.rs` validates Flow 7
  - `CLAUDE.md` flow table currently shows 6 flows (L147-157 per requirements.md)
  - `open_questions.md` (OQ-SIG-003) asks whether Flow 7 needs ST-007 compliance coverage
- Impact:
  - If pack-check policies derive from stale "six flow" documentation, Flow 7 validation may be incorrectly flagged
  - Compliance drift-proofing runs (ST-001 through ST-006) may miss Flow 7 coverage
  - Users running pack-check may receive conflicting signals about flow count validity
- Mitigation:
  - REQ-004 addresses root cause: Update CLAUDE.md flow table to seven-flow model
  - Verify wisdom.rs checks continue to pass after documentation updates (NFR-TRACE-001)
  - OQ-SIG-003 must be resolved to determine if ST-007 is needed
- Verification:
  - Gate: `pack-check` validation passes after all documentation updates
  - Gate: wisdom.rs checks continue to pass
- Recommendation:
  - PROCEED; pack-check already validates Flow 7 (wisdom.rs exists); documentation alignment will remove confusion

### RSK-004: Stale Flow Count Claims in Public Documentation
- Category: DOCUMENTATION
- Severity: MEDIUM
- Status: OPEN
- Likelihood: High
- Impact: Medium
- Evidence:
  - `README.md` L67 references "six flows" (per github_research.md)
  - `DEMO_RUN.md` references "six flows"
  - `docs/explanation/architecture.md` references "six flows"
  - `CHANGELOG.md` v1.0.0 claims "6 flow commands"
  - `CLAUDE.md` L13 states "7 flows" (authoritative per ASM-001)
  - 10 command files exist implementing flows
- Impact:
  - Users confused about actual flow count when learning the system
  - Integrators may miss Flow 7 entirely or assume it is unsupported
  - Documentation maintainers unclear which source is authoritative
- Mitigation:
  - REQ-001 addresses this: Update all public docs to reference "seven flows"
  - AC-5 verification: No remaining "six flows" occurrences in public docs
  - NFR-DOC-001 metric: Automated search returns zero matches for "six flows"
- Verification:
  - Gate: grep "six flows" returns zero matches in public docs
  - Cross-file consistency check confirms "seven flows" everywhere
- Recommendation:
  - PROCEED; straightforward documentation update

### RSK-005: Flow Overlap Semantics Undocumented
- Category: DOCUMENTATION
- Severity: LOW
- Status: OPEN
- Likelihood: High
- Impact: Low
- Evidence:
  - 10 command files implement 7 flows with variant paths
  - `flow-4-gate` vs `flow-4-review` represent different entry points
  - `flow-5-gate` vs `flow-5-deploy` represent gate re-entry vs deployment
  - `flow-6-deploy` vs `flow-6-wisdom` represent deploy vs wisdom extraction
  - `flow-7-wisdom` purpose completely undocumented
  - No user-facing documentation explains when to use each variant
- Impact:
  - Users may choose wrong entry point for their use case
  - Re-entry patterns after gate rejection not discoverable
  - Confusion about relationship between variant commands
- Mitigation:
  - REQ-002 addresses this: Document multi-path design with guidance on variant selection
  - AC-4 requires guidance like "use /flow-4-review after PR feedback, /flow-4-gate before merge"
- Verification:
  - Gate: Documentation includes flow overlap explanation
  - User guidance specifies when to use each variant
- Recommendation:
  - PROCEED; documentation improvement

### RSK-006: Flow 7 Completely Undocumented
- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Likelihood: High
- Impact: Medium
- Evidence:
  - `flow-7-wisdom.md` command file exists and is functional
  - Flow 7 appears in no public documentation (README, DEMO_RUN, architecture.md)
  - `open_questions.md` (OQ-SIG-002) tracks purpose question
  - `problem_statement.md` (L26): "Flow 7 exists as a command file but appears in no public documentation whatsoever"
- Impact:
  - Users unaware Flow 7 exists may skip wisdom extraction step
  - Distinction between `/flow-6-wisdom` and `/flow-7-wisdom` unknown
  - Operational guidance missing for multi-iteration runs
- Mitigation:
  - REQ-003 addresses this: Document Flow 7 purpose, when to invoke, how it differs from Flow 6
  - OQ-SIG-002 must be resolved to determine exact purpose
  - Suggested default: Flow 7 is for second-cycle wisdom extraction
- Verification:
  - Gate: Flow 7 appears in flow enumeration in public docs
  - Documentation explains when to use Flow 7 vs Flow 6
- Recommendation:
  - PROCEED with suggested default; adjust if OQ-SIG-002 resolves differently

### RSK-007: Agent Color Coding Purpose Unclear
- Category: DOCUMENTATION
- Severity: LOW
- Status: OPEN
- Likelihood: Medium
- Impact: Low
- Evidence:
  - Agent frontmatter contains `color:` field (e.g., `color: red` in test-critic.md)
  - `problem_statement.md` (L41): Documentation claims color coding is "documentation-only" but field is consistently present
  - `open_questions.md` (OQ-SIG-005) tracks whether colors are advisory or schema-validated
- Impact:
  - If used for routing/UI logic but documented as decorative, missing colors would cause failures
  - If purely decorative but documented as functional, maintainers waste effort ensuring consistency
  - Minor user confusion about metadata purpose
- Mitigation:
  - REQ-007 addresses this: Clarify whether color coding is advisory or schema-validated
  - OQ-SIG-005 must be resolved to determine actual usage
  - Suggested default: Advisory (human consumption), not routing
- Verification:
  - Gate: Documentation clarifies color field purpose
  - If functional: Schema validation confirms all agents have color field
- Recommendation:
  - PROCEED with suggested default (advisory); adjust if evidence shows functional usage

## Deltas Since Prior (if any)
- NEW: [RSK-001, RSK-002, RSK-003, RSK-004, RSK-005, RSK-006, RSK-007]
- CHANGED: []
- CLOSED: []

## Recommended Next
- Proceed to Flow 2 (Plan) to design documentation updates addressing REQ-001 through REQ-007
- Resolve OQ-SIG-001 (Seven-Flow vs Six-Flow) before finalizing plan; use suggested default if unresolved
- Resolve OQ-SIG-002 (Flow 7 purpose) before finalizing Flow 7 documentation; use suggested default if unresolved
- RSK-001 (path traversal) does not block documentation work; defer to separate security hardening run if threat model escalates risk
- No CRITICAL or HIGH severity risks identified; all risks have mitigation strategies in requirements
