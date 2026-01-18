# Principle: Truth Flows Downward

> When sources conflict, trust the layer closer to reality.

## The Principle

The system operates under a 5-layer truth hierarchy. When information conflicts, the lower layer (closer to mechanical reality) wins.

```
Tool outputs → Derived facts → Intent → Implementation → Narrative
    (most trusted)                               (least trusted)
```

This is Law 8 of the swarm. It is physics, not preference.

---

## Why This Matters

### Agents Can Hallucinate. Tools Cannot

LLMs generate plausible text. Under completion pressure, they may claim success without evidence. They may "remember" running tests they did not run.

Tools cannot hallucinate. `pytest` returns exit code 1 when tests fail. `git status` shows what is actually staged. Exit codes and stdout are mechanical truth.

### Prevents Process Confabulation

"Process confabulation" is when agents claim steps happened that did not. The hierarchy forces verification: claims must be backed by derived facts, which must be backed by tool outputs.

---

## The 5-Layer Hierarchy

### Layer 1: Tool Outputs (What Actually Happened)

Exit codes, stdout, stderr, structured output from tools.

**Examples:** `pytest` exit code, `eslint` warnings, `git status` output, compiler errors.

This is ground truth. When an agent claims tests passed but `pytest` returned exit code 1, the exit code wins.

### Layer 2: Derived Facts (Mechanical Extraction)

Counts and metrics extracted from tool outputs through deterministic operations.

**Examples:** "15 passed, 3 failed" from pytest, "5 REQ markers" from grep, `build_receipt.json` summaries.

Derived facts are one step removed from ground truth. They depend on accurate extraction.

### Layer 3: Intent (What We Meant to Build)

BDD scenarios, ADRs, contracts. Intent is authoritative for design questions.

**Examples:** `features/*.feature`, `plan/adr.md`, `plan/api_contracts.yaml`.

When implementation diverges from intent, intent wins unless explicitly amended.

### Layer 4: Implementation (What We Built)

The code itself. Checked against higher layers, not trusted in isolation.

### Layer 5: Narrative (Interpretation)

Agent descriptions, status updates, prose. Useful for context, never for pass/fail.

---

## How Conflicts Resolve

### Tool Output vs Narrative

```
Narrative: "All tests pass."
Tool output: pytest exit code 1

Resolution: Tool output wins.
```

**Violation:** "The agent said tests passed, so we proceed."
**Correct:** "The test runner shows exit code 0. We proceed."

### Derived Count vs Narrative

```
Narrative: "I addressed all 3 critical issues."
Derived: grep shows 5 CRITICAL markers

Resolution: Derived wins.
```

**Violation:** "The agent says 3, so 3 it is."
**Correct:** "The count shows 5. Agent may have missed some."

### Intent vs Implementation

```
Intent (ADR): "Tokens expire after 24 hours"
Code: TOKEN_EXPIRY = 3600000 // 1 hour

Resolution: Intent is authoritative. Code is wrong.
```

**Violation:** "The code says 1 hour, so that must be correct."
**Correct:** "The ADR says 24 hours. This is a bug."

### Missing Evidence

```
Narrative: "I ran the linter and it passed."
Evidence: No lint artifact exists

Resolution: UNKNOWN, not PASS.
```

**Violation:** "Agent claims it passed, so PASS."
**Correct:** "No evidence exists. Route to run linter."

---

## The Corollary

**An agent's claim does not override tool output.**

When an agent says one thing and a tool shows another, the tool wins. Always.

This is not distrust of agents. Tools provide mechanical truth; agents provide interpretation. Both are valuable. When they conflict, mechanical truth arbitrates.

---

## Practical Application

| Role              | What To Do                                                      |
| ----------------- | --------------------------------------------------------------- |
| **Agents**        | Run tools, cite outputs. "Not measured" beats false certainty.  |
| **Gates**         | Verify claims against tool outputs. Missing evidence = UNKNOWN. |
| **Orchestrators** | Verify at boundaries. Narrative is context, not truth.          |

### Anti-Patterns

| Anti-Pattern                        | What Goes Wrong                  |
| ----------------------------------- | -------------------------------- |
| Trusting narrative over tool output | Hallucinated success passes gate |
| Inventing evidence                  | Claims without running tools     |
| Ignoring the hierarchy              | Implementation overrides intent  |
| Stale evidence                      | Evidence from previous run used  |

---

## See Also

- [truth-hierarchy.md](../truth-hierarchy.md) --- The 5-layer epistemology in detail
- [evidence-over-trust.md](evidence-over-trust.md) --- Decisions require artifacts
- [the-five-physics.md](../the-five-physics.md) --- Mechanical Truth as physics
