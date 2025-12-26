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
|-- .claude/               # Claude Code pack (agents, commands, skills, scripts)
|-- .github/               # CI workflow + PR template
|-- docs/                  # Documentation (Diataxis layout)
|
|-- ARCHITECTURE.md        # Architecture overview (explanation)
|-- CHANGELOG.md           # Pack release history
|-- CHEATSHEET.md          # Quick reference (one-screen)
|-- CLAUDE.md              # Full pack reference (canonical contracts)
|-- CONTRIBUTING.md        # Contribution guidelines
|-- DEMO_RUN.md            # Demo walkthrough (show the pack)
|-- LICENSE                # Apache-2.0
|-- README.md              # Repository overview + quick start
|-- REPO_MAP.md            # Stub (links here)
`-- SUPPORT.md             # Support expectations
```

---

## Claude Pack: `.claude/`

The portable pack that gets copied into target repos.

```
.claude/
|-- agents/                     # Subagent definitions
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
|-- skills/                     # Claude Code Skill definitions
|   |-- test-runner/SKILL.md    # Execute tests
|   |-- auto-linter/SKILL.md    # Formatting/linting
|   |-- policy-runner/SKILL.md  # Policy-as-code checks
|   |-- runs-derive/SKILL.md    # Read-only .runs/ derivations
|   |-- runs-index/SKILL.md     # Index updates
|   |-- openq-tools/SKILL.md    # Open questions register
|   `-- secrets-tools/SKILL.md  # Secrets scanning/redaction
|
|-- scripts/
|   `-- pack-check.sh           # Canonical pack validation
|
`-- settings.json               # Claude Code project settings
```

---

## Documentation: `docs/`

Organized by Diataxis (intent-based).

```
docs/
|-- README.md                    # Docs index (intent router)
|
|-- tutorials/                   # Learning-oriented
|   |-- README.md                # Tutorials index
|   |-- quickstart.md            # Quick start (includes offline mode)
|   |-- validation-run.md        # Pack validation guide
|   `-- walkthrough.md           # Demonstration walkthrough
|
|-- how-to/                      # Task-oriented
|   |-- README.md                # How-to index
|   |-- customize-pack.md        # Stack adaptation
|   |-- run-topology.md          # Swarm repo setup
|   |-- upstream-export.md       # Export to human repo
|   |-- work-without-github.md   # Local-only operation
|   |-- adapt-to-non-github.md   # GitLab/Azure/Bitbucket
|   `-- troubleshoot.md          # Debug common issues
|
|-- reference/                   # Information-oriented
|   |-- README.md                # Reference index
|   |-- glossary.md              # Term definitions
|   |-- contracts.md             # Control-plane blocks, enums
|   |-- stable-markers.md        # Marker prefixes
|   |-- repo-map.md              # This file
|   |-- pack-check.md            # pack-check and CI
|   `-- demoswarm-cli.md         # CLI helper commands for .runs/ operations
|
|-- explanation/                 # Understanding-oriented
|   |-- README.md                # Explanation index
|   |-- architecture.md          # Pack design
|   |-- ai-physics.md            # Sealed stations, entropy
|   |-- why-two-planes.md        # Control vs audit
|   |-- why-two-gates.md         # GitHub ops gating
|   `-- why-reseal.md            # Receipt correctness
|
`-- maintainers/                 # Maintainer docs
    |-- README.md                # Maintainers index
    |-- handover.md              # Take over maintenance
    |-- release-checklist.md     # Ship new versions
    |-- validation-log.md        # Record validation runs
    `-- ADR-runner-bounded-reseal.md  # ADR on reseal limits
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
| Drift validation | `.claude/scripts/pack-check.sh` |
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
- [CLAUDE.md](../../CLAUDE.md) - Full pack reference
