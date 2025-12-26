# Repository Map

> Quick structural reference - full version at [docs/reference/repo-map.md](docs/reference/repo-map.md)

---

## Top-Level Structure

```
/ (repo root)
|- .claude/             # Claude Code pack (agents, commands, skills, scripts)
|- .demoswarm/          # Local CLI/tooling build output (.crates*, bin/)
|- .github/             # CI workflow + PR template
|- .runs/               # Demo receipts and index (created by flows)
|- docs/                # Documentation (Diataxis layout)
|- scripts/             # Bootstrap + helper scripts for vendored tooling
|- tools/               # Vendored Rust tooling sources
|
|- README.md            # Repo overview + quick start
|- CLAUDE.md            # Pack reference + contracts
|- CHEATSHEET.md        # One-screen reference
|- ARCHITECTURE.md      # Architecture overview
|- CHANGELOG.md         # Release history
|- CONTRIBUTING.md      # Contribution guidelines
|- CODE_OF_CONDUCT.md   # Code of conduct
|- DEMO_RUN.md          # Demo walkthrough
|- SECURITY.md          # Security policy
|- SUPPORT.md           # Support expectations
|- LICENSE              # Apache-2.0
```

---

## Claude Pack (`.claude/`)

```
.claude/
|- agents/               # Subagent definitions
|- commands/             # Slash-command orchestrators (/flow-*, /customize-pack)
|- skills/               # test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools
|- scripts/              # pack-check.sh + demoswarm.sh entrypoints
|- settings.json         # Claude Code project settings
```

---

## Vendored tooling and bootstrap

```
tools/
|- demoswarm-pack-check/ # Rust pack validator used by pack-check.sh
|- demoswarm-runs-tools/ # Rust helpers for .runs/ receipts/artifacts

scripts/
|- bootstrap.sh          # Build/install vendored tools into .demoswarm/bin
|- check-doc-drift.sh    # Docs drift gate
|- check_portable_claude.py
|- lint_frontmatter.py

.demoswarm/              # Cargo metadata + installed binaries for demoswarm tools
```

---

## Documentation (Diataxis)

```
docs/
|- README.md              # Docs index (router)
|- tutorials/             # Learning-oriented (quickstart, walkthrough, validation-run)
|- how-to/                # Task-oriented (customize-pack, run-topology, upstream-export, work-without-github, adapt-to-non-github, troubleshoot)
|- reference/             # Information-oriented (glossary, contracts, stable-markers, pack-check, repo-map, demoswarm-cli)
|- explanation/           # Understanding-oriented (architecture, why-two-planes, why-two-gates, why-reseal, ai-physics)
|- maintainers/           # Maintainer docs (handover, release-checklist, validation-log, ADR-runner-bounded-reseal)
```

---

## See also

- [docs/reference/repo-map.md](docs/reference/repo-map.md) - Full repo map
- [docs/README.md](docs/README.md) - Documentation index
- [CLAUDE.md](CLAUDE.md) - Pack reference
