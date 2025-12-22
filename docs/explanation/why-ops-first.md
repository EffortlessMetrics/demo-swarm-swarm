# Why Ops-First

> Engineering is default-allow. Publishing is gated.

---

## The Problem: Compliance Theater

Traditional approaches to AI safety in SDLC tooling often create what we call "Compliance Theater":

```
Agent: "I'd like to read this file."
System: "Prove you're allowed."
Agent: "I'd like to write this test."
System: "Prove you're allowed."
Agent: "I'd like to run this command."
System: "Prove you're allowed."
```

The result:
- **Token waste:** More tokens spent on justification than engineering
- **Friction cascade:** Every step requires permission negotiation
- **False security:** The gates don't actually prevent bad outcomes—they just slow down good ones
- **Context exhaustion:** By the time you're allowed to act, you've forgotten what you were doing

---

## The Insight: Where Does Risk Actually Live?

Risk doesn't live in *thinking* or *working*. Risk lives in *publishing*.

| Action | Risk Level | Why |
|--------|------------|-----|
| Reading a file | None | Information doesn't leave the session |
| Writing a test | None | It's not committed yet |
| Running tests locally | None | Results don't leave the session |
| Exploring code | None | No side effects |
| **Committing secrets** | **High** | Secrets enter git history |
| **Pushing to remote** | **High** | Changes become public |
| **Posting to GitHub** | **High** | Content becomes visible |

The insight: **Gate the boundary, not the interior.**

---

## The Solution: Two Planes

### Work Plane (Default-Allow)

Everything up to `git add` runs without friction:

- Read any file in the repo
- Write code, tests, documentation
- Run tests, linters, formatters
- Search, analyze, explore
- Security scans produce *findings* (advisory), not *blocks*

**Posture:** Trust the agent to do its job. It's working *for* you.

### Publish Plane (Gated)

Gates engage only when crossing the boundary:

| Boundary | Gate | What It Checks |
|----------|------|----------------|
| Commit | `secrets-sanitizer` | No secrets in staged changes |
| Push | `repo-operator` | No anomalies, no test deletions |
| GitHub post | Content mode | What can be quoted/linked |

**Posture:** Verify before sharing. Publishing has consequences.

---

## What This Enables

### 1. Early Feedback Loops

With default-allow work plane:

```
Build starts
  └─→ Write first tests
  └─→ Write first code
  └─→ Push (early!)
  └─→ Bots start reviewing
  └─→ Continue building while bots work
  └─→ Harvest feedback when ready
```

Without it:

```
Build starts
  └─→ Prove you can write tests
  └─→ Write tests
  └─→ Prove you can write code
  └─→ Write code
  └─→ Prove you can push
  └─→ Push (late)
  └─→ Wait for bots
  └─→ Discover issues (late)
```

### 2. Human Collaboration

Developers often jump in during a swarm run:
- Fix a typo in README
- Tweak a config value
- Add a comment

**Old behavior:** "Anomaly detected! Unknown file change! Block everything!"

**Ops-First behavior:** "Oh, the human fixed something. Stage it, note it, continue."

### 3. Honest Reporting

When security findings are advisory (Work Plane), agents can:
- Report what they find honestly
- Continue working on the actual task
- Fix issues as they go
- Only escalate truly blocking issues

When security findings are gates (everywhere), agents:
- Fear reporting findings (might block progress)
- Game the system to avoid triggers
- Waste tokens on justification

---

## The Key Trade-offs

### What We Accept

1. **Agents see everything:** An agent in the Work Plane can read any file. If you don't want it seen, don't put it in the repo.

2. **Local state can be messy:** The working tree might have uncommitted experiments. That's fine—they're local.

3. **Extras get committed:** If a human fixes something mid-run, it gets included. We record it, not reject it.

### What We Prevent

1. **Secret exposure:** Secrets never cross the Publish boundary without remediation.

2. **Reward hacking:** Test deletion is caught before commit, not after.

3. **Accidental publication:** Anomalies block push, not commit. You can always work locally.

---

## How It's Implemented

### In Flows

Each flow has a **Publish Surface** (what it's allowed to commit/push):

| Flow | Publish Surface |
|------|-----------------|
| Signal | `.runs/<run-id>/signal/` |
| Plan | `.runs/<run-id>/plan/` |
| Build | `.runs/<run-id>/build/` + project code/tests |
| Review | `.runs/<run-id>/review/` + project code/tests |
| Gate | `.runs/<run-id>/gate/` |
| Deploy | `.runs/<run-id>/deploy/` |
| Wisdom | `.runs/<run-id>/wisdom/` |

### In Agents

**repo-operator** implements the Publish boundary:
- Intent-based staging (flow tells it what to include)
- Extras detection (includes ad-hoc human fixes)
- Anti-reward-hacking guard (blocks test deletion)
- Anomaly classification (warns vs blocks)

**secrets-sanitizer** implements the secrets gate:
- Scans only the Publish Surface
- Fix-first (redact in place when possible)
- Block only when remediation requires human judgment

### In Gating

The two-gate rule for GitHub operations:
1. `safe_to_publish: true` (secrets gate)
2. `proceed_to_github_ops: true` (repo hygiene gate)

Both must pass for `gh` commands to execute.

---

## Philosophy Statement

> The pack is a **Code Conveyor Belt**, not a **Compliance Engine**.
>
> Our job is to ship trusted code efficiently.
> Gates exist to make publishing safe, not to make working hard.
> If an agent can't read a file, it can't do its job.
> If an agent pushes a secret, that's a gate failure.
>
> Default-allow the work. Gate the publish.

---

## See Also

- [architecture.md](architecture.md) — Overall pack design
- [why-two-gates.md](why-two-gates.md) — GitHub ops gating
- [ai-physics.md](ai-physics.md) — LLM-specific constraints
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
