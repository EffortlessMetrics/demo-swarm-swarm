# Flow Flexibility

> Flows are tools, not cages.

---

## Context

This document explains the relationship between flows, humans, and the swarm. The key insight: flows exist to serve you, not the other way around.

---

## Two Different Disciplines

**Humans can bypass flows whenever they want.**

- Flows don't lock you in
- You can do work between flows (manually or with other LLMs)
- You can skip flows entirely
- You can start mid-stream
- The system is designed to accept work that was done outside the swarm

**The swarm doesn't bypass.**

- When the swarm runs, it follows its routing
- It calls agents, gets corrected by critics and verification, and keeps moving
- Internal discipline is what makes the swarm reliable
- "Blocked" routes to another agent, not to "skip this step"

This isn't a contradiction. It's **external flexibility + internal discipline**.

---

## Why This Matters

The swarm is a tool that works really well. You use it because it's effective, not because you're forced to.

If you:

- Want to write code yourself: do it, then run Flow 5 (Gate) to verify
- Have another LLM do design work: feed it into Flow 3 (Build)
- Need a hotfix NOW: commit manually, run Wisdom later
- Want to skip straight to testing: that's fine, expect UNVERIFIED markers

The flows accept work from anywhere. They don't demand provenance.

---

## How Mixed Work Flows

### Scenario: Human writes code, swarm verifies

```
Human: writes implementation manually
  Flow 3 (Build): swarm runs tests, critics review, receipt generated
  Flow 5 (Gate): swarm verifies, decides merge
```

### Scenario: External LLM does design, swarm implements

```
External LLM: produces ADR and contracts
  Human: drops artifacts into .runs/<id>/plan/
  Flow 3 (Build): swarm implements from those artifacts
```

### Scenario: Swarm does Signal, human does Plan, swarm does Build

```
Flow 1 (Signal): swarm produces requirements, BDD
  Human: writes own ADR, ignoring swarm's design-optioneer
  Flow 3 (Build): swarm implements from human's ADR
```

All of these work. The system checks what exists and proceeds.

---

## Resume-Ready Design

Every flow starts by checking what already exists:

- If artifacts are present, use them
- If artifacts are missing, note it and proceed best-effort
- If artifacts conflict, flag it for resolution

This is why mixed work is possible. The swarm doesn't assume it did the previous step.

See [architecture.md](architecture.md) for the "Every Call Is an Implicit Resume" law.

---

## What "Out-of-Order" Means

From CLAUDE.md: "Out-of-order is allowed: proceed best-effort, document assumptions, expect UNVERIFIED outcomes when upstream artifacts are missing."

This means:

- You CAN run Flow 5 without running Flows 1-4
- The system will note what's missing
- Results will be marked appropriately (UNVERIFIED, assumptions documented)
- This is a feature, not a bug

The swarm treats UNVERIFIED as a legitimate outcome. It means "we did what we could with what we had." The receipts tell you exactly what was verified and what wasn't.

---

## The Swarm's Internal Discipline

When the swarm IS running:

- It doesn't skip steps to save time
- It doesn't bypass critics because "the code looks fine"
- It routes to agents, gets corrected, routes again
- It documents what it finds, even if inconvenient

This discipline is what makes the swarm trustworthy. If it cut corners, you couldn't trust its receipts.

The swarm follows its routing because **that's what makes it useful**. Recipes you can trust because they always run the same way.

---

## Why Both Are Needed

**External flexibility** means the system serves you, not the other way around.

- You're not a prisoner of the workflow
- Emergency hotfixes don't require ceremony
- You can integrate work from any source

**Internal discipline** means the swarm's output is reliable.

- When it says "verified," it actually verified
- When it says "tests pass," tests actually ran
- When it routes to a critic, the critic actually critiques

You get flexibility for humans AND reliability from automation.

---

## Entry Points

Different situations call for different starting points:

| Situation                 | Start From         | Why                              |
| ------------------------- | ------------------ | -------------------------------- |
| New feature from scratch  | Flow 1 (Signal)    | Shape requirements before design |
| Design already exists     | Flow 3 (Build)     | Skip to implementation           |
| Code already written      | Flow 5 (Gate)      | Just verify what exists          |
| Quick fix, already tested | Commit manually    | Flow 7 (Wisdom) later            |
| Hotfix emergency          | Whatever's fastest | Document gaps afterward          |

The seven flows are a complete pipeline, but they're not a mandatory sequence.

---

## Anti-Patterns

**Don't:**

- Feel obligated to run all 7 flows for a typo fix
- Reject human-written code because "it didn't go through Signal"
- Force-fit work into flows where it doesn't belong
- Skip flows and then expect VERIFIED markers

**Do:**

- Use flows when they add value
- Accept work from any source
- Run verification (Gate) on anything you want confidence in
- Document when you've skipped steps

---

## The Economics

Machine iteration is cheap. Human attention is expensive.

When you run flows, you're spending machine time to compress the human review burden. But if the machine work isn't adding value (typo fix, emergency hotfix, already-tested change), you're not obligated to run it.

The question is always: "Does running this flow produce evidence that makes review cheaper?" If yes, run it. If no, skip it.

---

## Summary

Flows are a tool. A very good tool. Use them because they work, not because you must.

Humans operate with flexibility. The swarm operates with discipline. Both are correct for their context.

The swarm accepts your work regardless of where it came from. It verifies what it can, marks what it can't, and gives you honest receipts. That's the contract.

---

## See Also

- [CLAUDE.md](../../CLAUDE.md) — Pack reference (the "out-of-order" policy)
- [architecture.md](architecture.md) — Resume-ready design and architecture laws
- [routing-table.md](../reference/routing-table.md) — How agents route between flows
