# ADR: Inline Extension of Existing Modules for Compliance Drift-Proofing

## Status

Swarm-Proposed (run-scoped; pending human review at Flow 2 boundary)

## Context

- Problem: The DemoSwarm pack lacks comprehensive, mechanical enforcement of compliance contracts. Flow commands can contain skill-layer CLI details, agents using demoswarm.sh may lack required Skills sections, OpenQ prefixes are inconsistent (PLN vs PLAN), and the Build-to-Gate handshake has no automated test fixtures. Prior work (issue #49) bounced at Gate, indicating implementation complexity.
- Constraints:
  - Canonical status and action enums are frozen
  - Three-tier ownership model (flow commands -> agent docs -> skill docs) is authoritative
  - New checks must not break existing valid artifacts without migration path (NFR-COMP-001)
  - CI runtime must complete under 30 seconds (NFR-PERF-001)
  - pack-check (Rust) is the preferred venue for structural validation
- Non-goals:
  - Creating new enforcement tooling from scratch
  - Merging pack-check and check-doc-drift.sh into a single tool
  - Implementing semantic/behavioral compliance validation (syntactic/structural only)
  - Full test fixture coverage for all receipt scenarios (Build-to-Gate focus)
  - Changing existing OpenQ IDs (normalize future generation only)

## Decision Drivers (bound, machine-countable)

Each driver MUST include a stable marker line, then a short explanation.

- DRIVER: DR-001 req=[REQ-001,REQ-002,REQ-003] nfr=[NFR-MAINT-001] option_ref="OPT-001"
  - Why it matters: OPT-001 extends existing drift.rs which already has 14 checks (38-49) using proven patterns; adding checks 50-51 follows established conventions with minimal coupling increase.

- DRIVER: DR-002 req=[REQ-005] nfr=[NFR-COMP-001] option_ref="OPT-001"
  - Why it matters: OPT-001 leverages existing --strict_warnings flag in cli.rs (line 38), requiring verification rather than new implementation; backward compatibility preserved.

- DRIVER: DR-003 req=[REQ-004] nfr=[NFR-REL-001] option_ref="OPT-001"
  - Why it matters: Standard Rust test fixtures in tests/fixtures/ directory follow existing project conventions; deterministic validation via committed files.

- DRIVER: DR-004 req=[REQ-006] nfr=[NFR-PERF-001,NFR-OPS-001] option_ref="OPT-001"
  - Why it matters: OPT-001 has lowest implementation effort and negligible performance overhead (O(n) file scan); existing reporter already supports check_id, file path, and line numbers.

- DRIVER: DR-005 req=[] nfr=[] option_ref="OPT-001"
  - Why it matters: Check 49 (check_skills_section_required) already exists for REQ-002; only verification and possible enhancement needed, not new implementation.

## Decision

We choose **OPT-001: Inline Extension of Existing Modules**.

### What we are doing

- Adding check 50 to drift.rs for REQ-001 (flow boundary enforcement): scan flow-\*.md files for demoswarm.sh and skill CLI subcommands
- Verifying check 49 adequacy for REQ-002 (Skills section enforcement); enhance if gaps found
- Adding check 51 to drift.rs for REQ-003 (OpenQ prefix validation): validate QID patterns in .runs/\*\*/open_questions.md
- Creating test fixtures at tools/demoswarm-pack-check/tests/fixtures/ for REQ-004 (Build-to-Gate handshake)
- Verifying --strict_warnings flag behavior for REQ-005; adjust exit code handling if needed
- Adding skill CLI subcommands list and OpenQ flow codes (SIG/PLN/BLD/GAT/DEP/WIS) to contracts.rs per NFR-MAINT-001
- Establishing validation baseline by running pack-check before introducing new rules per REQ-006

### What we are NOT doing

- Creating a new ComplianceRule trait or pluggable rules framework (OPT-002)
- Implementing validation in check-doc-drift.sh (Bash) (OPT-003)
- Migrating existing checks to a new architecture
- Adding per-rule severity configuration (global --strict is sufficient)
- Validating semantic/behavioral compliance (syntactic/structural only)
- Creating test fixtures for Signal-to-Plan or Plan-to-Build handshakes

### Requirements & NFR Traceability

- **Satisfied by this decision**
  - REQ-001: Add check 50 to drift.rs scanning flow-\*.md for demoswarm.sh and skill subcommands
  - REQ-002: Check 49 already exists; verify coverage against AC-1 through AC-4, enhance if needed
  - REQ-003: Add check 51 to drift.rs with regex for OQ-<FLOW>-<NNN> pattern validation
  - REQ-004: Create valid and invalid build_receipt.json fixtures in tests/fixtures/
  - REQ-005: Verify --strict_warnings flag matches AC-1 through AC-4; adjust exit codes if needed
  - REQ-006: Run pack-check baseline before introducing new rules; verify no false positives
  - NFR-PERF-001: Pattern matching is O(n) file scan; negligible overhead well under 30s
  - NFR-REL-001: File-based scan; inherently deterministic; sorted output
  - NFR-OPS-001: Reporter already supports check_id, file path, line numbers via format_line_matches
  - NFR-COMP-001: New checks added; existing behavior unchanged; warnings-first mode
  - NFR-SEC-001: Only file paths and rule violations printed; inherits existing behavior
  - NFR-MAINT-001: Constants in contracts.rs; established pattern

- **Trade-offs / partial support**
  - None identified; all REQ and NFR fully supported by OPT-001

## Alternatives Considered

- ALT: OPT-002 (Modular Compliance Rules Framework) -- Rejected because: Higher initial investment (trait design + registry), introduces new abstraction layer with more moving parts, over-engineering risk for only 2-3 new rules, existing check 49 would need migration or coexistence, and team learning curve for new trait interface. OPT-002 is preferable if 10+ compliance rules are anticipated in the next 6 months, but current scope does not justify the architectural investment.

- ALT: OPT-003 (Minimal + Deferred Architecture) -- Rejected because: Splits validation across two tools (Bash + Rust), creating maintenance divergence and inconsistent --strict behavior. Cross-platform issues on Windows (Git Bash dependency), diagnostic quality regression (bash guards lack structured rule IDs), and NFR-OPS-001 only partially satisfied. While fastest to write, the technical debt is not acceptable for production-quality enforcement.

## Consequences

### Positive

- Fastest path to delivery: copy existing check patterns, minimal new code
- No architectural disruption: follows proven drift.rs conventions (checks 38-49)
- Reversibility is easy: checks can be extracted later if modularization becomes valuable
- Zero backward compatibility risk: new checks added, existing behavior unchanged
- Minimal test surface increase: same testing approach as existing checks
- Check 49 already addresses REQ-002 core requirement (Skills section enforcement)
- Constants in contracts.rs pattern already established; no new infrastructure

### Negative

- drift.rs grows by 2 more checks (16 total); may become unwieldy long-term if compliance rules continue to grow significantly
- Check ID allocation requires manual review of mod.rs to avoid collision (50, 51 must be reserved)
- If --strict_warnings behavior does not match REQ-005, small CLI changes are needed
- No per-rule severity configuration; all new rules elevated together with --strict flag

## Risks and Mitigations

Use stable markers:

- RISK: RSK-001 Prior related work (issue #49) bounced at Gate, indicating implementation complexity may be underestimated. -> Mitigation: Design for warning-first mode (REQ-005) to allow incremental rollout; limit scope to syntactic checks; review #49 Gate bounce reasons before implementation.

- RISK: RSK-002 PLN vs PLAN prefix inconsistency in existing documentation may cause validation confusion. -> Mitigation: Resolve OQ-SIG-002/OQ-PLAN-004 definitively before implementation; update stable-markers.md and contracts.md to use PLN/BLD (canonical per openq-tools).

- RISK: RSK-003 4 agents using demoswarm.sh may lack required Skills sections, requiring content remediation alongside validation code. -> Mitigation: Enumerate specific agents via audit during implementation; remediate in parallel with validation rule development per OQ-PLAN-009.

- RISK: RSK-004 --strict_warnings flag behavior may not exactly match REQ-005 requirements (exit code handling). -> Mitigation: Add tests to verify exit code behavior during implementation; small CLI changes if needed.

- RISK: RSK-005 Check ID collision when assigning 50 and 51. -> Mitigation: Review mod.rs comments before assigning IDs; document reservation in code comments.

- RISK: RSK-006 Hardcoded skill CLI subcommand list may drift as new skills are added. -> Mitigation: Define list as constant in contracts.rs per NFR-MAINT-001; document update process; MET-3 limits changes to 2 files.

## Assumptions Made to Proceed

Use stable markers:

- ASM: ASM-001 The three-tier ownership model (flow commands -> agent docs -> skill docs) from issue #49 is authoritative and stable. (impact if wrong: Ownership boundary enforcement rules would need redesign)

- ASM: ASM-002 PLN/BLD abbreviations in openq-tools/SKILL.md are canonical over PLAN/BUILD in stable-markers.md and contracts.md. (impact if wrong: Would need to update openq-tools Rust code and existing PLN/BLD QIDs rather than documentation)

- ASM: ASM-003 Check 49 (check_skills_section_required) fully addresses REQ-002 Skills section enforcement. (impact if wrong: Would need to enhance check 49 logic, adding minor implementation effort)

- ASM: ASM-004 Existing --strict_warnings flag provides the needed warning-to-error elevation for REQ-005. (impact if wrong: Additional CLI changes required, but exit code behavior is well-scoped)

- ASM: ASM-005 Two new checks (50, 51) will not exceed 30-second CI budget. (impact if wrong: Would need regex optimization, but pattern matching is inherently fast)

- ASM: ASM-006 The 4 agents using demoswarm.sh without Skills sections are gaps to fix, not intentional exceptions. (impact if wrong: Would need exemption list in validation rule rather than adding sections)

## Questions / Clarifications Needed

Use stable markers and include suggested defaults:

- Q: OQ-PLAN-004 Should REQ-003 normalize to PLN/BLD (per openq-tools) or PLAN/BUILD (per stable-markers.md)? Suggested default: PLN/BLD (openq-tools is canonical implementation). Impact: If PLAN/BUILD chosen, requires openq-tools code changes.

- Q: OQ-PLAN-002 How should --strict flag interact with existing exit codes? Suggested default: Exit 0 for success, exit 1 for errors, exit 2 for warnings with --strict (provides diagnostic granularity). Impact: Different exit code scheme may affect CI integration.

- Q: OQ-PLAN-009 Which 4 agents are missing Skills sections? Suggested default: Identify via audit (grep demoswarm.sh minus grep "## Skills"); remediate as part of implementation. Impact: If some are intentionally exempt, need exemption mechanism.

- Q: OQ-PLAN-001 Should checks 50-51 go in drift.rs specifically or a new validation.rs module? Suggested default: drift.rs (aligns with existing boundary enforcement checks 38-49). Impact: New module adds file overhead but provides separation.

## Next Steps (Flow 2 binding)

- Interface/contracts -> `.runs/compliance-drift-proofing/plan/api_contracts.yaml` + `.runs/compliance-drift-proofing/plan/schema.md`
- Observability -> `.runs/compliance-drift-proofing/plan/observability_spec.md`
- Tests -> `.runs/compliance-drift-proofing/plan/test_plan.md` (map to BDD + verification_notes if present)
- Work breakdown -> `.runs/compliance-drift-proofing/plan/work_plan.md`

## Pointers

- Options: `.runs/compliance-drift-proofing/plan/design_options.md`
- Requirements: `.runs/compliance-drift-proofing/signal/requirements.md`
- Problem statement: `.runs/compliance-drift-proofing/signal/problem_statement.md`
- Impact: `.runs/compliance-drift-proofing/plan/impact_map.json`
- Open questions (Signal): `.runs/compliance-drift-proofing/signal/open_questions.md`
- Open questions (Plan): `.runs/compliance-drift-proofing/plan/open_questions.md`
- Option critique: `.runs/compliance-drift-proofing/plan/option_critique.md`
- Early risks: `.runs/compliance-drift-proofing/signal/early_risks.md`

## Inventory (machine countable)

(Only the following prefixed lines; do not rename prefixes)

- ADR_CHOSEN_OPTION: OPT-001
- ADR_DRIVER: DR-001
- ADR_DRIVER: DR-002
- ADR_DRIVER: DR-003
- ADR_DRIVER: DR-004
- ADR_DRIVER: DR-005
- ADR_ALT: OPT-002
- ADR_ALT: OPT-003
- ADR_RISK: RSK-001
- ADR_RISK: RSK-002
- ADR_RISK: RSK-003
- ADR_RISK: RSK-004
- ADR_RISK: RSK-005
- ADR_RISK: RSK-006
- ADR_ASM: ASM-001
- ADR_ASM: ASM-002
- ADR_ASM: ASM-003
- ADR_ASM: ASM-004
- ADR_ASM: ASM-005
- ADR_ASM: ASM-006
- ADR_Q: OQ-PLAN-004 OpenQ prefix normalization
- ADR_Q: OQ-PLAN-002 --strict exit codes
- ADR_Q: OQ-PLAN-009 Missing Skills sections
- ADR_Q: OQ-PLAN-001 Check module location

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
  - OQ-PLAN-004 (PLN vs PLAN prefix) should be resolved before implementation; suggested default is PLN/BLD
  - 4 agents missing Skills sections not yet enumerated (deferred to implementation per OQ-PLAN-009)

chosen_option: OPT-001 Inline Extension of Existing Modules
drivers_total: 5
```
