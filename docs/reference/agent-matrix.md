# Agent Capability Matrix

Quick reference for orchestrators and agents. Shows what each agent does, what it produces, and who it typically hands off to.

---

## Signal Agents (Flow 1)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| signal-run-prep | Establish run infrastructure, create directories, write run_meta.json | `.runs/<run-id>/signal/`, run_meta.json, index.json entry | signal-normalizer |
| signal-normalizer | Normalize raw input into structured facts | issue_normalized.md, context_brief.md | problem-framer |
| gh-researcher | Read-only GitHub reconnaissance (issues/PRs/discussions) | github_research.md | problem-framer, requirements-author |
| problem-framer | Synthesize normalized signal into problem statement | problem_statement.md | requirements-author, clarifier |
| requirements-author | Write functional and non-functional requirements | requirements.md | requirements-critic, bdd-author |
| requirements-critic | Review requirements for testability and consistency | requirements_critique.md | requirements-author (if issues), bdd-author |
| bdd-author | Convert requirements into BDD scenarios | features/*.feature, example_matrix.md, verification_notes.md | bdd-critic, clarifier |
| bdd-critic | Review BDD scenarios for traceability and coverage | bdd_critique.md | bdd-author (if issues), scope-assessor |
| scope-assessor | Assess stakeholders, early risks, T-shirt scope | stakeholders.md, early_risks.md, scope_estimate.md | spec-auditor, signal-cleanup |
| spec-auditor | Holistic audit of Flow 1 spec coherence | spec_audit.md | signal-cleanup, (bounce to authors if issues) |
| signal-cleanup | Summarize Flow 1, write receipt, update index | signal_receipt.json, cleanup_report.md, github_report.md | secrets-sanitizer, Flow 2 |

---

## Plan Agents (Flow 2)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| design-optioneer | Generate design options from requirements | design_options.md | option-critic, adr-author |
| option-critic | Critique design options for feasibility | option_critique.md | design-optioneer (if issues), adr-author |
| adr-author | Write architecture decision records | adr.md | design-critic, interface-designer |
| design-critic | Review ADR for completeness and coherence | design_critique.md | adr-author (if issues), interface-designer |
| interface-designer | Define API contracts and schemas | api_contracts.yaml, schema.md, interface_spec.md | contract-critic, observability-designer |
| contract-critic | Review API contracts for consistency | contract_critique.md | interface-designer (if issues), test-strategist |
| observability-designer | Define metrics, logs, traces, alerts | observability_spec.md | observability-critic, work-planner |
| observability-critic | Review observability spec | observability_critique.md | observability-designer (if issues), work-planner |
| test-strategist | Define test plan and coverage thresholds | test_plan.md, ac_matrix.md | work-planner |
| work-planner | Decompose work into ordered subtasks | work_plan.md | plan-cleanup |
| plan-cleanup | Summarize Flow 2, write receipt, update index | plan_receipt.json, cleanup_report.md | secrets-sanitizer, Flow 3 |

---

## Build Agents (Flow 3)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| run-prep | Establish build run infrastructure | build directories, run_meta updates | context-loader |
| context-loader | Load and summarize context for implementation | subtask_context_manifest.json | code-implementer |
| code-implementer | Write production code per work plan | implementation code, impl_changes_summary.md | code-critic, test-author |
| code-critic | Critique implementation for quality issues | code_critique.md | fixer (if issues), test-author |
| test-author | Write tests for implementation | test code, test_changes_summary.md | test-critic, test-executor |
| test-critic | Review test quality and coverage | test_critique.md | fixer (if issues), test-executor |
| test-executor | Run tests and capture results | test_execution.md | fixer (if failures), self-reviewer |
| fixer | Apply fixes from critic feedback | fixed code, fix_summary.md | code-critic, test-executor |
| self-reviewer | Self-review changes before cleanup | self_review.md | build-cleanup |
| doc-writer | Write inline documentation | updated docs | doc-critic |
| doc-critic | Critique documentation quality | doc_critique.md | doc-writer (if issues), build-cleanup |
| standards-enforcer | Enforce coding standards and conventions | standards_report.md | fixer, build-cleanup |
| build-cleanup | Summarize Flow 3, write receipt, update index | build_receipt.json, cleanup_report.md | secrets-sanitizer, pr-creator |

---

## Review Agents (Flow 4)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| pr-creator | Create draft PR with structured description | Draft PR, pr_creation.md | pr-feedback-harvester |
| pr-feedback-harvester | Collect feedback from CI/bots/humans | pr_feedback.md | review-worklist-writer |
| review-worklist-writer | Transform feedback into prioritized worklist | review_worklist.md, review_worklist.json | feedback-applier |
| feedback-applier | Apply actionable feedback items | updated code, feedback_applied.md | test-executor, pr-feedback-harvester |
| pr-commenter | Post comments to PR | PR comments | pr-status-manager |
| pr-status-manager | Update PR status (draft/ready) | PR status updates | review-cleanup |
| review-cleanup | Summarize Flow 4, write receipt | review_receipt.json, cleanup_report.md | secrets-sanitizer, Flow 5 |

---

## Gate Agents (Flow 5)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| receipt-checker | Verify build receipt is valid and complete | receipt_audit.md | gate-fixer, merge-decider |
| contract-enforcer | Verify API implementation matches contracts | contract_compliance.md | gate-fixer, merge-decider |
| coverage-enforcer | Verify test coverage meets thresholds | coverage_audit.md | gate-fixer, merge-decider |
| security-scanner | Best-effort security review of changed surface | security_scan.md | gate-fixer, merge-decider |
| traceability-auditor | Audit run identity, receipts, and spec traceability | traceability_audit.md | merge-decider |
| gate-fixer | Plan and coordinate gate fix-forward lane | gate_fix_summary.md | fix-forward-runner |
| fix-forward-runner | Execute fix-forward plan (format/lint fixes) | fix_forward_report.md | build-cleanup, repo-operator |
| merge-decider | Make final merge/bounce decision | merge_decision.md | repo-operator, gate-cleanup |
| gate-cleanup | Summarize Flow 5, write receipt | gate_receipt.json, cleanup_report.md | secrets-sanitizer, Flow 6 |

---

## Deploy Agents (Flow 6)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| deploy-decider | Evaluate readiness for deployment | deployment_decision.md | deploy-monitor, repo-operator |
| deploy-monitor | Monitor CI/CD pipeline and deployment status | deployment_log.md | smoke-verifier |
| smoke-verifier | Run smoke tests against deployed changes | verification_report.md | deploy-cleanup |
| deploy-cleanup | Summarize Flow 6, write receipt | deploy_receipt.json, cleanup_report.md | secrets-sanitizer, Flow 7 |

---

## Wisdom Agents (Flow 7)

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| artifact-auditor | Audit existence and coherence of artifacts | artifact_audit.md | learning-synthesizer |
| learning-synthesizer | Extract learnings and patterns from run | learnings.md | pattern-analyst, wisdom-cleanup |
| pattern-analyst | Identify recurring patterns and anti-patterns | pattern_report.md | process-analyst |
| process-analyst | Analyze process efficiency and bottlenecks | process_report.md | quality-analyst |
| quality-analyst | Analyze quality signals across the run | quality_report.md | signal-quality-analyst |
| signal-quality-analyst | Assess accuracy of feedback sources | signal_quality_report.md | solution-analyst |
| solution-analyst | Verify implementation solves the stated problem | solution_analysis.md | regression-analyst |
| regression-analyst | Analyze regressions with blame and issue correlation | regression_report.md | maintainability-analyst |
| maintainability-analyst | Assess code maintainability trends | maintainability_report.md | flow-historian |
| flow-historian | Document the run timeline and decisions | flow_history.md | wisdom-cleanup |
| wisdom-cleanup | Summarize Flow 7, write receipt, close loop | wisdom_receipt.json, latest.md | gh-reporter |

---

## Cross-Flow Agents

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| clarifier | Capture open questions, research answers, default safely | open_questions.md | (back to caller) |
| fixer | Apply fixes from any critic feedback | fixed code, fix_summary.md | (critic that raised issue), test-executor |
| repo-operator | Execute git operations (commit, push, branch) | commits, branches, tags | pr-creator, gh-reporter |
| secrets-sanitizer | Scan for secrets before publish | secrets_scan.md | repo-operator (if clean), (blocks if not) |
| gh-issue-manager | Manage GitHub issue status and labels | issue updates | gh-reporter |
| gh-issue-resolver | Link issues to commits/PRs | issue comments | gh-reporter |
| gh-reporter | Post flow reports to GitHub issues | issue/PR comments | (next flow) |
| risk-analyst | Identify and track risks across flows | risk_assessment.md | (flow cleanup) |
| policy-analyst | Analyze policy compliance | policy_report.md | gate-fixer, merge-decider |
| impact-analyzer | Analyze impact of changes | impact_analysis.md | work-planner, risk-analyst |

---

## Specialized Analysis Agents

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| flakiness-detector | Identify flaky tests and instability | flakiness_report.md | test-author, fixer |
| fuzz-triager | Triage fuzz testing findings | fuzz_triage.md | fixer, code-implementer |
| mutation-auditor | Audit mutation testing results | mutation_audit.md | test-author |

---

## Infrastructure Agents

| Agent | Does | Produces | Hands Off To |
|-------|------|----------|--------------|
| pack-customizer | Interactively customize DemoSwarm for stack | pack configuration | (user) |

---

## Handoff Patterns

Common routing patterns used across the swarm.

### Author -> Critic -> Fixer Loop

```
requirements-author -> requirements-critic -> (back to author if issues)
code-implementer -> code-critic -> fixer -> (back to critic for verification)
test-author -> test-critic -> fixer -> (back to critic)
bdd-author -> bdd-critic -> (back to author if issues)
```

### Work -> Cleanup Pattern

```
[any build agent] -> self-reviewer -> build-cleanup
[any flow agents] -> *-cleanup agent for that flow
```

### Publish Boundary

```
[any cleanup agent] -> secrets-sanitizer -> repo-operator -> gh-* agents
```

### Gate Verification Chain

```
receipt-checker -> contract-enforcer -> coverage-enforcer -> security-scanner -> merge-decider
```

---

## Agent Quick Reference by Task

"I need to..." lookup table.

| Task | Agent |
|------|-------|
| Write requirements | requirements-author |
| Critique requirements | requirements-critic |
| Write BDD scenarios | bdd-author |
| Critique BDD scenarios | bdd-critic |
| Frame the problem | problem-framer |
| Assess scope and risks | scope-assessor |
| Write code | code-implementer |
| Fix code issues | fixer |
| Critique code | code-critic |
| Write tests | test-author |
| Critique tests | test-critic |
| Run tests | test-executor |
| Write API contracts | interface-designer |
| Critique API contracts | contract-critic |
| Write ADR | adr-author |
| Critique ADR | design-critic |
| Plan work decomposition | work-planner |
| Commit changes | repo-operator |
| Create PR | pr-creator |
| Harvest PR feedback | pr-feedback-harvester |
| Apply PR feedback | feedback-applier |
| Verify build receipt | receipt-checker |
| Check API compliance | contract-enforcer |
| Check coverage thresholds | coverage-enforcer |
| Scan for security issues | security-scanner |
| Scan for secrets | secrets-sanitizer |
| Decide merge | merge-decider |
| Monitor deployment | deploy-monitor |
| Run smoke tests | smoke-verifier |
| Extract learnings | learning-synthesizer |
| Analyze regressions | regression-analyst |
| Post to GitHub | gh-reporter |
| Clarify ambiguities | clarifier |
| Assess risk | risk-analyst |

---

## Notes

- This matrix should be kept in sync with `.claude/agents/`
- "Hands Off To" shows typical targets, not all possible targets
- Agents can always hand back to their caller
- Cleanup agents always exist at flow boundaries
- Cross-flow agents (clarifier, fixer, repo-operator, secrets-sanitizer) work across multiple flows
- Status model is consistent: `VERIFIED | UNVERIFIED | CANNOT_PROCEED`
- Action vocabulary is closed: `PROCEED | RERUN | BOUNCE | FIX_ENV`
