# Failure Recovery

> Recover from failed or conflicted runs using the nuclear delete/re-run strategy.

**Goal:** Get unstuck quickly when a run fails, conflicts, or produces broken state.

**Prereqs:** Understanding of git branches and the [run topology](run-topology.md).

---

## Core Principle: Delete and Restart, Don't Salvage

When a run fails or produces conflicts, the correct response is:

1. **Delete the broken run branch** (`git branch -D run/<run-id>`)
2. **Optionally delete `.runs/<run-id>/`** (if artifacts are corrupted)
3. **Start fresh** with `/flow-1-signal` or the appropriate flow

**Why?** Salvaging broken runs costs more human time than re-running. Each run costs $5-$10 in API calls. Spending 2 hours debugging a merge conflict is irrational when you can delete and restart in 5 minutes.

---

## When to Delete vs Repair

### Delete (Nuclear Option)

Use the nuclear option when:

- Run branch has irreconcilable merge conflicts
- `.runs/<run-id>/` artifacts are corrupted or inconsistent
- Flow crashed mid-execution with orphaned state
- Multiple flows attempted on stale base commit
- Upstream changed significantly during a multi-day run

### Repair (Rare Cases)

Only repair when:

- Simple fixup (typo in commit message)
- Single file conflict that's obvious to resolve
- Flow completed successfully but needs minor adjustment

**Default bias:** When in doubt, delete and restart.

---

## Recovery Runbook

### Scenario 1: Flow Crashed Mid-Execution

```bash
# Check current state
git status
git log --oneline -5

# If artifacts exist but are incomplete:
# Option A: Resume from checkpoint
/flow-3-build  # Flows are idempotent; will resume from ac_status.json

# Option B: Nuclear restart
git checkout main
git branch -D run/<run-id>
rm -rf .runs/<run-id>/
/flow-1-signal "original task"
```

### Scenario 2: Merge Conflict with Swarm Mainline

```bash
# Don't try to resolve complex conflicts
git checkout main
git branch -D run/<run-id>

# Re-run on fresh HEAD
/flow-1-signal "original task"
```

### Scenario 3: Upstream Changed During Run

```bash
# Check divergence
git fetch origin
git log --oneline origin/main..HEAD

# If significant divergence (>50 commits or colliding files):
git checkout main
git branch -D run/<run-id>

# Re-run on fresh HEAD
/flow-1-signal "original task"
```

### Scenario 4: Corrupted Artifacts

```bash
# If receipts are malformed or station outputs are inconsistent
git checkout main
git branch -D run/<run-id>
rm -rf .runs/<run-id>/

# Start fresh
/flow-1-signal "original task"
```

### Scenario 5: Multiple Failed Attempts

```bash
# After several failed reruns, clean slate is fastest
git checkout main
git branch -D run/<run-id>
git branch -D run/<run-id>-attempt2  # If exists
rm -rf .runs/<run-id>/

# Start fresh with clear mind
/flow-1-signal "original task"
```

---

## Preserving the Audit Trail

Even when deleting a run, the audit trail is preserved:

1. **Git history**: Prior commits remain in reflog for 30+ days
2. **Committed `.runs/`**: If the run was checkpointed, artifacts exist in git history
3. **GitHub issue**: If bound, the issue preserves flow summaries
4. **New run lineage**: Use `supersedes` field to link new run to old

To link a restart to the original run:

```bash
# When restarting, the new run can reference the old
# signal-run-prep will set supersedes: "<old-run-id>" if requested
/flow-1-signal "restart: original task"
```

---

## The "Disposable Asset" Mindset

Runs are **disposable assets**:

- **Cheap to create**: API costs are low ($5-$10 per run)
- **Expensive to salvage**: Human debugging time is precious
- **Easy to reproduce**: Flows are deterministic given same inputs
- **Audit trail survives**: Git history preserves everything

When in doubt: **delete and restart**.

---

## Cost-Benefit Analysis

| Scenario | Salvage Cost | Restart Cost | Recommended |
|----------|--------------|--------------|-------------|
| Simple typo | 2 min (edit + amend) | 5 min (rerun signal) | Repair |
| Single file conflict | 10-30 min (resolve + test) | 5-15 min (restart) | Delete |
| Multiple conflicts | 1-2 hours (resolve + verify) | 15-30 min (restart) | Delete |
| Corrupted artifacts | Unknown (investigation) | 15-30 min (restart) | Delete |
| Stale base (>50 commits) | 2+ hours (rebase + test) | 30-60 min (restart) | Delete |

**Rule of thumb:** If salvage takes more than 3x restart time, delete.

---

## Advanced: Salvage When Required

In rare cases, salvage is necessary (e.g., unreproducible manual input):

### Resolving Merge Conflicts

```bash
# On run branch with conflicts
git status

# Resolve conflicts in editor
# Then:
git add .
git commit -m "Resolve merge conflicts"

# Resume flow
/flow-3-build  # Continue where left off
```

### Fixing Corrupted Receipt

```bash
# If only the receipt is broken but work is good
cd .runs/<run-id>/<flow>/

# Manually fix the JSON (use jq to validate)
vim <flow>_receipt.json

# Validate
jq . <flow>_receipt.json

# Commit fix
git add <flow>_receipt.json
git commit -m "Fix corrupted receipt JSON"

# Continue
/flow-4-gate  # Next flow
```

**Warning:** Manual edits bypass verification. Only use when restart is truly infeasible.

---

## Related

- [Run Topology](run-topology.md) — Branch and commit model
- [Troubleshoot](troubleshoot.md) — Common issues
- [Working with Receipts](working-with-receipts.md) — Receipt structure and validation
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference

---

## Quick Reference

| Problem | Command | Time Cost |
|---------|---------|-----------|
| Crashed flow | `/flow-<N>-<name>` (resume) | 5-15 min |
| Merge conflict | `git checkout main && git branch -D run/<id>` + restart | 15-30 min |
| Corrupted artifacts | `rm -rf .runs/<id>` + restart | 15-30 min |
| Stale base | Delete branch + restart | 30-60 min |
| Multiple failures | Clean slate + restart | 30-60 min |

**Default action:** Delete and restart.
