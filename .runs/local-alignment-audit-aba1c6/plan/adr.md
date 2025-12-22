# ADR: Layered Documentation Alignment for Seven-Flow Model

## Status
Swarm-Proposed (run-scoped; pending human review at Flow 2 boundary)

## Context
- Problem: DemoSwarm pack documentation claims "six flows" across public documentation (README.md, DEMO_RUN.md, architecture.md, CHANGELOG.md) while CLAUDE.md and the implementation define seven flows with ten command files. This drift creates confusion for integrators, leads to conflicting security claims (invalid ReDoS vulnerability, undocumented path traversal limitation), and test count claims (102 passing tests) do not appear in documentation.
- Constraints:
  - Seven-Flow model is canonical (CLAUDE.md L13 is authoritative)
  - Test counts must derive from actual execution artifacts (test_output.log)
  - Security claims require code evidence (secrets.rs, Rust regex crate)
  - Agent color coding is functional metadata, not documentation-only
  - No secrets in artifacts; no code logic changes in scope
- Non-goals:
  - Changing the flow architecture (seven flows with multi-path variants is intentional)
  - Refactoring secrets.rs path handling (separate security hardening work item)
  - Modifying test count or coverage (only aligning documentation claims to results)
  - Creating new flows or removing existing command files

## Decision Drivers (bound, machine-countable)
Each driver MUST include a stable marker line, then a short explanation.

- DRIVER: DR-001 req=[REQ-001,REQ-002,REQ-003,REQ-004] nfr=[NFR-DOC-001] option_ref="OPT-003"
  - Why it matters: Phased approach satisfies all flow-related requirements (REQ-001 through REQ-004) in Phases 1-2 while respecting the pack hierarchy where CLAUDE.md is authoritative.

- DRIVER: DR-002 req=[REQ-005,REQ-006] nfr=[NFR-SEC-001] option_ref="OPT-003"
  - Why it matters: Security posture and test count corrections (REQ-005, REQ-006) are placed in authoritative sources (Phase 1) with code evidence references, satisfying NFR-SEC-001.

- DRIVER: DR-003 req=[REQ-007] nfr=[] option_ref="OPT-003"
  - Why it matters: Agent color coding clarification (REQ-007) is a low-priority item that can be addressed in Phase 1 or 2 without blocking merge.

- DRIVER: DR-004 req=[] nfr=[NFR-TRACE-001] option_ref="OPT-003"
  - Why it matters: Phase 4 addresses pack-check test fixtures only if needed (reactive, not proactive), avoiding unnecessary test churn while ensuring NFR-TRACE-001 is satisfied.

- DRIVER: DR-005 req=[] nfr=[NFR-DOC-001] option_ref="OPT-003"
  - Why it matters: Allows incremental merge after Phase 2 if time-constrained, with explicit tracking of Phase 3/4 follow-up for secondary docs and pack-check fixtures.

## Decision
We choose **OPT-003: Layered Approach - Authoritative First, Generate/Validate Downstream**.

### What we are doing
- **Phase 1: Authoritative Sources** - Update CLAUDE.md (flow table expansion to 7 flows with variants) and docs/explanation/architecture.md (flow overlap semantics, Flow 7 purpose, security posture, test count)
- **Phase 2: Primary Public Docs** - Update README.md, DEMO_RUN.md, CHANGELOG.md to reference "seven flows" with validation against Phase 1 content
- **Phase 3: Secondary Docs (optional, gated on time)** - Update glossary.md, CONTRIBUTING.md, work-without-github.md, walkthrough.md for consistency
- **Phase 4: Pack Tooling (if needed)** - Update structure.rs test fixtures only if pack-check fails after Phases 1-2
- Each phase produces a checkpoint commit for granular revert capability

### What we are NOT doing
- Comprehensive one-pass update of all 14 files (OPT-002 rejected due to coordination overhead)
- Minimal touch leaving secondary docs inconsistent without tracking (OPT-001 rejected due to NFR-DOC-001 gap)
- Proactive test fixture changes before verifying pack-check failure
- Code changes to secrets.rs or flow architecture

### Requirements & NFR Traceability
- **Satisfied by this decision**
  - REQ-001: All five ACs satisfied in Phases 1-2 (README, DEMO_RUN, architecture.md, CHANGELOG updated to "seven flows")
  - REQ-002: Flow overlap semantics documented in architecture.md (Phase 1) per AC-1 through AC-5
  - REQ-003: Flow 7 purpose documented in architecture.md (Phase 1) per AC-1 through AC-4
  - REQ-004: CLAUDE.md flow table expanded in Phase 1 per AC-1 through AC-4
  - REQ-005: Test count (102 passing) documented in Phase 1 with source artifact reference per AC-1 through AC-4
  - REQ-006: Security posture (ReDoS immunity, path traversal limitation) documented in Phase 1 with code evidence per AC-1 through AC-4
  - REQ-007: Agent color coding clarified as advisory metadata in Phase 1 or 2 per AC-1 through AC-4
  - NFR-SEC-001: Security claims reference specific source files and line numbers (satisfied in Phase 1)
  - NFR-TRACE-001: Pack-check tests continue to pass; Phase 4 addresses fixtures only if needed

- **Trade-offs / partial support**
  - NFR-DOC-001: Primary docs (Phases 1-2) achieve consistency; secondary docs (Phase 3) may remain stale if time-constrained. Explicit follow-up tracking mitigates this gap.

## Alternatives Considered
- ALT: OPT-001 - Rejected because: Leaves secondary documentation inconsistent with no explicit Phase 3 tracking; NFR-DOC-001 would remain PARTIAL without a clear path to resolution. Time savings (5 files vs 8-14) do not justify the audit gap.

- ALT: OPT-002 - Rejected because: Comprehensive sweep of all 14 files introduces coordination overhead and potential merge conflicts. Test fixture changes (structure.rs) before verifying pack-check failure is premature. NFR-TRACE-001 TRADE_OFF risk is unnecessary when reactive approach (Phase 4) suffices.

## Consequences

### Positive
- **Clear derivation lineage**: Authoritative sources (CLAUDE.md, architecture.md) updated first; downstream docs derive from them
- **Incremental merge option**: Phase 2 completion enables merge if time-constrained; Phases 3-4 can be follow-up commits
- **Pack hierarchy respected**: Aligns with CLAUDE.md statement "repo-level policy + shared contracts" (L5)
- **Test fixture risk deferred**: Avoids proactive changes to structure.rs; reactive update only if pack-check fails
- **Granular revert capability**: Per-phase commits enable surgical rollback if issues arise

### Negative
- **Secondary docs may remain stale short-term**: If Phase 3 is deferred, glossary.md, CONTRIBUTING.md, work-without-github.md, walkthrough.md may still say "6 flows"
- **Manual derivation**: Downstream doc updates are not automated; requires human validation against authoritative sources
- **Phase 3/4 follow-up tracking burden**: Explicit blockers or follow-up issues needed if phases are deferred

## Risks and Mitigations
Use stable markers:

- RISK: RSK-001 Pack-check validation may fail on "Six Flows" test fixtures after CLAUDE.md update -> Mitigation: Phase 4 addresses test fixtures reactively; run pack-check after Phase 2 to verify; only update structure.rs if tests fail

- RISK: RSK-002 Phase 3/4 never completed if deferred -> Mitigation: Create explicit follow-up issue in Gate or Review flow; include in blockers list if mandatory for merge

- RISK: RSK-003 CLAUDE.md changes conflict with concurrent pack development -> Mitigation: CLAUDE.md is rarely edited; coordinate timing if other PRs touch it; merge Phase 1 quickly

- RISK: RSK-004 Test count drifts again after documentation update -> Mitigation: Add test count source reference (test_output.log line number) per REQ-005 AC-4; consider automation in Wisdom flow

- RISK: RSK-005 Flow variant documentation becomes stale if new variants added -> Mitigation: Add flow variant enumeration to pack-check scope (future enhancement); document derivation source

## Assumptions Made to Proceed
Use stable markers:

- ASM: ASM-001 CLAUDE.md is the authoritative source for flow architecture (impact if wrong: Would need to update CLAUDE.md to match public docs instead of vice versa; unlikely given CLAUDE.md L5 explicitly claims authority)

- ASM: ASM-002 Flow variants (flow-4-gate, flow-4-review, etc.) are intentional re-entry points, not duplicates (impact if wrong: Would need to deprecate variant commands; unlikely given consistent re-entry pattern across multiple flow pairs)

- ASM: ASM-003 102 passing unit tests from test_output.log is the current authoritative count (impact if wrong: Would need to identify correct test execution artifact and reconcile counts)

- ASM: ASM-004 Path traversal in secrets.rs is a documentation/awareness issue, not an immediate exploitable vulnerability (impact if wrong: Would escalate to security hardening work item with higher priority)

- ASM: ASM-005 Agent color coding is advisory metadata, not schema-enforced (impact if wrong: Would need to add schema validation and populate missing color fields)

- ASM: ASM-006 Pack-check structure.rs test fixtures use "Six Flows" as string literals, not semantic assertions (impact if wrong: Test fixture updates in Phase 4 may require deeper investigation)

## Questions / Clarifications Needed
Use stable markers and include suggested defaults:

- Q: OQ-PLAN-001 Should the documentation update be structured as a single atomic PR or partitioned into logical commits per file/topic? Suggested default: Single atomic PR with logical commits per phase. Impact: If single commit preferred, simplifies review but loses granular revert capability.

- Q: OQ-PLAN-002 Should Flow 7 (/flow-7-wisdom) documentation explicitly reference "second-cycle" or "iteration" use case? Suggested default: Yes, explicitly describe as "second-cycle wisdom extraction for multi-iteration runs" to distinguish from /flow-6-wisdom. Impact: If generic, loses distinction from /flow-6-wisdom.

- Q: OQ-PLAN-003 Should compliance partitioning schema be updated to include ST-007 for Flow 7? Suggested default: Add ST-007 for completeness since Flow 7 is a distinct flow. Impact: If ST-006 covers both, no schema change but may confuse compliance tracing.

## Next Steps (Flow 2 binding)
- Interface/contracts -> `.runs/local-alignment-audit-aba1c6/plan/api_contracts.yaml` + `.runs/local-alignment-audit-aba1c6/plan/schema.md` (N/A for documentation-only work; no API or schema changes)
- Observability -> `.runs/local-alignment-audit-aba1c6/plan/observability_spec.md` (N/A for documentation-only work; pack-check validation serves as observability)
- Tests -> `.runs/local-alignment-audit-aba1c6/plan/test_plan.md` (verification via grep "six flows" returning zero matches; pack-check execution)
- Work breakdown -> `.runs/local-alignment-audit-aba1c6/plan/work_plan.md` (Phase 1: CLAUDE.md + architecture.md; Phase 2: README + DEMO_RUN + CHANGELOG; Phase 3: secondary docs; Phase 4: pack-check fixtures if needed)

## Pointers
- Options: `.runs/local-alignment-audit-aba1c6/plan/design_options.md`
- Requirements: `.runs/local-alignment-audit-aba1c6/signal/requirements.md`
- Problem statement: `.runs/local-alignment-audit-aba1c6/signal/problem_statement.md`
- Impact: `.runs/local-alignment-audit-aba1c6/plan/impact_map.json`
- Option critique: `.runs/local-alignment-audit-aba1c6/plan/option_critique.md`
- Open questions: `.runs/local-alignment-audit-aba1c6/plan/open_questions.md`
- Early risks: `.runs/local-alignment-audit-aba1c6/signal/early_risks.md`

## Inventory (machine countable)
(Only the following prefixed lines; do not rename prefixes)

- ADR_CHOSEN_OPTION: OPT-003
- ADR_DRIVER: DR-001
- ADR_DRIVER: DR-002
- ADR_DRIVER: DR-003
- ADR_DRIVER: DR-004
- ADR_DRIVER: DR-005
- ADR_ALT: OPT-001
- ADR_ALT: OPT-002
- ADR_RISK: RSK-001
- ADR_RISK: RSK-002
- ADR_RISK: RSK-003
- ADR_RISK: RSK-004
- ADR_RISK: RSK-005
- ADR_ASM: ASM-001
- ADR_ASM: ASM-002
- ADR_ASM: ASM-003
- ADR_ASM: ASM-004
- ADR_ASM: ASM-005
- ADR_ASM: ASM-006
- ADR_Q: OQ-PLAN-001
- ADR_Q: OQ-PLAN-002
- ADR_Q: OQ-PLAN-003

## Machine Summary Block

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Phase 3 (secondary docs) may be deferred; track as follow-up
  - Phase 4 (pack-check fixtures) reactive only; depends on pack-check result

chosen_option: OPT-003 Layered Approach
drivers_total: 5
```
