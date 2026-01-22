# Agent Data Flows

> Producer → Consumer relationships between agents.

This document maps which agents produce outputs that feed into which other agents. Understanding these dependencies is essential for debugging flow issues and customizing agent behavior.

---

## Flow 1: Signal

```
┌─────────────────────┐
│  gh-issue-resolver  │ (GitHub issue → run_id, issue metadata)
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   signal-run-prep   │ (creates .runs/<run-id>/signal/)
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  signal-normalizer  │ → issue_normalized.md, context_brief.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   problem-framer    │ → problem_statement.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   scope-assessor    │ → stakeholders.md, early_risks.md, scope_estimate.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐      ┌────────────────────┐
│ requirements-author │ ←──→ │ requirements-critic │ (microloop)
└──────────┬──────────┘      └────────────────────┘
           │                   ▲
           ▼                   │
    requirements.md ───────────┘
           │
           ▼
┌─────────────────────┐      ┌────────────────────┐
│    bdd-author       │ ←──→ │    bdd-critic      │ (microloop)
└──────────┬──────────┘      └────────────────────┘
           │
           ▼
    features/*.feature
    example_matrix.md
    verification_notes.md
           │
           ▼
┌─────────────────────┐
│    risk-analyst     │ → risk_assessment.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   signal-cleanup    │ → signal_receipt.json
└─────────────────────┘
```

### Flow 1 Artifact Dependencies

| Producer              | Artifact                                                           | Consumers                                            |
| --------------------- | ------------------------------------------------------------------ | ---------------------------------------------------- |
| `gh-issue-resolver`   | `run_id`, issue metadata                                           | `signal-run-prep`                                    |
| `signal-run-prep`     | `.runs/<run-id>/signal/` directory                                 | All Flow 1 agents                                    |
| `signal-normalizer`   | `issue_normalized.md`, `context_brief.md`                          | `problem-framer`                                     |
| `problem-framer`      | `problem_statement.md`                                             | `scope-assessor`, `requirements-author`              |
| `scope-assessor`      | `stakeholders.md`, `early_risks.md`, `scope_estimate.md`           | `requirements-author`, `risk-analyst`                |
| `requirements-author` | `requirements.md`                                                  | `requirements-critic`, `bdd-author`, Flow 2+3 agents |
| `requirements-critic` | `requirements_critique.md`                                         | `requirements-author` (rerun), Flow 2 agents         |
| `bdd-author`          | `features/*.feature`, `example_matrix.md`, `verification_notes.md` | `bdd-critic`, `test-strategist`, `test-author`       |
| `bdd-critic`          | `bdd_critique.md`                                                  | `bdd-author` (rerun), Flow 2 agents                  |
| `risk-analyst`        | `risk_assessment.md`                                               | Flow 2 design agents, `test-strategist`              |
| `signal-cleanup`      | `signal_receipt.json`                                              | Flow 2 `run-prep`, reporters                         |

---

## Flow 2: Plan

```
┌─────────────────────┐
│      run-prep       │ (creates .runs/<run-id>/plan/)
└──────────┬──────────┘
           ▼
    [reads signal/ artifacts]
           │
           ▼
┌─────────────────────┐      ┌────────────────────┐
│  design-optioneer   │ ←──→ │   option-critic    │ (microloop)
└──────────┬──────────┘      └────────────────────┘
           │
           ▼
    design_options.md
           │
           ▼
┌─────────────────────┐
│     adr-author      │ → adr.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐      ┌────────────────────┐
│ interface-designer  │ ←──→ │  contract-critic   │ (microloop)
└──────────┬──────────┘      └────────────────────┘
           │
           ▼
    api_contracts.yaml, schema.md
           │
           ▼
┌─────────────────────────┐    ┌─────────────────────────┐
│ observability-designer  │ ←→ │  observability-critic   │ (microloop)
└──────────┬──────────────┘    └─────────────────────────┘
           │
           ▼
    observability_spec.md
           │
           ▼
┌─────────────────────┐
│   test-strategist   │ → test_plan.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│    work-planner     │ → work_plan.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│    design-critic    │ → design_validation.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   policy-analyst    │ → policy_analysis.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│    plan-cleanup     │ → plan_receipt.json, cleanup_report.md
└─────────────────────┘
```

### Flow 2 Artifact Dependencies

| Producer                 | Artifact                          | Consumers                                                  |
| ------------------------ | --------------------------------- | ---------------------------------------------------------- |
| `run-prep`               | `.runs/<run-id>/plan/` directory  | All Flow 2 agents                                          |
| `design-optioneer`       | `design_options.md`               | `option-critic`, `adr-author`                              |
| `option-critic`          | `option_critique.md`              | `design-optioneer` (rerun)                                 |
| `adr-author`             | `adr.md`                          | `interface-designer`, `code-implementer`, `design-critic`  |
| `interface-designer`     | `api_contracts.yaml`, `schema.md` | `contract-critic`, `contract-enforcer`, `code-implementer` |
| `contract-critic`        | `contract_critique.md`            | `interface-designer` (rerun)                               |
| `observability-designer` | `observability_spec.md`           | `observability-critic`, `code-implementer`                 |
| `observability-critic`   | `observability_critique.md`       | `observability-designer` (rerun)                           |
| `test-strategist`        | `test_plan.md`                    | `test-author`, `coverage-enforcer`                         |
| `work-planner`           | `work_plan.md`                    | `context-loader`, Flow 3 orchestrator                      |
| `design-critic`          | `design_validation.md`            | Flow 3, `merge-decider`                                    |
| `policy-analyst`         | `policy_analysis.md`              | `merge-decider`, reporters                                 |
| `plan-cleanup`           | `plan_receipt.json`               | Flow 3 `run-prep`, reporters                               |

---

## Flow 3: Build

```
┌─────────────────────┐
│      run-prep       │ (creates .runs/<run-id>/build/)
└──────────┬──────────┘
           ▼
    [reads plan/ artifacts]
           │
           ▼
┌─────────────────────┐
│   context-loader    │ → subtask_context_manifest.json
└──────────┬──────────┘
           │
           ▼
    ┌─────────────────────────────────────────────────┐
    │           Build Microloop (per subtask)         │
    │                                                 │
    │  ┌─────────────┐      ┌────────────────┐       │
    │  │ test-author │ ←──→ │  test-critic   │       │
    │  └──────┬──────┘      └────────────────┘       │
    │         ▼                                      │
    │    test files + test_changes_summary.md        │
    │         │                                      │
    │         ▼                                      │
    │  ┌──────────────────┐   ┌────────────────┐     │
    │  │ code-implementer │ ← │   code-critic  │     │
    │  └───────┬──────────┘   └────────────────┘     │
    │          ▼                                     │
    │    code + impl_changes_summary.md              │
    │          │                                     │
    │          ▼                                     │
    │  ┌───────────────┐                             │
    │  │ test-executor │ → test_execution.md         │
    │  └───────────────┘                             │
    └─────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────┐      ┌────────────────────┐
│     doc-writer      │ ←──→ │    doc-critic      │
└──────────┬──────────┘      └────────────────────┘
           │
           ▼
    doc_updates.md + project docs
           │
           ▼
┌─────────────────────┐
│   self-reviewer     │ → self_review.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   build-cleanup     │ → build_receipt.json
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│    pr-creator       │ → creates Draft PR, updates run_meta.json
└──────────┬──────────┘
           ▼
┌─────────────────────────┐
│  pr-feedback-harvester  │ → build/pr_feedback.md
└─────────────────────────┘
```

### Flow 3 Artifact Dependencies

| Producer                | Artifact                                 | Consumers                                             |
| ----------------------- | ---------------------------------------- | ----------------------------------------------------- |
| `run-prep`              | `.runs/<run-id>/build/` directory        | All Flow 3 agents                                     |
| `context-loader`        | `subtask_context_manifest.json`          | `test-author`, `code-implementer`                     |
| `test-author`           | Test files + `test_changes_summary.md`   | `test-critic`, `test-executor`, `code-implementer`    |
| `test-critic`           | `test_critique.md`                       | `test-author` (rerun), `self-reviewer`                |
| `code-implementer`      | Code files + `impl_changes_summary.md`   | `code-critic`, `test-executor`, `doc-writer`          |
| `code-critic`           | `code_critique.md`                       | `code-implementer` (rerun), `self-reviewer`           |
| `test-executor`         | `test_execution.md`                      | `self-reviewer`, `build-cleanup`, `coverage-enforcer` |
| `doc-writer`            | `doc_updates.md` + project docs          | `doc-critic`, `self-reviewer`                         |
| `doc-critic`            | `doc_critique.md`                        | `doc-writer` (rerun), `self-reviewer`                 |
| `self-reviewer`         | `self_review.md`                         | `build-cleanup`                                       |
| `build-cleanup`         | `build_receipt.json`                     | Flow 4, Flow 5, reporters                             |
| `pr-creator`            | Draft PR, `pr_number` in `run_meta.json` | `pr-feedback-harvester`, Flow 4                       |
| `pr-feedback-harvester` | `build/pr_feedback.md`                   | Flow 3 routing (blockers), Flow 4                     |

---

## Flow 4: Review

```
┌─────────────────────┐
│      run-prep       │ (creates .runs/<run-id>/review/)
└──────────┬──────────┘
           ▼
┌─────────────────────────┐
│  pr-feedback-harvester  │ → review/pr_feedback.md
└──────────┬──────────────┘
           ▼
┌─────────────────────────┐
│ review-worklist-writer  │ → review/review_worklist.md
└──────────┬──────────────┘
           │
           ▼
    ┌─────────────────────────────────────────────────┐
    │            Review Fix Loop                      │
    │                                                 │
    │  For each worklist item:                        │
    │  ┌─────────┐                                    │
    │  │  fixer  │ → fix_summary.md, code changes     │
    │  └─────────┘                                    │
    │                                                 │
    └─────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────┐
│   review-cleanup    │ → review_receipt.json
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  pr-status-manager  │ → PR state transitions (Draft → Ready)
└─────────────────────┘
```

### Flow 4 Artifact Dependencies

| Producer                 | Artifact                           | Consumers                             |
| ------------------------ | ---------------------------------- | ------------------------------------- |
| `run-prep`               | `.runs/<run-id>/review/` directory | All Flow 4 agents                     |
| `pr-feedback-harvester`  | `review/pr_feedback.md`            | `review-worklist-writer`              |
| `review-worklist-writer` | `review/review_worklist.md`        | `fixer`, `review-cleanup`             |
| `fixer`                  | `fix_summary.md`, code changes     | `review-cleanup`, `pr-status-manager` |
| `review-cleanup`         | `review_receipt.json`              | Flow 5, reporters                     |
| `pr-status-manager`      | PR state changes                   | Flow 5                                |

---

## Flow 5: Gate

```
┌─────────────────────┐
│      run-prep       │ (creates .runs/<run-id>/gate/)
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   receipt-checker   │ → receipt_audit.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  contract-enforcer  │ → contract_compliance.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  security-scanner   │ → security_scan.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  coverage-enforcer  │ → coverage_audit.md
└──────────┬──────────┘
           ▼
┌─────────────────────────┐
│  traceability-auditor   │ → traceability_audit.md
└──────────┬──────────────┘
           ▼
┌─────────────────────┐
│    merge-decider    │ → merge_decision.md (MERGE | BOUNCE)
└──────────┬──────────┘
           │
           ▼ (if BOUNCE with mechanical fixes possible)
    ┌─────────────────────────────────────────────────┐
    │            Fix-Forward Lane                     │
    │                                                 │
    │  ┌────────────┐                                 │
    │  │ gate-fixer │ → gate_fix_summary.md +         │
    │  └─────┬──────┘   FIX_FORWARD_PLAN_V1 block     │
    │        ▼                                        │
    │  ┌────────────────────┐                         │
    │  │ fix-forward-runner │ → fix_forward_report.md │
    │  └────────────────────┘                         │
    └─────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────┐
│    gate-cleanup     │ → gate_receipt.json
└─────────────────────┘
```

### Flow 5 Artifact Dependencies

| Producer               | Artifact                                     | Consumers                                        |
| ---------------------- | -------------------------------------------- | ------------------------------------------------ |
| `run-prep`             | `.runs/<run-id>/gate/` directory             | All Flow 5 agents                                |
| `receipt-checker`      | `receipt_audit.md`                           | `merge-decider`                                  |
| `contract-enforcer`    | `contract_compliance.md`                     | `merge-decider`                                  |
| `security-scanner`     | `security_scan.md`                           | `merge-decider`                                  |
| `coverage-enforcer`    | `coverage_audit.md`                          | `merge-decider`                                  |
| `traceability-auditor` | `traceability_audit.md`                      | `merge-decider`                                  |
| `merge-decider`        | `merge_decision.md`                          | `gate-fixer` (on BOUNCE), `gate-cleanup`, Flow 6 |
| `gate-fixer`           | `gate_fix_summary.md`, `FIX_FORWARD_PLAN_V1` | `fix-forward-runner`                             |
| `fix-forward-runner`   | `fix_forward_report.md`                      | `gate-cleanup`                                   |
| `gate-cleanup`         | `gate_receipt.json`                          | Flow 6, reporters                                |

---

## Flow 6: Deploy

```
┌─────────────────────┐
│      run-prep       │ (creates .runs/<run-id>/deploy/)
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   deploy-decider    │ → deployment_decision.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   deploy-monitor    │ → verification_report.md (CI + deploy signals)
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   smoke-verifier    │ → appends to verification_report.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   deploy-cleanup    │ → deploy_receipt.json
└─────────────────────┘
```

### Flow 6 Artifact Dependencies

| Producer         | Artifact                            | Consumers                          |
| ---------------- | ----------------------------------- | ---------------------------------- |
| `run-prep`       | `.runs/<run-id>/deploy/` directory  | All Flow 6 agents                  |
| `deploy-decider` | `deployment_decision.md`            | `deploy-monitor`, `deploy-cleanup` |
| `deploy-monitor` | `verification_report.md`            | `smoke-verifier`, `deploy-cleanup` |
| `smoke-verifier` | `verification_report.md` (appended) | `deploy-cleanup`                   |
| `deploy-cleanup` | `deploy_receipt.json`               | Flow 7, reporters                  |

---

## Flow 7: Wisdom

```
┌─────────────────────┐
│      run-prep       │ (creates .runs/<run-id>/wisdom/)
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   flow-historian    │ → flow_history.json
└──────────┬──────────┘
           ▼
┌─────────────────────────┐
│   regression-analyst    │ → regression_report.md
└──────────┬──────────────┘
           ▼
┌─────────────────────────┐
│  learning-synthesizer   │ → learnings.md
└──────────┬──────────────┘
           ▼
┌─────────────────────┐
│   feedback-applier  │ → feedback_actions.md
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   wisdom-cleanup    │ → wisdom_receipt.json
└─────────────────────┘
```

### Flow 7 Artifact Dependencies

| Producer               | Artifact                           | Consumers                                    |
| ---------------------- | ---------------------------------- | -------------------------------------------- |
| `run-prep`             | `.runs/<run-id>/wisdom/` directory | All Flow 7 agents                            |
| `flow-historian`       | `flow_history.json`                | `regression-analyst`, `learning-synthesizer` |
| `regression-analyst`   | `regression_report.md`             | `learning-synthesizer`, `feedback-applier`   |
| `learning-synthesizer` | `learnings.md`                     | `feedback-applier`, `wisdom-cleanup`         |
| `feedback-applier`     | `feedback_actions.md`              | `wisdom-cleanup`                             |
| `wisdom-cleanup`       | `wisdom_receipt.json`              | Reporters, future runs                       |

---

## Cross-Flow Dependencies

These artifacts are produced in earlier flows and consumed in later flows:

| Artifact              | Producer Flow | Consumer Flows |
| --------------------- | ------------- | -------------- |
| `requirements.md`     | 1 (Signal)    | 2, 3           |
| `features/*.feature`  | 1 (Signal)    | 2, 3           |
| `adr.md`              | 2 (Plan)      | 3, 5           |
| `api_contracts.yaml`  | 2 (Plan)      | 3, 5           |
| `test_plan.md`        | 2 (Plan)      | 3, 5           |
| `work_plan.md`        | 2 (Plan)      | 3              |
| `build_receipt.json`  | 3 (Build)     | 4, 5, 7        |
| `test_execution.md`   | 3 (Build)     | 5              |
| `review_receipt.json` | 4 (Review)    | 5              |
| `gate_receipt.json`   | 5 (Gate)      | 6, 7           |
| `merge_decision.md`   | 5 (Gate)      | 6              |
| `deploy_receipt.json` | 6 (Deploy)    | 7              |
| All receipts          | 1-6           | 7 (Wisdom)     |

---

## Cross-Flow Agents

These agents run in multiple flows with different responsibilities:

| Agent                   | Flow Usage                   |
| ----------------------- | ---------------------------- |
| `repo-operator`         | All flows (git side effects) |
| `secrets-sanitizer`     | All flows (publish gate)     |
| `gh-issue-manager`      | All flows (issue sync)       |
| `gh-reporter`           | All flows (flow summaries)   |
| `clarifier`             | Any flow (open questions)    |
| `risk-analyst`          | 1, 2, 5 (risk tracking)      |
| `policy-analyst`        | 2, 5 (policy mapping)        |
| `fixer`                 | 3, 4 (apply fixes)           |
| `pr-feedback-harvester` | 3, 4 (harvest feedback)      |

---

## See Also

- [agents-index.md](agents-index.md) — Master agent listing
- [contracts.md](contracts.md) — Control-plane blocks and schemas
- [stable-markers.md](stable-markers.md) — Marker prefixes for counting
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
