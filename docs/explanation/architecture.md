# Architecture Overview

> How the pack is built and why.

---

## What the pack is

The DemoSwarm pack is a **Claude Code definition layer**:

- 6 flows exposed as slash commands
- 50+ agents as subagents
- 7 skills as mechanical helpers
- Validation via pack-check

The pack doesn't run code. It describes **how flows behave**; execution happens in Claude Code sessions.

---

## Key patterns

### Receipts-first

Every flow produces a receipt (`<flow>_receipt.json`).

Receipts are:
- **Mechanical:** Counts from grep/wc/parse, never estimated
- **Sealed:** Once written, reporters read them—they don't recompute
- **Canonical:** The source of truth for flow outcome

Why: Receipts prevent "re-interpret the prose" failures and ensure consistent reporting.

### Critics never fix

Critics write harsh assessments; implementers apply fixes.

```
author → artifact → critic → critique → author → improved artifact → ...
```

Why: Separation prevents "critic fixes its own issues" loops and maintains clear accountability.

### Microloops

Writer ↔ Critic iteration until:
- `status: VERIFIED`, OR
- `can_further_iteration_help: no`

Why: Bounded iteration prevents infinite loops while ensuring quality.

### Two planes

| Plane | Purpose | Example |
|-------|---------|---------|
| Audit | Durable artifacts for inspection/reruns | `.runs/<run-id>/<flow>/*` |
| Control | Machine-readable routing blocks | `## Gate Result`, `## Machine Summary` |

Why: Orchestrators route on control plane (fast, deterministic); humans inspect audit plane (rich, contextual).

---

## The six flows

| Flow | Input | Output | Purpose |
|------|-------|--------|---------|
| 1. Signal | Raw request | Requirements, BDD, risks | Shape the work |
| 2. Plan | Signal outputs | ADR, contracts, plans | Design the solution |
| 3. Build | Plan outputs | Code, tests, reviews | Implement with tests |
| 4. Gate | Build outputs | Merge decision | Pre-merge verification |
| 5. Deploy | Gate outputs | Verification, deployment | Release and verify |
| 6. Wisdom | All outputs | Learnings, regressions | Close feedback loops |

Flows can run out-of-order; missing inputs result in documented assumptions and UNVERIFIED outcomes.

---

## Agent taxonomy

| Family | Color | Behavior |
|--------|-------|----------|
| Shaping | Yellow | Early signal processing |
| Spec | Purple | Write requirements/design |
| Implementation | Green | Write code/tests/docs |
| Critic | Red | Harsh review (never fixes) |
| Verification | Blue | Audit and check |
| Analytics | Orange | Analysis and learning |
| Infra | Cyan | Git and run infrastructure |
| Reporter | Pink | GitHub posting |
| Cleanup | Various | Seal receipts, update index |

---

## Safety boundaries

### Two-gate rule

GitHub operations require:
1. `safe_to_publish: true` (secrets-sanitizer)
2. `proceed_to_github_ops: true` (repo-operator)

Why: No accidental exposure or push of unexpected content.

### Reseal pattern

If secrets-sanitizer modifies files:
```
cleanup → sanitizer → modified → cleanup → sanitizer → stable
```

Why: Receipt reflects final tree, not intermediate state.

### Safe-bail

When reseal doesn't converge:
- `checkpoint_mode: local_only`
- Never push
- Flow completes UNVERIFIED with evidence

Why: Prefer local completion over stuck or exposed state.

---

## What lives where

| Content | Location |
|---------|----------|
| Flow behavior | `.claude/commands/flow-*.md` |
| Agent behavior | `.claude/agents/*.md` |
| Shared invariants | `CLAUDE.md` |
| Mechanical helpers | `.claude/skills/*/SKILL.md` |
| Validation | `.claude/scripts/pack-check.sh` |
| Run artifacts | `.runs/<run-id>/` (in target repo) |

---

## See also

- [why-two-planes.md](why-two-planes.md) — Control vs audit plane
- [why-two-gates.md](why-two-gates.md) — GitHub ops gating
- [why-reseal.md](why-reseal.md) — Receipt correctness
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
