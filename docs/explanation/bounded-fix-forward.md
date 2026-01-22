# Bounded Fix-Forward

> Small deterministic fixes at the gate, not full reruns.

---

## The Problem: Gate Finds Lint Errors

You have completed Build. Tests pass. Critics are satisfied. You reach Gate.

Gate finds three trailing whitespace violations and an unsorted import.

**Traditional approach:** Bounce to Build. Re-run the entire Build flow. Wait for tests. Wait for critics. Wait for cleanup. Come back to Gate.

**Bounded fix-forward:** Run the formatter. Verify it worked. Commit. Continue with Gate.

The difference: 5 minutes vs 45 minutes. The outcome: identical.

---

## What Is Bounded Fix-Forward?

Bounded fix-forward is a **constrained repair lane** within Flow 5 (Gate) that handles mechanical drift without bouncing to earlier flows.

| Term            | Definition                                                                          |
| --------------- | ----------------------------------------------------------------------------------- |
| **Fix-forward** | Repair issues in the current flow rather than bouncing to a previous flow           |
| **Bounded**     | Strictly limited scope: no new features, no logic changes, deterministic fixes only |
| **Mechanical**  | Issues that can be fixed by tools without human judgment                            |

The pattern separates two concerns:

1. **Diagnosis** (gate-fixer): Identify what is wrong and whether it qualifies for fix-forward
2. **Execution** (fix-forward-runner): Apply the prescribed fixes exactly as specified

---

## What Makes a Fix "Bounded"?

A fix qualifies for the bounded fix-forward lane only when it meets all three criteria:

### 1. No Behavioral Change

The fix does not alter program behavior. Tests that passed before will pass after. No logic is modified.

| Bounded                    | Not Bounded                |
| -------------------------- | -------------------------- |
| Whitespace and indentation | Fix a failing test         |
| Import sorting             | Add missing error handling |
| Trailing newline           | Change algorithm           |
| Spelling in comments       | Fix a race condition       |

### 2. Deterministic Execution

The fix can be automated by standard tools with predictable results. Running the same command twice produces identical output.

| Bounded              | Not Bounded                |
| -------------------- | -------------------------- |
| `prettier --write .` | "Make this code cleaner"   |
| `isort --apply src/` | "Reorganize these modules" |
| `cargo fmt`          | "Refactor for performance" |

### 3. No Judgment Required

The fix does not require understanding business requirements, making trade-offs, or choosing between alternatives.

| Bounded                 | Not Bounded                     |
| ----------------------- | ------------------------------- |
| Linter-fixable warnings | "Should this be async?"         |
| Lockfile regeneration   | "Which dependency version?"     |
| Doc typo correction     | "Is this the right API design?" |

---

## Why Bounded Fixes Are Safer Than Full Reruns

### Context Preservation

A full Build rerun resets context. The LLM may make different choices. "The same code" might not be the same.

Bounded fix-forward:

- Runs specific commands
- Touches specific files
- Preserves everything else

### Scope Enforcement

The fix-forward-runner enforces the scope declared by gate-fixer:

```yaml
change_scope:
  allowed_globs: ["src/**/*.py", "tests/**/*.py"]
  deny_globs: [".runs/**", ".github/**"]
```

If the formatter touches a file outside the declared scope, the runner fails. This catches runaway tools.

### Audit Trail

Every step is recorded in `fix_forward_report.md`:

- Exact commands run
- Exit codes and durations
- Files touched
- Scope violations (if any)

The audit trail shows exactly what happened. A full rerun would bury the formatter invocation in a mountain of other activity.

### Non-Convergence Guard

Observation window: if the same issues persist across two reseal passes (no new signal / no net improvement), this lane isn't converging—route out. Gate proceeds to merge-decider with the issues documented. The lane stops; the flow continues.

This prevents:

- Infinite loops where formatters fight
- Tools that generate different output on each run
- Chasing formatting drift forever

---

## How Scope Enforcement Works

The bounded fix-forward pattern uses two specialized agents with distinct responsibilities.

### Gate-Fixer (Diagnosis)

Gate-fixer reads Gate artifacts and classifies issues:

```markdown
## Mechanical Fixes

### MECH-001: Trailing whitespace

- **Evidence:** security_scan.md line 42
- **Files/Paths:** src/auth.py, src/session.py
- **Category:** FORMAT
- **Why mechanical:** Formatter can fix

## Non-Mechanical Findings

### NONMECH-001: Missing input validation

- **Evidence:** contract_compliance.md
- **Likely Target:** Flow 3 (Build)
- **Why not mechanical:** Requires logic change
```

Gate-fixer then emits a fix-forward plan:

```yaml
fix_forward_eligible: true
scope: [FORMAT, LINT_AUTOFIX]
rationale: "All findings are formatter-fixable"

apply_steps:
  - id: FF-APPLY-001
    purpose: "Apply Python formatter"
    command: "ruff format src/ tests/"

verify_steps:
  - id: FF-VERIFY-001
    purpose: "Verify clean"
    command: "ruff check src/ tests/"

change_scope:
  allowed_globs: ["src/**/*.py", "tests/**/*.py"]
  deny_globs: [".runs/**", ".github/**"]
```

### Fix-Forward-Runner (Execution)

Fix-forward-runner executes the plan exactly as written:

1. **Parse the plan** from `gate_fix_summary.md`
2. **Baseline snapshot** (record HEAD, check for uncommitted changes)
3. **Run apply_steps** (execute each command, capture output)
4. **Enforce scope** (verify only declared files were touched)
5. **Run verify_steps** (confirm the fixes worked)
6. **Write report** (document everything that happened)

The runner never diagnoses. It never invents commands. It executes what gate-fixer prescribed.

### The Separation Matters

| Concern            | Gate-Fixer | Fix-Forward-Runner |
| ------------------ | ---------- | ------------------ |
| Read evidence      | Yes        | No                 |
| Classify issues    | Yes        | No                 |
| Generate commands  | Yes        | No                 |
| Execute commands   | No         | Yes                |
| Enforce scope      | No         | Yes                |
| Write audit report | No         | Yes                |

This separation ensures:

- One agent decides what to fix
- Another agent executes and verifies
- Neither can unilaterally expand scope

---

## When to Use Fix-Forward vs Bounce

### Use Fix-Forward When

1. **Issues are mechanical** (FORMAT, LINT_AUTOFIX, IMPORT_ORDER, DOCS_TYPO, LOCKFILE_REGEN)
2. **Fixes are deterministic** (standard tools with predictable output)
3. **No behavioral changes** (tests will still pass)
4. **Scope is clear** (known files, known commands)

### Bounce to Build (Flow 3) When

1. **Logic errors** even if they cause build failure
2. **Missing implementation** (function not implemented)
3. **Test failures** (tests fail, not just formatting)
4. **Contract violations** (API doesn't match spec)
5. **Coverage below threshold** (missing test coverage)

### Bounce to Plan (Flow 2) When

1. **Design flaws** (architecture doesn't support requirements)
2. **Missing requirements** (spec is incomplete)
3. **Contract conflicts** (API design is wrong)

### Bounce to Signal (Flow 1) When

1. **Requirements conflict** (cannot satisfy all requirements)
2. **Scope unclear** (don't know what to build)

---

## Examples

### Bounded: Import Sorting

**Finding:** `isort` reports unsorted imports in 5 files.

**Plan:**

```yaml
fix_forward_eligible: true
apply_steps:
  - id: FF-APPLY-001
    command: "isort src/ tests/"
verify_steps:
  - id: FF-VERIFY-001
    command: "isort --check-only src/ tests/"
change_scope:
  allowed_globs: ["src/**/*.py", "tests/**/*.py"]
```

**Why bounded:** Deterministic tool, no behavioral change, clear scope.

### Bounded: Trailing Whitespace

**Finding:** Pre-commit hook would fail on trailing whitespace.

**Plan:**

```yaml
fix_forward_eligible: true
apply_steps:
  - id: FF-APPLY-001
    command: "pre-commit run trailing-whitespace --all-files"
```

**Why bounded:** Mechanical fix, no logic change.

### Not Bounded: Failing Test

**Finding:** `test_auth_flow` fails with assertion error.

**Decision:** BOUNCE to Flow 3 (Build).

**Why not bounded:** Fixing the test requires understanding what the test is checking and why it fails. This is not mechanical.

### Not Bounded: Missing Error Handling

**Finding:** Security scan reports unhandled exception in `/api/users`.

**Decision:** BOUNCE to Flow 3 (Build).

**Why not bounded:** Adding error handling requires judgment about what errors to catch and how to respond. The fix changes behavior.

### Not Bounded: Wrong API Design

**Finding:** Contract enforcer reports response shape does not match `api_contracts.yaml`.

**Decision:** Investigate first. If the code is wrong, BOUNCE to Build. If the contract is wrong, BOUNCE to Plan.

**Why not bounded:** Either the code or the contract needs to change. Both require understanding intent.

---

## The Reseal Loop

When fix-forward makes changes, Gate must verify the new state:

```
gate-fixer diagnoses -> fix_forward_eligible: true
    |
fix-forward-runner executes plan
    |
Changes made? --No--> Continue with Gate
    |
    Yes
    |
repo-operator commits changes
    |
receipt-checker verifies receipts
    |
gate-fixer rerun (once)
    |
Continue with Gate (merge-decider)
```

The reseal ensures:

- Build receipt is updated to reflect the new state
- Gate artifacts reflect reality
- The merge decision is based on current code

---

## See Also

- [Local Resolution](principles/local-resolution.md) - Exhaust local options before escalating
- [The Microloop](principles/microloop.md) - Writer/critic iteration pattern
- [Flow Composition](flow-composition.md) - How flows connect
- [Why Seven Flows](why-seven-flows.md) - The flow structure and boundaries
