# DemoSwarm

SDLC pack for Claude Code.

---

## The Shift

Models now produce nearly-working code faster than humans can review it—10x to 500x human reading speed. A model can generate, test, critique, and revise a change in the time it takes a human to read the first draft.

This changes the economics. Generation and iteration are cheap. Human review time is expensive. The bottleneck isn't "how fast can we produce code"—it's "how fast can a human decide yes or no with confidence."

Most AI coding tools optimize the wrong thing. They make generation faster, which just produces more code for humans to review. The backlog grows.

DemoSwarm optimizes for what actually matters: **better PR per dev touchpoint**. Spend tokens on iteration—critics, tests, revision loops—so by the time a human sees the PR, it's already been through the wringer. Review becomes confirmation, not discovery.

---

## How It Works

You dispatch a flow. The flow spawns agents. Agents do focused work and hand off. Each step is scoped to what the model handles well. State accumulates on disk. By the end, you have a Draft PR that's been through:

- **Requirements** — shaped from your initial request
- **Design** — ADR, contracts, work plan
- **Implementation** — code written against the design
- **Testing** — tests written from BDD scenarios
- **Critics** — code-critic and test-critic reviewing the output
- **Self-review** — final check before surfacing to you

The iteration happens in tokens, not in your calendar. When you review, you're reviewing the output of a process, not the first draft of a prompt.

---

## Why Orchestration

LLMs do well on small, scoped tasks. Give a model a clear, contained problem and it produces reasonable code. But larger tasks—multi-file features, end-to-end flows, anything that requires holding multiple constraints in mind—lose coherence in a single prompt.

The model forgets earlier constraints. It produces pieces that don't fit together. It drifts from the original intent. This isn't a model quality problem; it's a context problem. The task exceeds what fits in a single generation.

The fix isn't better prompts or larger context windows. It's breaking the work down:

1. **Scope each step** — give the model what it needs for this task, nothing more
2. **Accumulate state** — write artifacts to disk so nothing gets lost
3. **Run critics in the loop** — surface issues early, when fixing is cheap
4. **Hand off clearly** — each agent says what it did and what should happen next

This is how you get larger changes through without the model losing the thread.

---

## Oppositional Validation

Single-pass generation lies to please. The model produces something plausible, you accept it, problems surface later.

DemoSwarm runs critics inside the build loop. `code-critic` reviews implementation. `test-critic` reviews tests. They're adversarial—their job is to find problems, not to agree.

When a critic finds issues, the orchestrator routes to a fixer. The fixer addresses the issues. The critic runs again. This continues until the critic passes or the orchestrator decides to surface the remaining issues to a human.

The result: problems that would have shown up in human review—or worse, in production—get caught and fixed in the loop. The PR that reaches you has already survived scrutiny.

---

## Try It

```text
/customize-pack                              # once per repo
/flow-1-signal "Add a health check endpoint" # start a run
```

Continue with `/flow-2-plan` → `/flow-3-build` to get a Draft PR.

**[Quickstart](docs/tutorials/quickstart.md)** · [CLAUDE.md](CLAUDE.md)

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

Each flow breaks work into focused tasks. Agents handle one thing—`code-implementer` writes code, `test-author` writes tests, `code-critic` reviews—then hand off. State accumulates on disk, so larger changes don't exceed what the model can hold in context.

---

## What You Review

The PR description is your interface. The swarm produces a summary: what changed, what was tested, what to look at if something seems off.

For most reviews: read the description, skim the diff, approve or comment. That's the goal—a PR that's ready enough that review is confirmation, not discovery.

If you need to debug or audit:
- `.runs/<run-id>/` contains the working artifacts (requirements, plans, critiques)
- Receipts summarize what each flow did
- **[Flow Studio](https://github.com/EffortlessMetrics/flow-studio-swarm)** renders `.runs/` into a visual cockpit

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

They work autonomously—research, decide, implement, fix what they encounter—then hand off with a clear recommendation: *"Did X, found Y, recommend routing to Z next."*

If a flow can't finish (missing info, failing tests), it checkpoints. Rerun the same flow to resume where it left off.

---

## Repo Layout

| Location | What's There |
|----------|--------------|
| `.claude/commands/` | Flow orchestrators |
| `.claude/agents/` | Agent prompts (specialists) |
| `.claude/skills/` | Deterministic tools (test-runner, linter) |
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

---

## License

Apache-2.0
