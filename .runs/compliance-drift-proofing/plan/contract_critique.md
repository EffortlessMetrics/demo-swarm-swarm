# Contract Critique for compliance-drift-proofing

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - OQ-IFACE-001 and OQ-IFACE-002 in schema.md are implementation decisions, not contract blockers
  - ASM-IFACE-002 (PLN vs PLAN prefix) carried forward; contracts assume PLN/BLD canonical
```

## Iteration Control

```yaml
can_further_iteration_help: no
rationale: "Contracts are coherent, complete for implementation scope, and testable. Remaining concerns (prefix resolution, prose vs CLI detection) are implementation decisions documented as open questions, not contract gaps."
```

## Metrics

```yaml
severity_summary:
  critical: 0
  major: 0
  minor: 3
```

## Summary (1-5 bullets)

- api_contracts.yaml parses correctly and includes CONTRACT_INVENTORY_V1 header with 4 ENDPOINT and 4 SCHEMA markers
- schema.md is well-structured with complete entity definitions, invariants, and traceability mapping
- All 6 REQ identifiers (REQ-001 through REQ-006) are traced in both contracts
- All 6 NFR identifiers have traceability entries in schema.md
- Error model is consistent: Level enum (pass/warn/fail) applies uniformly across CheckDiagnostic output

## Critical Issues

(None)

## Major Issues

(None)

## Minor Issues

- [MINOR] CC-MIN-001: Exit code semantics clarification - api_contracts.yaml x-cli-interface exit_codes section shows 0=SUCCESS, 1=FAILURE, 2=RUNTIME_ERROR, but OQ-PLN-002 in ADR suggests "exit 2 for warnings with --strict" as a possible alternative. The contract correctly defines the current behavior; the open question is documented but resolved by the contract (warnings with --strict elevate to exit 1, not 2). This is informational only.

- [MINOR] CC-MIN-002: BuildReceipt.tests sub-object required fields inconsistency - schema.md entity table marks `tests` as Required=Yes, but within the tests sub-object, passed/failed/skipped are marked Required=No. This is intentional flexibility (metrics may not always be available), but the asymmetry could confuse implementers. Documentation is adequate.

- [MINOR] CC-MIN-003: Context filter prose vs code detection - Check 50 x-checks definition includes a `context_filter` note indicating skill subcommands should only be flagged in CLI invocation contexts, not prose. This is implementation guidance, not a formal contract. OQ-IFACE-001 in schema.md captures this as an open question with a suggested default. Contract coverage is adequate.

## Traceability Gaps

(None - all REQ and NFR identifiers from requirements.md are present in contract traceability sections)

Verified coverage:
- REQ-001: api_contracts.yaml x-traceability.REQ-001, x-checks.new.50
- REQ-002: api_contracts.yaml x-traceability.REQ-002, x-checks.existing.49
- REQ-003: api_contracts.yaml x-traceability.REQ-003, x-checks.new.51, OpenQId schema
- REQ-004: api_contracts.yaml x-traceability.REQ-004, BuildReceipt schema
- REQ-005: api_contracts.yaml x-traceability.REQ-005, x-cli-interface.arguments.strict_warnings
- REQ-006: api_contracts.yaml x-traceability.REQ-006 (baseline validation)
- NFR-PERF-001: schema.md traceability row
- NFR-REL-001: schema.md traceability row (deterministic output)
- NFR-OPS-001: schema.md traceability row (diagnostic format)
- NFR-COMP-001: schema.md traceability row (exit codes, backward compat)
- NFR-SEC-001: schema.md traceability row (no secrets in output)
- NFR-MAINT-001: schema.md traceability row, api_contracts.yaml x-constants

## Questions for Humans

- Q1: OQ-IFACE-001 (prose vs CLI context detection for skill subcommands) has a suggested default in schema.md. The contract does not prescribe implementation details, leaving this to Build. Is the suggested default (only CLI invocation contexts) acceptable, or should the contract be more prescriptive?

- Q2: OQ-IFACE-002 (historical OpenQ IDs grandfathering) suggests warnings for invalid patterns in historical runs. The contract does not specify exemption handling. Should the contract include an explicit exemption mechanism, or is warning-first mode sufficient for migration?

## Inventory (machine countable)

- CC_MINOR: CC-MIN-001
- CC_MINOR: CC-MIN-002
- CC_MINOR: CC-MIN-003

---

## Detailed Validation

### 1) Handshake Validity

**api_contracts.yaml**
- Parses as valid YAML: PASS
- Contains `# CONTRACT_INVENTORY_V1` header at line 1: PASS
- Inventory markers present:
  - `# ENDPOINT: CLI pack-check --strict` (line 2)
  - `# ENDPOINT: CLI pack-check --format json` (line 3)
  - `# SCHEMA: CheckDiagnostic` (line 4)
  - `# SCHEMA: RunReport` (line 5)
  - `# SCHEMA: BuildReceipt` (line 6)
  - `# SCHEMA: OpenQId` (line 7)

**schema.md**
- Contains `## Inventory (machine countable)` section: PASS
- Inventory prefixes present:
  - 3 ENDPOINT markers
  - 4 SCHEMA markers
  - 4 ENTITY markers

### 2) Contract Surface Completeness

**CLI Interface (x-cli-interface)**
- Arguments defined: repo_root, format, no_color, strict_warnings - COMPLETE
- Exit codes defined: 0 (SUCCESS), 1 (FAILURE), 2 (RUNTIME_ERROR) - COMPLETE
- Exit code semantics for --strict_warnings documented - COMPLETE

**Schemas (components/schemas)**
- Level enum: pass, warn, fail - COMPLETE
- CheckDiagnostic: level, check_id, check_title, message - COMPLETE with required markers
- PackCounts: agents, commands, skills - COMPLETE
- RunReport: schema_version, repo_root, errors, warnings, counts, diagnostics - COMPLETE
- BuildReceipt: Extensive schema with run_id, flow, status, recommended_action, route_to_flow, route_to_agent, missing_required, blockers, completed_at, tests, critic_verdicts, counts - COMPLETE
- OpenQId: String pattern ^OQ-(SIG|PLN|BLD|GAT|DEP|WIS)-[0-9]{3}$ - COMPLETE with examples

**Error Model (x-error-model)**
- Text format: symbol + message per level - COMPLETE
- File location format: rel_path:line_number:line_content - COMPLETE
- JSON provides structured diagnostics - COMPLETE

**Check Definitions (x-checks)**
- Check 49 (existing): Skills section enforcement, severity fail - COMPLETE
- Check 50 (new): Flow boundary, severity warn/strict_severity fail, patterns defined - COMPLETE
- Check 51 (new): OpenQ prefix, severity warn/strict_severity fail, patterns defined - COMPLETE

**Constants (x-constants)**
- FLOW_CODES: 6 codes defined (SIG, PLN, BLD, GAT, DEP, WIS) with Rust constant - COMPLETE
- SKILL_SUBCOMMANDS: 12 subcommands defined with Rust constant - COMPLETE

### 3) Versioning + Compatibility Discipline

- schema_version: 1 in RunReport (line 289-291) - COMPLETE
- Versioning discipline table in schema.md (lines 177-183) - COMPLETE
- Breaking change policy documented (lines 169-173) - COMPLETE
- Check IDs stable: existing IDs 1-49, new IDs 50+ - COMPLETE

### 4) Data Model Coherence

- No database changes implied by this run (CLI tool enhancement)
- No migrations directory present: EXPECTED (confirmed no SQL migrations needed)
- schema.md entities (CheckResult, RunReport, BuildReceipt, OpenQId) have invariants documented - COMPLETE

### 5) Traceability + Testability Bindings

**api_contracts.yaml x-traceability section**
- REQ-001 -> check_id 50, validation_points defined
- REQ-002 -> check_id 49, validation_points defined
- REQ-003 -> check_id 51, validation_points defined
- REQ-004 -> BuildReceipt schema, validation_points defined
- REQ-005 -> --strict_warnings flag, validation_points defined
- NFR-MAINT-001 -> contracts.rs constants

**schema.md Requirements to Interfaces table**
- All REQ/NFR identifiers mapped to interface elements and validation points

**test_plan.md references contract surfaces**
- 40 scenarios across 6 feature files
- Contract Test Plan section references:
  - Receipt schema assertions
  - Exit code contract
  - Diagnostic output contract
- Coverage summary shows all 6 REQ and 6 NFR covered
