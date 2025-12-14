# Test Changes Summary for align-doc-ownership

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
tests_added: 3
reqs_covered: [REQ-001, REQ-002]
scenarios_covered: 4
```

## Overview

This document defines the test cases for the new pack-check boundary enforcement rules (Checks 45-47). These are specification-level tests; execution requires Rust implementation in ST-004.

## New Pack-Check Rules

### Check 45: Flow Skill Plumbing (ERROR)

**Purpose**: Enforce REQ-001 - Flow commands must contain no skill plumbing.

**Detection patterns**:
- Skill names: `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`, `test-runner`, `auto-linter`, `policy-runner`
- CLI shim: `demoswarm.sh`
- CLI flags in skill context: `--file`, `--prefix`, `--run-id`, `--format`, `--output`

**Test cases**:

| ID | Input | Expected | Rationale |
|----|-------|----------|-----------|
| TC-45-001 | "Computes counts mechanically (via runs-derive skill)" | FAIL | Skill name in flow |
| TC-45-002 | "Computes counts mechanically (never estimates)" | PASS | Invariant wording |
| TC-45-003 | "bash .claude/scripts/demoswarm.sh count" | FAIL | CLI shim in flow |
| TC-45-004 | "Call cleanup agent to finalize receipt" | PASS | Agent reference only |
| TC-45-005 | "Use runs-derive to count markers" | FAIL | Direct skill reference |

**Current violations** (to be fixed in ST-001 through ST-006):
- `flow-1-signal.md`: "via runs-derive skill"
- `flow-2-plan.md`: "via runs-derive skill"
- `flow-3-build.md`: "via runs-derive skill"
- `flow-4-gate.md`: "via runs-derive skill"
- `flow-5-deploy.md`: "via runs-derive skill"

### Check 46: Missing Skills Section (WARNING)

**Purpose**: Enforce REQ-002/AC-3 - Agents using skills must declare them.

**Detection logic**:
- Scan agent docs for `demoswarm.sh` invocation
- Check if `## Skills` section exists
- WARN if invocation found without Skills section

**Test cases**:

| ID | Input | Expected | Rationale |
|----|-------|----------|-----------|
| TC-46-001 | Agent with `demoswarm.sh` and `## Skills` | PASS | Properly declared |
| TC-46-002 | Agent with `demoswarm.sh` but no Skills section | WARN | Undeclared usage |
| TC-46-003 | Agent without any skill usage | PASS | N/A |

**Current state**: All 8 cleanup agents + clarifier already have Skills sections (compliant).

### Check 47: Flow Output Paths (WARNING)

**Purpose**: Advisory check for agent-contract leakage in flow step lists.

**Detection patterns**:
- `agent-name -> .runs/`
- Agent name followed by arrow and file path

**Test cases**:

| ID | Input | Expected | Rationale |
|----|-------|----------|-----------|
| TC-47-001 | "work-planner -> subtasks.yaml" | WARN | Output path in flow |
| TC-47-002 | "work-planner -> Produce work breakdown" | PASS | Purpose only |
| TC-47-003 | "cleanup -> *_receipt.json" | WARN | Output path in flow |

**Severity**: WARNING (not ERROR) because this is advisory for maintainability, not a hard boundary violation.

## Negative Tests (for validation after implementation)

- **NEG-001**: Introduce skill name in flow-1-signal.md, verify Check 45 fails
- **NEG-002**: Remove Skills section from signal-cleanup.md, verify Check 46 warns
- **NEG-003**: Add arrow path to flow step, verify Check 47 warns

## Requirements Traceability

| Requirement | Test Coverage |
|-------------|---------------|
| REQ-001/AC-1 | Check 45 (skill names) |
| REQ-001/AC-2 | Check 45 (CLI flags) |
| REQ-002/AC-3 | Check 46 (Skills section) |
| REQ-002/AC-5 | Check 46 + doc-drift |

## Inventory
- TEST_CASE: TC-45-001
- TEST_CASE: TC-45-002
- TEST_CASE: TC-45-003
- TEST_CASE: TC-45-004
- TEST_CASE: TC-45-005
- TEST_CASE: TC-46-001
- TEST_CASE: TC-46-002
- TEST_CASE: TC-46-003
- TEST_CASE: TC-47-001
- TEST_CASE: TC-47-002
- TEST_CASE: TC-47-003
