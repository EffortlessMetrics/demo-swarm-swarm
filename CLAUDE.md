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

**Quick links for reviewers:**
- [What is quality here?](docs/reference/pr-quality-scorecard.md) — the multi-sensor trust panel
- [How do I review a swarm PR?](docs/how-to/review-a-swarm-pr.md) — the decision procedure
- [Show me examples](docs/examples/) — sample artifacts

---

## Core Philosophy

**Engineering is default-allow. Publishing is gated.**

The pack is a **build pipeline with guardrails**, not a guardrail pipeline that sometimes builds. Agents explore, implement, and iterate freely. Gates engage only at publish boundaries (commit, push, GitHub post).

**The thesis:** Humans author intent. Swarms manufacture verified change. Reviewers audit the cockpit, not the diff.

**The ratio:** Quality per DevLT (PR quality achieved per minute of developer attention). Compute is cheap; architect attention is expensive. Trade accordingly.

See: [docs/explanation/why-ops-first.md](docs/explanation/why-ops-first.md)

---

## Rules (The Constitution)

Claude Code rules live in `.claude/rules/`. They encode the physics and vibe that shape all behavior.

| Rule | What It Governs |
|------|-----------------|
| `00-doctrine.md` | Core thesis, the triangle, anti-austerity |
| `10-operating-model.md` | PM + IC swarm, when to spawn agents |
| `20-intent-to-narrative.md` | The pipeline from intent to PR |
| `30-autonomy-and-boundaries.md` | Default-allow + strict gates |
| `40-evidence-and-quality.md` | Claims require pointers, the quality panel |
| `50-agent-contract.md` | Agent prompts (scoped to `.claude/agents/`) |
| `60-flow-orchestrators.md` | Flow commands (scoped to `.claude/commands/flow-*`) |
| `70-docs-and-teaching.md` | Documentation (scoped to `docs/`) |
| `80-developer-experience.md` | UX, accessibility, investing in quality |
| `90-voice-and-tone.md` | How we communicate: industrial clarity, human warmth |

Rules are the constitution Claude loads reliably. CLAUDE.md is the contract. Docs are the textbook.

---

## Agent Philosophy

**Orchestrators are PMs. Agents are well-trained juniors.**

Orchestrators scope work, route tasks, and make sequencing decisions. Agents do real cognitive work: they think, investigate, make judgment calls, and produce artifacts with substance. Agents are not clipboard-copiers or template-fillers.

**Why spawn an agent?** Two reasons only:
1. **Do work** — The task requires focused expertise (code-implementer writes code, test-author writes tests)
2. **Compress context** — A specialist can summarize, filter, or derive information more efficiently than carrying full context forward

**Every agent returns two things:**
1. **An answer** — What they found, built, or concluded
2. **A routing suggestion** — What should happen next and why

**Honest partial reports are successful outcomes.** An agent that completes 60% of the work and clearly documents what's done, what's blocked, and what to try next has succeeded. Failure to complete work is not failure as an agent. Hiding uncertainty behind false completion is the actual failure mode.

**Artifacts exist because content matters.** Receipts tell stories about what happened. Decision documents explain reasoning. These artifacts have value as records and communication, not just as routing gates. Write them for the next reader (human or agent), not for a parser.

**Single responsibility, done deeply.** Each agent has one clear job. They do it thoroughly, then hand off. code-critic critiques but never fixes. code-implementer implements but never commits. repo-operator handles git but doesn't judge code quality.

See: [docs/explanation/agent-philosophy.md](docs/explanation/agent-philosophy.md)

---

## Coordination Patterns

These patterns keep agents aligned without over-constraining them:

1. **Repo root orientation**
   Commands run from repo root; paths are repo-root-relative. This keeps agents oriented consistently.

2. **Git ownership**
   `repo-operator` owns git operations. Other agents describe what they want ("commit these changes with this message") rather than running git directly. This concentrates git expertise and prevents conflicts.

3. **Natural language routing**
   Orchestrators route based on what agents tell them in their handoff response. Agents explain what happened and recommend next steps. Files on disk are durable records, not routing inputs.

4. **Publish gates**
   Before posting to GitHub, two conditions apply:
   - Secrets scan passes (no credentials in content)
   - Repo is in a clean, pushable state

   These gates protect the publish boundary, not internal iteration.

5. **Stable run identities**
   Run folders keep their names. If a run's meaning changes, update metadata (`canonical_key`, `aliases[]`) rather than renaming directories.

---

## PR Review Interface

Most reviewers will only read the GitHub PR description. Treat it as the cockpit display.

- Every run that produces a PR **must** produce a **PR Brief** (hotspots, quality events, proof pointers, explicit "not measured")
- Receipts are the underlying truth; the PR Brief is the primary human interface; the diff is spot-check/audit
- Avoid "dev vs machine" framing. This is a developer enablement system: cheap iteration buys verification so humans spend time on decisions

See: [docs/reference/pr-review-interface.md](docs/reference/pr-review-interface.md) for the template and guidelines.

See: [docs/explanation/reviewing-as-audit.md](docs/explanation/reviewing-as-audit.md) for the philosophy.

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

## Handoffs

Agents communicate through natural language. When an agent finishes work, it provides a handoff:

1. **What was done?** — Summary of work completed
2. **What still needs to be done?** — Remaining work, blockers, open questions
3. **My recommendation** — Specific next step with reasoning

Orchestrators read these responses and route accordingly. There's no parsing of structured blocks; agents are trusted to communicate clearly.

**Always make a recommendation.** Name specific agents when you know them. Explain your reasoning. If you're uncertain, say so and explain why.

**Specialized agents** (like secrets-sanitizer, repo-operator) include specific details the orchestrator needs to make routing decisions, but these are part of a natural response, not a machine-parsed format.

See: [docs/reference/contracts.md](docs/reference/contracts.md) for communication patterns and examples.

---

## Architecture Principles

Seven principles that guide how agents collaborate:

1. **Clear roles** — Orchestrators scope and route. Agents do focused work. Cleanup agents verify state.

2. **Resume-ready** — Agents check what exists on disk when they start. They pick up where things left off rather than assuming a blank slate.

3. **Workers own their ledger** — The agent who does the work updates the status. If you wrote the code, you update the implementation status.

4. **Completion = verified** — Work is complete when tests pass and the orchestrator agrees. Partial progress with honest reporting is a valid intermediate state.

5. **Research before asking** — Investigate the codebase, derive from patterns, choose safe defaults, then escalate only if truly stuck.

6. **Foundation first** — Infrastructure and shared components come before features that depend on them.

7. **Local resolution** — When something doesn't fit, try to resolve it within your scope before bouncing back to earlier flows.

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

See [docs/how-to/troubleshoot.md](docs/how-to/troubleshoot.md) for common situations:
- **Mechanical failures** — Environment or tooling issues blocking execution
- **GitHub operations not happening** — Check that secrets scan passed and repo is in pushable state
- **Loops not terminating** — Follow the agent's recommended action
- **Missing information** — Check that upstream artifacts contain the expected content

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
| **Rules (constitution)** | `.claude/rules/*.md` |
| The Thesis | `docs/explanation/the-thesis.md` |
| Architecture & Philosophy | `docs/explanation/` |
| The Physics | `docs/explanation/the-physics.md` |
| Emergent Phenomena | `docs/explanation/emergent-phenomena.md` |
| Authority vs Difficulty | `docs/explanation/authority-not-difficulty.md` |
| Org Design as Code | `docs/explanation/org-design-as-code.md` |
| Reviewing as Audit | `docs/explanation/reviewing-as-audit.md` |
| Codebase as Mold | `docs/explanation/codebase-as-mold.md` |
| Control-plane blocks, schemas | `docs/reference/contracts.md` |
| Schemas | `docs/reference/schemas.md` |
| Run state schemas | `docs/reference/run-state.md` |
| Stable markers | `docs/reference/stable-markers.md` |
| Trust model | `docs/reference/trust-model.md` |
| Calibration | `docs/reference/calibration.md` |
| CLI commands | `docs/reference/demoswarm-cli.md` |
| How-to guides | `docs/how-to/` |
| Flow commands | `.claude/commands/flow-*.md` |
| Agent prompts | `.claude/agents/*.md` |
