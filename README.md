# DemoSwarm

Ops-first SDLC pack for Claude Code.

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

## The Seven Flows

| Flow | Purpose | Key Outputs |
|------|---------|-------------|
| **1. Signal** | Shape intent into contract | Requirements, BDD scenarios, risks |
| **2. Plan** | Design the solution | ADR, contracts, work plan |
| **3. Build** | Implement AC-by-AC | Code, tests, Draft PR |
| **4. Review** | Harvest feedback, fix | Drained worklist, Ready PR |
| **5. Gate** | Forensic audit | MERGE or BOUNCE verdict |
| **6. Deploy** | Merge to swarm mainline | CI verification |
| **7. Wisdom** | Extract learnings | Feedback actions |

---

## Key Concepts

- **Artifacts are the handoff:** `.runs/<run-id>/<flow>/` — chat is transient
- **Gates engage at publish boundaries:** work freely; gates constrain commit/push/post
- **Receipts are logs, not locks:** the git log is the audit trail

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

## License

Apache-2.0
