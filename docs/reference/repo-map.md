# Repository Map

> What's where in this repo.

---

## Purpose

This repository contains the portable SDLC swarm pack for Claude Code: agents, commands, and skills that can be copied into a target repo.

This is a **pack-only repo**; no runtime, UI, harness, or application code lives here.

---

## Top-Level Structure

```
/  (repo root)
|-- .claude/               # Claude Code pack (agents, commands, skills, scripts, rules)
|-- .github/               # CI workflow + PR template
|-- docs/                  # Documentation (Diataxis layout)
|-- scripts/               # Bootstrap and validation scripts
|-- tools/                 # Rust CLI tools (pack-check, runs-tools)
|
|-- ARCHITECTURE.md        # Architecture overview (explanation)
|-- CHANGELOG.md           # Pack release history
|-- CHEATSHEET.md          # Quick reference (one-screen)
|-- CLAUDE.md              # Full pack reference (canonical contracts)
|-- CODE_OF_CONDUCT.md     # Community code of conduct
|-- CONTRIBUTING.md        # Contribution guidelines
|-- DEMO_RUN.md            # Demo walkthrough (show the pack)
|-- LICENSE                # Apache-2.0
|-- README.md              # Repository overview + quick start
|-- REPO_MAP.md            # Stub (links here)
|-- SECURITY.md            # Security policy
|-- SUPPORT.md             # Support expectations
|-- demo-swarm.config.template.json  # Configuration template
|
|-- .cspell.json           # Spell-check configuration
|-- .gitattributes         # Git attributes
|-- .gitignore             # Git ignore patterns
`-- .markdownlint.json     # Markdown linting rules
```

---

## Claude Pack: `.claude/`

The portable pack that gets copied into target repos.

```
.claude/
|-- agents/                     # Subagent definitions (84 agents)
|   `-- <agent-name>.md         # YAML frontmatter + prompt
|
|-- commands/                   # Slash command orchestrators
|   |-- customize-pack.md       # /customize-pack
|   |-- flow-1-signal.md        # /flow-1-signal
|   |-- flow-2-plan.md          # /flow-2-plan
|   |-- flow-3-build.md         # /flow-3-build
|   |-- flow-4-review.md        # /flow-4-review
|   |-- flow-5-gate.md          # /flow-5-gate
|   |-- flow-6-deploy.md        # /flow-6-deploy
|   `-- flow-7-wisdom.md        # /flow-7-wisdom
|
|-- hooks/                      # Pre-commit and validation hooks
|   |-- contract_lint.py        # Contract linting
|   `-- gh_outbound_guard.py    # GitHub outbound guard
|
|-- rules/                      # Claude Code rules (constitution)
|   |-- 00-doctrine.md          # Core thesis
|   |-- 10-operating-model.md   # PM + IC swarm
|   |-- 20-intent-to-narrative.md # Pipeline from intent to PR
|   |-- 30-autonomy-and-boundaries.md # Default-allow + gates
|   |-- 40-evidence-and-quality.md # Claims require pointers
|   |-- 50-agent-contract.md    # Agent prompts
|   |-- 60-flow-orchestrators.md # Flow commands
|   |-- 70-docs-and-teaching.md # Documentation
|   |-- 80-developer-experience.md # UX, accessibility
|   `-- 90-voice-and-tone.md    # Communication style
|
|-- scripts/
|   |-- demoswarm.sh            # CLI operations
|   `-- pack-check.sh           # Pack validation
|
|-- skills/                     # Claude Code Skill definitions
|   |-- auto-linter/SKILL.md    # Formatting/linting
|   |-- openq-tools/SKILL.md    # Open questions register
|   |-- policy-runner/SKILL.md  # Policy-as-code checks
|   |-- runs-derive/SKILL.md    # Read-only .runs/ derivations
|   |-- runs-index/SKILL.md     # Index updates
|   |-- secrets-tools/SKILL.md  # Secrets scanning/redaction
|   `-- test-runner/SKILL.md    # Execute tests
|
|-- index.md                    # Pack index
`-- settings.json               # Claude Code project settings
```

---

## Documentation: `docs/`

Organized by Diataxis (intent-based).

```
docs/
|-- README.md                    # Docs index (intent router)
|-- START-HERE.md                # New user starting point
|
|-- tutorials/                   # Learning-oriented (4 files)
|   |-- README.md                # Tutorials index
|   |-- first-swarm-run.md       # First swarm run guide
|   |-- quickstart.md            # Quick start (includes offline mode)
|   |-- validation-run.md        # Pack validation guide
|   `-- walkthrough.md           # Demonstration walkthrough
|
|-- how-to/                      # Task-oriented (22 files)
|   |-- README.md                # How-to index
|   |-- adapt-to-non-github.md   # GitLab/Azure/Bitbucket
|   |-- add-an-agent.md          # Add new agents
|   |-- adopt-fork-workflow.md   # Fork workflow adoption
|   |-- configure-claude-code.md # Claude Code configuration
|   |-- create-a-flow.md         # Create new flows
|   |-- customize-pack.md        # Stack adaptation
|   |-- decompose-work.md        # Work decomposition
|   |-- design-agents.md         # Agent design guide
|   |-- documentation-governance.md # Doc governance
|   |-- failure-recovery.md      # Failure recovery
|   |-- handle-open-questions.md # Open questions handling
|   |-- maintain-the-pack.md     # Pack maintenance
|   |-- orchestrator-decision-tree.md # Orchestrator decisions
|   |-- review-a-swarm-pr.md     # PR review guide
|   |-- run-topology.md          # Swarm repo setup
|   |-- team-operations.md       # Team operations
|   |-- troubleshoot.md          # Debug common issues
|   |-- upstream-export.md       # Export to human repo
|   |-- work-without-github.md   # Local-only operation
|   |-- working-with-microloops.md # Microloop patterns
|   `-- working-with-receipts.md # Receipt patterns
|
|-- reference/                   # Information-oriented (28 files)
|   |-- README.md                # Reference index
|   |-- agent-data-flows.md      # Agent data flows
|   |-- agent-handoff-graph.md   # Agent handoff graph
|   |-- agent-matrix.md          # Agent matrix
|   |-- agent-patterns.md        # Agent patterns
|   |-- agents-index.md          # Agent index
|   |-- calibration-signals.md   # Calibration signals
|   |-- calibration.md           # Calibration guide
|   |-- contracts.md             # Control-plane blocks, enums
|   |-- demoswarm-cli.md         # CLI helper commands
|   |-- documentation-conventions.md # Doc conventions
|   |-- evidence-freshness.md    # Evidence freshness
|   |-- flow-comparison.md       # Flow comparison
|   |-- glossary.md              # Term definitions
|   |-- model-allocation.md      # Model allocation
|   |-- pack-check-scope.md      # Pack check scope
|   |-- pack-check.md            # pack-check and CI
|   |-- pr-quality-scorecard.md  # PR quality scorecard
|   |-- pr-review-interface.md   # PR review interface
|   |-- repo-map.md              # This file
|   |-- routing-table.md         # Routing table
|   |-- run-state.md             # Run state schemas
|   |-- sandbox-threat-model.md  # Sandbox threat model
|   |-- schemas.md               # Schemas reference
|   |-- stable-markers.md        # Marker prefixes
|   |-- trust-model.md           # Trust model
|   `-- visual-style.md          # Visual style guide
|
|-- explanation/                 # Understanding-oriented (53 files)
|   |-- README.md                # Explanation index
|   |-- adversarial-loops.md     # Adversarial loops
|   |-- agent-philosophy.md      # Agent philosophy
|   |-- ai-physics.md            # Sealed stations, entropy
|   |-- anti-patterns.md         # Anti-patterns
|   |-- architecture.md          # Pack design
|   |-- authority-not-difficulty.md # Authority vs difficulty
|   |-- boundary-physics.md      # Boundary physics
|   |-- bounded-fix-forward.md   # Bounded fix-forward
|   |-- candidates-to-artifacts.md # Candidates to artifacts
|   |-- claims-and-evidence.md   # Claims and evidence
|   |-- claude-native-design.md  # Claude-native design
|   |-- code-as-binary.md        # Code as binary
|   |-- codebase-as-mold.md      # Codebase as mold
|   |-- competitive-positioning.md # Competitive positioning
|   |-- context-discipline.md    # Context discipline
|   |-- coordination-by-artifact.md # Coordination by artifact
|   |-- economics.md             # Economics
|   |-- emergent-phenomena.md    # Emergent phenomena
|   |-- flow-composition.md      # Flow composition
|   |-- flow-flexibility.md      # Flow flexibility
|   |-- how-claude-md-works.md   # How CLAUDE.md works
|   |-- human-escalation.md      # Human escalation
|   |-- laws-of-the-swarm.md     # Laws of the swarm
|   |-- operating-model.md       # Operating model
|   |-- operational-philosophy.md # Operational philosophy
|   |-- org-design-as-code.md    # Org design as code
|   |-- pr-as-review-surface.md  # PR as review surface
|   |-- publish-boundaries.md    # Publish boundaries
|   |-- reviewing-as-audit.md    # Reviewing as audit
|   |-- shadow-fork.md           # Shadow fork
|   |-- skills-vs-agents.md      # Skills vs agents
|   |-- state-and-resumption.md  # State and resumption
|   |-- stateless-execution.md   # Stateless execution
|   |-- stochastic-compiler.md   # Stochastic compiler
|   |-- teaching-repo.md         # Teaching repo
|   |-- the-flywheel.md          # The flywheel
|   |-- the-physics.md           # The physics
|   |-- the-thesis.md            # The thesis
|   |-- throughput-inversion.md  # Throughput inversion
|   |-- traceability-spine.md    # Traceability spine
|   |-- trust-architecture.md    # Trust architecture
|   |-- truth-hierarchy.md       # Truth hierarchy
|   |-- verification-stack.md    # Verification stack
|   |-- what-makes-this-different.md # What makes this different
|   |-- why-ops-first.md         # Why ops first
|   |-- why-reseal.md            # Receipt correctness
|   |-- why-seven-flows.md       # Why seven flows
|   |-- why-two-gates.md         # GitHub ops gating
|   |-- why-two-planes.md        # Control vs audit
|   |-- worklist-pattern.md      # Worklist pattern
|   `-- principles/              # Principles subdirectory (19 files)
|       |-- artifacts-reduce-work.md
|       |-- artifacts-with-substance.md
|       |-- composability.md
|       |-- disk-is-memory.md
|       |-- doctrine.md
|       |-- evidence-over-trust.md
|       |-- gate-at-boundaries.md
|       |-- gate-pattern.md
|       |-- graceful-outcomes.md
|       |-- local-resolution.md
|       |-- microloop.md
|       |-- pm-junior-model.md
|       |-- positive-prompting.md
|       |-- real-cognitive-work.md
|       |-- single-responsibility.md
|       |-- system-improves.md
|       |-- truth-flows-downward.md
|       `-- two-reasons-for-agents.md
|
|-- examples/                    # Example artifacts (6 files)
|   |-- README.md                # Examples index
|   |-- build-receipt.json       # Sample build receipt
|   |-- code-critique.md         # Sample code critique
|   |-- merge-decision.md        # Sample merge decision
|   |-- open-questions.md        # Sample open questions
|   `-- pr-cockpit.md            # Sample PR cockpit
|
`-- maintainers/                 # Maintainer docs (5 files)
    |-- README.md                # Maintainers index
    |-- ADR-runner-bounded-reseal.md  # ADR on reseal limits
    |-- handover.md              # Take over maintenance
    |-- release-checklist.md     # Ship new versions
    `-- validation-log.md        # Record validation runs
```

---

## Scripts: `scripts/`

Bootstrap and validation scripts at the repo root.

```
scripts/
|-- bootstrap.sh              # Bootstrap script
|-- check-doc-drift.sh        # Documentation drift checker
|-- check_portable_claude.py  # Portable Claude checker
`-- lint_frontmatter.py       # Frontmatter linting
```

---

## Tools: `tools/`

Rust CLI tools for pack validation and runs operations.

```
tools/
|-- demoswarm-pack-check/     # Pack validation tool (Rust)
|   |-- Cargo.toml
|   |-- src/
|   `-- tests/
|
`-- demoswarm-runs-tools/     # Runs operations tool (Rust)
    |-- Cargo.toml
    |-- src/
    `-- tests/
```

---

## GitHub: `.github/`

CI workflows and PR template.

```
.github/
|-- pull_request_template.md  # PR template
`-- workflows/
    `-- pack.yml              # CI workflow
```

---

## Agent Categories

| Category | Color | Role |
|----------|-------|------|
| Shaping | Yellow | Early signal processing |
| Spec | Purple | Requirements and design |
| Implementation | Green | Writing code/tests/docs |
| Critic | Red | Harsh review (never fixes) |
| Verification | Blue | Checking work / audits |
| Analytics | Orange | Analysis and learning |
| Infra | Cyan | Infrastructure + repo ops |
| Reporter | Pink | GitHub posting |
| Cleanup | Various | Flow finalization / receipts |

---

## Where to change what

| To change... | Edit... |
|--------------|---------|
| Test command | `.claude/skills/test-runner/SKILL.md` |
| Lint command | `.claude/skills/auto-linter/SKILL.md` |
| Flow behavior | `.claude/commands/flow-*.md` |
| Agent behavior | `.claude/agents/<agent>.md` |
| Pack invariants | `CLAUDE.md` |
| Claude rules | `.claude/rules/*.md` |
| Drift validation | `.claude/scripts/pack-check.sh` |
| CLI operations | `.claude/scripts/demoswarm.sh` |
| Human docs | `docs/*` |

---

## Run directories (in target repos)

When the pack runs in a target repo, it creates:

```
.runs/<run-id>/
|-- run_meta.json
|-- signal/
|-- plan/
|-- build/
|-- review/
|-- gate/
|-- deploy/
`-- wisdom/
```

This pack repo does not ship `.runs/` content; `.runs/` is created where flows execute.

---

## See also

- [contracts.md](contracts.md) - Canonical schemas
- [pack-check.md](pack-check.md) - pack-check and CI
- [run-state.md](run-state.md) - Run state schemas
- [agents-index.md](agents-index.md) - Full agent listing
- [CLAUDE.md](../../CLAUDE.md) - Full pack reference
