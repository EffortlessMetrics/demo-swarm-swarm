# Lint Report

## Machine Summary

status: UNVERIFIED
recommended_action: BOUNCE
route_to_flow: 2
route_to_agent: pack-customizer
blockers:
  - No Markdown linting command configured or available in environment
  - auto-linter SKILL.md is Rust-focused; no Rust code changes in this build
  - .markdownlint.json exists but no compatible runner installed
missing_required:
  - demo-swarm.config.json with lint/format commands (not found)
  - Markdown linter binary or npm package (not available)
concerns:
  - This is documentation-only build; need customizer to define appropriate Markdown linting strategy
lint_summary:
  mode: check
  format_command: null
  format_exit_code: null
  lint_command: null
  lint_exit_code: null
  files_modified: false
  touched_paths: []

## Inputs Used

- .markdownlint.json (configuration file present, no commands defined)
- .cspell.json (spell checker config, no commands defined)
- auto-linter SKILL.md (Rust-focused; not applicable to this build)
- Changed files snapshot via git diff (7 Markdown files changed; no Rust code)

## Execution

- tool: auto-linter (invoked; routed to lint-executor)
- mode: check
- format: null → no formatter configured
- lint: null → no linter configured

## Canonical Summary (tool-bound)

- No linting commands defined for this repository.
- Configuration files exist (.markdownlint.json, .cspell.json) but no automated invocation path.
- Build is documentation-only (Markdown files): CHANGELOG.md, CONTRIBUTING.md, DEMO_RUN.md, README.md, docs/explanation/architecture.md, docs/how-to/work-without-github.md, docs/reference/glossary.md
- No Rust code changes in this build (SKILL.md would normally run cargo fmt + cargo clippy; not applicable here).
- Environment has npm and Rust tooling, but no Markdown linter runner installed.

## Failures (if any)

- No linting was attempted due to missing command definitions.
- Cannot proceed without pack-customizer defining Markdown linting strategy and commands.

## Notes

- This repository has linting configuration files (.markdownlint.json, .cspell.json) but no execution mechanism is defined in demo-swarm.config.json.
- For documentation-only builds, the pack customizer should define:
  - Format command (e.g., markdownlint with --fix if desired)
  - Lint command (e.g., markdownlint --config .markdownlint.json)
  - Optional: spell-check via cspell
- Once configured, lint execution can resume.
