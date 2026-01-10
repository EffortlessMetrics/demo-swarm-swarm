# Candidates to Artifacts

> LLM output is a candidate. Verified output is an artifact. Never confuse them.

---

## The Transformation

LLM output is not an artifact. It's a **candidate**.

```
Candidate (untrusted) --> Verification --> Artifact (trusted)
```

This distinction is fundamental. Skipping it is how failures happen.

---

## What Is a Candidate?

A candidate is:
- Raw LLM output
- Probabilistically generated
- Possibly wrong
- Possibly hallucinated
- Unverified

Candidates include:
- Generated code
- Proposed tests
- Draft documentation
- Analysis and recommendations

### The Nature of Candidates

Candidates are **guesses**. Educated guesses, often correct guesses, but guesses nonetheless.

The LLM doesn't "know" the code works. It predicts what working code looks like based on patterns. That prediction might be excellent. It might have subtle bugs. It might hallucinate an API that doesn't exist.

Until verification, you can't tell.

---

## What Is an Artifact?

An artifact is:
- Verified output
- Evidence-backed
- Worth preserving
- Trusted for decisions
- Part of the record

Artifacts include:
- Tested code (tests pass)
- Verified claims (evidence exists)
- Reviewed decisions (critics approved)
- Published content (gates cleared)

### The Nature of Artifacts

Artifacts are **proven**. Not perfect—nothing is perfect—but verified against explicit criteria.

An artifact has crossed the verification gap. Evidence exists that it satisfies requirements. Downstream decisions can rely on it because the reliability has been established, not assumed.

---

## The Verification Gap

Between candidate and artifact is the **verification gap**.

```
+-------------+                           +-------------+
|  Candidate  | --- Verification Gap ---> |  Artifact   |
|  (untrusted)|                           |  (trusted)  |
+-------------+                           +-------------+
```

What happens in the gap:
- Tests run and pass
- Critics review and approve
- Evidence is collected
- Claims are verified

### The Gap Is Not Optional

There is no shortcut across the verification gap. You cannot:
- Infer the result from the candidate's quality
- Trust the output because the prompt was good
- Assume correctness because it looks right
- Skip verification because the LLM seems confident

The gap must be crossed. Every time.

---

## Why This Matters

### Candidate Does Not Equal Done

An LLM saying "I implemented the feature" is a candidate claim.
The feature being implemented (with tests, evidence) is an artifact.

Treating candidates as artifacts is how you get:
- "Tests pass" that didn't run
- "Complete" that isn't
- "Secure" that wasn't checked
- "Works" that doesn't

### The Confidence Trap

LLMs sound confident. That's what they're trained to do.

A confident-sounding candidate is still a candidate. Confidence is a property of the output's tone, not a property of its correctness.

```
# Both are candidates. Both sound certain.

Candidate A: "I've implemented robust input validation
              that handles all edge cases."

Candidate B: "The implementation handles typical inputs
              but edge cases may need review."
```

Verification distinguishes correct from incorrect. Tone distinguishes nothing.

---

## Verification Is Required

Every candidate must cross the verification gap:

| Candidate | Verification | Artifact |
|-----------|--------------|----------|
| Generated code | Tests pass | Tested code |
| Proposed fix | Issue resolved | Verified fix |
| Claimed complete | Checklist verified | Confirmed complete |
| Security assertion | Scan run | Security evidence |
| Performance claim | Benchmark executed | Performance data |
| Coverage claim | Coverage measured | Coverage report |

### Verification Is Cheap

This is the key insight. The economics favor verification:

| Step | Cost | What It Is |
|------|------|------------|
| Generate | Cheap | LLM produces candidate |
| Execute | Cheap | Tests run, scanners scan |
| Collect | Cheap | Evidence captured |
| Verify | Cheap | Artifacts created |

Human time is expensive. Machine time is cheap. Run the verification.

---

## The Verification Process

### Step 1: Generate (Cheap)

LLM produces candidate:
- Code, tests, analysis, recommendations
- Fast, cheap, possibly wrong
- No trust yet

This is speculation. Educated speculation, but speculation.

### Step 2: Execute (Cheap)

Run verification:
- Tests execute
- Linters run
- Scanners check
- Critics review

Still machine time. Still cheap. Now we're gathering data.

### Step 3: Evidence (Cheap)

Collect proof:
- Test output captured
- Critic report generated
- Evidence pointers created
- Artifacts linked

The evidence exists independent of claims. It can be audited.

### Step 4: Artifact (Trusted)

If verification passes:
- Candidate becomes artifact
- Evidence backs the claim
- Human can trust the result
- Downstream can depend on it

If verification fails:
- Candidate remains candidate
- Fix and try again
- No pretending it passed

---

## Candidate Failure Modes

### Hallucinated Success

**Candidate says:** "All tests pass"

**Reality:** Tests weren't run, or some failed

**How it happens:** The LLM predicts what a successful outcome looks like. It generates the success message without the success.

**Fix:** Evidence requirement. Show the output. Point to the artifact.

### Confident Wrongness

**Candidate says:** "This is the correct implementation"

**Reality:** Logic error, missing edge case, wrong algorithm

**How it happens:** The LLM pattern-matched to similar code. The pattern was slightly wrong for this context.

**Fix:** Critics review. Tests verify. Multiple perspectives catch what single authors miss.

### Process Confabulation

**Candidate says:** "I followed the spec exactly"

**Reality:** Spec misread, requirements missed, interpretation drifted

**How it happens:** The LLM believes it followed instructions. Its understanding of the instructions was incomplete or wrong.

**Fix:** Traceability. AC-by-AC verification. Requirement mapping.

### Phantom Completion

**Candidate says:** "Implementation complete"

**Reality:** 80% done, edge cases missing, error handling absent

**How it happens:** Pressure to complete drives premature closure. The LLM wants to report success.

**Fix:** Checklist verification. Critics look for gaps. Tests cover edges.

---

## The Artifact Record

Artifacts persist because:
- **They're verified** — worth keeping
- **They're evidence** — needed for audit
- **They're resumable** — enable continuation
- **They're trusted** — downstream depends on them

Candidates don't persist:
- They might be wrong
- They're superseded by artifacts
- They're debugging context, not truth
- They're intermediate, not final

### What Gets Stored

```
.runs/<run-id>/
  signal/
    requirements.md          <-- Artifact (reviewed)
    features/*.feature       <-- Artifacts (reviewed)
  plan/
    adr.md                   <-- Artifact (reviewed)
    api_contracts.yaml       <-- Artifact (reviewed)
  build/
    impl_changes_summary.md  <-- Artifact (tests passed)
    code_critique.md         <-- Artifact (review complete)
    test_execution.md        <-- Artifact (execution captured)
  gate/
    merge_decision.md        <-- Artifact (evidence-backed)
    gate_receipt.json        <-- Artifact (verified)
```

Each artifact has crossed the verification gap. The record is trustworthy.

---

## Implementation in the Pack

The candidate-to-artifact transformation is implemented through:

### Verification Flows

Each flow transforms candidates to artifacts:

| Flow | Candidates In | Artifacts Out |
|------|---------------|---------------|
| Signal | Raw requirements ideas | Reviewed requirements.md |
| Plan | Design proposals | Approved ADR, contracts |
| Build | Generated code/tests | Tested, reviewed code |
| Review | Fix attempts | Verified fixes |
| Gate | Merge recommendation | Evidence-backed decision |

### Evidence Requirements

Claims require pointers. See [Claims and Evidence](claims-and-evidence.md).

```yaml
# Artifact (good)
test_execution:
  command: "npm test"
  exit_code: 0
  summary: "47 passed, 0 failed"
  artifact: ".runs/feat-auth/build/test_execution.md"

# Candidate claim (insufficient)
tests: "all passing"  # No command, no output, no artifact
```

### Artifact Schemas

Receipts, summaries, and decisions have defined structures. These structures require evidence fields that force verification.

### Clear Naming

The location tells you the status:
- Draft files are candidates
- Committed artifacts have passed verification
- Receipt files summarize verified outcomes

---

## The Transformation in Practice

### Example: Code Implementation

**Candidate stage:**
```
code-implementer produces src/auth/login.ts
Claims: "Implements REQ-001 per ADR-005"
Status: CANDIDATE (unverified)
```

**Verification:**
```
test-runner executes test suite
code-critic reviews implementation
Evidence collected in build/test_execution.md
Evidence collected in build/code_critique.md
```

**Artifact stage:**
```
Tests: 12 passed, 0 failed
Critic: No CRITICAL/MAJOR issues
Status: ARTIFACT (verified)
Record: impl_changes_summary.md points to evidence
```

### Example: Security Claim

**Candidate stage:**
```
Agent claims: "No security vulnerabilities"
Status: CANDIDATE (unverified)
```

**Verification:**
```
secrets-tools scans staged changes
Scope: 15 files, all content reviewed
Findings: None detected
```

**Artifact stage:**
```
secrets_scan:
  tool: "secrets-tools"
  scope: "staged changes (15 files)"
  findings: "none"
  artifact: ".runs/feat-auth/gate/secrets_scan.md"
Status: ARTIFACT (verified)
```

---

## Anti-Patterns

### Treating Candidates as Artifacts

```
# WRONG
Agent: "I've completed the implementation"
Orchestrator: "Great, merge it"
```

The claim is a candidate. It needs verification before it becomes a fact.

### Skipping Verification for Speed

```
# WRONG
"We're in a hurry, skip the tests this time"
```

Verification is cheap. Bugs in production are expensive. The math doesn't favor skipping.

### Trusting Confident Language

```
# WRONG
Agent: "I'm absolutely certain this handles all edge cases"
Human: "They sound sure, must be correct"
```

Confidence is tone. Evidence is truth. Look at the evidence.

### Accepting "Should Work"

```
# WRONG
Agent: "This should work with the service"
Status: COMPLETE
```

"Should" is a candidate word. "Does" requires verification.

---

## The Economics

Why go through this? Because the alternative is worse.

### Candidate Economics

- **Cost to generate:** ~seconds
- **Cost if wrong:** hours to days (debugging, fixing, re-deploying)
- **Detection:** Eventually, often in production

### Verification Economics

- **Cost to verify:** ~seconds
- **Cost if catches bug:** minutes (fix before merge)
- **Detection:** Immediately, before damage

### The Math

```
Verification cost: 30 seconds per candidate
Bug cost: 4 hours per escaped bug
Bug rate: ~10% of unverified candidates

Expected cost without verification:
  0.10 * 4 hours = 24 minutes per candidate

Expected cost with verification:
  30 seconds + 0.01 * 4 hours = ~2.4 minutes per candidate

ROI: ~10x
```

The numbers are illustrative, but the direction is clear. Verification pays.

---

## Summary

**Candidates are guesses.** LLM output is probabilistic. It might be right. It might be wrong. You don't know until you check.

**Artifacts are proven.** Verification crosses the gap. Evidence backs the claim. Trust is earned, not assumed.

**The gap is mandatory.** Every candidate must pass through verification. No shortcuts. No exceptions.

**Verification is cheap.** Machine time costs nearly nothing. The economics strongly favor running the checks.

**The mantra:** LLM output is a candidate. Verified output is an artifact. Never confuse them.

---

## See Also

- [claims-and-evidence.md](claims-and-evidence.md) — Evidence requirements and pointers
- [adversarial-loops.md](adversarial-loops.md) — Author/critic verification pattern
- [ai-physics.md](ai-physics.md) — LLM constraints this addresses
- [why-ops-first.md](why-ops-first.md) — Gates at publish boundaries
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
