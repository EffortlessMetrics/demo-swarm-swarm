# Interface Schema: Compliance Drift-Proofing

## Overview

This document defines the data schemas, validation contracts, and interface boundaries for the compliance drift-proofing enhancement to pack-check.

### System Boundary

```
+------------------+     +-------------------+     +------------------+
|   Flow Commands  |     |   Agent Prompts   |     |   Skill Docs     |
|   flow-*.md      | --> |   agents/*.md     | --> |   skills/SKILL.md|
+------------------+     +-------------------+     +------------------+
        ^                        ^                        ^
        |                        |                        |
        +------------------------+------------------------+
                                 |
                    +------------v------------+
                    |      pack-check         |
                    |  (Rust validation tool) |
                    +-------------------------+
                              |
              +---------------+----------------+
              |               |                |
        Check 49        Check 50         Check 51
       (existing)      (new: REQ-001)   (new: REQ-003)
       Skills sec.     Flow boundary    OpenQ prefix
```

### Interface List

| Interface | Type | Direction | Contract Location |
|-----------|------|-----------|-------------------|
| CLI invocation | External | In | `api_contracts.yaml` x-cli-interface |
| JSON report output | External | Out | `api_contracts.yaml` RunReport schema |
| Text diagnostic output | External | Out | `api_contracts.yaml` x-error-model |
| Build receipt validation | Internal | In | `api_contracts.yaml` BuildReceipt schema |
| Constants (validation patterns) | Internal | N/A | `contracts.rs` |

---

## Data Models

### Entity: CheckResult

Internal representation of a single validation check result.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| check_id | u32 | 1-99 | Unique check identifier |
| title | String | non-empty | Human-readable check description |
| level | Level | {Pass, Warn, Fail} | Result severity |
| message | String | non-empty | Diagnostic message |
| file_path | Option<String> | repo-root-relative | Source file (if file-specific) |
| line_no | Option<usize> | >= 1 | Line number (if line-specific) |

**Invariants:**
- If `level == Pass`, diagnostic is not included in JSON output
- `file_path` and `line_no` are both present or both absent
- Check IDs are stable across versions (no renumbering)

### Entity: RunReport (JSON Output)

Machine-readable validation report.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| schema_version | u32 | const 1 | Report format version |
| repo_root | String | absolute path | Validated repository root |
| errors | usize | >= 0 | Count of Fail-level diagnostics |
| warnings | usize | >= 0 | Count of Warn-level diagnostics |
| counts | PackCounts | non-null | Pack content counts |
| diagnostics | Vec<Diagnostic> | Pass filtered | Non-pass diagnostics |

**Invariants:**
- `errors == diagnostics.iter().filter(|d| d.level == Fail).count()`
- `warnings == diagnostics.iter().filter(|d| d.level == Warn).count()`
- `schema_version` increments only on breaking changes

### Entity: BuildReceipt (Validation Target)

JSON structure for Build flow receipts, validated by receipt-checker and pack-check test fixtures.

| Field | Type | Required | Constraints |
|-------|------|----------|-------------|
| run_id | String | Yes | Non-empty |
| flow | String | Yes | Must be "build" |
| status | String | Yes | VERIFIED, UNVERIFIED, CANNOT_PROCEED |
| recommended_action | String | Yes | PROCEED, RERUN, BOUNCE, FIX_ENV |
| route_to_flow | int/null | Yes | 1-6 or null |
| route_to_agent | String/null | Yes | Agent name or null |
| missing_required | Array<String> | Yes | May be empty |
| blockers | Array<String> | Yes | May be empty |
| completed_at | String | Yes | ISO8601 format |
| tests | Object | Yes | Test grounding (see below) |
| critic_verdicts | Object | Yes | Critic results (see below) |

**tests sub-object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| canonical_summary | String | Yes | Pytest summary line |
| summary_source | String | Yes | Source file reference |
| metrics_binding | String | Yes | Binding identifier |
| passed | int | No | Test count |
| failed | int | No | Test count |
| skipped | int | No | Test count |

**critic_verdicts sub-object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| test_critic | String/null | No | VERIFIED/UNVERIFIED/CANNOT_PROCEED/null |
| code_critic | String/null | No | VERIFIED/UNVERIFIED/CANNOT_PROCEED/null |

**Invariants:**
- If `recommended_action != BOUNCE`, then `route_to_flow` and `route_to_agent` must be null
- `metrics_binding` must not be a placeholder (no `<...>` tokens)
- `completed_at` must be valid ISO8601

### Entity: OpenQId

Canonical Open Question identifier.

| Component | Pattern | Description |
|-----------|---------|-------------|
| Prefix | `OQ-` | Fixed prefix |
| FlowCode | `SIG\|PLN\|BLD\|GAT\|DEP\|WIS` | Canonical flow abbreviation |
| Separator | `-` | Dash |
| Sequence | `[0-9]{3}` | Zero-padded 001-999 |

**Full Pattern:** `^OQ-(SIG|PLN|BLD|GAT|DEP|WIS)-[0-9]{3}$`

**Valid Examples:**
- `OQ-SIG-001`
- `OQ-PLN-004`
- `OQ-BLD-012`

**Invalid Examples:**
- `OQ-PLAN-001` (wrong flow code)
- `OQ-SIG-1` (not zero-padded)
- `OQ-SIG-1234` (too many digits)

---

## Events / Messages

No async events are introduced by this change. pack-check is a synchronous CLI tool with deterministic output.

---

## Compatibility and Versioning

### Backward Compatibility Guarantees

1. **Exit Codes**: Existing exit code semantics are preserved:
   - 0 = success (or success with warnings when not strict)
   - 1 = failure
   - 2 = runtime error

2. **JSON Schema**: `schema_version: 1` is maintained. New fields may be added but not removed or renamed.

3. **Check IDs**: Existing check IDs (1-49) are stable. New checks use IDs 50+.

4. **CLI Flags**: `--strict_warnings` already exists and is verified to work per REQ-005.

### Breaking Change Policy

If breaking changes become necessary:

1. Increment `schema_version` in JSON output
2. Document migration path in release notes
3. Consider --legacy flag for transition period

### Versioning Discipline

| Change Type | Version Impact | Example |
|-------------|----------------|---------|
| New check added | Patch | Check 50 added |
| New JSON field | Patch | New optional field |
| Check behavior modified | Minor | Check 49 pattern expanded |
| JSON field removed | Major | Requires schema_version bump |
| Exit code semantics changed | Major | Would break CI |

---

## Constants (contracts.rs)

### OPENQ_FLOW_CODES

**Purpose:** Canonical flow code abbreviations for OpenQ ID validation (REQ-003).

**Source of truth:** `.claude/skills/openq-tools/SKILL.md`

```rust
/// Canonical OpenQ flow codes (REQ-003, NFR-MAINT-001).
/// Source: .claude/skills/openq-tools/SKILL.md
pub const OPENQ_FLOW_CODES: &[&str] = &[
    "SIG",  // Signal (Flow 1)
    "PLN",  // Plan (Flow 2)
    "BLD",  // Build (Flow 3)
    "GAT",  // Gate (Flow 4)
    "DEP",  // Deploy (Flow 5)
    "WIS",  // Wisdom (Flow 6)
];
```

**Update Process:** When adding a new flow:
1. Update `.claude/skills/openq-tools/SKILL.md`
2. Update `contracts.rs` OPENQ_FLOW_CODES
3. No other files require changes (per NFR-MAINT-001 MET-3)

### SKILL_CLI_SUBCOMMANDS

**Purpose:** Skill-layer CLI subcommands that should not appear in flow commands (REQ-001).

**Source of truth:** `.claude/scripts/demoswarm.sh` and CLAUDE.md CLI Tooling Surface section.

```rust
/// Skill CLI subcommands (REQ-001, NFR-MAINT-001).
/// These should not appear in flow commands (boundary violation).
pub const SKILL_CLI_SUBCOMMANDS: &[&str] = &[
    "count",     // runs-derive: counting
    "ms",        // runs-derive: Machine Summary extraction
    "yaml",      // runs-derive: YAML extraction
    "index",     // runs-index: index.json updates
    "receipt",   // runs-derive: receipt reading
    "receipts",  // runs-derive: receipt counting
    "openapi",   // runs-derive: OpenAPI path counting
    "line",      // runs-derive: line extraction
    "inv",       // runs-derive: inventory marker extraction
    "time",      // runs-derive: timestamp generation
    "openq",     // openq-tools: open question management
    "secrets",   // secrets-tools: secrets scanning
];
```

**Update Process:** When adding a new skill subcommand:
1. Update `tools/demoswarm-runs-tools/` (implementation)
2. Update `contracts.rs` SKILL_CLI_SUBCOMMANDS
3. No other files require changes (per NFR-MAINT-001 MET-3)

---

## Traceability Mapping

### Requirements to Interfaces

| REQ/NFR | Interface Element | Validation Point |
|---------|-------------------|------------------|
| REQ-001 | Check 50 | Flow commands scanned for demoswarm.sh and subcommands |
| REQ-001 | SKILL_CLI_SUBCOMMANDS | Pattern list in contracts.rs |
| REQ-002 | Check 49 (existing) | Agents with demoswarm.sh checked for ## Skills |
| REQ-003 | Check 51 | open_questions.md files scanned for QID patterns |
| REQ-003 | OPENQ_FLOW_CODES | Valid flow codes in contracts.rs |
| REQ-004 | BuildReceipt schema | Test fixtures validate against schema |
| REQ-005 | --strict_warnings flag | Exit code behavior verified |
| REQ-006 | All checks | Baseline validation before rule introduction |
| NFR-PERF-001 | All checks | Pattern matching is O(n) |
| NFR-REL-001 | RunReport | Deterministic output, sorted diagnostics |
| NFR-OPS-001 | CheckDiagnostic | check_id, file_path, line_no, message |
| NFR-COMP-001 | Exit codes | 0/1/2 semantics preserved |
| NFR-SEC-001 | Diagnostic format | Only paths and rule IDs, no file contents |
| NFR-MAINT-001 | OPENQ_FLOW_CODES, SKILL_CLI_SUBCOMMANDS | Constants in contracts.rs |

### Check to Error Code Mapping

| Check ID | Error Code | Message Pattern |
|----------|------------|-----------------|
| 49 | E049 | "Agents using demoswarm.sh must have a ## Skills section" |
| 50 | W050/E050 | "Flow command contains [demoswarm.sh\|skill CLI subcommand]" |
| 51 | W051/E051 | "Non-canonical OpenQ ID: [pattern description]" |

Note: W = warning (default), E = error (with --strict_warnings)

---

## Assumptions Made to Proceed

1. **ASM-IFACE-001**: The existing `--strict_warnings` flag semantics (elevate warnings to errors for exit code) match REQ-005 requirements exactly. Impact if wrong: CLI behavior adjustment needed.

2. **ASM-IFACE-002**: PLN/BLD abbreviations (per openq-tools/SKILL.md) are canonical, not PLAN/BUILD (per stable-markers.md). Impact if wrong: Update openq-tools Rust code and existing QIDs.

3. **ASM-IFACE-003**: Check 49 fully addresses REQ-002 without modification. Impact if wrong: Check 49 logic needs enhancement (minor effort).

4. **ASM-IFACE-004**: Build receipt JSON schema matches receipt-checker.md expectations. Impact if wrong: Test fixtures need adjustment.

---

## Questions / Clarifications Needed

1. **OQ-IFACE-001**: Should Check 50 (flow boundary) flag skill subcommands appearing in prose descriptions (e.g., "the agent uses `count`"), or only in actual CLI invocation contexts? **Suggested default:** Only CLI invocation contexts (code blocks, lines with `bash` or `demoswarm`).

2. **OQ-IFACE-002**: Should invalid OpenQ IDs in historical runs (before rule introduction) be grandfathered, or should they produce warnings? **Suggested default:** Produce warnings; --strict would fail them.

---

## Inventory (machine countable)

- ENDPOINT: CLI pack-check
- ENDPOINT: CLI pack-check --strict
- ENDPOINT: CLI pack-check --format json
- SCHEMA: CheckDiagnostic
- SCHEMA: RunReport
- SCHEMA: PackCounts
- SCHEMA: BuildReceipt
- SCHEMA: OpenQId
- ENTITY: CheckResult
- ENTITY: RunReport
- ENTITY: BuildReceipt
- ENTITY: OpenQId

---

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - ASM-IFACE-002: PLN vs PLAN prefix resolution not yet formally documented in stable-markers.md
  - OQ-IFACE-001: Skill subcommand detection scope (CLI only vs prose) needs implementation decision
```
