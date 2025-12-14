# Schema: Documentation Ownership Contracts

## Overview

This run (align-doc-ownership) is a documentation alignment effort. There are **no HTTP APIs, database schemas, or runtime events** to define.

Instead, this schema document captures the **structural validation contracts** between documentation layers:

```
Flow Commands  -->  Agent Docs  -->  Skill Docs
(orchestration)     (behavior)       (CLI truth)
```

**System Boundary:** The pack's documentation system, specifically:
- 6 flow command files (`.claude/commands/flow-*.md`)
- 55 agent documentation files (`.claude/agents/*.md`)
- 7 skill documentation files (`.claude/skills/*/SKILL.md`)
- 1 CLAUDE.md file (repo-level policy)

**Interface List:**
1. Flow Command Boundary Enforcement (pack-check rule)
2. Agent Skill Declaration Enforcement (pack-check rule)
3. Agent Enum Consistency Enforcement (pack-check rule)
4. CLAUDE.md Skills Table Enforcement (pack-check rule)

---

## Data Models

### FlowCommandBoundaryViolation

Represents a boundary violation detected in a flow command file.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | string | Pattern: `FLOW_VIO_\d{3}` | Unique violation identifier |
| `file` | string | Repo-root-relative path | Path to violating file |
| `line` | integer | >= 0 | Line number (0 = file-level) |
| `pattern_matched` | string | Non-empty | Regex pattern that matched |
| `context` | string | Optional | Snippet of violating content |
| `severity` | enum | `error` | Always error for flow violations |

**Invariants:**
- File path must match `.claude/commands/flow-*.md`
- Pattern must be one of: `demoswarm\.sh`, skill-name patterns, CLI flag patterns

### AgentSkillDeclaration

Represents skill declaration requirements for an agent file.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `agent_file` | string | Repo-root-relative path | Path to agent doc |
| `skills_detected` | string[] | Canonical skill names | Skills referenced in agent |
| `has_skills_section` | boolean | - | Whether ## Skills section exists |
| `skills_section_format` | enum | `table`, `list`, `none` | Format of Skills section |

**Invariants:**
- If `skills_detected` is non-empty, `has_skills_section` must be true
- `skills_section_format` cannot be `none` if `has_skills_section` is true

### AgentEnumConsistency

Represents enum validation for an agent's Machine Summary.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `agent_file` | string | Repo-root-relative path | Path to agent doc |
| `status_value` | string | Must be canonical | Machine Summary status |
| `action_value` | string | Must be canonical | Machine Summary recommended_action |
| `is_valid` | boolean | - | Whether all enums are canonical |

**Canonical Values:**
- `status`: `VERIFIED`, `UNVERIFIED`, `CANNOT_PROCEED`
- `recommended_action`: `PROCEED`, `RERUN`, `BOUNCE`, `ESCALATE`, `FIX_ENV`

### ClaudeMdSkillsEntry

Represents a single skill entry in CLAUDE.md Skills table.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `skill_name` | string | Canonical skill name | Name of the skill |
| `purpose` | string | Max 120 chars, no flags | Brief purpose description |
| `line_count` | integer | Max 2 | Number of lines used |

**Invariants:**
- `purpose` must NOT contain: `--`, `demoswarm.sh`, `.claude/scripts/`
- `line_count` must be <= 2

### ViolationReport

Represents the complete output from pack-check boundary enforcement.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `violations` | Violation[] | - | Array of detected violations |
| `summary.errors` | integer | >= 0 | Count of error-level violations |
| `summary.warnings` | integer | >= 0 | Count of warning-level violations |
| `summary.files_checked` | integer | >= 0 | Number of files examined |

---

## Events/Messages

**Not Applicable.** This run does not introduce runtime events or message passing. Documentation validation is a build-time concern handled by pack-check during CI.

---

## Compatibility and Versioning

### Contract Versioning

The `api_contracts.yaml` declares `contract_version: 1.0.0`.

**Breaking Change Policy:**
- Adding new violation patterns is additive (minor version bump)
- Removing violation patterns is breaking (major version bump)
- Changing pattern semantics (e.g., making advisory rules blocking) is breaking (major version bump)

**Migration Path:**
If violation patterns change in future:
1. Document deprecation in `api_contracts.yaml`
2. Allow grace period (1 flow cycle) before removal
3. Update affected agent/flow docs during grace period

### Error Code Taxonomy

| Code Range | Domain | Description |
|------------|--------|-------------|
| `FLOW_VIO_0xx` | Flow commands | Skill plumbing in flow commands |
| `AGENT_VIO_0xx` | Agent docs | Missing Skills section, invalid enums |
| `CLAUDE_VIO_0xx` | CLAUDE.md | Skills table format violations |

**Reserved for Future:**
| Code Range | Domain | Description |
|------------|--------|-------------|
| `SKILL_VIO_0xx` | Skill docs | (Not implemented; future validation) |
| `PACK_VIO_0xx` | Pack structure | (Not implemented; future validation) |

---

## Traceability Mapping

### REQ to Contract/Violation

| Requirement | Contract | Violations | Error Codes |
|-------------|----------|------------|-------------|
| REQ-001 (Flow Command Boundary) | `flow_command_boundary` | demoswarm.sh, skill-name, CLI flags | FLOW_VIO_001, FLOW_VIO_002, FLOW_VIO_003 |
| REQ-002 (Agent Doc Consistency) | `agent_skill_declaration`, `agent_enum_consistency` | missing Skills, invalid enums | AGENT_VIO_001, AGENT_VIO_002, AGENT_VIO_003 |
| REQ-003 (Skill Doc Ownership) | `agent_skill_reference` | (advisory) | - |
| REQ-004 (CLAUDE.md Scope) | `claudemd_skills_table` | (advisory) | - |

### NFR to Constraints

| NFR | Constraint | Validation |
|-----|------------|------------|
| NFR-TEST-001-MET-1 | pack-check exits 0 | CI gate |
| NFR-TEST-001-MET-3 | Negative tests detect violations | Test files in pack-check |
| NFR-MAINT-001-MET-2 | No duplicate CLI docs | doc-drift script |

---

## Assumptions Made to Proceed

| ID | Assumption | Impact if Wrong |
|----|------------|-----------------|
| ASM-004 | pack-check can be extended with boundary checks without major refactoring | May need to fall back to shell scripts for enforcement |
| ASM-005 | Pack maintainers can exercise judgment on "minimal examples" consistently | Inconsistent application; may need to tighten to strict enforcement |
| NEW | Flow commands do not legitimately need to mention skill names in prose | If they do, FLOW_VIO_002 would produce false positives (mitigated by code_block context scoping) |
| NEW | Machine Summary format is consistent enough to parse programmatically | If not, AGENT_VIO_002/003 detection may fail |

---

## Questions / Clarifications Needed

| ID | Question | Suggested Default | Impact |
|----|----------|-------------------|--------|
| Q-SCHEMA-001 | Should CLAUDEMD violations be blocking (error) or advisory (warning)? | Advisory initially, can escalate later | If blocking, CLAUDE.md changes must be atomic with pack-check rule addition |
| Q-SCHEMA-002 | Should agent skill reference checks be automated or manual-review-only? | Manual review initially | If automated, requires content comparison between agent and skill docs |
| Q-SCHEMA-003 | What is the fallback if pack-check Rust extension fails? | Shell script implementation | Shell scripts are less maintainable but faster to implement |

---

## Inventory (machine countable)

- VALIDATION_RULE: flow_command_boundary
- VALIDATION_RULE: agent_skill_declaration
- VALIDATION_RULE: agent_enum_consistency
- VALIDATION_RULE: claudemd_skills_table
- ENTITY: FlowCommandBoundaryViolation
- ENTITY: AgentSkillDeclaration
- ENTITY: AgentEnumConsistency
- ENTITY: ClaudeMdSkillsEntry
- ENTITY: ViolationReport
- VIOLATION_CODE: FLOW_VIO_001
- VIOLATION_CODE: FLOW_VIO_002
- VIOLATION_CODE: FLOW_VIO_003
- VIOLATION_CODE: AGENT_VIO_001
- VIOLATION_CODE: AGENT_VIO_002
- VIOLATION_CODE: AGENT_VIO_003

---

## Machine Summary

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - OPT-002 pragmatic exception for agent examples requires judgment calls
  - CLAUDEMD violations advisory initially; enforcement level TBD
  - pack-check Rust implementation assumed feasible per ASM-004
```
