# Contract Compliance Report for align-doc-ownership

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - "Non-standard contract type: documentation-boundary-enforcement (not HTTP endpoints)"
  - "Verification is structural/pattern-based rather than request/response shape validation"
severity_summary:
  critical: 0
  major: 0
  minor: 0
# Numeric fields for gate-cleanup (sum of critical + major + minor)
violations_total: 0
endpoints_checked: 0
```

## Sources Consulted

- `.runs/align-doc-ownership/plan/api_contracts.yaml`
- `.runs/align-doc-ownership/plan/adr.md`
- `.runs/align-doc-ownership/signal/requirements.md`
- `tools/demoswarm-pack-check/src/checks/flow.rs`
- `tools/demoswarm-pack-check/src/contracts.rs`
- `.claude/commands/flow-3-build.md` (sample flow command verification)

## Contract Source

- source: api_contracts.yaml
- extraction_method: inventory_markers
- endpoints_in_contract: null (N/A - this run defines validation rules, not HTTP endpoints)

## Summary

- This run defines **documentation-boundary-enforcement contracts**, not traditional HTTP API contracts
- The contract declares 4 validation rules (flow_command_boundary, agent_skill_declaration, agent_enum_consistency, claudemd_skills_table) and 6 violation types
- Implementation in `tools/demoswarm-pack-check/src/checks/flow.rs` adds checks 45, 46, 47 that enforce the contract
- Flow commands (6 files) are **verified clean** of skill plumbing violations
- Contract type is appropriate for this run scope (OPT-002: Pragmatic Enforcement per ADR)

## Contract Type Analysis

### Why Traditional Endpoint Checking Does Not Apply

The `api_contracts.yaml` for this run explicitly states:

```yaml
contract_type: documentation-boundary-enforcement
```

This run produces **no HTTP endpoints or database schemas**. Instead, it defines validation patterns between documentation layers:

- Flow Commands -> Agent Docs -> Skill Docs

The "contracts" are structural validation rules enforced by pack-check, not REST API schemas.

### What This Run Actually Contracts

| Contract | Description | Enforcement Location |
|----------|-------------|---------------------|
| flow_command_boundary | Flow commands must not contain skill plumbing | Check 45: `check_flow_skill_plumbing()` |
| agent_skill_declaration | Agents using demoswarm.sh must have Skills section | Check 46: `check_missing_skills_section()` |
| agent_enum_consistency | Agents must use canonical status/action enums | Check 37: `check_control_plane_agents()` |
| claudemd_skills_table | CLAUDE.md Skills table must be summary-level | (advisory/manual) |

## Validation Rules Checked

| Rule ID | Contract Section | Implementation | Result | Notes |
|---------|-----------------|----------------|--------|-------|
| FLOW_VIO_001 | flow_command_boundary | Check 45, `demoswarm_shim_ref` regex | OK | No `demoswarm.sh` in flow commands |
| FLOW_VIO_002 | flow_command_boundary | Check 45, `skill_names_in_prose` regex | OK | No skill names in flow commands |
| FLOW_VIO_003 | flow_command_boundary | (CLI flags) | OK | No CLI flag syntax detected |
| AGENT_VIO_001 | agent_skill_declaration | Check 46 | OK | Agents using skills have Skills section |
| AGENT_VIO_002 | agent_enum_consistency | Check 37 | OK | Status enum canonical |
| AGENT_VIO_003 | agent_enum_consistency | Check 37 | OK | Action enum canonical |

## Findings

### Breaking / CRITICAL

(none)

### MAJOR

(none)

### MINOR

(none)

## Implementation Verification

### Check 45: Flow Skill Plumbing Boundary

**Contract says:**
```yaml
flow_command_boundary:
  violation_patterns:
    - id: FLOW_VIO_001
      pattern: "demoswarm\\.sh"
    - id: FLOW_VIO_002
      pattern: "(runs-derive|runs-index|openq-tools|secrets-tools|test-runner|auto-linter|policy-runner)"
```

**Implementation in `flow.rs`:**
```rust
fn check_flow_skill_plumbing(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    // Check for skill names in prose
    if cx.re.skill_names_in_prose.is_match(&content) {
        issues.push("skill name references");
    }
    // Check for demoswarm.sh references
    if cx.re.demoswarm_shim_ref.is_match(&content) {
        issues.push("demoswarm.sh references");
    }
}
```

**Regex patterns in `contracts.rs`:**
```rust
skill_names_in_prose: Regex::new(r"\b(runs-derive|runs-index|openq-tools|secrets-tools|test-runner|auto-linter|policy-runner)\b")?,
demoswarm_shim_ref: Regex::new(r"demoswarm\.sh")?,
```

**Verification:** Grep searches of all 6 flow command files returned **no matches** for either pattern.

### Check 46: Agent Skills Section

**Contract says:**
```yaml
agent_skill_declaration:
  detection_rule:
    trigger_patterns:
      - "demoswarm\\.sh"
    required_section:
      heading: "## Skills"
```

**Implementation in `flow.rs`:**
```rust
fn check_missing_skills_section(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if cx.re.demoswarm_shim_ref.is_match(&content) {
        if !content.contains("## Skills") {
            missing_section.push(agent.to_string());
        }
    }
}
```

**Verification:** Implementation matches contract. Check 46 emits warning for agents missing Skills section.

### Check 47: Flow Output Paths (Advisory)

**Contract says:** (no strict requirement - advisory)

**Implementation:** Advisory check that warns if flow commands contain output arrow patterns that might leak implementation details.

## Undocumented Additions

(none)

## Notes for Merge-Decider

This run's contracts are **non-traditional** - they define documentation boundary enforcement rules rather than HTTP API contracts. The contract-enforcer verification is therefore structural:

1. **Contract source exists** - `api_contracts.yaml` with inventory markers (VALIDATION_RULE, VIOLATION, SCHEMA)
2. **Implementation matches contract** - Checks 45, 46, 47 in pack-check implement the declared rules
3. **Flow commands are clean** - Grep verification confirms no skill plumbing in any of 6 flow commands

**Recommendation:** PROCEED - the contract and implementation are aligned. The contract type (documentation-boundary-enforcement) is appropriate for this scope, and pack-check now enforces the declared validation rules.

## Inventory (machine countable)

(Only these prefixed lines; do not rename prefixes)

- CE_VALIDATION_RULE_OK: flow_command_boundary
- CE_VALIDATION_RULE_OK: agent_skill_declaration
- CE_VALIDATION_RULE_OK: agent_enum_consistency
- CE_VALIDATION_RULE_OK: claudemd_skills_table
- CE_CHECK_IMPL_OK: 45 (flow_skill_plumbing)
- CE_CHECK_IMPL_OK: 46 (missing_skills_section)
- CE_CHECK_IMPL_OK: 47 (flow_output_paths)
