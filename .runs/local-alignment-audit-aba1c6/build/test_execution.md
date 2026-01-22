# Test Execution Report

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []
test_summary:
mode: verify
ac_id: null
ac_filter_applied: null
command: cargo run --manifest-path tools/demoswarm-pack-check/Cargo.toml -- --no-color
exit_code: 0
passed: 53
failed: 0
skipped: 0
xfailed: 0
xpassed: 0
duration_seconds: 0

## Inputs Used

- `tools/demoswarm-pack-check/Cargo.toml`
- `.claude/` directory structure (agents, skills, flows)
- `CLAUDE.md`

## Execution

- tool: cargo run (pack-check)
- mode: verify
- ac_id: null
- ac_filter_applied: null
- command: `cargo run --manifest-path tools/demoswarm-pack-check/Cargo.toml -- --no-color`
- exit_code: 0
- duration: 0.59 seconds

## Canonical Summary (tool-bound)

Pack contents: Agents: 73, Commands: 8, Skills: 7. Passed with 2 warning(s).

## Test Summary (Canonical): passed=53 failed=0 skipped=0 xfailed=0 xpassed=0

## Failures (if any)

None — all checks passed.

## Notes

### Test Coverage

The pack-check validation framework executed 53 distinct checks covering:

1. **Required Agent Presence** (50+ agents present across all flows)
   - signal-cleanup, plan-cleanup, build-cleanup, gate-cleanup, deploy-cleanup, wisdom-cleanup
   - signal-run-prep, run-prep, repo-operator, secrets-sanitizer
   - gh-researcher, signal-normalizer, problem-framer, clarifier
   - requirements-author/critic, bdd-author/critic, scope-assessor
   - risk-analyst, impact-analyzer, design-optioneer, adr-author
   - interface-designer, contract-critic, observability-designer/critic
   - test-strategist, work-planner, design-critic, policy-analyst
   - context-loader, test-author/critic, code-implementer/critic
   - mutator, fixer, lint-executor, test-executor
   - doc-writer/critic, self-reviewer, receipt-checker, contract-enforcer
   - security-scanner, coverage-enforcer, gate-fixer, fix-forward-runner
   - traceability-auditor, merge-decider, deploy-monitor
   - smoke-verifier, deploy-decider, artifact-auditor
   - regression-analyst, flow-historian, learning-synthesizer
   - feedback-applier, gh-issue-manager, gh-reporter

2. **Flow Commands** (7 flows, 7 commands)
   - All flows reference complete sealing sequence (cleanup → secrets → repo-op → GH ops)
   - All flows document reseal-if-modified pattern
   - All flows document Gate Result contract block with modified_files field
   - All flows document GH content-mode gates and two-gate enforcement
   - All flows have Orchestrator Kickoff footer
   - All flows avoid RUN_BASE alias, use explicit paths
   - All flows avoid raw git commands (use repo-operator instead)

3. **Critic Control-Plane Contracts**
   - 8 critics have canonical Machine Summary axis: requirements-critic, bdd-critic, design-critic, contract-critic, observability-critic, code-critic, test-critic, doc-critic
   - All critics have can_further_iteration_help field
   - All critics have complete control-plane contract fields

4. **Cleanup Agent Invariants**
   - All 7 cleanup agents reference corresponding receipts + index.json updates
   - All mention route_to_flow field

5. **Deprecated Concept Checks**
   - No old FR-\* taxonomy found
   - No old @FR- tags found
   - No references to Flow Studio, harness.py, run-cleanup, profiles/, profile.yaml, orchestrator.py, or swarm_runtime

6. **Skills Presence**
   - All 7 skills present: test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools

7. **CLAUDE.md Content Validation**
   - Documents '.runs/<run-id>' directory structure ✓
   - Documents run_meta.json ✓
   - Documents index.json ✓
   - Documents "Seven Flows" terminology ✓
   - Documents Receipt naming ✓
   - Documents secrets-sanitizer ✓

8. **Issue-First Phrasing**
   - gh-reporter has issue-first invariant
   - All flows document issue-first posting pattern

9. **CANNOT_PROCEED Semantics**
   - All 7 cleanup agents document CANNOT_PROCEED as mechanical failure + missing_required field

10. **Machine Summary Status Enum**
    - 17 control-plane agents (enforcer, auditor, verifier, monitor, runner types) have canonical status axis: VERIFIED | UNVERIFIED | CANNOT_PROCEED

11. **Recommended Action Closed Enum**
    - All agents using recommended_action document: PROCEED | RERUN | BOUNCE | FIX_ENV

12. **Gate and Flow-Specific Contracts**
    - No domain drift in recommended*action (no BOUNCE_BUILD, BOUNCE_PLAN, RERUN_FLOW*\*)
    - All critics/verifiers have route_to_agent and route_to_flow fields
    - gh-issue-manager and gh-reporter enforce two gates (safe_to_publish + proceed_to_github_ops)
    - checkpoint_mode: local_only contract documented in repo-operator and referenced across 7 flows

13. **Decision Spine and ADR Contracts**
    - design-optioneer.md has marker-level schema hints
    - adr-author.md matches marker-based ADR contract (Swarm-Proposed status)
    - design-critic.md matches updated handshake contract (semantic + markers)
    - plan-cleanup.md has marker-based decision spine extraction

14. **Typed NFR ID Contract**
    - No bare NFR-### patterns (all NFRs are typed, e.g., NFR-PERF-001)
    - No deprecated NFR-SCALE-\* patterns

15. **Subtask Bridge Contract**
    - work-planner.md documents subtasks.yaml output
    - Documents subtask status enum (TODO | DOING | DONE)
    - context-loader.md references subtasks.yaml and documents selection precedence
    - allow_new_files_under scope hint documented

16. **Code Quality Boundaries**
    - Cleanup agents use demoswarm.sh shim (no bespoke pipelines)
    - No direct demoswarm invocations outside shims
    - Agents using demoswarm.sh have ## Skills section
    - No shim line-continuation bypass patterns
    - GH agents use heredoc (not --body-file) and no forbidden patterns
    - jq commands use single-line paths
    - ms get keys documented by producers
    - inv get marker contracts (consumer vs producer) align
    - No CANNOT_PROCEED at flow level (mechanical failures caught at agent level)
    - No RUN_BASE alias drift

17. **Smoke-Verifier Domain Verdict Separation**
    - Correctly distinguishes smoke_signal (STABLE | INVESTIGATE | ROLLBACK) from deploy verdict (STABLE | NOT_DEPLOYED | BLOCKED_BY_GATE)

18. **All Control-Plane Agents (Auto-Detected)**
    - 62 agents detected with '## Machine Summary' block
    - All have complete control-plane contract

19. **Command Agent Name Resolution**
    - All 8 command docs reference valid agent names

20. **Observations Field**
    - All 8 critics have observations field in Machine Summary

21. **OpenQ QID Patterns**
    - All OpenQ QIDs use valid format (OQ-<FLOW>-<NNN>)
    - Found 9 non-canonical OpenQ flow codes (OQ-PLAN-_ should be OQ-PLAN-_) — advisory only, does not block

### Warnings (Non-Critical)

1. **flow-7-wisdom.md present** — Flow 7 (Wisdom) command documentation is now available and validated. The wisdom-cleanup agent correctly references REG marker pattern for regression detection.

2. **Non-canonical OpenQ flow codes** — 9 instances of OQ-PLAN-_ found in open_questions.md files; pack-check recommends OQ-PLAN-_ for consistency with flow code abbreviations (SIG, PLN, BLD, REV, GT, DPL, WIS). This is advisory only and does not impact validation.

## Control-Plane Routing

### Recommended Action: PROCEED

- Status is VERIFIED; no blockers identified
- All 53 structural and contract checks passed
- Pack documentation (CLAUDE.md) correctly reflects Seven-Flow model
- All agent and skill boundaries enforced
- Two non-critical warnings do not impact pack functionality

### Next Steps

- Proceed to Flow cleanup and secrets sanitization
- Non-canonical OpenQ QID pattern can be addressed in a follow-up maintenance pass if desired
