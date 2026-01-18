# Principle: The Gate Pattern

> Engineering is default-allow. Publishing is gated.

## The Principle

Inside the development sandbox, Claude has freedom to explore, implement, and iterate without approval. Gates engage only at publish boundaries — when work is about to leave the sandbox and affect the outside world.

## Why This Matters

### Machine Time vs Human Attention

Machine time is cheap. Human attention is expensive.

Let Claude iterate 10 times to find what works. Only engage human judgment when it matters:

- "Should we ship this?" (merge gate)
- "Is this safe to publish?" (secrets gate)
- "Does this solve the problem?" (review)

Don't spend human attention on:

- "Can Claude read this file?" (yes)
- "Can Claude run this test?" (yes)
- "Can Claude try this approach?" (yes)

The economics are asymmetric. A human reviewing "permission to read auth.rs" costs 30 seconds of context switch plus cognitive load. Claude reading the file costs fractions of a cent and milliseconds. When you multiply this across hundreds of operations per feature, approval-per-action becomes absurdly expensive.

### The Sandbox Is the Boundary

We don't constrain each action. We constrain the blast radius:

- The repo is the sandbox
- Changes stay local until pushed
- Secrets can't leak until publish
- Production isn't affected until deploy

Claude can do anything inside the sandbox. The gates catch problems at the boundary.

This is exactly how you'd onboard a trusted new team member: "Here's your dev environment. Experiment freely. When you're ready to ship, we'll review." You don't supervise every keystroke — you verify the deliverable.

### Velocity Through Freedom

Approval-per-action kills velocity:

- Constant interruption breaks flow
- Context loss between approvals
- Stop-and-wait cycles waste time
- Claude becomes a suggestion engine instead of a builder

Freedom enables velocity:

- Try approaches rapidly without permission
- Run tests immediately to get feedback
- Fix issues in-flight as they're discovered
- Iterate to working code in a single session

The difference is an order of magnitude. A feature that takes 10 approval cycles over 2 hours can be built in 15 minutes with sandbox freedom.

## How It Works

### Inside the Sandbox (Default-Allow)

Claude can freely:

- Read any file in the repo
- Write and edit code
- Run commands and scripts
- Execute tests
- Create branches
- Iterate on implementations
- Try multiple approaches
- Refactor as needed

No approval needed. No permission dialogs. Full capability.

This isn't reckless — the sandbox itself is the security boundary. Claude operates on a clone of the repo with no production access, no credentials, and no ability to affect the outside world until explicitly publishing.

### At the Boundaries (Gated)

Gates engage at publish points:

| Boundary | Gate              | What It Checks                    |
| -------- | ----------------- | --------------------------------- |
| Commit   | secrets-sanitizer | No secrets in staged changes      |
| Push     | repo-operator     | Repo hygiene, anomaly detection   |
| PR       | merge-decider     | Evidence synthesis, quality gates |
| Merge    | GitHub/CI         | Tests pass, reviews complete      |
| Deploy   | deploy-decider    | Governance, verification          |

Each gate is a discrete checkpoint with a clear question and a boolean answer. Either the evidence supports proceeding, or it doesn't.

### The Gate Contract

Gates are binary decisions backed by evidence:

- **Pass**: Evidence supports proceeding
- **Block**: Specific reason with actionable fix

Gates don't micromanage. They don't second-guess implementation choices. They verify boundaries:

- "Are there secrets in this diff?" (not "is this good code?")
- "Do tests pass?" (not "are there enough tests?")
- "Is the PR well-formed?" (not "is this the best approach?")

Quality judgments happen in upstream flows (critics, reviewers). Gates verify that those judgments happened and were addressed.

## The Three Boundaries

### 1. Secrets Boundary

**Question:** Could sensitive data leak?

**Gate:** secrets-sanitizer scans before commit/push

**Why here:** Once pushed, secrets are in git history forever. This is the last chance to catch them. A secret that reaches GitHub requires rotating credentials, notifying users, and potentially reporting breaches.

**What it checks:**

- API keys and tokens
- Passwords and credentials
- Private keys
- Connection strings
- Environment files that shouldn't be committed

### 2. Code Boundary

**Question:** Is this ready for the team to see?

**Gate:** merge-decider synthesizes evidence

**Why here:** Once merged to main, it's the team's shared reality. Other developers will build on it. Reverting becomes a coordination problem.

**What it checks:**

- Requirements were addressed (receipts exist)
- Critics signed off (no blocking issues)
- Tests pass (CI green)
- Scope is appropriate (no drive-by refactoring)

### 3. Production Boundary

**Question:** Is this safe to deploy?

**Gate:** deploy-decider checks governance + verification

**Why here:** Once deployed, users are affected. Rollback is possible but disruptive. Production issues cost real money and trust.

**What it checks:**

- Deployment prerequisites met
- Verification tests ready
- Rollback path exists
- Governance requirements satisfied

## Anti-Patterns

### Approval-Per-Action

```
Claude: "Can I read src/auth.rs?"
Human: "Yes"
Claude: "Can I read src/auth_test.rs?"
Human: "Yes"
Claude: "Can I run the tests?"
Human: "Yes"
Claude: "Can I edit src/auth.rs?"
Human: "Yes"
```

This is exhausting and pointless. Every approval costs human attention for zero security benefit. The sandbox already prevents Claude from affecting anything outside the repo.

### Pre-Constraining

"Claude can only read files in src/, cannot run commands, cannot write to..."

This removes capability without adding safety. Claude can't leak secrets by reading a config file — it can only leak them by publishing. Removing read access just makes Claude less effective while doing nothing for security.

If you're worried about Claude reading certain files, ask why those files are in the repo at all.

### Trust Without Verification

Pushing without secrets scan. Merging without evidence review. Deploying without governance check.

"We trust Claude" doesn't mean "skip the gates." It means "trust Claude to work freely in the sandbox, then verify at boundaries."

Gates exist because boundaries are where mistakes become expensive. Skipping them trades cheap verification for expensive recovery.

### Verification Without Trust

Checking every file read, every command, every edit. Requiring approval for test execution. Blocking exploration.

This treats Claude as a threat rather than a tool. The sandbox already contains the blast radius. Adding approval layers inside the sandbox just slows things down without improving safety.

If you can't trust Claude inside a sandbox, you shouldn't be using Claude at all.

## Examples

### Good: Freedom + Gate

1. Claude explores the codebase freely (reads auth, tests, configs)
2. Claude implements the feature (writes code, runs tests, iterates)
3. Claude fixes test failures (edits, reruns, adjusts)
4. Claude stages changes and commits — secrets-sanitizer scans (gate)
5. Claude pushes — repo-operator verifies hygiene (gate)
6. PR created — merge-decider reviews evidence (gate)
7. Human reviews PR description, approves
8. Merge — CI runs final checks (gate)

The human spent attention on one thing: reviewing the finished PR. Claude did all the exploration, implementation, and iteration autonomously.

### Bad: Constant Approval

1. "Can I read the requirements?" — Yes
2. "Can I read the existing auth code?" — Yes
3. "Can I run the test suite?" — Yes
4. "Can I create a new file?" — Yes
5. "Can I write the implementation?" — Yes
6. "Can I run the tests again?" — Yes
7. ...

The human spent attention on dozens of trivial approvals. Each one broke Claude's flow. The total time tripled. Zero security improvement.

### Good: Gate Catches Real Problem

1. Claude implements feature freely
2. Claude commits — secrets-sanitizer finds hardcoded API key
3. Gate blocks with specific fix: "Remove API key from line 47 of config.ts"
4. Claude removes the key, uses environment variable instead
5. Claude commits again — gate passes
6. Continue to push

The gate caught a real problem at the right moment: before the secret could leak. Claude fixed it immediately. Human never had to be involved.

### Bad: Gate Too Late

1. Claude implements feature
2. Commits without secrets scan
3. Pushes without secrets scan
4. Secret is now in GitHub history
5. Now you need to rotate credentials

The boundary was crossed without verification. Recovery is expensive.

## The Trust Architecture

```
+---------------------------------------------+
|             SANDBOX (Free)                  |
|  +---------------------------------------+  |
|  |  Read files     - no approval         |  |
|  |  Write code     - no approval         |  |
|  |  Run tests      - no approval         |  |
|  |  Run commands   - no approval         |  |
|  |  Iterate        - no approval         |  |
|  +---------------------------------------+  |
+---------------------+-----------------------+
                      |
                      v GATES
             +----------------+
             | Commit Gate    | <- secrets scan
             +--------+-------+
                      v
             +----------------+
             | Push Gate      | <- repo hygiene
             +--------+-------+
                      v
             +----------------+
             | Merge Gate     | <- evidence review
             +--------+-------+
                      v
             +----------------+
             | Deploy Gate    | <- governance check
             +----------------+
                      |
                      v
                PRODUCTION
```

Trust flows downward. Each gate is a checkpoint that verifies the work before it moves to the next level. The sandbox is where work happens. The gates are where work is verified. Production is where work has impact.

## Design Implications

### Gates Must Be Fast

If gates are slow, people skip them. A secrets scan that takes 10 minutes trains developers to commit without scanning "just this once."

Good gates:

- Run in seconds
- Provide clear pass/fail
- Give actionable feedback on failure
- Don't require human intervention to pass

### Gates Must Be Consistent

A gate that sometimes catches a problem and sometimes doesn't is worse than no gate. Developers learn to distrust it and route around it.

Good gates:

- Deterministic results for same input
- No false positives (or very rare ones)
- No false negatives (ever)

### Gates Don't Replace Quality

Gates verify boundaries, not quality. A gate that asks "does this code follow best practices?" is doing the wrong job at the wrong time.

Quality comes from:

- Good requirements (Signal flow)
- Good design (Plan flow)
- Good implementation (Build flow)
- Good critique (critics)

Gates come last and verify that quality work happened.

### Sandbox Must Be Real

A sandbox that leaks is not a sandbox. If Claude can accidentally affect production from the "sandbox," you don't have a gate pattern — you have theater.

Real sandbox:

- No production credentials
- No production network access
- Git is the only path out
- Push is the boundary

## The Economic Argument

Consider two approaches for a 50-operation feature:

**Approval-per-action:**

- 50 operations x 30 seconds human attention = 25 minutes of human time
- Plus context-switch overhead: ~10 minutes
- Plus approval latency: ~30 minutes of waiting
- Human cost: ~65 minutes
- Result: Human attention spent on low-value decisions

**Gate pattern:**

- 50 operations x 0 seconds human attention = 0 minutes
- 3 gates x 10 seconds each = 30 seconds of automated verification
- 1 PR review x 5 minutes human attention = 5 minutes
- Human cost: ~5 minutes
- Result: Human attention spent on one high-value decision

The gate pattern is 13x more efficient with human time while providing better security (automated scanning vs. human reviewing each operation).

## See Also

- [Operational Philosophy](../operational-philosophy.md) — bypassPermissions and why it works
- [What Makes This Different](../what-makes-this-different.md) — Breaking old assumptions
- [Why Two Gates](../why-two-gates.md) — The secrets + repo hygiene pattern
