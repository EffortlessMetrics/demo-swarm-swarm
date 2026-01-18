# Changelog

All notable changes to the demo-swarm pack will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [Unreleased]

### Added
- **Rules**: Added `10-operating-model.md` and `20-intent-to-narrative.md` to `.claude/rules/`.
- **Docs**: Comprehensive overhaul of `README.md`, including voice/tone guidelines and stability checks.
- **Docs**: Added "Truth Surfaces" and clarified "Authority vs Difficulty" concepts.

### Changed
- **Routing**: Aligned documentation with natural language routing across all agents.
- **Consistency**: Refined completion discipline and routing guidance in rules and flow docs.

---

## [2.2.0] - 2026-01-10

### Added
- **Origin Alignment**: Incorporated provenance additions to ensure alignment with upstream sources.

### Changed
- **Documentation**: Comprehensive overhaul of documentation structure and content.
- **Agents**: Significant refactor of agent responsibilities and interactions.
- **Versioning**: Bumped version to reflect major documentation and agent architecture changes.

---

## [2.1.0] - 2025-12-27

### Changed
- **Routing**: Full transition to natural language handoffs for all agents. Removed rigid JSON-based routing blocks in favor of conversational recommendations.
- **Agent Docs**: Streamlined agent documentation (`.claude/agents/*.md`) to focus on core responsibilities and autonomy.
- **Handoffs**: Refactored specific handoff sections into general "Handoff Guidelines" for better flexibility.
- **Routing Guidance**: Updated routing logic across multiple agents for consistency and clarity.

---

## [2.0.0] - 2025-12-21

**Major Release: Ops-First & Seven-Flow Alignment**

### Added
- **Seven Flows**: Fully implemented and aligned the 7-flow model (Signal, Plan, Build, Review, Gate, Deploy, Wisdom).
- **Obstacle Protocol**: Introduced formal protocol for agents to handle ambiguity and errors (stops "spinning").
- **State-First Verification**: Shifted verification philosophy to rely on current disk state rather than memory.
- **Flow 3 Checkpoints**: Added early checkpoint pushes and PR creation in Build flow for continuous feedback.
- **Run Artifacts**: Added `SKIPPED` stubs for better artifact tracking in cleanup agents.

### Changed
- **Philosophy**: Adopted "Ops-First" philosophy - engineering is default-allow, publishing is gated.
- **Roles**: Redefined Agents as "Junior Developers" and Orchestrators as "PMs".
- **Cleanup**: Updated cleanup agents to be more permissive and artifact-driven.
- **Governance**: Enhanced governance checks in `deploy-decider` and `gh-issue-manager`.
- **Reporting**: Improved GitHub reporting and operational guidelines across all flows.
- **Removed**: Deleted `lint-executor` agent (capabilities moved to `auto-linter` skill).
- **Metric**: Updated "Dev Lead Time" calculation to exclude machine time.

---

## [1.1.0] - 2025-12-19

### Added
- **Local Alignment**: Alignment audit completed for Flows 4, 5, 6, 7.
- **Flow Checkpoints**: Added checkpoints for Signal and Plan flows during audit.

### Changed
- **Sync**: Direct synchronization with upstream `demo-swarm` repository (squash-merges).
- **Cleanup**: Temporarily removed outdated Flow 5/6 documentation before re-introducing aligned versions.

---

## [1.0.1] - 2025-12-13

### Changed

- Documentation cleanup: Updated REPO_MAP.md, CONTRIBUTING.md, DEMO_RUN.md, PR template to reflect pack-only structure
- Updated agent count in docs to use "50+" (avoids drift)
- Version consistency updates: Updated demoswarm-pack-check and demoswarm-runs-tools to version 1.0.1

---

## [1.0.0] - 2025-12-11

### Added

#### Pack Structure

- **Agents** across 8 role families (see `.claude/agents/` for current count)
- **7 flow commands**: `/flow-1-signal` through `/flow-7-wisdom` (Signal, Plan, Build, Review, Gate, Deploy, Wisdom)
- **7 skills**: test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools
- **Pack customizer**: `/customize-pack` for adapting to target stack

#### Flow System

- **Signal flow**: Intent capture, requirements, BDD scenarios, early risks
- **Plan flow**: ADR, API contracts, observability spec, test/work plans
- **Build flow**: Test/code microloops with critic review
- **Gate flow**: Receipt verification, security scan, merge decision
- **Deploy flow**: Deployment verification and monitoring
- **Wisdom flow**: Regression analysis, learnings extraction, feedback loops

#### Infrastructure

- **Receipts-first reporting**: Flows write `*_receipt.json`; reporters read receipts
- **GitHub integration**: One issue per work item, flow summaries as comments
- **Secrets publish gate**: Scans and fixes secrets before publishing
- **Identity model**: Immutable run-ids with alias resolution

#### Documentation

- CLAUDE.md — Complete pack reference for Claude Code
- CONTRIBUTING.md — Contribution guide
- DEMO_RUN.md — Demo walkthrough
- docs/CUSTOMIZATION.md — Stack adaptation guide

---

## Versioning Note

Pack versioning starts at 1.0.0 with the first stable release. This pack was extracted from `demo-swarm`, which has its own version history. Changes to this pack are tracked here; upstream dev repo tags are separate.

For provenance: the dev repo contains additional components (Flow Studio UI, selftest harness, validation tooling) not included in this portable pack.
