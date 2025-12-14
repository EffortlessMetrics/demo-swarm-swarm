# Security Policy

## Supported Versions

This repository is a pack definition plus local tooling. Only the latest release is supported with security fixes.

| Version | Supported          |
| ------- | ------------------ |
| 1.x     | :white_check_mark: |
| < 1.0   | :x:                |

## Scope

This pack includes:

- Agent prompts and flow orchestration (`.claude/`)
- Local CLI tooling (`tools/demoswarm-pack-check`, `tools/demoswarm-runs-tools`)
- Scripts and shims (`.claude/scripts/`)

Security issues in the pack may include:

- Secrets leaking through artifacts or outputs
- Unsafe command execution patterns
- Broken access control in ownership boundaries
- Vulnerabilities in Rust CLI tooling

## Reporting a Vulnerability

Please **do not** open public issues for security reports.

**Email:** security@demo-swarm.dev (or file a private security advisory on GitHub)

**Subject:** `[demo-swarm] Security report`

**Include:**

- What you found and why it matters
- How to reproduce (minimal steps)
- Any proof-of-concept (redacted; no real secrets)
- Your environment (OS, shell, Claude Code version)

## Disclosure Process

1. We will acknowledge receipt within **72 hours**
2. We will validate and triage within **7 days**
3. We will coordinate a fix and release timeline with you
4. We will credit you in the release notes (unless you prefer anonymity)

## Security Expectations

### Secrets handling

The `secrets-tools` skill is designed with strict output contracts:

- **Never prints secret content** to stdout, stderr, or artifacts
- **Only outputs**: file path, line number, secret type
- **Redacts in-place** with type-tagged placeholders

Violations of these contracts are treated as security bugs.

### Publish gates

All flows enforce two gates before GitHub operations:

- `safe_to_publish: true` (secrets gate)
- `proceed_to_github_ops: true` (repo hygiene gate)

If either gate fails, external operations are skipped.

### Local-only execution

This pack runs **entirely locally** in your repo. It does not:

- Send data to external services (other than GitHub API for issue/PR operations)
- Store credentials (uses your existing git/gh configuration)
- Execute remote code

## See Also

- [SUPPORT.md](SUPPORT.md) — General support expectations
- [CLAUDE.md](CLAUDE.md) — Pack contracts and non-negotiables
- [`.claude/skills/secrets-tools/SKILL.md`](.claude/skills/secrets-tools/SKILL.md) — Secrets handling reference
