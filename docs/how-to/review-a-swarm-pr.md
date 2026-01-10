# Review a Swarm PR

> How to decide yes/no on a swarm-generated PR in bounded time.

This guide teaches the reviewer decision procedure: how a senior engineer reviews machine-generated PRs efficiently and safely. The model is "code as binary" — you review intent, evidence, and hotspots rather than every line.

---

## The 60-Second Cockpit Scan

Start every review with a quick orientation. Open the PR description and answer:

**What changed?**
- Scope: feature/bug/refactor
- Size: files touched, lines changed
- Surface: API, schema, deps, internal

**What evidence exists?**
- Test results: passed/failed/skipped
- Critiques: code_critique.md, test_critique.md
- Receipts: build_receipt.json, gate_receipt.json

**What's flagged?**
- CRITICAL or blocker items in critiques
- UNVERIFIED status in receipts
- Explicit "not measured" sections

**What's explicitly not measured?**
- Mutation testing: ran or skipped?
- Fuzz testing: ran or skipped?
- Integration tests: ran or skipped?

If the cockpit scan raises no flags and scope is small, you may approve after hotspot check. If anything is unclear, dig deeper.

---

## Hotspot Selection (3-8 Files)

You cannot read everything. Pick 3-8 files for spot-checking based on:

**High-value targets:**
- Files with the most changes (high churn)
- Files with the most complexity (new logic, algorithms)
- Files touching boundaries (API handlers, schema definitions, external deps)
- Files flagged in critiques (code_critique.md, test_critique.md)
- New files (especially new abstractions or modules)
- Files in the "Review map" section of the PR Brief

**Skip unless flagged:**
- Generated files (migrations auto-generated from schema)
- Test files for trivial cases
- Config files with obvious changes
- Documentation updates (unless they're the point)

**Selection heuristic:** If a file touches user input, external systems, or auth, it's a hotspot. If it's internal refactoring with passing tests, probably not.

---

## Must-Read Surfaces

Some surfaces require manual review regardless of verification status. If these files changed, read them:

### Security-sensitive

| Surface | Risk | What to check |
|---------|------|---------------|
| Auth/authz | Access control bypass | Permission checks on all paths |
| Crypto primitives | Weak/broken crypto | Standard algorithms, no custom crypto |
| Secrets handling | Credential exposure | No hardcoded secrets, proper storage |
| Input validation | Injection attacks | Sanitization before use |

### Structural risk

| Surface | Risk | What to check |
|---------|------|---------------|
| New dependencies | Supply chain attack | Package source, version pinning, necessity |
| Schema migrations | Data loss | Reversibility, null handling, indexes |
| Concurrency primitives | Race conditions | Lock ordering, async patterns, deadlock potential |
| Unsafe/raw operations | Memory safety | Bounds checking, lifetime correctness |

### Compliance/policy

| Surface | Risk | What to check |
|---------|------|---------------|
| Privacy-relevant code | Regulatory violation | Data handling, consent, retention |
| Audit logging | Compliance gap | Event capture, tamper resistance |
| Third-party integrations | License violation | License compatibility |

**If a must-read surface has no verification ("not measured") and you can't review it yourself, don't merge.**

---

## Interpreting "Not Measured"

"Not measured" is honest. The question is whether it's acceptable for this change.

### Acceptable "not measured"

- Low-risk area (internal refactoring, docs)
- Explanation provided (mutation testing not configured for this repo)
- Change is reversible (feature flag, easy rollback)
- Already covered by other verification (unit tests cover the surface)

### Needs attention

- Medium-risk area without verification
- No explanation for why it's not measured
- Surface affects users or data

### Blocking

- Security surface with no verification
- Data mutation with no test coverage
- Auth changes without security review
- New external dependency with no audit

**Decision rule:** If "not measured" applies to a must-read surface, either read it yourself or request verification before merging.

---

## The Decision

Three outcomes:

### 1. Merge

**When:** Evidence sufficient, spot-checks pass, risk acceptable.

Signs you can merge:
- Receipts show VERIFIED or UNVERIFIED with clear explanation
- Gate passed or UNVERIFIED only on low-risk surfaces
- Hotspot review revealed no issues
- Must-read surfaces verified or reviewed
- "Not measured" items are acceptable for this change

### 2. Route fix-forward

**When:** Minor issues worth fixing before merge, but not a fundamental problem.

Use this for:
- Small code style issues
- Missing edge case tests
- Documentation gaps
- Minor refactoring suggestions

Route to appropriate agent (code-implementer, test-author, doc-writer) for fix. Don't bounce to earlier flows for small issues.

### 3. Bounce

**When:** Fundamental issues requiring rework.

Signs you should bounce:
- Architecture doesn't match plan (route to Plan)
- Requirements misunderstood (route to Signal)
- Contract violations (route to interface-designer)
- Security issues in must-read surface

Bouncing is rare. Usually, route to a fixer/critic first. Only bounce when the problem is upstream.

---

## What "Code as Binary" Actually Means

Traditional review: read every line, understand every change, form mental model of entire diff.

Swarm review: trust verification, spot-check risk, review intent.

**You're reviewing:**
- Intent: Does the PR Brief explain what and why?
- Evidence: Do receipts and tests verify the claims?
- Hotspots: Do high-risk files look correct?

**You're NOT doing:**
- Line-by-line archaeology of every file
- Mental execution of all code paths
- Reconstruction of the full diff in your head

**The model:**
- Machine cycles buy verification (tests, critiques, receipts)
- Human attention goes to judgment calls (must-read surfaces, risk assessment)
- Diff is for audit, not primary review

**Analogy:** You don't read object code to approve a binary. You check the build receipts, run the tests, and spot-check the source. Same model, different scale.

---

## Common Mistakes

### Trying to read all code

**Problem:** Defeats the bounded-time model. You'll burn out or rubber-stamp.

**Fix:** Trust verification. Spot-check hotspots. If you can't trust the verification, fix the verification.

### Ignoring "not measured" flags

**Problem:** False confidence. "CI green" doesn't mean "fully verified."

**Fix:** Read the "not measured" section. Decide if it's acceptable for this change.

### Treating CI green as sufficient

**Problem:** CI tests what's configured. It doesn't test what's not measured.

**Fix:** Check receipts for station status. What ran? What was skipped?

### Not checking must-read surfaces

**Problem:** Verification doesn't cover all risks. Auth changes can pass all tests and still be wrong.

**Fix:** If a must-read surface changed, read it. No exceptions.

### Approving without understanding scope

**Problem:** You don't know what you're approving.

**Fix:** Spend the 60 seconds on the cockpit scan. If scope is unclear, ask.

### Bouncing when you should route forward

**Problem:** Expensive rework for minor issues.

**Fix:** Minor issues go to fix-forward. Only bounce for fundamental problems.

---

## Checklist

Use this for every swarm PR:

```
[ ] 60-second scan: scope, evidence, flags, not-measured
[ ] Hotspots selected (3-8 files)
[ ] Must-read surfaces checked (if changed)
[ ] "Not measured" acceptable for this change?
[ ] Receipts reviewed (status, blockers, quality_gates)
[ ] Decision: MERGE / FIX-FORWARD / BOUNCE
```

---

## See Also

- [pr-review-interface.md](../reference/pr-review-interface.md) — PR Brief template and philosophy
- [working-with-receipts.md](working-with-receipts.md) — How to read receipts
- [troubleshoot.md](troubleshoot.md) — When things look wrong
