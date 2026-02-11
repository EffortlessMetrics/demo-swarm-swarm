# DemoSwarm Context

This repository contains the **DemoSwarm** pack for Claude Code. It is a collection of agents, commands, and skills designed to produce review-ready PRs with executed evidence.

**Crucial Distinction:** This is the *definition* repository for the pack. It contains the logic that gets copied into target repositories to enable Swarm capabilities. It does not contain the target application code itself.

## Project Overview

DemoSwarm optimizes for **verification speed** over generation speed. It structures the SDLC into "Flows" where specialized agents work to produce code, tests, and critiques, resulting in a "PR Cockpit" that allows humans to review high-level evidence rather than just line-by-line code.

### Core Concepts

*   **Flows:** Logical stages of work (Signal, Plan, Build, Review, Gate, Deploy, Wisdom).
*   **Agents:** Specialized sub-agents (e.g., `code-implementer`, `test-critic`) defined in `.claude/agents/`.
*   **Skills:** Deterministic tools (e.g., `test-runner`, `auto-linter`) defined in `.claude/skills/`.
*   **Orchestrators:** High-level commands (e.g., `/flow-1-signal`) that coordinate agents.
*   **The "Cockpit":** The PR description serves as the primary interface for review, populated with evidence from the run.

## Repository Structure

*   **.claude/**: The core pack definitions.
    *   `agents/`: Markdown definitions for specialist agents.
    *   `commands/`: Markdown definitions for flow orchestrators (slash commands).
    *   `skills/`: Tools and helpers invoked by agents.
    *   `rules/`: Constitutional rules governing agent behavior.
    *   `scripts/`: Internal scripts for the pack (e.g., `pack-check.sh`).
*   **docs/**: Comprehensive documentation following the Diátaxis framework.
    *   `explanation/`: Deep dives into architecture and philosophy.
    *   `reference/`: Contracts, schemas, and CLI reference.
    *   `how-to/`: Guides for customization and troubleshooting.
*   **scripts/**: Development and validation scripts for *this* repository.

## Development & Validation

Since this is a pack repository, "building" means validating the integrity of the pack definitions.

### Key Commands

*   **Validate Pack:**
    ```bash
    bash .claude/scripts/pack-check.sh
    ```
    This runs comprehensive validation on agent frontmatter, file structures, and flow definitions.

*   **Lint Python Scripts:**
    ```bash
    ruff check scripts/
    ```

*   **Check Portability:**
    ```bash
    python scripts/check_portable_claude.py
    ```

### Contribution Guidelines

*   **Agents:** Defined in `.claude/agents/`. Must include specific frontmatter (name, description, color, model).
*   **Colors:** strict mapping between agent color and role family (e.g., Green = Implementation, Red = Critic).
*   **Docs:** Follow `docs/reference/documentation-conventions.md`.

## Usage (In Target Repos)

While this is the source repo, the pack is intended to be used via slash commands in a target repository:

1.  `/customize-pack`: Sets up the pack in a new repo.
2.  `/flow-1-signal "intent"`: Starts the flow.
3.  `/flow-2-plan`, `/flow-3-build`, etc.: Progress through the SDLC.

## References

*   **Operational Policy:** `CLAUDE.md` (The "Constitution")
*   **Architecture:** `docs/explanation/architecture.md` (pointer from `ARCHITECTURE.md`)
*   **CLI Reference:** `docs/reference/demoswarm-cli.md`
