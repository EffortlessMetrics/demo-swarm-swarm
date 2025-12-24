# Upstream Export

> Move swarm work to your human repo.

**Goal:** Export completed work from the swarm repo to your main development repo.

**Prereqs:** Swarm repo with completed run(s), access to origin repo.

---

## See Also

**Related workflows:**
- [run-topology.md](run-topology.md) — Swarm repo setup and branch model
- [adopt-fork-workflow.md](adopt-fork-workflow.md) — Fork-based swarm workflow (coming soon)

**Core reference:**
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference, Flow 6 (Deploy) semantics

---

## Why this is separate

The pack is designed for swarm autonomy:

- Swarm repo runs flows end-to-end
- Flow 5 merges into swarm's `main`
- `.runs/` artifacts stay in swarm repo

Upstream export (moving code to human repo) is intentionally **not automated**. This keeps the human repo calm and gives you control over what lands.

---

## Decision tree: When to export

Use this tree to decide if and how to export:

```
Should I export this run?
│
├─ Is Gate verdict = MERGE?
│  ├─ No → Fix issues first, don't export yet
│  └─ Yes → Continue ↓
│
├─ Do I need human repo integration?
│  ├─ No → Keep in swarm only (experimentation, prototypes, learning runs)
│  └─ Yes → Continue ↓
│
├─ What's my team's workflow?
│  ├─ Solo/small team, direct commits → Cherry-pick (Option 1)
│  ├─ Team review process required → PR from swarm (Option 2)
│  └─ Selective/manual control needed → Diff and apply (Option 3)
```

**Keep in swarm only when:**
- Experimenting with ideas
- Building throwaway prototypes
- Running learning exercises
- Work is still in draft state

**Export to human repo when:**
- Production feature is complete and verified
- Team needs to review and integrate
- Work must be deployed from human repo
- Traceability to human commit history is required

---

## Common export scenarios

### Scenario A: Solo developer, clean feature
- **Context:** You've completed a feature, Gate says MERGE, you want it in human repo
- **Best approach:** Cherry-pick (Option 1)
- **Why:** Direct, clean history, minimal overhead

### Scenario B: Team review required
- **Context:** Team uses GitHub PR review workflow
- **Best approach:** PR from swarm branch (Option 2)
- **Why:** Fits existing team process, visibility, discussion thread

### Scenario C: Complex change, selective merge
- **Context:** Swarm made extra changes you don't want, or you need to adapt the diff
- **Best approach:** Diff and apply (Option 3)
- **Why:** Manual control, selective application, adaptation as needed

### Scenario D: Multiple small fixes
- **Context:** Several independent runs completed, want to batch export
- **Best approach:** Cherry-pick multiple commits (Option 1) or create a consolidation branch
- **Why:** Efficient batching, clean history

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

## Next steps

After exporting:

1. **Test in human repo context**: The swarm's tests passed, but verify in your repo's environment
2. **Update issue tracking**: Link the exported commit to your issue tracker
3. **Clean up swarm branch** (optional): Keep or delete `run/<run-id>` based on your retention policy
4. **Monitor production**: If deploying from human repo, watch for issues

---

## Troubleshooting

### Cherry-pick conflicts

If cherry-picking causes conflicts:

```bash
# Option A: Resolve manually
git cherry-pick <sha>
# Fix conflicts
git cherry-pick --continue

# Option B: Use diff strategy instead
# (See Option 3 above)
```

### PR from swarm branch rejected by CI

The swarm's tests passed, but human repo CI fails:

- **Cause:** Different CI config, dependencies, or environment
- **Fix:** Iterate in swarm (rerun Flow 3), then re-export

### Lost traceability

Can't remember which swarm run produced a commit:

- **Prevention:** Always include run ID in commit messages
- **Recovery:** Check `.runs/index.json` in swarm repo for timing correlation
