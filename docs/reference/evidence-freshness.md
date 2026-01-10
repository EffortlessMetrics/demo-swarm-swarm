# Evidence Freshness

> When is evidence stale? When to trust it? When to re-verify?

---

## Core Concept

Evidence has a timestamp. Repo state is "now." These can diverge.

- **Receipts** are historical summaries of a moment
- **Current repo state** is live (`git rev-parse HEAD`)
- **CI signals** are tied to a specific SHA
- **Critiques** were written against a specific version of code

The question isn't "is it fresh?" but **"is it fresh enough for this decision?"**

---

## Evidence Types and Their Lifespans

| Evidence Type | Tied To | Stale When |
|---------------|---------|------------|
| Test results | SHA | Code changed since run |
| Critiques | SHA + file content | Files critiqued were modified |
| Receipts | Flow completion | Upstream artifacts changed |
| Coverage reports | SHA | New code added |
| Mutation results | SHA + test content | Tests or code changed |
| API contracts | Schema version | Schema files modified |

**Key insight:** Evidence freshness is determined by what it depends on, not by wall-clock time.

---

## Staleness Is Normal

Staleness is information, not failure.

- Evidence from 10 minutes ago on unchanged files = **fresh**
- Evidence from yesterday on modified files = **stale**
- Evidence from a prior flow that completed = **historical record**, not current state

A stale receipt tells you "this was true then; verify if needed now."

**Why this matters:** Receipts are designed to be logs, not gatekeepers (see [trust-model.md](trust-model.md#receipts-as-logs-not-gatekeepers)). When `receipt.evidence_sha != git HEAD`, that's normal. The receipt remains valid as historical evidence; it just doesn't prove current state.

---

## When to Re-verify vs Accept

### Accept existing evidence when:

- SHA matches current HEAD
- Files in scope haven't changed
- Upstream artifacts (requirements, contracts) unchanged
- Risk level is low

### Re-verify when:

- Code changed since evidence was generated
- Upstream requirements or contracts changed
- Evidence is for a high-risk surface (auth, crypto, schema)
- Evidence was marked "estimated" or "partial"

### Never blindly trust:

- Evidence from a different branch
- Evidence older than the last merge to main
- Evidence for files that no longer exist

---

## Freshness in the Scorecard

When populating the PR Quality Scorecard (see [pr-quality-scorecard.md](pr-quality-scorecard.md)):

- Include evidence SHA or timestamp
- Mark surfaces appropriately:
  - `measured` — evidence is fresh (SHA matches HEAD)
  - `measured (stale)` — evidence exists but SHA drifted
  - `needs re-run` — evidence too old or scope changed significantly

**"Stale but acknowledged"** is acceptable for low-risk surfaces. The scorecard communicates what you know, not what you wish were true.

### Scorecard Example

```markdown
## Quality Scorecard

| Surface | Status | Notes |
|---------|--------|-------|
| Correctness | measured | 12 BDD scenarios pass (SHA: abc123) |
| Verification | measured (stale) | mutation run at abc123, HEAD is def456 |
| Boundaries | clean | no API/schema changes |
| Maintainability | noted | 3 hotspots identified |
| Explanation | complete | evidence pointers present |
```

---

## Avoiding Re-verification Loops

The failure mode: "SHA changed -> re-run everything -> SHA changed again -> infinite loop"

### Prevention:

1. **Re-verify only the affected scope** — If 3 files changed, re-run verification for those files, not the entire codebase.

2. **Accept that some evidence will be slightly stale at merge time** — Between the last verification run and the actual merge, small changes may occur. This is normal.

3. **Gate decisions are made on "good enough" evidence, not perfect freshness** — A gate agent asking "did tests pass?" uses the most recent test run. It doesn't demand tests re-run for every formatting change.

4. **If in doubt, note staleness and let reviewer decide** — Include staleness in your handoff. "Tests passed at abc123, HEAD is now def456 (formatting-only changes since)."

---

## Freshness Markers

Agents should include freshness context when reporting evidence.

### Fresh evidence:

```
Evidence SHA: abc123
Current HEAD: abc123
Status: FRESH
```

### Stale evidence:

```
Evidence SHA: abc123
Current HEAD: def456
Status: STALE (3 files changed since evidence generated)
Changed files: src/auth/login.py, src/auth/session.py, tests/test_auth.py
Recommendation: Re-run test suite for auth module
```

### Unknown freshness:

```
Evidence SHA: unknown (artifact missing timestamp)
Current HEAD: def456
Status: UNKNOWN
Recommendation: Re-run verification or treat as unverified
```

---

## Historical vs Active Evidence

| Category | Definition | Behavior |
|----------|------------|----------|
| **Historical** | Receipts from completed flows | Document what happened. Never update them retroactively. |
| **Active** | Current verification state | Updated by re-running checks against HEAD. |

**Example:** A receipt saying "tests passed at SHA abc123" remains true forever. It documents a historical fact. Whether tests pass NOW is a different question answered by running tests against current HEAD.

**Why this distinction matters:**

- Historical evidence is immutable audit trail
- Active evidence is what you base current decisions on
- Comparing historical to active tells you what changed

---

## Staleness Detection in Practice

### For cleanup agents writing receipts:

Always include `evidence_sha` and `generated_at`:

```json
{
  "evidence_sha": "abc123def456...",
  "generated_at": "2025-12-22T10:45:00Z"
}
```

### For gate agents making decisions:

1. Read the receipt's `evidence_sha`
2. Compare to current `git rev-parse HEAD`
3. If different, check what changed: `git diff <evidence_sha>..HEAD --name-only`
4. Decide: re-verify, accept with note, or proceed

### For reviewers reading PR Briefs:

- Look for evidence SHAs in the Quality Scorecard
- Compare to the PR's HEAD SHA
- If evidence is stale, check if changes since evidence are relevant

---

## Common Freshness Scenarios

### Scenario 1: Formatting-only changes after test run

```
Evidence SHA: abc123 (tests passed)
Current HEAD: def456
Changes: whitespace/formatting only
Decision: Accept. Formatting doesn't affect test outcomes.
```

### Scenario 2: New code added after coverage run

```
Evidence SHA: abc123 (80% coverage)
Current HEAD: def456
Changes: 50 new lines of code added
Decision: Stale. Coverage number no longer meaningful. Re-run or mark "needs re-run."
```

### Scenario 3: Critique written against old code

```
Evidence SHA: abc123 (critique: "missing error handling")
Current HEAD: def456
Changes: error handling added
Decision: Stale critique. Re-run critic or note "addressed since critique."
```

### Scenario 4: Receipt from different branch

```
Evidence SHA: abc123 (from feature-x branch)
Current HEAD: def456 (on feature-y branch)
Decision: Invalid. Evidence from a different branch cannot verify current branch.
```

---

## Integration with Trust Model

Evidence freshness is a dimension of the trust hierarchy (see [trust-model.md](trust-model.md)):

1. **Live repo state + executed evidence** (primary) — Always fresh by definition
2. **Receipts** (cached evidence) — Fresh if SHA matches, stale otherwise
3. **Narrative summaries** — Freshness irrelevant (never use for gating)

The drift rule from trust-model.md applies:

> If `receipt.evidence_sha != git HEAD`, treat as **stale** — use for investigation, not for gating.

---

## See Also

- [trust-model.md](trust-model.md) — Evidence hierarchy and verification patterns
- [pr-quality-scorecard.md](pr-quality-scorecard.md) — Quality surfaces and status values
- [run-state.md](run-state.md) — Receipt schemas and `evidence_sha` field
- [contracts.md](contracts.md) — Receipt schemas with freshness fields
