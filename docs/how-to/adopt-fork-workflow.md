# Adopt Fork Workflow

> Use a fork-based model to isolate swarm activity from human development.

**Goal:** Set up a dedicated swarm fork that runs flows autonomously while your human repo stays calm.

**Prereqs:** GitHub account with fork permissions, Git basics, DemoSwarm pack available.

---

## Why fork instead of clone?

The fork workflow gives you:

- **Clean separation:** Human development in origin, swarm activity in fork
- **GitHub PR integration:** Open PRs from fork to origin when ready
- **Inspectability:** `.runs/` artifacts visible in fork, not cluttering origin
- **Team isolation:** Swarm commits don't trigger origin CI unnecessarily
- **Easier export:** Fork PRs are native GitHub, no remote juggling

Clone workflow (covered in [run-topology.md](run-topology.md)) is simpler but loses GitHub PR integration. Fork is the recommended default when you need GitHub visibility.

---

## Fork topology

```
your-org/my-project            # Origin (human workspace)
    ↓ fork
your-org/my-project-swarm      # Fork (swarm workspace)
```

Or if you prefer user-scoped forks:

```
your-org/my-project            # Origin (team repo)
    ↓ fork
your-username/my-project-swarm # Fork (your swarm workspace)
```

Both work. Choose based on who owns the swarm runs (team vs individual).

---

## Setup guide

### Step 1: Fork the repo

Via GitHub UI:

1. Navigate to `https://github.com/your-org/my-project`
2. Click "Fork"
3. Name it `my-project-swarm`
4. Uncheck "Copy the `main` branch only" (keep all branches if needed)
5. Click "Create fork"

Or via `gh` CLI:

```bash
gh repo fork your-org/my-project --fork-name my-project-swarm --clone=false
```

### Step 2: Clone the fork locally

```bash
git clone https://github.com/your-org/my-project-swarm.git
cd my-project-swarm
```

Verify remotes:

```bash
git remote -v
# origin    https://github.com/your-org/my-project-swarm.git (fetch)
# origin    https://github.com/your-org/my-project-swarm.git (push)
# upstream  https://github.com/your-org/my-project.git (fetch)
# upstream  https://github.com/your-org/my-project.git (push)
```

GitHub automatically creates the `upstream` remote pointing to the original repo.

### Step 3: Install the pack

Copy the DemoSwarm pack into your fork:

```bash
# If you have demo-swarm cloned locally
cp -r /path/to/demo-swarm/.claude .

# Or clone demo-swarm and copy
git clone https://github.com/your-org/demo-swarm.git /tmp/demo-swarm
cp -r /tmp/demo-swarm/.claude .
rm -rf /tmp/demo-swarm
```

Commit the pack:

```bash
git add .claude/
git commit -m "feat: Add DemoSwarm pack"
git push origin main
```

### Step 4: Customize (optional)

Adapt test/lint/policy commands to your stack:

```text
/customize-pack
```

Commit the customizations:

```bash
git add .claude/
git commit -m "chore: Customize pack for <project> stack"
git push origin main
```

### Step 5: Run your first flow

```text
/flow-1-signal "Add health check endpoint"
```

The flow will:
- Create `.runs/<run-id>/signal/` artifacts
- Commit to `run/<run-id>` branch
- Push to fork (if gates pass)

---

## Managing two repos in parallel

### Daily development (origin)

Continue normal work in your cloned origin repo:

```bash
cd /path/to/my-project  # origin clone

# Regular development workflow
git checkout -b feat/new-thing
# ... make changes ...
git commit -m "Add new thing"
git push origin feat/new-thing
```

Swarm activity doesn't touch this repo.

### Swarm runs (fork)

Run flows in the fork:

```bash
cd /path/to/my-project-swarm  # fork clone

# Run flows
/flow-1-signal "Task description"
/flow-2-plan
/flow-3-build
# etc.
```

Artifacts live in `.runs/<run-id>/` and are committed to the fork.

### Keeping fork in sync (optional)

If origin changes significantly and you want the fork to track it:

```bash
cd /path/to/my-project-swarm

# Fetch origin changes
git fetch upstream

# Merge into fork's main
git checkout main
git merge upstream/main
git push origin main
```

**Note:** This is optional. The fork can diverge from origin — you're only exporting specific completed runs, not maintaining perfect sync.

---

## Exporting completed work to origin

### When to export

Export when:

- Gate verdict (Flow 5) is `MERGE`
- You've reviewed build artifacts and receipts
- You're ready to integrate into human workflow

Don't export:

- Mid-flow (wait for Gate)
- If Gate verdict is `BOUNCE` (fix issues first)
- Automatically (human judgment is the point)

### Export via GitHub PR (recommended)

Flow 3 creates a Draft PR from `run/<run-id>` to fork's `main`. After Gate approves:

1. Change PR base from fork to origin:

```bash
cd /path/to/my-project-swarm

# Update PR to target origin instead of fork
gh pr edit <pr-number> --base main --repo your-org/my-project
```

Or create a fresh PR targeting origin:

```bash
# Push run branch to origin
git push upstream run/<run-id>

# Open PR from fork branch to origin main
gh pr create \
  --repo your-org/my-project \
  --head your-org/my-project-swarm:run/<run-id> \
  --base main \
  --title "feat: Add health check endpoint" \
  --body "Swarm run: <run-id>
Receipt: VERIFIED
Issue: #456"
```

2. Request review, merge when approved

**Benefits:**
- Native GitHub PR review workflow
- CI runs against origin
- Code appears in origin with clean attribution

### Export via cherry-pick

Best for selective changes without PR overhead:

```bash
cd /path/to/my-project  # origin clone

# Add fork as remote
git remote add swarm https://github.com/your-org/my-project-swarm.git
git fetch swarm

# Cherry-pick the build commit(s)
git cherry-pick <commit-sha>

# Push to origin
git push origin <branch-name>
```

Find commit SHAs in `.runs/<run-id>/build/build_receipt.json` or Git log.

### What to export vs. what to keep

| Content | Export? | Notes |
|---------|---------|-------|
| Code changes | Yes | The implementation |
| Test changes | Yes | Tests validating the implementation |
| `.runs/` artifacts | No | Keep in fork only |
| `run_meta.json` | No | Swarm-internal metadata |
| Receipts | No | Audit trail stays in fork |
| `.claude/` pack | Maybe | If origin wants swarm capability too |

**Traceability:** Include swarm run reference in commit messages:

```
feat: add health check endpoint

Swarm run: feat-health-check
Receipt: VERIFIED
Issue: #456
```

This lets you trace origin commits back to fork artifacts if needed.

---

## Decision tree: when to export

```
Gate verdict = MERGE?
  ├─ No → Fix issues, rerun flow, don't export yet
  └─ Yes → Review artifacts, decide export strategy
      ├─ Need team review? → GitHub PR (fork to origin)
      ├─ Trusted + small change? → Cherry-pick
      └─ Experimental / uncertain? → Keep in fork, don't export
```

**Key principle:** Export is a human decision gate, not automatic. The fork preserves the full audit trail; origin only gets vetted, approved changes.

---

## Fork vs. clone comparison

| Aspect | Fork Workflow | Clone Workflow |
|--------|---------------|----------------|
| Setup | Fork on GitHub, clone locally | Clone origin, no fork |
| GitHub integration | Native fork PRs to origin | Manual remote setup for PRs |
| Visibility | Fork is a GitHub repo (team visible) | Clone is local-only unless pushed |
| Export | GitHub PR (easy) | Cherry-pick or manual PR (harder) |
| Isolation | Perfect (separate GitHub repo) | Good (separate clone) |
| Recommended for | Team workflows, GitHub-heavy projects | Solo workflows, local-first development |

Choose fork when:
- You want GitHub PR integration
- Team needs visibility into swarm runs
- You're comfortable with GitHub fork mechanics

Choose clone when:
- You prefer local-only simplicity
- No need for GitHub PR workflow
- You're already managing multiple remotes

---

## Troubleshooting

### "Fork is out of sync with origin"

This is fine. Forks don't need perfect sync. Only export specific run branches when ready.

If you want to sync:

```bash
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

### "Can't push to fork"

Check fork permissions:

```bash
gh auth status

# Ensure you can push to the fork
git push origin main
```

If using org-scoped fork, ensure you have write access to `your-org/my-project-swarm`.

### "PR from fork is failing CI"

CI failures in fork-to-origin PRs are expected on first attempt. The swarm runs its tests in the fork environment. Origin CI may have different requirements.

**Fix:**
1. Review CI failures in origin PR
2. Make fixes in fork's `run/<run-id>` branch
3. Push updates to fork
4. PR updates automatically

This is normal iteration between fork and origin constraints.

### "Want to delete the fork and start over"

Safe to do. Forks are disposable:

```bash
# Delete fork on GitHub
gh repo delete your-org/my-project-swarm --yes

# Delete local clone
rm -rf /path/to/my-project-swarm

# Start over with Step 1
```

Origin is unaffected (assuming you haven't merged any PRs yet).

---

## Advanced: Multiple swarm forks

Some teams run multiple swarm forks:

```
my-project                    # Origin
  ├─ my-project-swarm-alice   # Alice's swarm fork
  ├─ my-project-swarm-bob     # Bob's swarm fork
  └─ my-project-swarm-team    # Shared team swarm fork
```

Benefits:
- Personal experimentation doesn't affect team
- Each developer controls their swarm runs
- Can still PR from personal fork to origin

Tradeoffs:
- More repos to manage
- Less visibility into each other's work

Use this when:
- Team is large (>3 people using swarm)
- Developers work on separate features
- You want personal swarm sandboxes

---

## See also

- [run-topology.md](run-topology.md) — Swarm repo setup (clone-based)
- [upstream-export.md](upstream-export.md) — Export strategies in detail
- [work-without-github.md](work-without-github.md) — Local-only workflow
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
