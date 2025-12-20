# Architecture Overview

> How the pack is built and why.

---

## What the pack is

The DemoSwarm pack is a **Claude Code definition layer**:

- 7 flows exposed as slash commands
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

## The seven flows

| Flow | Input | Output | Purpose |
|------|-------|--------|---------|
| 1. Signal | Raw request | Requirements, BDD, risks | Shape the work |
| 2. Plan | Signal outputs | ADR, contracts, plans | Design the solution |
| 3. Build | Plan outputs | Code, tests, reviews | Implement with tests |
| 4. Review | Build outputs + Draft PR | PR feedback, worklist | Harvest PR feedback |
| 5. Gate | Review outputs | Merge decision | Pre-merge verification |
| 6. Deploy | Gate outputs | Verification, deployment | Release to mainline |
| 7. Wisdom | All outputs | Learnings, regressions | Close feedback loops |

Flows can run out-of-order; missing inputs result in documented assumptions and UNVERIFIED outcomes.

### Flow command variants

Some flows have alternative entry points for different contexts:

| Variant | Primary Flow | Use When |
|---------|--------------|----------|
| `/flow-4-review` | 4. Review | After PR feedback arrives; harvests comments before Gate |
| `/flow-5-gate` | 5. Gate | Re-running gate checks (e.g., after fix-forward) |
| `/flow-6-deploy` | 6. Deploy | Merge + deploy to mainline |
| `/flow-7-wisdom` | 7. Wisdom | Second-cycle wisdom extraction for multi-iteration runs |

The primary sequence is: `/flow-1-signal` → `/flow-2-plan` → `/flow-3-build` → `/flow-4-review` → `/flow-5-gate` → `/flow-6-deploy` → `/flow-7-wisdom`

### Flow 7: Second-cycle wisdom

Flow 7 (`/flow-7-wisdom`) is for **second-cycle wisdom extraction**. Use it when:
- Multiple runs have completed and you want to synthesize cross-run learnings
- An iteration has finished and you want to extract batch insights
- You've already run `/flow-6-deploy` and want a deeper retrospective

Flow 7 differs from the wisdom extraction in Flow 6 in that it's designed for **post-cycle reflection** rather than immediate run closure.

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

## Security posture

### Regex safety

The secrets scanner uses the **Rust regex crate**, which implements finite automata (not backtracking). This makes it immune to ReDoS (Regular Expression Denial of Service) attacks. The regex engine has guaranteed linear time complexity relative to input size.

Reference: `tools/demoswarm-runs-tools/src/secrets.rs` uses the `regex` crate.

### Known limitations

**Path traversal in secrets scanner**: The secrets scanner operates on provided paths without full canonicalization. This is a known limitation in the local execution context. Mitigation: The scanner runs within Claude Code sessions where filesystem access is already scoped. A formal threat assessment for production deployments is recommended.

---

## Test status

Test counts are **receipt-derived** (mechanical, from actual execution). Current baseline:
- Unit tests: derived from `cargo test --workspace` execution
- Test counts should be read from `test_summary.md` or receipt artifacts, not hard-coded in documentation

This ensures documentation stays aligned with actual test results.

---

## Agent metadata

### Color coding

Agent frontmatter includes a `color:` field for categorization:

```yaml
---
name: requirements-author
description: Write functional + non-functional requirements
color: purple
---
```

Color coding is **advisory metadata** for human consumption and tooling visualization. It is not schema-enforced or used for routing decisions. The taxonomy table above shows color conventions by agent family.

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
