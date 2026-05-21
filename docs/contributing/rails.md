# Contributing to Rails artifacts

When adding or updating Rails artifacts in this repository:

1. Keep durable framework state under `.rails/`.
2. Keep human-facing explanation docs under `docs/`.
3. Link all owned artifacts through `.rails/index.toml`.
4. Use focused lane trackers under `.rails/lanes/` instead of a global active queue.
5. Keep external namespaces awareness-only (`.codex/`, `.spec/`, `.claude/`, `.jules/`).

## Do not mutate external namespaces

This lane does not migrate, rewrite, or validate:

- `.codex/`
- `.spec/`
- `.claude/`
- `.jules/`

Those directories can be referenced in `.rails/index.toml` as external namespaces only.
