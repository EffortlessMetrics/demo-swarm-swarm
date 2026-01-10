# Operational Philosophy

> How we run Claude Code, and why it enables high-velocity development.

## The Core Insight

**Claude is a trusted team member, not a restricted tool.**

Most agent systems treat the AI as dangerous — constrained, supervised, limited. We treat Claude as a capable colleague who has been given a sandbox to work in freely.

This isn't reckless. It's intentional organizational design.

---

## bypassPermissions: Intentional Trust

### What We Do

We run Claude Code with `bypassPermissions` enabled. Claude can:
- Read and write files freely
- Run commands without approval
- Execute multi-step operations autonomously

### Why This Works

**The sandbox is the boundary, not the prompt.**

We don't constrain Claude's moment-to-moment actions. Instead:
- The repository is the sandbox
- Gates engage at publish boundaries (commit, push, GitHub post)
- Claude has freedom to explore, implement, iterate within that sandbox

This is exactly how you'd treat a trusted new hire:
- "Here's your dev environment. Experiment freely."
- "When you're ready to ship, we'll review."

### The Alternative (and why it fails)

Approval-per-action systems:
- Interrupt flow constantly
- Reduce Claude to a suggestion engine
- Make iteration expensive
- Treat capability as threat

We'd rather have Claude try 10 things and find what works than ask permission for each step.

---

## Engineering Is Default-Allow, Publishing Is Gated

### The Principle

Inside the sandbox: **freedom**
- Explore the codebase
- Try implementations
- Run tests
- Fix issues
- Iterate rapidly

At the boundary: **gates**
- Secrets scanning before commit
- Repo hygiene before push
- Review before merge
- Verification before deploy

### Why This Pattern

**Machine time is cheap. Human attention is expensive.**

Let Claude iterate rapidly on implementation. Only engage human judgment at decision points:
- "Should we ship this?" (Gate)
- "Is this approach right?" (Design review)
- "Does this solve the problem?" (Acceptance)

Don't spend human attention on:
- "Can Claude read this file?" (Yes)
- "Can Claude run this test?" (Yes)
- "Can Claude try this fix?" (Yes)

---

## Agile Principles Applied to Agents

### Individuals Over Process

We don't constrain Claude with rigid processes. We give it:
- Clear goals
- Helpful guidance
- Freedom to achieve them

The agent decides HOW to accomplish the work, not just WHAT to type.

### Working Software Over Documentation

Agents focus on producing working code, not satisfying schemas. Artifacts exist because they're useful, not because they're required.

### Responding to Change

Agents adapt to what they find. If the implementation reveals a problem with the design, they can:
- Fix it locally
- Flag it for discussion
- Recommend an alternative

They don't blindly follow a plan that no longer makes sense.

### Collaboration Over Contracts

Agent communication is natural language, not rigid contracts. The orchestrator and agents collaborate through understanding, not protocol compliance.

---

## Teal Organization Principles

### Self-Management

Agents manage their own work. They don't wait for micromanagement:
- Read the context
- Understand the goal
- Do the work
- Report back

### Wholeness

Agents bring their full capability. We don't artificially limit them to narrow tasks or forbid them from noticing problems outside their scope.

### Evolutionary Purpose

The system evolves. Agents can observe friction, suggest improvements, and contribute to making the process better.

---

## Trust Architecture

### Trust Is Efficient

Constantly verifying is expensive. We trust by default:
- Trust agents to do reasonable things
- Trust that file reads are appropriate
- Trust that commands are well-intentioned

### Verify at Boundaries

When trust meets the outside world, verify:
- Before secrets could leak (publish gate)
- Before changes become permanent (merge gate)
- Before production is affected (deploy gate)

### Recovery Over Prevention

We don't try to prevent all possible mistakes. Instead:
- State is on disk (recoverable)
- Git is the safety net (revertible)
- Human review at boundaries (catchable)

A mistake in the sandbox is cheap. A mistake in production is expensive. Gate accordingly.

---

## What This Enables

### High Velocity

Claude can:
- Try multiple approaches rapidly
- Run tests without asking
- Fix issues immediately
- Iterate to working code

Without:
- Permission dialogs
- Approval workflows
- Stop-and-wait cycles

### Deep Focus

Claude can:
- Maintain context across long operations
- Follow a complex implementation through
- Handle multi-file changes atomically

Without:
- Constant interruption
- Context loss from approval cycles
- Fragmented attention

### Quality Through Iteration

More iterations = better code:
- Try something, see if it works
- Run tests, fix failures
- Get critique, address issues
- Repeat until solid

This is only possible with freedom to iterate.

---

## The Counter-Argument (and our response)

### "But what if Claude does something wrong?"

It will. That's fine. The sandbox is the boundary.

- Wrong file edit? Git diff will show it.
- Wrong command? It runs in the sandbox.
- Wrong approach? Tests will fail.

We catch mistakes at gates, not at every keystroke.

### "But what about security?"

- Secrets scanning at publish gate
- No credentials in the sandbox
- Repo-level isolation
- Human review before merge

Security comes from architecture, not permission dialogs.

### "But what about runaway operations?"

- Timeouts on commands
- Resource limits on the environment
- Human can interrupt at any time

The sandbox has boundaries.

---

## How to Adopt This

If you want to run like this:

1. **Set up a proper sandbox** — Isolated environment, no production access
2. **Enable bypassPermissions** — Let Claude work freely
3. **Configure gates** — Secrets, lint, tests before publish
4. **Trust the process** — Let Claude iterate

You'll be surprised how much more gets done.

---

## See Also

- [Claude-Native Design](claude-native-design.md) — The design philosophy
- [What Makes This Different](what-makes-this-different.md) — Breaking assumptions
- [Architecture](architecture.md) — Technical implementation
