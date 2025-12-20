# Problem Statement

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - Path traversal concern in secrets.rs requires threat modeling to determine exploitability
  - Integration test coverage (41 filtered tests) not included in test narrative
  - Flow overlap semantics (alternate paths vs distinct flows) need authoritative decision

confidence: High

## The Problem

DemoSwarm pack documentation has drifted from the actual implementation across three critical dimensions: (1) flow architecture claims ("six flows" in public docs) do not match the implemented seven-flow model with 10 command files, (2) test coverage metrics claimed in documentation do not match actual test execution results (102 passing unit tests vs undocumented claims), and (3) security posture documentation contains both invalid claims (ReDoS vulnerability in Rust regex crate, which is immune by design) and unaddressed valid concerns (path traversal in secrets scanner due to missing canonicalization).

This documentation drift creates confusion for integrators who rely on flow count and test coverage claims to understand the system, and may cause users to either overestimate security risks (believing ReDoS is possible when it is not) or underestimate them (not being aware of path traversal exposure). The pack's internal reference (CLAUDE.md) is more accurate than public-facing documentation (README, DEMO_RUN, architecture.md), indicating a sync discipline gap between authoritative pack contracts and user-facing guides.

Additionally, the pack implements a multi-path flow design (flow-4-gate vs flow-4-review, flow-5-gate vs flow-5-deploy, etc.) that enables re-entry patterns but is nowhere explained to users. Flow 7 (/flow-7-wisdom) exists as a command file but appears in no public documentation whatsoever.

## Who Is Affected

- **Pack integrators**: Cannot reliably understand how many flows exist or when to use flow variants; may choose wrong entry point for review/gate/deploy cycles
- **Security reviewers**: Receive conflicting signals about vulnerability posture (ReDoS claimed but not exploitable; path traversal valid but undocumented)
- **Test reviewers / compliance auditors**: Cannot reconcile documented test counts with actual test execution output
- **Documentation maintainers**: Unclear which source (CLAUDE.md vs README vs architecture.md) is authoritative for flow count
- **Downstream tooling (pack-check)**: May enforce stale "six flow" constraints if policy is derived from incorrect documentation

## Constraints

- **Immutable flow architecture**: Seven-Flow model is canonical (confirmed in CLAUDE.md line 13); documentation must align to code, not vice versa
- **Test counts must reflect actual pass count**: Claims must be derived from test execution artifacts, not aspirational targets
- **Security claims require code evidence**: Statements about vulnerability posture must be supported by implementation inspection
- **Agent metadata discipline**: Color coding in frontmatter exists and is used; documentation claiming it is "documentation-only" is incorrect
- **No secrets in artifacts**: Any security analysis must not reproduce actual secret patterns or values

## Non-Goals

- Changing the flow architecture (the seven-flow model with multi-path variants is intentional design)
- Refactoring secrets.rs to add path canonicalization (that would be a separate security hardening work item)
- Modifying test count or coverage (only aligning documentation claims to actual results)
- Creating new flows or removing existing flow command files
- Determining whether the multi-path design is optimal (only documenting what exists)

## Success Looks Like

- All public-facing documentation (README.md, DEMO_RUN.md, docs/explanation/architecture.md, CHANGELOG.md) references "seven flows" consistently
- Flow overlap semantics (flow-4-gate vs flow-4-review, etc.) are explained in user-facing documentation with guidance on when to use each variant
- Flow 7 (/flow-7-wisdom) is documented with its purpose and use case
- Test coverage claims in documentation match actual test execution results (102 passing unit tests as of last run)
- Security posture documentation accurately reflects: (a) Rust regex is immune to ReDoS, (b) path traversal in secrets scanner is a known limitation pending threat assessment
- Agent color coding is acknowledged as functional metadata, not documentation-only
- CLAUDE.md flow table (currently showing 6 flows) is updated to reflect the full seven-flow + variant model
- No regressions in pack-check validation (wisdom.rs checks should continue to pass)

## Known Context

- **Modules/files mentioned**:
  - `.claude/commands/flow-*.md` (10 command files implementing flows)
  - `tools/demoswarm-runs-tools/src/commands/secrets.rs` (path handling without canonicalization)
  - `CLAUDE.md` (pack reference, mostly accurate but flow table outdated)
  - `README.md`, `DEMO_RUN.md`, `docs/explanation/architecture.md` (public docs claiming "six flows")
  - `CHANGELOG.md` v1.0.0 (claims "6 flow commands")
  - `tools/demoswarm-pack-check/src/checks/wisdom.rs` (Flow 7 validation)
  - Agent frontmatter files (contain `color:` field)

- **Prior art / related runs**:
  - `align-doc-ownership`: Documentation ownership boundaries (completed with BOUNCE verdict); established separation between flow commands, agent docs, skill docs
  - `compliance-drift-proofing`: Mechanical compliance enforcement; subtask partitioning (ST-001 through ST-006) does not account for Flow 7

## Assumptions Made to Proceed

- **ASM-1**: The seven-flow model in CLAUDE.md line 13 is authoritative, and "six flow" references elsewhere are stale.
  - *If wrong*: Would need to reconcile by removing Flow 7 or redefining what "flow" means (unlikely given flow-7-wisdom.md exists and is functional)

- **ASM-2**: Flow variants (flow-4-gate vs flow-4-review, etc.) represent intentional alternate entry points, not bugs or duplicates.
  - *If wrong*: Would need to deprecate or consolidate command files (unlikely given the re-entry pattern is consistent across multiple flow pairs)

- **ASM-3**: The 102 passing unit tests in test_output.log represents the most recent authoritative test execution.
  - *If wrong*: Would need to identify correct test execution artifact and reconcile counts

- **ASM-4**: Path traversal in secrets.rs is a documentation/awareness issue, not an immediate exploitable vulnerability (depends on whether untrusted input reaches this code path).
  - *If wrong*: Would escalate to security hardening work item with higher priority

- **ASM-5**: Agent color coding is functional metadata used by tooling (routing, UI, etc.), not purely decorative.
  - *If wrong*: Could downgrade to "cosmetic documentation" category

## Questions / Clarifications Needed

- Q: What is the intended purpose of `/flow-7-wisdom` and when should users invoke it vs `/flow-6-wisdom`? Suggested default: Flow 7 is a second-cycle wisdom extraction for multi-iteration runs.

- Q: Should the documentation present the multi-path design as "7 flows with variant entry points" or "6 main flows with 4 alternate paths"? Suggested default: Present as "7 flows" matching CLAUDE.md, with a subsection explaining the variant pattern.

- Q: Is there a documented test count claim that conflicts with 102 passing tests, or is the "374 tests" reference from a different source? Suggested default: 102 is the current correct count; any conflicting claim is stale.

- Q: Does untrusted user input ever reach the path argument in secrets.rs, making path traversal exploitable? Suggested default: Assume `.runs/` paths are agent-controlled (low risk), but document the limitation.
