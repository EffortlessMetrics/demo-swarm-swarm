# Spec Style and Source-of-Truth Rails

DemoSwarm keeps a full source-of-truth chain:

- roadmap
- proposal / PRD
- spec
- ADR (when needed)
- lane tracker + implementation plan
- PRs and issues
- proof commands and receipts
- support-tier or policy updates
- closeout

## Durable location

The durable repo-native knowledge base lives in:

- `.demoswarm-spec/`

This is the long-term source of truth for spec artifacts.

## Human-facing explanation

Human-readable contributor guidance belongs in `docs/`.

In particular:

- this file explains the method
- `docs/contributing/spec-rails.md` explains contribution workflow

## External tool state

Tool-specific folders are **not** durable rails for this system:

- `.codex/`
- `.spec/`
- `.claude/`
- `.jules/`

They may coexist, but this spec lane does not own, migrate, or depend on their internal state.
