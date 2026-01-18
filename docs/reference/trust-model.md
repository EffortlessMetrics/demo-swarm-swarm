# Trust Model

> Evidence hierarchy, verification boundaries, and when to trust LLM reasoning vs mechanical checks.

---

## Core Principle

### Trust hierarchy: state → evidence → inference → narrative

The repo's current state (HEAD + working tree + executed evidence) is the asset you're building and shipping. Receipts and summaries help you investigate what happened, why, and where to look next—but they are not the primary mechanism for determining outcomes once the repo has moved forward.

---

## Evidence Hierarchy

### 1. Live repo state + executed evidence (primary)

This is the ground truth:

- `git rev-parse HEAD`, `git diff`, `git status`
- Test/lint/mutation outputs generated _now_ (not cached)
- CI check runs for the current SHA
- Actual tool execution results (pytest output, lint reports, mutation scores)

**When to use:** Always validate against current repo state before making routing decisions.

### 2. Receipts (cached evidence of prior state)

Historical breadcrumbs that document:

- What an agent ran earlier
- What it saw at that time
- Links/paths to logs and artifacts
- Quality gate verdicts at a specific commit

**When to use:** For investigation, summary inputs, and understanding what happened. **Not** for pass/fail decisions if stale.

**Staleness detection:**

```json
{
  "evidence_sha": "<commit-sha>",
  "generated_at": "<iso8601>"
}
```

**Drift rule:** If `receipt.evidence_sha != git HEAD`, treat as **stale**—use for investigation, not for gating.

### 3. Narrative summaries (useful for humans, never control input)

Prose explanations written for human readers:

- `self_review.md`
- `impl_changes_summary.md`
- `test_changes_summary.md`

**When to use:** For human review, PR descriptions, and retrospectives. **Never** parse these to determine pass/fail.

---

## Trust Boundaries

### Implicitly trusted (agent reasoning)

The LLM is trusted to:

- Reason about code quality by reading diffs and implementation
- Assess whether tests are adequate for the changes
- Identify edge cases and missing scenarios
- Evaluate design coherence
- Draft requirements and acceptance criteria
- Critique work products (code, tests, design)

**Why:** LLMs are strong at pattern recognition, semantic understanding, and reasoning about quality. They don't need hard ratio policies to know if a change is well-tested.

### Must be verified (mechanical checks)

Mechanical execution is required for:

- **Test execution:** Did the tests actually run? What was the exit code?
- **Lint/format conformance:** Does the code pass configured style checks?
- **Secrets scanning:** Are there secret patterns in the publish surface?
- **Commit integrity:** What's the current SHA? What's staged/unstaged?
- **CI status:** Are checks passing for this exact commit?

**Why:** These are facts about the system state that only execution can determine.

---

## Intelligence-Based Verification

### Critics reason about diffs, not math formulas

**What critics do:**

- Read implementation changes (`git diff`)
- Read test changes (`git diff` for test files)
- Assess whether the test **strategy** aligns with the code's **risk surface**
- Identify missing edge cases, boundary conditions, error paths
- Flag inadequate mocking or brittle test setup
- Recommend additional tests when needed

**What critics do NOT do:**

- Enforce coverage floors (no "must be 80%")
- Enforce test-to-code ratios (no "must add 2 tests per function")
- Count lines of test code vs implementation
- Block on missing trivial tests (getters, simple constructors)

### No hard ratio policies

**Rationale:**

- Coverage percentages don't measure quality (100% coverage ≠ good tests)
- Test-to-code ratios are arbitrary and context-blind
- A single property-based test may be worth 50 example-based tests
- Deleting tests can be the right move (removing brittle/redundant tests)

**Instead:**

- Critics reason: "This change introduces error handling; do the tests verify error paths?"
- Self-reviewers reason: "Are the acceptance criteria covered by the test suite?"
- Test authors reason: "What are the risk-weighted scenarios for this feature?"

### Tie-breaker: `can_further_iteration_help`

When a critic returns `status: UNVERIFIED` with `recommended_action: PROCEED`, use `can_further_iteration_help` as the routing signal:

| Value | Meaning                                                          | Routing                    |
| ----- | ---------------------------------------------------------------- | -------------------------- |
| `yes` | Another iteration could improve the work                         | RERUN the station          |
| `no`  | Iteration won't help; gaps are acceptable or need human judgment | PROCEED despite UNVERIFIED |

**Example scenarios:**

```yaml
# Scenario 1: Missing test, LLM can fix
status: UNVERIFIED
recommended_action: PROCEED
can_further_iteration_help: yes
blockers: ["Error path not tested"]
# → Orchestrator reruns test-author

# Scenario 2: Integration test needed, can't automate
status: UNVERIFIED
recommended_action: PROCEED
can_further_iteration_help: no
blockers: ["Integration test requires external service"]
# → Orchestrator proceeds, captures blocker in receipt
```

---

## When to Trust LLM Reasoning

**Trust LLM reasoning for:**

| Task                          | Why                                               |
| ----------------------------- | ------------------------------------------------- |
| Assessing test adequacy       | Understands risk surface, edge cases, error paths |
| Identifying missing scenarios | Pattern recognition over semantic context         |
| Evaluating design quality     | Coherence, consistency, separation of concerns    |
| Drafting requirements         | Semantic understanding of user intent             |
| Critiquing implementation     | Code smells, maintainability, clarity             |

**Example:**

A test critic reads:

```python
def divide(a, b):
    return a / b
```

The critic reasons: "This has a zero-division edge case. The tests should verify behavior when `b == 0`."

**No formula needed.** The LLM understands the risk surface.

---

## When to Enforce Mechanical Checks

**Enforce mechanical checks for:**

| Task                    | Why                                    |
| ----------------------- | -------------------------------------- |
| Test execution          | Only running tests proves they pass    |
| Lint/format conformance | Tooling has the final say on style     |
| Secrets scanning        | Pattern matching is deterministic      |
| Git state inspection    | Only git knows what's staged/committed |
| CI status polling       | Only CI knows the check run state      |

**Example:**

A cleanup agent needs to know if tests passed. It:

1. Reads `test_execution.md` (produced by test-runner)
2. Checks for `status: PASS` or `exit_code: 0`
3. **Does not** infer from test critique or self-review

---

## Verification Agent Pattern

Verification agents (critics, auditors) produce two surfaces:

1. **Prose handoff** (routing communication):

Natural language summary of findings and recommendations. This is what orchestrators read to make routing decisions. Example:

```markdown
## Handoff

**What I found:** Session timeout uses 30m instead of 15m per spec. Missing test for error path.

**Recommendation:** Route to fixer for the timeout (mechanical fix), then test-author for coverage.
```

1. **Machine Summary** (audit plane):

Structured fields that cleanup agents derive from the prose for receipts:

```yaml
## Machine Summary
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
can_further_iteration_help: yes | no
blockers: []
concerns: []
severity_summary:
  critical: 0
  major: 0
  minor: 0
```

1. **Artifact body** (human audit plane):

Human-readable narrative explaining findings, reasoning, and recommendations.

**Orchestrators route on prose handoffs. Machine Summary is for audit trails, not routing.**

---

## Evidence Over Format

If verification evidence exists but a receipt is malformed or missing fields:

- Treat it as a **pack/tooling defect**, not an engineering failure
- Keep the line moving: ship based on the underlying evidence **if gates pass**
- Record the defect explicitly (e.g., `status: UNVERIFIED`, `blockers: ["receipt_tooling_error: <details>"]`)
- Open a maintenance issue for the pack

If the evidence itself is missing (tests didn't run, CI unknown, etc.), that's not paperwork drift—that's **unverified work**. Route back to run the missing verification.

**Maintainer mantra:**

> Build the asset. Capture the evidence. State the truth. Ship when the evidence is green.

---

## Receipts as Logs, Not Gatekeepers

**What receipts ARE:**

- Durable audit trail of what ran and when
- Summary inputs for reporters and wisdom synthesis
- Historical breadcrumbs for debugging ("what did we see at commit X?")

**What receipts are NOT:**

- Permission slips to proceed
- Substitutes for current state validation
- Re-executable proofs of correctness

**State-first verification rule:**

When validating current work, prefer:

1. Execute checks now (run tests, run lint, query CI)
2. Compare current HEAD to evidence SHA
3. Use receipts for summary/context, not as the verdict

---

## SKIPPED Stubs (Explicit Absence)

When a station doesn't produce expected output, cleanup agents create SKIPPED stubs:

```markdown
# <Artifact Name>

status: SKIPPED
reason: <why it wasn't produced>
evidence_sha: <current HEAD>
generated_at: <iso8601>
```

**Purpose:**

- Makes absence explicit (nothing is silently missing)
- Downstream agents see what happened instead of guessing
- Flow 7 (Wisdom) can analyze skipped stations for process improvements

**VERIFIED requires executed evidence:**

A station being "skipped" means the work is **unverified**, not verified by default. Missing verification artifacts result in `status: UNVERIFIED`.

---

## See also

- [contracts.md](contracts.md) — Control-plane blocks and schemas
- [evidence-freshness.md](evidence-freshness.md) — When evidence is stale and when to re-verify
- [CLAUDE.md](../../CLAUDE.md) — State-first verification principle (Receipts section)
- [stable-markers.md](stable-markers.md) — Mechanical counting patterns
