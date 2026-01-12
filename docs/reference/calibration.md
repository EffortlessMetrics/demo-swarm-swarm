# Calibration

> Preventing trust decay and Goodhart gaming through active verification.

---

## Purpose

[Calibration Signals](calibration-signals.md) tells you how to track whether the system is healthy.

This document tells you how to **actively prevent** the system from becoming unhealthy:
- Detecting trust decay before it causes problems
- Preventing single-metric gaming (Goodhart's law)
- Running calibration exercises to verify gates work
- Maintaining the panel's integrity over time

This is active intervention, not passive observation.

---

## The Trust Decay Problem

As the swarm becomes reliable, humans start rubber-stamping. Trust decays silently.

### The Pattern

```
1. Swarm produces 50 good PRs
2. Humans learn to trust the green checkmarks
3. Humans stop actually reviewing
4. Quality gates become theater
5. Real problems slip through
```

This is not human failure---it is predictable psychology. Systems that appear reliable get less scrutiny. Reduced scrutiny means reduced detection. Reduced detection means silent failure accumulation.

### Why This Matters

The swarm's value proposition depends on the gates being real:
- If gates always pass, they provide no information
- If humans always approve, the review provides no value
- If problems only surface in production, the system has failed its purpose

Trust decay turns a verification system into a ceremony system.

---

## Calibration Mechanisms

### 1. Periodic Defect Injection

Inject known defects into a shadow fork. Verify gates catch them.

**What to inject:**

| Defect Type | Example | Expected Detection |
|-------------|---------|-------------------|
| Security issue | Credential in code | secrets-sanitizer |
| Test gap | Code that mutation would catch | test-critic |
| Contract violation | API does not match spec | interface-designer, code-critic |
| Style violation | Naming, structure issues | auto-linter, code-critic |

**What to verify:**

- Correct agent detected it
- Severity was appropriate
- Worklist included it
- Gate would have bounced

**Frequency:** ~1 in 20 runs, or when trust metrics look suspiciously clean (100% pass rate for extended periods is a red flag, not a green flag).

### 2. Explicit "Not Measured"

Force attention by requiring explicit gaps in the [PR Quality Scorecard](pr-quality-scorecard.md).

| Good | Bad |
|------|-----|
| "Mutation: not measured (no budget for this run)" | Silent omission |
| "Security: not measured (no changed attack surface)" | "Security: clean" without evidence |
| "Boundary: unknown (contract checker not configured)" | Blank field |

When reviewers see explicit gaps, they cannot assume everything was checked. This forces conscious acknowledgment of verification boundaries.

**The discipline:** Every surface is either `measured`, `estimated`, or `unknown`. Nothing is silently omitted.

### 3. Red Flag Detection

Patterns that should force slowdown, even when gates pass.

| Red Flag | Response |
|----------|----------|
| Missing verification on security-relevant changes | Route to security review |
| "Not measured" on risky surfaces (auth, payments, PII) | Require explicit human acknowledgment |
| UNVERIFIED status without explanation | Block until explained |
| Contract violations in any receipt | Route to interface-designer |
| Unaddressed CRITICAL critiques | Route to fixer |
| 100% pass rate for 20+ runs | Trigger calibration exercise |

Red flags do not block automatically---they force human attention. The goal is to interrupt the rubber-stamping pattern.

### 4. Trust Signal Tracking

Track over time to detect decay.

| Signal | What It Shows | Decay Indicator |
|--------|---------------|-----------------|
| Gate catch rate | Defects found by gates / total defects | Declining |
| Calibration pass rate | Injected defects caught / total injections | Below 90% |
| False positive rate | Bounces that were overridden | Rising |
| Time to detection | When in flow defects are caught | Moving right (later) |
| Review time | Minutes to approve | Declining suspiciously |

Regression in any metric triggers investigation. Stable metrics are not proof of health---they require calibration exercises to verify.

---

## The Panel as Goodhart Defense

Goodhart's law: "When a measure becomes a target, it ceases to be a good measure."

Single metrics get gamed---not always intentionally. Optimization pressure finds the path of least resistance.

### The Gaming Vectors

| Metric | Gaming Mode | Symptom |
|--------|-------------|---------|
| Coverage | Hollow tests (run but do not assert) | High coverage, mutation survivors |
| Mutation score | Over-specific assertions | Brittle tests, low maintainability |
| Green CI | Untested surfaces | Passes CI, fails in production |
| Clean lint | Compliant but complex | No lint errors, unreadable code |
| Fast review | Rubber-stamping | Low DevLT, high defect escape rate |

### The Defense: Multi-Sensor Panel

The [PR Quality Scorecard](pr-quality-scorecard.md) uses multiple sensors that fail in different ways:

| If You Game... | This Catches It |
|----------------|-----------------|
| Coverage (hollow tests) | Mutation testing |
| Mutation (over-specific assertions) | Maintainability sensors |
| Green CI (untested surfaces) | Manual hotspot review |
| Clean lint (compliant but complex) | Complexity deltas |
| Fast review (rubber-stamping) | Calibration exercises |

No single metric is the target. The panel collectively approximates "good engineering."

**The principle:** If all your metrics come from the same source or can be gamed by the same behavior, your panel is not a panel---it is a single metric in disguise.

---

## Wisdom Loop Guardrails

[The Flywheel](../explanation/the-flywheel.md) describes how Flow 7 (Wisdom) feeds learnings back to improve future runs. But the flywheel can overfit if fed bad signals.

### Wisdom Failure Modes

| Problem | Mechanism | Guardrail |
|---------|-----------|-----------|
| Flaky test causes patch | learning-synthesizer sees failure, recommends change | False failure detection before patching |
| One-off issue becomes permanent rule | Single data point extrapolated | Require 3+ occurrences before pattern |
| Template drift from original intent | Incremental patches accumulate | Patch log with evidence requirements |
| Overfitting to recent failures | Recent data weighted too heavily | Staleness rules (patches expire) |

### Patch Discipline

Wisdom-generated patches should be:

| Property | Meaning |
|----------|---------|
| Evidence-backed | Cite the failures they address |
| Scoped | Do not overgeneralize from one case |
| Reversible | Can be rolled back if they cause problems |
| Expiring | Require reaffirmation after N runs or M days |

**The anti-pattern:** A learning from one unusual run becomes a permanent constraint that hurts normal runs.

---

## How to Run Calibration

Practical steps for calibration exercises.

### Step 1: Prepare the Test

Create a branch with known defects. Document what you injected and where.

```markdown
# Calibration Run: 2024-01-15

## Injected Defects

| ID | Type | Location | Expected Detection |
|----|------|----------|-------------------|
| CAL-001 | Credential | src/auth.ts:42 | secrets-sanitizer |
| CAL-002 | Missing test | src/user.ts:88 | test-critic |
| CAL-003 | Contract mismatch | api/routes.ts:15 | code-critic |
```

### Step 2: Run the Flow

Execute a normal flow against the calibration branch. Do not tell agents this is a calibration run.

```bash
/flow-1-signal "Calibration run"
/flow-2-plan
/flow-3-build
/flow-4-review
/flow-5-gate
```

### Step 3: Evaluate Results

Compare actual detections against expected detections.

```markdown
## Results

| ID | Expected | Actual | Status |
|----|----------|--------|--------|
| CAL-001 | secrets-sanitizer | secrets-sanitizer | PASS |
| CAL-002 | test-critic | not detected | FAIL |
| CAL-003 | code-critic | code-critic | PASS |

## Analysis

- CAL-002 missed: test-critic prompt may need update for this pattern
- Overall catch rate: 2/3 (67%)
```

### Step 4: Act on Results

| Outcome | Action |
|---------|--------|
| All defects caught | System is calibrated; schedule next check |
| Some defects missed | Update prompts, retest |
| Many defects missed | Full prompt review; possible structural issue |

### Step 5: Document

Add results to the calibration log.

```
.runs/_calibration/
  2024-01-15.md
  2024-02-01.md
  ...
```

### Frequency

| Situation | Recommended Frequency |
|-----------|----------------------|
| Active development | 1 in 20 runs |
| Suspiciously clean metrics | Immediate |
| After prompt updates | Within 3 runs |
| After pack updates | Within 1 run |
| Routine maintenance | Monthly |

---

## Calibration Checklist

Quick reference for calibration activities.

### Before Each Run

- [ ] Check that "not measured" fields are explicit
- [ ] Verify no silent omissions in scorecard

### After Suspicious Metrics

- [ ] 100% gate pass rate for 20+ runs: trigger calibration
- [ ] Zero critic findings for 10+ runs: trigger calibration
- [ ] Review time dropped significantly: investigate

### Monthly

- [ ] Run calibration exercise with 3+ defect types
- [ ] Review trust signal trends
- [ ] Check for gaming patterns in metrics
- [ ] Verify Wisdom patches are still relevant

### After Pack Updates

- [ ] Run calibration within 1 run
- [ ] Verify no regressions in detection
- [ ] Check for new false positives

---

## See Also

- [Calibration Signals](calibration-signals.md) --- Passive health tracking
- [PR Quality Scorecard](pr-quality-scorecard.md) --- The multi-surface quality panel
- [Trust Model](trust-model.md) --- Evidence hierarchy and verification
- [The Flywheel](../explanation/the-flywheel.md) --- Learning loop and Wisdom
- [Economics](../explanation/economics.md) --- Why spending on verification pays off
