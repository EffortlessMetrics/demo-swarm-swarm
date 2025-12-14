# Flow 3 Parallel Plan - align-doc-ownership

Run strategy: **one Flow 3 thread per flow** (ST-001 -> ST-006) under run-id `align-doc-ownership`. Each thread owns its scope, touches only the listed files, and seals receipts independently.

## Shared rules

- Working lane: only the scoped files per subtask; no cross-subtask edits.
- Preflight: rely on run-prep; do not probe `.runs/` before it exists in the workspace.
- Acceptance for every thread: `pack-check` green for touched scope, `doc-drift` green, `build_receipt.json` + `cleanup_report.md` updated, lint/test executor reports present, and routing signals captured in Machine Summaries.
- GH ops gating: if `safe_to_publish: true` and `proceed_to_github_ops: true` with `gh` authenticated, GH ops are required; otherwise mark UNVERIFIED and route to `gh-issue-manager` or `repo-operator`.

## Subtasks

### ST-001 - Flow 1 docs + agents

- **Scope:** `.claude/commands/flow-1-signal.md`; Flow 1 agents (requirements, BDD, framing/scope/risk, signal-cleanup).
- **Touches:** remove skill-name mentions and path plumbing from the flow doc; ensure agents use canonical enums, include `## Skills` when invoking shims, and state exact file counts without contradictions; point to skill docs for flags.
- **Acceptance:** pack-check + doc-drift green; flow doc lists only agents/stations; GH ops required when gates open.

### ST-002 - Flow 2 docs + agents

- **Scope:** `.claude/commands/flow-2-plan.md`; Plan agents (impact/design/ADR/interface/test/work/policy/cleanup).
- **Touches:** same flow-boundary enforcement; ensure `work-planner` agent doc owns the "writes exactly two files" contract; flow doc Step 9 stays filename-agnostic.
- **Acceptance:** pack-check + doc-drift green; flow doc free of skill/flag plumbing; agent enums canonical.

### ST-003 - Flow 3 docs + agents

- **Scope:** `.claude/commands/flow-3-build.md`; Build agents (context-loaders, critics, fixers, lint-executor, test-executor, build-cleanup).
- **Touches:** keep lint-executor and test-executor as explicit stations; gate-fixer routes fmt/lint/test reruns to these agents (no shell commands); build-cleanup expects executor artifacts as inputs (no hardcoded layouts beyond `.runs/<run-id>/build/`).
- **Acceptance:** pack-check + doc-drift green; lint/test executor reports present; build-cleanup required-artifact checks reflect executors.

### ST-004 - Flow 4 docs + gate agents + CLAUDE.md

- **Scope:** `.claude/commands/flow-4-gate.md`; gate agents (receipt-checker, contract/security/coverage, gate-fixer, merge-decider, cleanup); boundary checks; `CLAUDE.md` TOC-level scope.
- **Touches:** receipt-checker runs first post-run-prep and routes on its Result before contracts/security/coverage; receipt-checker supports committed-state fallback without treating snapshot mismatch as automatic bounce; pack-check output is actionable (file:line + snippet) for new boundaries; keep CLAUDE.md summary-level with links (no flag tables).
- **Acceptance:** pack-check + doc-drift green; receipt-checker routing enforced; boundary checks emit actionable matches; CLAUDE.md remains scope-only.

### ST-005 - Flow 5 docs + deploy agents

- **Scope:** `.claude/commands/flow-5-deploy.md`; deploy agents (deploy-monitor, smoke-verifier, deploy-decider, deploy-cleanup, GH agents).
- **Touches:** clearly separate release ops (merge/tag) from reporting ops; enforce two-gate prerequisite for reporting; ensure stable `github_repo` identity from `run_meta` is used consistently.
- **Acceptance:** pack-check + doc-drift green; deploy doc reflects two-gate/identity requirements; GH ops use `github_repo`.

### ST-006 - Flow 6 docs + validation

- **Scope:** `.claude/commands/flow-6-wisdom.md`; wisdom agents (artifact-auditor, regression-analyst, learning-synthesizer, feedback-applier, cleanup); `docs/maintainers/validation-log.md` after Toy Run A/B.
- **Touches:** ensure stable markers for wisdom counting; align agent docs with canonical enums and skills; run Toy Run A/B and record outcomes in validation log.
- **Acceptance:** pack-check + doc-drift green; validation log updated with Toy Run A/B; wisdom receipt reflects marker counts.

## Checklist (grep targets by subtask)

- ST-001/002/003/004/005/006: flow commands should not contain skill names or CLI flag strings; target regex: `rg -n "runs-[a-z-]+" .claude/commands/flow-*`.
- ST-001/002/003/004/005/006: flow commands should avoid file-path plumbing in steps; target regex: `rg -n "\\.runs/.+" .claude/commands/flow-*` (allow in context/outputs tables only).
- ST-004: boundary checks must report file and line; target regex for outputs: `pack-check` violations include `:<line>` in message.
- ST-003: lint/test executor artifacts required; target presence: `.runs/<run-id>/build/lint_report.md` and `test_execution.md` in `build-cleanup` inputs.
- ST-006: wisdom markers present; target regex: `^## Machine Summary` blocks with canonical enums and marker prefixes in wisdom artifacts.
