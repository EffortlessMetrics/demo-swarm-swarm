# Economics

> The math that makes this system work economically.

---

## The Core Equation

**DevLT (Dev Lead Time)** = Minutes of developer attention per trusted change

This is the metric that matters. Not lines generated. Not tokens consumed. Not calendar time.

**DevLT is the only metric that matters. Everything else is vanity.**

---

## The Economics

### The Inversion

Traditional development:
- Human writes code: expensive
- Human reviews code: expensive
- Human runs tests: expensive
- Everything depends on human throughput

AgOps development:
- Machine generates code: cheap (~$0.05/1k lines)
- Machine verifies code: cheap (~$0.10/verification loop)
- Machine produces evidence: cheap
- Human reviews evidence: moderate
- Human makes ship decision: valuable (this is what we optimize)

### The Math

**Old model (hand-written PR):**
```
Feature: 2000 lines
Writing time: 8 hours ($400 @ $50/hr)
Review time: 2 hours ($100)
Test/debug: 4 hours ($200)
Total: $700, 14 hours
```

**AgOps model (generated + verified PR):**
```
Feature: 2000 lines
Generation: 500k tokens (~$5)
Verification loops: 10 iterations (~$10)
Evidence generation: (~$5)
Human review of evidence: 30 min ($25)
Total: $45, 30 minutes human time
```

The 10x cost savings matter. The 28x time savings matter more.

### Token Economics

| Operation | Tokens | Cost (approx) |
|-----------|--------|---------------|
| Generate 500 LOC | ~50k | $0.50 |
| Critic review | ~20k | $0.20 |
| Test execution + report | ~10k | $0.10 |
| Full microloop iteration | ~100k | $1.00 |
| Complete flow (Signal to Gate) | ~2M | $20 |

**Key insight:** A full flow costs less than 10 minutes of senior dev time.

### The Real Comparison

```
1 hour senior dev time: $100-250
1 hour of swarm operation: $5-20

Senior dev reviewing 70k LOC: 40 hours ($4,000-10,000)
Senior dev reviewing evidence: 30 minutes ($25-60)
```

---

## Why This Works

### Tokens Are Cheap

Model inference is a commodity. Prices drop continuously. The trend is toward near-zero marginal cost for generation.

### Attention Is Expensive

Senior developer attention is the scarcest resource in software. Every minute spent on mechanical review is a minute not spent on architecture, design, or genuine problem-solving.

### Verification Scales

Machine verification (tests, critics, scans) scales linearly with tokens spent. Human verification scales with attention—which doesn't scale.

### Evidence Compresses

A 70,000 line change becomes:
- 5-bullet summary
- 10-row evidence table
- 3 hotspot pointers
- 1 ship/no-ship recommendation

That's 30 seconds to scan, not 40 hours to read.

---

## DevLT Optimization Strategies

### Spend Tokens to Save Attention

- Run more verification loops (cheap)
- Generate better evidence summaries (cheap)
- Produce cleaner PR descriptions (cheap)

All to reduce the human review burden (expensive).

### Front-Load Quality

Shift-left with:
- BDD scenarios (intent before code)
- Critics (find issues before human sees them)
- Mutation testing (prove tests work)

Issues caught by machines don't require human attention.

### Compress Ruthlessly

- Cleanup agents summarize, don't dump
- Receipts highlight, don't enumerate
- PR cockpits orient, don't overwhelm

Every unnecessary line increases DevLT.

---

## The ROI Calculation

For a team considering adoption:

```
Current state:
- 10 PRs/week
- 2 hours review each = 20 hours/week
- @ $75/hour = $1,500/week on review

With AgOps:
- 10 PRs/week
- 30 min review each = 5 hours/week
- @ $75/hour = $375/week on review
- + $200/week token costs

Savings: $925/week = $48k/year (per reviewer)
```

Plus: faster ship times, fewer production issues, better documentation.

---

## The Meta-Point

**We're not trying to replace developers. We're trying to replace toil.**

The senior architect still:
- Sets direction
- Makes judgment calls
- Handles true ambiguity
- Decides what ships

The system handles:
- Mechanical implementation
- Verification grinding
- Evidence generation
- Documentation production

Architects architect. The system grinds.

---

## See Also

- [Why Ops-First](why-ops-first.md) — Engineering default-allow philosophy
- [Agent Philosophy](agent-philosophy.md) — How agents work
- [What Makes This Different](what-makes-this-different.md) — Assumptions that don't apply here
- [PR as Review Surface](pr-as-review-surface.md) — Why evidence compression matters
