# Changelog

All notable changes to the demo-swarm pack will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [2.2.1] - 2026-01-14

### Added
- **Voice and Tone rule** (`.claude/rules/90-voice-and-tone.md`): Establishes communication standards—industrial clarity with human warmth
- **Two-pass stability checks** in flow orchestrator rules: Re-review at least once to confirm stability before proceeding

### Changed
- **README overhaul**: Reorganized for clarity, improved navigation, updated examples
- **Examples documentation** (`docs/examples/README.md`): Enhanced guidance and structure

---

## [2.2.0] - 2026-01-12

### Added
- **Claude Code rules** (`.claude/rules/`): Ten constitutional rules encoding core physics and behaviors:
  - `00-doctrine.md`: Core thesis, the triangle, anti-austerity
  - `10-operating-model.md`: PM + IC swarm, when to spawn agents
  - `20-intent-to-narrative.md`: Pipeline from intent to PR
  - `30-autonomy-and-boundaries.md`: Default-allow + strict gates
  - `40-evidence-and-quality.md`: Claims require pointers, the quality panel
  - `50-agent-contract.md`: Agent prompts contract
  - `60-flow-orchestrators.md`: Flow commands contract
  - `70-docs-and-teaching.md`: Documentation philosophy
  - `80-developer-experience.md`: UX, accessibility, quality investment
- **New agents**:
  - `evidence-sufficiency-critic.md`: Evaluates whether evidence meets requirements
  - `intent-auditor.md`: Audits intent artifacts for completeness
  - `merge-reconciler.md`: Handles merge conflict resolution
  - `mold-improver.md`: Improves codebase patterns for better generation
  - `review-cockpit-designer.md`: Designs PR review surfaces
- **Enhanced skills**: Expanded documentation for `test-runner`, `auto-linter`, and `policy-runner` with detailed usage guidance

### Changed
- **CLAUDE.md alignment**: Updated to reference rules as the constitution
- **All flow commands**: Added rule references for consistent behavior
- **Cleanup agents**: Enhanced with ledger maintenance patterns
- **Agent handoffs**: Aligned to natural language routing throughout

---

## [2.1.0] - 2026-01-10

### Added
- **New analyst agents**:
  - `maintainability-analyst.md`: Analyzes code maintainability
  - `pattern-analyst.md`: Identifies codebase patterns
  - `process-analyst.md`: Analyzes development processes
  - `quality-analyst.md`: Evaluates quality metrics
  - `signal-quality-analyst.md`: Assesses signal artifact quality
  - `solution-analyst.md`: Analyzes solution approaches
  - `spec-auditor.md`: Audits specifications for completeness
- **Problem framer agent** (`problem-framer.md`): Frames problems for effective solution design

### Changed
- **Comprehensive agent refactor**: All 70+ agents updated for clarity, consistency, and focused responsibilities
- **Handoff standardization**: Converted structured handoff blocks to natural language prose throughout
- **Agent documentation**: Streamlined for clarity, autonomy, and scope management
- **Routing terminology**: Updated from "Handoff Sections" to "Handoff Guidelines" for clarity
- **Flow commands**: Enhanced with TodoWrite guidance and explore agent references

### Removed
- **mutator.md**: Removed (functionality consolidated elsewhere)

---

## [2.0.0] - 2025-12-25

### Added
- **Seven-flow model**: Consolidated from previous flow structure
- **Ops-First philosophy**: State-first verification approach emphasizing repo state as primary truth
- **Obstacle Protocol**: Guides code and test authors through ambiguity and errors
- **GitHub reporting enhancements**: Improved operational guidelines across all flows
- **Early checkpoint push**: Flow 3 now creates draft PRs early for continuous feedback
- **SKIPPED stubs**: Deploy and wisdom cleanup agents now track skipped artifacts
- **Standards enforcer safety checks**: Added detection for silent test deletions

### Changed
- **Flow rhythm documentation**: Enhanced explanations of compressor stations and the "Feel" test
- **Agent roles clarified**: Agents positioned as Junior Developers with clear maintainer responsibilities
- **Dev Lead Time calculation**: Now estimates developer effort excluding machine and wait time
- **PR Feedback Harvester**: Enhanced with anti-reward-hacking measures and better routing logic
- **Gate Result structure**: Updated block structure and semantics for clarity
- **Cleanup agents**: Reflect Ops-First philosophy with permissive cleanup and artifact requirements
- **repo-operator**: Enhanced with intent-based operations and extras handling
- **Flow documentation**: Eliminated reseal loops, clarified execution order
- **Review worklist writer**: Supports grouped markdownlint MINOR items
- **Debug artifact handling**: Improved clarity across agents

### Fixed
- **Markdown formatting issues**: Resolved across review documentation
- **Flow execution order**: Clarified to eliminate confusion about reseal loops

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
