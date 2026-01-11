# Test Plan

## Machine Summary
status: VERIFIED
missing_required: []
blockers: []
concerns:
  - Negative test execution requires temporary patch-revert workflow (non-automated)
  - Toy Run A/B validation depends on all subtasks completing first (ST-006 dependency)
  - pack-check boundary rules are pending implementation (OQ-PLAN-001 defaults to Rust)
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

counts:
  scenarios_total: 31
  requirements_total: 7
  requirements_with_scenarios: 7

severity_summary:
  critical: 0
  major: 0
  minor: 3

## Scope

**Covered:**
- Verification that pack-check passes after alignment changes (NFR-TEST-001 MET-1)
- Verification that doc-drift script passes (NFR-TEST-001 MET-2)
- Negative testing to prove pack-check boundary rules fire on violations (NFR-TEST-001 MET-3)
- Validation run (Toy Run A/B through flows 1-4) to verify no regressions (NFR-REGR-001 MET-1)
- Manual inspection of agent doc consistency during PR review (NFR-MAINT-001)

**Not covered:**
- Unit tests (no runtime code changes; this is documentation alignment)
- Integration tests with external systems (no integrations modified)
- Performance testing (no performance-sensitive changes)
- Security testing (no security-sensitive changes)
- Mutation testing (not applicable: documentation-only changes)

## Coverage Thresholds

This is a documentation-alignment run with no runtime code changes. Traditional code coverage does not apply.

- COVERAGE_LINE_REQUIRED: null
- COVERAGE_BRANCH_REQUIRED: null
- COVERAGE_CRITICAL_PATH: null

Measurement notes:
- No runtime code is being added or modified
- pack-check Rust changes (if any) are structural checks, not business logic
- Validation is via pack-check pass/fail and doc-drift pass/fail, not code coverage

## Mutation Testing

- mutation_required: false
- mutation_threshold: null
- mutation_scope: []
- mutation_tool_hint: null
- rationale: Documentation-only run. No business logic or runtime code changes. pack-check rule additions are structural pattern matching, not complex conditional logic requiring mutation hardening.

## Test Types for This Run

This run uses non-traditional "test types" appropriate for documentation alignment:

| Test Type | Description | Tooling | When Run |
|-----------|-------------|---------|----------|
| **pack-check** | Structural validation of pack consistency | `bash .claude/scripts/pack-check.sh` | Build + Gate |
| **doc-drift** | Detection of content duplication across tiers | `scripts/check-doc-drift.sh` | Build + Gate |
| **negative-test** | Prove enforcement rules fire on known-bad input | Manual patch-verify-revert workflow | Gate (pre-merge) |
| **validation-run** | End-to-end flow execution (Toy Run A/B) | `/flow-1-signal` through `/flow-4-gate` | Post-Gate (ST-006) |

## Scenario to Test Type Matrix

| REQ | Feature File | Scenario | Priority | pack-check | doc-drift | negative-test | validation-run | Notes |
|-----|--------------|----------|----------|------------|-----------|---------------|----------------|-------|
| REQ-001 | doc-ownership.feature | Flow commands contain only orchestration content | P1 | X | | | X | Verified by boundary rules |
| REQ-001 | doc-ownership.feature | Flow commands avoid CLI flag syntax | P1 | X | | | | CLI flag detection |
| REQ-001 | doc-ownership.feature | Detection of skill plumbing triggers pack-check failure | P0 | | | X | | **Must** demonstrate failure |
| REQ-002 | doc-ownership.feature | Agent docs use canonical status enum values | P1 | X | | | | Enum validation in pack-check |
| REQ-002 | doc-ownership.feature | Agent docs use canonical recommended_action enum values | P1 | X | | | | Enum validation in pack-check |
| REQ-002 | doc-ownership.feature | Agent that invokes skills includes Skills section | P1 | X | | | | Skills section presence |
| REQ-002 | doc-ownership.feature | Agent docs with file-write rules use explicit format | P2 | | | | X | Manual review + validation run |
| REQ-002 | doc-ownership.feature | Agent docs reference skill docs for CLI details | P2 | | X | | | doc-drift catches duplicates |
| REQ-002 | doc-ownership.feature | Agent with invalid status enum fails pack-check | P0 | | | X | | **Must** demonstrate failure |
| REQ-003 | doc-ownership.feature | Skill doc contains complete CLI command reference | P1 | | | | X | Manual review + validation run |
| REQ-003 | doc-ownership.feature | Skill doc contains runnable examples | P2 | | | | X | Manual review |
| REQ-003 | doc-ownership.feature | CLI details migrate from CLAUDE.md to skill docs | P1 | | X | | | doc-drift catches duplication |
| REQ-004 | doc-ownership.feature | CLAUDE.md Skills table is summary-level only | P1 | | X | | | doc-drift detects flag syntax |
| REQ-004 | doc-ownership.feature | CLAUDE.md does not duplicate skill doc content | P1 | | X | | | doc-drift primary check |
| REQ-004 | doc-ownership.feature | CLAUDE.md references skill docs for detailed usage | P2 | | | | X | Manual review |
| REQ-004 | doc-ownership.feature | Detection of duplicated CLI flags triggers doc-drift failure | P0 | | | X | | **Must** demonstrate failure |
| REQ-005 | doc-ownership.feature | ST-001 covers Flow 1 documentation | P2 | | | | | Work plan concern, not runtime |
| REQ-005 | doc-ownership.feature | ST-002 covers Flow 2 documentation | P2 | | | | | Work plan concern |
| REQ-005 | doc-ownership.feature | ST-003 covers Flow 3 documentation | P2 | | | | | Work plan concern |
| REQ-005 | doc-ownership.feature | ST-004 covers Flow 4 plus cross-cutting | P2 | | | | | Work plan concern |
| REQ-005 | doc-ownership.feature | ST-005 covers Flow 5 documentation | P2 | | | | | Work plan concern |
| REQ-005 | doc-ownership.feature | ST-006 covers Flow 6 plus validation | P2 | | | | X | Validation run proves coverage |
| REQ-005 | doc-ownership.feature | Subtasks have distinct touches patterns | P2 | | | | | Work plan concern |
| REQ-006 | doc-ownership.feature | Validation run recorded after completion | P1 | | | | X | **Final gate for this run** |
| REQ-006 | doc-ownership.feature | Validation log entry includes required fields | P2 | | | | X | Log format verified |
| REQ-006 | doc-ownership.feature | Validation not recorded if pack-check fails | P1 | X | | | | Prerequisite chain |
| REQ-006 | doc-ownership.feature | Validation not recorded if doc-drift fails | P1 | | X | | | Prerequisite chain |
| REQ-007 | doc-ownership.feature | Moved content retains reference to new location | P1 | | | | X | PR review + validation |
| REQ-007 | doc-ownership.feature | Removed content is archived not deleted | P1 | | | | X | PR review |
| REQ-007 | doc-ownership.feature | Content moves documented in PR description | P2 | | | | | PR hygiene |
| REQ-007 | doc-ownership.feature | Direct deletion without archive is flagged | P1 | | | | | PR review |

## Requirement Coverage Summary

| Requirement | Scenarios | Priority | Required Test Types | Notes |
|-------------|-----------|----------|---------------------|-------|
| REQ-001 | 3 | P0-P1 | pack-check, negative-test | Flow command boundary enforcement |
| REQ-002 | 6 | P0-P2 | pack-check, doc-drift, negative-test, validation-run | Agent doc consistency |
| REQ-003 | 3 | P1-P2 | doc-drift, validation-run | Skill doc ownership |
| REQ-004 | 4 | P0-P2 | doc-drift, negative-test, validation-run | CLAUDE.md scope normalization |
| REQ-005 | 7 | P2 | validation-run | Subtask partitioning (work plan) |
| REQ-006 | 4 | P1-P2 | pack-check, doc-drift, validation-run | Validation run recording |
| REQ-007 | 4 | P1-P2 | validation-run | Archive-over-delete pattern |

## pack-check Test Plan

### Existing Rules (must continue to pass)
- Pack structure validation
- Agent/skill file presence
- CLAUDE.md format checks

### New Boundary Rules (ST-004 implementation)
These rules will be added to `tools/demoswarm-pack-check/` as part of ST-004:

| Rule | Detection Pattern | Error Severity |
|------|-------------------|----------------|
| Flow skill plumbing | `demoswarm\.sh`, `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`, `test-runner`, `auto-linter`, `policy-runner` in `.claude/commands/flow-*.md` | ERROR |
| Flow CLI flags | `--file`, `--prefix`, `--run-id`, `--format` in `.claude/commands/flow-*.md` | ERROR |
| Agent enum inconsistency | Status values not in `{VERIFIED, UNVERIFIED, CANNOT_PROCEED}` | ERROR |
| Agent action inconsistency | recommended_action values not in `{PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV}` | ERROR |
| Missing Skills section | Agent uses `demoswarm.sh` but lacks `## Skills` section | WARNING |

### Execution
```bash
# Human-readable output
bash .claude/scripts/pack-check.sh --no-color

# Machine-readable JSON (for CI)
bash .claude/scripts/pack-check.sh --format json
```

### Pass Criteria
- Exit code 0
- No ERROR-level findings
- WARNING-level findings documented in concerns

## doc-drift Test Plan

### Checks
The existing `scripts/check-doc-drift.sh` script checks for:
- CLI flag documentation duplicated between CLAUDE.md and skill docs
- Command syntax duplicated across documentation tiers

### Execution
```bash
scripts/check-doc-drift.sh
```

### Pass Criteria
- Exit code 0
- No duplication warnings

## Negative Test Plan

Negative tests prove that enforcement rules actually fire on violations. These are executed manually during Gate (pre-merge).

### NEG-001: Skill Plumbing in Flow Command
**Purpose:** Prove REQ-001 boundary enforcement works

**Steps:**
1. Create temporary patch to `.claude/commands/flow-1-signal.md` adding:
   ```
   bash .claude/scripts/demoswarm.sh runs-derive count bdd
   ```
2. Run `bash .claude/scripts/pack-check.sh`
3. Verify non-zero exit code
4. Verify error message identifies boundary violation
5. Revert patch

**Expected:** pack-check fails with error identifying skill plumbing in flow command

### NEG-002: Invalid Status Enum
**Purpose:** Prove REQ-002 enum enforcement works

**Steps:**
1. Create temporary patch to any agent doc adding:
   ```yaml
   status: COMPLETE
   ```
2. Run `bash .claude/scripts/pack-check.sh`
3. Verify non-zero exit code
4. Verify error message identifies non-canonical enum
5. Revert patch

**Expected:** pack-check fails with error identifying invalid status value

### NEG-003: CLI Flag Duplication
**Purpose:** Prove REQ-004 doc-drift detection works

**Steps:**
1. Create temporary patch to CLAUDE.md Skills section adding:
   ```
   | count | Count items `--file X --prefix Y` |
   ```
2. Run `scripts/check-doc-drift.sh`
3. Verify non-zero exit code
4. Verify error identifies duplication
5. Revert patch

**Expected:** doc-drift fails with error identifying CLI flag duplication

### Negative Test Prerequisites
- pack-check boundary rules implemented (ST-004 dependency)
- doc-drift script functional

## Validation Run Plan

### Purpose
Prove that alignment changes do not regress existing flow behavior (NFR-REGR-001).

### Execution (ST-006)
After all subtasks (ST-001 through ST-005) complete and pack-check + doc-drift pass:

1. **Toy Run A:** Execute `/flow-1-signal` with a synthetic feature request through `/flow-4-gate`
2. **Toy Run B:** Execute `/flow-1-signal` with a different synthetic feature request through `/flow-4-gate`

### Pass Criteria
- Both runs complete without errors attributable to alignment changes
- Gate verdict for both runs: MERGE or ESCALATE (not BOUNCE due to alignment issues)
- No secrets exposed (secrets-sanitizer gate passes)

### Recording
Record results in `docs/maintainers/validation-log.md`:
- Date of execution
- Run IDs for Toy Run A and Toy Run B
- Flows executed (1-4)
- Pass/fail status
- Any notes on issues encountered

## Non-Behavioral Verification (from verification_notes.md)

| Requirement | Type | Verification Strategy | When |
|-------------|------|----------------------|------|
| NFR-MAINT-001 | Maintainability | Manual inspection during PR review; doc-drift CI check | Gate / PR Review |
| NFR-TEST-001 | Validation Tooling | pack-check exit 0; doc-drift exit 0; negative test for violation detection | Gate / CI |
| NFR-REGR-001 | Regression Prevention | Toy Run A/B completion; diff review; secrets-sanitizer gate | Gate / Validation Run |

### NFR-TEST-001 Verification Details
- **MET-1:** pack-check passes (`bash .claude/scripts/pack-check.sh` exits 0)
- **MET-2:** doc-drift passes (`scripts/check-doc-drift.sh` exits 0)
- **MET-3:** Negative tests (NEG-001, NEG-002, NEG-003) demonstrate enforcement works

### NFR-REGR-001 Verification Details
- **MET-1:** Toy Run A/B complete with no alignment-related errors
- **MET-2:** PR diff review confirms documentation-only changes
- **MET-3:** secrets-sanitizer returns `safe_to_publish: true`

## Priority Definitions

| Priority | Definition | Gate Behavior |
|----------|------------|---------------|
| P0 | Security/enforcement must work | Block merge if test fails |
| P1 | Primary verification path | Block merge if test fails |
| P2 | Secondary/nice-to-have | Document concern, do not block |

### P0 Tests (Must Pass Before Gate)
- NEG-001: Skill plumbing in flow command detected
- NEG-002: Invalid status enum detected
- NEG-003: CLI flag duplication detected

### P1 Tests (Must Pass Before Gate)
- pack-check exits 0 (no ERROR findings)
- doc-drift exits 0 (no duplication warnings)
- Toy Run A/B complete without alignment-related errors

### P2 Tests (Document Concerns Only)
- Manual review of file-write rules format
- Manual review of PR description for content moves

## Gaps and Questions

- Q: Should negative tests be automated as part of CI? Suggested default: No, manual patch-verify-revert is sufficient for this run. Impact: If yes, would need test fixtures with known-bad input.

- Q: Should Toy Run A/B use specific synthetic features or any reasonable input? Suggested default: Any reasonable feature request that exercises multiple agents. Impact: Specific features would make validation more reproducible.

- Q: Is doc-drift script complete enough to catch all CLAUDE.md duplication? Suggested default: Yes for major patterns; edge cases caught by manual review. Impact: If no, would need script enhancements before Gate.

## Recommended Next (for Flow 3)

1. Implement pack-check boundary rules in ST-004 (REQ-001, REQ-002 enforcement)
2. Complete doc-drift script enhancements if needed
3. Execute negative tests (NEG-001, NEG-002, NEG-003) to prove rules work
4. Run pack-check and doc-drift to verify clean state
5. Execute Toy Run A/B as final validation (ST-006)
6. Record validation results in `docs/maintainers/validation-log.md`

---

## Test Strategist Result
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
missing_required: []
blockers: []
severity_summary:
  critical: 0
  major: 0
  minor: 3
counts:
  scenarios_total: 31
  requirements_total: 7
  requirements_with_scenarios: 7
