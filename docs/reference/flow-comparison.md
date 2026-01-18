# Flow Comparison Reference

> Quick reference comparing all 7 flows side-by-side.

---

## Summary Table

| Flow | Name   | Primary Input                | Primary Output                                 | Key Agents                                                        | Typical Duration |
| ---- | ------ | ---------------------------- | ---------------------------------------------- | ----------------------------------------------------------------- | ---------------- |
| 1    | Signal | Feature request / issue      | `signal_receipt.json`, requirements, BDD       | gh-issue-resolver, requirements-author, bdd-author, spec-auditor  | 15-30 min        |
| 2    | Plan   | Requirements + BDD scenarios | `plan_receipt.json`, ADR, contracts, work plan | impact-analyzer, design-optioneer, adr-author, interface-designer | 20-40 min        |
| 3    | Build  | ADR + contracts + work plan  | `build_receipt.json`, code, tests, Draft PR    | test-author, code-implementer, test-executor, pr-creator          | 30-90 min        |
| 4    | Review | Draft PR + feedback          | `review_receipt.json`, resolved worklist       | pr-feedback-harvester, review-worklist-writer, fixer              | 15-45 min        |
| 5    | Gate   | Build + Review receipts      | `gate_receipt.json`, merge decision            | receipt-checker, security-scanner, merge-decider                  | 10-25 min        |
| 6    | Deploy | Gate MERGE decision          | `deploy_receipt.json`, deployment verdict      | deploy-decider, repo-operator, deploy-monitor                     | 5-15 min         |
| 7    | Wisdom | All prior receipts           | `wisdom_receipt.json`, learnings               | solution-analyst, quality-analyst, learning-synthesizer           | 20-40 min        |

---

## Agent Count per Flow

| Flow       | Domain Agents | Cross-Cutting Agents | Infrastructure Agents | Total |
| ---------- | ------------- | -------------------- | --------------------- | ----- |
| 1 - Signal | 8             | 3                    | 6                     | 17    |
| 2 - Plan   | 10            | 4                    | 5                     | 19    |
| 3 - Build  | 13            | 3                    | 6                     | 22    |
| 4 - Review | 6             | 2                    | 6                     | 14    |
| 5 - Gate   | 7             | 3                    | 5                     | 15    |
| 6 - Deploy | 4             | 0                    | 5                     | 9     |
| 7 - Wisdom | 12            | 2                    | 5                     | 19    |

---

## Key Artifacts per Flow

### Flow 1: Signal

| Artifact               | Producer            | Purpose                                  |
| ---------------------- | ------------------- | ---------------------------------------- |
| `signal_receipt.json`  | signal-cleanup      | Machine-readable summary for downstream  |
| `requirements.md`      | requirements-author | Functional + non-functional requirements |
| `features/*.feature`   | bdd-author          | BDD scenarios in Gherkin format          |
| `problem_statement.md` | problem-framer      | Goals, non-goals, constraints            |
| `early_risks.md`       | scope-assessor      | Initial risk identification              |
| `scope_estimate.md`    | scope-assessor      | S/M/L/XL estimate with rationale         |

### Flow 2: Plan

| Artifact                | Producer               | Purpose                          |
| ----------------------- | ---------------------- | -------------------------------- |
| `plan_receipt.json`     | plan-cleanup           | Receipt for downstream flows     |
| `adr.md`                | adr-author             | Architecture decision record     |
| `api_contracts.yaml`    | interface-designer     | Endpoints, schemas, error shapes |
| `work_plan.md`          | work-planner           | Subtasks, ordering, dependencies |
| `ac_matrix.md`          | test-strategist        | AC-driven build contract         |
| `observability_spec.md` | observability-designer | Metrics, logs, traces, SLOs      |

### Flow 3: Build

| Artifact                  | Producer         | Purpose                             |
| ------------------------- | ---------------- | ----------------------------------- |
| `build_receipt.json`      | build-cleanup    | AC completion tracker, test results |
| `ac_status.json`          | build-cleanup    | Per-AC pass/fail status             |
| `impl_changes_summary.md` | code-implementer | What code changed and why           |
| `test_changes_summary.md` | test-author      | What tests were added/modified      |
| `code_critique.md`        | code-critic      | Implementation quality assessment   |
| `pr_creation_status.md`   | pr-creator       | Draft PR details                    |

### Flow 4: Review

| Artifact               | Producer               | Purpose                         |
| ---------------------- | ---------------------- | ------------------------------- |
| `review_receipt.json`  | review-cleanup         | Worklist resolution summary     |
| `review_worklist.json` | review-worklist-writer | Machine-readable item tracker   |
| `pr_feedback.md`       | pr-feedback-harvester  | Summarized bot + human feedback |
| `review_actions.md`    | orchestrator           | Cumulative log of changes made  |

### Flow 5: Gate

| Artifact                | Producer             | Purpose                       |
| ----------------------- | -------------------- | ----------------------------- |
| `gate_receipt.json`     | gate-cleanup         | Final verification summary    |
| `merge_decision.md`     | merge-decider        | MERGE / BOUNCE with reasoning |
| `receipt_audit.md`      | receipt-checker      | Build receipt verification    |
| `security_scan.md`      | security-scanner     | Security scan findings        |
| `traceability_audit.md` | traceability-auditor | REQ-BDD-Code coherence        |

### Flow 6: Deploy

| Artifact                 | Producer       | Purpose                                 |
| ------------------------ | -------------- | --------------------------------------- |
| `deploy_receipt.json`    | deploy-cleanup | Deployment outcome record               |
| `deployment_decision.md` | deploy-decider | STABLE / NOT_DEPLOYED / BLOCKED_BY_GATE |
| `deployment_log.md`      | repo-operator  | Merge, tag, release actions             |
| `verification_report.md` | deploy-monitor | CI status + smoke check results         |

### Flow 7: Wisdom

| Artifact               | Producer             | Purpose                          |
| ---------------------- | -------------------- | -------------------------------- |
| `wisdom_receipt.json`  | wisdom-cleanup       | Final run receipt                |
| `learnings.md`         | learning-synthesizer | Narrative lessons extracted      |
| `friction_log.md`      | process-analyst      | Where the swarm hit walls        |
| `pack_improvements.md` | feedback-applier     | Suggested diffs to agent prompts |
| `quality_report.md`    | quality-analyst      | Code health assessment           |

---

## Common Failure Modes

### Flow 1: Signal

| Failure Mode           | Symptom                                      | Recovery                                        |
| ---------------------- | -------------------------------------------- | ----------------------------------------------- |
| Ambiguous requirements | requirements-critic flags testability issues | Rerun requirements microloop with clarification |
| Missing sad paths      | bdd-critic flags coverage gaps               | Rerun bdd-author with sad path focus            |
| GitHub unavailable     | `github_ops_allowed: false`                  | Proceeds locally; binding deferred              |
| Scope unclear          | scope-assessor returns UNVERIFIED            | Document assumptions; proceed                   |

### Flow 2: Plan

| Failure Mode          | Symptom                             | Recovery                                |
| --------------------- | ----------------------------------- | --------------------------------------- |
| No upstream artifacts | Missing requirements/BDD            | Proceed best-effort; mark UNVERIFIED    |
| Design conflicts      | option-critic flags contradictions  | Rerun design-optioneer with constraints |
| Contract mismatch     | contract-critic flags schema issues | Rerun interface-designer                |
| Policy violation      | policy-analyst returns blockers     | Address policy concerns or defer        |

### Flow 3: Build

| Failure Mode        | Symptom                             | Recovery                                     |
| ------------------- | ----------------------------------- | -------------------------------------------- |
| Test failures       | test-executor returns non-green     | Rerun code-implementer with failures         |
| Logic mismatch      | code-critic flags ADR contradiction | Route to design-optioneer for micro-decision |
| Missing ac_matrix   | No ACs to iterate                   | Call test-strategist to generate             |
| Standards violation | standards-enforcer flags issues     | Route to fixer or auto-linter                |

### Flow 4: Review

| Failure Mode    | Symptom                              | Recovery                           |
| --------------- | ------------------------------------ | ---------------------------------- |
| No PR exists    | `pr_number` is null                  | Call pr-creator first              |
| Stale feedback  | Worklist items already fixed         | review-worklist-writer marks stale |
| Design feedback | Fundamental concern raised           | Route to design-optioneer          |
| Stuck loop      | worklist-writer reports stuck signal | Checkpoint PARTIAL; rerun          |

### Flow 5: Gate

| Failure Mode         | Symptom                              | Recovery                 |
| -------------------- | ------------------------------------ | ------------------------ |
| Incomplete ACs       | `ac_completed < ac_total`            | BOUNCE to Flow 3         |
| Security findings    | security-scanner flags HIGH/CRITICAL | BOUNCE to Flow 3         |
| Review items pending | `has_critical_pending: true`         | BOUNCE to Flow 4         |
| Mechanical drift     | gate-fixer flags formatting          | Execute fix-forward lane |

### Flow 6: Deploy

| Failure Mode    | Symptom                     | Recovery                         |
| --------------- | --------------------------- | -------------------------------- |
| Gate not MERGE  | `gate_verdict != MERGE`     | Path B: write NOT_DEPLOYED       |
| Merge conflicts | Rebase fails                | Write conflict, set NOT_DEPLOYED |
| CI failure      | deploy-monitor sees red     | Set NOT_DEPLOYED with CI link    |
| Smoke failure   | smoke-verifier fails checks | Set NOT_DEPLOYED with details    |

### Flow 7: Wisdom

| Failure Mode           | Symptom                            | Recovery                       |
| ---------------------- | ---------------------------------- | ------------------------------ |
| Missing receipts       | Prior flow artifacts absent        | Analyze what exists; mark gaps |
| No learnings extracted | learning-synthesizer returns empty | Check if run was trivial       |
| Anomaly detected       | repo-operator flags dirty state    | Checkpoint allowlist only      |

---

## When to Skip / Abbreviate

| Flow       | Skip When                                 | Abbreviate When                         |
| ---------- | ----------------------------------------- | --------------------------------------- |
| 1 - Signal | Requirements already clear (hotfix, typo) | Small, well-defined change              |
| 2 - Plan   | Design exists from external source        | Trivial implementation path             |
| 3 - Build  | Code already written manually             | Single-file change                      |
| 4 - Review | No PR feedback to harvest                 | All feedback already addressed in Build |
| 5 - Gate   | Need emergency deployment                 | Low-risk change with full verification  |
| 6 - Deploy | Manual deployment preferred               | Gate said BOUNCE                        |
| 7 - Wisdom | Trivial change with no learnings          | Time-critical follow-up needed          |

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

| Flow       | Required from Previous                         | Proceeds Without           |
| ---------- | ---------------------------------------------- | -------------------------- |
| 2 - Plan   | `requirements.md`, `features/*.feature`        | Yes (UNVERIFIED)           |
| 3 - Build  | `adr.md`, `api_contracts.yaml`, `ac_matrix.md` | Yes (UNVERIFIED)           |
| 4 - Review | Draft PR, `build_receipt.json`                 | Creates PR if missing      |
| 5 - Gate   | `build_receipt.json`, `review_receipt.json`    | Yes (UNVERIFIED)           |
| 6 - Deploy | `merge_decision.md`                            | Writes BLOCKED_BY_GATE     |
| 7 - Wisdom | All prior receipts                             | Yes (analyzes what exists) |

---

## Recovery Patterns

### Flow Bounce

When a flow cannot complete, it bounces to an earlier flow.

| From   | Bounce To | Trigger                        | Example                           |
| ------ | --------- | ------------------------------ | --------------------------------- |
| Gate   | Build     | Test failures, security issues | `ac_completed < ac_total`         |
| Gate   | Review    | Unaddressed PR feedback        | `has_critical_pending: true`      |
| Gate   | Plan      | Design flaws                   | Contract violations               |
| Build  | Plan      | Architecture invalid           | ADR contradiction discovered      |
| Review | Build     | New code needed                | Design feedback requires refactor |

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

| Symptom          | Cause                    | Fix                                  |
| ---------------- | ------------------------ | ------------------------------------ |
| CANNOT_PROCEED   | IO/permissions/tooling   | Fix environment, rerun               |
| `gh` unavailable | Auth or network          | Check `gh auth status`               |
| Secrets blocked  | `safe_to_publish: false` | Route to fixer or manual remediation |

---

## Per-Flow Quick Reference

### Flow 1: Signal

**Purpose:** Transform messy input into testable requirements.

**Key agents:** gh-issue-resolver, signal-normalizer, problem-framer, requirements-author, requirements-critic, bdd-author, bdd-critic, spec-auditor

**Main outputs:** `signal_receipt.json`, `requirements.md`, `features/*.feature`

**Common issues:** Ambiguous input, missing sad paths, GitHub unavailable

**Skip when:** Requirements already documented externally

**Details:** [.claude/commands/flow-1-signal.md](../../.claude/commands/flow-1-signal.md)

---

### Flow 2: Plan

**Purpose:** Turn requirements into architecture and execution plan.

**Key agents:** impact-analyzer, design-optioneer, option-critic, adr-author, interface-designer, contract-critic, test-strategist, work-planner, design-critic

**Main outputs:** `plan_receipt.json`, `adr.md`, `api_contracts.yaml`, `work_plan.md`

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

**Key agents:** receipt-checker, contract-enforcer, security-scanner, coverage-enforcer, gate-fixer, traceability-auditor, merge-decider

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

**Key agents:** artifact-auditor, solution-analyst, quality-analyst, maintainability-analyst, process-analyst, regression-analyst, pattern-analyst, learning-synthesizer, feedback-applier

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
