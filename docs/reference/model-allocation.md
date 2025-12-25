# Model Allocation Strategy

This document describes how DemoSwarm allocates work across Claude model tiers.

## Overview

DemoSwarm uses a three-tier allocation strategy:

| Tier | Use Case |
|------|----------|
| **Haiku** | Research, cleanup, mechanical work |
| **Sonnet** | High-judgment routing and triage (few agents) |
| **Inherit** | Core creation/review (user-configurable, majority of agents) |

Run the verification commands below for current counts.

## Allocation Philosophy

**Core principle:** Speed is a commodity. Model choice is driven by **cost-to-reasoning ratio**, not speed.

The gap between Sonnet and Haiku is not about speed—both are fast. It's about balancing cost against the reasoning complexity required for the task.

## Tier Definitions

### Haiku Tier

**Use for:** High-speed execution, research, cleanup, mechanical work

Haiku agents handle:
- Context distillation and RAG-style search
- Mechanical receipt generation and index updates
- Test execution and output capture
- Linting and formatting (deterministic operations)
- Timeline compilation and audit checks
- Read-only GitHub reconnaissance

**Examples:**
- `context-loader` — RAG-style search and context distillation
- `*-cleanup` agents — Mechanical receipt generation and index updates
- `test-executor` — Run tests and capture output
- `standards-enforcer` — Run formatters/linters (mechanical)
- `fixer` — Apply targeted fixes from critics (mechanical)
- `traceability-auditor` — Coherence checks (mechanical)
- `flow-historian` — Timeline compilation (mechanical)
- `gh-researcher` — Read-only GitHub reconnaissance

### Sonnet Tier

**Use for:** High-judgment routing and triage

Sonnet is reserved for agents that make **complex routing decisions** with high downstream impact:

1. **`option-critic`** — Reviews architecture options for decision-readiness
2. **`pr-feedback-harvester`** — Harvests and triages PR feedback (CI, bots, reviews)
3. **`review-worklist-writer`** — Converts raw feedback into actionable worklist

These agents are **routing specialists** that shape what other agents work on. Poor routing decisions cascade into wasted work.

### Inherit Tier (majority)

**Use for:** Core creation and review work (user-configurable)

Most agents default to `model: inherit`, which respects the user's configured model preference. This tier includes:

- **Implementation agents:** `code-implementer`, `test-author`
- **Review agents:** `code-critic`, `test-critic`
- **Design agents:** `design-optioneer`, `adr-author`
- **Requirements agents:** `requirements-author`, `bdd-author`
- **Orchestrators:** Flow commands that route and mediate

This design allows users to:
- Use Haiku for cost-sensitive development
- Use Sonnet for higher-quality output
- Use Opus for complex architectural work

The pack doesn't enforce a model choice for core creative work—**you decide the cost-to-quality tradeoff**.

## Why Only 3 Sonnet Agents?

Sonnet is expensive. The pack uses it **only where judgment has high leverage**:

- **Routing decisions** (option-critic, pr-feedback-harvester, review-worklist-writer) shape what other agents work on
- Poor triage cascades into wasted work downstream
- Haiku can't reliably distinguish "valid blocker" from "bot noise" at the same cost point

For **creative work** (implementation, design, requirements), the pack uses `inherit` instead of forcing Sonnet. This gives users control over their cost-to-quality tradeoff.

## Model Selection Principles

When considering model allocation for new agents:

1. **Is this mechanical?** → Haiku
   - Deterministic operations (format, lint, count)
   - Research and context gathering
   - Receipt and index updates

2. **Is this a high-leverage routing decision?** → Sonnet
   - Triage with high downstream impact
   - Option evaluation that shapes architecture
   - Feedback classification that determines worklists

3. **Is this core creative/review work?** → Inherit
   - Implementation and testing
   - Design and requirements authorship
   - Code and design review

## Cost Impact

Relative cost (check [Anthropic pricing](https://www.anthropic.com/pricing) for current rates):

- **Haiku:** Baseline
- **Sonnet:** ~10-15x Haiku
- **Opus:** ~50-60x Haiku

The minimal Sonnet allocation means:
- Most agents run at Haiku or user-controlled cost
- High-judgment routing gets proportionally more reasoning budget
- Users control the cost-to-quality tradeoff for creative work

## Verification

To verify the current model allocation:

```bash
# Count agents by model tier
echo "Total: $(ls .claude/agents/*.md | wc -l)"
echo "Haiku: $(grep -l '^model: haiku' .claude/agents/*.md | wc -l)"
echo "Sonnet: $(grep -l '^model: sonnet' .claude/agents/*.md | wc -l)"

# List Sonnet agents
grep -l '^model: sonnet' .claude/agents/*.md
```

The Sonnet tier should remain small. These are agents that need slightly more reasoning than Haiku can reliably handle today — candidates for demotion to Haiku as model capabilities improve.

## See Also

- [Agents Index](../../.claude/agents/index.md) — Full agent catalog with model guidance
- [Agent Data Flows](agent-data-flows.md) — How agents interact
- [Contracts](contracts.md) — Machine Summary and Result block schemas
