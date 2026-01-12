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

## Human Edits Between Flows

The swarm is designed for human interleaving. You can make manual changes at any point; the swarm reconciles at the next flow boundary.

### The Reconciliation Model

Each flow starts by examining what actually exists on disk. Agents don't assume they're seeing a pristine state from the previous flow. They read artifacts, check git status, and work from reality.

**This means you can:**

- Edit code between Build and Review
- Fix typos in generated docs
- Adjust configuration values
- Add files the swarm didn't create
- Delete files you don't want
- Use other tools (your IDE, other AI assistants, scripts)

**The swarm will:**

- See your changes at the next flow boundary
- Incorporate them into its understanding
- Continue from the actual state, not an assumed state

### When to Make Manual Edits

| Timing | Impact |
|--------|--------|
| **Between flows** | Clean. Next flow sees your changes as the starting state. |
| **During a flow** | Depends. If the flow hasn't touched that file yet, fine. If it has, you may create conflicts. |
| **During agent execution** | Risky. Wait for the agent to finish. |

**Safest pattern:** Let the current flow complete, make your edits, then start the next flow.

### What Gets Reconciled vs. What Gets Lost

**Reconciled (preserved):**
- File content changes (code, docs, config)
- New files you added
- Deleted files (they stay deleted)
- Git state (commits, branch position)

**May cause confusion:**
- Editing artifact files (`.runs/`) that agents expect to control
- Changing file paths that receipts reference
- Modifying mid-flow while an agent is running

### Example: Human Fix Between Build and Review

```
Flow 3 (Build) completes
├── Code implemented
├── Tests written
└── build_receipt.json generated

Human notices a typo in error message
├── Edits src/auth/errors.py directly
└── Commits: "fix: typo in auth error message"

Flow 4 (Review) starts
├── Sees the extra commit
├── Includes it in the review scope
└── Continues normally
```

The swarm doesn't "notice" or "complain" about your edit. It just sees the current state and works from there.

### Interleaving Other Tools

You can use other tools between flows:

- Your IDE's refactoring tools
- Other AI assistants
- Linters and formatters
- Database migration tools
- Deployment scripts

**Treat the swarm as one tool among many.** It maintains state in `.runs/` and works via git. Other tools can coexist as long as they don't corrupt those interfaces.

### When Manual Edits Require Re-verification

If your manual changes affect:
- Test behavior - Re-run tests before Review
- API contracts - Verify alignment with `api_contracts.yaml`
- Security-sensitive code - Consider re-running relevant checks

The Gate (Flow 5) will catch many issues, but earlier verification saves time.

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

## Questions and Escalation

The swarm surfaces questions continuously but only requests human input at flow boundaries.

### How Questions Flow

Questions arise during agent work:
- Ambiguous requirements
- Design trade-offs
- Missing information
- Blockers requiring human judgment

**Questions are captured immediately** in `open_questions.md` as they arise. Agents don't stop to wait for answers; they document the question, make a reasonable assumption (if possible), and continue.

**Human input is requested at flow boundaries.** When a flow completes, open questions are surfaced in the receipt and handoff. This is when humans review and provide answers.

### The Question Lifecycle

```
During Flow
├── Agent encounters ambiguity
├── Records question in open_questions.md
├── Makes assumption (documented) OR flags as blocking
└── Continues work

At Flow Boundary
├── Receipt lists open questions
├── Human reviews questions
├── Human provides answers (or confirms assumptions)
└── Answers feed into next flow

Next Flow
├── Reads previous answers
├── Adjusts approach based on human input
└── May generate new questions
```

### Question Severity Levels

| Level | Description | Handling |
|-------|-------------|----------|
| **Informational** | Curious, not blocking | Log and continue |
| **Assumption-made** | Needed answer, chose default | Log assumption, continue, human can override |
| **Blocking** | Cannot proceed without answer | Complete partial work, surface at boundary |

**Most questions should be assumption-made.** The swarm is biased toward progress. Make a reasonable choice, document it clearly, and let humans correct if needed.

### Providing Answers at Flow Boundaries

When you review open questions at a flow boundary, update the artifact:

```markdown
## OQ-BUILD-003: Token refresh interval

**Status:** ANSWERED
**Answer:** Use 15 minutes, not 1 hour
**Answered by:** Alice
**Date:** 2024-01-15

**Original question:**
What should the token refresh interval be? Defaulted to 1 hour based on industry standard.

**Impact:** Update auth config before Gate
```

Or confirm the assumption:

```markdown
## OQ-BUILD-003: Token refresh interval

**Status:** CONFIRMED
**Confirmed by:** Alice
**Date:** 2024-01-15

**Original assumption:** 1 hour refresh interval
**Confirmation:** Assumption is correct, proceed with 1 hour
```

### When to Interrupt a Flow

Almost never. But these situations warrant interruption:

- **Security issue discovered** - Stop immediately
- **Working on wrong problem** - Stop before wasting more effort
- **Critical blocker** - External dependency unavailable
- **Human changed their mind** - Requirements shifted fundamentally

**To interrupt:** Simply stop the current session. Start fresh with updated context. The swarm will read the actual state and adjust.

### Escalation Paths

| Issue Type | Escalation Target | Method |
|------------|-------------------|--------|
| Technical question about code | Team tech lead | Update open_questions.md, tag in PR |
| Requirements clarification | Product owner | Out-of-band communication |
| Security concern | Security team | Stop flow, communicate immediately |
| Infrastructure blocker | DevOps | Out-of-band, document in run artifacts |

**The swarm doesn't have escalation automation.** Humans route to humans. The swarm captures the questions and context; you decide who answers them.

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

## Merge Discipline and Break Glass

The swarm enforces merge discipline through Gate (Flow 5). Sometimes you need to bypass it.

### Standard Merge Path

```
Flow 5 (Gate)
├── Verification checks pass
├── Policy checks pass
├── Secrets scan clean
├── merge_decision.md: MERGE
└── repo-operator executes merge
```

This is the expected path. Gate provides the audit trail and verification.

### When to Use Break Glass

"Break glass" means bypassing normal controls. Use it when:

| Scenario | Justification | Risk Level |
|----------|---------------|------------|
| **Production hotfix** | Outage requires immediate fix | High - verify manually |
| **Gate false positive** | Check failing incorrectly, verified manually | Medium - document bypass |
| **Blocked on infrastructure** | CI down, need to ship | Medium - run checks locally |
| **Security patch** | Vulnerability disclosure timeline | High - minimal change, verify manually |
| **Reverts** | Undoing a bad merge quickly | Low - revert is well-understood |

### Break Glass Procedure

**1. Announce your intent**

Tell your team you're bypassing Gate. This is not optional.

```
[Slack/Teams]: Breaking glass on feat-auth. Reason: CI infrastructure down,
manually verified tests pass. Merging directly.
```

**2. Document the bypass**

Create a record in the run artifacts:

```markdown
# Break Glass Record

**Run ID:** feat-auth
**Date:** 2024-01-15T14:30:00Z
**Operator:** alice@example.com

## Reason for Bypass
CI infrastructure (GitHub Actions) has been down for 2 hours.
Feature is blocking release. Tests verified locally.

## Verification Performed Manually
- [ ] Unit tests: `npm test` - all passing
- [ ] Integration tests: `npm run test:integration` - all passing
- [ ] Lint: `npm run lint` - clean
- [ ] Build: `npm run build` - success
- [ ] Security scan: `npm audit` - no high/critical

## Risks Accepted
- CI may catch issues we missed
- No automated policy checks ran
- Merge decision not formally documented

## Planned Follow-up
- Re-run Gate when CI recovers
- Verify no regressions in next deploy
```

Save as `.runs/<run-id>/gate/break_glass_record.md`

**3. Execute the merge**

```bash
# Merge directly (no swarm involvement)
git checkout main
git merge feat-auth --no-ff -m "feat: user authentication (break glass - CI down)"
git push origin main
```

**4. Follow up**

When conditions normalize:
- Run verification that was skipped
- Address any issues found
- Update the break glass record with outcomes

### Break Glass Anti-Patterns

| Don't | Why |
|-------|-----|
| Break glass because Gate is "too slow" | Speed is not an emergency |
| Skip documentation | You lose the audit trail |
| Break glass alone in secret | Team needs visibility |
| Use break glass for convenience | Erodes the discipline for everyone |
| Break glass on others' runs | Only the run owner should decide |

### Merge Discipline Beyond Gate

Even with Gate, teams should maintain discipline:

**Protected branches:** Configure GitHub/GitLab to require:
- PR before merge (no direct pushes)
- At least one approval
- CI checks passing
- Up-to-date with base branch

**Merge windows:** Consider limiting merges to:
- Business hours (so humans are available if issues arise)
- Not Friday afternoon (Monday debugging is painful)
- Not during incidents

**Merge order:** When multiple runs are ready:
- Smaller changes first (less conflict potential)
- Infrastructure before features
- Communicate the queue

### Post-Merge Verification

After any merge (Gate or break glass):

1. **Monitor:** Watch for errors in production/staging
2. **Verify:** Confirm the feature works as expected
3. **Communicate:** Update the team that the merge completed

The swarm's Flow 6 (Deploy) and Flow 7 (Wisdom) handle this for normal paths. Break glass bypasses them, so you're responsible for follow-through.

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
