# Reviewing as Audit

> The skill shift from diff-reading to evidence evaluation.

---

## The Old Model

Traditional code review:
- Read the diff line by line
- Hold the whole change in your head
- Check for bugs, style, logic errors
- Hope you catch what matters

This scales with change size. A 100-line change takes 10 minutes. A 10,000-line change takes... too long.

---

## The New Model

Evidence-based review:
- Read the cockpit (PR description, evidence panel)
- Verify evidence is fresh and attributable
- Spot-check 3-8 hotspots based on risk
- Decide: merge, bounce, or request specific fixes

This scales with evidence quality, not change size.

---

## The Skill Set

### 1. Evidence Sufficiency

For each major claim, ask:
- **Is there evidence?** (pointer to artifact, not just assertion)
- **Is the evidence fresh?** (SHA matches HEAD)
- **Is the evidence attributable?** (can trace back to tool output)
- **Is the evidence meaningful?** (does this actually prove the claim)

### 2. Risk-Based Spot-Checking

You can't read everything. So you read what matters:

| Risk Signal | What to Spot-Check |
|-------------|-------------------|
| Security-sensitive paths | Auth, crypto, user input handling |
| High complexity delta | Files with significant cyclomatic complexity increase |
| Churn hotspots | Files that have caused problems before |
| Public API changes | Contract-breaking modifications |
| Critic-flagged issues | Items marked MAJOR or CRITICAL in critiques |

### 3. Freshness Discipline

| Status | Action |
|--------|--------|
| FRESH | Trust the evidence |
| ACCEPTABLE_STALE | Trust with awareness |
| STALE | Request re-verification |
| UNKNOWN | Request verification |

Stale evidence is not worthless, but it's not current proof either.

### 4. Panel Reading

The quality panel shows multiple sensors:

| Surface | Question It Answers |
|---------|---------------------|
| Intent fidelity | Did we build what we meant to build? |
| Verification depth | Are the tests meaningful? (mutation score helps) |
| Maintainability | Is the code worse or better than before? |
| Safety | Are boundaries respected? |
| Operability | Will this work in production? |
| Explainability | Can I understand what happened? |

A single green checkmark can be gamed. A multi-sensor panel is much harder to fake.

---

## The Questions to Ask

1. **What evidence proves the major claims?**
   - Not "agent said tests pass" but "test_execution.md shows exit code 0"

2. **What's explicitly "not measured"?**
   - Acceptable if declared and reasonable
   - Suspicious if major areas are silently absent

3. **What are the hotspots?**
   - PR cockpit should identify 3-8 files worth spot-checking
   - If not identified, ask for them

4. **Is anything stale or missing?**
   - Check evidence_sha against current HEAD
   - Treat missing evidence as UNKNOWN, not PASS

5. **What would I do if this were wrong?**
   - Is the change reversible?
   - Is there a rollback plan?
   - What's the blast radius?

---

## Anti-Patterns

### Reading Every Line

- **Why it's wrong:** Doesn't scale, creates false confidence
- **What to do instead:** Read hotspots, trust verified evidence for the rest

### Trusting Narrative Over Evidence

- **Why it's wrong:** Narrative is Layer 5 (lowest trust)
- **What to do instead:** Check for pointers; no pointer = no evidence

### Ignoring "Not Measured"

- **Why it's wrong:** May hide important gaps
- **What to do instead:** Evaluate whether the gap is acceptable for this change's risk profile

### Green-CI-Means-Ship

- **Why it's wrong:** CI is one sensor; mutations, critics, and humans are others
- **What to do instead:** Use the full panel

---

## The Mindset

You are a **cockpit pilot**, not a proofreader.

Your instruments (evidence panel, receipts, critiques) tell you the state of the system. Your job is:
1. Verify the instruments are reading correctly (fresh, attributable)
2. Interpret what they mean together (panel synthesis)
3. Spot-check where risk is high (hotspots)
4. Make the call (merge/bounce)

This is audit thinking applied to code review.

---

## See Also

- [pr-as-review-surface.md](pr-as-review-surface.md) — Why the PR cockpit matters
- [truth-hierarchy.md](truth-hierarchy.md) — The evidence hierarchy
- [../how-to/review-a-swarm-pr.md](../how-to/review-a-swarm-pr.md) — The decision procedure
