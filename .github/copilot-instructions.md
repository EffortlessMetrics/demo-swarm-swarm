# Copilot instructions for demo-swarm pack

Purpose

- Short, action-oriented guidance for AI-assisted sessions (Copilot/agents) working in this repo.

Quick commands (pack-centric)

- Validate the pack (full validation):
  - bash .claude/scripts/pack-check.sh
- Use the runs/ shim (always invoke via shim):
  - bash .claude/scripts/demoswarm.sh <command> [OPTIONS]
  - Note: the shim unsets DEMOSWARM_STRICT for agent-safe behavior; use `--strict` or `DEMOSWARM_STRICT=1` for human debugging.
- Install local helpers (developer convenience):
  - cargo install --path tools/demoswarm-pack-check --root .demoswarm
  - cargo install --path tools/demoswarm-runs-tools --root .demoswarm

Testing (how to run tests)

- This is a "pack-only" repository (agents, skills, docs). The canonical validation is `pack-check.sh`.
- When working with a target repo (where .runs/ and application code live), common single-test commands follow framework conventions:
  - Rust (single test):
    - cargo test <full::path::to::test>
    - cargo test --test <integration_test_name>
  - Python (single test function):
    - pytest tests/unit/test_module.py::test_name -v
    - pytest tests/unit/test_module.py -v
  - JavaScript/TS (single test file):
    - npm test -- --colors tests/auth.test.ts
    - npm test -- --testNamePattern="login"
- Fallback detection used by the test-runner skill: presence of Cargo.toml, package.json, pyproject.toml, or go.mod triggers respective commands.

Linting / Formatting

- Auto-linter shim behavior and examples (see .claude/skills/auto-linter/SKILL.md):
  - Rust: cargo fmt --all; cargo clippy --all-targets --all-features
  - Python: black src/ tests/; ruff check src/ tests/; ruff check --fix
  - JS/TS: npx prettier --write "src/**/*.{js,ts,jsx,tsx}"; npx eslint --fix
- Prefer repository-provided scripts or package.json/Makefile targets when present (they encode project-specific config).

High-level architecture (big picture)

- Purpose: this repo is a portable "pack" for Claude Code (agents, commands, skills) that gets copied into target repos; it is *not* an application repository.
- Key directories:
  - .claude/ — agents (.md), commands (.md), skills (SKILL.md), scripts (pack-check.sh, demoswarm.sh)
  - docs/ — Diátaxis documentation
  - .runs/ — NOT present here; in target repos this stores run artifacts: `.runs/<run-id>/<flow>/`
  - tools/ and .demoswarm/ — place for Rust/CLI implementations (preferred runtimes)
- Flow model: seven flows exposed as slash commands (/flow-1-signal → /flow-7-wisdom). Each flow writes receipts and artifacts; receipt files (e.g., *_receipt.json) are canonical machine-readable signals.
- Shims: always call bash .claude/scripts/demoswarm.sh and .claude/scripts/pack-check.sh rather than assuming binaries on PATH; shims standardize behavior across platforms.

Key conventions (repo-specific)

- Agents vs Skills:
  - Agents perform decision-making and authoring (natural language handoffs). Skills are deterministic mechanical helpers (tests, linters, policy checks) — keep responsibilities separate.
- Git ownership: `repo-operator` handles git/GitHub side effects; other agents should not perform pushes directly.
- Null-over-guess: demoswarm CLI prints `null` for missing/unparseable results (shim normally exits 0). Use `--strict`/DEMOSWARM_STRICT for debug-mode non-zero exit codes.
- Frontmatter & agent files:
  - Agent files in `.claude/agents/` require YAML frontmatter (2-space indentation). Required fields: name, description, color, model (often `inherit`).
  - Color → role-family mapping is meaningful (see CONTRIBUTING.md); keep color choices consistent with role family semantics.
- Validation: after editing `.claude/*` or agents/skills/commands, run `bash .claude/scripts/pack-check.sh` to validate structure and frontmatter.
- Receipts-first reporting: flows write `*_receipt.json` and other artifacts under `.runs/<run-id>/<flow>/` — use these files for machine parsing rather than natural-language claims.
- Docs & style: Markdown = GitHub-flavored; YAML uses 2-space indentation; lines ~<= 120 chars where practical.

Where to look (authoritative files)

- CLAUDE.md — pack-level policy, rules, and flow model (primary source for agent behavior)
- .claude/commands/* — flow orchestrators (what slash commands do)
- .claude/skills/*/SKILL.md — canonical test/lint/policy command examples and detection rules
- .claude/scripts/pack-check.sh and .claude/scripts/demoswarm.sh — shims to use for tooling
- CONTRIBUTING.md — developer guidance for editing agents, colors, frontmatter, and local validation
- .github/workflows/pack.yml — CI for pack validation

Notes for Copilot sessions

- Prioritize reading CLAUDE.md and `.claude/skills/*/SKILL.md` before making changes that affect agents or tooling.
- When proposing edits to `.claude/agents/*.md` or `.claude/commands/*.md`, include: what changed, which skill/agent owns behavior, and run `bash .claude/scripts/pack-check.sh` in your description.
- Do not make git/GitHub ops; suggest `repo-operator` actions instead.

Questions or adjustments

If you'd like, add MCP server configuration (Playwright, etc.) for the target repo type — skip if not applicable.

---

Generated from: README.md, CONTRIBUTING.md, CLAUDE.md, .claude/skills/* and .claude/scripts (pack-check & demoswarm)