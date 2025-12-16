# DemoSwarm

> SDLC pack for Claude Code: Signal -> Plan -> Build -> Gate -> Deploy -> Wisdom  
> Produces inspectable artifacts under `.runs/<run-id>/` and routes via returned result blocks.

**Important:** Run DemoSwarm in a dedicated swarm clone. It creates `run/<run-id>` branches and commits `.runs/` artifacts as an audit trail. Flow 5 promotes to the swarm mainline. Open PRs back to your upstream repo when ready.

[Documentation](docs/README.md) | [Quickstart](docs/tutorials/quickstart.md) | [Tutorials](docs/tutorials/README.md) | [Maintenance](docs/maintainers/README.md)

---

DemoSwarm is a portable "pack" (Agents + Commands + Skills) that keeps Claude Code workflows structured and auditable.

- **Receipts-first:** Every flow produces a JSON receipt; reporters summarize from receipts (no recomputation).
- **Two-gate safety:** GitHub operations require both a Secrets Gate and a Repo Hygiene Gate.
- **Deterministic tooling:** Counts and extractions are performed by shimmed tooling (Rust preferred for determinism/reliability; Python supported).

---

## Quick start (reference run)

Run this repository as a self-contained demo to see the agents in action. The first run bootstraps tooling; later runs are faster.
---
**Prerequisites:** Claude Code, Git (2.25+), Rust 1.89+ or Python 3.8+, and `bash` (Git Bash/WSL2 on Windows). `gh` is optional; GitHub ops skip if unauthenticated.

### 1) Clone and bootstrap
```bash
gh repo clone EffortlessMetrics/demo-swarm
cd demo-swarm

bash scripts/bootstrap.sh
```

### 2) Run a toy Signal flow (GitHub-optional)
Open Claude Code in this directory:
```bash
claude
```

Run the canonical Signal:
```text
/flow-1-signal toy-run "Add a 'demoswarm version' CLI command that prints JSON (version, git_sha, build_time)."
```

### 3) Inspect artifacts
Look in `.runs/toy-run/signal/`:
- `signal_receipt.json`
- `requirements.md`
- `open_questions.md`
- `secrets_status.json`

See `docs/tutorials/toy-run.md` for the full walkthrough.

---

## Adopt DemoSwarm in your project

Use a dedicated `<repo>-swarm` clone (the pack creates `run/<run-id>` branches and commits `.runs/`). Copy the pack, bootstrap, and customize for your stack.

- Start with **`docs/tutorials/quickstart.md`** for installation steps.
- Then run `/customize-pack` (see `docs/how-to/customize-pack.md`) to align tests/lint/policy with your repo.

---

## Core concepts

### The six flows
| Flow | Command | Input -> Output |
| :--- | :--- | :--- |
| 1. Signal | `/flow-1-signal` | Intent -> requirements, BDD, risks, receipt |
| 2. Plan | `/flow-2-plan` | Spec -> ADR, contracts, observability, plans, receipt |
| 3. Build | `/flow-3-build` | Design -> code/tests + build receipt |
| 4. Gate | `/flow-4-gate` | Code -> verdict (**MERGE/BOUNCE/ESCALATE**) + gate receipt |
| 5. Deploy | `/flow-5-deploy` | Verdict -> promote to swarm mainline (or NOT_DEPLOYED) + deploy receipt |
| 6. Wisdom | `/flow-6-wisdom` | Run history -> regressions, learnings, feedback + terminal receipt |

### The "sealed station" pattern
Agents do not pass context via chat history (which drifts). They pass context via artifacts.
- **Flow 1** writes `requirements.md`.
- **Flow 2** reads `requirements.md` from disk.
- If Flow 2 crashes, you can restart it anytime because the state is on disk.

### The two planes
1. **Audit plane (files):** Markdown files (`adr.md`) for humans to read.
2. **Control plane (blocks):** Machine-parsable YAML blocks (`## Gate Result`) for the orchestrator to route logic (e.g., "If `safe_to_publish: false`, BOUNCE").

---

## Tooling and safety

This pack includes a lightweight CLI (`demoswarm`) to ensure agents cannot fabricate metrics.

### Usage
Agents call the shim. You do not need to manage paths.
```bash
bash .claude/scripts/demoswarm.sh <command>
```

### Validation
Ensure your pack has not drifted (e.g., after editing prompts):
```bash
bash .claude/scripts/pack-check.sh
```

### GitHub ops
- **Auth:** Requires `gh` CLI.
- **Safety:** GitHub operations are skipped if `gh` is not authenticated.
- **Gating:** Even with auth, the swarm refuses to push/post if the Secrets Gate or Repo Hygiene Gate fails.

---

## Documentation

| Guide | Content |
| :--- | :--- |
| [**Quickstart**](docs/tutorials/quickstart.md) | Detailed first-run guide |
| [**Walkthrough**](docs/tutorials/walkthrough.md) | Full demo script for presenters |
| [**CLI Reference**](docs/reference/demoswarm-cli.md) | Full `demoswarm` command reference |
| [**Contracts**](docs/reference/contracts.md) | Receipt schemas and stable markers |
| [**Maintenance**](docs/maintainers/handover.md) | How to modify and release this pack |

---

## License

Apache-2.0
