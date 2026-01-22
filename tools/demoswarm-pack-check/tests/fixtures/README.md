# Test Fixtures for pack-check

This directory contains test fixtures for pack-check validation rules.

## Required Fields

### Build Receipt (`build_receipt_*.json`)

| Field           | Type   | Required | Description                                           |
| --------------- | ------ | -------- | ----------------------------------------------------- |
| `run_id`        | string | Yes      | Unique identifier for the run                         |
| `flow`          | string | Yes      | Flow name (signal, plan, build, gate, deploy, wisdom) |
| `status`        | enum   | Yes      | One of: VERIFIED, UNVERIFIED, CANNOT_PROCEED          |
| `counts`        | object | Yes      | Mechanical counts from the flow                       |
| `quality_gates` | object | Yes      | Quality gate results                                  |
| `timestamp`     | string | Yes      | ISO8601 timestamp                                     |

## Valid Status Values

The `status` field must be one of these canonical values:

| Value            | Meaning                                              |
| ---------------- | ---------------------------------------------------- |
| `VERIFIED`       | Flow completed successfully, all gates passed        |
| `UNVERIFIED`     | Flow completed with gaps or uncertainties documented |
| `CANNOT_PROCEED` | Mechanical failure (I/O, permissions, tooling)       |

These values are shared across all flow receipts per the pack contract.

## Cross-Flow Expectations

### Build-to-Gate Handshake

When Flow 3 (Build) produces a receipt, Flow 4 (Gate) expects:

1. **Required receipt location**: `.runs/<run-id>/build/build_receipt.json`
2. **Status validation**: Gate agents verify the status is one of the canonical values
3. **Counts integrity**: Gate verifies counts are mechanical (not estimated)
4. **Quality gates**: Gate uses `quality_gates` to determine merge eligibility

### Receipt Validation Rules

1. Missing `run_id` is a validation error
2. Invalid `status` value is a validation error
3. Empty `counts` object is valid (minimal receipt)
4. `timestamp` must be ISO8601 format

## Fixture Files

### Build Receipt Fixtures (REQ-004)

| File                                | Purpose                | Expected Validation |
| ----------------------------------- | ---------------------- | ------------------- |
| `build_receipt_valid.json`          | Valid complete receipt | PASS                |
| `build_receipt_invalid.json`        | Invalid status value   | FAIL                |
| `build_receipt_missing_run_id.json` | Missing required field | FAIL                |

### Flow Command Fixtures (REQ-001)

| File                               | Purpose                 | Expected Validation |
| ---------------------------------- | ----------------------- | ------------------- |
| `flow_command_clean.md`            | No violations           | PASS                |
| `flow_command_violation.md`        | Contains demoswarm.sh   | WARN/FAIL           |
| `flow_command_skill_subcommand.md` | Contains skill CLI refs | WARN/FAIL           |
| `flow_command_prose_count.md`      | Prose context only      | PASS                |

### Agent Fixtures (REQ-002)

| File                       | Purpose                     | Expected Validation |
| -------------------------- | --------------------------- | ------------------- |
| `agent_with_skills.md`     | Has demoswarm.sh + Skills   | PASS                |
| `agent_without_skills.md`  | Has demoswarm.sh, no Skills | WARN/FAIL           |
| `agent_no_demoswarm.md`    | No demoswarm.sh             | PASS                |
| `agent_skill_tool_only.md` | Uses Skill() tool only      | PASS                |

### OpenQ Fixtures (REQ-003)

| File                            | Purpose                  | Expected Validation |
| ------------------------------- | ------------------------ | ------------------- |
| `open_questions_valid.md`       | Valid canonical QIDs     | PASS                |
| `open_questions_invalid.md`     | Non-canonical flow codes | WARN/FAIL           |
| `open_questions_bad_padding.md` | Invalid numeric suffixes | WARN/FAIL           |
| `open_questions_mixed.md`       | Mix of valid and invalid | Multi-match test    |

## Canonical OpenQ Flow Codes

Per `stable-markers.md` and `openq-tools/SKILL.md`:

| Canonical | Flow            |
| --------- | --------------- |
| SIG       | Signal (Flow 1) |
| PLAN      | Plan (Flow 2)   |
| BUILD     | Build (Flow 3)  |
| REVIEW    | Review (Flow 4) |
| GATE      | Gate (Flow 5)   |
| DEPLOY    | Deploy (Flow 6) |
| WISDOM    | Wisdom (Flow 7) |

Non-canonical codes that should be flagged:

- PLN (use PLAN)
- BLD (use BUILD)
- REV (use REVIEW)
- GAT (use GATE)
- DEP (use DEPLOY)
- WIS (use WISDOM)

## Security Notes

All fixtures use synthetic/obviously fake values:

- No real tokens or credentials
- No real file paths
- No real run IDs or issue numbers
- Values are clearly test data (e.g., "test-run-001")
