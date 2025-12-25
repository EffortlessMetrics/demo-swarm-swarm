# Architecture

> **Canonical location:** [docs/explanation/architecture.md](docs/explanation/architecture.md)

This stub exists so that top-level discovery finds the architecture doc. The canonical copy lives in the Diátaxis tree.

---

## Quick summary

DemoSwarm is a Claude Code pack providing:

- **Seven SDLC flows** (Signal → Plan → Build → Review → Gate → Deploy → Wisdom)
- **Narrow agents** (authors, critics, infra, reporters)
- **7 skills** (test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools)

The pack defines the **control plane** (agents + commands). Runtimes and UIs are adapters on top.

For the full architecture explanation, see [docs/explanation/architecture.md](docs/explanation/architecture.md).
