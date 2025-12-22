# DemoSwarm

> **Stop Babysitting. Start Shipping.**
> The Ops-First SDLC pack for Claude Code that trades tokens for trusted change.

[Documentation](docs/README.md) | [Quickstart](docs/tutorials/quickstart.md) | [Architecture](docs/explanation/architecture.md)

---

**Tokens are cheap. Senior Developer attention is expensive.**

Most AI coding tools are "Chatbots"—you type, watch them generate, fix their typos, and paste the code. You are the compiler.

**DemoSwarm** is an **Agentic CI/CD Pipeline** that runs locally in your terminal. It uses adversarial multi-agent swarms to **manufacture trust** through evidence (tests, plans, critiques) before you ever see the PR.

### The Promise: Buy Back Your Attention
We optimize for **Dev Lead Time (DevLT)**—minimizing the minutes *you* spend verifying changes, even if the machine takes longer to generate them.

*   **Copilot** is autocomplete. You watch it work.
*   **DemoSwarm** is async (within a flow). You dispatch, walk away, and return to a receipt with a status: `VERIFIED`, `UNVERIFIED`, or `PARTIAL`.

---

## How It Feels: You Are the Tech Lead

It behaves like a **team of well-instructed Junior Developers** working from your playbook. You dispatch each flow; there's no always-on background daemon.

1.  **Dispatch a flow:** You run `/flow-1-signal "Add user auth"`.
2.  **It grinds inside that run:** Agents loop, write artifacts, and (where relevant) push checkpoint commits and harvest CI/bot feedback.
3.  **Quick phase-boundary skim:** You review the receipt + key questions/summary (locally under `.runs/` and, if enabled, in GitHub issue/PR comments).
4.  **Dispatch the next flow:** The pack tells you what to run next; you choose when.

### What "walk away" actually means

You can walk away **while a flow is executing** (especially Flow 3/4). If the run hits a context/time budget, it checkpoints to disk and exits `PARTIAL` with the next command to continue.

### Typical loop

*   **Flow 1** writes requirements + open questions → you skim/answer → run **Flow 2**
*   **Flow 2** writes ADR + AC matrix → you skim/agree → run **Flow 3**
*   **Flow 3** pushes early, harvests feedback, interrupts for CRITICAL blockers → ends with `build_receipt.json` (VERIFIED/UNVERIFIED/PARTIAL)
*   **Flow 4** drains the worklist (may require reruns if PARTIAL) → ends with a Ready PR
*   **You merge.**

**The "extra wait" is a feature.** You trade machine time for attention. While the swarm grinds through the build loop or drains the review worklist, you are free.

---

## The Philosophy: Ops-First

We resolve the tension between "Velocity" and "Safety" by splitting the world in two:

### 1. The Work Plane (Default: ALLOW)
*   **High Velocity:** Agents explore, code, and test freely.
*   **Ad-Hoc Reality:** If you fix a typo while the swarm runs, the swarm accepts it ("Extras") instead of crashing.
*   **Advisory Security:** Findings during the build loop are suggestions, not blockers.

### 2. The Publish Plane (Default: GATE)
*   **Hard Boundaries:** We only gate at the border (`commit`, `push`, `gh post`).
*   **Mechanical Safety:** We don't ask agents to "be good." We enforce it mechanically.
    *   **Anti-Reward Hacking:** If an agent deletes a test to make the build pass, the push is **physically blocked** by the `repo-operator`.
    *   **Fix-First Sanitization:** Secrets are redacted in-place automatically. No bureaucracy.

---

## Quick Start

Run this repository as a self-contained demo to see the agents in action.

**Prerequisites:** Claude Code (`claude`), Git, `bash`, `gh` CLI.

### 1. Bootstrap
```bash
# Clone the swarm repo (runs in isolation)
gh repo clone EffortlessMetrics/demo-swarm my-feature-swarm
cd my-feature-swarm

# Install the shim (ensures deterministic counting)
bash scripts/bootstrap.sh
```

### 2. Run a Flow
Open Claude Code and give it a task.

```bash
claude
```

```text
/flow-1-signal "Add a CLI command 'demoswarm version' that prints the git sha."
```

### 3. Inspect the Evidence
Don't read the chat. Look at the **Artifacts** in `.runs/`:
*   `.runs/<id>/signal/requirements.md` (The Contract)
*   `.runs/<id>/plan/adr.md` (The Decision)
*   `.runs/<id>/build/build_receipt.json` (The Proof)

---

## The Seven Flows

We map the software lifecycle to 7 stateful flows. Context is passed via **Artifacts** on disk.

| Flow | Vibe | Input -> Output |
| :--- | :--- | :--- |
| **1. Signal** | *Discovery* | Messy intent -> **Requirements**, BDD scenarios, Risk profile. |
| **2. Plan** | *Architecture* | Requirements -> **ADR**, Contracts, Observability Spec. |
| **3. Build** | *Velocity* | Spec -> **Draft PR**. *(Pushes early. Checks "Pulse" (CI status). Interrupts only for fires.)* |
| **4. Review** | *Rigor* | Draft PR -> **Ready PR**. *(Unbounded loop. Harvests CodeRabbit/CI feedback. Drains worklist.)* |
| **5. Gate** | *Audit* | PR -> **Merge Verdict**. *(Verifies evidence/receipts, not vibes.)* |
| **6. Deploy** | *Reality* | Verdict -> **Production**. *(Distinguishes "Failed" from "Governance Invisible".)* |
| **7. Wisdom** | *Learning* | History -> **Retrospective**. *(Feeds learnings back into the pack.)* |

---

## Usage Guide

### Adopting in your Repo
DemoSwarm is designed to run in a **downstream clone** (e.g., `my-app-swarm`). It creates `run/<run-id>` branches and commits `.runs/` artifacts as an audit trail.

1.  **Clone** your repo.
2.  **Copy** the `.claude/` folder from this pack.
3.  **Run** `/customize-pack` to auto-detect your stack (Rust/Python/Node) and align the test runners.

### The Dispatch Rhythm
1.  `/flow-1-signal` → skim requirements, answer open questions
2.  `/flow-2-plan` → approve ADR
3.  `/flow-3-build` → Draft PR created; if `PARTIAL`, rerun
4.  `/flow-4-review` → worklist drained; if `PARTIAL`, rerun
5.  `/flow-5-gate` → MERGE or BOUNCE verdict
6.  Review **receipts** (the evidence), not raw code
7.  **Merge.**

---

## Documentation

| Guide | Content |
| :--- | :--- |
| [**Quickstart**](docs/tutorials/quickstart.md) | Detailed first-run guide |
| [**Walkthrough**](docs/tutorials/walkthrough.md) | Full demo script for presenters |
| [**Architecture**](docs/explanation/architecture.md) | Ops-First philosophy and design patterns |
| [**CLI Reference**](docs/reference/demoswarm-cli.md) | Full `demoswarm` command reference |
| [**Contracts**](docs/reference/contracts.md) | Receipt schemas and stable markers |
| [**Maintenance**](docs/maintainers/handover.md) | How to modify and release this pack |

---

## License

Apache-2.0
