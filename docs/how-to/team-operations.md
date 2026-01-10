# Team Operations

> How multiple humans coordinate when using the swarm on the same repo.

**Goal:** Operate cleanly when 2-5 people share a swarm-enabled repository.

**Prereqs:** Understanding of [run state](../reference/run-state.md) and [flow flexibility](../explanation/flow-flexibility.md).

---

## The Multi-Operator Reality

When multiple people use the swarm on the same repo:
- Runs can overlap
- Branches can conflict
- State can diverge

This guide teaches coordination patterns that keep everyone productive.

---

## Run Identity and Ownership

### Each Run Has an Owner

The person (or session) that starts a run owns it until completion.

**Naming conventions that signal ownership:**
- Person-based: `feat-auth-alice`, `fix-bug-bob`
- Issue-based: `issue-142`, `gh-142` (naturally unique)
- Ticket-based: `jira-PROJ-456`

Issue-based IDs are generally cleaner because they're already unique and don't require coordination.

### Don't Share Active Runs

If Alice is running Flow 3 on `feat-auth`, Bob should not also run Flow 3 on `feat-auth` from a different session. Wait for the run to complete or explicitly hand off.

**Why?** Active runs modify artifacts and branches. Concurrent modifications create conflicts that are painful to resolve.

### Completed Runs Are Shared Artifacts

Once a run completes (all flows finished, or explicitly handed off):
- Anyone can read the artifacts
- Anyone can continue from them (new run, same `canonical_key`)
- Receipts become shared team history

---

## Branch Discipline

### Swarm Branches Track Runs

Branch naming conventions:
- `swarm/<run-id>` - standard swarm branch
- `feat/<run-id>` - feature-style naming
- `run/<run-id>` - alternative pattern

**One active run per branch at a time.** If you need parallel work, use separate branches with separate run IDs.

### Main Branch Is Sacred

The swarm follows merge discipline:
- Never pushes directly to main
- All merges go through Gate (Flow 5)
- Merge decisions are documented in `merge_decision.md`

Humans can bypass this (see [flow flexibility](../explanation/flow-flexibility.md)), but the swarm itself maintains discipline.

### Handling Upstream Movement

Your branch diverges from main when others merge their work:

```
Your branch: feat-auth (3 commits ahead)
Main moved: 5 new commits since you branched
```

**Options:**

1. **Complete your run first, handle conflicts at Gate**
   - Recommended for most cases
   - Gate detects upstream movement and handles it
   - Don't interrupt mid-flow

2. **Rebase your branch onto main**
   - Re-run affected verification after rebase
   - Use when conflicts are trivial and you want a clean history

3. **Merge main into your branch**
   - Re-run affected verification after merge
   - Use when you need upstream changes to continue work

**Recommendation:** Complete the run, handle conflicts at Gate. Mid-flow interruptions create more problems than they solve.

---

## Collision Avoidance

### File-Level Collisions

Two runs touching the same files will likely conflict at merge time.

**Before starting:** Check for overlap:
```bash
# On your branch
git diff main --name-only > my-files.txt

# On their branch
git diff main --name-only > their-files.txt

# Check overlap
comm -12 <(sort my-files.txt) <(sort their-files.txt)
```

If overlap is significant, sequence the runs (one completes before the other starts).

### Artifact Collisions

Two runs with the same ID is a broken state.

**Prevention:**
- Always use unique run IDs
- Check `.runs/index.json` before creating new runs
- Use issue-based IDs when possible (they're unique by definition)

### Gate Collisions

Two runs trying to merge at the same time:
- First one wins
- Second one needs to rebase and re-verify

Gate handles this automatically. The second run's Gate will detect upstream movement and request re-verification.

---

## Handoffs Between Humans

When one person hands a run to another:

### 1. Complete the Current Flow

Don't hand off mid-flow. Flows are designed as atomic units. Complete the flow, let cleanup run, then hand off.

### 2. Write a Handoff Note

Add to the run's artifacts (e.g., `.runs/<run-id>/handoff.md`):

```markdown
## Handoff: Alice -> Bob

**Date:** 2024-01-15
**Run ID:** feat-auth-alice
**State:** Flow 3 complete, ready for Review

### Context
- Implemented OAuth2 login flow
- All tests passing (see build_receipt.json)
- Used Google as the first provider; other providers are stubbed

### Open Questions
- OQ-BUILD-003: Token refresh interval (defaulted to 1 hour)
- Should we support Remember Me? (not implemented)

### What Bob Needs to Know
- The auth module is at src/auth/oauth.py
- Integration tests require GOOGLE_CLIENT_ID env var (see .env.example)
- ADR chose stateless tokens over sessions (see plan/adr.md)
```

### 3. Communicate Out-of-Band

Slack, email, standup - whatever your team uses. The handoff note is the record; verbal communication ensures nothing falls through the cracks.

### 4. Bob Starts Fresh Session

Bob opens a new Claude Code session on the same run:
```bash
/flow-4-review  # Continue from where Alice left off
```

The swarm's resume-ready design means Bob doesn't need Alice's session state.

---

## Parallel Runs (Safe Patterns)

### Safe

| Pattern | Example |
|---------|---------|
| Different features on different branches | Alice: auth, Bob: payments |
| Same feature, sequential flows | Alice does Signal+Plan, Bob does Build+Gate |
| Independent bug fixes | Alice: `fix-123`, Bob: `fix-456` |
| Disjoint file sets | Alice: frontend, Bob: backend |

### Risky

| Pattern | Problem |
|---------|---------|
| Same files, different runs | Merge conflicts at Gate |
| Same run, different sessions simultaneously | Artifact corruption |
| Rebasing while another flow is active | Branch state divergence |
| Both editing shared infrastructure | Semantic conflicts even if files differ |

---

## Shared State Locations

| State | Location | Who Writes | Collision Risk |
|-------|----------|------------|----------------|
| Run artifacts | `.runs/<run-id>/` | Owner of run | Low (unique IDs) |
| Index | `.runs/index.json` | run-prep, cleanup agents | Medium (serialize updates) |
| Git branches | `refs/heads/*` | repo-operator | Medium (coordinate pushes) |
| GitHub issues/PRs | GitHub | gh-* agents | Low (API handles) |

**Serialize writes to index.json:** The cleanup agents write to index.json at flow boundaries. If two runs complete flows simultaneously, the writes should be serialized (git will reject the second push, requiring a pull and retry).

---

## Recovery Patterns

### Run Got Into a Bad State

| Option | When to Use | How |
|--------|-------------|-----|
| **A: Abandon and start fresh** | Artifacts are corrupted or confusing | New run with new ID |
| **B: Manually fix artifacts** | Know exactly what's wrong | Edit, commit, resume flow |
| **C: Revert to last known good** | Recent corruption | `git checkout <sha> -- .runs/<id>/` |

See [failure-recovery.md](failure-recovery.md) for detailed procedures.

### Branch Conflicts at Merge

1. Standard git resolution
2. Re-run verification (tests at minimum) after resolution
3. Document the resolution in merge commit or PR

### Competing Merges

Gate handles this: if main moved, rebase and re-verify.

**Never force-push over someone else's work.** If you must force-push, communicate first.

---

## Communication Discipline

### Before Starting a Run

- Check if anyone else is working on related code
- Claim the work (issue assignment, Slack, team standup)
- Choose a unique run ID

### During a Run

- Others can read artifacts (they're committed)
- Coordinate before touching shared files
- Update the issue if scope changes

### At Completion

- Announce when ready for review/merge
- Clean up stale branches after merge
- Update team on what shipped

---

## Anti-Patterns

### Don't

| Anti-Pattern | Why |
|--------------|-----|
| Start runs with duplicate IDs | Corrupts index and artifacts |
| Work on the same run from different sessions simultaneously | Race conditions on artifacts |
| Force-push shared branches | Loses others' work |
| Merge without re-verifying after upstream changes | Untested combinations |
| Leave abandoned runs cluttering the repo | Confuses team, wastes disk |

### Do

| Pattern | Why |
|---------|-----|
| Use unique, descriptive run IDs | Clear ownership, no collisions |
| Complete flows before handing off | Clean boundaries |
| Communicate about overlapping work | Prevents conflicts |
| Clean up after yourself | Keeps repo manageable |
| Check index.json before starting | Avoids duplicate IDs |

---

## Quick Reference

### Starting Work

```bash
# Check what runs exist
cat .runs/index.json | jq '.runs[].run_id'

# Check who's working on what (communication tool)
# <check Slack, issue board, etc.>

# Start with unique ID
/flow-1-signal "feat: add user auth"
# Run ID: feat-user-auth-alice
```

### Handing Off

```bash
# Complete current flow
# Write handoff note
# Notify teammate out-of-band
```

### Picking Up a Handoff

```bash
# Read handoff note
cat .runs/<run-id>/handoff.md

# Check current state
cat .runs/<run-id>/run_meta.json | jq '.flows_started'

# Continue from appropriate flow
/flow-4-review
```

### Resolving Collisions

```bash
# If duplicate run ID: use a different ID
# If branch conflict: rebase after other run merges
# If artifact corruption: see failure-recovery.md
```

---

## See Also

- [Run State](../reference/run-state.md) - Run identity and artifact schemas
- [Flow Flexibility](../explanation/flow-flexibility.md) - When and how to skip flows
- [Routing Table](../reference/routing-table.md) - How agents route between flows
- [Failure Recovery](failure-recovery.md) - Nuclear delete/restart procedures
- [CLAUDE.md](../../CLAUDE.md) - Full pack reference
