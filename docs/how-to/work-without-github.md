# Work Without GitHub

> Run flows locally when `gh` isn't available.

**Goal:** Use the pack without GitHub integration.

**Prereqs:** Pack installed, shell with `bash`, `git`, `jq`.

---

## What still works

Everything local:

- All 7 flows execute normally
- Artifacts written to `.runs/<run-id>/`
- Receipts computed and written
- Local Git commits (checkpoint + build)
- Reseal pattern (secrets sanitization)

---

## What gets skipped

GitHub operations are gated by `gh` availability:

| Operation            | Behavior without `gh` |
| -------------------- | --------------------- |
| Issue creation       | SKIPPED               |
| Status board updates | SKIPPED               |
| Flow comments        | SKIPPED               |
| PR creation          | SKIPPED               |
| Merge (Flow 5)       | SKIPPED               |

Evidence of skipped operations is recorded in:

- `gh_issue_status.md`
- `gh_report_status.md`

---

## How the gates work

GitHub operations require two gates:

1. **Secrets gate:** `safe_to_publish: true` from secrets-sanitizer
2. **Repo hygiene gate:** `proceed_to_github_ops: true` from repo-operator

When `gh` is unavailable, `gh-issue-manager` and `gh-reporter` detect this and skip gracefully. The gates still run; the operations don't.

---

## Running flows

Same commands, same behavior:

```text
/flow-1-signal "Add health endpoint"
/flow-2-plan
/flow-3-build
/flow-4-review
/flow-5-gate
```

You'll see messages like:

- "GitHub CLI not available, skipping issue operations"
- "GitHub operations skipped (auth not configured)"

---

## Flow 5 without GitHub

Flow 5 has two operation types:

| Type                     | Without GitHub            |
| ------------------------ | ------------------------- |
| Release ops (merge, tag) | SKIPPED — handle manually |
| Reporting ops            | SKIPPED — handle manually |

The flow still:

- Writes deployment artifacts
- Computes deploy_receipt.json
- Records what would have happened

Manual steps for production:

1. Review Gate verdict (must be MERGE)
2. Manually merge the run branch
3. Manually create tags/releases if needed

---

## Manual GitHub equivalents

If you want observability without automation:

### Create issue manually

```bash
# After Flow 1
cat .runs/<run-id>/signal/signal_receipt.json | jq '.status'
# Copy summary to a GitHub issue manually
```

### Post status manually

```bash
# After any flow
cat .runs/<run-id>/<flow>/<flow>_receipt.json
# Paste key fields into issue comment
```

---

## Enabling GitHub later

When you're ready:

```bash
# Install and authenticate
brew install gh  # or equivalent
gh auth login

# Verify
gh auth status
```

Future flow runs will use GitHub operations automatically.

---

## Troubleshooting

### "GitHub operations skipped"

Expected behavior. Check `gh auth status`.

### "Can I backfill GitHub posts?"

Not automatically. The pack doesn't have a "post-hoc reporting" mode. You can manually create issues and copy receipt summaries.

### "Will reruns post to GitHub?"

Yes, once `gh` is authenticated. The idempotent reporting system will create/update issues and comments.

---

## See also

- [adapt-to-non-github.md](adapt-to-non-github.md) — Use GitLab/Azure/Bitbucket
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
