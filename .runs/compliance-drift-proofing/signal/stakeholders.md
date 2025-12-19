# Stakeholders

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - Clarifier agent role in OpenQ generation is now explicit (REQ-003 consumer)
  - 4 agents missing Skills sections are gaps requiring remediation (per ASM-004)

## Primary

- **Pack Maintainers**: Directly affected by new validation rules; responsible for evolving pack-check (Rust) and maintaining compliance infrastructure. Will author Rust code for REQ-001, REQ-002, REQ-003, REQ-005.

- **Agent Authors**: Produce agent prompts that must comply with Skills section requirements (REQ-002); affected by validation feedback loop. 4 existing agents may need remediation.

- **Cleanup Agents** (signal-cleanup, plan-cleanup, build-cleanup, gate-cleanup, deploy-cleanup, wisdom-cleanup): Depend on stable markers for mechanical counts; any drift in marker patterns would break receipt generation. Primary consumers of runs-derive skill.

## Secondary

- **Flow Command Maintainers**: Must ensure flow commands remain free of skill-layer CLI syntax (REQ-001); affected by boundary enforcement. Flow commands own routing; skill docs own CLI truth.

- **Receipt-Checker Agent**: Consumes Build receipts for Gate validation; handshake test fixtures (REQ-004) directly impact its validation logic. Key consumer of Build-to-Gate contract.

- **OpenQ Authors**: Anyone generating open questions must use canonical QID prefixes (REQ-003: SIG/PLN/BLD/GAT/DEP/WIS per stable-markers.md).

- **Clarifier Agent**: Generates OpenQ IDs with flow prefixes using openq-tools skill. Must emit normalized prefixes per REQ-003. Key consumer of OQ-SIG-002 resolution (PLN vs PLAN).

## Consulted

- **CI Pipeline Owners**: Input needed on NFR-PERF-001 (30-second runtime bound) and NFR-REL-001 (deterministic output verification in CI).

- **Documentation Owners**: Input needed on migration path documentation (NFR-COMP-001 MET-3) and pack-check help text (NFR-OPS-001 MET-3).

- **Issue #49 Stakeholders**: Prior align-doc-ownership work bounced at Gate; coordination may be needed to avoid conflicting changes if #49 resumes (OQ-SIG-008).

## Informed

- **Downstream Swarm Users**: Will benefit from improved compliance enforcement; need to know about --strict flag for stricter validation (REQ-005). Warning-first mode enables incremental adoption.

- **Rust Tooling Contributors**: New pack-check rules require Rust development; contracts.rs will be extended with SKILL_CLI_SUBCOMMANDS and OPENQ_FLOW_CODES constants (NFR-MAINT-001).

## Notes

- **Key dependency**: pack-check (Rust) is the primary validation tool; most new rules will be implemented in drift.rs, control_plane.rs, or structure.rs.

- **Related work**: Issue #49 (align-doc-ownership) is a bounced Gate; this work complements but does not depend on its resolution per ASM-001.

- **Integration points identified**:
  1. `tools/demoswarm-pack-check/src/checks/drift.rs` (14 existing drift checks; foundation for REQ-001, REQ-002)
  2. `tools/demoswarm-pack-check/src/contracts.rs` (canonical constants; update for REQ-003 OpenQ codes)
  3. `scripts/check-doc-drift.sh` (6 Bash-based drift guards; possible venue for some checks per OQ-SIG-005)

- **4 agents** may be missing Skills sections (gap to remediate per ASM-004, not intentional exceptions). Enumeration needed via audit.

- **Three-tier ownership model** (authoritative from issue #49): Flow commands own routing; agent docs own operational detail; skill docs own CLI truth; CLAUDE.md is table of contents only.

## Iteration 2 Changes

- Added explicit Clarifier agent role in Secondary stakeholders (REQ-003 consumer).
- Added three-tier ownership model reference in Notes (connects to RSK-001).
- Clarified cleanup agents span all 6 flows.
- Referenced specific open questions where stakeholder input is needed (OQ-SIG-002, OQ-SIG-008).
