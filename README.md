# DemoSwarm

SDLC pack for Claude Code that produces **review-ready PRs**: code + executed evidence + critiques, surfaced as a PR cockpit in the GitHub PR description.

---

## Why This Exists

The job is moving up the stack. Again.

Punchcards → Assembly → High-level languages → **Now**.

Models now emit and revise nearly-working implementation at machine speed—10x to 500x faster than humans can review. The bottleneck isn't "can it write code." It's *trust*: can a human decide **ship / don't ship** quickly, without reading every line?

Most AI coding tools optimize the wrong thing. They make generation faster, which produces more code for humans to review. The backlog grows. The bottleneck tightens.

DemoSwarm optimizes for what actually matters: **better PR per dev touchpoint**. Spend cheap iteration (critics, tests, revision loops) to buy down uncertainty, so a human touchpoint is spent on judgment, not grinding.

---

## What DemoSwarm Does

DemoSwarm turns Claude Code into a repeatable pipeline:

- **Breaks larger requests into smaller, composable steps** — so the model doesn't lose coherence on multi-file changes
- **Runs oppositional validation inside the loop** — critics and tests catch problems before you see the PR
- **Produces a Draft PR early** to wake bots (CodeRabbit, CI), then harvests their feedback into fixes
- **Leaves humans with a PR that's further along** than what a single prompt can produce

This is not "AI that codes for you." It's a workflow that manufactures trust.

---

## Try It

```text
/customize-pack                              # once per repo
/flow-1-signal "Add a health check endpoint" # start a run
```

Flow 1 produces requirements and BDD scenarios. Continue with `/flow-2-plan` → `/flow-3-build` to get a Draft PR.

Flow 3 opens the Draft PR to wake bots. Flow 4 harvests PR feedback. Flow 5 writes the merge decision. The GitHub PR page is where you review.

**[Quickstart](docs/tutorials/quickstart.md)** · [CLAUDE.md](CLAUDE.md)

---

## What You Review

**The PR description is your primary interface.** DemoSwarm aims to make the GitHub PR page sufficient for a fast, confident decision.

A good PR cockpit answers:

1. **What changed** — bounded scope, intent summary
2. **What ran** — tests, linters, scanners that actually executed
3. **What critics found** — and what was fixed
4. **What's not measured** — explicit gaps, not silent ones
5. **Where to deepen verification** — hotspots for escalation if doubt exists

Hotspots are **not** "where humans line-read." Hotspots are where you escalate verification (targeted tests, scans, mutation/fuzz) if the evidence isn't convincing. Critics apply the adversarial pressure; humans arbitrate based on executed evidence.

---

## Truth Surfaces

Where the real answers live:

| Surface | Location | What It Proves |
|---------|----------|----------------|
| **PR cockpit** | PR description | Primary review UI: scope, proof pointers, explicit unknowns |
| **Gate verdict** | `.runs/<run-id>/gate/merge_decision.md` | Ship or bounce (with rationale) |
| **Test proof** | `.runs/<run-id>/build/test_execution.md` | Tests actually ran (exit codes, not claims) |
| **Critiques** | `.runs/<run-id>/build/*_critique.md` | What critics found and prioritized |
| **Receipts** | `.runs/<run-id>/*/*_receipt.json` | Mechanical summaries + evidence pointers |
| **Diff** | GitHub PR diff | Final audit surface when evidence raises doubt |

The PR cockpit is the default. `.runs/` is drill-down evidence and resumability.

**Want a UI for runs?** [Flow Studio](https://github.com/EffortlessMetrics/flow-studio-swarm) renders `.runs/` into an operator view.

---

## The Seven Flows

| Flow | Purpose | Output |
|------|---------|--------|
| **1. Signal** | Shape intent into contract | requirements, BDD scenarios |
| **2. Plan** | Design before code | ADR, contracts, work plan |
| **3. Build** | Implement with critics in the loop | code, tests, Draft PR |
| **4. Review** | Harvest PR feedback, fix | Ready PR |
| **5. Gate** | Final checks | MERGE or BOUNCE |
| **6. Deploy** | Merge to main | CI verification |
| **7. Wisdom** | Extract learnings | feedback for next run |

Each flow breaks work into focused tasks. Agents handle one thing—`code-implementer` writes code, `test-author` writes tests, `code-critic` reviews—then hand off with a clear recommendation.

---

## The Mentality Shift

Most AI tools optimize for **generation speed**. DemoSwarm optimizes for **verification speed**—how quickly can a human decide yes/no with confidence?

The machine iterates. Critics apply pressure. Tools produce executed evidence. Humans arbitrate based on what's proven—and escalate verification where doubt exists.

**Completion states:**
- **VERIFIED** — Evidence is green and fresh. Blockers empty. Done.
- **UNVERIFIED** — Checkpointed state. Artifacts written, next steps documented, resumable.
- **CANNOT_PROCEED** — Mechanical failure (tooling broken, permissions missing).

UNVERIFIED means "checkpointed"—the next step is routing, not blame.

---

## How Agents Work

Agents are specialists. Each does one thing well:

| Agent | What It Does |
|-------|--------------|
| `code-implementer` | Writes implementation code |
| `test-author` | Writes tests from BDD scenarios |
| `code-critic` | Reviews code, finds issues |
| `test-critic` | Reviews tests for coverage gaps |
| `repo-operator` | Handles git operations |

They work autonomously—research, decide, implement, fix what they encounter—then hand off: *"Did X, found Y, recommend routing to Z next."*

If a flow can't finish (missing info, failing tests), it checkpoints. Rerun the same flow to resume.

---

## Operating Model

### Work vs. Publish

- **Work is default-allow:** explore, implement, run tests, iterate freely
- **Publish is gated:** sanitize before commit/push/post

Gates constrain what leaves the workspace, not what the model can analyze.

### Routing Is Prose

Agents recommend next steps in plain language. Orchestrators route by reading handoffs and choosing what makes sense. No brittle routing blocks to parse.

Example handoff:
> "Implemented 3 of 5 endpoints. Remaining 2 need the User schema. Route to code-implementer with User schema first."

---

## Repo Layout

| Location | What's There |
|----------|--------------|
| `.claude/commands/` | Flow orchestrators |
| `.claude/agents/` | Workers, critics, auditors, operators |
| `.claude/skills/` | Deterministic helpers (test-runner, linter) |
| `.runs/` | Run artifacts (in your repo) |

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

## Author

Created by Steven Zimmerman (@EffortlessSteven) · [effortlesssteven.com/demoswarm](https://effortlesssteven.com/demoswarm/)

This pack encodes the AgOps posture: trade cheap iteration for expensive review time. Verification is the product.

---

## License

Apache-2.0
