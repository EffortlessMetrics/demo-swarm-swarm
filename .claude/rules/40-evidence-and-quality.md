# Evidence and Quality

> Claims require pointers. The PR cockpit is the primary review surface.

---

## The Evidence Discipline

**Law 4: Evidence Over Trust**

Claims require pointers. Receipts summarize tool outputs. "Not measured" is acceptable. False certainty is not.

### The Standard

| Good | Bad |
|------|-----|
| "Tests pass (see `test_execution.md`: 15 passed, 0 failed, exit code 0)" | "I ran the tests and they all pass" |
| "Coverage: 78% (see coverage_audit.md)" | "Coverage looks good" |
| "Not measured: mutation testing skipped (no budget)" | "Tests are probably effective" |

Uncertainty is fine when labeled. Certainty without evidence is the failure mode.

---

## The Quality Panel

### Why a Panel (Not One Metric)

Any single metric can be gamed (intentionally or accidentally):
- "Passes tests" can hide complexity bloat
- "High coverage" can hide hollow tests
- "Mutation score" can be gamed with bad structure

We use a **panel** that collectively approximates "good engineering."

### Required PR Cockpit Surfaces

Include (or link to) evidence for:

| Surface | What It Shows |
|---------|---------------|
| **Intent fidelity** | REQ/BDD mapping; what changed and why |
| **Verification depth** | Tests + mutation; are the tests meaningful? |
| **Maintainability** | Complexity deltas, churn hotspots, idioms |
| **Safety** | Secrets scan, boundary notes |
| **Operability/NFRs** | Perf budgets, memory, binary size (where relevant) |
| **Explainability** | Risks, assumptions, "not measured", next steps |

### Claims Policy

- **Important claims must have a pointer:** file path, command output, artifact link
- **"Not measured" is allowed:** Be explicit about what was skipped and why
- **Silent assertion is not allowed:** Never imply certainty without evidence

---

## The North Star

**PR Quality + Maintainability** with **minimal human review time**.

This is not just speed. Not just quality. The *efficiency* of producing quality.

### The Ratio

**Quality:DevLT** = PR quality achieved per minute of developer attention

### The Trade-Off

Machine time is cheap. Dev time is expensive.

| Option A | Option B | Winner |
|----------|----------|--------|
| 4 hrs machine, 30 min dev | 8 hrs machine, 25 min dev | **B** (probably) |
| 2 hrs machine, 45 min dev | 6 hrs machine, 20 min dev | **B** (definitely) |
| 1 hr machine, 60 min dev | 4 hrs machine, 15 min dev | **B** (absolutely) |

More verification loops, more critic passes, more evidence generation = more machine time but less dev review time. That's almost always the right trade.

---

## Verification Strategies

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

## What Reviewers Actually Need

Most reviewers will only read the PR description. Treat it as the cockpit display.

| They Need | They Don't Need |
|-----------|-----------------|
| What changed (summary) | Every line that changed |
| Why it changed (intent link) | The full requirements doc |
| What was verified (evidence table) | Raw test output |
| Where to spot-check (hotspots) | Comprehensive file list |
| What's not measured (explicit gaps) | Silent assumptions |

The diff is for spot-checks and audits. The cockpit is for decisions.

---

## See Also

- [claims-and-evidence.md](../../docs/explanation/claims-and-evidence.md) — Full evidence discipline
- [pr-as-review-surface.md](../../docs/explanation/pr-as-review-surface.md) — Why the PR cockpit matters
- [economics.md](../../docs/explanation/economics.md) — The math behind the trade-offs
