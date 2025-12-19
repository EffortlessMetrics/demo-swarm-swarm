# Adapt to Non-GitHub Providers

> Use GitLab, Azure DevOps, Bitbucket, or other providers.

**Goal:** Fork the GitHub integration for your Git provider.

**Prereqs:** Understanding of the pack structure, access to your provider's CLI/API.

---

## Two sane options

### Option 1: Skip integration (recommended start)

Run flows locally, handle provider operations manually.

- No code changes required
- All local functionality works
- See [work-without-github.md](work-without-github.md)

### Option 2: Fork integration agents

Create equivalents of the `gh-*` agents for your provider.

This is more work but gives you automation.

---

## What needs forking

If you choose Option 2, these agents need provider-specific versions:

| Agent | Purpose | What changes |
|-------|---------|--------------|
| `gh-issue-manager` | Create/update issues | API calls, issue format |
| `gh-reporter` | Post comments | Comment API, formatting |
| `gh-researcher` | Read existing issues/PRs | Search/query API |

Additionally:
- `repo-operator` merge/release behaviors (Flow 5)
- `deploy-monitor` CI querying

---

## Contract checklist (must preserve)

When forking, keep these pack contracts intact:

### Two-gate semantics

```yaml
# Secrets gate (from secrets-sanitizer)
safe_to_publish: true | false

# Repo hygiene gate (from repo-operator)
proceed_to_github_ops: true | false
```

Your provider agents must respect both gates.

### Control-plane blocks

Keep the block schemas:

```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED_PUBLISH
safe_to_commit: true | false
safe_to_publish: true | false
...

## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_ANOMALY | FAILED | BLOCKED
proceed_to_github_ops: true | false
...
```

### Status/routing enums

Keep these closed:

```
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
```

### Reseal loop

```
(cleanup ↔ secrets-sanitizer) until modified_files: false
```

### Receipts-first reporting

Reporters read receipts; they don't recompute counts or upgrade statuses.

### Issue-first invariant

One tracking item per work item (issue equivalent in your provider).

---

## Example: GitLab

Create new agents:

```
.claude/agents/glab-issue-manager.md
.claude/agents/glab-reporter.md
.claude/agents/glab-researcher.md
```

Update flow commands to reference them:

```diff
- Call gh-issue-manager with task: "..."
+ Call glab-issue-manager with task: "..."
```

Update pack-check to recognize the new agents.

---

## Example: Azure DevOps

Use `az boards` CLI or REST API.

Key mappings:
- GitHub Issue → Azure Work Item
- GitHub Comment → Work Item Discussion
- GitHub PR → Azure PR

---

## What NOT to change

- Local artifact structure (`.runs/`)
- Receipt schemas
- Machine Summary format
- Microloop semantics
- Critics-never-fix pattern

These are pack-invariant, not provider-specific.

---

## Testing your fork

1. Run `bash .claude/scripts/pack-check.sh` — must pass
2. Run validation run (see [validation-run.md](../tutorials/validation-run.md))
3. Test both gates (safe_to_publish, proceed_to_github_ops)
4. Verify receipts are correct

---

## See also

- [work-without-github.md](work-without-github.md) — Running without provider integration
- [customize-pack.md](customize-pack.md) — General customization
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
