# Sandbox Threat Model

Containment requirements for running Claude Code with `bypassPermissions: true`.

---

## Purpose

This document defines the containment checklist that makes high-autonomy operation safe. When all items are satisfied, `bypassPermissions: true` can be used with confidence.

**The sandbox is infrastructure. Treat it like production security, not like a development convenience.**

---

## The Threat Model

Running `bypassPermissions` means:

- Agents execute commands without per-action approval
- The system relies on containment and boundaries, not prompts
- Safety comes from the environment, not the model

### Threats Addressed

| Threat | Example | Mitigation |
|--------|---------|------------|
| Destructive commands | `rm -rf /`, `git push --force` | Workspace isolation, non-privileged user |
| Credential exposure | Secrets in commits, env vars in logs | Credential hygiene, secrets scanning |
| Scope creep | Changes outside intended area | Dedicated workspace, anomaly detection |
| Data exfiltration | Posting secrets to external services | Scoped tokens, network controls |

### Threats NOT Addressed

This model does not protect against:

- Deliberate adversarial prompts from authorized users
- Compromise of the host machine itself
- Network-level attacks from external actors

These require controls outside the pack scope.

---

## The Containment Checklist

### Workspace Isolation

- [ ] Dedicated directory for swarm work (not home directory)
- [ ] Clone of repo, not the primary working copy
- [ ] No access to other projects or sensitive directories

### Credential Hygiene

- [ ] No secrets in environment variables (except scoped tokens)
- [ ] No credentials in git config (`~/.gitconfig`)
- [ ] No API keys in working tree
- [ ] SSH keys are agent-based or absent

### User Privileges

- [ ] Non-privileged user account
- [ ] No sudo access
- [ ] No ability to modify system files

### Network Controls

- [ ] (Ideal) Restricted egress to known endpoints
- [ ] (Minimum) Awareness that outbound access exists
- [ ] GitHub token scoped to minimum permissions

### Publish Boundary Gates

- [ ] Secrets scanning runs on staged content before commit
- [ ] Anomaly detection catches unexpected file changes
- [ ] Human approval required for merge/push to protected branches
- [ ] Branch protection rules enforced

---

## Violation Response

If any checklist item cannot be satisfied:

1. **Do not run with `bypassPermissions: true`**
2. Use standard permission prompts instead
3. Document the gap and remediation plan

Partial compliance is not sufficient. The checklist is a unit.

---

## The Safety Argument

With all checklist items satisfied:

| Layer | Protection |
|-------|------------|
| Workspace | Worst-case damage is contained to the clone |
| Credentials | No secrets available to leak |
| Privileges | Cannot escalate beyond the workspace |
| Network | Scoped tokens limit external actions |
| Publish gates | Secrets scanning prevents credential commits |
| Merge gates | Human approval required for protected branches |

This is **infrastructure safety**, not **prompt safety**.

---

## Platform-Specific Notes

### Windows/WSL2

- Use WSL2 for the sandbox environment
- Windows host files are isolated from WSL filesystem
- Git Bash can work but has different path semantics
- Prefer WSL2 for consistency with Linux-based tooling

### Linux/macOS

- Use a dedicated user account
- Consider container-based isolation for additional safety
- Ensure no credential helpers are configured
- Check `~/.netrc`, `~/.aws/credentials`, `~/.ssh/` for stray secrets

### CI/CD Environments

- Use ephemeral runners (no persistent state)
- Scope tokens to minimum permissions
- Never persist state between runs
- Prefer GitHub Actions with `GITHUB_TOKEN` (auto-scoped)

---

## Token Scoping Reference

Minimum permissions for GitHub operations:

| Operation | Required Scope |
|-----------|----------------|
| Clone/push to feature branch | `repo` or fine-grained: `contents: write` |
| Create PR | `pull_requests: write` |
| Read CI status | `checks: read` |
| Merge to protected branch | Requires human approval (cannot bypass) |

Fine-grained tokens are preferred over classic PATs.

---

## Checklist Validation

Before enabling `bypassPermissions`:

```bash
# 1. Verify workspace isolation
pwd                        # Should be dedicated directory
ls ..                      # Should not expose sensitive directories

# 2. Verify credential hygiene
env | grep -i secret       # Should be empty or expected
env | grep -i token        # Should only show scoped tokens
cat ~/.gitconfig           # No credentials
cat ~/.netrc 2>/dev/null   # Should not exist or be empty

# 3. Verify user privileges
whoami                     # Non-root user
sudo -n true 2>/dev/null && echo "FAIL: has sudo" || echo "OK: no sudo"

# 4. Verify token scope
gh auth status             # Check scopes
```

---

## See Also

- [Trust Model](trust-model.md) — Evidence hierarchy and verification boundaries
- [Contracts](contracts.md) — Control-plane blocks and publish gates
- [How to Customize Pack](../how-to/customize-pack.md) — Adapting to your environment
