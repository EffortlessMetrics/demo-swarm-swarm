# Changelog

All notable changes to the demo-swarm pack will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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
