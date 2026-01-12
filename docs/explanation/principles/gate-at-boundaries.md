# Law 6: Gate at Boundaries

> Default-allow engineering inside the workspace. Gates engage at publish boundaries only.

## The Law

Agents work freely inside the development sandbox. No permission checks, no approval dialogs, no access control theater. Gates engage only when work crosses a publish boundary: commit, push, or GitHub post.

**Corollary:** Gates everywhere creates permission theater, not safety.

## Why This Matters

### Risk Lives at Boundaries, Not Inside

| Action | Risk | Why |
|--------|------|-----|
| Reading a file | None | Information stays in session |
| Writing code | None | Not committed yet |
| Running tests | None | Results are local |
| **Committing secrets** | **High** | Secrets enter git history forever |
| **Pushing to remote** | **High** | Changes become visible to the world |
| **Posting to GitHub** | **High** | Content is permanent, notifications sent |

The insight: Thinking and working have no side effects. Publishing does. Gate accordingly.

### Permission Theater Wastes Everyone's Time

```
Agent: "Can I read src/auth.rs?"  Human: "Yes"
Agent: "Can I run the tests?"     Human: "Yes"
Agent: "Can I edit the file?"     Human: "Yes"
```

This is exhausting and pointless. Every approval costs human attention for zero security benefit. The sandbox already prevents agents from affecting anything outside the session.

### The Economics

| Approach | Human Cost |
|----------|------------|
| Approval-per-action (50 ops x 30s + overhead + waiting) | ~65 minutes |
| Gate pattern (0 approvals + 3 gates + 1 PR review) | ~5 minutes |

The gate pattern is 13x more efficient while providing *better* security.

## The Three Boundaries

| Boundary | Gate | What It Checks | Why Here |
|----------|------|----------------|----------|
| **Commit** | secrets-sanitizer | API keys, tokens, private keys, passwords, credentials | Secrets in git history are permanent; rotation is expensive |
| **Push** | repo-operator | Expected paths only, no anomalies, no test deletions | Changes become visible to collaborators, CI, the world |
| **GitHub post** | Content restrictions | No secrets in PR/issue content, safe to publish | Posts trigger notifications, appear in search, cannot be deleted |

## Correct vs Incorrect Behavior

### Violation: Permission Checks Inside

```
"Before reading this file, let me check if I have permission..."
"I should ask before running this command..."
```

This is permission theater. The sandbox is the permission.

### Correct: Work Freely, Gate at Exit

```
Agent reads files, writes code, runs tests freely
Agent stages changes -> secrets-sanitizer scans -> commit
repo-operator checks anomalies -> push
```

The agent worked without friction. The gates caught problems at the boundary.

### Violation: Skipping Gates

```
Commit without secrets scan
Push without anomaly check
"We trust the agent"
```

Trust does not mean skip gates. Trust means work freely in the sandbox, then verify at boundaries.

### Correct: Consistent Gates

Every commit gets scanned. Every push gets checked. No exceptions. Gates that run inconsistently train people to route around them.

## How This Enables Velocity Without Sacrificing Safety

The key insight is that safety and velocity are not in tension when boundaries are properly defined.

**Safety comes from the gates**, not from restricting interior actions:
- Gates are deterministic (same input = same output)
- Gates are fast (seconds, not minutes)
- Gates provide actionable feedback when they fail
- Gates catch real problems at the moment they matter

**Velocity comes from freedom inside**, not from approvals:
- Try approaches without permission
- Run tests immediately
- Fix issues as discovered
- Iterate to working code in single sessions

A feature that takes 10 approval cycles over 2 hours can be built in 15 minutes with sandbox freedom plus boundary gates.

## What This Enables

| Without Law 6 | With Law 6 |
|---------------|------------|
| Agents ask permission for every action | Agents work freely |
| Flows stop constantly for approval | Flows complete smoothly |
| Security theater everywhere | Real security at real boundaries |
| Slow, frustrating, permission-obsessed | Fast, productive, appropriately controlled |

The freedom inside the sandbox is what makes the gates effective. When agents can work freely, they have no incentive to bypass the gates.

## The Trust Equation

```
Freedom Inside + Gates at Boundary = Safe Autonomy
```

- Remove freedom inside: velocity drops, agents become suggestion engines
- Remove gates at boundary: secrets leak, mistakes become permanent

Both are required. Neither alone is sufficient.

## See Also

- [publish-boundaries.md](../publish-boundaries.md) --- The three gates in detail
- [why-ops-first.md](../why-ops-first.md) --- The philosophy behind default-allow
- [gate-pattern.md](gate-pattern.md) --- Comprehensive gate mechanics
- [laws-of-the-swarm.md](../laws-of-the-swarm.md) --- All eleven laws
