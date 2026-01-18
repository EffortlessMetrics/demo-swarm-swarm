# Agents Index

> Master listing of all domain agents in the DemoSwarm pack.

This index lists agents organized by role family. For behavior details, see individual agent files in `.claude/agents/`.

---

## Quick Reference

| Family                                         | Color  | Count  | Role                                 |
| ---------------------------------------------- | ------ | ------ | ------------------------------------ |
| [Shaping](#shaping-agents-yellow)              | Yellow | 7      | Early signal processing and framing  |
| [Spec](#spec-agents-purple)                    | Purple | 9      | Requirements, design, and planning   |
| [Implementation](#implementation-agents-green) | Green  | 7      | Writing code, tests, and docs        |
| [Critic](#critic-agents-red)                   | Red    | 11     | Harsh review (never fixes)           |
| [Verification](#verification-agents-blue)      | Blue   | 13     | Audits, checks, and compliance       |
| [Analytics](#analytics-agents-orange)          | Orange | 11     | Analysis, learning, and insights     |
| [Infra](#infra-agents-cyan)                    | Cyan   | 2      | Repo and run infrastructure          |
| [Reporter](#reporter-agents-pink)              | Pink   | 1      | GitHub posting                       |
| **Listed**                                     |        | **61** | (see `.claude/agents/` for full set) |

Note: Some agents appear in multiple categories based on their color/role.

---

## Shaping Agents (Yellow)

Early signal processing and problem framing.

| Agent               | Description                                                                                                                                   | Primary Flows |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `clarifier`         | Detect ambiguities and log answerable questions + explicit defaults (append-only) → `open_questions.md`                                       | 1, 2, 3       |
| `gh-issue-manager`  | Ensure GitHub issue exists and keep run identity metadata in sync. Writes `gh_issue_status.md` + updates `run_meta.json` + `.runs/index.json` | All           |
| `gh-researcher`     | Read-only GitHub reconnaissance (issues/PRs/discussions + local prior art pointers) → `github_research.md`                                    | 1             |
| `pack-customizer`   | Detect repo conventions and adapt DemoSwarm pack to match the target stack. Writes `demo-swarm.config.json`                                   | Setup         |
| `problem-framer`    | Synthesize normalized signal → `problem_statement.md`                                                                                         | 1             |
| `signal-normalizer` | Normalize raw signal into machine-friendly facts + repo context → `issue_normalized.md`, `context_brief.md`                                   | 1             |
| `signal-run-prep`   | Establish or reattach Flow 1 run infrastructure. Creates `.runs/<run-id>/signal/*`                                                            | 1             |

---

## Spec Agents (Purple)

Requirements, design, and planning artifacts.

| Agent                    | Description                                                                                                              | Primary Flows |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------ | ------------- |
| `adr-author`             | Write run-local ADR binding design options to REQ/NFRs → `plan/adr.md`                                                   | 2             |
| `bdd-author`             | Turn requirements into BDD scenarios → `features/*.feature` + `example_matrix.md` + `verification_notes.md`              | 1             |
| `design-optioneer`       | Propose 2–3 distinct architecture options with structured trade-offs → `plan/design_options.md`                          | 2             |
| `interface-designer`     | Define API/event/RPC contracts + data model + planned migrations → `api_contracts.yaml`, `schema.md`, `migrations/*.sql` | 2             |
| `observability-designer` | Metrics, logs, traces, SLOs, alerts → `plan/observability_spec.md`                                                       | 2             |
| `pr-commenter`           | Post idempotent PR comment summarizing what changed and what's left                                                      | 4             |
| `pr-creator`             | Create Draft PR from run branch to main at end of Flow 3. Updates `run_meta.json` with `pr_number`                       | 3             |
| `test-strategist`        | Map Flow 1 BDD scenarios + risks to concrete test types and coverage thresholds → `plan/test_plan.md`                    | 2             |
| `work-planner`           | Break design into subtasks + sequencing + rollout/rollback → `work_plan.md`                                              | 2             |

---

## Implementation Agents (Green)

Writing code, tests, and documentation.

| Agent               | Description                                                                                                                               | Primary Flows |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `code-implementer`  | Implement changes to satisfy tests and REQ/NFR, aligned with ADR/contracts/observability → project code + `build/impl_changes_summary.md` | 3             |
| `context-loader`    | Select and list relevant code/tests/specs for a build subtask → `build/subtask_context_manifest.json`                                     | 3             |
| `doc-writer`        | Update documentation and docstrings to match implemented behavior + ADR/contracts → docs + `build/doc_updates.md`                         | 3             |
| `fixer`             | Apply targeted fixes from critics/mutation within subtask scope → `build/fix_summary.md`                                                  | 3, 4          |
| `gate-fixer`        | Report-only mechanical fix assessment (format/lint/imports/docs hygiene) → `gate/gate_fix_summary.md` + `FIX_FORWARD_PLAN_V1` block       | 5             |
| `pr-status-manager` | Manage PR state transitions (Draft to Ready, add labels, request reviewers)                                                               | 4             |
| `repo-operator`     | Git workflows (branch, stage, commit, push, merge, tag). Sole owner of git side effects                                                   | All           |

---

## Critic Agents (Red)

Harsh review agents that never fix code directly.

| Agent                  | Description                                                                                                             | Primary Flows |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------- |
| `bdd-critic`           | Harsh review of BDD scenarios vs requirements → `signal/bdd_critique.md`                                                | 1             |
| `code-critic`          | Harsh review of implementation vs REQ/NFR + ADR + contracts + observability → `build/code_critique.md`                  | 3             |
| `contract-critic`      | Validate Plan contracts/schema for completeness + testability → `plan/contract_critique.md`                             | 2             |
| `design-critic`        | Validate design vs constraints and upstream spec → `plan/design_validation.md`                                          | 2             |
| `fix-forward-runner`   | Execute the `FIX_FORWARD_PLAN_V1` block emitted by gate-fixer. Run apply/verify commands, write `fix_forward_report.md` | 5             |
| `gh-issue-resolver`    | Pre-run agent for Flow 1. Resolves or creates a GitHub issue before any run directory exists                            | 1             |
| `observability-critic` | Validate Plan observability_spec for required signals + verification readiness → `plan/observability_critique.md`       | 2             |
| `option-critic`        | Evaluate design options for decision-readiness before ADR authoring → `plan/option_critique.md`                         | 2             |
| `requirements-critic`  | Harsh review: requirements are testable, consistent, traceable → `signal/requirements_critique.md`                      | 1             |
| `secrets-sanitizer`    | Publish gate. Scans publish surface for secrets, fixes what it can, blocks publish when unsafe                          | All           |
| `test-critic`          | Harsh review of tests vs BDD + REQ/NFR + test plan → `build/test_critique.md`                                           | 3             |

---

## Verification Agents (Blue)

Audits, checks, compliance, and flow finalization.

| Agent               | Description                                                                                                     | Primary Flows                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------- | ---------------------------------- | --- |
| `artifact-auditor`  | Audit existence + obvious coherence of expected artifacts across Flows 1–5 → `artifact_audit.md`                | 5                                  |
| `build-cleanup`     | Finalizes Flow 3 by verifying artifacts, writing `build_receipt.json`, and updating `.runs/index.json`          | 3                                  |
| `contract-enforcer` | Best-effort verification that API implementation matches Plan contracts → `gate/contract_compliance.md`         | 5                                  |
| `coverage-enforcer` | Best-effort verification that test coverage meets Plan thresholds → `gate/coverage_audit.md`                    | 5                                  |
| `deploy-cleanup`    | Finalizes Flow 6 by writing `deploy_receipt.json` and updating `.runs/index.json`                               | 6                                  |
| `deploy-decider`    | Decide deploy readiness by verifying governance enforcement (CI + branch protection) → `deployment_decision.md` | 6                                  |
| `deploy-monitor`    | Read-only monitoring of CI + deployment signals → `deploy/verification_report.md`                               | 6                                  |
| `gate-cleanup`      | Finalizes Flow 5 by writing `gate_receipt.json` and updating `.runs/index.json`                                 | 5                                  |
| `merge-decider`     | Synthesize Gate evidence into a merge decision (MERGE                                                           | BOUNCE) → `gate/merge_decision.md` | 5   |
| `mutator`           | Run mutation testing on the subtask scope → `mutation_report.md`                                                | 3                                  |
| `plan-cleanup`      | Finalizes Flow 2 by writing `plan_receipt.json` + `cleanup_report.md`                                           | 2                                  |
| `receipt-checker`   | Verify Build receipt is parseable, contract-compliant, and internally consistent → `gate/receipt_audit.md`      | 5                                  |
| `smoke-verifier`    | Non-destructive release + health verification → appends to `verification_report.md`                             | 6                                  |

---

## Analytics Agents (Orange)

Analysis, learning, and operational insights.

| Agent                   | Description                                                                                                                     | Primary Flows |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `doc-critic`            | Critique documentation freshness and verification instructions after Build → `build/doc_critique.md`                            | 3             |
| `feedback-applier`      | Turn Wisdom learnings/regressions into issue drafts + doc/playbook suggestions → `wisdom/feedback_actions.md`                   | 7             |
| `flakiness-detector`    | Re-run failures with repetition budget and classify deterministic vs flaky vs environment/tooling → `build/flakiness_report.md` | 3             |
| `flow-historian`        | Compile timeline → `flow_history.json`                                                                                          | 7             |
| `fuzz-triager`          | Run configured fuzzing (opt-in) and triage crashes into repro/worklist → `build/fuzz_report.md`                                 | 3             |
| `impact-analyzer`       | Map blast radius of the change → `impact_map.json`                                                                              | 2, 3          |
| `learning-synthesizer`  | Extract actionable lessons from run artifacts → `wisdom/learnings.md`                                                           | 7             |
| `mutation-auditor`      | Run bounded mutation testing and produce actionable survivor worklist → `build/mutation_report.md`                              | 3             |
| `policy-analyst`        | Map policy requirements to evidence in the current change → `policy_analysis.md`                                                | 2, 5          |
| `pr-feedback-harvester` | Read all PR feedback sources (CodeRabbit, GitHub Actions, Dependabot, review comments) → `pr_feedback.md`                       | 3, 4          |
| `regression-analyst`    | Analyze regressions (tests, coverage, stability) with blame + issue correlation → `wisdom/regression_report.md`                 | 7             |

---

## Infra Agents (Cyan)

Repo and run infrastructure management.

| Agent            | Description                                                                                                      | Primary Flows |
| ---------------- | ---------------------------------------------------------------------------------------------------------------- | ------------- |
| `run-prep`       | Establish or reattach run infrastructure for Flows 2-7. Creates `.runs/<run-id>/<flow>/`                         | 2-7           |
| `scope-assessor` | Stakeholders, early risks, and T-shirt scope estimate → `stakeholders.md`, `early_risks.md`, `scope_estimate.md` | 1             |

---

## Reporter Agents (Pink)

GitHub posting and external communication.

| Agent         | Description                                                             | Primary Flows |
| ------------- | ----------------------------------------------------------------------- | ------------- |
| `gh-reporter` | Post one idempotent flow summary comment to the GitHub issue (never PR) | All           |

---

## Additional Agents

These agents don't fit cleanly into one category or are cross-cutting:

| Agent                    | Color  | Description                                                                                                        | Primary Flows |
| ------------------------ | ------ | ------------------------------------------------------------------------------------------------------------------ | ------------- |
| `requirements-author`    | Purple | Write functional + non-functional requirements from problem statement → `requirements.md`                          | 1             |
| `review-cleanup`         | Blue   | Finalizes Flow 4 by writing `review_receipt.json` and updating `.runs/index.json`                                  | 4             |
| `review-worklist-writer` | Orange | Convert raw PR feedback into actionable worklist with stable markers → `review/review_worklist.md`                 | 4             |
| `risk-analyst`           | Orange | Identify and track risk patterns (security, compliance, data, performance, ops) → `risk_assessment.md`             | 1, 2, 5       |
| `security-scanner`       | Blue   | Best-effort security review of changed surface (SAST patterns + dependency risk signals) → `gate/security_scan.md` | 5             |
| `self-reviewer`          | Blue   | Final review of Flow 3 build artifacts → `self_review.md`                                                          | 3             |
| `signal-cleanup`         | Blue   | Finalizes Flow 1 by writing `signal_receipt.json` and updating `.runs/index.json`                                  | 1             |
| `standards-enforcer`     | Blue   | Enforce pack standards (no silent test deletions, marker compliance)                                               | 3, 5          |
| `test-author`            | Green  | Write/update tests from BDD scenarios + test plan → project tests + `build/test_changes_summary.md`                | 3             |
| `test-executor`          | Blue   | Execute the configured test suite → `build/test_execution.md`                                                      | 3             |
| `traceability-auditor`   | Blue   | Read-only coherence + spec traceability audit → `traceability_audit.md`                                            | 5             |
| `wisdom-cleanup`         | Blue   | Finalizes Flow 7 by writing `wisdom_receipt.json` and updating `.runs/index.json`                                  | 7             |

---

## Agent Model Selection

Agents specify a `model` field in their YAML frontmatter. See [Model Allocation](model-allocation.md) for the full strategy.

| Value     | Use Case                                           |
| --------- | -------------------------------------------------- |
| `haiku`   | Research, cleanup, mechanical work                 |
| `sonnet`  | Almost-Haiku tasks needing slightly more reasoning |
| `inherit` | Core creative work (user chooses Sonnet or Opus)   |

---

## Flow-to-Agent Mapping

### Flow 1: Signal

`gh-issue-resolver` → `signal-run-prep` → `signal-normalizer` → `problem-framer` → `scope-assessor` → `requirements-author` ↔ `requirements-critic` → `bdd-author` ↔ `bdd-critic` → `risk-analyst` → `signal-cleanup`

### Flow 2: Plan

`run-prep` → `design-optioneer` ↔ `option-critic` → `adr-author` → `interface-designer` ↔ `contract-critic` → `observability-designer` ↔ `observability-critic` → `test-strategist` → `work-planner` → `design-critic` → `policy-analyst` → `plan-cleanup`

### Flow 3: Build

`run-prep` → `context-loader` → `test-author` ↔ `test-critic` → `code-implementer` ↔ `code-critic` → `test-executor` → `doc-writer` ↔ `doc-critic` → `self-reviewer` → `build-cleanup` → `pr-creator` → `pr-feedback-harvester`

### Flow 4: Review

`run-prep` → `pr-feedback-harvester` → `review-worklist-writer` → `fixer` (loop) → `review-cleanup` → `pr-status-manager`

### Flow 5: Gate

`run-prep` → `receipt-checker` → `contract-enforcer` → `security-scanner` → `coverage-enforcer` → `traceability-auditor` → `merge-decider` → (`gate-fixer` → `fix-forward-runner`) → `gate-cleanup`

### Flow 6: Deploy

`run-prep` → `deploy-decider` → `deploy-monitor` → `smoke-verifier` → `deploy-cleanup`

### Flow 7: Wisdom

`run-prep` → `flow-historian` → `regression-analyst` → `learning-synthesizer` → `feedback-applier` → `wisdom-cleanup`

### Cross-Flow Agents

These agents run in multiple or all flows:

- `repo-operator` - Git operations (all flows)
- `secrets-sanitizer` - Publish gate (all flows)
- `gh-issue-manager` - Issue sync (all flows)
- `gh-reporter` - Flow summaries (all flows)
- `clarifier` - Open questions (any flow)

---

## See Also

- [glossary.md](glossary.md) — Term definitions
- [contracts.md](contracts.md) — Control-plane blocks and schemas
- [repo-map.md](repo-map.md) — Repository structure
- [CLAUDE.md](../../CLAUDE.md) — Pack reference and invariants
