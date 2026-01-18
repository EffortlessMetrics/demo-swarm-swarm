# Truth Hierarchy

> What the system believes when sources conflict.

---

## The Core Rule

> **When sources conflict, trust flows downward: tool outputs > derived facts > intent > implementation > narrative.**

This is the epistemology of the pack. Every agent, flow, and gate operates under this hierarchy. Violating it creates "process confabulation"—agents claiming success without evidence.

---

## The 5 Layers

### Layer 1: Tool Outputs (What Actually Happened)

**What it is:** The actual results returned by tools—exit codes, stdout, structured output.

| Source       | Examples                            |
| ------------ | ----------------------------------- |
| Test runner  | Exit code, pass/fail counts, output |
| Linter       | Warnings, errors, exit status       |
| Git commands | `git status`, `git diff` output     |
| Build tools  | Compilation results, error messages |
| CI checks    | GitHub Actions status               |

**This layer is what actually happened.** When an agent claims tests passed but the test runner returned failures, the test runner's output wins.

**Example:**

```
# Agent claims: "All tests pass"
# Tool output:
pytest tests/ --tb=short
  FAILED tests/test_auth.py::test_login_invalid_password
  Exit code: 1

# Tool output shows a failure. Agent claim is invalidated.
```

### Layer 2: Derived Facts (Mechanical Extraction)

**What it is:** Counts and metrics extracted from tool outputs through deterministic operations.

| Source   | Examples                                      |
| -------- | --------------------------------------------- |
| Counts   | Number of failures, number of warnings        |
| Parses   | JSON fields extracted from output             |
| Receipts | `build_receipt.json` summarizing tool outputs |
| Markers  | `REQ-001` counts extracted by regex           |

**Why derived, not direct?** These depend on:

1. The tool having run correctly (layer 1)
2. The extraction being correct (potential error)

**Example:**

```
# Receipt shows:
{ "test_count": 15, "failures": 0, "exit_code": 0 }

# This is derived from tool output.
# Verifiable: run the tool again, check the numbers match.
```

### Layer 3: Intent (What We Meant to Build)

**What it is:** The specifications that define success—BDD scenarios, ADRs, contracts.

| Source        | Examples                  |
| ------------- | ------------------------- |
| BDD scenarios | `features/*.feature`      |
| ADRs          | `plan/adr.md`             |
| API contracts | `plan/api_contracts.yaml` |
| Requirements  | `signal/requirements.md`  |

**This layer is authoritative for design.** When implementation diverges from intent, we ask: was the intent wrong, or is the implementation wrong?

**Example:**

```gherkin
# Intent (BDD scenario):
Scenario: User logs in with valid credentials
  Then the response status is 200

# If implementation returns 201, check the ADR.
# ADR says "200 for successful auth"
# Implementation is wrong, not the intent.
```

### Layer 4: Implementation (What We Built)

**What it is:** The code itself.

| Source      | Examples             |
| ----------- | -------------------- |
| Source code | `src/**/*.ts`        |
| Test code   | `tests/**/*.test.ts` |
| Config      | `package.json`       |

**Treated like compiled output.** We inspect when needed, but we don't pretend line-by-line review scales. Implementation is checked _against_ higher layers, not trusted in isolation.

### Layer 5: Narrative (Interpretation)

**What it is:** Agent descriptions, status updates, prose summaries.

| Source          | Examples                       |
| --------------- | ------------------------------ |
| Agent responses | "I implemented the login flow" |
| Status updates  | "Tests are passing"            |
| Working notes   | Prose in summaries             |

**Useful for understanding, not for truth.** Narrative helps humans understand what happened, but it's never the source of truth for pass/fail decisions.

---

## The Existence Rule (No Output = Unknown)

Tool outputs outrank narratives **when they exist and are attributable**.

If an agent claims "I ran X" but there is:

- no captured output,
- no report artifact,
- no CI check link,
- and no reproducible command/result in the run surface,

…then the correct state is **UNKNOWN / UNVERIFIED**, not PASS.

| Situation                                 | Correct Interpretation |
| ----------------------------------------- | ---------------------- |
| Tool ran, output captured, exit code 0    | PASS                   |
| Tool ran, output captured, exit code 1    | FAIL                   |
| Agent claims tool ran, no output exists   | UNKNOWN                |
| Receipt cites artifact that doesn't exist | UNKNOWN                |

**Unknown is fine. Unknown is honest. Unknown is routable.**

Route to run the tool now and capture output. Update the receipt to cite the actual artifact.

---

## The Review Posture

| Layer              | Human Posture                                      |
| ------------------ | -------------------------------------------------- |
| **Tool outputs**   | Enforced mechanically. Tools verify.               |
| **Derived**        | Review receipts. Spot-check counts.                |
| **Intent**         | Primary review target. Do requirements make sense? |
| **Implementation** | Spot-check guided by evidence.                     |
| **Narrative**      | Context, not evidence.                             |

**The formula:**

- Layers 1-2 are verified mechanically
- Layer 3 is where humans focus
- Layer 4 is spot-checked based on evidence
- Layer 5 is disposable after the run

---

## Why This Matters

### Prevents Process Confabulation

**Problem:** Agents under completion pressure claim success without evidence. They "remember" tests passing when they didn't run them.

**Solution:** Force verification through the hierarchy. An agent claiming success must be backed by derived facts (receipt shows pass) which must be backed by tool outputs (exit code was 0).

### Creates Predictable Resolution

**Problem:** When information conflicts, agents can get stuck or make arbitrary choices.

**Solution:** A deterministic path. When in doubt, check the tool output. `git status` output wins over an agent's memory of what was staged.

### Keeps Receipts Honest

**Problem:** Receipts could become narrative ("I think we passed") instead of evidence ("exit code: 0, failures: 0").

**Solution:** Receipts summarize tool outputs (derived facts), not opinions.

---

## Conflict Resolution Examples

### Tool Output vs Narrative

```
Narrative: "All tests pass. Ready for review."
Tool output: pytest returned exit code 1, 1 failure

Resolution: Tool output wins. There is a failure.
```

### Derived Count vs Narrative Estimate

```
Narrative: "I addressed all 3 critical issues."
Derived: grep -c "CRITICAL" shows 5

Resolution: Derived wins. There are 5, not 3.
```

### Intent vs Implementation

```
Intent (ADR): "Session tokens expire after 24 hours"
Implementation: TOKEN_EXPIRY = 60 * 60 * 1000 // 1 hour

Resolution: Intent is authoritative. Implementation is wrong.
```

### Stale Evidence

```
Receipt: { "evidence_sha": "abc123", "test_status": "PASS" }
Current HEAD: def456

Resolution: Receipt is stale. Re-verify against current HEAD.
```

### Missing Evidence (Most Common Failure Mode)

```
Narrative: "I ran the linter and it passed."
Evidence: no lint artifact exists.
Tool output: none captured.

Resolution:
- Treat as UNKNOWN / UNVERIFIED (do not infer PASS).
- Route to run the linter now and capture output.
- Update the receipt to cite the actual artifact/output.
```

This is the most common failure mode in AI-native systems: the agent claims a tool ran, but there's no artifact and no captured output. The correct state is UNKNOWN, not PASS.

---

## Implementation Implications

### For Agents

1. **Run tools, report outputs.** Don't estimate—check.
2. **Cite evidence.** If you say tests pass, point to the output.
3. **"Not measured" is acceptable.** Better than false certainty.

### For Cleanup Agents

1. **Verify claims against tool outputs.** Read the actual results.
2. **Detect drift.** If status says "done" but tests show failures, that's a mismatch.
3. **Report staleness.** If evidence_sha != HEAD, mark it.

### For Orchestrators

1. **Route on understanding.** Read handoffs, make decisions.
2. **Verify when it matters.** For gates, confirm tool outputs.
3. **Don't trust narrative alone.** "Done" should be backed by receipts.

---

## See Also

- [trust-model.md](../reference/trust-model.md) — Evidence hierarchy and verification boundaries
- [the-physics.md](the-physics.md) — Mechanical Truth as a physics
- [claims-and-evidence.md](claims-and-evidence.md) — Evidence discipline
