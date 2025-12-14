# Risk Assessment

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []

missing_required: []

concerns:
  - ST-004 carries heavier scope (Gate + cross-cutting enforcement + CLAUDE.md cleanup); may require proportionally more effort
  - 55 agent files to audit increases merge conflict risk if subtasks overlap in touches
  - Open questions (OQ-SIG-001 through OQ-SIG-006) have defaults but remain open; Plan phase should confirm
  - pack-check boundary enforcement rules may produce false positives on legitimate skill references

severity_summary:
  critical: 0
  high: 0
  medium: 3
  low: 4

## Context
- flow: signal
- run_id: align-doc-ownership
- inputs_used:
  - .runs/align-doc-ownership/run_meta.json
  - .runs/align-doc-ownership/signal/problem_statement.md
  - .runs/align-doc-ownership/signal/requirements.md
  - .runs/align-doc-ownership/signal/early_risks.md
  - .runs/align-doc-ownership/signal/context_brief.md
- prior_risk_assessments_seen:
  - none

## Risk Register

| ID | Category | Severity | Status | Summary | Owner |
|----|----------|----------|--------|---------|-------|
| RSK-001 | OPS | MEDIUM | OPEN | Merge conflicts across parallel subtasks due to overlapping file touches | pack-maintainers |
| RSK-002 | OPS | MEDIUM | OPEN | ST-004 scope concentration may cause timeline imbalance | pack-maintainers |
| RSK-003 | OPS | MEDIUM | OPEN | pack-check rule additions may produce false positives | pack-maintainers |
| RSK-004 | OPS | LOW | OPEN | Validation run dependency blocks completion until flows 1-4 pass | pack-maintainers |
| RSK-005 | PERFORMANCE | LOW | OPEN | pack-check runtime may increase with new boundary checks | pack-maintainers |
| RSK-006 | SECURITY | LOW | MITIGATED | No code execution changes; secrets-sanitizer gate preserved | pack-maintainers |
| RSK-007 | COMPLIANCE | LOW | MITIGATED | No regulatory impact; internal tooling only | pack-maintainers |

## Risk Details

### RSK-001: Merge conflicts across parallel subtasks
- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Likelihood: Medium
- Impact: Medium
- Evidence:
  - `.runs/align-doc-ownership/signal/problem_statement.md` (55 agent files need consistency auditing)
  - `.runs/align-doc-ownership/signal/requirements.md` (REQ-005 AC-7: each subtask has distinct touches pattern)
  - `.runs/align-doc-ownership/signal/context_brief.md` (Risks Spotted Early: merge conflicts remain possible)
- Impact:
  - If subtasks ST-001 through ST-006 modify overlapping files (e.g., CLAUDE.md is touched by multiple subtasks, or a shared skill doc is updated from multiple flows), merge conflicts will require manual resolution
  - Conflicts increase effort and risk introducing inconsistencies during resolution
- Mitigation:
  - REQ-005 AC-7 already specifies distinct touches patterns per subtask
  - ST-004 owns CLAUDE.md exclusively; other subtasks should not touch it
  - Sequential execution of subtasks (rather than parallel) can avoid conflicts at cost of throughput
  - Use atomic commits with clear scope per subtask to minimize conflict surface
- Verification:
  - PR review should verify each subtask's touched files match declared scope
  - pack-check should still pass after each subtask merge
- Recommendation:
  - Proceed with distinct touches enforcement; document file ownership matrix in work_plan.md

### RSK-002: ST-004 scope concentration
- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Likelihood: High
- Impact: Medium
- Evidence:
  - `.runs/align-doc-ownership/signal/problem_statement.md` (Concerns: ST-004 carries heavier scope)
  - `.runs/align-doc-ownership/signal/requirements.md` (REQ-005 AC-4: ST-004 covers Gate + cross-cutting + CLAUDE.md)
  - `.runs/align-doc-ownership/signal/context_brief.md` (ST-004 is heavier than others)
- Impact:
  - ST-004 is responsible for: Flow 4 (Gate) documentation, pack-check boundary enforcement rules (possibly Rust development), and CLAUDE.md normalization
  - This concentration may cause ST-004 to take 2-3x longer than other subtasks
  - If ST-004 becomes a bottleneck, downstream subtasks may be delayed
- Mitigation:
  - Consider splitting ST-004 into ST-004a (Gate docs) and ST-004b (cross-cutting enforcement + CLAUDE.md)
  - Alternative: Accept the imbalance and allocate proportionally more time to ST-004
  - Plan phase should produce explicit time estimates per subtask
- Verification:
  - work_plan.md should reflect the heavier scope of ST-004 in its estimates
  - Progress tracking should flag if ST-004 is taking longer than planned
- Recommendation:
  - Proceed; Plan phase should account for ST-004 scope in work_plan.md estimates

### RSK-003: pack-check rule false positives
- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Likelihood: Medium
- Impact: Medium
- Evidence:
  - `.runs/align-doc-ownership/signal/problem_statement.md` (Concerns: pack-check boundary enforcement rules do not exist yet)
  - `.runs/align-doc-ownership/signal/requirements.md` (REQ-001 AC-1: detect skill plumbing in flow commands)
  - `.runs/align-doc-ownership/signal/context_brief.md` (pack-check extension may require Rust development)
- Impact:
  - New boundary-enforcement rules in pack-check may flag legitimate content as violations
  - Example: A flow command might legitimately mention "runs-derive" in prose (e.g., "agents use runs-derive skill") without actually embedding CLI invocations
  - False positives increase developer friction and may lead to pack-check being ignored or disabled
- Mitigation:
  - Rules should match specific invocation patterns (e.g., `bash .claude/scripts/demoswarm.sh runs-derive`) rather than bare skill names in prose
  - Add negative test cases that verify legitimate prose references do not trigger violations
  - REQ-001 AC-1 should be interpreted as "direct invocations" not "mentions"
- Verification:
  - NFR-TEST-001 MET-3 requires negative tests for boundary violations
  - Manual testing of edge cases before merging pack-check changes
- Recommendation:
  - Proceed; Plan phase should specify precise regex patterns for violation detection

### RSK-004: Validation run blocks completion
- Category: OPS
- Severity: LOW
- Status: OPEN
- Likelihood: High
- Impact: Low
- Evidence:
  - `.runs/align-doc-ownership/signal/problem_statement.md` (Success Looks Like: Toy Run A/B succeeds and is recorded)
  - `.runs/align-doc-ownership/signal/requirements.md` (REQ-006: validation run recording)
  - `.runs/align-doc-ownership/signal/context_brief.md` (Validation dependency: work cannot be considered complete until flows 1-4 execute)
- Impact:
  - The run cannot be marked complete until Toy Run A/B (flows 1-4) executes successfully
  - If validation fails, debugging and fixes extend the timeline
  - This is an expected gate, not a true risk; severity is LOW because it is a designed quality control
- Mitigation:
  - Run validation early (after each subtask if practical) rather than only at the end
  - Keep alignment changes minimal and incremental to reduce validation failure risk
- Verification:
  - REQ-006 AC-3 requires pack-check and doc-drift pass before validation
  - Validation log entry per REQ-006 AC-2
- Recommendation:
  - Proceed; this is a designed quality gate, not a risk to mitigate away

### RSK-005: pack-check runtime increase
- Category: PERFORMANCE
- Severity: LOW
- Status: OPEN
- Likelihood: Low
- Impact: Low
- Evidence:
  - `.runs/align-doc-ownership/signal/requirements.md` (NFR-TEST-001 MET-1: pack-check passes with exit code 0)
  - `.runs/align-doc-ownership/signal/context_brief.md` (tools/demoswarm-pack-check/ may need new drift rules)
- Impact:
  - Adding new boundary-enforcement checks may increase pack-check runtime
  - Current pack-check is already performant (sub-second); new checks are expected to add minimal overhead
  - Impact is LOW because pack-check runs only during flow gates, not during development
- Mitigation:
  - Implement checks as regex scans over file content, which are O(n) in file size
  - Avoid expensive operations like AST parsing for simple pattern matching
- Verification:
  - Measure pack-check runtime before and after changes; accept if increase is < 500ms
- Recommendation:
  - Proceed; monitor runtime during development but do not block on this

### RSK-006: No security regression
- Category: SECURITY
- Severity: LOW
- Status: MITIGATED
- Likelihood: Very Low
- Impact: N/A (no change)
- Evidence:
  - `.runs/align-doc-ownership/signal/problem_statement.md` (Non-Goals: Changing functional behavior of any flow, agent, or skill)
  - `.runs/align-doc-ownership/signal/requirements.md` (NFR-REGR-001 MET-3: No secrets exposed)
- Impact:
  - This run modifies only documentation files; no code execution paths change
  - secrets-sanitizer gate remains in place for all flows
- Mitigation:
  - secrets-sanitizer continues to scan publish surface in every flow
  - REQ-007 archive-over-delete pattern prevents accidental exposure of removed content
- Verification:
  - secrets-sanitizer Gate Result in Flow 4 (Gate)
  - NFR-REGR-001 MET-3 verification during PR review
- Recommendation:
  - No additional mitigation required; proceed

### RSK-007: No compliance impact
- Category: COMPLIANCE
- Severity: LOW
- Status: MITIGATED
- Likelihood: Very Low
- Impact: N/A (no change)
- Evidence:
  - `.runs/align-doc-ownership/signal/problem_statement.md` (internal tooling, no regulatory impact)
  - `.runs/align-doc-ownership/signal/requirements.md` (no compliance-related requirements)
- Impact:
  - This is internal pack tooling documentation; no PII, PHI, or regulated data is involved
  - No audit logging, data residency, or consent requirements apply
- Mitigation:
  - N/A; compliance is not a concern for this run
- Verification:
  - Compliance domain is out of scope; no verification needed
- Recommendation:
  - No action required; proceed

## Deltas Since Prior (if any)
- NEW: [RSK-001, RSK-002, RSK-003, RSK-004, RSK-005, RSK-006, RSK-007]
- CHANGED: []
- CLOSED: []

## Recommended Next
- Proceed to Flow 2 (Plan) to develop work_plan.md with explicit time estimates, especially for ST-004
- Plan phase should define precise regex patterns for pack-check boundary violations to avoid false positives (RSK-003)
- Plan phase should confirm the file ownership matrix to minimize merge conflicts (RSK-001)
- Open questions (OQ-SIG-001 through OQ-SIG-006) should be resolved in Plan phase
- Consider running pack-check and doc-drift early in each subtask to catch issues incrementally
