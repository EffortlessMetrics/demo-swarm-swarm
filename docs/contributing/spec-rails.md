# Contributing to DemoSwarm Spec Rails

When adding or updating durable planning and specification artifacts:

1. Place durable artifacts under `.demoswarm-spec/`.
2. Keep each artifact type focused:
   - proposal = why and alternatives
   - spec = behavioral contract and proof obligations
   - ADR = durable architecture decision
   - lane tracker = execution state and work-item progression
   - implementation plan = PR-sized sequence
   - closeout = what landed, proof, and remaining work
3. Link artifacts through `.demoswarm-spec/index.toml`.
4. Do not put durable rails inside agent or tool state directories.

## Non-owned directories for this lane

The following directories are awareness-only and not managed by this system:

- `.codex/`
- `.spec/`
- `.claude/`
- `.jules/`

If these directories are present, leave them untouched unless a separate lane explicitly governs them.
