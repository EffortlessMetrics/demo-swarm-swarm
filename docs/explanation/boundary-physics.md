# Boundary Physics

> The mechanics of publish boundaries and why order matters.

This document explains the safety architecture that makes `bypassPermissions` viable. It's not about prompts or permissions—it's about physics.

---

## The Core Principle

> **Engineering is default-allow. Publishing is gated.**

Inside the sandbox: autonomy, velocity, iteration.

At boundaries: boring, deterministic checks.

This isn't a philosophy. It's an engineering trade-off. We accept that agents can do anything inside the sandbox because we verify everything that crosses the boundary. The sandbox is where work happens. The boundary is where work becomes visible.

---

## What Counts as a Boundary

Publish boundaries are where work becomes visible or permanent:

| Boundary             | Risk                            | Why It Matters                            |
| -------------------- | ------------------------------- | ----------------------------------------- |
| Git commit           | Secrets enter history           | History is forever; rotation is expensive |
| Git push             | Changes leave local machine     | Visible to collaborators and CI           |
| PR creation/update   | Content becomes public          | Appears in GitHub, notifications sent     |
| GitHub issue/comment | Content reaches external system | Visible to the world                      |
| Release/deploy       | Code reaches users              | Production impact                         |

These are the **only** places where "not safe yet" is a valid outcome. Everything else is local iteration.

**The key insight:** Reading a file isn't a risk. Writing code locally isn't a risk. Running tests isn't a risk. Only _publishing_ creates risk. Gate accordingly.

---

## The Canonical Order: Stage, Sanitize, Persist

This order is physics, not preference. Violating it creates real vulnerabilities.

### 1. Stage — Define the Intended Change Surface

Before you can verify what will be published, you must know what will be published.

**What happens:**

- Identify which files would be committed
- Determine what content would be posted
- Capture the exact diff that would leave the sandbox

**Why first:** You cannot scan what you haven't staged. Scanning "the repo" instead of "the staged changes" creates false negatives. Secrets added after the scan but before the commit will leak.

### 2. Sanitize — Scan the Staged Surface

Scan exactly what would be published. Not the repo. Not the working tree. The staged surface.

**What happens:**

- Secrets detection on staged files
- Anomaly detection (unexpected changes)
- Surface verification (is this what we meant to publish?)

**Why second:** The scan must see exactly what the persist step will publish. Scanning something else creates a TOCTOU (time-of-check to time-of-use) gap. That gap is where secrets leak.

### 3. Persist — Commit/Push/Post, Gated on Scan Results

Only proceed if sanitization passed. Otherwise, pause publishing (not work).

**What happens:**

- If `safe_to_publish: true` — execute the publish operation
- If `safe_to_publish: false` — block publishing, route to remediation

**Why last:** The gate must be the final step before the boundary crossing. Anything between the gate and the crossing is a vulnerability.

### Why Order Matters

```
WRONG:                          RIGHT:
1. Scan repo                    1. Stage changes
2. Stage changes                2. Scan staged changes
3. Commit                       3. Commit (if scan passed)
   ^-- Secret added here           ^-- Secret caught here
       is not scanned                   before crossing
```

Scanning an unstaged or stale surface creates phantom confidence. You think you're clean, but you're not. The order is load-bearing.

---

## The Boundary Vocabulary

### Safe to Publish

The gate returns one of two values:

| Value                    | Meaning                          | Response                               |
| ------------------------ | -------------------------------- | -------------------------------------- |
| `safe_to_publish: true`  | Staged surface passed all checks | Proceed with publish operation         |
| `safe_to_publish: false` | Staged surface has issues        | Block publishing, route to remediation |

### This Is Not "Blocked"

When `safe_to_publish: false`:

- Publishing is blocked
- **Work continues locally**
- Route to remediation (fix secrets, explain anomalies)
- Try again after remediation

The agent is not stuck. The sandbox is not locked. The boundary simply isn't crossed yet. This distinction matters: blocking work is expensive and creates pressure to bypass gates. Blocking publishing is cheap and creates pressure to fix the issue.

### The Flow

```
[Work freely in sandbox]
         |
         v
    +--------+
    | Stage  |
    +--------+
         |
         v
    +-----------+
    | Sanitize  |---> safe_to_publish: false ---> [Fix issue]
    +-----------+                                     |
         |                                            |
         | safe_to_publish: true                      |
         v                                            |
    +---------+                                       |
    | Persist | <-------------------------------------+
    +---------+       (retry after remediation)
         |
         v
    [Content crosses boundary]
```

---

## Anomaly Detection

Beyond secrets, boundaries should catch unexpected changes.

### What Anomalies Look Like

- Files outside intended scope being staged
- Unexpected deletions (especially tests)
- Changes that don't match the stated intent
- Files that shouldn't exist in the repo

### Response Protocol

1. **Explain or remediate** — Anomalies aren't automatically forbidden. Sometimes a human fixed a typo while the swarm ran. That's fine.
2. **If explainable** — Document in `extra_changes.md`, proceed
3. **If suspicious** — Block and require human review
4. **Then retry** — Gate checks again with new context

The goal isn't to prevent all surprises. It's to ensure surprises are visible and intentional.

---

## Why This Makes bypassPermissions Safe

The safety isn't in prompts or permissions. It's in architecture.

### 1. Containment (Sandbox Workspace)

The sandbox is a real boundary:

- No production credentials
- No production network access
- No ability to affect users
- Git is the only path out

Claude can do anything inside the sandbox because the sandbox itself is the containment. A mistake in the sandbox is cheap—revert the file, try again.

### 2. Boundary Gates (Deterministic Checks at Exits)

Gates are boring and mechanical:

- Regex-based secrets detection
- Diff-based anomaly detection
- Boolean pass/fail outcomes

No judgment, no prompting, no "trust me." The gate passes or it doesn't. This is what makes it trustworthy.

### 3. Observable Evidence (Audit Trail)

Everything is logged:

- What was staged
- What was scanned
- What the gate decided
- What crossed the boundary

If something goes wrong, you can reconstruct exactly what happened. The audit trail is the safety net that makes autonomy acceptable.

---

## The Metaphor

> **We don't restrict what agents can think. We gate what crosses the boundary.**

Consider a secure facility:

- Inside: employees move freely, work on anything, access any room
- At the exit: security checks bags, scans badges, verifies nothing leaves that shouldn't

The security comes from the exit check, not from following employees around. Surveillance inside is expensive and counterproductive. A good exit check is cheap and effective.

Same with agents:

- Inside the sandbox: read any file, write any code, run any command
- At the boundary: scan what would be published, block if unsafe

The security comes from the boundary check, not from approving each action. Permission dialogs are expensive and counterproductive. A good boundary gate is cheap and effective.

---

## Implementation Notes

### Secrets Scanning

The `secrets-sanitizer` implements the secrets gate:

- Scans only the staged surface (not the whole repo)
- Runs regex patterns for known secret formats
- Returns `safe_to_publish: true | false`
- Provides specific findings when blocked

### Repo Hygiene

The `repo-operator` implements the hygiene gate:

- Detects anomalies (files outside expected scope)
- Guards against test deletion (anti-reward-hacking)
- Returns `proceed_to_github_ops: true | false`

### Two-Gate Rule

GitHub operations require BOTH gates to pass:

1. `secrets-sanitizer` returns `safe_to_publish: true`
2. `repo-operator` returns `proceed_to_github_ops: true`

This is defense in depth. Secrets and hygiene are independent concerns, and both must be clean before crossing the boundary.

---

## The Trust Equation

```
Freedom Inside + Gates at Boundary = Safe Autonomy
```

Remove freedom inside: velocity drops, agents become suggestion engines

Remove gates at boundary: secrets leak, mistakes become permanent

Both are required. Neither alone is sufficient.

---

## See Also

- [why-ops-first.md](why-ops-first.md) — The philosophy behind default-allow engineering
- [why-two-gates.md](why-two-gates.md) — Why secrets and hygiene are separate gates
- [operational-philosophy.md](operational-philosophy.md) — bypassPermissions and why it works
- [gate-pattern.md](principles/gate-pattern.md) — The gate pattern in detail
- [architecture.md](architecture.md) — Overall pack design
