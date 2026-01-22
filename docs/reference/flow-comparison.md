# Flow Comparison Reference

> Quick reference comparing all 7 flows side-by-side.

---

## Summary Table

| Flow | Name | Primary Input | Primary Output | Key Agents | Typical Duration |
|------|------|---------------|----------------|------------|------------------|
| 1 | Signal | Feature request / issue | `signal_receipt.json`, requirements, BDD | gh-issue-resolver, requirements-author, bdd-author, scope-assessor, spec-auditor | 15-30 min |
| 2 | Plan | Requirements + BDD scenarios | `plan_receipt.json`, ADR, contracts, work plan | impact-analyzer, design-optioneer, adr-author, interface-designer, test-strategist, work-planner | 20-40 min |
| 3 | Build | ADR + contracts + work plan | `build_receipt.json`, code, tests, Draft PR | test-author, code-implementer, test-executor, standards-enforcer, pr-creator | 30-90 min |
| 4 | Review | Draft PR + feedback | `review_receipt.json`, resolved worklist | pr-feedback-harvester, review-worklist-writer, pr-commenter, pr-status-manager | 15-45 min |
| 5 | Gate | Build + Review receipts | `gate_receipt.json`, merge decision | receipt-checker, contract-enforcer, security-scanner, coverage-enforcer, merge-decider | 10-25 min |
| 6 | Deploy | Gate MERGE decision | `deploy_receipt.json`, deployment verdict | deploy-decider, repo-operator, deploy-monitor, smoke-verifier | 5-15 min |
| 7 | Wisdom | All prior receipts | `wisdom_receipt.json`, learnings | solution-analyst, quality-analyst, maintainability-analyst, learning-synthesizer, feedback-applier | 20-40 min |

---

## Agent Categories

Agents are organized into three categories across all flows:

| Category | Examples | Purpose |
|----------|----------|---------|
| **Domain Agents** | requirements-author, code-implementer, merge-decider | Do the primary work of each flow |
| **Cross-Cutting Agents** | clarifier, risk-analyst, policy-analyst | Provide expertise used across multiple flows |
| **Infrastructure Agents** | run-prep, repo-operator, secrets-sanitizer, *-cleanup | Handle setup, git operations, and finalization |

For authoritative agent lists per flow, see the individual flow command files in `.claude/commands/flow-*.md`.

---

## Key Artifacts per Flow

### Flow 1: Signal

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `signal_receipt.json` | signal-cleanup | Machine-readable summary for downstream |
| `requirements.md` | requirements-author | Functional + non-functional requirements |
| `requirements_critique.md` | requirements-critic | Critique and iteration guidance |
| `features/*.feature` | bdd-author | BDD scenarios in Gherkin format |
| `verification_notes.md` | bdd-author | NFR verification criteria (non-BDD) |
| `bdd_critique.md` | bdd-critic | Critique of BDD scenarios |
| `problem_statement.md` | problem-framer | Goals, non-goals, constraints |
| `stakeholders.md` | scope-assessor | Teams, systems, users affected |
| `early_risks.md` | scope-assessor | Initial risk identification by category |
| `risk_assessment.md` | risk-analyst | Deep risk analysis with severity ratings |
| `scope_estimate.md` | scope-assessor | S/M/L/XL estimate with rationale |
| `spec_audit.md` | spec-auditor | Integrative audit verdict |

### Flow 2: Plan

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `plan_receipt.json` | plan-cleanup | Receipt for downstream flows |
| `impact_map.json` | impact-analyzer | Affected services, modules, data |
| `design_options.md` | design-optioneer | 2-3 architecture options with trade-offs |
| `option_critique.md` | option-critic | Options critique + worklist |
| `adr.md` | adr-author | Architecture decision record |
| `api_contracts.yaml` | interface-designer | Endpoints, schemas, error shapes |
| `schema.md` | interface-designer | Data models, relationships, invariants |
| `contract_critique.md` | contract-critic | Contract validation critique |
| `observability_spec.md` | observability-designer | Metrics, logs, traces, SLOs |
| `observability_critique.md` | observability-critic | Observability validation critique |
| `test_plan.md` | test-strategist | BDD to test types mapping |
| `ac_matrix.md` | test-strategist | AC-driven build contract |
| `work_plan.md` | work-planner | Subtasks, ordering, dependencies |
| `design_validation.md` | design-critic | Feasibility assessment |
| `policy_analysis.md` | policy-analyst | Policy compliance check |

### Flow 3: Build

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `build_receipt.json` | build-cleanup | AC completion tracker, test results |
| `ac_status.json` | build-cleanup | Per-AC pass/fail status |
| `test_changes_summary.md` | test-author | What tests were added/modified |
| `test_critique.md` | test-critic | Test quality assessment |
| `impl_changes_summary.md` | code-implementer | What code changed and why |
| `code_critique.md` | code-critic | Implementation quality assessment |
| `test_execution.md` | test-executor | Test run results |
| `standards_report.md` | standards-enforcer | Format/lint verification |
| `mutation_report.md` | mutation-auditor | Mutation testing results (if run) |
| `flakiness_report.md` | flakiness-detector | Test flakiness analysis (if run) |
| `doc_updates.md` | doc-writer | Documentation changes |
| `doc_critique.md` | doc-critic | Documentation review |
| `self_review.md` | self-reviewer | Final consistency check |
| `pr_creation_status.md` | pr-creator | Draft PR details |
| `pr_feedback.md` | pr-feedback-harvester | Bot/human feedback from PR |

### Flow 4: Review

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `review_receipt.json` | review-cleanup | Worklist resolution summary |
| `pr_feedback.md` | pr-feedback-harvester | Summarized bot + human feedback |
| `review_worklist.md` | review-worklist-writer | Actionable items with stable markers |
| `review_worklist.json` | review-worklist-writer | Machine-readable item tracker |
| `review_actions.md` | review-worklist-writer | Cumulative log of changes made |
| `style_sweep.md` | orchestrator | Style sweep result |
| `cleanup_report.md` | review-cleanup | Cleanup summary |

### Flow 5: Gate

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `gate_receipt.json` | gate-cleanup | Final verification summary |
| `receipt_audit.md` | receipt-checker | Build receipt verification |
| `contract_compliance.md` | contract-enforcer | API contract check results |
| `security_scan.md` | security-scanner | Security scan findings |
| `coverage_audit.md` | coverage-enforcer | Coverage threshold check |
| `gate_fix_summary.md` | gate-fixer | Mechanical issues report + fix-forward plan |
| `fix_forward_report.md` | fix-forward-runner | Runner execution log (if eligible) |
| `traceability_audit.md` | traceability-auditor | Run-level coherence + spec traceability |
| `risk_assessment.md` | risk-analyst | Risk analysis |
| `policy_analysis.md` | policy-analyst | Policy compliance check |
| `merge_decision.md` | merge-decider | MERGE / BOUNCE with reasoning |
| `cleanup_report.md` | gate-cleanup | Cleanup summary |

### Flow 6: Deploy

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `deploy_receipt.json` | deploy-cleanup | Deployment outcome record |
| `deployment_log.md` | repo-operator | Merge, tag, release actions (or why skipped) |
| `verification_report.md` | deploy-monitor, smoke-verifier | CI status + smoke check results |
| `deployment_decision.md` | deploy-decider | STABLE / NOT_DEPLOYED / BLOCKED_BY_GATE |
| `cleanup_report.md` | deploy-cleanup | Cleanup status and evidence |

### Flow 7: Wisdom

| Artifact | Producer | Purpose |
|----------|----------|---------|
| `wisdom_receipt.json` | wisdom-cleanup | Final run receipt |
| `artifact_audit.md` | artifact-auditor | Structural sanity check of all flows |
| `solution_analysis.md` | solution-analyst | Requirement/implementation alignment |
| `quality_report.md` | quality-analyst | Code health, complexity |
| `maintainability_analysis.md` | maintainability-analyst | Naming, modularity, DRY, coupling |
| `process_analysis.md` | process-analyst | Flow efficiency, iterations, bounces |
| `friction_log.md` | process-analyst | Where the swarm hit walls |
| `regression_report.md` | regression-analyst | What got worse and where |
| `pattern_report.md` | pattern-analyst | Cross-run recurring issues and trends |
| `signal_quality_report.md` | signal-quality-analyst | Feedback source accuracy analysis |
| `flow_history.json` | flow-historian | Timeline + DevLT metrics |
| `learnings.md` | learning-synthesizer | Narrative lessons extracted |
| `feedback_actions.md` | feedback-applier | Concrete follow-ups (issues, doc updates) |
| `pack_improvements.md` | feedback-applier | Suggested diffs to pack/agent prompts |
| `codebase_wisdom.md` | feedback-applier | Structural insights for humans |
| `risk_assessment.md` | risk-analyst | Risk perspective (predicted vs actual) |

---

## Common Failure Modes

### Flow 1: Signal

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| Ambiguous requirements | requirements-critic flags testability issues | Rerun requirements microloop with clarification |
| Missing sad paths | bdd-critic flags coverage gaps | Rerun bdd-author with sad path focus |
| GitHub unavailable | `github_ops_allowed: false` | Proceeds locally; binding deferred |
| Scope unclear | scope-assessor returns UNVERIFIED | Document assumptions; proceed |

### Flow 2: Plan

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| No upstream artifacts | Missing requirements/BDD | Proceed best-effort; mark UNVERIFIED |
| Design conflicts | option-critic flags contradictions | Rerun design-optioneer with constraints |
| Contract mismatch | contract-critic flags schema issues | Rerun interface-designer |
| Policy violation | policy-analyst returns blockers | Address policy concerns or defer |

### Flow 3: Build

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| Test failures | test-executor returns non-green | Rerun code-implementer with failures |
| Logic mismatch | code-critic flags ADR contradiction | Route to design-optioneer for micro-decision |
| Missing ac_matrix | No ACs to iterate | Call test-strategist to generate |
| Standards violation | standards-enforcer flags issues | Route to fixer or auto-linter |

### Flow 4: Review

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| No PR exists | `pr_number` is null | Call pr-creator first |
| Stale feedback | Worklist items already fixed | review-worklist-writer marks stale |
| Design feedback | Fundamental concern raised | Route to design-optioneer |
| Stuck loop | worklist-writer reports stuck signal | Checkpoint PARTIAL; rerun |

### Flow 5: Gate

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| Incomplete ACs | `ac_completed < ac_total` | BOUNCE to Flow 3 |
| Security findings | security-scanner flags HIGH/CRITICAL | BOUNCE to Flow 3 |
| Review items pending | `has_critical_pending: true` | BOUNCE to Flow 4 |
| Mechanical drift | gate-fixer flags formatting | Execute fix-forward lane |

### Flow 6: Deploy

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| Gate not MERGE | `gate_verdict != MERGE` | Path B: write NOT_DEPLOYED |
| Merge conflicts | Rebase fails | Write conflict, set NOT_DEPLOYED |
| CI failure | deploy-monitor sees red | Set NOT_DEPLOYED with CI link |
| Smoke failure | smoke-verifier fails checks | Set NOT_DEPLOYED with details |

### Flow 7: Wisdom

| Failure Mode | Symptom | Recovery |
|--------------|---------|----------|
| Missing receipts | Prior flow artifacts absent | Analyze what exists; mark gaps |
| No learnings extracted | learning-synthesizer returns empty | Check if run was trivial |
| Anomaly detected | repo-operator flags dirty state | Checkpoint allowlist only |

---

## When to Skip / Abbreviate

| Flow | Skip When | Abbreviate When |
|------|-----------|-----------------|
| 1 - Signal | Requirements already clear (hotfix, typo) | Small, well-defined change |
| 2 - Plan | Design exists from external source | Trivial implementation path |
| 3 - Build | Code already written manually | Single-file change |
| 4 - Review | No PR feedback to harvest | All feedback already addressed in Build |
| 5 - Gate | Need emergency deployment | Low-risk change with full verification |
| 6 - Deploy | Manual deployment preferred | Gate said BOUNCE |
| 7 - Wisdom | Trivial change with no learnings | Time-critical follow-up needed |

**Note:** Skipping flows produces UNVERIFIED markers. This is expected and acceptable when documented.

---

## Flow Dependencies

```
Flow 1 (Signal)
    |
    v  requirements.md, features/*.feature
Flow 2 (Plan)
    |
    v  adr.md, api_contracts.yaml, ac_matrix.md, work_plan.md
Flow 3 (Build)
    |
    v  build_receipt.json, code, tests, Draft PR
Flow 4 (Review)
    |
    v  review_receipt.json, resolved worklist
Flow 5 (Gate)
    |
    v  merge_decision.md (MERGE/BOUNCE)
Flow 6 (Deploy)
    |
    v  deploy_receipt.json
Flow 7 (Wisdom)
    |
    v  wisdom_receipt.json, learnings.md
```

### What Each Flow Needs from Its Predecessor

| Flow | Required from Previous | Proceeds Without |
|------|------------------------|------------------|
| 2 - Plan | `requirements.md`, `features/*.feature` | Yes (UNVERIFIED) |
| 3 - Build | `adr.md`, `api_contracts.yaml`, `ac_matrix.md` | Yes (UNVERIFIED) |
| 4 - Review | Draft PR, `build_receipt.json` | Creates PR if missing |
| 5 - Gate | `build_receipt.json`, `review_receipt.json` | Yes (UNVERIFIED) |
| 6 - Deploy | `merge_decision.md` | Writes BLOCKED_BY_GATE |
| 7 - Wisdom | All prior receipts | Yes (analyzes what exists) |

---

## Recovery Patterns

### Flow Bounce

When a flow cannot complete, it bounces to an earlier flow.

| From | Bounce To | Trigger | Example |
|------|-----------|---------|---------|
| Gate | Build | Test failures, security issues | `ac_completed < ac_total` |
| Gate | Review | Unaddressed PR feedback | `has_critical_pending: true` |
| Gate | Plan | Design flaws | Contract violations |
| Build | Plan | Architecture invalid | ADR contradiction discovered |
| Review | Build | New code needed | Design feedback requires refactor |

### Local Resolution (Before Bouncing)

Before bouncing to a previous flow, try local resolution:

1. **Call specialist:** Route to `design-optioneer` or `impact-analyzer`
2. **Get micro-decision:** Specialist proposes scoped fix
3. **Apply fix:** Route to implementer with specialist's guidance
4. **Verify:** Run test-executor
5. **Resume:** Continue flow if verified

**Only bounce when specialists agree the issue cannot be resolved locally.**

### Rerun Strategy

When a flow produces PARTIAL or UNVERIFIED:

1. **Read `flow_plan.md`:** Check which steps completed
2. **Call cleanup agent:** Get AC/worklist status
3. **Pre-mark completed items:** Based on agent's report
4. **Resume from pending:** Continue where you left off

### Mechanical Failures

| Symptom | Cause | Fix |
|---------|-------|-----|
| CANNOT_PROCEED | IO/permissions/tooling | Fix environment, rerun |
| `gh` unavailable | Auth or network | Check `gh auth status` |
| Secrets blocked | `safe_to_publish: false` | Route to fixer or manual remediation |

---

## Per-Flow Quick Reference

### Flow 1: Signal

**Purpose:** Transform messy input into testable requirements.

**Key agents:** gh-issue-resolver, signal-normalizer, problem-framer, requirements-author, requirements-critic, bdd-author, bdd-critic, scope-assessor, risk-analyst, spec-auditor

**Main outputs:** `signal_receipt.json`, `requirements.md`, `features/*.feature`, `scope_estimate.md`

**Common issues:** Ambiguous input, missing sad paths, GitHub unavailable

**Skip when:** Requirements already documented externally

**Details:** [.claude/commands/flow-1-signal.md](../../.claude/commands/flow-1-signal.md)

---

### Flow 2: Plan

**Purpose:** Turn requirements into architecture and execution plan.

**Key agents:** impact-analyzer, design-optioneer, option-critic, adr-author, interface-designer, contract-critic, observability-designer, observability-critic, test-strategist, work-planner, design-critic, policy-analyst

**Main outputs:** `plan_receipt.json`, `adr.md`, `api_contracts.yaml`, `ac_matrix.md`, `work_plan.md`

**Common issues:** Missing upstream, design conflicts, policy violations

**Skip when:** Design exists from external source

**Details:** [.claude/commands/flow-2-plan.md](../../.claude/commands/flow-2-plan.md)

---

### Flow 3: Build

**Purpose:** Build working, codebase-aligned implementation.

**Key agents:** test-author, test-critic, code-implementer, code-critic, test-executor, standards-enforcer, mutation-auditor, pr-creator

**Main outputs:** `build_receipt.json`, `ac_status.json`, code, tests, Draft PR

**Common issues:** Test failures, logic mismatches, missing ac_matrix

**Skip when:** Code already written and tested

**Details:** [.claude/commands/flow-3-build.md](../../.claude/commands/flow-3-build.md)

---

### Flow 4: Review

**Purpose:** Harvest feedback and resolve issues before merge.

**Key agents:** pr-feedback-harvester, review-worklist-writer, fixer, code-implementer, test-author, pr-commenter, pr-status-manager

**Main outputs:** `review_receipt.json`, `review_worklist.json`, resolved items

**Common issues:** No PR exists, stale feedback, design concerns

**Skip when:** No external feedback to process

**Details:** [.claude/commands/flow-4-review.md](../../.claude/commands/flow-4-review.md)

---

### Flow 5: Gate

**Purpose:** Decide merge or bounce with evidence.

**Key agents:** receipt-checker, contract-enforcer, security-scanner, coverage-enforcer, gate-fixer, fix-forward-runner, traceability-auditor, risk-analyst, policy-analyst, merge-decider

**Main outputs:** `gate_receipt.json`, `merge_decision.md`

**Common issues:** Incomplete ACs, security findings, pending review items

**Skip when:** Emergency deployment (document gaps)

**Details:** [.claude/commands/flow-5-gate.md](../../.claude/commands/flow-5-gate.md)

---

### Flow 6: Deploy

**Purpose:** Execute merge and verify deployment.

**Key agents:** deploy-decider, repo-operator, deploy-monitor, smoke-verifier

**Main outputs:** `deploy_receipt.json`, `deployment_decision.md`, `verification_report.md`

**Common issues:** Gate not MERGE, merge conflicts, CI failures

**Skip when:** Manual deployment process

**Details:** [.claude/commands/flow-6-deploy.md](../../.claude/commands/flow-6-deploy.md)

---

### Flow 7: Wisdom

**Purpose:** Extract learnings and close feedback loops.

**Key agents:** artifact-auditor, solution-analyst, quality-analyst, maintainability-analyst, process-analyst, regression-analyst, pattern-analyst, signal-quality-analyst, flow-historian, learning-synthesizer, feedback-applier, traceability-auditor, risk-analyst

**Main outputs:** `wisdom_receipt.json`, `learnings.md`, `pack_improvements.md`

**Common issues:** Missing receipts, no patterns to extract

**Skip when:** Trivial change with no learnings value

**Details:** [.claude/commands/flow-7-wisdom.md](../../.claude/commands/flow-7-wisdom.md)

---

## See Also

- [why-seven-flows.md](../explanation/why-seven-flows.md) --- Why these seven, not more or fewer
- [flow-flexibility.md](../explanation/flow-flexibility.md) --- When to skip or abbreviate
- [routing-table.md](routing-table.md) --- How agents route between flows
- [CLAUDE.md](../../CLAUDE.md) --- The Seven Flows section
