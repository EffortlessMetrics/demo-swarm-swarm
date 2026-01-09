# DemoSwarm

SDLC pack for Claude Code.

**The economics shift:** Open-weight coding models can now produce a first pass that's often comparable to what you'd expect from a solid junior engineer for many scoped tasks. More importantly, they can generate and revise code far faster than humans can review it, and cheaply enough that multiple passes (tests, critique, refactors, mutation/fuzz where it matters) are economical.

**Operating principle:** Spend machine iteration to buy down human uncertainty.

**What it produces:** A PR with evidence (tests, receipts, critiques) you can review in minutes, not hours.

---

## Start Here

```text
/customize-pack                              # once per repo
/flow-1-signal "Add a health check endpoint" # first run
```

Then open:
- `.runs/<run-id>/signal/requirements.md` — the contract
- `.runs/<run-id>/signal/open_questions.md` — assumptions needing validation

If the contract is wrong, rerun Flow 1. Fixing the spec is cheaper than fixing a bad build.

**Full setup:** [Quickstart](docs/tutorials/quickstart.md) · **Reference:** [CLAUDE.md](CLAUDE.md)

---

## How to Review What DemoSwarm Produces

**The PR description is your primary interface.** Most reviewers won't drill into `.runs/` artifacts unless something looks wrong. The swarm produces a PR Brief in the description with: what changed, review hotspots, quality events, and proof pointers. The artifacts below are drill-down evidence when you need them.

If you're reviewing a run (or a PR produced by the swarm), start here:

1. **Gate verdict:** `.runs/<run-id>/gate/merge_decision.md` — ship or no-ship
2. **Test proof:** `.runs/<run-id>/build/test_execution.md` — did tests actually pass
3. **Critiques:** `.runs/<run-id>/build/code_critique.md`, `test_critique.md` — what the critics found
4. **Receipts:** `.runs/<run-id>/*/*_receipt.json` — mechanical summaries with evidence pointers
5. **The diff:** the PR diff is the final audit surface

If you're evaluating quickly, these four files tell the whole story:
- `signal/requirements.md` — what we intended
- `plan/adr.md` — how we decided to build it
- `build/build_receipt.json` — what actually ran
- `gate/merge_decision.md` — ship or bounce

Artifacts are the handoff. Chat is transient.

---

## What This Actually Is

DemoSwarm is a `.claude/` pack (flows + agents + skills) that turns Claude Code into a repeatable build pipeline. It's a reference implementation of a mentality: the bottleneck was always *how long until the code is trusted*. LLMs just changed the economics — generation and verification are now cheap and fast.

You dispatch flows explicitly. Agents do work and write artifacts to disk. The filesystem is the record; chat is transient.

This is not "AI that codes for you." It's a system for producing **review-ready changes** — code plus the evidence needed to trust it. The artifact trail is what lets a reviewer skim receipts and approve with confidence, rather than re-auditing everything the model claimed to do.

### The Mentality Shift

Most AI coding tools optimize for **generation speed** — how fast can the model produce code?

DemoSwarm optimizes for **verification speed** — how quickly can a human decide yes/no with confidence?

The output isn't just code. It's code + tests + receipts + critiques + a clear audit trail.

Open-weight models are now good enough that, for many well-scoped changes, their first draft is at least "junior-quality" — and often cleaner once you add tests and basic cleanup. Since generation is faster than review and cheap enough to repeat, the winning strategy isn't one big prompt. It's many small loops: research → plan → implement → test → critique → harden.

### Trust and Verify

Agents are treated like capable peers: autonomous, productive, and occasionally wrong.

- **Trust agents to act** — research, decide, implement, fix issues they encounter
- **Verify with executed evidence** — tests, diffs, receipts are proof; prose is navigation
- **Catch problems early** — critics run inside build loops, not just at the end

If a flow exits **PARTIAL**, that's a save point: state is on disk, next steps are documented, and rerunning the same flow resumes where it left off.

---

## Repo Layout

| Location | What It Is |
|----------|------------|
| `.claude/commands/` | Flow playbooks (routing tables) |
| `.claude/agents/` | Stations: workers, critics, auditors, operators |
| `.claude/skills/` | Deterministic helpers (test-runner, auto-linter, etc.) |
| `tools/` | Rust CLI tooling (pack-check, runs-derive) |
| `.runs/` | Run artifacts (in the target repo) |

---

## The Seven Flows

| Flow | Purpose | Key Outputs |
|------|---------|-------------|
| **1. Signal** | Shape intent into contract | requirements, BDD scenarios, risks |
| **2. Plan** | Design the solution | ADR, contracts, AC matrix, work plan |
| **3. Build** | Implement AC-by-AC | code, tests, Draft PR, build receipt |
| **4. Review** | Harvest feedback, fix | drained worklist, Ready PR |
| **5. Gate** | Forensic audit | MERGE or BOUNCE verdict |
| **6. Deploy** | Merge to swarm mainline | CI verification |
| **7. Wisdom** | Extract learnings | feedback actions, scent trail |

---

## How This Differs

### From AI Coding Assistants

Most tools optimize the interactive loop (prompt → answer → prompt). DemoSwarm optimizes the review loop (run → evidence → decision).

You dispatch a flow, it runs (batch-oriented; can run for a while), and you come back to a finished artifact with evidence.

### From Autonomous Agents

DemoSwarm doesn't ship to prod. It produces a review-ready PR and leaves the merge decision to humans, backed by evidence.

### From Typical SDLC Tooling

Traditional tooling tracks what humans decided. DemoSwarm also records what the machine did, what it checked, and what evidence exists.

---

## Where This Fits

**Stack position:** Workflow layer on Claude Code. The pack defines flows (orchestration), agents (workers), and skills (deterministic helpers). Claude Code provides the runtime.

**Platform engineering:** A golden path for AI-assisted development. Opinionated workflow, consistent outputs, self-service dispatch. The `.runs/` directory is the contract between the swarm and the reviewer.

**Agent operations:** The same concerns apply to operating AI agents as to operating services — observability, governance, reproducibility. DemoSwarm maps these:
- Receipts → observability (what ran, what it produced, what evidence exists)
- Gates → governance (secrets, anomalies, merge criteria)
- `.runs/` as committed state → reproducibility (resume, audit, replay)

---

## Operating Model

### Work vs. Publish

- **Work is default-allow:** explore, implement, run tests, iterate freely
- **Publish is gated:** sanitize content before commit/push/post

Gates constrain what leaves the workspace, not what the model can analyze.

### Shadow Fork Topology

Recommended: run flows in a dedicated `*-swarm` repo.

```
my-project/        # human workspace
my-project-swarm/  # swarm workspace (commits freely)
```

The pack's "Deploy" (Flow 6) merges into the swarm repo's `main`. Upstream export is a separate step.

### Learning Loop

Flow 7 writes learnings to `.runs/_wisdom/latest.md`. Flow 1 reads it before starting research.

Institutional memory in plain text, committed with the artifacts.

---

## Docs

| If you want to... | Go here |
|-------------------|---------|
| Get running in 5 minutes | [Quickstart](docs/tutorials/quickstart.md) |
| Customize for your stack | [Customize Pack](docs/how-to/customize-pack.md) |
| Recover from failures | [Failure Recovery](docs/how-to/failure-recovery.md) |
| Look up contracts/schemas | [Contracts](docs/reference/contracts.md) |
| Understand the design | [Architecture](docs/explanation/architecture.md) |
| Browse all docs | [Docs Index](docs/README.md) |

---

## License

Apache-2.0
