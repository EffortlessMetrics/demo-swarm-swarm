# Combined Markdown

## customize-pack.md

---
description: Interactively customize DemoSwarm for your stack
---

## Customize DemoSwarm Pack

You are guiding the user through customizing the DemoSwarm pack for their specific repository and stack.

### Purpose

This command helps newcomers adapt the pack to their codebase by:
1. Detecting existing patterns in the repository
2. Asking targeted questions about stack and preferences
3. Updating skill files and agent prompts accordingly
4. Writing a receipt documenting all changes

### Customization Workflow

#### Step 1: Detect Current Environment

Before asking questions, use explore agents to scan the repository to infer:

```bash
# Detect language/runtime (use test -f for reliability)
test -f package.json && echo "Node.js detected"
test -f Cargo.toml && echo "Rust detected"
test -f pyproject.toml && echo "Python (pyproject) detected"
test -f setup.py && echo "Python (setup.py) detected"
test -f requirements.txt && echo "Python (requirements) detected"
test -f go.mod && echo "Go detected"

# Detect test framework
test -f package.json && grep -q '"jest"\|"vitest"\|"mocha"' package.json && echo "JS test framework detected"
test -f pyproject.toml && grep -q 'pytest' pyproject.toml && echo "pytest detected"
test -f Cargo.toml && echo "Rust tests (cargo test) detected"

# Detect lint tools
test -f .eslintrc.js -o -f .eslintrc.json -o -f .eslintrc.yml && echo "ESLint detected"
test -f .prettierrc -o -f .prettierrc.json -o -f .prettierrc.js && echo "Prettier detected"
test -f ruff.toml && echo "Ruff detected"
test -f pyproject.toml && grep -Fq '[tool.ruff]' pyproject.toml && echo "Ruff (in pyproject) detected"
test -f rustfmt.toml && echo "rustfmt detected"

# Detect source layout (use test -d for directories)
test -d src && echo "src/ found"
test -d lib && echo "lib/ found"
test -d app && echo "app/ found"
test -d tests && echo "tests/ found"
test -d test && echo "test/ found"
test -d __tests__ && echo "__tests__/ found"
test -d spec && echo "spec/ found"
```

#### Step 2: Ask Targeted Questions

Based on detection, ask for confirmation or clarification:

**Questions to ask (adapt based on detection):**

1. **Language/Runtime**: "I detected [X]. Is this the primary language for this project?"
2. **Test Command**: "What command runs your tests?" (suggest based on detection)
3. **Lint/Format Command**: "What commands format and lint your code?"
4. **Source Layout**: "I see your source code is in [X] and tests in [Y]. Is this correct?"
5. **Git Provider**: "Are you using GitHub, GitLab, Bitbucket, or another provider?"
6. **Windows Environment**: "Are you using WSL2, Git Bash, or native PowerShell?"
7. **Mutation Harness (optional)**: "Do you have mutation testing configured? If so, what command should we run?"
8. **Fuzz Harness (optional)**: "Do you have fuzzing configured? If so, what command should we run?"

Use the AskUserQuestion tool to gather this information.

#### Step 3: Update Skills

Based on answers, update the following files:

**`.claude/skills/test-runner/SKILL.md`**:
- Replace the test command with the user's command
- Update any language-specific patterns

**`.claude/skills/auto-linter/SKILL.md`**:
- Replace format and lint commands
- Update file patterns if needed

**`.claude/skills/policy-runner/SKILL.md`**:
- Update policy check commands if the user has them

#### Step 4: Update Agent Prompts (If Needed)

If source layout differs from default (`src/`, `tests/`, `features/`), update:
- `code-implementer.md` - Where to write code
- `test-author.md` - Where to write tests
- `bdd-author.md` - Where to write features

If Git provider is not GitHub, update:
- `gh-issue-manager.md`
- `gh-reporter.md`
- `gh-researcher.md`
- `repo-operator.md`
- `deploy-monitor.md`

#### Step 5: Write Configuration Receipt

Create `demo-swarm.config.json` in repo root:

```json
{
  "version": 1,
  "customized_at": "<ISO8601>",
  "stack": {
    "language": "rust | node | python | go | other",
    "runtime": "<specific runtime if relevant>",
    "package_manager": "cargo | npm | pnpm | yarn | pip | poetry | go"
  },
  "commands": {
    "test": "<test command>",
    "lint": "<lint command>",
    "format": "<format command>"
  },
  "mutation": {
    "command": "<mutation command or null>",
    "budget_seconds": 300,
    "survivor_threshold": 0
  },
  "fuzz": {
    "command": "<fuzz command or null>",
    "budget_seconds": 300
  },
  "flakiness": {
    "command": "<flakiness rerun command or null>",
    "rerun_count": 3,
    "budget_seconds": 180
  },
  "layout": {
    "source": "src/",
    "tests": "tests/",
    "features": "features/",
    "docs": "docs/"
  },
  "environment": {
    "platform": "linux | macos | windows-wsl2 | windows-gitbash",
    "git_provider": "github | gitlab | bitbucket | azure-devops"
  },
  "files_modified": [
    ".claude/skills/test-runner/SKILL.md",
    ".claude/skills/auto-linter/SKILL.md"
  ]
}
```

#### Step 6: Write Customization Receipt

Create `docs/CUSTOMIZATION_RECEIPT.md`:

```markdown
# DemoSwarm Customization Receipt

## Customized: <ISO8601 timestamp>

## Detected Stack

- **Language**: <detected>
- **Test Framework**: <detected>
- **Lint Tools**: <detected>

## User Choices

- **Test Command**: `<command>`
- **Lint Command**: `<command>`
- **Source Layout**: <layout>
- **Platform**: <platform>

## Files Modified

| File | Change |
|------|--------|
| `.claude/skills/test-runner/SKILL.md` | Updated test command to `<cmd>` |
| `.claude/skills/auto-linter/SKILL.md` | Updated lint commands |

## Next Steps

1. Review the changes in the modified files
2. Run `/flow-1-signal "test feature"` to validate the setup
3. If issues arise, manually adjust skill files per `docs/CUSTOMIZATION.md`
```

### Completion

Report to the user:
1. Summary of changes made
2. List of modified files
3. Suggested next step: run a test flow

### Important Notes

- Always preserve existing file structure when editing
- Make minimal changes—only what's needed for the stack
- Document every change in the receipt
- If unsure about a setting, ask rather than guess
- Default to GitHub if git provider is unclear

---

## flow-1-signal.md

---
description: Run Flow 1 (Signal -> Spec): shape the problem, identify stakeholders, flag early risks, estimate scope.
argument-hint: "[optional-run-id] <feature request or signal>"
---

## Flow 1: Signal -> Spec

You are orchestrating Flow 1 of the SDLC swarm. This flow transforms messy input into testable requirements, BDD features, early risks, and a GitHub-ready summary.

### Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/signal/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Run identity now comes from `gh-issue-resolver` (Step 0). The `repo-operator` ensures the run branch (Step 0b) and `signal-run-prep` (Step 0c) establishes the run directory using that run-id. Do not skip these steps.
- `run_id_kind: LOCAL_ONLY` means the run-id is a local slug (`local-...`) and the issue is not bound yet (`issue_number: null`).
  - If `github_ops_allowed: false` → repo mismatch / trust block (never bind/create issues in this repo).
  - If `github_ops_allowed: true` + `issue_number: null` → GitHub binding is deferred (bind later when GitHub works; handled by `gh-issue-manager`).
- Only repo mismatch sets `github_ops_allowed: false`. If GitHub is temporarily unavailable/unauthenticated, `github_ops_allowed` remains `true` and binding is deferred (`issue_binding: DEFERRED`; later handled by `gh-issue-manager` when access allows).

##### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

### Your Goals

- Turn messy input into testable requirements
- Identify affected stakeholders (teams, systems, users)
- Flag early security/compliance/performance risks
- Estimate scope (S/M/L/XL t-shirt size)
- Produce BDD scenarios
- Post summary to GitHub issue

### Before You Begin (Required)

#### Two State Machines

Flow 1 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

#### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Microloops (`requirements-author` ↔ `requirements-critic`, `bdd-author` ↔ `bdd-critic`) are ONE todo each.

2. Mirror the same list into `.runs/<run-id>/signal/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

#### Suggested TodoWrite Items

```
- gh-issue-resolver (resolve/bind GitHub issue; may defer when GH unavailable)
- repo-operator (ensure run branch)
- signal-run-prep (establish run infrastructure)
- gh-researcher (research GitHub context + wisdom scent trail)
- signal-normalizer (normalize signal)
- problem-framer (frame the problem; check for state/migration implications)
- clarifier (capture open questions; non-blocking)
- requirements-author ↔ requirements-critic (microloop; signal-based termination)
- bdd-author ↔ bdd-critic (microloop; signal-based termination; enforce sad paths)
- scope-assessor (assess scope + initial risks)
- risk-analyst (deep risk analysis)
- spec-auditor (integrative audit; may bounce for fixes)
- signal-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (sync GitHub issue; skip when `github_ops_allowed: false`; restricted mode when publish is blocked or artifacts are not pushed)
- gh-reporter (post GitHub summary; skip when `github_ops_allowed: false`; restricted handoff when publish is blocked or artifacts are not pushed)
```

#### On Rerun

If running `/flow-1-signal` on an existing run-id:
- Read `.runs/<run-id>/signal/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

### Agents to Use

#### Issue binding (Step 0)

- **gh-issue-resolver** - MUST be called first to resolve/create the GitHub issue (or mark repo mismatch / defer binding) and emit `run_id` (`gh-<issue_number>` or `local-<slug>-<hash6>`)

#### Infrastructure (Step 0b/0c)

- **repo-operator** - Ensure run branch `run/<run-id>` exists
- **signal-run-prep** - Establish the run directory using the issue-derived run-id

#### Research (Step 1)

- gh-researcher - research existing GitHub issues/PRs for context

#### Domain Agents (Flow 1 Specific)

- signal-normalizer
- problem-framer
- requirements-author ↔ requirements-critic (microloop; signal-based termination)
- bdd-author ↔ bdd-critic (microloop; signal-based termination)
- scope-assessor

#### Cross-Cutting Agents

- clarifier
- risk-analyst

#### Integrative Audit (Before Cleanup)

- spec-auditor - final holistic audit of all Flow 1 artifacts; routes back if critical gaps

#### Cleanup + Reporting (End of Flow)

- signal-cleanup - seal receipt, update index
- secrets-sanitizer - publish gate: scans for secrets, fixes or blocks
- repo-operator - checkpoint commit (gated on secrets-sanitizer result)
- gh-issue-manager - sync GitHub issue metadata (always attempt when `gh` auth is available; full vs restricted mode based on publish gates and publish_surface)
- gh-reporter - post summary to GitHub issue (full vs restricted handoff based on publish gates)

### Orchestration Outline

#### Step 0: Resolve or Create GitHub Issue

**Call `gh-issue-resolver` first.**

This agent will:
- Resolve an explicit issue reference **or** create a new GitHub issue from the signal text
- Return `run_id` (gh-<issue_number> or local-<slug>-<hash6> when repo mismatch prevents GitHub ops) plus issue metadata in a control-plane block
- Perform no filesystem writes (runs before `.runs/<run-id>/` exists)

Use the returned `run_id` for all subsequent steps.

#### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main so run artifacts land on the run branch.

#### Step 0c: Establish Run Infrastructure

**Call `signal-run-prep`** using the issue-derived `run_id` while on the run branch.

This agent will:
- Confirm the provided `run-id` (should already be `gh-<issue_number>`)
- Create `.runs/<run-id>/signal/` directory structure
- Write `.runs/<run-id>/run_meta.json` with run metadata (binding `issue_number` when the run-id matches `gh-<n>`)
- Create artifact stub files

After this step, you will have a confirmed run directory on the run branch. All subsequent agents write to `.runs/<run-id>/signal/`.

#### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/signal/flow_plan.md`:

```markdown
# Flow 1: Signal Plan for `run-id`

## Planned Steps

- [ ] gh-issue-resolver (resolve/create issue, emit run_id)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] signal-run-prep (establish run directory)
- [ ] gh-researcher (GitHub context + wisdom scent trail)
- [ ] signal-normalizer (parse input)
- [ ] problem-framer (synthesize problem; check state/migration)
- [ ] clarifier (document ambiguities)
- [ ] requirements-author ↔ requirements-critic (microloop; signal-based termination)
- [ ] bdd-author ↔ bdd-critic (microloop; signal-based termination; enforce sad paths)
- [ ] scope-assessor (stakeholders, risks, estimate)
- [ ] risk-analyst (enrich risks)
- [ ] spec-auditor (integrative audit; may bounce for fixes)
- [ ] signal-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (sync issue metadata; restricted issue updates when publish is blocked or not pushed)
- [ ] gh-reporter (post summary; restricted handoff when publish is blocked or not pushed)

## Progress Notes

<Update as each step completes>
```

#### Step 2: Research Context

**a) GitHub context:** Use `gh-researcher` to gather related issues/PRs, prior decisions, and constraints.

This context informs problem framing and requirements. If `gh` CLI is not available, proceed without this step (document as assumption).

**b) Wisdom scent trail (optional):** Check `.runs/_wisdom/latest.md` if it exists. This file contains top learnings from the most recent wisdom flow — insights that may inform this run's approach.

If present, extract relevant learnings (especially any that relate to similar feature areas or common pitfalls) and pass them to `problem-framer` as additional context. This enables the pack to learn from itself across runs.

#### Step 3: Normalize Signal

Use `signal-normalizer` to parse the raw input into structured form.

#### Step 4: Frame the Problem

Use `problem-framer` to synthesize the normalized signal into a clear problem statement with goals, non-goals, and constraints.

#### Step 5: Clarify Ambiguities (Non-Blocking)

Use `clarifier` to document ambiguities and assumptions. This step is non-blocking—it produces questions for humans to review later, not gates for the flow.

#### Step 6: Refine Requirements (Microloop)

Alternate between `requirements-author` and `requirements-critic`:

1. Call `requirements-author` to draft requirements.
   - Writes functional requirements (REQ-001, REQ-002, etc.)
   - Writes non-functional requirements (NFR-SEC-001, NFR-PERF-001, etc.)
   - Includes acceptance criteria for each

2. Call `requirements-critic` to critique requirements.
   - Reviews testability, consistency, completeness, traceability
   - Sets `Status: VERIFIED | UNVERIFIED`
   - Sets `can_further_iteration_help: yes | no`
   - Lists issues by severity (critical, major, minor)

 3. **Route on the Requirements Critic Result block** (not by re-reading the file):
    - If `status: CANNOT_PROCEED` -> **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention
    - If `recommended_action: BOUNCE` -> follow `route_to_flow/route_to_agent`
    - If `recommended_action: RERUN` -> do the apply pass: rerun `requirements-author` once with the critique worklist, then rerun `requirements-critic` once; proceed after the second critique even if still UNVERIFIED (carry blockers honestly)
    - If `recommended_action: PROCEED` -> proceed after the re-critique pass (even if UNVERIFIED)

**Loop guidance (Signal-Based Termination)**:
- Route on critic's Result block, not pass counts.
- If critic returns `recommended_action: RERUN` AND `can_further_iteration_help: yes`: call writer again with critique worklist, then call critic again.
- Exit conditions (in priority order):
  1. `status: VERIFIED` → proceed (success)
  2. `recommended_action: PROCEED` → proceed (even if UNVERIFIED; carry blockers honestly)
  3. `can_further_iteration_help: no` → proceed (no improvement possible)
  4. Context exhausted → checkpoint and exit `PARTIAL`
- The Result block is the control plane; the critique file is the audit artifact.

#### Step 7: BDD Scenarios (Microloop)

Alternate between `bdd-author` and `bdd-critic`:

1. Call `bdd-author` to draft scenarios and examples.
   - Creates Gherkin scenarios for each functional requirement
   - Covers happy paths, edge cases, error scenarios
   - Tags scenarios with requirement IDs (@REQ-001, etc.)
   - Writes `verification_notes.md` for NFRs not expressible as BDD (always present; richer when NFRs exist)

2. Call `bdd-critic` to critique scenarios.
   - Reviews traceability (every REQ-* has scenarios)
   - Checks testability (concrete, not vibes)
   - Assesses coverage (edge cases, errors)
   - Sets `Status: VERIFIED | UNVERIFIED`
   - Sets `can_further_iteration_help: yes | no`
   - Lists issues by severity (critical, major, minor)

 3. **Route on the BDD Critic Result block** (not by re-reading the file):
    - If `status: CANNOT_PROCEED` -> **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention
    - If `recommended_action: BOUNCE` -> follow `route_to_flow/route_to_agent`
    - If `recommended_action: RERUN` -> do the apply pass: rerun `bdd-author` once with the critique worklist, then rerun `bdd-critic` once; proceed after the second critique even if still UNVERIFIED (carry blockers honestly)
    - If `recommended_action: PROCEED` -> proceed after the re-critique pass (even if UNVERIFIED)

**Loop guidance (Signal-Based Termination)**:
- Route on critic's Result block, not pass counts.
- If critic returns `recommended_action: RERUN` AND `can_further_iteration_help: yes`: call writer again with critique worklist, then call critic again.
- **Sad Path enforcement:** The critic will flag REQs missing negative scenarios. Ensure the author addresses these before proceeding.
- Exit conditions (in priority order):
  1. `status: VERIFIED` → proceed (success)
  2. `recommended_action: PROCEED` → proceed (even if UNVERIFIED; carry blockers honestly)
  3. `can_further_iteration_help: no` → proceed (no improvement possible)
  4. Context exhausted → checkpoint and exit `PARTIAL`
- The Result block is the control plane; the critique file is the audit artifact.

#### Step 8: Assess Scope

Use `scope-assessor` to capture stakeholders, early risks, and scope estimate.

Identify stakeholders, flag early risks by category, and estimate scope (S/M/L/XL).

#### Step 9: Analyze Risks

Use `risk-analyst` for deeper risk assessment.

Add risk patterns (security, compliance, data, performance) and severity ratings. This supplements `.runs/<run-id>/signal/early_risks.md` with deeper analysis.

#### Step 9b: Final Spec Audit (Integrative)

**Call `spec-auditor`** to perform an integrative audit of all Flow 1 artifacts.

This is the "Staff Engineer" check before handoff to Flow 2. The auditor reviews:
- Problem → Requirements alignment
- Requirements → BDD coverage
- Risk coverage completeness
- Cross-artifact consistency
- Unresolved critic findings

**Route on its Result block:**
- If `status: VERIFIED` → proceed to cleanup
- If `status: UNVERIFIED` with `recommended_action: BOUNCE` → route back to the specified agent (e.g., `requirements-author`, `bdd-author`) for rework, then re-run the auditor
- If `status: UNVERIFIED` with `recommended_action: PROCEED` → proceed with blockers documented (human judgment needed)
- If `status: CANNOT_PROCEED` → `FIX_ENV` (mechanical failure)

**Loop limit:** Re-run the auditor at most twice after routing to fix agents. If still UNVERIFIED after 2 fix attempts, proceed with blockers and let Flow 2/human handle.

#### Step 10: Finalize and Write Receipt

Use `signal-cleanup` to seal the receipt and update index.

This agent:
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Reads quality gate status from critic outputs
- Writes the definitive `signal_receipt.json`
- Updates `.runs/index.json` with status, last_flow, updated_at

**This step MUST complete before secrets-sanitizer and gh-issue-manager.**

#### Step 11: Sanitize Secrets (Publish Gate)

Use `secrets-sanitizer` (publish gate).

This agent is a **publish gate** that ensures no secrets are accidentally committed or posted:
- Scans the commit surface (`.runs/<run-id>/`, staged changes)
- **Fixes** what it can: redacts artifacts, externalizes code secrets
- Writes `secrets_status.json` with `safe_to_commit` and `safe_to_publish` flags

**Status semantics** (status describes what the sanitizer *did*; flags tell you what you're allowed to do):
- `CLEAN`: No secrets found; flags typically true (but always read flags, not status)
- `FIXED`: Secrets found and remediated; flags typically true **unless** `needs_upstream_fix` forced gating
- `BLOCKED_PUBLISH`: Sanitizer couldn't complete (mechanical); `safe_to_publish: false`

**Control plane:** The sanitizer returns a **Gate Result block** for orchestrator routing. `secrets_status.json` is the durable audit record. Route on the Gate Result, not by re-reading the file.

**Gate Result block (returned by secrets-sanitizer):**

<!-- PACK-CONTRACT: GATE_RESULT_V3 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
findings_count: <int>
blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
blocker_reason: <string | null>
```
<!-- PACK-CONTRACT: GATE_RESULT_V3 END -->

**Gating logic (boolean gate — the sanitizer says yes/no, orchestrator decides next steps):**
- The sanitizer is a fix-first pre-commit hook, not a router
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 11c)
- If `safe_to_commit: false`:
  - `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO failure)
  - `blocker_kind: SECRET_IN_CODE` → route to appropriate agent (orchestrator decides)
  - `blocker_kind: SECRET_IN_ARTIFACT` → investigate manually
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`
- GitHub issue/comment updates still run in restricted mode when publish is blocked or `publish_surface: NOT_PUSHED`

#### Step 11b: Checkpoint Commit

Checkpoint the audit trail **before** any GitHub operations.

**Call `repo-operator`** in checkpoint mode. The agent handles:
1. Resets staging and stages allowlist only
2. Enforces allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/signal/git_status.md` if anomaly detected
4. Handles no-op gracefully (nothing to commit = success)
5. Returns **Repo Operator Result** (control plane)

**Allowlist for Flow 1:**
- `.runs/<run-id>/signal/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

**Control plane:** The `repo-operator` returns a **Repo Operator Result block** for orchestrator routing:

```md
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null. `publish_surface` must always be present (PUSHED or NOT_PUSHED), even on no-op commits, anomalies, `safe_to_commit: false`, or push failures.

**Routing logic (from Repo Operator Result):**
- `status: COMPLETED` + `proceed_to_github_ops: true` → proceed to GitHub ops
- `status: COMPLETED_WITH_ANOMALY` → allowlist committed, anomaly documented in `git_status.md`; `proceed_to_github_ops: false`
- `status: FAILED` or `status: CANNOT_PROCEED` → mechanical failure; stop and require human intervention

**Gating interaction with secrets-sanitizer:**
- `repo-operator` reads `safe_to_commit` and `safe_to_publish` from the prior Gate Result
- If `safe_to_commit: false`: skips commit entirely
- If `safe_to_publish: false`: commits locally but skips push; sets `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`

**Why checkpoint before GitHub ops:** The issue comment can reference a stable commit SHA. Also keeps local history clean if the flow is interrupted.

#### Step 12-13: GitHub Reporting

**Call `gh-issue-manager`** (sync/update/bind issue) then **`gh-reporter`** (post summary).

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

If `issue_number` is missing and `gh` is available, `gh-issue-manager` may attempt to create/bind.

#### Step 14: Finalize Flow

**Note:** Receipt derivation is handled by `signal-cleanup` (Step 10). See the `signal-cleanup` agent documentation for derivation rules.

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Open Questions**: See `open_questions.md`
- **Assumptions Made**: See individual artifacts
- **Next Flow**: `/flow-2-plan` (after human review)

## Human Review Checklist

Before proceeding to Flow 2, humans should review:
- [ ] `.runs/<run-id>/signal/requirements.md` - Are these the right requirements?
- [ ] `.runs/<run-id>/signal/features/*.feature` - Do these scenarios cover the expected behavior?
- [ ] `.runs/<run-id>/signal/verification_notes.md` - Are NFR verification criteria adequate?
- [ ] `.runs/<run-id>/signal/early_risks.md` and `.runs/<run-id>/signal/risk_assessment.md` - Are risks acceptable?
- [ ] `.runs/<run-id>/signal/open_questions.md` - Can any questions be answered now?
```

### Artifact Outputs

All written to `.runs/<run-id>/signal/`:

| Artifact | Source Agent | Description |
|----------|--------------|-------------|
| `run_meta.json` | signal-run-prep, gh-issue-manager | Run metadata (in `.runs/<run-id>/`) |
| `flow_plan.md` | orchestrator | Execution plan and progress |
| `github_research.md` | gh-researcher | Related issues/PRs and constraints |
| `issue_normalized.md` | signal-normalizer | Structured summary of raw signal |
| `context_brief.md` | signal-normalizer | Related history and context |
| `problem_statement.md` | problem-framer | Goals, non-goals, constraints |
| `open_questions.md` | clarifier | Open questions and assumptions |
| `requirements.md` | requirements-author | Functional + non-functional requirements |
| `requirements_critique.md` | requirements-critic | Critique and iteration guidance |
| `features/*.feature` | bdd-author | BDD scenarios (Gherkin) |
| `example_matrix.md` | bdd-author | Example mapping for BDD |
| `verification_notes.md` | bdd-author | NFR verification criteria (non-BDD) |
| `bdd_critique.md` | bdd-critic | Critique of BDD scenarios |
| `stakeholders.md` | scope-assessor | Teams, systems, users affected |
| `early_risks.md` | scope-assessor | Initial risk identification by category |
| `risk_assessment.md` | risk-analyst | Deep risk analysis with severity ratings |
| `scope_estimate.md` | scope-assessor | S/M/L/XL estimate with rationale |
| `spec_audit.md` | spec-auditor | Integrative audit verdict and cross-artifact consistency |
| `signal_receipt.json` | signal-cleanup | Structured summary for downstream flows |
| `cleanup_report.md` | signal-cleanup | Artifact verification and count derivation |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings and actions taken |
| `secrets_status.json` | secrets-sanitizer | Machine-readable publish gate status |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | GitHub issue sync status |
| `gh_report_status.md` | gh-reporter | GitHub posting status |
| `github_report.md` | gh-reporter | Record of GitHub post |

### Assumptions + Questions Contract

All Flow 1 agents must emit:
- **Assumptions Made to Proceed**: What was assumed, why, and impact if wrong
- **Questions / Clarifications Needed**: Questions that would change the spec, with defaults

These sections enable humans to review what was assumed at the flow boundary, and to re-run with better inputs if needed.

**Flow 1 is designed to be re-run.** If you run `/flow-1-signal` on an existing run-id:
- `signal-run-prep` will lock onto the existing directory
- Agents will read and refine existing artifacts
- Each run improves the output based on newly resolved ambiguity

### Status States

Agents set status in their output artifacts:

- **VERIFIED** - `blockers` empty, `missing_required` empty, and all quality gates passed; assumptions documented. Set `recommended_action: PROCEED`.
- **UNVERIFIED** - `blockers` non-empty OR `missing_required` non-empty OR any quality gate UNVERIFIED; contains concrete concerns and assumptions. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED** - IO/permissions/tool failure only (exceptional); cannot read/write files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED. If agents can read inputs and form an opinion, status is VERIFIED or UNVERIFIED with assumptions, never CANNOT_PROCEED. Ambiguity uses documented assumptions + UNVERIFIED status.

### Human Collaboration

**Humans do not intervene mid-flow.** Flow 1 runs from start to finish without stopping for human input. At the end:

1. All artifacts are written to `.runs/<run-id>/signal/`
2. Summary is posted to GitHub
3. Human reviews:
   - `requirements.md`
   - `features/*.feature`
   - `early_risks.md`
   - `open_questions.md`
4. Human answers questions and updates assumptions as needed
5. Human runs `/flow-2-plan` when ready

### Completion

Flow 1 is complete when:
1. All artifacts exist under `.runs/<run-id>/signal/` (even if imperfect)
2. `flow_plan.md` is updated with final status
3. GitHub summary is posted (or `github_report.md` written if gh unavailable)

Human gate at end: **"Is this the right problem to solve?"**

If yes, proceed to `/flow-2-plan`.

---

### Orchestrator Kickoff

#### Station order + templates

##### Station order

1. `gh-issue-resolver`

2. `repo-operator` (ensure run branch)

3. `signal-run-prep`

4. `gh-researcher`

5. `signal-normalizer`

6. `problem-framer`

7. `clarifier`

8. `requirements-author` ↔ `requirements-critic` (microloop; apply Microloop Template)

9. `bdd-author` ↔ `bdd-critic` (microloop; apply Microloop Template)

10. `scope-assessor`

11. `risk-analyst`

12. `spec-auditor` (integrative audit; may route back for fixes)

13. `signal-cleanup`

14. `secrets-sanitizer`

15. `repo-operator` (checkpoint; read Repo Operator Result)

16. `gh-issue-manager` (if allowed)

17. `gh-reporter` (if allowed)

##### Microloop Template (writer ↔ critic)

Run this template for: tests, code, docs, requirements, BDD, options, contracts, observability.

1) Writer pass: call `<writer>`
2) Critique pass: call `<critic>` and read its control-plane Result
3) Route on critic Result:
   - If `recommended_action: PROCEED` → proceed (no apply pass needed)
   - If `recommended_action: RERUN` AND `can_further_iteration_help: yes` → continue to step 4
   - Otherwise → proceed with `UNVERIFIED` + blockers recorded
4) Apply pass: call `<writer>` with the critique worklist
5) Re-critique: call `<critic>` again, return to step 3

**Termination:** Signal-based, not count-based. Loop continues while critic says RERUN + can_further_iteration_help: yes. Exit when signal says stop or context exhausted.

#### TodoWrite (copy exactly)

- [ ] gh-issue-resolver (issue binding -> run_id)
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] signal-run-prep
- [ ] gh-researcher
- [ ] signal-normalizer
- [ ] problem-framer
- [ ] clarifier
- [ ] requirements-author ↔ requirements-critic (microloop; signal-based termination)
- [ ] bdd-author ↔ bdd-critic (microloop; signal-based termination)
- [ ] scope-assessor
- [ ] risk-analyst
- [ ] spec-auditor (integrative audit; may bounce for fixes)
- [ ] signal-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint; capture Repo Operator Result)
- [ ] gh-issue-manager (skip when `github_ops_allowed: false`; full when `safe_to_publish` + `proceed_to_github_ops` + `publish_surface: PUSHED`; restricted updates otherwise when gh auth is available)
- [ ] gh-reporter (skip when `github_ops_allowed: false`; full only when publish gates are clear and artifacts pushed; restricted handoff otherwise)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

---

## flow-2-plan.md

---
description: Run Flow 2 (Spec to Design): produce ADR, contracts, observability spec, test/work plans, design validation.
---

## Flow 2: Spec to Design

You are orchestrating Flow 2 of the SDLC swarm.

### Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/plan/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/plan/` exists.

##### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

### Your Goals

- Turn requirements into architecture decisions
- Define API contracts and data models
- Create observability, test, and work plans
- Validate design feasibility

### Before You Begin (Required)

#### Two State Machines

Flow 2 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

#### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Parallel steps (6-9) are ONE todo.
   - Microloops (`design-optioneer` ↔ `option-critic`, `interface-designer` ↔ `contract-critic`, `observability-designer` ↔ `observability-critic`) are ONE todo each.

2. Mirror the same list into `.runs/<run-id>/plan/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

#### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure; initialize `flow_plan.md`)
- repo-operator (ensure run branch)
- clarifier (plan open questions)
- impact-analyzer (map impact)
- design-optioneer ↔ option-critic (microloop; signal-based termination)
- adr-author (write ADR)
- interface-designer / observability-designer / test-strategist / work-planner (lanes; parallel)
- interface-designer ↔ contract-critic (microloop; signal-based termination; recommended)
- observability-designer ↔ observability-critic (microloop; signal-based termination; recommended)
- design-critic (integrative validation; may return worklist)
- policy-analyst (policy compliance)
- plan-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (update issue status board; gated)
- gh-reporter (post Plan summary; gated)
```

#### Critic choreography (default behavior)

Think in **worklists**, not "who wins".

- **Signal-based microloop:** writer → critic → route on Result. If critic returns `RERUN` AND `can_further_iteration_help: yes`: call writer with critique worklist, then call critic again. Otherwise proceed (carry blockers honestly).
- **Option critique (early):** Apply microloop pattern between `design-optioneer` and `option-critic`.
- **Lane worklists:** If `contract-critic` or `observability-critic` returns `recommended_action: RERUN | BOUNCE | FIX_ENV`, treat that as the active worklist for its lane unless you resolve it or explicitly defer it (Decision Log entry).
- **Integration read (late):** `design-critic` is integrative across artifacts. Run it after lane worklists are resolved/deferred. A later `design-critic` `PROCEED` does not clear an open lane worklist.

#### Decision log (only when you defer a critic worklist)

If you intentionally proceed while a critic still has an open worklist (e.g., you choose not to rerun/bounce), record a short entry in `.runs/<run-id>/plan/flow_plan.md` capturing what you deferred, why, evidence, and what you will re-check before sealing `plan_receipt.json`.

#### On Rerun

If running `/flow-2-plan` on an existing run-id:
- Read `.runs/<run-id>/plan/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

If you encounter ambiguity or missing information, **document it and continue**. Write assumptions clearly in artifacts.

### Subagents to use

Flow 2 uses infrastructure + domain agents + cross-cutting agents:

#### Infrastructure (Step 0)
- run-prep (establish run directory)

#### Domain agents (in order)
- impact-analyzer
- design-optioneer
- option-critic
- adr-author
- interface-designer
- contract-critic
- observability-designer
- observability-critic
- test-strategist
- work-planner
- design-critic

#### Cross-cutting agents
- clarifier (Plan-local open questions)
- risk-analyst (if risk patterns identified)
- policy-analyst (policy compliance check)
- plan-cleanup (seal receipt, update index)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit - gated on secrets-sanitizer result)
- gh-issue-manager (update issue status board)
- gh-reporter (one comment per Plan run)

### Upstream Inputs

Read from `.runs/<run-id>/signal/` (if available):
- `problem_statement.md`
- `requirements.md`
- `requirements_critique.md`
- `features/*.feature` (BDD scenarios)
- `example_matrix.md`
- `bdd_critique.md`
- `verification_notes.md` (NFR verification criteria)
- `stakeholders.md`
- `early_risks.md`
- `risk_assessment.md`
- `scope_estimate.md`
- `open_questions.md` (Signal's question register)
- `signal_receipt.json` (optional; provides counts and quality gate status without re-parsing)

**If upstream artifacts are missing**: Flow 2 can start without Flow 1. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue. This enables flexibility for hotfixes or design-first workflows.

### Orchestration outline

#### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/plan/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "plan" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/plan/`.

#### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

#### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/plan/flow_plan.md`:

```markdown
# Flow 2: Plan for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] clarifier (Plan open questions)
- [ ] impact-analyzer (map affected components)
- [ ] design-optioneer ↔ option-critic (microloop; apply Microloop Template)
- [ ] adr-author (write architecture decision)
- [ ] interface-designer (contracts/schema; lane; parallel)
- [ ] interface-designer ↔ contract-critic (microloop; apply Microloop Template)
- [ ] observability-designer (observability; lane; parallel)
- [ ] observability-designer ↔ observability-critic (microloop; apply Microloop Template)
- [ ] test-strategist (test plan; lane; parallel)
- [ ] work-planner (work plan; lane; parallel)
- [ ] design-critic (integrative validation; may return worklist)
- [ ] policy-analyst (check compliance)
- [ ] plan-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>

## Decision Log (only when you defer a critic worklist)

- Deferred: <critic-name> requested <RERUN|BOUNCE|FIX_ENV> on <artifact> -> proceeding with <action>
  - Why: <short>
  - Evidence: <artifact/path pointers>
  - Re-check before seal: <what you will re-verify before plan-cleanup>
```

#### Step 2: Plan Open Questions (Non-blocking)

Call `clarifier` to create the Plan-local questions register. Signal's `open_questions.md` is upstream input; Plan gets its own register for design-phase questions.

#### Step 3: Map impact
- Use `impact-analyzer` to map impact and blast radius.

#### Step 4: Propose design options
- Use `design-optioneer` to propose design options.

#### Step 4b: Critique design options (microloop; recommended)
- Use `option-critic` to critique `design_options.md` and write `option_critique.md`.

**Route on the Option Critic Result block** (not by re-reading the file):
 - If `recommended_action: FIX_ENV` -> stop (mechanical failure; IO/permissions/tooling)
- If `recommended_action: BOUNCE` -> bounce to `route_to_flow`/`route_to_agent`
- If `recommended_action: RERUN` -> do the apply pass: rerun `route_to_agent` once (typically `design-optioneer`) using the critique worklist, then rerun `option-critic` once; proceed after the second critique even if still UNVERIFIED (Decision Log when deferring)
- If `recommended_action: PROCEED` -> proceed after the re-critique pass

#### Step 5: Write ADR
- Use `adr-author` to write the ADR.

#### Step 6: Define contracts and schema (can run in parallel with steps 7-9)
- Use `interface-designer` for contracts/schema/migrations (planned migrations live under the run directory; actual migrations move during Build).

#### Step 6b: Validate contracts (microloop; recommended)
- Use `contract-critic` to validate `api_contracts.yaml` + `schema.md` and write `contract_critique.md`.

**Route on the Contract Critic Result block** (not by re-reading the file):
- If `recommended_action: FIX_ENV` → stop (mechanical failure; IO/permissions/tooling)
- If `recommended_action: BOUNCE` → bounce to `route_to_flow`/`route_to_agent`
- If `recommended_action: RERUN` → do the apply pass: rerun `route_to_agent` once (typically `interface-designer`) using the critique worklist, then rerun `contract-critic` once; proceed after the second critique even if still UNVERIFIED (Decision Log when deferring)
- If `recommended_action: PROCEED` → proceed after the re-critique pass

**Conflict note (default):** If Contract Critic requests `RERUN`/`BOUNCE`/`FIX_ENV`, treat that as an open contract-lane worklist unless you resolve it or explicitly defer it (record a Decision Log entry in `flow_plan.md`).

#### Step 7: Plan observability (parallel)
- Use `observability-designer` to define observability.

#### Step 7b: Validate observability (microloop; recommended)
- Use `observability-critic` to validate `observability_spec.md` and write `observability_critique.md`.

**Route on the Observability Critic Result block** (not by re-reading the file):
- If `recommended_action: FIX_ENV` → stop (mechanical failure; IO/permissions/tooling)
- If `recommended_action: BOUNCE` → bounce to `route_to_flow`/`route_to_agent`
- If `recommended_action: RERUN` → do the apply pass: rerun `route_to_agent` once (typically `observability-designer`) using the critique worklist, then rerun `observability-critic` once; proceed after the second critique even if still UNVERIFIED (Decision Log when deferring)
- If `recommended_action: PROCEED` → proceed after the re-critique pass

**Conflict note (default):** If Observability Critic requests `RERUN`/`BOUNCE`/`FIX_ENV`, treat that as an open observability-lane worklist unless you resolve it or explicitly defer it (record a Decision Log entry in `flow_plan.md`).

#### Step 8: Plan testing (parallel)
- Use `test-strategist` to write the test plan (incorporate Signal BDD + verification notes).

#### Step 9: Plan work (parallel)
- Use `work-planner` — "produce subtask index + work plan".

#### Step 10: Validate design (microloop)
- Use `design-critic` to validate the design.

**Conflict handling (default):**
- If a targeted critic is still requesting `RERUN`/`BOUNCE`/`FIX_ENV`, keep that lane's worklist open until resolved or explicitly deferred (Decision Log entry in `flow_plan.md`). You can still run `design-critic` for an integration read.

**Route on the Design Critic Result block** (not by re-reading the file):
- If `status: VERIFIED` → proceed to policy check
- If `status: UNVERIFIED` AND `can_further_iteration_help: yes` → rerun affected steps (options/ADR/contracts/plans); if you rerun `interface-designer` or `observability-designer`, run the matching targeted critic (`contract-critic` / `observability-critic`) before re-running design-critic
- If `status: UNVERIFIED` AND `can_further_iteration_help: no` → proceed (remaining issues documented)
- If `status: CANNOT_PROCEED` → **FIX_ENV** (mechanical failure; IO/permissions/tooling); stop and require human intervention

**Loop guidance**: The Result block is the control plane; `design_validation.md` is the audit artifact. Agents do not know they are in a loop—they read inputs, write outputs, and set a status. The orchestrator routes on the Result block.

#### Step 11: Check policy compliance
- Use `policy-analyst` for policy compliance.

#### Step 12: Finalize Plan (receipt + index)
- Use `plan-cleanup` to seal the receipt, verify artifacts, and update index counts mechanically.

#### Step 13: Sanitize secrets (publish gate)
- Use `secrets-sanitizer` (publish gate).

**Gate Result block (returned by secrets-sanitizer):**

The agent returns a Gate Result block for orchestrator routing:

<!-- PACK-CONTRACT: GATE_RESULT_V3 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
findings_count: <int>
blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
blocker_reason: <string | null>
```
<!-- PACK-CONTRACT: GATE_RESULT_V3 END -->

**Field semantics:**
- `status` is **descriptive** (what happened). **Never infer permissions** from it.
- `safe_to_commit` / `safe_to_publish` are **authoritative permissions**.
- `modified_files` signals that artifact files were changed (for audit purposes).
- `blocker_kind` explains why blocked (machine-readable category): `NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT`

**Control plane vs audit plane:** The Gate Result block is the control plane for orchestrator routing. `secrets_status.json` is the durable audit record. Route on the returned block, not by re-reading the file.

**Gating logic (boolean gate — the sanitizer says yes/no, orchestrator decides next steps):**
- The sanitizer is a fix-first pre-commit hook, not a router
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 13c)
- If `safe_to_commit: false`:
  - `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO failure)
  - `blocker_kind: SECRET_IN_CODE` → route to appropriate agent (orchestrator decides)
  - `blocker_kind: SECRET_IN_ARTIFACT` → investigate manually
- Publish mode gating: `FULL` only when `safe_to_publish: true`, Repo Operator Result `proceed_to_github_ops: true`, **and** `publish_surface: PUSHED`. Otherwise, GitHub ops (when access is allowed) run in `RESTRICTED` mode. Publish blocked implies RESTRICTED, **not skip**.

#### Step 13b: Checkpoint Commit

Checkpoint the audit trail **before** any GitHub operations.

**Call `repo-operator`** in checkpoint mode. The agent handles:
1. Resets staging and stages allowlist only
2. Enforces allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/plan/git_status.md` if anomaly detected
4. Handles no-op gracefully (nothing to commit = success)
5. Returns **Repo Operator Result** (control plane)

**Allowlist for Flow 2:**
- `.runs/<run-id>/plan/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

**Control plane:** The `repo-operator` returns a **Repo Operator Result block** for orchestrator routing:

```md
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null. `publish_surface` must always be present (PUSHED or NOT_PUSHED), even on no-op commits, anomalies, `safe_to_commit: false`, push skipped, or push failure.

**Routing logic (from Repo Operator Result):**
- `status: COMPLETED` + `proceed_to_github_ops: true` → proceed to GitHub ops
- `status: COMPLETED_WITH_ANOMALY` → allowlist committed, anomaly documented in `git_status.md`; `proceed_to_github_ops: false`
- `status: FAILED` or `status: CANNOT_PROCEED` → mechanical failure; stop and require human intervention

**Gating interaction with secrets-sanitizer:**
- `repo-operator` reads `safe_to_commit` and `safe_to_publish` from the prior Gate Result
- If `safe_to_commit: false`: skips commit entirely
- If `safe_to_publish: false`: commits locally but skips push; sets `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`

**Why checkpoint before GitHub ops:** The issue comment can reference a stable commit SHA. Also keeps local history clean if the flow is interrupted.

#### Step 14-15: GitHub Reporting

**Call `gh-issue-manager`** then **`gh-reporter`** to update the issue.

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

#### Step 16: Finalize flow_plan.md

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **ADR Decision**: <brief summary of chosen approach>
- **Design Concerns**: See `.runs/<run-id>/plan/design_validation.md`
- **Next Flow**: `/flow-3-build` (after human review)

## Human Review Checklist

Before proceeding to Flow 3, humans should review:
- [ ] `.runs/<run-id>/plan/adr.md` - Is this the right architecture decision?
- [ ] `.runs/<run-id>/plan/api_contracts.yaml` - Are the contracts correct?
- [ ] `.runs/<run-id>/plan/work_plan.md` - Is the breakdown reasonable?
- [ ] `.runs/<run-id>/plan/design_validation.md` - Are flagged concerns acceptable?
```

### Downstream Contract

Flow 2 is complete when these exist (even if imperfect):

- `flow_plan.md` - Execution plan and progress
- `plan_receipt.json` - Receipt for downstream consumers
- `impact_map.json` - Services, modules, data, external systems affected
- `design_options.md` - 2-3 architecture options with trade-offs
- `option_critique.md` - Options critique + worklist (decision readiness)
- `adr.md` - Chosen option with rationale and consequences
- `api_contracts.yaml` - Endpoints, schemas, error shapes
- `schema.md` - Data models, relationships, invariants
- `migrations/*.sql` - Draft migrations (optional, if DB changes needed)
- `observability_spec.md` - Metrics, logs, traces, SLOs, alerts
- `test_plan.md` - BDD to test types mapping, priorities
- `ac_matrix.md` - AC-driven build contract (Flow 3 iterates per AC; Build creates `build/ac_status.json` at runtime)
- `work_plan.md` - Subtasks, ordering, dependencies
- `design_validation.md` - Feasibility assessment, known issues

### Status States

Agents set status in their output artifacts:

- **VERIFIED**: `blockers` empty, `missing_required` empty, and all quality gates passed; artifact complete for its purpose. Set `recommended_action: PROCEED`.
- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR any quality gate UNVERIFIED; artifact created but has issues. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read/write files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.

Use `plan_receipt.json` (primary) and the latest critic Result blocks (secondary) to determine flow outcome. When critic signals conflict, default to keeping targeted-critic `RERUN`/`BOUNCE`/`FIX_ENV` as an open lane worklist unless explicitly deferred (Decision Log entry in `flow_plan.md`).

### Notes

- Steps 6-9 can run in parallel after `adr-author` completes
- `design-critic` reviews ALL artifacts before policy check
- `option-critic` critiques options before ADR authoring
- Human gate at end: "Is this the right design?"
- Agents never block; they document concerns and continue

### Artifact Outputs

All written to `.runs/<run-id>/plan/`:

| Artifact | Source Agent | Description |
|----------|--------------|-------------|
| `flow_plan.md` | orchestrator | Execution plan and progress |
| `open_questions.md` | clarifier | Plan-local questions register |
| `impact_map.json` | impact-analyzer | Affected services, modules, data |
| `design_options.md` | design-optioneer | 2-3 architecture options |
| `option_critique.md` | option-critic | Options critique + worklist |
| `adr.md` | adr-author | Chosen option with rationale |
| `api_contracts.yaml` | interface-designer | Endpoints, schemas, errors |
| `schema.md` | interface-designer | Data models, relationships |
| `migrations/*.sql` | interface-designer | Draft migrations (if needed) |
| `contract_critique.md` | contract-critic | Contract validation critique (optional) |
| `observability_spec.md` | observability-designer | Metrics, logs, traces, SLOs |
| `observability_critique.md` | observability-critic | Observability validation critique (optional) |
| `test_plan.md` | test-strategist | BDD to test types mapping |
| `ac_matrix.md` | test-strategist | AC-driven build contract (Build creates `build/ac_status.json`) |
| `work_plan.md` | work-planner | Subtasks, ordering, dependencies |
| `design_validation.md` | design-critic | Feasibility assessment |
| `policy_analysis.md` | policy-analyst | Policy compliance check |
| `plan_receipt.json` | plan-cleanup | Receipt for downstream |
| `cleanup_report.md` | plan-cleanup | Cleanup status and evidence |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan report |
| `secrets_status.json` | secrets-sanitizer | Publish gate status |
| `gh_issue_status.md` | gh-issue-manager | Issue board update status |
| `gh_report_status.md` | gh-reporter | GitHub posting status |
| `github_report.md` | gh-reporter | Report content (local copy) |
| `git_status.md` | repo-operator | Git tree status (if anomaly detected) |

---

### Orchestrator Kickoff

#### Station order + templates

##### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `clarifier`

4. `impact-analyzer`

5. `design-optioneer` ↔ `option-critic` (microloop; apply Microloop Template)

6. `adr-author`

7. `interface-designer` / `observability-designer` / `test-strategist` / `work-planner` (parallel)

8. `interface-designer` ↔ `contract-critic` (microloop; apply Microloop Template; recommended)

9. `observability-designer` ↔ `observability-critic` (microloop; apply Microloop Template; recommended)

10. `design-critic` (integrative validation; route to options/contracts/observability/plans as returned; rerun once to confirm the top worklist moved)

11. `policy-analyst`

12. `plan-cleanup`

13. `secrets-sanitizer`

14. `repo-operator` (checkpoint; read Repo Operator Result)

15. `gh-issue-manager` (if allowed)

16. `gh-reporter` (if allowed)

##### Microloop Template (writer ↔ critic)

Run this template for: tests, code, docs, requirements, BDD, options, contracts, observability.

1) Writer pass: call `<writer>`
2) Critique pass: call `<critic>` and read its control-plane Result
3) Route on critic Result:
   - If `recommended_action: PROCEED` → proceed (no apply pass needed)
   - If `recommended_action: RERUN` AND `can_further_iteration_help: yes` → continue to step 4
   - Otherwise → proceed with `UNVERIFIED` + blockers recorded
4) Apply pass: call `<writer>` with the critique worklist
5) Re-critique: call `<critic>` again, return to step 3

**Termination:** Signal-based, not count-based. Loop continues while critic says RERUN + can_further_iteration_help: yes. Exit when signal says stop or context exhausted.

#### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] clarifier (plan open questions)
- [ ] impact-analyzer
- [ ] design-optioneer ↔ option-critic (microloop; signal-based termination)
- [ ] adr-author
- [ ] interface-designer / observability-designer / test-strategist / work-planner (parallel)
- [ ] interface-designer ↔ contract-critic (microloop; signal-based termination; recommended)
- [ ] observability-designer ↔ observability-critic (microloop; signal-based termination; recommended)
- [ ] design-critic (microloop if needed)
- [ ] policy-analyst
- [ ] plan-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint; capture Repo Operator Result)
- [ ] gh-issue-manager (skip when github_ops_allowed: false; FULL/RESTRICTED based on gates/publish_surface)
- [ ] gh-reporter (skip when github_ops_allowed: false; FULL/RESTRICTED based on gates/publish_surface)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

---

## flow-3-build.md

---
description: Run Flow 3 (Design -> Code): build a working, codebase-aligned implementation.
## argument-hint: [run-id]
---

## Flow 3: Build

You are orchestrating Flow 3 of the SDLC swarm.

**Goal:** Build a working, codebase-aligned implementation. Tests pass. Diff is honest.

### Mental Model

**Flow 3 does not stop until the AC is verifiable and the code is clean.**

This is the Stubborn Loop: implement → test → critique → fix → repeat until the critics are satisfied. The implementer says "I'm done" — the critics verify or reject that claim. If rejected, the implementer goes again.

Flow 3 grabs external feedback (PR, CI, bots) when available to unblock the build. Route CRITICAL blockers immediately. Defer the full worklist to Flow 4.

### Working Directory + Paths

- All commands run from **repo root**
- All paths are **repo-root-relative**
- Run artifacts: `.runs/<run-id>/build/`
- Code/tests: project-defined locations

### Orchestration Model

**You direct agents and route on their responses.**

- Call agents - they do the work
- Listen to responses - agents tell you what happened via Result blocks
- Route on `status`, `recommended_action`, `route_to_*`
- Do not re-read files to make routing decisions

### Before You Begin

#### State Machines

1. **TodoWrite** = session navigation (ephemeral)
2. **`flow_plan.md`** = durable on-disk state (enables reruns)

Create TodoWrite immediately. Write `flow_plan.md` after `run-prep` creates the run directory.

#### On Rerun

If `.runs/<run-id>/build/` exists:
- Read `flow_plan.md` for navigation state
- **Call `build-cleanup`** to get AC completion status (every call is an implicit resume — the agent checks disk state)
- Route on the returned `Build Cleanup Result` block:
  - `ac_completed` / `ac_total` tells you where to resume
  - Do NOT parse `ac_status.json` directly — the agent owns that file
- Pre-mark completed items as done based on the agent's report

### The Build Loop

#### Step 0: Infrastructure

**Call `run-prep`** to establish `.runs/<run-id>/build/`.

#### Step 1: Git Prep

**Call `repo-operator`**: "ensure run branch `run/<run-id>`"

#### Step 2: Load Context

**Call `context-loader`** to assemble the working set.

#### Step 3: Clarify (Non-blocking)

**Call `clarifier`** to capture open questions. Document assumptions and continue.

#### Step 4: AC Loop

Read `.runs/<run-id>/plan/ac_matrix.md` for the ordered AC list.

**If `ac_matrix.md` is missing:** Call `test-strategist` to generate it first.

**Note:** `build-cleanup` owns `ac_status.json` — it will create/update it based on test-executor results. The orchestrator does not touch this file directly.

**For each AC in order:**

1. **test-author**: Write tests for this AC
2. **test-critic**: Verify tests are solid
3. **code-implementer**: Implement to pass tests
4. **code-critic**: Verify implementation is honest
5. **test-executor**: Confirm tests pass (AC-scoped); emits `ac_status` in result block
6. **build-cleanup** (or `ac-tracker` if added): Updates `ac_status.json` based on test-executor result

**Note:** The orchestrator routes on `test-executor`'s result block. It does NOT parse `ac_status.json` directly. The cleanup agent owns state file updates.

**Adversarial Microloop (writer ↔ critic):**

The critic's job is to *find the flaw*. The writer's job is to *fix it*. This is not friendly peer review — it's adversarial verification.

```
writer → critic → [if RERUN] → writer → critic → ... → [PROCEED]
```

Route on the critic's Result block:
- `RERUN`: Send the worklist back to the writer
- `PROCEED`: Move forward (even if `status: UNVERIFIED` — blockers are documented)
- `can_further_iteration_help: no`: Stop iterating, proceed with blockers

**Handling Logic Mismatches (Law 7: Local Resolution):**

When implementation contradicts the ADR or hits an impossible constraint:

1. **Don't bail.** Machine time is cheap.
2. **Call a specialist:** Route to `design-optioneer` or `impact-analyzer` for a surgical "Design Fix" scoped to the current AC.
3. **Re-plan locally:** Have the specialist update `ac_matrix.md` or emit a micro-decision.
4. **Resume:** Hand the fix back to the implementer.

**Only BOUNCE to Flow 2 if the specialists agree the entire architecture is invalid.**

This is the "Stubborn PM" posture: exhaust local options before interrupting the human with a flow bounce.

**AC Termination (Law 4: Green + Orchestrator Agreement):**

An AC is complete when BOTH conditions are met:
1. **test-executor returns Green** for that AC's scope
2. **Orchestrator agrees** there's nothing left worth fixing

**"Green is a floor, not a ceiling."** Passing tests prove functional correctness. But professional code also needs maintainability.

**Post-Green Polish Pass Protocol:**

When `test-executor` returns Green, **read the latest `code-critic` report** before marking the AC complete:

1. **Check `code_critique.md`:** Does the critic identify:
   - Logic debt (fragile patterns, hidden coupling)
   - Maintainability risks (unclear naming, duplicated code)
   - Obvious improvements (missing error handling, unsafe patterns)

2. **Authorize one polish pass** if:
   - Critic identified concrete, fixable issues (not just stylistic preferences)
   - `can_further_iteration_help: yes`
   - The fix is scoped to the current AC (not architectural)

3. **Proceed without polish** if:
   - Critic says `can_further_iteration_help: no`
   - Issues are minor/stylistic (defer to Flow 4)
   - Issues require architectural changes (defer to Flow 4 or future work)

**The single polish pass rule:** One extra iteration to clean up what the critic found. Not an infinite loop of gold-plating. One pass, then proceed.

**After first vertical slice (AC-1 complete):**
1. Call `repo-operator`: checkpoint push
2. Call `pr-creator`: create Draft PR (gets bots spinning early)
3. Call `pr-feedback-harvester`: check for CRITICAL blockers only
   - Route top 1-3 blockers to appropriate agent
   - Continue AC loop (don't drain the full list - Flow 4 owns that)

**Checkpoint cadence:** Push after every 3-5 ACs. Feedback check after each push.

#### Step 5: Global Hardening

After all ACs complete:

1. **standards-enforcer**: Format/lint + honest diff check
   - If `HIGH_RISK` (suspicious test deletion): proceed, but flag is visible to Gate
   - If `UNVERIFIED` (coherence issues): route to `code-implementer`

2. **test-executor**: Full suite (not AC-filtered)

3. **flakiness-detector**: If failures, classify deterministic vs flaky

4. **mutation-auditor**: Bounded mutation run on changed files
   - Route survivors to `test-author` or `fixer`

5. **fuzz-triager**: If configured, run bounded fuzz

6. **fixer**: Apply targeted fixes if critiques/worklists require it

#### Step 6: Documentation

**doc-writer ↔ doc-critic** microloop

#### Step 7: Self-Review

**Call `self-reviewer`** for final consistency check.

#### Step 8: Flow Boundary Harvest

**Call `pr-feedback-harvester`** one last time (if PR exists):
- Route CRITICAL blockers only (bounded)
- Record unresolved items for Flow 4

#### Step 9: Cleanup + Commit

1. **build-cleanup**: Write `build_receipt.json`, update index
2. **repo-operator**: Stage intended changes
3. **secrets-sanitizer**: Pre-publish sweep
4. **repo-operator**: Commit and push (if gates allow)
5. **gh-issue-manager** + **gh-reporter**: Update GitHub (if allowed)

#### Step 10: Finalize

Update `flow_plan.md` with completion status.

### Routing Rules

Route on the Result block returned by each agent:

| `status` | `recommended_action` | What to do |
|----------|---------------------|------------|
| VERIFIED | PROCEED | Continue to next station |
| UNVERIFIED | PROCEED | Continue with blockers documented |
| UNVERIFIED | RERUN | Rerun the producer/writer |
| UNVERIFIED | BOUNCE | Route to `route_to_flow` / `route_to_agent` |
| CANNOT_PROCEED | FIX_ENV | Stop - mechanical failure |

If `recommended_action` is absent: use `can_further_iteration_help` as tie-breaker (`no` → proceed).

### Agents

**Infrastructure:**
- `run-prep` - establish run directory

**Git:**
- `repo-operator` - branch, stage, commit, push

**Context:**
- `context-loader` - curate working set
- `clarifier` - document ambiguities (non-blocking)

**Test loop:**
- `test-author` - write tests
- `test-critic` - verify tests

**Code loop:**
- `code-implementer` - implement code
- `code-critic` - verify implementation

**Hardening:**
- `test-executor` - run tests
- `standards-enforcer` - format/lint + honest diff check
- `flakiness-detector` - classify test failures
- `mutation-auditor` - mutation testing
- `fuzz-triager` - fuzz testing (if configured)
- `fixer` - targeted fixes

**Polish:**
- `doc-writer` - update docs
- `doc-critic` - review docs
- `self-reviewer` - final consistency check

**Cleanup:**
- `build-cleanup` - write receipt, update index
- `secrets-sanitizer` - pre-publish sweep
- `pr-creator` - create Draft PR
- `pr-feedback-harvester` - harvest bot/human feedback
- `gh-issue-manager` - update issue board
- `gh-reporter` - post summary to GitHub

### Upstream Inputs

Read from `.runs/<run-id>/plan/` (if available):
- `adr.md`, `api_contracts.yaml`, `schema.md`
- `test_plan.md`, `ac_matrix.md`, `work_plan.md`

**If upstream artifacts are missing:** Proceed best-effort, document assumptions, set status UNVERIFIED.

### Output Artifacts

After completion, `.runs/<run-id>/build/` contains:
- `flow_plan.md` - execution plan with checkboxes
- `ac_status.json` - AC completion tracker
- `test_changes_summary.md`, `test_critique.md`
- `impl_changes_summary.md`, `code_critique.md`
- `test_execution.md`, `standards_report.md`
- `mutation_report.md`, `flakiness_report.md` (if run)
- `doc_updates.md`, `doc_critique.md`
- `self_review.md`, `build_receipt.json`
- `pr_feedback.md`, `feedback_blockers.md` (if PR exists)

Plus code/test changes in project-defined locations.

### Status States

- **VERIFIED**: Tests pass, diff is honest, ready for Flow 4
- **UNVERIFIED**: Gaps documented, proceed with blockers
- **CANNOT_PROCEED**: Mechanical failure (IO/permissions/tooling)

### TodoWrite (copy exactly)

**These are the agents you call, in order. Do not group. Do not summarize. Execute each line.**

```
- [ ] run-prep
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] context-loader
- [ ] clarifier
- [ ] test-strategist (if ac_matrix.md missing)
- [ ] AC-1: test-author ↔ test-critic microloop
- [ ] AC-1: code-implementer ↔ code-critic microloop
- [ ] AC-1: test-executor (emits ac_status in result)
- [ ] repo-operator (checkpoint push)
- [ ] pr-creator (create Draft PR)
- [ ] pr-feedback-harvester (check CRITICAL only, route blockers)
- [ ] [repeat AC-2..N with same pattern]
- [ ] standards-enforcer (format/lint + suspicious deletion check)
- [ ] test-executor (full suite)
- [ ] flakiness-detector (if failures exist)
- [ ] mutation-auditor
- [ ] fuzz-triager (if configured)
- [ ] fixer (if critiques/mutation have worklist)
- [ ] doc-writer ↔ doc-critic microloop
- [ ] self-reviewer
- [ ] pr-feedback-harvester (flow boundary check)
- [ ] build-cleanup (writes ac_status.json + build_receipt.json)
- [ ] repo-operator (stage intended changes)
- [ ] secrets-sanitizer
- [ ] repo-operator (commit and push)
- [ ] gh-issue-manager
- [ ] gh-reporter
```

**Why explicit?** The orchestrator (you) executes what's in the list. Grouped phases get skipped. Explicit agents get called.

Use explore agents to answer immediate questions, then create the todo list and call agents.

---

## flow-4-review.md

---
description: "Run Flow 4 (Review): harvest PR feedback, apply fixes, flip Draft to Ready when complete."
---

## Flow 4: PR Review + Improvement

You are orchestrating Flow 4 of the SDLC swarm.

### The Mental Model: "The Finishing School"

Flow 3 built the house. Flow 4 does the punch list.

**Mentality:** Feedback is noisy, time is linear, code rots instantly. Grab what's available, fix it, report it, move on. Don't wait for perfect signal.

**Three Phases:**
1. **Harvest & Cluster** — Pull all feedback, cluster into actionable Work Items
2. **Execute** — Route Work Items to agents, fix what's current, skip what's stale
3. **Close the Loop** — Update the PR, show humans what was addressed

**Key principle:** Agents are smart. They read the file, see if the code is there, fix it or report "context changed." No separate stale-check ceremony.

### Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/review/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Setup (run-prep) establishes the run directory and ensures `.runs/<run-id>/review/` exists.

##### Artifact visibility rule

* Do **not** attempt to prove files exist under `.runs/<run-id>/` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on verification agents to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

### Your Goals

- Ensure a PR exists (create Draft if missing)
- Harvest all available PR feedback (grab partials from CI if already failing)
- Convert feedback into clustered Work Items (by file/theme, not individual comments)
- Apply fixes until completion (agents handle staleness naturally)
- Flip Draft PR to Ready when review is complete
- Post a closure checklist so humans see feedback was addressed

### Before You Begin (Required)

#### Two State Machines

Flow 4 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

#### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - The worklist loop is ONE todo (unbounded iterations).

2. Mirror the same list into `.runs/<run-id>/review/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

#### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- pr-creator (create Draft PR if none exists)
- pr-feedback-harvester (pull all bot/human feedback)
- review-worklist-writer (cluster into actionable items)
- worklist loop (unbounded: resolve items until completion/context/unrecoverable)
- pr-commenter (post/update PR summary comment)
- pr-status-manager (flip Draft to Ready if review complete)
- review-cleanup (finalize receipt; update index; update flow_plan.md)
- secrets-sanitizer (publish gate)
- repo-operator (commit/push)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report to GitHub; gated)
```

#### On Rerun

If running `/flow-4-review` on an existing run-id:
- Read `.runs/<run-id>/review/flow_plan.md`
- Read `.runs/<run-id>/review/review_worklist.json` for current item statuses
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Resume the worklist loop from pending items

If you encounter missing PR or unclear state, **document it and continue**. Create the PR if possible.

### Subagents to use

**Infrastructure (Step 0)**:
- **run-prep** -- establish the run directory and `.runs/<run-id>/review/`

**Git operations (cross-cutting)**:
- repo-operator -- branch at start, commit at end

**PR lifecycle**:
- pr-creator -- create Draft PR if none exists
- pr-feedback-harvester -- read all PR feedback sources
- review-worklist-writer -- convert feedback to actionable worklist
- pr-commenter -- post idempotent PR summary comment (after worklist loop)
- pr-status-manager -- flip Draft to Ready when review complete

**Fix loop agents (reused from Build)**:
- test-author -- fix test-related items
- code-implementer -- fix code-related items
- doc-writer -- fix documentation items
- fixer -- apply targeted fixes
- test-executor -- verify fixes

**Polish and wrap-up**:
- build-cleanup -- reseal build receipt after code changes
- review-cleanup -- write review_receipt.json, update index

**Cleanup + Reporting (End of Flow)**:
- secrets-sanitizer -- publish gate
- repo-operator -- commit/push (gated on secrets)
- gh-issue-manager -- update issue board
- gh-reporter -- post summary to GitHub

### Upstream Inputs

Read from `.runs/<run-id>/build/` (if available):
- `build_receipt.json`
- `pr_creation_status.md`

Read from `.runs/<run-id>/run_meta.json`:
- `pr_number` (from pr-creator in Flow 3)
- `issue_number`
- `github_repo`

**If PR does not exist**: Call `pr-creator` to create a Draft PR first.

**If upstream artifacts are missing**: Flow 4 can start without Flows 1-3. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue.

### Artifact Outputs

| Artifact | Producer | Description |
|----------|----------|-------------|
| `flow_plan.md` | Orchestrator | Flow progress tracking |
| `pr_feedback.md` | pr-feedback-harvester | Summarized bot + human feedback |
| `pr_feedback_raw.json` | pr-feedback-harvester | Raw API responses (optional) |
| `review_worklist.md` | review-worklist-writer | Actionable items with stable markers |
| `review_worklist.json` | review-worklist-writer | Machine-readable worklist |
| `review_actions.md` | Orchestrator | Cumulative log of changes made |
| `style_sweep.md` | Orchestrator | Style sweep result (NOOP if no pending MINOR Markdown items) |
| `cleanup_report.md` | review-cleanup | Cleanup summary |
| `review_receipt.json` | review-cleanup | Machine-readable receipt |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings |
| `secrets_status.json` | secrets-sanitizer | Gate status (audit record) |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | Issue operation status |
| `github_report.md` | gh-reporter | Local copy of GitHub post |
| `gh_report_status.md` | gh-reporter | GitHub posting status |

All artifacts live under `.runs/<run-id>/review/`.

### Orchestration Outline

Flow 4 follows the 3-phase model with setup and seal bookends:

```
[Setup] → [Phase 1: Harvest & Cluster] → [Phase 2: Execute] → [Phase 3: Close] → [Seal]
```

---

#### Setup: Infrastructure

**run-prep** → **repo-operator** (branch) → **pr-creator** (if needed)

1. **Call `run-prep`** to establish `.runs/<run-id>/review/`
2. **Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"
3. **Call `pr-creator`** to ensure a Draft PR exists

After setup, you have a run directory and a PR to harvest feedback from.

---

#### Phase 1: Harvest & Cluster

**pr-feedback-harvester** → **review-worklist-writer**

**Call `pr-feedback-harvester`:**
- Grabs all available feedback (bots, humans, CI)
- Grabs partial CI failures if jobs are still running but already failing
- Doesn't wait for pending checks

**Call `review-worklist-writer`:**
- Clusters feedback into Work Items (by file/theme, not individual comments)
- 50 comments → 5-10 Work Items
- Items get stable `RW-NNN` IDs
- Markdown nits grouped into single `RW-MD-SWEEP`

**Route on worklist:** If no items, proceed to Close. Otherwise, enter Execute loop.

---

#### Phase 2: Execute (Unbounded Loop)

**The core of Flow 4: iteratively resolve Work Items.**

**This is an explicit agent call chain, not a narrative algorithm.**

**The Worklist Microloop:**

```
while pending > 0 and not exhausted:
    1. review-worklist-writer (mode: create or refresh)
       → returns: pending_blocking, stuck_signal, next_batch (IDs + route_to + batch_hint)

    2. Route next_batch to fix-lane agent:
       - TESTS → test-author
       - CORRECTNESS → code-implementer
       - STYLE → fixer
       - DOCS → doc-writer

       Agent receives: batch IDs + file paths + evidence
       Agent reports naturally: what it fixed, what was stale, what it couldn't fix

    3. review-worklist-writer (mode: apply)
       → Receives: worker's natural language response + batch_ids
       → Parses response to determine per-item status
       → Updates review_worklist.json
       → Appends to review_actions.md
       → Returns updated pending count + next_batch

    4. Periodically: Checkpoint Routine (explicit agent chain)
       a. repo-operator (stage intended changes)
       b. secrets-sanitizer (gate staged surface; capture Gate Result)
       c. repo-operator (commit/push; gated on Gate Result)
       d. pr-feedback-harvester (re-harvest CI/bot status)
       e. review-worklist-writer (mode: refresh; may add new items)
       → If stuck_signal: true → exit loop
```

**Key principle:** The orchestrator does NOT read `review_worklist.json` directly. It calls `review-worklist-writer` which reads the JSON, picks the batch, and returns routing info. After the fix-lane agent works, it calls `review-worklist-writer` in apply mode to parse the worker's response and update state.

**Handling Design Feedback (Law 7: Local Resolution):**

If a reviewer flags a fundamental design issue (not just a code fix):
1. **Call `design-optioneer`** to analyze the feedback against the current code and ADR
2. **If the analysis suggests a scoped fix:** Call `code-implementer` to apply it
3. **Verification:** Run `test-executor` to confirm no regressions
4. **Report back:** "Resolved design concern [RW-NNN] with surgical refactor; verified with tests."

**Only escalate to Flow 2** if the design feedback invalidates the entire architecture.

**Workers report naturally:** Fix-lane agents (code-implementer, fixer, test-author, doc-writer) do their job and describe what happened. They don't need special output formats. The `review-worklist-writer` parses their natural language response to update item statuses.

**Checkpoint Routine:** Sanitizer gates the **staged surface**. Always stage before scan. The re-harvest immediately captures bot feedback on the new push.

**Exit conditions:**
- `pending == 0` → complete
- Context exhausted → PARTIAL (checkpoint, rerun to continue)
- `stuck_signal: true` → PARTIAL (human may need to intervene)

**Style Sweep:** If `RW-MD-SWEEP` is pending, call `fixer` once to apply all markdown fixes in one pass.

---

#### Phase 3: Close the Loop

**pr-commenter** → **pr-status-manager**

**Call `pr-commenter`:**
- Posts resolved items checklist (closure signal)
- Shows what was fixed, skipped, or pending
- Idempotent (updates existing comment)

**Call `pr-status-manager`:**
- If review complete: flip Draft → Ready for Review
- If incomplete: keep Draft, document remaining items

---

#### Seal: Receipt + Publish

**review-cleanup** → **secrets-sanitizer** → **repo-operator** → **gh-issue-manager** → **gh-reporter**

1. **`review-cleanup`** — Write `review_receipt.json`, update index
2. **`secrets-sanitizer`** — Publish gate (returns Gate Result)
3. **`repo-operator`** — Commit/push (gated on secrets + hygiene)
4. **`gh-issue-manager`** + **`gh-reporter`** — Update issue (if allowed)

**Gate Result semantics:**
- `safe_to_commit: false` → skip commit
- `safe_to_publish: false` → commit locally, skip push
- `proceed_to_github_ops: false` → skip GitHub updates

---

#### flow_plan.md Template

```markdown
# Flow 4: Review for <run-id>

## Agents (explicit checklist)

- [ ] run-prep
- [ ] repo-operator (ensure run branch)
- [ ] pr-creator (create Draft PR if needed)
- [ ] pr-feedback-harvester
- [ ] review-worklist-writer
- [ ] worklist loop (unbounded)
- [ ] pr-commenter
- [ ] pr-status-manager
- [ ] review-cleanup
- [ ] secrets-sanitizer
- [ ] repo-operator (commit/push)
- [ ] gh-issue-manager
- [ ] gh-reporter

## Worklist Progress

| Item | Category | Severity | Status |
|------|----------|----------|--------|
| (populated by worklist loop) |

## Summary

- **Final Status**: VERIFIED | PARTIAL | UNVERIFIED
- **Worklist Items**: <resolved>/<total> resolved
- **PR State**: draft | ready
- **Next Flow**: `/flow-5-gate`
```

**Important:** Do NOT use phase checkboxes ("Setup", "Harvest & Cluster", etc.). Use the explicit agent checklist above. Phases are explanatory prose, not TodoWrite items.

### Status States

Agents report one of:
- **VERIFIED**: All critical items resolved, review complete.
- **UNVERIFIED**: Items still pending or incomplete feedback.
- **CANNOT_PROCEED**: IO/permissions/tool failure only.

### Review Completion Criteria

Flow 4 is VERIFIED when:
- All CRITICAL worklist items are resolved
- All MAJOR worklist items are resolved (or explicitly deferred with reason)
- CI checks are passing
- No blocking review requests

MINOR and INFO items may remain pending without blocking.

---

### Orchestrator Kickoff

#### Station Order (5 groups)

```
SETUP          run-prep → repo-operator (branch) → pr-creator
HARVEST        pr-feedback-harvester → review-worklist-writer
EXECUTE        worklist loop (unbounded)
CLOSE          pr-commenter → pr-status-manager
SEAL           review-cleanup → secrets-sanitizer → repo-operator → gh-issue-manager → gh-reporter
```

#### Execute Loop (Detailed)

**Entry:** `review_worklist.json` exists with pending items

**This is an explicit agent call chain. The orchestrator routes on returned fields, not by parsing JSON.**

**Loop:**
```
1) Call review-worklist-writer (mode: refresh)
   → Returns: pending_blocking, stuck_signal, next_batch

2) If pending_blocking == 0: exit (complete)
3) If context exhausted: checkpoint and exit (PARTIAL)
4) If stuck_signal: true: checkpoint and exit (PARTIAL)

5) Style Sweep (if next_batch contains `RW-MD-SWEEP`):
   - Call fixer once for all markdown fixes
   - fixer reports naturally what it fixed

6) Route next_batch to fix-lane agent:
   - TESTS → test-author
   - CORRECTNESS → code-implementer
   - STYLE → fixer
   - DOCS → doc-writer

   Agent behavior:
   - Receives: batch IDs + file paths + evidence
   - Reports naturally: what it fixed, what was stale, what needs escalation

7) Call review-worklist-writer (mode: apply)
   → Receives: worker's natural language response + batch_ids
   → Parses response to determine per-item status
   → Updates review_worklist.json
   → Appends to review_actions.md (agent handles this, not orchestrator)
   → Returns: updated pending count, next_batch

8) Periodically: Checkpoint Routine (explicit agent chain)
   a) repo-operator (stage intended changes)
   b) secrets-sanitizer (gate staged surface)
   c) repo-operator (commit/push; gated on Gate Result)
   d) pr-feedback-harvester (re-harvest)
   e) review-worklist-writer (mode: refresh; may add new items)
   → If stuck_signal: true → exit loop
```

**Checkpoint Routine:** Sanitizer gates the **staged surface**. Stage first, then scan. Every push must be gated.

**Exit conditions:**
- `pending_blocking == 0` (all resolved) → VERIFIED
- Context exhausted → PARTIAL
- `stuck_signal: true` → PARTIAL
- Unrecoverable blocker → UNVERIFIED

#### TodoWrite (copy exactly)

**These are the agents you call, in order. Do not group. Do not summarize. Execute each line.**

```
- [ ] run-prep
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] pr-creator (create Draft PR if needed)
- [ ] pr-feedback-harvester
- [ ] review-worklist-writer
- [ ] worklist loop (unbounded: resolve items until completion/context/unrecoverable)
- [ ] pr-commenter (post/update PR summary comment)
- [ ] pr-status-manager (flip Draft to Ready if review complete)
- [ ] review-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (commit/push; return Repo Operator Result)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth)
```

**Why explicit?** The orchestrator (you) executes what's in the list. Grouped phases get skipped. Explicit agents get called.

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.


---

## flow-5-gate.md

---
description: Run Flow 5 (Code -> Artifact): verify receipts, contracts, security, policies; decide merge vs bounce; execute bounded fix-forward lane when eligible.
---

## Flow 5: Code -> Artifact (Gate)

You are orchestrating Flow 5 of the SDLC swarm.

### Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/gate/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/gate/` exists.

##### Artifact visibility rule

* Do **not** attempt to prove files exist under `.runs/<run-id>/` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on verification agents (e.g., `receipt-checker`) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

### Your Goals

- Verify build receipts exist and are complete
- Check API/schema contracts
- Scan security and coverage
- Enforce policies
- Decide: MERGE / BOUNCE (with reason for human-review vs fix-required)
- **Runner-bounded fix-forward lane** for deterministic mechanical drift (fmt/import/whitespace/docs) when `gate-fixer` says it is safe and resealable

### Role Clarification: Final Verification, Not Primary Detection

Flow 5 Gate is the **last line of defense**, not the first.

**Primary detection happens earlier:**
- Flow 3 (Build): Critics catch issues per-AC, standards-enforcer catches reward hacking
- Flow 4 (Review): Worklist drains all feedback items, stale check prevents wasted work

**Gate's job is to VERIFY**, not DISCOVER:
- Verify that receipts from earlier flows are complete and consistent
- Verify that policy compliance was checked (not run the checks from scratch)
- Verify that security findings were addressed (not scan for new ones)
- Make the merge decision based on accumulated evidence

**If Gate is catching issues that should have been caught earlier:**
- That's a signal that earlier flows need improvement
- Document the gap in `observations[]` for Flow 7 (Wisdom)
- Fix-forward only for mechanical drift (formatting, imports)
- BOUNCE for semantic issues (they should have been caught in Build/Review)

**Anti-pattern:** Running full security scans, coverage checks, and lint sweeps in Gate that duplicate earlier flows. Gate should READ results, not RE-RUN analysis.

### Before You Begin (Required)

#### Two State Machines

Flow 5 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

#### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.
   - Parallel checks (contracts/security/coverage) are ONE todo.

2. Mirror the same list into `.runs/<run-id>/gate/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

#### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- receipt-checker (verify receipts first; route on Result)
- contract-enforcer / security-scanner / coverage-enforcer (parallel checks)
- gate-fixer (mechanical issues report; emits FIX_FORWARD_PLAN_V1)
- fix-forward-runner (if eligible; execute `FIX_FORWARD_PLAN_V1`; confirm via `receipt-checker` + `gate-fixer`)
- traceability-auditor (traceability audit)
- risk-analyst (risk assessment)
- policy-analyst (policy compliance)
- merge-decider (merge decision)
- gate-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report gate verdict; gated)
```

#### On Rerun

If running `/flow-5-gate` on an existing run-id:
- Read `.runs/<run-id>/gate/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

If you encounter missing receipts or unclear state, **document it and continue with available information**. Gate agents should note gaps in their reports rather than blocking.

### Subagents to use

**Infrastructure (Step 0)**:
- run-prep (establish run directory)

Domain agents (Flow 5 specific):
- receipt-checker
- contract-enforcer
- security-scanner
- coverage-enforcer
- gate-fixer (reports mechanical issues; no repo mutations)
- fix-forward-runner (executes FIX_FORWARD_PLAN_V1; no git side effects)
- build-cleanup (reseal Build receipt after runner changes code)
- merge-decider

Cross-cutting agents:
- risk-analyst
- policy-analyst
- traceability-auditor (run-level coherence + spec traceability before merge decision)

Cleanup + Reporting (End of Flow):
- gate-cleanup -- writes gate_receipt.json, updates index.json status
- secrets-sanitizer -- publish gate
- repo-operator -- checkpoint commit (gated on secrets-sanitizer result); writes git_status.md if anomaly
- gh-issue-manager -- updates issue body status board
- gh-reporter -- posts gate verdict to issue

### Upstream Inputs

Read from `.runs/<run-id>/build/` (if available):
- build receipt and supporting critiques (tests, code, self-review)
- `build/ac_status.json` (AC completion tracker)

Read from `.runs/<run-id>/review/` (if available):
- `review_receipt.json` (for bounce-to-review evidence)
- Check `worklist_status.has_critical_pending` and `counts.worklist_pending` for unresolved items

If these files are not visible locally but may exist in committed state, do **not** block Gate. Proceed and let `receipt-checker` pull evidence from the committed snapshot; workspace visibility alone is not a missing-artifact signal.

**If upstream artifacts are missing**: Flow 5 can start without Flows 1-3. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue. This enables flexibility for gate-only checks.

### Artifact Outputs

| Artifact | Producer | Description |
|----------|----------|-------------|
| `flow_plan.md` | Orchestrator | Flow progress tracking |
| `receipt_audit.md` | receipt-checker | Build receipt verification |
| `contract_compliance.md` | contract-enforcer | API contract check results |
| `security_scan.md` | security-scanner | Security scan findings |
| `coverage_audit.md` | coverage-enforcer | Coverage threshold check |
| `gate_fix_summary.md` | gate-fixer | Mechanical issues report (no fixes) + fix-forward plan |
| `fix_forward_report.md` | fix-forward-runner (conditional) | Runner execution log: commands run, scope check, files touched, reseal guidance |
| `traceability_audit.md` | traceability-auditor | Run-level coherence + spec traceability (REQ<->BDD) across receipts/index/GitHub markers |
| `risk_assessment.md` | risk-analyst | Risk analysis |
| `policy_analysis.md` | policy-analyst | Policy compliance check |
| `merge_decision.md` | merge-decider | MERGE / BOUNCE decision (with reason) |
| `cleanup_report.md` | gate-cleanup | Cleanup summary |
| `gate_receipt.json` | gate-cleanup | Machine-readable receipt |
| `secrets_scan.md` | secrets-sanitizer | Secrets scan findings |
| `secrets_status.json` | secrets-sanitizer | Gate status (audit record) |
| `git_status.md` | repo-operator | Anomaly documentation (if detected) |
| `gh_issue_status.md` | gh-issue-manager | Issue operation status |
| `github_report.md` | gh-reporter | Local copy of GitHub post |
| `gh_report_status.md` | gh-reporter | GitHub posting status |

All artifacts live under `.runs/<run-id>/gate/`.

**Fix-forward contract:** `gate_fix_summary.md` must contain the `## Fix-forward Plan (machine readable)` block (`PACK-CONTRACT: FIX_FORWARD_PLAN_V1`). `fix_forward_report.md` records what the runner actually executed (commands, scope check, files touched, reseal guidance).

### Orchestration outline

#### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/gate/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "gate" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/gate/`.

#### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

**Do not** read `.runs/` artifacts before run-prep. After run-prep, call `receipt-checker` first and route on its Result block before running contracts/security/coverage.

#### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/gate/flow_plan.md`:

```markdown
# Flow 5: Gate for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] receipt-checker (verify receipts first; route on Result)
- [ ] contract-enforcer / security-scanner / coverage-enforcer (parallel)
- [ ] gate-fixer (mechanical issues report)
- [ ] fix-forward-runner (if eligible; execute `FIX_FORWARD_PLAN_V1`; confirm via `receipt-checker` + `gate-fixer`)
- [ ] traceability-auditor (run-level coherence)
- [ ] risk-analyst (risk assessment)
- [ ] policy-analyst (policy compliance)
- [ ] merge-decider (decide: MERGE/BOUNCE + reason)
- [ ] gate-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

#### Step 2: Verify receipts
- `receipt-checker` -> `.runs/<run-id>/gate/receipt_audit.md`
- Run this before contracts/security/coverage; route on its Result block.
- **Evidence audit, not re-execution:** The receipt-checker verifies that earlier flows produced complete artifacts with passing gates. It does NOT re-run tests or re-scan for secrets—it reads the receipts.
- **Receipts are logs, not locks:** The git log is the audit trail. If code was modified after a receipt was written (ad-hoc fixes, fix-forward), the receipt is still valid as historical evidence of what happened at that station. Don't BOUNCE just because `evidence_sha != HEAD`.
- **AC completion check:** Receipt-checker should verify `build_receipt.json.counts.ac_completed == build_receipt.json.counts.ac_total`. If either is null/missing, treat as UNVERIFIED with blocker. If not equal, BOUNCE to Flow 3 with blocker: "AC loop incomplete: {ac_completed}/{ac_total}".

#### Step 3: Check contracts (can run in parallel with security/coverage)
- `contract-enforcer` -> `.runs/<run-id>/gate/contract_compliance.md`

#### Step 4: Security scan (can run in parallel with contracts/coverage)
- `security-scanner` -> `.runs/<run-id>/gate/security_scan.md`

#### Step 5: Coverage (can run in parallel with contracts/security)
- `coverage-enforcer` -> `.runs/<run-id>/gate/coverage_audit.md`

#### Step 6: Mechanical issues report
- `gate-fixer` -> `.runs/<run-id>/gate/gate_fix_summary.md` (recommendations only; **no repo mutations in Gate**)
- Identifies lint, format, and doc issues that would be fixed in Build

#### Step 7: Fix-forward lane (conditional; runner-bounded)
Treat fix-forward as a **subroutine station**, not a per-call checklist.

- Entry condition: `fix_forward_eligible: true` (from `gate-fixer` / `gate_fix_summary.md`)
- Apply Fix-forward Subroutine Template with:
  - producer = `gate-fixer` (emits `FIX_FORWARD_PLAN_V1`)
  - fix lane = `fix-forward-runner` (executes `apply_steps`/`verify_steps`; no git side effects)
  - confirm = rerun `receipt-checker`, then rerun `gate-fixer` once

If the runner reports `changes_detected: true`, update build receipt + stage + secrets gate + commit/push the runner-touched scope, then run the confirm pass.

If the runner reports UNVERIFIED or scope violation, proceed with remaining Gate stations; `merge-decider` should BOUNCE to Flow 3 with the runner report as evidence.

#### Step 8: Traceability audit
- `traceability-auditor` -> `.runs/<run-id>/gate/traceability_audit.md`
- Run after fix-forward reruns so receipts/index are current.

#### Step 9: Risk assessment
- `risk-analyst` -> `.runs/<run-id>/gate/risk_assessment.md`

#### Step 10: Policy compliance
- `policy-analyst` -> `.runs/<run-id>/gate/policy_analysis.md`

#### Step 11: Merge decision
- `merge-decider` -> `.runs/<run-id>/gate/merge_decision.md` (MERGE/BOUNCE with reason)

#### Step 12: Finalize and Write Receipt
- `gate-cleanup` -> `.runs/<run-id>/gate/gate_receipt.json`, `.runs/<run-id>/gate/cleanup_report.md`
- Verifies all required artifacts exist
- Computes counts mechanically (never estimates)
- Updates `.runs/index.json` with status, last_flow, updated_at

#### Step 13: Sanitize Secrets (Publish Gate)
- `secrets-sanitizer` -> `.runs/<run-id>/gate/secrets_scan.md`, `.runs/<run-id>/gate/secrets_status.json`
- Scans .runs/ artifacts before GitHub posting
- Returns a **Gate Result** block (control plane; file is audit-only)

<!-- PACK-CONTRACT: GATE_RESULT_V3 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
findings_count: <int>
blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
blocker_reason: <string | null>
```
<!-- PACK-CONTRACT: GATE_RESULT_V3 END -->

**Gating logic (boolean gate — the sanitizer says yes/no, orchestrator decides next steps):**
- The sanitizer is a fix-first pre-commit hook, not a router
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 13c)
- If `safe_to_commit: false`:
  - `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO failure)
  - `blocker_kind: SECRET_IN_CODE` → route to `fixer` (orchestrator decides)
  - `blocker_kind: SECRET_IN_ARTIFACT` → investigate manually
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`
- GitHub reporting ops still run in RESTRICTED mode when publish is blocked or `publish_surface: NOT_PUSHED`

#### Step 13b: Checkpoint Commit

- `repo-operator` -> `.runs/<run-id>/gate/git_status.md` (if anomaly detected)

Checkpoint the audit trail **before** any GitHub operations.

**Allowlist for Flow 5:**
- `.runs/<run-id>/gate/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`

**Call `repo-operator`** with `checkpoint_mode: normal` (default). The agent:
1. Resets staging and stages only the allowlist (not `git add .`)
2. Enforces the allowlist/anomaly interlock mechanically
3. Writes `.runs/<run-id>/gate/git_status.md` if anomaly detected
4. Handles no-op (nothing staged) gracefully—no empty commits

**Control plane:** `repo-operator` returns a Repo Operator Result block:
```
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```
**Note:** `commit_sha` is always populated (current HEAD on no-op), never null.

Orchestrators route on this block, not by re-reading `git_status.md`.

**Safe-bail enforcement:** If this checkpoint was invoked due to safe-bail (Step 13b), `repo-operator` must set `proceed_to_github_ops: false` even if `safe_to_publish: true`.

**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
- If `safe_to_commit: false` (from Gate Result): `repo-operator` skips commit entirely
- If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`
- If `safe_to_publish: true` and no anomaly: `repo-operator` commits and pushes, returns `proceed_to_github_ops: true`
- If `safe_to_publish: false`:
  - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`; flow ends UNVERIFIED
  - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
  - Otherwise → UNVERIFIED; skip push (`publish_surface: NOT_PUSHED`). Continue with GitHub Reporting Ops in RESTRICTED mode when access allows.

#### Step 14-15: GitHub Reporting

**Call `gh-issue-manager`** then **`gh-reporter`** to update the issue.

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

#### Step 16: Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Merge Decision**: MERGE | BOUNCE (reason: NEEDS_HUMAN_REVIEW | FIX_REQUIRED | POLICY_BLOCK | OTHER)
- **Blockers**: <list if any>
- **Next Flow**: `/flow-6-deploy` (if MERGE) or bounce target

## Human Review Checklist

Before proceeding:
- [ ] `.runs/<run-id>/gate/merge_decision.md` - Is the decision correct?
- [ ] `.runs/<run-id>/gate/security_scan.md` - Are security findings acceptable?
- [ ] `.runs/<run-id>/gate/policy_analysis.md` - Are policy concerns addressed?
```

### Bounce Semantics

Gate-fixer **remains report-only**. It emits the fix-forward plan; the **fix-forward lane** applies deterministic hygiene once (fmt/import order/docs) and reseals before merge-decision. Formatting/import-only drift should be fixed-forward when `fix_forward_eligible: true`; bounce only if ineligible or the fix-forward attempt failed.

**BOUNCE to Review (Flow 4)**:
- Unaddressed PR feedback (CodeRabbit, CI issues, review comments)
- Review worklist items still pending

**Evidence-based check:** If `review_receipt.json` exists, Gate should read:
- `review_receipt.json.worklist_status.has_critical_pending` — if true, BOUNCE to Flow 4
- `review_receipt.json.counts.worklist_pending` — if > 0 and items are CRITICAL/MAJOR, BOUNCE to Flow 4
- `review_receipt.json.worklist_status.review_complete` — if false, consider BOUNCE to Flow 4

**BOUNCE to Build (Flow 3)**:
- Logic errors
- Test failures
- API contract violations
- Security vulnerabilities
- Coverage below threshold
- AC loop incomplete (`build_receipt.json.counts.ac_completed < build_receipt.json.counts.ac_total`, or either is null)
- Mechanical drift that is **not** eligible for fix-forward or failed within the runner-bounded lane

**BOUNCE to Plan (Flow 2)**:
- Design flaws
- Architecture issues
- Missing requirements

### Status States

Agents set status in their output artifacts:

- **VERIFIED**: `blockers` empty, `missing_required` empty, and check passed. Set `recommended_action: PROCEED`.
- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR check has concerns. Set `recommended_action: RERUN | BOUNCE` depending on fix location.
- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.

**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.

`merge-decider` synthesizes all statuses into a merge decision.

### Merge Decision States

`merge-decider` outputs one of:

- **MERGE**: All checks pass or concerns are acceptable; ready to deploy.
- **BOUNCE**: Issues found **or** human judgment is required. Include a `reason` field (e.g., `NEEDS_HUMAN_REVIEW`, `FIX_REQUIRED`, `POLICY_BLOCK`, `UNKNOWN_UPSTREAM`) and the target flow/agent when action is known.

Human-review-only cases use `reason: NEEDS_HUMAN_REVIEW` instead of a separate human-only verdict.

---

### Orchestrator Kickoff

#### Station order + templates

##### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `receipt-checker`

4. `contract-enforcer` / `security-scanner` / `coverage-enforcer` (parallel)

5. `gate-fixer` (report + fix-forward plan)

6. `fix-forward-runner` (if eligible; execute `FIX_FORWARD_PLAN_V1`; confirm via rerun `receipt-checker` + `gate-fixer`)

7. `traceability-auditor`

8. `risk-analyst`

9. `policy-analyst`

10. `merge-decider`

11. `gate-cleanup`

12. `secrets-sanitizer`

13. `repo-operator` (checkpoint; read Repo Operator Result)

14. `gh-issue-manager` (if allowed)

15. `gh-reporter` (if allowed)

##### Fix-forward Subroutine Template (plan -> execute -> confirm)

Do not treat fix-forward as "run runner, rerun runner". It is a bounded subroutine:

1) Plan: run `gate-fixer` to emit `FIX_FORWARD_PLAN_V1` (report-only)
2) Execute: if eligible, run `fix-forward-runner` to execute the plan (runner-bounded; no git side effects)
3) If changes were made, update Build receipt (build-cleanup) and run secrets-sanitizer (rescan the new staged surface)
4) Confirm: rerun `receipt-checker`, then rerun `gate-fixer` once

##### Worklist Loop Template (producer → fix lane → confirm)

1) Run the producer (`mutation-auditor` / `fuzz-triager` / `flakiness-detector`)
2) If it returns `recommended_action: RERUN` or a worklist that routes to an agent:
   - call the routed agent once (`test-author` / `code-implementer` / `fixer`)
3) Confirm once: rerun the producer one time to verify the top items moved.
4) If still UNVERIFIED, proceed with blockers unless the producer says another pass will help and the fix lane can actually address it.

#### TodoWrite (copy exactly)
- [ ] run-prep
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] receipt-checker
- [ ] contract-enforcer / security-scanner / coverage-enforcer (parallel)
- [ ] gate-fixer (report + fix-forward plan)
- [ ] fix-forward-runner (if eligible; execute `FIX_FORWARD_PLAN_V1`; confirm via rerun `receipt-checker` + `gate-fixer`)
- [ ] traceability-auditor
- [ ] risk-analyst
- [ ] policy-analyst
- [ ] merge-decider
- [ ] gate-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint; allowlist interlock + no-op handling)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)


Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

---

## flow-6-deploy.md

---
description: Run Flow 6 (Artifact -> Prod): execute GitHub-native deployment, monitor CI, verify, create audit trail.
---

## Flow 6: Artifact -> Prod (Deploy)

You are orchestrating Flow 6 of the SDLC swarm.

### Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/deploy/`
- Do **not** rely on `cd` into any folder to make relative paths work.

**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/deploy/` exists.

##### Artifact visibility rule

* Do **not** attempt to “prove files exist” under `.runs/<run-id>/…` **before** `signal-run-prep` / `run-prep`.
* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flow’s verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.
* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.

### Your Goals

Execute the merge of the feature branch into swarm mainline (`origin/main`). Handle simple rebases if needed. Verify health. Create audit trail.

**Flow 6 is always callable.** Its behavior depends on Gate's decision:
- If Gate said MERGE: sanity check → handle rebase if needed → merge → verify → report.
- If Gate said BOUNCE (including NEEDS_HUMAN_REVIEW): don't merge, write receipts explaining why.

**Scope:** Flow 6 merges the run's feature branch into swarm `origin/main`. It does NOT merge into upstream. Upstream integration is a separate, post-Wisdom concern.

### Before You Begin (Required)

#### Two State Machines

Flow 6 uses **two complementary state machines**:

1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)
2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)

#### Setup Steps

1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.
   - Track at the behavioral/station level, NOT per agent call.

2. Mirror the same list into `.runs/<run-id>/deploy/flow_plan.md` as checkboxes.
   - As each station completes: mark TodoWrite done AND tick the checkbox.

#### Suggested TodoWrite Items

```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- deploy-decider (EARLY: read gate_verdict for routing)
- repo-operator (merge + tag + release; only if gate_verdict == MERGE)
- deploy-monitor (monitor CI; only if gate_verdict == MERGE)
- smoke-verifier (smoke tests; only if gate_verdict == MERGE)
- deploy-decider (FINAL: synthesize verification into deployment decision)
- deploy-cleanup (finalize receipt)
- secrets-sanitizer (publish gate)
- repo-operator (checkpoint commit)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report deployment status; gated)
```

#### On Rerun

If running `/flow-6-deploy` on an existing run-id:
- Read `.runs/<run-id>/deploy/flow_plan.md`
- Create TodoWrite from the checklist
- Pre-mark items done if artifacts exist and look current
- Run remaining stations to refine

This flow uses **git and GitHub** (via `gh` CLI). No external deployment platform required.

**For production extensions** (k8s, canary, metrics): extend this flow with your deployment platform.

### Agents to Use

| Agent | Responsibility |
|-------|----------------|
| **run-prep** | MUST be called first to establish the run directory and `.runs/<run-id>/deploy/` |
| repo-operator | Merge PR, create git tag/release (only if Gate approved MERGE) |
| deploy-monitor | Watch CI and deployment events, write verification report |
| smoke-verifier | Health checks, artifact verification, append to verification report |
| deploy-decider | Synthesize verification into deployment decision |
| **deploy-cleanup** | Write deploy receipt, update index.json status |
| **secrets-sanitizer** | Publish gate before GitHub posting |
| **gh-issue-manager** | Update issue body status board |
| **gh-reporter** | Post deployment summary to issue |

### Upstream Inputs

Read from `.runs/<run-id>/gate/` (if available):
- `merge_decision.md`
- `gate_receipt.json`

**If upstream artifacts are missing**: Flow 6 can start without Flow 4. Proceed best-effort: document assumptions, set status to UNVERIFIED, and continue.

### Orchestration Outline

#### Step 0: Establish Run Infrastructure

**Call `run-prep` first.**

This agent will:
- Derive or confirm the `<run-id>` from context, branch name, or user input
- Create `.runs/<run-id>/deploy/` directory structure
- Update `.runs/<run-id>/run_meta.json` with "deploy" in `flows_started`
- Update `.runs/index.json`

After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/deploy/`.

#### Step 0b: Ensure Run Branch

**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"

The agent handles branch creation/switching safely. This keeps checkpoint commits off main.

#### Step 1: Initialize Flow Plan

Create or update `.runs/<run-id>/deploy/flow_plan.md`:

```markdown
# Flow 6: Deploy for <run-id>

## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch `run/<run-id>`)
- [ ] deploy-decider (EARLY: read gate_verdict for routing)
- [ ] repo-operator (merge + tag + release; only if gate_verdict == MERGE)
- [ ] deploy-monitor (only if gate_verdict == MERGE)
- [ ] smoke-verifier (only if gate_verdict == MERGE)
- [ ] deploy-decider (FINAL: synthesize verification into deployment decision)
- [ ] deploy-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

<Update as each step completes>
```

#### Step 2: Determine Gate Decision (delegated to deploy-decider)

**Do NOT parse `merge_decision.md` in the orchestrator.** The gate decision is read by `deploy-decider`.

**Early call to deploy-decider:**
Call `deploy-decider` BEFORE any merge operations. This agent:
- Reads `.runs/<run-id>/gate/merge_decision.md`
- Parses the `verdict:` field from `## Machine Summary`
- Returns a Result block with `gate_verdict: MERGE | BOUNCE | null`

**Orchestrator routing (pure routing, no parsing):**
- If `gate_verdict: MERGE`: proceed to Path A (merge + verify)
- If `gate_verdict: BOUNCE` or `null`: proceed to Path B (NOT_DEPLOYED)

**Why delegated?** The orchestrator should not parse files. The `deploy-decider` agent already needs to read the gate decision for its governance checks — it can return the verdict for routing.

#### Path A: Gate Decision = MERGE

**Two operation types in Flow 6:**

| Category | Operations | Gating |
|----------|------------|--------|
| **Release Ops** | Merge PR, create tag, create release | Gate decision = MERGE + repo-operator mechanics |
| **Reporting Ops** | `gh-issue-manager`, `gh-reporter` | Two-gate prerequisites (secrets + repo hygiene) |

Release Ops execute only when Gate's `merge_decision.md` says MERGE. Reporting Ops use the same two-gate system as all other flows.

**Note on secrets governance:** The code being merged was already sanitized in Build (Flow 3). Deploy's secrets-sanitizer is for `.runs/deploy/` artifacts and GitHub posting—not code merge governance.

0. **Pre-merge Sanity Check + Rebase** (repo-operator)
   - Check if `origin/main` has diverged from the feature branch
   - If diverged: attempt automatic rebase
   - If rebase conflicts: write `deployment_log.md` noting conflict, set NOT_DEPLOYED with blockers
   - Simple conflicts (different files) are resolved automatically
   - This mirrors how developers actually work: rebase, then merge

1. **Merge & Tag** (repo-operator) - **Release Op**
   - **Prerequisite:** Gate decision = MERGE, rebase clean (if needed)
   - Execute `gh pr merge`, create git tag + GitHub release
   - Write `.runs/<run-id>/deploy/deployment_log.md` with merge details
   - **If `gh` CLI not authenticated or PR not found:** Write `deployment_log.md` noting failure, set status NOT_DEPLOYED, `recommended_action: RERUN`. Do not silently skip—this is a failed release op.

2. **Monitor CI** (deploy-monitor)
   - Watch GitHub Actions status on main branch
   - Write `.runs/<run-id>/deploy/verification_report.md` with CI status

3. **Smoke Tests** (smoke-verifier)
   - If URL available, curl health endpoints; else verify artifacts
   - Append results to `.runs/<run-id>/deploy/verification_report.md`

4. **Decide** (deploy-decider)
   - Synthesize CI + smoke results
   - Write `.runs/<run-id>/deploy/deployment_decision.md` with `verdict`:
     - STABLE: Merge succeeded, CI passing, smoke checks green
     - NOT_DEPLOYED: Merge failed, CI failing, or verification issues
     - BLOCKED_BY_GATE: Gate verdict was not MERGE

5. **Finalize Receipt** (deploy-cleanup)
   - Write `.runs/<run-id>/deploy/deploy_receipt.json`, `.runs/<run-id>/deploy/cleanup_report.md`
   - Update `.runs/index.json` with status, last_flow, updated_at

6. **Sanitize Secrets** (secrets-sanitizer)
   - Scan artifacts before GitHub posting
   - Write `.runs/<run-id>/deploy/secrets_scan.md`, `.runs/<run-id>/deploy/secrets_status.json`
   - **Returns a Gate Result block** for orchestrator routing (control plane)
   - **Status vs flags:** `status` is descriptive (CLEAN/FIXED/BLOCKED); `safe_to_commit`/`safe_to_publish` are authoritative permissions; `blocker_kind` explains why blocked
   - The JSON file is an audit record; orchestrator routes on the Gate Result block, not by re-reading the file

   **Gate Result block (returned by secrets-sanitizer):**

   <!-- PACK-CONTRACT: GATE_RESULT_V3 START -->
   ```yaml
   ## Gate Result
   status: CLEAN | FIXED | BLOCKED
   safe_to_commit: true | false
   safe_to_publish: true | false
   modified_files: true | false
   findings_count: <int>
   blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
   blocker_reason: <string | null>
   ```
   <!-- PACK-CONTRACT: GATE_RESULT_V3 END -->

6b. **Checkpoint Commit** (repo-operator)

   Checkpoint the audit trail **before** any GitHub operations.

   **Allowlist for Flow 6:**
   - `.runs/<run-id>/deploy/`
   - `.runs/<run-id>/run_meta.json`
   - `.runs/index.json`

   **Call `repo-operator`** with `checkpoint_mode: normal` (default). The agent:
   1. Resets staging and stages only the allowlist (not `git add .`)
   2. Enforces the allowlist/anomaly interlock mechanically
   3. Writes `.runs/<run-id>/deploy/git_status.md` if anomaly detected
   4. Handles no-op (nothing staged) gracefully—no empty commits

   **Control plane:** `repo-operator` returns a Repo Operator Result block:
   ```
## Repo Operator Result
operation: checkpoint
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_paths: []
```
   **Note:** `commit_sha` is always populated (current HEAD on no-op), never null.

   Orchestrators route on this block, not by re-reading `git_status.md`.

   **Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**
   - If `safe_to_commit: false` (from Gate Result): `repo-operator` skips commit entirely
   - If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`
   - If `safe_to_publish: true` and no anomaly: `repo-operator` commits and pushes, returns `proceed_to_github_ops: true`
   - If `safe_to_publish: false`:
     - If `needs_upstream_fix: true` → **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`; flow ends UNVERIFIED
     - If `status: BLOCKED_PUBLISH` → **CANNOT_PROCEED** (mechanical failure); stop and require human intervention
     - Otherwise → UNVERIFIED; commit locally but skip push; returns `proceed_to_github_ops: false` and `publish_surface: NOT_PUSHED`.

7. **GitHub Reporting** (gh-issue-manager → gh-reporter) - **Reporting Ops**

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR
- Reporting Ops are distinct from Release Ops (merge/tag) above

#### Path B: Gate Decision = BOUNCE (including human-review reasons)

1. **Skip Merge** (no repo-operator merge)
   - Write `.runs/<run-id>/deploy/deployment_log.md` noting: "No merge performed; Gate decision = <verdict>"

2. **Minimal Monitoring** (deploy-monitor)
   - Write `.runs/<run-id>/deploy/verification_report.md` noting: "No deployment to verify; Gate decision = <verdict>"
   - Status: NOT_DEPLOYED

3. **Decision** (deploy-decider)
   - Write `.runs/<run-id>/deploy/deployment_decision.md` with:
     - Verdict: NOT_DEPLOYED
     - Explanation of why deployment did not occur
     - Reference to Gate's concerns

4. **Finalize Receipt** (deploy-cleanup)
   - Write receipt with NOT_DEPLOYED status
   - Update index.json

5. **Sanitize + Checkpoint + Report** (secrets-sanitizer → repo-operator → gh-issue-manager → gh-reporter)
   - Same as Path A (steps 6, 6b, 6c, 7, 8)

### Output Artifacts

| Artifact | Description |
|----------|-------------|
| `flow_plan.md` | Execution plan and progress |
| `deployment_log.md` | Record of merge, tag, release actions (or why skipped) |
| `verification_report.md` | CI status + smoke check results |
| `deployment_decision.md` | Final verdict: STABLE / NOT_DEPLOYED / BLOCKED_BY_GATE |
| `deploy_receipt.json` | Receipt for downstream |
| `cleanup_report.md` | Cleanup status and evidence |
| `secrets_scan.md` | Secrets scan report |
| `secrets_status.json` | Publish gate status (audit record) |
| `git_status.md` | Repository status and anomaly documentation (if anomaly detected) |
| `gh_issue_status.md` | Issue board update status |
| `gh_report_status.md` | Log of GitHub posting |
| `github_report.md` | Report content (local copy) |

### deploy-decider Verdicts

| `Verdict` | Meaning |
|---------|---------|
| STABLE | Merge succeeded and CI/smoke verification passes |
| NOT_DEPLOYED | Merge failed, or CI failing, or smoke tests indicate issues |
| BLOCKED_BY_GATE | Gate verdict was not MERGE; no deployment attempted |

**Note:** We trust the GitHub merge flow handles branch protection. Flow 6 doesn't re-verify governance—it executes the merge and verifies the result.

#### Finalize Flow

Update `flow_plan.md`:
- Mark all steps as complete
- Add final summary section:

```markdown
## Summary

- **Final Status**: VERIFIED | UNVERIFIED
- **Deployment Verdict**: STABLE | NOT_DEPLOYED | BLOCKED_BY_GATE
- **Next Flow**: `/flow-7-wisdom` (post-deployment analysis)

- [ ] `.runs/<run-id>/deploy/deployment_decision.md` - Is the verdict correct?
- [ ] `.runs/<run-id>/deploy/verification_report.md` - Are checks passing?
- [ ] If NOT_DEPLOYED/BLOCKED_BY_GATE - What action is needed?
```

### Completion

Flow 6 is complete when:
- `deployment_log.md` exists (even if minimal for BOUNCE (including NEEDS_HUMAN_REVIEW))
- `verification_report.md` exists
- `deployment_decision.md` exists with valid verdict

Human gate at end: "Did deployment succeed?" (or "Why didn't we deploy?")

---

### Orchestrator Kickoff

#### Station order

##### Station order

- `run-prep`
- `repo-operator` (ensure run branch)
- `deploy-decider` (EARLY CALL: read gate verdict for routing; returns `gate_verdict` in Result block)
- **Route on `gate_verdict`:**
  - If `MERGE`: proceed with merge operations
  - If `BOUNCE` or `null`: skip to deploy-cleanup (Path B)
- `repo-operator` (merge + tag + release; only if gate_verdict == MERGE)
- `deploy-monitor` (only if gate_verdict == MERGE)
- `smoke-verifier` (only if gate_verdict == MERGE)
- `deploy-decider` (FINAL CALL: synthesize verification into deployment decision)
- `deploy-cleanup`
- `secrets-sanitizer`
- `repo-operator` (checkpoint; read Repo Operator Result)
- `gh-issue-manager` (if allowed)
- `gh-reporter` (if allowed)

#### TodoWrite (copy exactly)

```
- [ ] run-prep
- [ ] repo-operator (ensure run/<run-id> branch)
- [ ] deploy-decider (EARLY: read gate_verdict for routing)
- [ ] repo-operator (merge + tag + release; only if gate_verdict == MERGE)
- [ ] deploy-monitor (only if gate_verdict == MERGE)
- [ ] smoke-verifier (only if gate_verdict == MERGE)
- [ ] deploy-decider (FINAL: synthesize verification into deployment decision)
- [ ] deploy-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint commit; allowlist interlock + no-op handling)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
```

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

---

## flow-7-wisdom.md

---

description: Run Flow 7 (Prod -> Wisdom): analyze artifacts, detect regressions, extract learnings, close feedback loops.

---



## Flow 7: Prod -> Wisdom



You are orchestrating Flow 7 of the SDLC swarm.



### Working Directory + Paths (Invariant)



- All commands run from **repo root**.

- All paths in this doc are **repo-root-relative**.

- Run artifacts live under: `.runs/<run-id>/`

- Flow artifacts live under: `.runs/<run-id>/wisdom/`

- Do **not** rely on `cd` into any folder to make relative paths work.



**Important**: Step 0 (run-prep) establishes the run directory and ensures `.runs/<run-id>/wisdom/` exists.



##### Artifact visibility rule



* Do **not** attempt to prove files exist under `.runs/<run-id>/` **before** `signal-run-prep` / `run-prep`.

* If `.runs/` is not directly readable in the current tool context, **do not conclude artifacts are missing**. Proceed with the flow and rely on the flows verification agents (e.g., `receipt-checker` in Gate) to obtain evidence from committed state when necessary.

* Preflight in flow docs is **policy**, not mechanics. Mechanics live in agents.



### Your Goals

**Primary focus:** Wisdom is a **retrospective factory**. You call specialized analysts, synthesize their findings, and surface learnings to humans.

**The questions you're answering:**
- **Solution Fit:** Did we solve the right problem? (→ `solution-analyst`)
- **Code Quality:** How healthy is the code we just shipped? (→ `quality-analyst`)
- **Maintainability:** Will this code be easy to work with? (→ `maintainability-analyst`)
- **Friction:** Where did the swarm hit walls? (→ `friction_log.md`)
- **Process:** Did we build it efficiently? (→ `process-analyst`)
- **Regressions:** Did we break anything? (→ `regression-analyst`)
- **Patterns:** Are we seeing the same issues across runs? (→ `pattern-analyst`)
- **Signal Quality:** Which feedback sources were accurate? (→ `signal-quality-analyst`)
- **Timeline:** How long did this take? Where did we stall? (→ `flow-historian` with DevLT)
- **Learning:** What should we do differently next time? (→ `learning-synthesizer`)

**You are a manager, not an analyst.** Call the analysts, collect their reports, and synthesize. Don't do the analysis yourself.

**Fix-forward authority:** Wisdom can fix minor nits (typos, leftover console.logs, stale comments) discovered during retrospective. These become a checkpoint commit before the final seal. If the fixes are substantial enough, consider a follow-up PR.



### Before You Begin (Required)



#### Two State Machines



Flow 7 uses **two complementary state machines**:



1. **TodoWrite** = session navigation (keeps the orchestrator on track during this run)

2. **`flow_plan.md`** = durable on-disk state (enables reruns, handoffs, inspection)



#### Setup Steps



1. Use Claude Code's **TodoWrite** tool to create a TODO list of **major stations**.

   - Track at the behavioral/station level, NOT per agent call.



2. Mirror the same list into `.runs/<run-id>/wisdom/flow_plan.md` as checkboxes.

   - As each station completes: mark TodoWrite done AND tick the checkbox.



#### Suggested TodoWrite Items



```
- run-prep (establish run infrastructure)
- repo-operator (ensure run branch)
- artifact-auditor (verify artifacts)
- solution-analyst (requirement/implementation alignment)
- quality-analyst (code health/complexity)
- maintainability-analyst (naming, modularity, DRY, coupling)
- process-analyst (flow efficiency, iterations, bounces)
- regression-analyst (analyze regressions)
- pattern-analyst (cross-run patterns)
- signal-quality-analyst (feedback accuracy)
- flow-historian (build history + DevLT)
- learning-synthesizer (synthesize learnings)
- feedback-applier (draft actions only; no gh issue create before secrets gate)
- traceability-auditor (run-level coherence + spec traceability)
- risk-analyst (compare predicted vs actual)
- wisdom-cleanup (finalize receipt; update index; update `flow_plan.md`)
- secrets-sanitizer (publish gate; capture Gate Result block)
- repo-operator (checkpoint commit; allowlist interlock)
- gh-issue-manager (update issue board; gated)
- gh-reporter (report learnings; gated)
```



#### On Rerun



If running `/flow-7-wisdom` on an existing run-id:

- Read `.runs/<run-id>/wisdom/flow_plan.md`

- Create TodoWrite from the checklist

- Pre-mark items done if artifacts exist and look current

- Run remaining stations to refine



This flow uses **flow artifacts and git/GitHub**. No external observability platform required.



**For production extensions** (metrics, logs, traces, incidents, SLOs): extend this flow with your observability platform.



### Subagents to use



**Infrastructure (Step 0)**:

- **run-prep** -- MUST be called first to establish the run directory and `.runs/<run-id>/wisdom/`



Domain agents (Flow 7):

- artifact-auditor

- solution-analyst

- quality-analyst

- maintainability-analyst

- process-analyst

- regression-analyst

- pattern-analyst

- signal-quality-analyst

- flow-historian

- learning-synthesizer

- feedback-applier

- traceability-auditor



Cross-cutting agents:

- risk-analyst



Cleanup + Reporting (End of Flow):

- wisdom-cleanup -- writes wisdom_receipt.json, updates index.json status

- secrets-sanitizer -- publish gate (returns Gate Result block)

- repo-operator -- checkpoint commit (gated on Gate Result + anomaly check)

- gh-issue-manager -- updates issue body status board (final update)

- gh-reporter -- posts mini-postmortem summary



### Upstream Inputs



Read from all prior flow directories (if available):

- `.runs/<run-id>/signal/signal_receipt.json`

- `.runs/<run-id>/plan/plan_receipt.json`

- `.runs/<run-id>/build/build_receipt.json`

- `.runs/<run-id>/gate/gate_receipt.json`

- `.runs/<run-id>/deploy/deploy_receipt.json`



**If upstream artifacts are missing**: Flow 7 can start without all prior flows. Proceed best-effort: analyze what's available, document gaps, set status to UNVERIFIED, and continue.



### Orchestration outline



This is a **linear pipeline**. The sanitizer scans before checkpoint — rescans are allowed if new changes are staged, but no reseal loop (don't regenerate receipts after sanitizer runs).



#### Step 0: Establish Run Infrastructure



**Call `run-prep` first.**



This agent will:

- Derive or confirm the `<run-id>` from context, branch name, or user input

- Create `.runs/<run-id>/wisdom/` directory structure

- Update `.runs/<run-id>/run_meta.json` with "wisdom" in `flows_started`

- Update `.runs/index.json`



After this step, you will have a confirmed run directory. All subsequent agents write to `.runs/<run-id>/wisdom/`.



#### Step 0b: Ensure Run Branch



**Call `repo-operator`** with task: "ensure run branch `run/<run-id>`"



The agent handles branch creation/switching safely. This keeps checkpoint commits off main.



#### Step 1: Initialize Flow Plan



Create or update `.runs/<run-id>/wisdom/flow_plan.md`:



```markdown

# Flow 7: Wisdom for <run-id>



## Planned Steps

- [ ] run-prep (establish run directory)
- [ ] repo-operator (ensure run branch)
- [ ] artifact-auditor (verify all flow artifacts)
- [ ] solution-analyst (requirement/implementation alignment)
- [ ] quality-analyst (code health/complexity)
- [ ] maintainability-analyst (naming, modularity, DRY, coupling)
- [ ] process-analyst (flow efficiency, iterations, bounces)
- [ ] regression-analyst (analyze test/coverage regressions)
- [ ] pattern-analyst (cross-run patterns)
- [ ] signal-quality-analyst (feedback accuracy)
- [ ] flow-historian (build timeline + DevLT)
- [ ] learning-synthesizer (extract learnings)
- [ ] feedback-applier (draft actions; no gh issue create before secrets gate)
- [ ] traceability-auditor (run-level coherence + spec traceability)
- [ ] risk-analyst (compare predicted vs actual)
- [ ] wisdom-cleanup (write receipt, update index)
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint commit with allowlist interlock)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)


## Progress Notes



<Update as each step completes>

```



#### Step 2: Artifact Audit

**Call `artifact-auditor`** — verifies all expected flow artifacts exist.

#### Step 3: Solution Analysis

**Call `solution-analyst`** — traces requirements → BDD → implementation → tests. Verifies we built the right thing.

#### Step 4: Quality Analysis

**Call `quality-analyst`** — analyzes code health, complexity of the changed files.

#### Step 5: Maintainability Analysis

**Call `maintainability-analyst`** — deep dive on naming, modularity, DRY, coupling, documentation, test quality.

#### Step 6: Process Analysis + Friction Log

**Call `process-analyst`** — analyzes flow efficiency: iterations, bounces, stalls, where we could improve.

The process-analyst also writes `.runs/<run-id>/wisdom/friction_log.md`:
- Where the swarm hit walls (stuck loops, CANNOT_PROCEED states)
- Context exhaustion events (PARTIAL exits)
- Tool/environment failures
- Unclear prompts or missing context that caused rerun loops
- Agent capabilities that were missing or underperforming

This friction log informs pack improvements—the "Staff Engineer" whispers in the ear of the next run.

#### Step 7: Regression Analysis

**Call `regression-analyst`** — checks for test regressions, coverage changes, stability issues.

#### Step 8: Pattern Analysis

**Call `pattern-analyst`** — looks across historical runs to find recurring issues, repeated failures, and trends.

#### Step 9: Signal Quality Analysis

**Call `signal-quality-analyst`** — analyzes accuracy of feedback sources (CI, bots, humans). Tracks which signals were valid vs noise.

#### Step 10: Timeline + DevLT

**Call `flow-historian`** — compiles the run timeline and calculates Developer Lead Time (DevLT): how much human attention did this run actually require?

#### Step 11: Synthesize Learnings

**Call `learning-synthesizer`** — extracts patterns from the analysis: what worked, what didn't, what to do differently.

#### Step 12: Apply Feedback

**Call `feedback-applier`** — turns learnings into concrete actions. Does NOT create GitHub issues directly.

**Audience-Segmented Outputs:**

Wisdom learnings are only valuable if they reach the right consumer:

| Output | Audience | Content |
|--------|----------|---------|
| `pack_improvements.md` | **Pack (Machine)** | Ready-to-apply diffs for agent prompts, flow docs, skills |
| `codebase_wisdom.md` | **Repo (Human)** | Structural hotspots, brittle patterns, architectural observations |
| `feedback_actions.md` | **Project (Both)** | Issue drafts, doc suggestions, follow-up work items |
| `.runs/_wisdom/latest.md` | **Future (Scent Trail)** | Top learnings that inform the next run's researcher |

**The Scent Trail:** `.runs/_wisdom/latest.md` is a special file that persists across runs. It captures the top 3-5 learnings from this run that should inform future runs. The `gh-researcher` reads this file before starting research, closing the learning loop.

**Wisdom Produces Edits, Not Advice:**

When Flow 7 identifies pack/flow improvements (from friction log, process analysis, or pattern analysis):
- `feedback-applier` should produce **suggested diffs** to agent prompts, not just prose advice
- Example: If `bdd-critic` keeps missing edge cases, propose a specific edit to `.claude/agents/bdd-critic.md` with the new guidance
- The diff goes in `.runs/<run-id>/wisdom/pack_improvements.md` as fenced code blocks
- Humans review and apply the diffs (or reject them)

This is the "Staff Engineer whisper" — concrete improvements to the factory, not vague recommendations.

**Pack improvement output format:**
```markdown
## Pack Improvement: <title>

**Pattern observed:** <what friction/failure was seen>
**Evidence:** <which runs, which agents, which artifacts>
**Suggested edit:**

File: `.claude/agents/<agent>.md`
```diff
- <old line>
+ <new line>
```

**Risk:** <Low/Medium/High>
**Rationale:** <why this fix addresses the pattern>
```

#### Step 12b: Traceability

**Call `traceability-auditor`** — verifies run identity, receipts, and GitHub markers are coherent.

#### Step 12c: Risk Comparison

**Call `risk-analyst`** — compares predicted risks (from Signal) vs actual outcomes.



#### Step 13: Finalize and Write Receipt

- `wisdom-cleanup` -> `.runs/<run-id>/wisdom/wisdom_receipt.json`, `.runs/<run-id>/wisdom/cleanup_report.md`

- Verifies all required artifacts exist

- Computes counts mechanically (never estimates)

- Updates `.runs/index.json` with status, last_flow, updated_at

- This is the final receipt for the run



#### Step 14: Sanitize Secrets (Publish Gate)

- `secrets-sanitizer` -> `.runs/<run-id>/wisdom/secrets_scan.md`, `.runs/<run-id>/wisdom/secrets_status.json`

- Scans all wisdom artifacts before posting

- **Returns a Gate Result block**  this is the control plane for routing decisions



**Status vs flags:**

- `status` is descriptive (CLEAN/FIXED/BLOCKED_PUBLISH)

- `safe_to_commit` / `safe_to_publish` are authoritative



**Control plane:** Route on the **Gate Result block** returned by `secrets-sanitizer`. `secrets_status.json` is audit-only (optional last-mile verification).



**Gate Result block (returned by secrets-sanitizer):**

<!-- PACK-CONTRACT: GATE_RESULT_V3 START -->
```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
findings_count: <int>
blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
blocker_reason: <string | null>
```
<!-- PACK-CONTRACT: GATE_RESULT_V3 END -->

**Gating logic (boolean gate — the sanitizer says yes/no, orchestrator decides next steps):**
- The sanitizer is a fix-first pre-commit hook, not a router
- If `safe_to_commit: true` → proceed to checkpoint commit (Step 14b)
- If `safe_to_commit: false`:
  - `blocker_kind: MECHANICAL` → **FIX_ENV** (tool/IO failure)
  - `blocker_kind: SECRET_IN_CODE` → route to appropriate agent (orchestrator decides)
  - `blocker_kind: SECRET_IN_ARTIFACT` → investigate manually
- Push requires: `safe_to_publish: true` AND Repo Operator Result `proceed_to_github_ops: true`
- GitHub reporting ops still run in RESTRICTED mode when publish is blocked or `publish_surface: NOT_PUSHED`



#### Step 14b: Checkpoint Commit



Checkpoint the audit trail **before** any GitHub operations.



**Call `repo-operator`** with checkpoint mode. The agent:

1. Resets staging and stages only the allowlist (not `git add .`)

2. Enforces the allowlist/anomaly interlock mechanically

3. Writes `.runs/<run-id>/wisdom/git_status.md` if anomaly detected

4. Handles no-op (nothing staged) gracefullyno empty commits



**Allowlist for Flow 7:**

- `.runs/<run-id>/wisdom/`

- `.runs/<run-id>/run_meta.json`

- `.runs/index.json`



**Control plane:** `repo-operator` returns a Repo Operator Result block:

```

## Repo Operator Result

operation: checkpoint

status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED

proceed_to_github_ops: true | false

commit_sha: <sha>

publish_surface: PUSHED | NOT_PUSHED

anomaly_paths: []

```

**Note:** `commit_sha` is always populated (current HEAD on no-op), never null.



Orchestrators route on this block, not by re-reading `git_status.md`.



**Anomaly detection:** If anything outside allowlist is dirty (modified/staged/untracked):

- **Anomaly detected**  commit allowlist only

- Set `proceed_to_github_ops: false`

- Write `.runs/<run-id>/wisdom/git_status.md` documenting unexpected paths

- Flow completes locally **UNVERIFIED**



**Gating logic (from prior secrets-sanitizer Gate Result + repo-operator result):**

- If `safe_to_commit: false` (from Gate Result): skip commit entirely

- If anomaly detected: `repo-operator` commits allowlist only, skips push, returns `proceed_to_github_ops: false`

- If `safe_to_publish: true` AND no anomaly: `repo-operator` pushes the branch, returns `proceed_to_github_ops: true`

- If `safe_to_publish: false`:

  - If `needs_upstream_fix: true`  **BOUNCE** to `route_to_agent` (and optionally `route_to_flow`) with pointer to `secrets_scan.md`

  - If `status: BLOCKED_PUBLISH`  **CANNOT_PROCEED** (mechanical failure); stop and require human intervention

  - Otherwise → UNVERIFIED; skip push (`publish_surface: NOT_PUSHED`). Continue with GitHub Reporting Ops in RESTRICTED mode when access allows.



#### Step 15-16: GitHub Reporting (Final)

**Call `gh-issue-manager`** (marks run complete) then **`gh-reporter`** (mini-postmortem).

See `CLAUDE.md` → **GitHub Access + Content Mode** for gating rules. Quick reference:
- Skip if `github_ops_allowed: false` or `gh` unauthenticated
- Content mode is derived from secrets gate + push surface (not workspace hygiene)
- Issue-first: flow summaries go to the issue, never the PR

**Quality-first reporting:** The GitHub postmortem should lead with:
1. **Solution Verdict** — Did we solve the right problem?
2. **Maintainability Score** — Will this code be easy to work with?
3. **Quality Summary** — Code health assessment

DevLT and process metrics go in a **"Process Metrics" fold** at the bottom. We want humans to see the quality assessment first, not just how fast we worked.

**Content for postmortem:** Quality/solution verdicts, learnings, pack/flow observations, feedback actions, meta-notes on the wisdom synthesis.

#### Step 17: Finalize Flow



Update `flow_plan.md`:

- Mark all steps as complete

- Add final summary section:



```markdown

## Summary



- **Final Status**: VERIFIED | UNVERIFIED

- **Regressions Found**: <count>

- **Learnings Extracted**: <count>

- **Feedback Actions Created**: <count>

- **Run Complete**: This run-id is now closed



## Human Review Checklist



- [ ] `.runs/<run-id>/wisdom/learnings.md` - Are learnings actionable?

- [ ] `.runs/<run-id>/wisdom/feedback_actions.md` - Which actions should be prioritized?

- [ ] `.runs/<run-id>/wisdom/regression_report.md` - Are regressions understood?

```



### Closed Feedback Loops



Flow 7 closes the SDLC loop by feeding learnings back (recommendations, not direct calls):



#### -> Flow 1 (Signal)

- `learning-synthesizer` extracts problem patterns

- `feedback-applier` suggests updates to requirement templates

- Builds institutional memory of "problems that recur"



#### -> Flow 2 (Plan)

- `feedback-applier` suggests architecture doc updates

- Documents patterns that worked/failed

- Improves design templates and ADR prompts



#### -> Flow 3 (Build)

- `feedback-applier` drafts GitHub issues for test gaps (for human review)

- Links regression failures to coverage gaps

- Suggests test pattern improvements
- If Build produced hardening worklists (e.g., `build/mutation_report.md`, `build/fuzz_report.md`, `build/flakiness_report.md`), promote the top items into `feedback_actions.md` as issue drafts (with evidence pointers).



These are **recommendations in artifacts**, not direct flow invocations. Humans decide which to act on.



### Expected Outputs



When complete, `.runs/<run-id>/wisdom/` should contain:



- `flow_plan.md` - execution plan and progress

- `artifact_audit.md` - structural sanity check of all flows

- `solution_analysis.md` - requirement/implementation alignment

- `quality_report.md` - code health, complexity

- `maintainability_analysis.md` - naming, modularity, DRY, coupling deep dive

- `process_analysis.md` - flow efficiency, iterations, bounces

- `friction_log.md` - where the swarm hit walls (for pack improvement)

- `regression_report.md` - what got worse and where

- `pattern_report.md` - cross-run recurring issues and trends

- `signal_quality_report.md` - feedback source accuracy analysis

- `flow_history.json` - timeline linking all flow events + DevLT metrics

- `learnings.md` - narrative lessons extracted

- `feedback_actions.md` - concrete follow-ups (issues, doc updates)

- `pack_improvements.md` - suggested diffs to pack/agent prompts (from feedback-applier)
- `codebase_wisdom.md` - structural insights for humans: hotspots, brittle patterns, architectural observations (from feedback-applier)

- `risk_assessment.md` - risk perspective (optional, if risk-analyst invoked)

- `wisdom_receipt.json` - final receipt for the run

- `cleanup_report.md` - cleanup status and evidence

- `secrets_scan.md` - secrets scan report

- `secrets_status.json` - publish gate status

- `git_status.md` - repo state at checkpoint (if anomaly detected)

- `gh_issue_status.md` - issue board update status

- `gh_report_status.md` - GitHub posting status

- `github_report.md` - report content (local copy)



### Completion States



Flow 7 agents report:



- **VERIFIED**: `blockers` empty, `missing_required` empty, and analysis complete with all artifacts processed. Set `recommended_action: PROCEED`.

- **UNVERIFIED**: `blockers` non-empty OR `missing_required` non-empty OR some data unavailable (GitHub, git, etc.) OR anomaly detected during checkpoint. Set `recommended_action: RERUN | BOUNCE` depending on fix location.

- **CANNOT_PROCEED**: IO/permissions/tool failure only (exceptional); cannot read files, tool missing, etc. Set `missing_required` with paths and `recommended_action: FIX_ENV`.



**Key rule**: CANNOT_PROCEED is strictly for mechanical failures. Missing upstream artifacts are UNVERIFIED with `missing_required` populated, not CANNOT_PROCEED.



Any of these are valid outcomes. Document concerns and continue.



### Stable Marker Contract (for mechanical counting)



Flow 7 producers must use these stable markers so `wisdom-cleanup` can derive counts mechanically:



| Agent | Marker Pattern | Artifact | Example |

|-------|----------------|----------|---------|

| solution-analyst | `^### SOL-[0-9]{3}:` | solution_analysis.md | `### SOL-001: Missing OAuth implementation` |

| quality-analyst | `^- QUALITY_ISSUE_` | quality_report.md | `- QUALITY_ISSUE_HIGH: 3` |

| maintainability-analyst | `^- \*\*MAINT-[0-9]{3}\*\*:` | maintainability_analysis.md | `- **MAINT-001**: Auth handler too large` |

| process-analyst | `^### PROC-[0-9]{3}:` | process_analysis.md | `### PROC-001: AC-002 took 4 iterations` |

| regression-analyst | `^### REG-[0-9]{3}:` | regression_report.md | `### REG-001: test_foo::bar  assertion failed` |

| pattern-analyst | `^### PAT-[0-9]{3}:` | pattern_report.md | `### PAT-001: Flaky auth tests` |

| signal-quality-analyst | `^### SQ-FP-[0-9]{3}:` | signal_quality_report.md | `### SQ-FP-001: FB-RC-123456789` |

| learning-synthesizer | `^## Learning: ` | learnings.md | `## Learning: Requirements` |

| feedback-applier | `^- ISSUE: ` | feedback_actions.md | `- ISSUE: Missing tests for REQ-004` |

| flow-historian | `"devlt":` | flow_history.json | `"devlt": {"total_run_minutes": 45, "human_attention_minutes": 8}` |



**Regression format rule:** Each regression MUST have exactly one `### REG-NNN:` heading section. (You may also include a register table, but headings are the source for counting.)



**Why this matters:** Without stable markers, `wisdom-cleanup` cannot derive counts mechanically and must set them to `null` with reasons. Agents that omit markers degrade receipt quality.



---



### Orchestrator Kickoff


#### Station order

1. `run-prep`

2. `repo-operator` (ensure run branch)

3. `artifact-auditor`

4. `solution-analyst`

5. `quality-analyst`

6. `maintainability-analyst`

7. `process-analyst`

8. `regression-analyst`

9. `pattern-analyst`

10. `signal-quality-analyst`

11. `flow-historian`

12. `learning-synthesizer`

13. `feedback-applier`

14. `traceability-auditor`

15. `risk-analyst`

16. `wisdom-cleanup`

17. `secrets-sanitizer`

18. `repo-operator` (checkpoint commit)

19. `gh-issue-manager` (if allowed)

20. `gh-reporter` (if allowed)

#### TodoWrite (copy exactly)

- [ ] run-prep
- [ ] repo-operator (ensure `run/<run-id>` branch)
- [ ] artifact-auditor
- [ ] solution-analyst (requirement/implementation alignment)
- [ ] quality-analyst (code health/complexity)
- [ ] maintainability-analyst (naming, modularity, DRY, coupling)
- [ ] process-analyst (flow efficiency, iterations, bounces)
- [ ] regression-analyst (test/coverage regressions)
- [ ] pattern-analyst (cross-run patterns)
- [ ] signal-quality-analyst (feedback accuracy)
- [ ] flow-historian (timeline + DevLT)
- [ ] learning-synthesizer
- [ ] feedback-applier (draft actions only; no gh issue create before secrets gate)
- [ ] traceability-auditor (run-level coherence + spec traceability)
- [ ] risk-analyst
- [ ] wisdom-cleanup
- [ ] secrets-sanitizer (capture Gate Result block)
- [ ] repo-operator (checkpoint commit; allowlist interlock + no-op handling)
- [ ] gh-issue-manager (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)
- [ ] gh-reporter (skip only if github_ops_allowed: false or gh unauth; FULL/RESTRICTED from gates + publish_surface)

Use explore agents to answer any immediate questions you have and then create the todo list and call the agents.

---

