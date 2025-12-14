# Support & Maintenance

> Expectations for engaging with the demo-swarm pack.

---

## What This Is

demo-swarm is a **portable SDLC pack** for Claude Code: agents, commands, and skills you copy into your repo and customize.

This is a **pattern library**, not a product or service.

---

## Maintenance Posture

Maintainers:

- Read issues (bugs, questions, feature requests)
- Do not promise SLAs — this is a pack repo, not a hosted service
- Bias toward changes that improve:
  - Pack reliability (contracts, drift guards, safer defaults)
  - Clarity (docs, error messages, receipts)
  - Adoptability (examples, onboarding)

---

## Before You Open an Issue

Run canonical validation from repo root:

```bash
bash .claude/scripts/pack-check.sh
```

If your issue is about a flow run, include the run-id and the relevant `.runs/` artifacts. Most "it didn't post / it didn't push" reports are explained by the gates below.

### Two gates for GitHub operations (common pitfall)

GitHub operations (issue creation, posting comments, pushing) are allowed only when **both** are true:

- `safe_to_publish: true` (from `secrets-sanitizer` Gate Result)
- `proceed_to_github_ops: true` (from `repo-operator` Repo Operator Result)

If either gate is false, the flow should still write artifacts and receipts, but external ops will be skipped and logged.

---

## How to Engage

### Found a Bug?

Open an issue with:

- What you expected vs what happened
- Steps to reproduce
- Which flow/agent
- Your environment (OS + shell: WSL2/Git Bash/PowerShell; Claude Code version if relevant)
- The smallest set of files needed to inspect:
  - `bash .claude/scripts/pack-check.sh` output
  - `.runs/<run-id>/<flow>/*_receipt.json`
  - `.runs/<run-id>/<flow>/secrets_status.json` (audit record)
  - `.runs/<run-id>/<flow>/git_status.md` (if present)
  - Any returned control-plane blocks you captured in the session (Gate Result / Repo Operator Result)

### Have a Question?

Open an issue with:

- What you're trying to do
- What you tried
- Your stack + repo layout (language, test runner, where code/tests live)
- Whether you ran `/customize-pack` (and what it inferred)

Good questions:

- "How do I wire this into CI?"
- "Can I adapt it for GitLab instead of GitHub?"
- "How do I customize test/lint commands safely?"

### Security issues / secret exposure

If you believe you found a vulnerability or a pattern that could leak secrets, don't post credentials or tokens in a public issue.
Open a minimal issue that says "security report" (no details), and we'll respond with a safer path to share reproduction.

### Want to Contribute?

PRs are welcome. Before submitting:

1. Run `bash .claude/scripts/pack-check.sh` — must pass
2. Update docs if you change behavior or contracts
3. Prefer small, focused changes over large refactors

Good PR targets:

- Doc clarifications / typo fixes
- Agent prompt improvements
- Bug fixes with evidence (before/after artifacts)
- New examples in `docs/`

Not accepting (without discussion):

- Large architectural changes
- New agent types without a clear use case and contract
- Changes that break pack-wide enums / routing contracts

---

## What You Can Expect

| Type                           | Response                            |
| ------------------------------ | ----------------------------------- |
| Bug in agent/command contracts | Prioritized; we'll try to reproduce |
| Doc clarity issues             | Usually quick to merge              |
| Feature requests               | Considered case-by-case             |
| "Can you build X for me?"      | No — this is a pattern library      |

---

## See Also

- [README.md](README.md) — Quick start
- [CLAUDE.md](CLAUDE.md) — Full pack reference
- [docs/README.md](docs/README.md) — Documentation index
- [docs/how-to/customize-pack.md](docs/how-to/customize-pack.md) — Stack adaptation
- [docs/tutorials/validation-run.md](docs/tutorials/validation-run.md) — Validation run guide
- [docs/how-to/troubleshoot.md](docs/how-to/troubleshoot.md) — Troubleshooting
