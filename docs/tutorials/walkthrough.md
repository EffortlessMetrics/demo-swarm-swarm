# Walkthrough

A walkthrough for demonstrating the swarm pack in action.

---

## Prerequisites

1. A target repo with `.claude/` copied from this pack
2. Claude Code installed and configured
3. (Optional) `gh` CLI authenticated for GitHub integration

---

## Canonical Demo Goal

All demos use the same goal:

> Add a `demoswarm version` CLI subcommand that prints JSON with tool version info.

This goal is **self-referential**: you're building a feature *for the pack's own CLI tooling*. It's deterministic, offline-friendly, and exercises the pack's core primitives (receipts, mechanical derivation, gating) without inventing a separate "product."

---

## 1. Setup

### Copy the pack to a sandbox repo

```bash
# Create or use a sandbox repo
cd my-project-sandbox/

# Copy the pack
cp -r /path/to/demo-swarm/.claude .
```

### Open in Claude Code

Open the sandbox repo in Claude Code. It will discover:
- `.claude/commands/` — slash commands
- `.claude/agents/` — subagents
- `.claude/skills/` — skills

---

## 2. Run Flow 1: Signal

**You say:** "Let's start a new feature. Flow 1 captures the intent and produces requirements."

**You run:**
```
/flow-1-signal "Add a demoswarm version CLI subcommand that prints JSON with tool version info. Constraints: Must work via bash .claude/scripts/demoswarm.sh version. Output is JSON to stdout. No network calls. Include demoswarm_version, pack_version (if available), git_sha (optional/null-safe)."
```

**What happens:**
1. `signal-run-prep` establishes the run directory
2. `signal-normalizer` parses the input
3. `requirements-author` writes requirements
4. `bdd-author` creates BDD scenarios
5. `signal-cleanup` computes the receipt
6. `gh-issue-manager` creates/updates the GitHub issue
7. `gh-reporter` posts a summary

**You show:**
- `.runs/<run-id>/signal/` directory with artifacts
- `signal_receipt.json` with status and counts
- GitHub issue (if `gh` is authenticated)

---

## 3. Run Flow 2: Plan

**You say:** "Now we design the solution. Flow 2 produces architecture and contracts."

**You run:**
```
/flow-2-plan
```

**What happens:**
1. `run-prep` locks onto the existing run
2. `design-optioneer` proposes options
3. `adr-author` writes the architecture decision
4. `interface-designer` defines contracts
5. `plan-cleanup` computes the receipt

**You show:**
- `.runs/<run-id>/plan/adr.md`
- `.runs/<run-id>/plan/api_contracts.yaml`
- `plan_receipt.json`

---

## 4. Run Flow 3: Build

**You say:** "Flow 3 implements via test/code microloops until the critic is satisfied."

**You run:**
```
/flow-3-build
```

**What happens:**
1. `test-author` writes tests
2. `test-critic` reviews and returns `recommended_action`
3. Route on `recommended_action` (RERUN/BOUNCE/PROCEED; `FIX_ENV` only with `status: CANNOT_PROCEED`). PROCEED is the default even with open questions; use `can_further_iteration_help` only as a tie-breaker when no action is set.
4. `code-implementer` writes code
5. `code-critic` reviews (same routing rules)
6. `self-reviewer` produces final review
7. `build-cleanup` computes the receipt
8. `repo-operator` commits changes

**You show:**
- Test and code files
- `build_receipt.json` with microloop counts
- Git commit history

**Feature verification:**
```bash
# The implemented feature should now work
bash .claude/scripts/demoswarm.sh version
# Should output JSON with demoswarm_version, pack_version, git_sha
```

---

## 5. Run Flow 4: Review

**You say:** "Flow 4 drains all PR feedback—CodeRabbit, CI, human reviews—until the worklist is empty."

**You run:**
```
/flow-4-review
```

**What happens:**
1. `pr-feedback-harvester` collects all feedback (full severity range)
2. `review-worklist-writer` creates actionable worklist
3. Worklist loop: fix items, push, re-harvest (may run multiple times if `PARTIAL`)
4. `review-cleanup` computes the receipt

**You show:**
- `.runs/<run-id>/review/review_worklist.md`
- `review_receipt.json` with status (VERIFIED/PARTIAL)

---

## 6. Run Flow 5: Gate

**You say:** "Flow 5 verifies the build and recommends merge or bounce."

**You run:**
```
/flow-5-gate
```

**What happens:**
1. `receipt-checker` validates build receipt
2. `coverage-enforcer` checks coverage
3. `security-scanner` runs SAST
4. `merge-decider` issues decision (MERGE/BOUNCE with reason)
5. `gate-cleanup` computes the receipt

**You show:**
- `merge_decision.md` with the verdict
- `gate_receipt.json`

---

## 7. Run Flow 6: Deploy

**You say:** "Flow 6 merges to mainline and verifies."

**You run:**
```
/flow-6-deploy
```

**What happens:**
1. Verifies gate passed
2. `deploy-decider` confirms deployment readiness
3. `deploy-monitor` watches CI/deployment
4. `smoke-verifier` runs health checks
5. `deploy-cleanup` computes the receipt

---

## 8. Run Flow 7: Wisdom

**You say:** "Flow 7 extracts learnings and closes feedback loops."

**You run:**
```
/flow-7-wisdom
```

**What happens:**
1. `regression-analyst` analyzes for regressions
2. `learning-synthesizer` extracts lessons
3. `feedback-applier` suggests improvements
4. `wisdom-cleanup` computes the final receipt

**You show:**
- `learnings.md`
- `wisdom_receipt.json`
- Complete run history in `.runs/<run-id>/`

---

## Artifacts Overview

After a complete run:

```
.runs/<run-id>/
├── run_meta.json
├── signal/
│   ├── requirements.md
│   ├── features/*.feature
│   └── signal_receipt.json
├── plan/
│   ├── adr.md
│   ├── api_contracts.yaml
│   └── plan_receipt.json
├── build/
│   ├── test_summary.md
│   ├── self_review.md
│   └── build_receipt.json
├── review/
│   ├── review_worklist.md
│   └── review_receipt.json
├── gate/
│   ├── merge_decision.md
│   └── gate_receipt.json
├── deploy/
│   └── deploy_receipt.json
└── wisdom/
    ├── learnings.md
    └── wisdom_receipt.json
```

---

## Demo Proof Points

After the full walkthrough:

1. **Receipts exist** for each flow (mechanical, not estimated)
2. **The feature works**: `bash .claude/scripts/demoswarm.sh version` outputs valid JSON
3. **Artifacts are inspectable**: `.runs/<run-id>/` contains the audit trail
4. **Index is updated**: `.runs/index.json` tracks the run status

This demonstrates the core value: **"prompt → structured artifacts → receipt"**.

---

## Troubleshooting

See [Quickstart troubleshooting](quickstart.md#troubleshooting) for common issues.

**Critic keeps bouncing?** Route on `recommended_action`:
- `BOUNCE` → follow `route_to_flow/route_to_agent`
- `RERUN` → loop once more
- No action set → use `can_further_iteration_help` as tie-breaker

---

## Next Steps

| Goal | Doc |
|------|-----|
| Customize for your stack | [customize-pack.md](../how-to/customize-pack.md) |
| Full reference | [CLAUDE.md](../../CLAUDE.md) |
| Validate pack contracts | [validation-run.md](validation-run.md) |
