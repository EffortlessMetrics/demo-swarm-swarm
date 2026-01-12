# PR Quality Scorecard

> Quality is multi-surface. Unknown is normal. Evidence replaces claims.

---

## Philosophy

"Quality" is vague. This document replaces vague quality assertions with **inspectable surfaces** that can be measured, estimated, or explicitly marked as unknown.

The goal is not exhaustive measurement. The goal is honest communication about what was verified, what was estimated, and what remains unknown.

**We start from intent, but we arbitrate truth from evidence.** Intent (BDD, ADR, contracts) defines what we're trying to build. Evidence (test results, mutation scores, exit codes) proves whether we built it. When narrative claims conflict with evidence, evidence wins.

---

## The Five Quality Surfaces

### 1. Correctness Surface

**What it answers:** Does the code do what it's supposed to do?

| Inspection Point | What to Check |
|------------------|---------------|
| BDD scenarios | Do acceptance criteria pass? |
| Test results | What do tests cover? What passed/failed? |
| Regression risk | Are there known risky areas? Edge cases? |

**Evidence pointers:** `test_execution.md`, BDD scenario results, gate receipt.

### 2. Verification Depth Surface

**What it answers:** How hard is it for the tests to lie?

| Inspection Point | What to Check |
|------------------|---------------|
| Test delta vs logic delta | Did verification grow with behavior? |
| Mutation testing | If run: survivors, score, hotspots |
| Fuzz/property tests | If used: coverage, findings |

**Key insight:** Coverage tells you what ran. Mutation tells you if running mattered.

**Evidence pointers:** Mutation report, fuzz results, test diff.

**Explicit "not measured" is acceptable here.** Mutation testing is expensive. Not running it is a valid choice when documented.

### 3. Boundary Discipline Surface

**What it answers:** Did contracts and dependencies stay stable (or change intentionally)?

| Inspection Point | What to Check |
|------------------|---------------|
| API/schema drift | None, additive, or breaking? |
| Dependency changes | New deps? Version bumps? Feature flags? |
| Layering violations | If enforced: any cross-layer calls? |

**Evidence pointers:** `api_contracts.yaml` diff, dependency lockfile diff, layer check output.

**Additive changes are normal.** Breaking changes require explicit version bumps.

### 4. Maintainability Surface

**What it answers:** Will the next reader (human or agent) understand this code?

| Inspection Point | What to Check |
|------------------|---------------|
| Hotspots | Which files deserve reviewer attention? |
| Complexity/coupling | Did it get worse, better, or stay stable? |
| Module boundaries | Splits, refactors, de-risking moves |
| Future reader notes | Comments, ADR references, context |

**Evidence pointers:** Hotspot list in PR Brief, complexity metrics, ADR.

**Hotspots guide review.** A 500-line diff with 3 hotspots called out is reviewable. A 500-line diff with no guidance is not.

### 5. Explanation Surface

**What it answers:** Can a reviewer reach a trust decision quickly?

| Inspection Point | What to Check |
|------------------|---------------|
| PR cockpit readability | Is the PR Brief scannable in 30 seconds? |
| Evidence pointers | Does every claim have a pointer? |
| Explicit unknowns | Are "not measured" items documented? |

**Evidence pointers:** The PR description itself.

**No claim without a pointer.** "Tests pass" means nothing without a link to test results.

---

## The Panel Beats Goodhart

Any single metric can be gamed:

| If You Optimize For... | You Risk... |
|------------------------|-------------|
| Coverage | Hollow tests that touch lines without asserting |
| Mutation score | Over-specific assertions that kill mutants but miss edge cases |
| Green CI | Untested surfaces, flaky test hiding |
| Clean lint | Compliant but complex code |
| Fast review | Rubber-stamping without reading |

The quality panel uses multiple sensors that fail in different ways. If you game one, another goes red.

This is deliberate. The panel collectively approximates "good engineering" in a way no single metric can.

### Trust Decay Prevention

Over time, reliable systems become less scrutinized. Prevent trust decay with:
- Periodic calibration (inject known defects, verify gates catch them)
- Explicit "not measured" (force attention to gaps)
- Red flags (patterns that force slowdown)

See [calibration.md](calibration.md) for the full calibration protocol.

---

## The Core Ratio: Quality per DevLT

**Quality** = multi-surface (the 5 above)
**DevLT** = architect minutes to reach a trust decision

The ratio we optimize: **maximum quality confidence per minute of human attention**.

Machine time is a lever. Spend it to compress DevLT:
- More verification loops reduce review burden
- Better evidence summaries reduce reading time
- Clearer hotspot pointers reduce search time

**This is a lens, not the only lens.** DevLT optimization doesn't replace correctness or maintainability. It's how we think about the economics of verification.

See: [economics.md](../explanation/economics.md)

---

## Scorecard Template

Use this in PR descriptions to communicate quality state:

```markdown
## Quality Scorecard

| Surface | Status | Notes |
|---------|--------|-------|
| Correctness | measured | 12 BDD scenarios pass |
| Verification | partial | mutation not run |
| Boundaries | clean | no API/schema changes |
| Maintainability | noted | 3 hotspots identified |
| Explanation | complete | evidence pointers present |
```

### Status Values

| Status | Meaning |
|--------|---------|
| `measured` | Automated verification ran and reported |
| `partial` | Some aspects measured, others not |
| `estimated` | Derived from patterns/precedent, not run |
| `noted` | Human observation recorded, not automated |
| `clean` | No changes in this surface |
| `unknown` | Not measured, no estimate available |

**`unknown` is acceptable when explained.** The point is honest communication, not green checkmarks.

---

## Measured vs Estimated vs Unknown

### Measured

Automated verification ran and produced evidence.

Example: "Mutation score: 87% (evidence: `.runs/feat-auth/build/mutation_report.md`)"

### Estimated

Derived from patterns, precedent, or lightweight inspection. Did not run full verification.

Example: "Estimated regression risk: low (similar pattern to feat-login, which had no issues)"

### Unknown / Not Measured

Explicitly stated gap. Acceptable when:
- The verification is expensive and the change is low-risk
- The tooling doesn't exist yet
- Time constraints apply

Example: "Mutation testing: not run (tooling not configured for this language)"

**The anti-pattern:** Leaving a surface blank or saying "N/A" without explanation. That's hiding uncertainty, not documenting it.

---

## What This Is NOT

### Not "CI green = quality"

CI passing is necessary but not sufficient. CI tells you the build worked. It doesn't tell you:
- Whether tests are meaningful (mutation score)
- Whether boundaries are respected (API drift)
- Whether the change is reviewable (hotspots)

### Not a gate that blocks

The scorecard is a **communication tool**, not a gate. A PR with "verification: unknown" can still ship if the reviewer decides the risk is acceptable.

Gates live in Flow 5. The scorecard lives in the PR description.

### Not exhaustive

The goal is 80% coverage of what matters, not 100% coverage of everything. Five surfaces is enough to communicate quality state. More surfaces would reduce readability.

### Not a replacement for judgment

The scorecard helps reviewers focus. It doesn't replace their judgment about whether to approve.

---

## Integration with PR Brief

The scorecard is a **component** of the PR Brief, not a replacement for it.

PR Brief structure (see [pr-review-interface.md](pr-review-interface.md)):
1. What changed
2. Why
3. Review map (hotspots)
4. Quality events
5. Proof (measured vs not measured)
6. Reproduce

The scorecard can appear in section 4 (Quality events) or as a standalone section. Either works as long as the information is present.

---

## See Also

- [pr-review-interface.md](pr-review-interface.md) — PR Brief template and philosophy
- [evidence-freshness.md](evidence-freshness.md) — When evidence is stale and when to re-verify
- [economics.md](../explanation/economics.md) — Quality:DevLT ratio and token economics
- [contracts.md](contracts.md) — Receipt schemas and evidence formats
- [stable-markers.md](stable-markers.md) — Markers for scorecard sections
- [reviewing-as-audit.md](../explanation/reviewing-as-audit.md) — The skill shift from diff-reading to evidence evaluation
