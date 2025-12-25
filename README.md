# DemoSwarm

Ops-first SDLC pack for Claude Code. Trades machine iteration for lower human verification time.

---

## Start Here

```text
/customize-pack                              # Once per repo
/flow-1-signal "Add a health check endpoint" # Your first run
```

Then open `.runs/<run-id>/signal/requirements.md` — that's the contract the swarm built from your one-liner.

**Full setup:** [Quickstart](docs/tutorials/quickstart.md) · **Reference:** [CLAUDE.md](CLAUDE.md)

---

## What This Is

A `.claude/` pack (agents, flows, skills) plus CLI tooling for deterministic receipts.

- You dispatch flows explicitly (no daemon)
- Agents hand off via `.runs/<run-id>/` artifacts (chat is transient)
- Publishing is gated; quality issues are surfaced and routed to rework

**Primary metric:** DevLT — minutes a human spends verifying the change.

---

## The Seven Flows

| Flow | Purpose | Key Outputs |
|------|---------|-------------|
| **1. Signal** | Shape messy intent into a contract | Requirements, BDD scenarios, risks |
| **2. Plan** | Design the solution | ADR, contracts, AC matrix, work plan |
| **3. Build** | Implement AC-by-AC, push early | Code + tests, Draft PR |
| **4. Review** | Harvest feedback, batch fixes | Drained worklist, Ready PR |
| **5. Gate** | Forensic audit, merge verdict | MERGE or BOUNCE decision |
| **6. Deploy** | Merge to swarm mainline | CI/smoke verification |
| **7. Wisdom** | Extract learnings, close loops | Learnings, feedback actions |

**Typical cadence:** Signal → Plan → Build → Review → Gate → Deploy → Wisdom → integrate to upstream.

---

## Operating Model

### Artifacts are the handoff

Each flow writes to `.runs/<run-id>/<flow>/`. These artifacts are the record — chat is transient. If a flow exits `PARTIAL`, rerun the same command; it resumes from disk state.

### Gates engage at publish boundaries

Work freely (read files, write code, run tests). Gates only engage at **commit / push / GitHub posting**:

- **Secrets**: Sanitized in-place; blocks only when manual remediation required
- **Anomalies**: Test deletions block push; extras (typo fixes) are staged automatically
- **Mechanical failures**: Block publish, not work

If a gate blocks, keep working locally. Gates constrain publishing, not thinking.

**More:** [Architecture](docs/explanation/architecture.md) · [Trust Model](docs/reference/trust-model.md)

### Merge permission comes from Flow 5

Per-flow status (`VERIFIED` / `UNVERIFIED` / `PARTIAL`) describes evidence quality. The **Gate verdict** (MERGE or BOUNCE) is what decides if the change ships.

Receipts are logs, not locks. The git log is the audit trail.

**More:** [Contracts](docs/reference/contracts.md)

---

## Quickstart (5 minutes)

**Prereqs:** Claude Code, Git, bash. `gh` CLI recommended.

### 1. Create a swarm repo

```bash
# Fork approach (GitHub PR integration)
gh repo fork <org>/<repo> --fork-name <repo>-swarm --clone
cd <repo>-swarm

# Or clone approach (simpler, local-only)
git clone <repo-url> <repo>-swarm && cd <repo>-swarm
```

**More:** [Fork Workflow](docs/how-to/adopt-fork-workflow.md)

### 2. Install the pack

```bash
cp -r /path/to/demo-swarm/.claude .
cp -r /path/to/demo-swarm/tools .
cp -r /path/to/demo-swarm/scripts .
bash scripts/bootstrap.sh
git add .claude/ && git commit -m "feat: Add DemoSwarm pack"
```

### 3. Customize and run

```text
/customize-pack
/flow-1-signal "Add a health check endpoint"
```

### 4. Verify

```bash
ls .runs/*/signal/
# requirements.md, features/, signal_receipt.json
```

**Full guide:** [Quickstart Tutorial](docs/tutorials/quickstart.md)

---

## Upstream Integration

Flow 6 merges to **swarm `origin/main`**, not upstream. Export to upstream is a human decision.

| Upstream state | Action |
|----------------|--------|
| Barely moved | Open PR from swarm branch to upstream `main` |
| Moved materially | Sync fork, rebase, rerun Flows 4–7, then PR |

**More:** [Upstream Export](docs/how-to/upstream-export.md)

---

## Docs Map (Diataxis)

| If you want to... | Go here |
|-------------------|---------|
| Get running in 5 minutes | [Quickstart](docs/tutorials/quickstart.md) |
| Walk through a full run | [Walkthrough](docs/tutorials/walkthrough.md) |
| Set up fork workflow | [Adopt Fork Workflow](docs/how-to/adopt-fork-workflow.md) |
| Customize for your stack | [Customize Pack](docs/how-to/customize-pack.md) |
| Recover from failures | [Failure Recovery](docs/how-to/failure-recovery.md) |
| Look up CLI commands | [CLI Reference](docs/reference/demoswarm-cli.md) |
| Look up receipt schemas | [Contracts](docs/reference/contracts.md) |
| Understand the architecture | [Architecture](docs/explanation/architecture.md) |
| Browse all docs | [Docs Index](docs/README.md) |

---

## License

Apache-2.0
