# Rails framework footprint

This repository uses `.rails/` as the durable Rails knowledge base.

## Ownership model

- `.rails/` is the durable Rails knowledge base.
- `docs/` explains Rails conventions and adoption to humans.
- `.codex/` is Codex execution state and is not owned by Rails.
- `.spec/` is Spec Kit / speckit state and is not owned by Rails.
- `.claude/` and `.jules/` are external agent/session spaces and are not owned by Rails.

## Artifact model

Rails artifacts are linked through `.rails/index.toml`.

- proposals describe why work exists.
- specs describe required behavior and evidence.
- ADRs capture durable architecture decisions.
- lanes provide focused implementation trackers.
- closeouts record what shipped, what proved it, and what remains.

No Rails-owned artifact path may live under `.codex/`, `.spec/`, `.claude/`, or `.jules/`.
