# Schema: DemoSwarm Documentation Alignment

Run ID: `local-alignment-audit-aba1c6`

## Overview

This document defines the data model, entities, and contracts for aligning DemoSwarm documentation to the canonical seven-flow SDLC model.

### System Boundary

This is a **documentation alignment task**, not an API or code change. The "interface" being defined is the contract between:

- **Authoritative sources** (CLAUDE.md, architecture.md) - define the canonical flow model
- **Derived documentation** (README.md, DEMO_RUN.md, etc.) - must align to authoritative sources
- **Validation tooling** (pack-check, grep) - verify consistency

### Interface List

| Interface | Type | Purpose |
|-----------|------|---------|
| FlowModel | Data Schema | Canonical 7-flow structure |
| FlowCommandRegistry | Data Schema | 10 command files enumeration |
| FlowVariant | Data Schema | Multi-path entry point semantics |
| DocumentationConsistencySpec | Validation Contract | Prohibited/required patterns |
| FlowArtifactPath | Data Schema | Standard artifact locations |

---

## Data Models

### Entity: Flow

The core unit of the SDLC model. Seven flows exist with the following structure:

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| number | integer | 1-7, unique | Flow sequence number |
| name | string | enum: Signal, Plan, Build, Review, Gate, Deploy, Wisdom | Canonical flow name |
| primary_command | string | pattern: `/flow-[1-7]-[a-z]+` | Primary slash command |
| variant_commands | string[] | optional | Alternate entry points (flows 4-7 only) |
| artifact_dir | string | lowercase flow name | Directory under `.runs/<run-id>/` |
| key_outputs | string[] | required | Primary artifacts produced |

**Invariants:**
- Flow numbers 1-3 have no variants
- Flow numbers 4-7 may have one variant each
- artifact_dir matches lowercase flow name

### Entity: FlowCommand

A slash command file under `.claude/commands/`.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| filename | string | pattern: `flow-[1-7]-[a-z]+.md` | Command file name |
| flow_number | integer | 1-7 | Which flow this implements |
| flow_name | string | required | Human-readable flow name |
| is_variant | boolean | required | True if alternate entry point |
| variant_purpose | string | required if is_variant | When to use this variant |

**Invariants:**
- Exactly 10 command files exist (verified by enumeration)
- 7 primary commands + 3 variants
- Variants exist for flows 4, 5, 6 (not 7)

### Entity: DocumentationFile

A documentation file subject to consistency validation.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| path | string | repo-root-relative | File path |
| tier | enum | authoritative, primary, secondary | Update priority |
| flow_references | boolean | required | Contains flow count references |

**Tier semantics (per OPT-003):**
- **authoritative**: CLAUDE.md, architecture.md - update in Phase 1
- **primary**: README.md, DEMO_RUN.md, CHANGELOG.md - update in Phase 2
- **secondary**: glossary, CONTRIBUTING, tutorials - update in Phase 3 (optional)

---

## Canonical Flow Enumeration

Per CLAUDE.md L13 and L186-197, the seven flows are:

| # | Flow | Primary Command | Variant Commands | Artifact Dir |
|---|------|-----------------|------------------|--------------|
| 1 | Signal | /flow-1-signal | - | signal |
| 2 | Plan | /flow-2-plan | - | plan |
| 3 | Build | /flow-3-build | - | build |
| 4 | Review | /flow-4-review | /flow-4-gate | review |
| 5 | Gate | /flow-5-gate | /flow-5-deploy | gate |
| 6 | Deploy | /flow-6-deploy | /flow-6-wisdom | deploy |
| 7 | Wisdom | /flow-7-wisdom | - | wisdom |

**Total: 7 flows, 10 command files**

---

## Events/Messages

Not applicable - this is a documentation alignment task with no runtime events.

---

## Compatibility and Versioning

### Breaking Change Discipline

Documentation changes are low-risk but should follow these rules:

1. **Authoritative-first**: Always update CLAUDE.md before derived docs
2. **Consistent terminology**: Use "seven flows" (not "7 flows" in prose) per existing style
3. **Traceability**: Each documentation claim should reference its source (e.g., "per CLAUDE.md L13")

### Migration Path (OPT-003 Phases)

| Phase | Files | Dependency |
|-------|-------|------------|
| 1 | CLAUDE.md, architecture.md | None (authoritative) |
| 2 | README.md, DEMO_RUN.md, CHANGELOG.md | Phase 1 complete |
| 3 | glossary.md, CONTRIBUTING.md, work-without-github.md, walkthrough.md | Phase 2 complete (optional) |
| 4 | structure.rs test fixtures | Only if pack-check fails |

---

## Traceability Mapping

### REQ -> Interface/Entity -> Constraints

| REQ | Interface/Entity | Constraint/Validation |
|-----|------------------|----------------------|
| REQ-001 | DocumentationConsistencySpec | prohibited_patterns: "six flows" -> zero matches |
| REQ-002 | FlowVariant | variant_purpose required for all 3 variants |
| REQ-003 | Flow (flow 7) | Flow 7 must appear in enumeration with usage_note |
| REQ-004 | FlowModel | CLAUDE.md flow table shows all 7 flows |
| REQ-005 | (out of scope) | Test count is separate; no schema change |
| REQ-006 | (out of scope) | Security posture is separate; no schema change |
| REQ-007 | (out of scope) | Color coding is metadata; no schema change |

### NFR -> Validation

| NFR | Validation Method |
|-----|------------------|
| NFR-DOC-001 | `grep "six flows" <files>` returns zero matches |
| NFR-SEC-001 | Security claims reference code files with line numbers |
| NFR-TRACE-001 | `pack-check` passes after all phases complete |

---

## Error Model

Not applicable - documentation changes have no runtime errors. Validation failures are:

| Validation | Failure Condition | Resolution |
|------------|-------------------|------------|
| Prohibited pattern found | `grep "six flows"` returns matches | Edit file to replace with "seven flows" |
| Required pattern missing | Flow enumeration incomplete | Add missing flow to documentation |
| Pack-check failure | Test fixture expects "Six Flows" | Update fixture in Phase 4 |

---

## Assumptions Made to Proceed

- **ASM-IFACE-001**: Flow variants (flow-4-gate, flow-5-deploy, flow-6-wisdom) are intentional alternate entry points and should be documented as such, not consolidated or deprecated.
  - Impact if wrong: Would need to remove variant documentation; unlikely given consistent pattern across multiple flow pairs

- **ASM-IFACE-002**: The 10 command files under `.claude/commands/flow-*.md` are the complete and authoritative enumeration.
  - Impact if wrong: Additional commands would need to be added to the registry

- **ASM-IFACE-003**: Pack-check test fixtures use string literals for "Six Flows" and can be updated to "Seven Flows" without breaking test semantics.
  - Impact if wrong: Phase 4 would require deeper investigation; ASM-006 from ADR covers this

---

## Questions / Clarifications Needed

- **Q-IFACE-001**: Should the variant commands be listed inline in the CLAUDE.md flow table, or in a separate "Flow Variants" section?
  - Suggested default: Separate section to avoid cluttering the main table
  - Impact: Inline is more discoverable but makes the table wider

- **Q-IFACE-002**: Should architecture.md explain why variants exist (historical context) or just when to use them (operational guidance)?
  - Suggested default: Operational guidance only; "when to use each variant"
  - Impact: Historical context may be useful for maintainers but adds verbosity

---

## Inventory (machine countable)

- SCHEMA: FlowModel
- SCHEMA: FlowCommandRegistry
- SCHEMA: FlowVariant
- SCHEMA: DocumentationConsistencySpec
- SCHEMA: FlowArtifactPath
- ENTITY: Flow
- ENTITY: FlowCommand
- ENTITY: DocumentationFile
- ENDPOINT: (none - documentation-only task)
- EVENT: (none - documentation-only task)
- MIGRATION: (none - no database changes)

---

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - This is a documentation-only task; api_contracts.yaml defines schemas for validation, not HTTP endpoints
  - Pack-check test fixtures may need update in Phase 4 if "Six Flows" is a string assertion
