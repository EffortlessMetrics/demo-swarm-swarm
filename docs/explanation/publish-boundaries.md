# Publish Boundaries

> The three gates between work and the world.

---

## What Are Publish Boundaries?

A publish boundary is where work becomes visible or permanent. Inside the boundary, agents work freely. At the boundary, gates verify before crossing.

**The three publish boundaries:**

| Boundary        | What Crosses                    | Risk If Unchecked                                       |
| --------------- | ------------------------------- | ------------------------------------------------------- |
| **Commit**      | Changes enter git history       | Secrets become permanent; rotation is expensive         |
| **Push**        | Changes leave local machine     | Visible to collaborators, CI, and potentially the world |
| **GitHub post** | Content reaches external system | Public visibility, notifications sent, no take-backs    |

Everything else is local iteration. Reading files, writing code, running tests, exploring options---none of these are publish boundaries. They have no side effects outside the session.

**The key insight:** Risk does not live in thinking or working. Risk lives in publishing. Gate accordingly.

---

## Law 6: Gate at Boundaries

This design implements Law 6 from the [Laws of the Swarm](laws-of-the-swarm.md):

> Default-allow engineering inside the workspace. Gates engage at publish boundaries only.

**Inside the workspace:**

- Read any file
- Write any code
- Run any test
- Iterate any number of times
- No permission checks

**At boundaries:**

- Require evidence
- Run deterministic checks
- Do not trust claims

This separation prevents "security theater" where agents spend more time proving they are allowed to act than actually acting.

---

## The Canonical Order: Stage, Sanitize, Persist

This order is physics, not preference. Violating it creates vulnerabilities.

```
Work freely in sandbox
        |
        v
   +--------+
   | STAGE  |  Define what would cross the boundary
   +--------+
        |
        v
  +----------+
  | SANITIZE |  Scan exactly what would be published
  +----------+
        |
        | safe_to_publish: true
        v
  +---------+
  | PERSIST |  Commit/push/post only if safe
  +---------+
        |
        v
Content crosses boundary
```

### 1. Stage --- Define the Intended Change Surface

Before you can verify what will be published, you must know what will be published.

**What happens:**

- Identify which files would be committed
- Capture the exact diff that would leave the sandbox
- Determine what content would be posted

**Why first:** You cannot scan what you have not staged. Scanning "the repo" instead of "the staged changes" creates false negatives. A secret added after the scan but before the commit will leak.

### 2. Sanitize --- Scan the Staged Surface

Scan exactly what would be published. Not the repo. Not the working tree. The staged surface.

**What happens:**

- Secrets detection on staged files
- Anomaly detection (unexpected changes)
- Surface verification (is this what we meant to publish?)

**Why second:** The scan must see exactly what the persist step will publish. Scanning something else creates a TOCTOU (time-of-check to time-of-use) gap. That gap is where secrets leak.

### 3. Persist --- Cross the Boundary (Gated)

Only proceed if sanitization passed. Otherwise, pause publishing (not work).

**What happens:**

- If `safe_to_publish: true` --- execute the publish operation
- If `safe_to_publish: false` --- block publishing, route to remediation

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

Scanning an unstaged or stale surface creates phantom confidence. You think you are clean, but you are not. The order is load-bearing.

---

## What Each Gate Checks

### Commit Boundary: secrets-sanitizer

The `secrets-sanitizer` agent guards the commit boundary.

**Scans for:**

- GitHub tokens: `gh[pousr]_[A-Za-z0-9_]{36,}`
- AWS access keys: `AKIA[0-9A-Z]{16}`
- Private keys: `-----BEGIN .*PRIVATE KEY-----`
- Stripe live keys: `sk_live_...`, `rk_live_...`
- Bearer tokens in authorization headers
- Database URLs with embedded passwords

**Returns:**

- `safe_to_publish: true` --- staged surface passed all checks
- `safe_to_publish: false` --- staged surface has issues

**Philosophy:** Fix aggressively, block only when necessary. Redact secrets in artifacts. Externalize credentials when the fix is obvious. Block only when remediation requires human judgment.

### Push Boundary: repo-operator

The `repo-operator` agent guards the push boundary.

**Checks for:**

- Only expected paths in commit (files match intent)
- No anomalous files outside allowlist
- Staging and commit operations succeeded
- No unexpected deletions (especially tests---anti-reward-hacking)

**Returns:**

- `proceed_to_github_ops: true` --- repo state is correct, safe to push
- `proceed_to_github_ops: false` --- anomalies detected, push blocked

**Anomaly classification:**

- Staged/unstaged anomalies (HIGH risk): Block push
- Untracked anomalies (LOW risk): Warning only

**Philosophy:** Be a senior dev running `git add`. Trust `.gitignore`. Trust developer ad-hoc fixes. Record what happened. Gate only real risks.

### GitHub Post Boundary: Content Mode

Before posting to GitHub (issues, comments, PR descriptions):

- Content restrictions apply
- No secrets in posted content
- Safe to quote/link

Both the secrets gate and repo hygiene gate must pass before any GitHub operation.

---

## The Two-Gate Rule

GitHub operations require BOTH gates to pass:

1. `secrets-sanitizer` returns `safe_to_publish: true`
2. `repo-operator` returns `proceed_to_github_ops: true`

This is defense in depth. Secrets and hygiene are independent concerns. Both must be clean before crossing the boundary.

| Scenario    | Secrets Gate | Hygiene Gate | Result                  |
| ----------- | ------------ | ------------ | ----------------------- |
| Both pass   | true         | true         | Proceed with GitHub ops |
| Secrets bad | false        | true         | Skip GitHub ops         |
| Hygiene bad | true         | false        | Skip GitHub ops         |
| Both bad    | false        | false        | Skip GitHub ops         |

See [why-two-gates.md](why-two-gates.md) for the full rationale.

---

## Why This Enables Default-Allow

The safety comes from architecture, not from permissions.

### Without Boundary Gates

```
Agent: "I'd like to read this file."
System: "Prove you're allowed."
Agent: "I'd like to write this test."
System: "Prove you're allowed."
Agent: "I'd like to run this command."
System: "Prove you're allowed."
```

Result: Token waste, friction cascade, false security, context exhaustion.

### With Boundary Gates

```
Agent: [reads files, writes code, runs tests freely]
System: [silence]
Agent: "Ready to commit."
System: "Let me scan the staged surface... clean. Proceed."
```

Result: Velocity inside, verification at edges.

### The Trust Equation

```
Freedom Inside + Gates at Boundary = Safe Autonomy
```

Remove freedom inside: velocity drops, agents become suggestion engines.

Remove gates at boundary: secrets leak, mistakes become permanent.

Both are required. Neither alone is sufficient.

---

## What Blocking Means (and Does Not Mean)

When `safe_to_publish: false`:

- **Publishing is blocked** --- the commit/push/post does not happen
- **Work continues locally** --- the agent is not stuck
- **Route to remediation** --- fix secrets, explain anomalies
- **Try again after remediation** --- the boundary is not permanently closed

The agent is not stuck. The sandbox is not locked. The boundary simply is not crossed yet.

This distinction matters: blocking work is expensive and creates pressure to bypass gates. Blocking publishing is cheap and creates pressure to fix the issue.

---

## The Metaphor

Consider a secure facility:

- Inside: employees move freely, work on anything, access any room
- At the exit: security checks bags, scans badges, verifies nothing leaves that shouldn't

The security comes from the exit check, not from following employees around. Surveillance inside is expensive and counterproductive. A good exit check is cheap and effective.

Same with agents:

- Inside the sandbox: read any file, write any code, run any command
- At the boundary: scan what would be published, block if unsafe

The security comes from the boundary check, not from approving each action.

---

## What This Prevents

| Risk                           | How It Is Caught                                     |
| ------------------------------ | ---------------------------------------------------- |
| Secret exposure                | secrets-sanitizer scans staged changes before commit |
| Reward hacking (test deletion) | repo-operator detects unexpected deletions           |
| Accidental publication         | Anomaly detection blocks push of unexpected files    |
| Credential leakage             | Secrets patterns caught before they enter history    |

What it does NOT prevent:

- Agents reading any file in the repo (that is allowed)
- Agents writing bad code locally (that is caught by tests and critics, not gates)
- Local experiments (they never cross the boundary)

---

## See Also

- [boundary-physics.md](boundary-physics.md) --- The mechanics in detail
- [why-ops-first.md](why-ops-first.md) --- The philosophy behind default-allow
- [why-two-gates.md](why-two-gates.md) --- Why secrets and hygiene are separate gates
- [laws-of-the-swarm.md](laws-of-the-swarm.md) --- Law 6 and the other immutable rules
- [architecture.md](architecture.md) --- Overall pack design
