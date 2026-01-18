# Model Allocation

How DemoSwarm allocates work across Claude model tiers.

---

## The Three Tiers

| Tier        | When to Use                                          |
| ----------- | ---------------------------------------------------- |
| **Haiku**   | Research, cleanup, mechanical work                   |
| **Sonnet**  | Almost-Haiku tasks that need slightly more reasoning |
| **Inherit** | Core creative work (user chooses Sonnet or Opus)     |

**Core principle:** All tiers have similar speed. Model choice is purely cost vs reasoning capacity.

---

## Haiku Tier

Research, cleanup, and mechanical work—tasks where Haiku's reasoning is sufficient.

**Examples** (not exhaustive—see `grep -l '^model: haiku' .claude/agents/*.md` for full list):

- Context distillation and search (`context-loader`)
- Receipt generation and index updates (`*-cleanup` agents)
- Test execution and output capture (`test-executor`)
- Formatting and linting (`standards-enforcer`)
- Targeted fixes from critics (`fixer`)
- Read-only GitHub research (`gh-researcher`)
- Timeline compilation (`flow-historian`)
- Coherence checks (`traceability-auditor`)

---

## Sonnet Tier

For tasks that are _almost_ Haiku-capable but need slightly more reasoning—not worth Opus cost.

**Current Sonnet agents** (small set—verify with `grep -l '^model: sonnet' .claude/agents/*.md`):

- `option-critic` — Evaluates architecture options for decision-readiness
- `pr-feedback-harvester` — Triages PR feedback (CI, bots, reviews)
- `review-worklist-writer` — Converts raw feedback into actionable worklist

These are **routing specialists** that shape what other agents work on. Poor triage cascades into wasted work, but the task itself is focused enough that Opus would be overkill.

**Candidates for Haiku demotion:** As Haiku improves, these agents may move down.

---

## Inherit Tier

Core creative and review work. The user controls the model (Sonnet or Opus).

**Examples by category** (most agents inherit—see `grep -l '^model: inherit' .claude/agents/*.md`):

- Implementation: `code-implementer`, `test-author`
- Review: `code-critic`, `test-critic`
- Design: `design-optioneer`, `adr-author`
- Requirements: `requirements-author`, `bdd-author`
- Orchestrators (flow commands)

**Why inherit?** These tasks benefit from user choice—cost vs reasoning tradeoff for your specific work.

---

## Verification

```bash
# Count agents by tier
echo "Haiku: $(grep -l '^model: haiku' .claude/agents/*.md | wc -l)"
echo "Sonnet: $(grep -l '^model: sonnet' .claude/agents/*.md | wc -l)"
echo "Inherit: $(grep -l '^model: inherit' .claude/agents/*.md | wc -l)"

# List Sonnet agents (should be small)
grep -l '^model: sonnet' .claude/agents/*.md
```

---

## See Also

- [Agents Index](agents-index.md) — Full agent catalog
- [Agent Data Flows](agent-data-flows.md) — Producer/consumer relationships
