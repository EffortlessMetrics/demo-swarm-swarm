# Upstream Export

> Move swarm work to your human repo.

**Goal:** Export completed work from the swarm repo to your main development repo.

**Prereqs:** Swarm repo with completed run(s), access to origin repo.

---

## Why this is separate

The pack is designed for swarm autonomy:

- Swarm repo runs flows end-to-end
- Flow 5 merges into swarm's `main`
- `.runs/` artifacts stay in swarm repo

Upstream export (moving code to human repo) is intentionally **not automated**. This keeps the human repo calm and gives you control over what lands.

---

## Export strategies

### Option 1: Cherry-pick commits

Best for: specific changes, clean history.

```bash
# In human repo
git remote add swarm /path/to/my-project-swarm
git fetch swarm

# Cherry-pick the build commit(s) you want
git cherry-pick <commit-sha>
```

Find the build commit SHA in `.runs/<run-id>/build/build_receipt.json` or Git history.

### Option 2: PR from swarm branch

Best for: review workflow, team visibility.

```bash
# In swarm repo, push the run branch to origin
git push origin run/<run-id>

# Open PR from run/<run-id> to origin/main
gh pr create --base main --head run/<run-id> --title "..."
```

### Option 3: Diff and apply

Best for: selective changes, manual control.

```bash
# Generate diff from swarm
cd my-project-swarm
git diff main~1..main -- src/ tests/ > changes.patch

# Apply in human repo
cd my-project
git apply changes.patch
```

---

## What to export vs. what to keep

| Content | Export? | Notes |
|---------|---------|-------|
| Code changes | Yes | The actual implementation |
| Test changes | Yes | Tests that validate the implementation |
| `.runs/` | No | Keep in swarm repo only |
| `run_meta.json` | No | Swarm-internal |
| Receipts | No | Audit trail stays in swarm |

---

## Preserving traceability

When exporting, include a reference to the swarm run:

```bash
git commit -m "feat: add OAuth2 login

Swarm run: feat-auth
Receipt: VERIFIED
Issue: #456"
```

This lets you trace upstream commits back to swarm artifacts if needed.

---

## Timing

Export when:

- Gate verdict is MERGE
- You've reviewed the build artifacts
- You're ready to integrate into human workflow

Don't export:

- Mid-flow (wait for Gate)
- If Gate verdict is BOUNCE (fix issues first)
- Automatically (human control is the point)

---

## See also

- [run-topology.md](run-topology.md) — Swarm repo setup
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
