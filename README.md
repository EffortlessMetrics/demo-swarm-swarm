# DemoSwarm

Ops-first SDLC pack for Claude Code. Trades machine iteration for lower human verification time.

**Docs:** [docs/README.md](docs/README.md) | [Architecture](docs/explanation/architecture.md) | [Quickstart](docs/tutorials/quickstart.md)

---

## What This Is

DemoSwarm is a `.claude/` pack (agents, flows, skills) plus deterministic tooling for receipts and counts.

You run flows explicitly (no daemon). Each flow produces on-disk artifacts under `.runs/` that become the handoff between agents and your review surface.

**Primary metric:** DevLT (Developer Lead Time) — minutes a human spends verifying the change.

## What This Is Not

- Not a hosted service
- Not "always-on" automation
- Not a replacement for code review — it produces better evidence to make review cheaper

---

## How It Works

Dispatch a flow (e.g., `/flow-1-signal "Add user auth"`). The swarm grinds, checkpoints, writes artifacts.

When a flow ends, you skim the artifacts (and optionally GitHub updates), then decide what to run next.

**Artifacts are the handoff.** Chat is transient; `.runs/<run-id>/` is the record.

### If Something Is Incomplete

Flows are rerunnable. If a flow exits `PARTIAL`, rerun the same command. It resumes from disk state.

---

## Planes and Gates

### Work Plane (default: allow)

Agents explore, build, and test freely:
- Read any files, search code, run checks
- Write tests early, iterate on code
- Push early to get bot feedback (CI, CodeRabbit)
- Ad-hoc human edits ("extras") are accepted and recorded, not treated as failure
- Security findings here are advisory, not throttles

### Publish Plane (default: gate)

Gates engage only at the boundary: **commit / push / GitHub posting**.

- **Secrets**: Sanitized in-place before publishing. Only blocks when manual remediation required.
- **Anomalies**: Suspicious changes (test deletions, unexpected staged files) may block publish depending on policy; always surfaced, never blocks local work.
- **Mechanical failures** (missing tools, auth issues, IO errors) block publish, not work.

If a gate blocks, keep working locally. Gates constrain publishing, not thinking.

---

## Status Semantics

These statuses describe **evidence quality**, not "permission to merge."

| Status | Meaning |
|--------|---------|
| **VERIFIED** | Artifacts exist, verification stations ran and passed, evidence is consistent |
| **UNVERIFIED** | Verification incomplete, contradictions, critical failures, or missing core outputs |
| **PARTIAL** | Real progress made, but flow checkpointed (context/time limit). Rerun to continue |

**Merge permission comes from Flow 5 (Gate), not from a per-flow status label.**

**Receipts are logs, not locks.** The git log is the audit trail; receipts summarize what happened at stations.

---

## Setup (Once Per Repo)

Run `/customize-pack` to detect your stack and configure skills:

```text
/customize-pack
```

This writes `demo-swarm.config.json` with test/lint commands for your stack. Rerun when commands change.

---

## The Seven Flows

| Flow | Purpose | Key Outputs |
|------|---------|-------------|
| **1. Signal** | Turn messy intent into a contract | Requirements, BDD scenarios, risks, open questions |
| **2. Plan** | Turn contract into buildable work | ADR, interfaces/contracts, AC matrix, work plan |
| **3. Build** | Implement AC-by-AC, push early | Code + tests, build receipt, Draft PR |
| **4. Review** | Harvest feedback, batch fixes | Drained worklist, Ready PR |
| **5. Gate** | Forensic audit, merge verdict | MERGE or BOUNCE decision, gate receipt |
| **6. Deploy** | Merge to swarm mainline | Merge + CI/smoke verification, deploy receipt |
| **7. Wisdom** | Extract learnings, close loops | Learnings, feedback actions, scent trail |

**Flow 6 scope:** Merges the run's feature branch into swarm `origin/main`. Does NOT merge to upstream. Upstream integration is a separate, post-Wisdom step.

---

## Quickstart

**Prerequisites:** Claude Code (`claude`), Git, bash. `gh` CLI recommended for PR/issue integration.

### 1. Set Up the Swarm Repo

Work in a downstream clone or fork (recommended):

```bash
# Fork approach (GitHub PR integration)
gh repo fork <org>/<repo> --fork-name <repo>-swarm --clone
cd <repo>-swarm

# Or clone approach (simpler, local-only)
git clone <repo-url> <repo>-swarm
cd <repo>-swarm
```

### 2. Install the Pack

If you don't have it yet:

```bash
# Copy from demo-swarm or install per your org's method
cp -r /path/to/demo-swarm/.claude .
git add .claude/ && git commit -m "feat: Add DemoSwarm pack"
```

### 3. Customize for Your Stack

See [Setup](#setup-once-per-repo) above. Run `/customize-pack` to configure for your stack.

### 4. Run a Flow

```text
/flow-1-signal "Add a health check endpoint"
```

### 5. Inspect the Evidence

Look at artifacts in `.runs/<run-id>/`:

| Artifact | What It Is |
|----------|------------|
| `signal/requirements.md` | The contract |
| `plan/adr.md` | The architecture decision |
| `build/build_receipt.json` | The evidence |
| `build/ac_status.json` | Progress checkpoint |

---

## The Dispatch Rhythm

```text
/customize-pack           # Once per repo
/flow-1-signal "..."      # Shape the work → skim requirements, answer questions
/flow-2-plan              # Design → approve ADR
/flow-3-build             # Build → Draft PR; if PARTIAL, rerun
/flow-4-review            # Drain worklist → Ready PR; if PARTIAL, rerun
/flow-5-gate              # Audit → MERGE or BOUNCE verdict
/flow-6-deploy            # Merge to swarm main, verify
/flow-7-wisdom            # Extract learnings
```

After Flow 7: Review wisdom outputs, then integrate to upstream when ready.

---

## Repo Topology

Recommended: run in a downstream swarm clone/fork.

```
my-project/              # Human workspace (stays calm)
my-project-swarm/        # Swarm workspace (commits freely)
```

Benefits:
- **Inspectability**: `.runs/` artifacts committed and reviewable
- **Isolation**: Swarm activity doesn't disrupt human development
- **Clean export**: Open PR from swarm to origin when ready

See: [adopt-fork-workflow.md](docs/how-to/adopt-fork-workflow.md)

---

## Upstream Integration

DemoSwarm does NOT automatically merge to upstream. After Wisdom, integration depends on how much upstream has moved.

### If upstream barely moved

Feature branch applies cleanly:

1. Open PR from swarm feature branch to upstream `main`
2. Done

### If upstream moved materially

Requires restabilization:

1. Sync swarm `origin/main` with upstream `main`
2. Rebase feature branch onto updated swarm main
3. Rerun Flows 4–7 to restabilize
4. Open PR from swarm to upstream when ready

This keeps the human-in-the-loop for cross-repo integration.

---

## Documentation

| Guide | Content |
|-------|---------|
| [Quickstart](docs/tutorials/quickstart.md) | First-run guide |
| [Architecture](docs/explanation/architecture.md) | Ops-First philosophy and design |
| [Contracts](docs/reference/contracts.md) | Receipt schemas and stable markers |
| [CLI Reference](docs/reference/demoswarm-cli.md) | `demoswarm` command reference |
| [Fork Workflow](docs/how-to/adopt-fork-workflow.md) | Setting up swarm repo |
| [Failure Recovery](docs/how-to/failure-recovery.md) | Handling stuck flows |

---

## License

Apache-2.0
