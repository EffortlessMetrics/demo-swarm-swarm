# Economics

> The math that makes this system work economically.

---

## The Core Metrics

### North Star: PR Quality + Maintainability

The ultimate goal is **high-quality, maintainable PRs** that meet modern standards:
- Well-tested (mutation score, coverage, BDD alignment)
- Well-documented (clear PR description, evidence, context for future readers)
- Well-structured (reviewable, modular, follows codebase patterns)
- Maintainable (readable code, clear intent, sustainable architecture)
- Verified (critics passed, gates cleared)

### The Ratio: Quality per DevLT

**Quality:DevLT** = PR quality achieved per minute of developer attention

This ratio is what we optimize. Not just speed. Not just quality. The *efficiency* of producing quality.

### The Full Picture

It's not just one ratio—it's multiple angles that add up:

| Dimension | What We Optimize |
|-----------|------------------|
| **Change Size** | Large, meaningful changes (not tiny PRs) |
| **Trust Level** | Verified, tested, critiqued (not "hope it works") |
| **Evidence Quality** | Clear proof of what was done (not just claims) |
| **Human Time** | Minimal architect attention required |
| **Scope of Automation** | Planning + testing + verification, not just coding |

**In practice:** Agents do the testing, verification, and planning—not just the coding. Humans review large trusted changes with minimal time investment because the work is already verified.

### Verification Arbitrage

**Burn cheap compute to buy back expensive attention.**

Machine time is cheap. Dev time is expensive. The arbitrage: spend infinite cheap tokens to save scarce human minutes.

| Option A | Option B | Winner |
|----------|----------|--------|
| 4 hrs machine, 30 min dev | 8 hrs machine, 25 min dev | **B** (probably) |
| 2 hrs machine, 45 min dev | 6 hrs machine, 20 min dev | **B** (definitely) |
| 1 hr machine, 60 min dev | 4 hrs machine, 15 min dev | **B** (absolutely) |

We don't care if the AI generates 500,000 lines of garbage to produce 100,000 lines of gold. The garbage costs nothing. The gold is verified.

More verification loops, more critic passes, more evidence generation = more machine time but less dev review time. That's the arbitrage.

**Within reason:** There's a point of diminishing returns. You wouldn't spend 100 hours of machine time to save 5 minutes of dev time. But in the typical range (hours of machine time vs tens of minutes of dev time), lean toward more machine grinding.

**The flywheel:** The more we push to machine and automation, the more we can improve and speed up those things in the future. Human workflows are hard to optimize. Machine workflows are just code—profile them, optimize them, parallelize them, cache them. Every hour of work moved from human to machine becomes an hour we can make faster tomorrow.

```
Today: 8 hrs machine time
Tomorrow: Optimize critic loop → 6 hrs
Next month: Parallelize testing → 4 hrs
Next quarter: Smarter caching → 2 hrs
```

Dev time doesn't compress like that. Machine time does.

**The exception:** When calendar time matters (urgent hotfix), you might accept more dev time to ship faster. But for normal work, burn machine hours to save dev minutes.

A 2000-line PR that's fully tested, mutation-verified, and critic-approved—reviewable in 30 minutes—is better than twenty 100-line PRs requiring 15 minutes each.

### DevLT (Dev Lead Time)

**DevLT** = Minutes of developer attention per trusted change

Not lines generated. Not tokens consumed. Not calendar time. Developer minutes.

---

## The Manufacturing Thesis

The factory produces trust, not code.

Code is a byproduct. The actual product is a PR that is boring to approve because its review surface is self-evidently trustworthy. A 70k-line PR becomes boring not because it's simple, but because the verification panel answers every question the reviewer would have asked.

### What We Actually Make

| What People Think | What We Actually Produce |
|-------------------|-------------------------|
| Code | Evidence that code is correct |
| Features | Verified changes with clear intent |
| PRs | Review surfaces that compress decision time |
| Automation | Trust on demand |

### The Process Inversion

Traditional development works forward from code:

```
Write code → Hope it's correct → Review catches problems → Fix → Ship
```

Swarm development works forward from intent:

```
Define intent → Generate candidates → Verify exhaustively → Ship what passes
```

The verification IS the product. Code is the artifact that verification produces.

This inverts the review conversation. Instead of "is this code correct?" the question becomes "does the evidence satisfy me?" The reviewer audits the cockpit, not the diff.

### The Triangle (No Tradeoffs)

```
      Quality + Maintainability
              /\
             /  \
            /    \
           /      \
    DevLT -------- Velocity
```

Traditional systems pick two. We reject the tradeoff.

The trick: machine time is the knob. Spend it to buy the other two.

This is why "The Trade-Off That Matters" above isn't really a tradeoff—it's an arbitrage. Machine time is cheap and getting cheaper. Dev time is expensive and staying expensive. Converting one to the other at favorable rates is just good economics.

### Quality:DevLT as the Lens

Quality:DevLT (PR quality achieved per minute of developer attention) is the north star metric because it captures the manufacturing thesis in one ratio. Everything else serves it:

- More verification loops → higher quality, same dev time
- Better cockpit → faster review, same quality
- Mutation testing → higher confidence, same review burden

When evaluating any change to the system, ask: does this improve Quality:DevLT? If yes, it's probably worth the machine time. If no, reconsider.

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
