# CLAUDE.md

This repository contains an SDLC swarm pack under `.claude/`.

**Operational reality:** This file is attached to every agent thread in Claude Code. Treat it as **repo-level policy + shared contracts** (not a marketing doc). If flow commands, agent prompts, or pack-check drift from what's written here, update the pack so everything agrees.

---

## For Humans

This pack provides:

- **7 flows**: Signal → Plan → Build → Review → Gate → Deploy → Wisdom
- **Narrow specialist agents**: requirements-author, code-critic, test-author, *-cleanup, etc. (see `.claude/agents/`)
- **7 skills**: test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools

Start here:

```
/flow-1-signal "your feature idea"
```

Then proceed in order (unless you are intentionally running out-of-order):

`/flow-2-plan` → `/flow-3-build` → `/flow-4-review` → `/flow-5-gate` → `/flow-6-deploy` → `/flow-7-wisdom`

---

## Core Philosophy

**Engineering is default-allow. Publishing is gated.**

The pack is a **build pipeline with guardrails**, not a guardrail pipeline that sometimes builds. Agents explore, implement, and iterate freely. Gates engage only at publish boundaries (commit, push, GitHub post).

See: [docs/explanation/why-ops-first.md](docs/explanation/why-ops-first.md)

**Agents are intelligent actors.** They investigate, derive, default, then escalate. Critics never fix. Workers maintain the ledger. Cleanup agents audit.

See: [docs/explanation/agent-philosophy.md](docs/explanation/agent-philosophy.md)

---

## Non-Negotiables

These rules exist to prevent drift and "model invention":

1. **Repo root only**
   All commands run from **repo root**; all paths are **repo-root-relative**. Do not rely on `cd`.

2. **No raw git in flow commands or agent prompts**
   Git operations are owned by `repo-operator`. Orchestrators call `repo-operator` using **task phrasing**.

3. **Control plane vs audit plane**
   Orchestrators route on returned result blocks (`Gate Result`, `Repo Operator Result`).
   Files like `secrets_status.json` and `git_status.md` are durable audit records, not routing inputs.

4. **Two gates for GitHub operations**
   GitHub operations (`gh-issue-manager`, `gh-reporter`) require BOTH:
   - `safe_to_publish: true` (secrets gate)
   - `proceed_to_github_ops: true` (repo hygiene gate)

5. **`run_id` folders never rename**
   Identity changes happen via `canonical_key` + `aliases[]`, never via renaming directories.

---

## PR Review Interface

Most reviewers will only read the GitHub PR description. Treat it as the cockpit display.

- Every run that produces a PR **must** produce a **PR Brief** (hotspots, quality events, proof pointers, explicit "not measured")
- Receipts are the underlying truth; the PR Brief is the primary human interface; the diff is spot-check/audit
- Avoid "dev vs machine" framing. This is a developer enablement system: cheap iteration buys verification so humans spend time on decisions

See: [docs/reference/pr-review-interface.md](docs/reference/pr-review-interface.md) for the template and guidelines.

---

## The Seven Flows

| Flow | Slash Command | Key Outputs |
|------|---------------|-------------|
| 1. Signal | `/flow-1-signal` | `requirements.md`, `features/*.feature`, `signal_receipt.json` |
| 2. Plan | `/flow-2-plan` | `adr.md`, `api_contracts.yaml`, `work_plan.md`, `plan_receipt.json` |
| 3. Build | `/flow-3-build` | code/tests, critiques, `build_receipt.json`, Draft PR |
| 4. Review | `/flow-4-review` | `review_worklist.md`, `review_receipt.json` |
| 5. Gate | `/flow-5-gate` | `merge_decision.md`, `gate_receipt.json` |
| 6. Deploy | `/flow-6-deploy` | `verification_report.md`, `deploy_receipt.json` |
| 7. Wisdom | `/flow-7-wisdom` | `learnings.md`, `wisdom_receipt.json` |

Out-of-order is allowed: proceed best-effort, document assumptions, expect UNVERIFIED outcomes when upstream artifacts are missing.

---

## Skills

| Skill | Purpose |
|-------|---------|
| `test-runner` | Run tests, capture output to run artifacts |
| `auto-linter` | Format + lint code |
| `policy-runner` | Run policy-as-code checks |
| `runs-derive` | Read-only .runs derivations (counts, extraction) |
| `runs-index` | Write .runs/index.json updates |
| `openq-tools` | Open questions register (QID generation) |
| `secrets-tools` | Secrets scanning/redaction for publish gates |

---

## Run State

Run artifacts live under: `.runs/<run-id>/<flow>/`

Example: `.runs/feat-auth/build/impl_changes_summary.md`

See: [docs/reference/run-state.md](docs/reference/run-state.md) for schemas (`index.json`, `run_meta.json`)

---

## Control-Plane Blocks

Gate agents emit structured blocks for routing. Orchestrators route on these, not by rereading files.

See: [docs/reference/contracts.md](docs/reference/contracts.md) for:
- Gate Result (secrets-sanitizer)
- Repo Operator Result
- PR Feedback Harvester Result
- Machine Summary format

---

## Handoff Contract

Agents communicate routing through natural language handoffs:

1. **What was done?** — Summary of work completed
2. **What still needs to be done?** — Remaining work, blockers, open questions
3. **My recommendation** — Specific next step with reasoning

**Always make a recommendation.** Name specific agents when you know them. Explain your reasoning.

See: [docs/reference/contracts.md](docs/reference/contracts.md) for status concepts and routing patterns.

---

## Architecture Laws

Seven invariants that prevent execution drift:

1. **PM/IC Boundary** — Orchestrators route. Agents work. Cleanup agents audit.
2. **Every Call Is an Implicit Resume** — Agents check disk state, not mode flags.
3. **Workers Maintain the Ledger** — The worker who touches code updates the status.
4. **AC Termination = Green + Orchestrator Agreement** — Tests + critic satisfaction.
5. **Research-First Autonomy** — Investigate → Derive → Default → Escalate.
6. **Foundation-First Sequencing** — Infrastructure subtasks have no dependencies.
7. **Local Resolution** — Resolve mismatches locally before bouncing to previous flows.

See: [docs/explanation/architecture.md](docs/explanation/architecture.md) for details and examples.

---

## Customization

See [docs/how-to/customize-pack.md](docs/how-to/customize-pack.md) for:
- Prerequisites (bash/jq/grep, Windows/WSL2/Git Bash)
- Test/lint command adaptation
- Source layout changes
- Git provider adaptation

---

## Troubleshooting

See [docs/how-to/troubleshoot.md](docs/how-to/troubleshoot.md) for:
- "CANNOT_PROCEED" — Mechanical failure; fix environment/tooling
- "No GitHub update" — Check two gates (`safe_to_publish`, `proceed_to_github_ops`)
- "Microloop won't terminate" — Route on `recommended_action`
- "Counts are null" — Check stable markers in producer artifacts

---

## CLI Tooling

Agents invoke via shims (never assume PATH):

```bash
bash .claude/scripts/pack-check.sh [OPTIONS]    # Pack validation
bash .claude/scripts/demoswarm.sh <command>     # CLI operations
```

See: [docs/reference/demoswarm-cli.md](docs/reference/demoswarm-cli.md) for command reference.

---

## Reference Index

| Topic | Location |
|-------|----------|
| Architecture & Philosophy | `docs/explanation/` |
| Control-plane blocks, schemas | `docs/reference/contracts.md` |
| Run state schemas | `docs/reference/run-state.md` |
| Stable markers | `docs/reference/stable-markers.md` |
| Trust model | `docs/reference/trust-model.md` |
| CLI commands | `docs/reference/demoswarm-cli.md` |
| How-to guides | `docs/how-to/` |
| Flow commands | `.claude/commands/flow-*.md` |
| Agent prompts | `.claude/agents/*.md` |
