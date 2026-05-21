# DemoSwarm Spec Rails (`.demoswarm-spec`)

This directory is the **durable, repo-owned source-of-truth control plane** for DemoSwarm specification work.

It owns long-term artifacts such as:

- proposals (why)
- specs (what)
- ADRs (decision)
- lane trackers and implementation plans (how)
- support claim mapping and policy references (what users may believe / what is enforced)
- closeouts (what happened)

## Scope and ownership

The `.demoswarm-spec/` namespace is owned by this repository and is tool-neutral.

External and agent-specific directories are awareness-only for this lane and are **not** owned by this system:

- `.codex/`
- `.spec/`
- `.claude/`
- `.jules/`

Those locations may exist and may read from this namespace, but durable rails live here.
