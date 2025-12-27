# DemoSwarm

SDLC pack for Claude Code.

**Core constraint:** Tokens are cheap; reviewer attention is the bottleneck.

**What it produces:** A PR with evidence (tests, receipts, critiques) you can review in minutes, not hours.

---

## Start Here

```text
/customize-pack                              # Once per repo
/flow-1-signal "Add a health check endpoint" # Your first run
```

Then open:
- `.runs/<run-id>/signal/requirements.md` — the contract
- `.runs/<run-id>/signal/open_questions.md` — assumptions needing validation

If the contract is wrong, rerun Flow 1. Fixing the spec is cheaper than fixing a bad build.

**Full setup:** [Quickstart](docs/tutorials/quickstart.md) · **Reference:** [CLAUDE.md](CLAUDE.md)

---

## What This Actually Is

DemoSwarm is a `.claude/` pack (agents + flows + skills) that turns Claude Code into a repeatable build pipeline. You dispatch flows explicitly. Agents write artifacts to disk. The filesystem is the record, chat is transient.

This isn't "AI that codes for you." It's a factory floor where the LLM is one worker among many, supervised by critics, auditors, and gates.

### The Economic Inversion

Traditional tooling optimizes for **generation speed**—how fast can the AI produce code?

DemoSwarm optimizes for **verification speed**—how quickly can a human decide yes/no with confidence?

We spend machine cycles (stubborn loops, parallel critics, multiple verification passes) to produce evidence that humans can skim. The output isn't just code; it's code + tests + receipts + critiques + a clear audit trail.

### The Trust Model

We treat LLM agents as **enthusiastic junior developers**: eager, tireless, prone to guessing when pressured.

The response isn't to constrain them—it's to **verify their work**:

- **Trust the worker.** Give agents autonomy to research, decide, and fix issues they encounter.
- **Verify the work.** Trust the git diff and test results, not the agent's prose about what it did.
- **Catch problems early.** Critics run inside build loops, not just at the end.

This is the same stance good teams take with humans: if the change matters, you want tests, diffs, and a clear story.

### Why Artifacts Matter

Most AI coding tools optimize the chat experience. DemoSwarm treats chat as exhaust.

Every flow writes to `.runs/<run-id>/<flow>/`:
- **Requirements and BDD scenarios** (what we're building)
- **Design decisions and contracts** (how we're building it)
- **Test execution logs** (proof it works)
- **Critiques and reviews** (what the critics found)
- **Receipts** (mechanical summaries for downstream flows)

These artifacts are the handoff. A new session can read `.runs/` and understand exactly what happened, what's left, and what was decided. No context window archaeology required.

### Honest State Over False Completion

LLMs under pressure tend to guess rather than admit uncertainty. DemoSwarm removes that pressure:

- **`PARTIAL` is a win.** If an agent hits context limits, it saves state to disk and exits. That's a save point, not a failure.
- **Reruns are routine.** Every flow is designed to resume from disk state. Kill the terminal, restart, and continue where you left off.
- **Honest reporting beats completion theater.** A 40% completion with documented blockers is more valuable than a 100% completion with hidden assumptions.

---

## The Seven Flows

| Flow | Purpose | Key Outputs |
|------|---------|-------------|
| **1. Signal** | Shape intent into contract | Requirements, BDD scenarios, risks |
| **2. Plan** | Design the solution | ADR, contracts, AC matrix, work plan |
| **3. Build** | Implement AC-by-AC | Code, tests, Draft PR, build receipt |
| **4. Review** | Harvest feedback, fix | Drained worklist, Ready PR |
| **5. Gate** | Forensic audit | MERGE or BOUNCE verdict |
| **6. Deploy** | Merge to swarm mainline | CI verification |
| **7. Wisdom** | Extract learnings | Feedback actions, scent trail |

Each flow is a station on the conveyor. You turn the crank (`/flow-N-...`), agents do the work, artifacts get written. The human role is architect + foreman, not typist.

---

## How This Is Different

### From "AI coding assistants"

Most AI coding tools are interactive—you prompt, it responds, you iterate in real-time.

DemoSwarm is batch-oriented. You dispatch a flow, it grinds (sometimes for hours), and you come back to a finished artifact with evidence. The model is closer to CI than to pair programming.

### From "autonomous agents"

DemoSwarm doesn't ship to prod automatically. It produces a **review-ready PR** with evidence. A human makes the merge decision.

The goal isn't autonomy—it's **reviewer readiness**. Can a senior engineer skim the receipts and confidently approve in 10 minutes?

### From typical SDLC tooling

Traditional SDLC tooling (Jira, Linear, etc.) tracks *what humans decided*. DemoSwarm tracks *what the machine did and why*.

Every decision has an artifact. Every artifact has evidence. The `.runs/` directory is a forensic record of the entire build, not just the outcome.

---

## Operating Model

### Work vs. Publish

- **Work is default-allow:** Explore, implement, run tests, iterate freely.
- **Publish is gated:** Sanitize content before commit/push/post.

Gates don't constrain thinking. They constrain what leaves the workspace.

### Shadow Fork Topology

We recommend running flows in a dedicated `*-swarm` repo:

```
my-project/        # Human workspace (stays calm)
my-project-swarm/  # Swarm workspace (commits freely)
```

The swarm works in isolation. When upstream moves, you rebase and rerun verification. Machine time is the solution to integration drift.

### The Learning Loop

Flow 7 (Wisdom) writes learnings to `.runs/_wisdom/latest.md`. Flow 1's researcher reads this file before starting research.

The factory self-patches. If Run #10 failed because of Library X, Run #11 knows to avoid Library X. No vector database required—just institutional memory in plain text.

---

## Key Concepts

- **Artifacts are the handoff:** `.runs/<run-id>/<flow>/` — chat is transient
- **Gates engage at publish boundaries:** work freely; gates constrain commit/push/post
- **Receipts are logs, not locks:** the git log is the audit trail
- **PARTIAL is a save point:** rerun the same flow to resume
- **Quality over speed:** asset value before efficiency

**More:** [Architecture](docs/explanation/architecture.md) · [Trust Model](docs/reference/trust-model.md)

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

## What You're Getting Into

This is an opinionated system. It assumes:

- You want verified code, not fast code.
- You're willing to spend machine cycles to save human review time.
- You value audit trails and reproducibility.
- You're comfortable with batch processing (dispatch and wait) over interactive chat.

If you want a chat-based coding assistant, this isn't it. If you want a factory that produces review-ready PRs with evidence, keep reading.

---

## License

Apache-2.0
