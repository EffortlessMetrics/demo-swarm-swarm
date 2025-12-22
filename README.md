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
*   **DemoSwarm** is async. You dispatch a flow, walk away, and return to a receipt that proves the work is correct.

---

## How It Feels: Dispatchable CI/CD

It behaves like a team of **Competent Junior Developers** who know the rules of the repo. It is **not** an always-on background daemon; it is a dispatchable worker.

1.  **Delegate:** You run `/flow-1-signal "Add user auth"` and step away.
2.  **The Grind (Async):** The swarm spins up a bounded run.
    *   It plans the architecture (**Plan**).
    *   It writes tests *before* code (**Build**).
    *   It pushes a Draft PR immediately to wake up CI.
    *   It harvests CI/Bot feedback at checkpoints and fixes critical issues automatically.
    *   It cleans up linting errors and redacts secrets (**Polish**).
3.  **Review:** You return to see a **Green PR** and a **Build Receipt** (`build_receipt.json`).
4.  **Resume:** If the swarm hits context limits, it checkpoints its state to disk. You simply rerun the flow to continue exactly where it left off.

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

### The "Senior Dev" Loop
1.  Run **Flow 1 & 2** to lock in the design.
2.  Run **Flow 3**. It will push a **Draft PR** immediately.
3.  **Walk away.** The swarm will work.
4.  If it stops (PARTIAL status), rerun the flow.
5.  Run **Flow 4** to polish.
6.  Review the **Evidence** (Receipts), not the Code.
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
