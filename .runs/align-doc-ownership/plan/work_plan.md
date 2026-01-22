# Work Plan for align-doc-ownership

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []
missing_required:
  - ".runs/align-doc-ownership/plan/test_plan.md" (not yet created by test-strategist)
  - ".runs/align-doc-ownership/plan/observability_spec.md" (not yet created by observability-designer)

concerns:
  - ST-004 carries heavier scope than other subtasks (Gate + pack-check + CLAUDE.md)
  - 55 agent files to audit increases risk of merge conflicts if subtasks overlap
  - Judgment calls on "minimal examples" may drift without clear guidelines
  - Skill doc coverage gaps may delay full agent example removal
```

## Scope Snapshot

- **ADR decision**: OPT-002 (Pragmatic Enforcement) - Add pack-check rules for major boundary violations while allowing minimal inline examples in agent docs when skill docs have coverage gaps.
- **Primary impacts**:
  - 6 flow commands to clean of skill plumbing
  - 55 agent docs to normalize (status/action enums, Skills sections, skill references)
  - 7 skill docs confirmed as CLI truth source
  - CLAUDE.md normalized to summary-level entry point
  - pack-check extended with 3 boundary enforcement rules
- **Key constraints**:
  - Flow commands must contain zero skill plumbing (strict enforcement)
  - Agent docs must use canonical enum values
  - Agents using skills must declare them in a Skills section
  - Archive-over-delete pattern applies to removed content
  - Validation run required after alignment complete
- **Verification posture**:
  - pack-check must pass including new boundary rules
  - doc-drift (scripts/check-doc-drift.sh) must pass
  - Toy Run A/B through flows 1-4 must succeed
  - No functional regression in flows/agents/skills

## Subtask Index (parseable)

The canonical subtask index is at: `.runs/align-doc-ownership/plan/subtasks.yaml`

Summary table:

| ID     | Title                                                       | Status | Depends On             | Estimate |
| ------ | ----------------------------------------------------------- | ------ | ---------------------- | -------- |
| ST-001 | Align Flow 1 (Signal) documentation                         | TODO   | -                      | M        |
| ST-002 | Align Flow 2 (Plan) documentation                           | TODO   | -                      | M        |
| ST-003 | Align Flow 3 (Build) documentation                          | TODO   | -                      | M        |
| ST-004 | Align Flow 4 (Gate) + cross-cutting enforcement + CLAUDE.md | TODO   | ST-001, ST-002, ST-003 | L        |
| ST-005 | Align Flow 5 (Deploy) documentation                         | TODO   | -                      | S        |
| ST-006 | Align Flow 6 (Wisdom) + validation run                      | TODO   | ST-001-ST-005          | M        |

## Subtasks

### ST-001: Align Flow 1 (Signal) documentation

- **Objective**: Remove skill plumbing from flow-1-signal.md and ensure all Flow 1 agents have consistent enums, Skills sections, and skill references.
- **Status**: TODO
- **Planned touchpoints**:
  - `.claude/commands/flow-1-signal.md`
  - `.claude/agents/signal-run-prep.md`
  - `.claude/agents/signal-normalizer.md`
  - `.claude/agents/problem-framer.md`
  - `.claude/agents/requirements-author.md`
  - `.claude/agents/requirements-critic.md`
  - `.claude/agents/bdd-author.md`
  - `.claude/agents/bdd-critic.md`
  - `.claude/agents/scope-assessor.md`
  - `.claude/agents/signal-cleanup.md`
  - `.claude/agents/clarifier.md`
  - `.claude/agents/risk-analyst.md`
- **REQ/NFR linkage**: REQ-001, REQ-002, REQ-005, NFR-MAINT-001, NFR-REGR-001
- **Acceptance criteria**:
  - flow-1-signal.md contains no skill plumbing (demoswarm.sh, skill names, CLI flags)
  - All Flow 1 agents use canonical status enums (VERIFIED, UNVERIFIED, CANNOT_PROCEED)
  - All Flow 1 agents use canonical action enums (PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV)
  - signal-cleanup.md has Skills section listing runs-derive, runs-index
  - clarifier.md has Skills section listing openq-tools
  - Agent docs reference skill docs instead of duplicating CLI examples
- **Scope hints**:
  - Code roots: (none)
  - Test roots: (none)
  - Allow new files under: (none)
- **Tests**:
  - pack-check passes for Flow 1 files
  - doc-drift passes for Flow 1 files
- **Observability**: (none)
- **Dependencies**: None
- **Risk / blast radius**: Low - isolated to Flow 1 files only; no code changes
- **Estimate**: M

---

### ST-002: Align Flow 2 (Plan) documentation

- **Objective**: Remove skill plumbing from flow-2-plan.md and ensure all Flow 2 agents have consistent enums, Skills sections, and skill references.
- **Status**: TODO
- **Planned touchpoints**:
  - `.claude/commands/flow-2-plan.md`
  - `.claude/agents/run-prep.md`
  - `.claude/agents/impact-analyzer.md`
  - `.claude/agents/design-optioneer.md`
  - `.claude/agents/adr-author.md`
  - `.claude/agents/interface-designer.md`
  - `.claude/agents/observability-designer.md`
  - `.claude/agents/test-strategist.md`
  - `.claude/agents/work-planner.md`
  - `.claude/agents/design-critic.md`
  - `.claude/agents/policy-analyst.md`
  - `.claude/agents/plan-cleanup.md`
- **REQ/NFR linkage**: REQ-001, REQ-002, REQ-005, NFR-MAINT-001, NFR-REGR-001
- **Acceptance criteria**:
  - flow-2-plan.md contains no skill plumbing (demoswarm.sh, skill names, CLI flags)
  - All Flow 2 agents use canonical status enums (VERIFIED, UNVERIFIED, CANNOT_PROCEED)
  - All Flow 2 agents use canonical action enums (PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV)
  - plan-cleanup.md has Skills section listing runs-derive, runs-index
  - Agent docs reference skill docs instead of duplicating CLI examples
- **Scope hints**:
  - Code roots: (none)
  - Test roots: (none)
  - Allow new files under: (none)
- **Tests**:
  - pack-check passes for Flow 2 files
  - doc-drift passes for Flow 2 files
- **Observability**: (none)
- **Dependencies**: None
- **Risk / blast radius**: Low - isolated to Flow 2 files only; no code changes
- **Estimate**: M

---

### ST-003: Align Flow 3 (Build) documentation

- **Objective**: Remove skill plumbing from flow-3-build.md and ensure all Flow 3 agents have consistent enums, Skills sections, and skill references. Special attention to context-loader, mutator, fixer, and self-reviewer docs.
- **Status**: TODO
- **Planned touchpoints**:
  - `.claude/commands/flow-3-build.md`
  - `.claude/agents/context-loader.md`
  - `.claude/agents/test-author.md`
  - `.claude/agents/test-critic.md`
  - `.claude/agents/code-implementer.md`
  - `.claude/agents/code-critic.md`
  - `.claude/agents/mutator.md`
  - `.claude/agents/fixer.md`
  - `.claude/agents/doc-writer.md`
  - `.claude/agents/self-reviewer.md`
  - `.claude/agents/build-cleanup.md`
- **REQ/NFR linkage**: REQ-001, REQ-002, REQ-005, NFR-MAINT-001, NFR-REGR-001
- **Acceptance criteria**:
  - flow-3-build.md contains no skill plumbing (demoswarm.sh, skill names, CLI flags)
  - All Flow 3 agents use canonical status enums (VERIFIED, UNVERIFIED, CANNOT_PROCEED)
  - All Flow 3 agents use canonical action enums (PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV)
  - build-cleanup.md has Skills section listing runs-derive, runs-index
  - context-loader.md, mutator.md, fixer.md, self-reviewer.md updated for consistency
  - Agent docs reference skill docs instead of duplicating CLI examples
- **Scope hints**:
  - Code roots: (none)
  - Test roots: (none)
  - Allow new files under: (none)
- **Tests**:
  - pack-check passes for Flow 3 files
  - doc-drift passes for Flow 3 files
- **Observability**: (none)
- **Dependencies**: None
- **Risk / blast radius**: Low - isolated to Flow 3 files only; no code changes
- **Estimate**: M

---

### ST-004: Align Flow 4 (Gate) + cross-cutting enforcement + CLAUDE.md

- **Objective**: Remove skill plumbing from flow-4-gate.md, ensure all Flow 4 agents have consistent enums and Skills sections, implement pack-check boundary enforcement rules, and normalize CLAUDE.md to entry-point level.
- **Status**: TODO
- **Planned touchpoints**:
  - `.claude/commands/flow-4-gate.md`
  - `.claude/agents/receipt-checker.md`
  - `.claude/agents/contract-enforcer.md`
  - `.claude/agents/security-scanner.md`
  - `.claude/agents/coverage-enforcer.md`
  - `.claude/agents/gate-fixer.md`
  - `.claude/agents/merge-decider.md`
  - `.claude/agents/gate-cleanup.md`
  - `.claude/agents/repo-operator.md`
  - `.claude/agents/secrets-sanitizer.md`
  - `.claude/agents/gh-issue-manager.md`
  - `.claude/agents/gh-reporter.md`
  - `.claude/agents/gh-researcher.md`
  - `.claude/agents/pack-customizer.md`
  - `CLAUDE.md`
  - `tools/demoswarm-pack-check/src/checks/mod.rs`
  - `tools/demoswarm-pack-check/src/checks/drift.rs`
  - `tools/demoswarm-pack-check/src/checks/flow.rs`
  - `tools/demoswarm-pack-check/src/checks/structure.rs`
  - `scripts/check-doc-drift.sh`
- **REQ/NFR linkage**: REQ-001, REQ-002, REQ-003, REQ-004, REQ-005, NFR-MAINT-001, NFR-TEST-001, NFR-REGR-001
- **Acceptance criteria**:
  - flow-4-gate.md contains no skill plumbing (demoswarm.sh, skill names, CLI flags)
  - All Flow 4 agents use canonical status/action enums
  - gate-cleanup.md has Skills section listing runs-derive, runs-index
  - secrets-sanitizer.md has Skills section listing secrets-tools
  - pack-check extended with boundary enforcement rules
  - pack-check detects skill plumbing in flow commands
  - pack-check detects missing Skills sections in agents using skills
  - pack-check detects non-canonical enum values in agent docs
  - CLAUDE.md Skills table normalized to summary-level (no flag details)
  - CLAUDE.md references skill docs for detailed usage
- **Scope hints**:
  - Code roots: `tools/demoswarm-pack-check/src/checks/`
  - Test roots: (none)
  - Allow new files under: `tools/demoswarm-pack-check/src/checks/`
- **Tests**:
  - pack-check passes with new boundary rules
  - pack-check correctly fails on known violations (negative test)
  - doc-drift passes
- **Observability**: (none)
- **Dependencies**: ST-001, ST-002, ST-003 (pack-check rules validate prior subtask work)
- **Risk / blast radius**: Medium - pack-check Rust development required; CLAUDE.md is high-touch file
- **Estimate**: L

---

### ST-005: Align Flow 5 (Deploy) documentation

- **Objective**: Remove skill plumbing from flow-5-deploy.md and ensure all Flow 5 agents have consistent enums, Skills sections, and skill references. Ensure clean separation of release ops vs reporting ops.
- **Status**: TODO
- **Planned touchpoints**:
  - `.claude/commands/flow-5-deploy.md`
  - `.claude/agents/deploy-monitor.md`
  - `.claude/agents/smoke-verifier.md`
  - `.claude/agents/deploy-decider.md`
  - `.claude/agents/deploy-cleanup.md`
- **REQ/NFR linkage**: REQ-001, REQ-002, REQ-005, NFR-MAINT-001, NFR-REGR-001
- **Acceptance criteria**:
  - flow-5-deploy.md contains no skill plumbing (demoswarm.sh, skill names, CLI flags)
  - All Flow 5 agents use canonical status enums (VERIFIED, UNVERIFIED, CANNOT_PROCEED)
  - All Flow 5 agents use canonical action enums (PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV)
  - deploy-cleanup.md has Skills section listing runs-derive, runs-index
  - Clean separation of release ops vs reporting ops documented
  - Agent docs reference skill docs instead of duplicating CLI examples
- **Scope hints**:
  - Code roots: (none)
  - Test roots: (none)
  - Allow new files under: (none)
- **Tests**:
  - pack-check passes for Flow 5 files
  - doc-drift passes for Flow 5 files
- **Observability**: (none)
- **Dependencies**: None
- **Risk / blast radius**: Low - smallest subtask; isolated to Flow 5 files
- **Estimate**: S

---

### ST-006: Align Flow 6 (Wisdom) + validation run

- **Objective**: Remove skill plumbing from flow-6-wisdom.md, ensure all Flow 6 agents have consistent enums and Skills sections, establish crisp stable marker contracts, and execute the validation run (Toy Run A/B through flows 1-4).
- **Status**: TODO
- **Planned touchpoints**:
  - `.claude/commands/flow-6-wisdom.md`
  - `.claude/agents/artifact-auditor.md`
  - `.claude/agents/regression-analyst.md`
  - `.claude/agents/flow-historian.md`
  - `.claude/agents/learning-synthesizer.md`
  - `.claude/agents/feedback-applier.md`
  - `.claude/agents/wisdom-cleanup.md`
  - `docs/maintainers/validation-log.md`
- **REQ/NFR linkage**: REQ-001, REQ-002, REQ-005, REQ-006, NFR-MAINT-001, NFR-TEST-001, NFR-REGR-001
- **Acceptance criteria**:
  - flow-6-wisdom.md contains no skill plumbing (demoswarm.sh, skill names, CLI flags)
  - All Flow 6 agents use canonical status enums (VERIFIED, UNVERIFIED, CANNOT_PROCEED)
  - All Flow 6 agents use canonical action enums (PROCEED, RERUN, BOUNCE, ESCALATE, FIX_ENV)
  - wisdom-cleanup.md has Skills section listing runs-derive, runs-index
  - Crisp stable marker contracts documented
  - Toy Run A completes flows 1-4 successfully
  - Toy Run B completes flows 1-4 successfully
  - Validation log entry recorded in docs/maintainers/validation-log.md
  - pack-check and doc-drift pass before validation log entry
- **Scope hints**:
  - Code roots: (none)
  - Test roots: (none)
  - Allow new files under: (none)
- **Tests**:
  - pack-check passes for Flow 6 files
  - doc-drift passes for Flow 6 files
  - Toy Run A/B validation succeeds
- **Observability**: (none)
- **Dependencies**: ST-001, ST-002, ST-003, ST-004, ST-005 (validation run requires all alignment complete)
- **Risk / blast radius**: Low - Flow 6 changes are low risk; validation run is verification only
- **Estimate**: M

---

## Dependency Graph

```
ST-001 (Flow 1) ─────┐
                     │
ST-002 (Flow 2) ─────┼──→ ST-004 (Flow 4 + pack-check + CLAUDE.md) ──→ ST-006 (Flow 6 + validation)
                     │                                                        ↑
ST-003 (Flow 3) ─────┘                                                        │
                                                                              │
ST-005 (Flow 5) ──────────────────────────────────────────────────────────────┘
```

In sequence:

```
[ST-001, ST-002, ST-003, ST-005] (parallel) → ST-004 → ST-006
```

## Parallelization Opportunities

- **ST-001, ST-002, ST-003, ST-005** can run in parallel:
  - Each targets a distinct set of files
  - No shared file ownership
  - All are documentation-only changes
  - No cross-dependencies between them

- **ST-004** must wait for ST-001, ST-002, ST-003:
  - pack-check rules validate the work done in prior subtasks
  - Testing the rules requires prior subtask changes to be in place

- **ST-006** must wait for all others:
  - Validation run tests the complete alignment
  - Cannot record validation success until all changes are complete

## Rollout Strategy

### Phase 0 (pre-merge): Contracts + Tests + Observability Hooks

- [ ] Ensure pack-check can be extended (verify `tools/demoswarm-pack-check/` structure)
- [ ] Identify all files per subtask using `touches` patterns
- [ ] Confirm doc-drift script works on baseline
- [ ] Establish baseline: run pack-check and doc-drift before changes

### Phase 1 (merge): What "green" means

For each subtask to be considered complete:

- pack-check passes (exit code 0)
- doc-drift passes (exit code 0)
- No regression in existing functionality
- All acceptance criteria met

For ST-004 specifically:

- New pack-check rules detect violations (positive case)
- New pack-check rules pass on compliant files (negative case)
- No false positives on legitimate documentation patterns

### Phase 2 (limited exposure): Feature flag / Staged enablement

Not applicable - this is documentation-only work without runtime behavior changes.

However, consider staged validation:

- Run pack-check on subtask files as each completes
- Run doc-drift incrementally
- Do not wait for full completion to catch issues

### Phase 3 (full): Final gates

Before recording completion:

- [ ] pack-check passes on entire `.claude/` directory
- [ ] doc-drift passes on entire repository
- [ ] Toy Run A completes flows 1-4 without errors
- [ ] Toy Run B completes flows 1-4 without errors
- [ ] Validation log entry written to `docs/maintainers/validation-log.md`

## Rollback Plan

### Fast Rollback Lever

- **Git revert**: All changes are documentation-only; `git revert` can undo any subtask
- **Per-subtask granularity**: Each subtask touches distinct files; can revert one without affecting others
- **pack-check rules**: Can disable new boundary checks by commenting out in `checks/mod.rs` if false positives occur

### Data / Schema Notes

- **No data migrations**: This work is documentation-only
- **No schema changes**: No database or API changes
- **No irreversible steps**: All changes are file edits that can be reverted

### What to Monitor for Rollback Decision

- pack-check exit code (should be 0)
- doc-drift exit code (should be 0)
- Toy Run A/B completion status
- Any agent or flow failures attributable to documentation changes (should be none)

If pack-check shows false positives:

1. Identify the regex pattern causing false positive
2. Refine pattern in `tools/demoswarm-pack-check/src/checks/`
3. Rebuild and retest

If Toy Run fails:

1. Identify which flow/agent failed
2. Check if failure is related to documentation changes
3. If yes, revert the specific subtask's changes
4. If no, investigate unrelated cause

## Assumptions

- **ASM-001**: The three-tier ownership model (flow -> agent -> skill) is the intended architecture.
  - Impact if wrong: Entire boundary definition and REQ-001 through REQ-004 would need rethinking.

- **ASM-002**: Cleanup agents legitimately need operational detail inline but should reference skill docs for CLI truth.
  - Impact if wrong: Cleanup agents would need to become thin orchestrators.

- **ASM-003**: CLAUDE.md is entry-point level; detailed CLI reference belongs in skill docs.
  - Impact if wrong: CLAUDE.md would expand to serve as primary reference.

- **ASM-004**: pack-check can be extended with boundary-enforcement checks without major refactoring.
  - Impact if wrong: May need to fall back to shell scripts for enforcement.

- **ASM-005**: Pack maintainers can exercise judgment on "minimal examples" consistently.
  - Impact if wrong: Inconsistent application of rules; may need to tighten to strict enforcement.

- **ASM-006**: Subtask partitioning (ST-001 through ST-006 by flow) is final and will not change.
  - Impact if wrong: Work plan would need restructuring.

## Open Questions

Reference: `.runs/align-doc-ownership/plan/open_questions.md`

The following questions are OPEN and may affect implementation:

- **OQ-PLAN-001**: Should pack-check boundary rules be implemented in Rust or shell scripts?
  - Default: Rust implementation in pack-check
  - Needs answer by: Flow 3 (Build)

- **OQ-PLAN-002**: What specific regex patterns should pack-check use to detect skill plumbing?
  - Default: Match `demoswarm\.sh`, skill names, CLI flag patterns
  - Needs answer by: Flow 3 (Build)

- **OQ-PLAN-003**: Should archive-over-delete apply to inline CLI examples being removed?
  - Default: No, only entire files/sections need archiving
  - Needs answer by: Flow 3 (Build)

- **OQ-PLAN-004**: What defines "brief inline example" vs "excessive CLI duplication"?
  - Default: Single-line patterns acceptable; multi-line with all flags is excessive
  - Needs answer by: Flow 3 (Build)

---

## Inventory (machine countable)

- WORK_PLAN_SUBTASK: ST-001
- WORK_PLAN_SUBTASK: ST-002
- WORK_PLAN_SUBTASK: ST-003
- WORK_PLAN_SUBTASK: ST-004
- WORK_PLAN_SUBTASK: ST-005
- WORK_PLAN_SUBTASK: ST-006
- WORK_PLAN_REQ_COVERED: REQ-001
- WORK_PLAN_REQ_COVERED: REQ-002
- WORK_PLAN_REQ_COVERED: REQ-003
- WORK_PLAN_REQ_COVERED: REQ-004
- WORK_PLAN_REQ_COVERED: REQ-005
- WORK_PLAN_REQ_COVERED: REQ-006
- WORK_PLAN_NFR_COVERED: NFR-MAINT-001
- WORK_PLAN_NFR_COVERED: NFR-TEST-001
- WORK_PLAN_NFR_COVERED: NFR-REGR-001

## Machine Summary Block

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required:
  - ".runs/align-doc-ownership/plan/test_plan.md" (not yet created)
  - ".runs/align-doc-ownership/plan/observability_spec.md" (not yet created)
concerns:
  - ST-004 carries heavier scope than other subtasks
  - Judgment calls on minimal examples may drift
  - OQ-PLAN-001 through OQ-PLAN-004 have defaults but remain open
subtasks_total: 6
estimates: [M, M, M, L, S, M]
```
