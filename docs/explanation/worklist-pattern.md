# Worklist Pattern

> How critics produce drainable queues and fix-forward mechanics.

---

## The Pattern

Critics don't just say "there are problems." They produce a **drainable worklist**—a structured queue of issues that can be systematically addressed.

**The key insight:**

> Critics produce queues. Fixers drain queues. The loop continues until the queue is empty or accepted.

This transforms vague "feedback" into a trackable, prioritizable, verifiable workflow.

---

## Why Worklists Matter

### The Problem with Unstructured Feedback

Unstructured feedback:

```
The code has some issues. The auth logic seems wrong.
There might be a race condition. Also check the error handling.
```

This is hard to:
- **Track** — What's fixed vs pending?
- **Prioritize** — What's critical vs nice-to-have?
- **Verify** — How do we know it's addressed?

The reader must parse, remember, and mentally track each issue. Progress is invisible. Completion is ambiguous.

### The Worklist Solution

Structured worklist:

```markdown
## Worklist

| ID | Issue | Severity | File:Line | Status |
|----|-------|----------|-----------|--------|
| W-001 | Missing null check | HIGH | auth.rs:45 | pending |
| W-002 | Race condition in session | HIGH | session.rs:112 | pending |
| W-003 | Generic error message | LOW | api.rs:89 | pending |
```

This is:
- **Trackable** — Clear status per item
- **Prioritizable** — Severity guides order
- **Verifiable** — Check the exact location

Each item has an identity. Progress is visible. Completion is unambiguous.

---

## The Worklist Lifecycle

### 1. Generation (Critic Phase)

Critics produce worklist items with:
- Specific issue description
- Severity/priority
- Location (file:line)
- How to verify closure

The critic's job is to **find and document**, not to fix.

### 2. Triage (Routing Phase)

The orchestrator/PM:
- Routes items to appropriate fixers
- Groups related items
- Identifies blockers vs nice-to-haves
- Decides which items to address now vs defer

### 3. Drain (Fix Phase)

Fixer agents:
- Take items from the queue
- Apply fixes
- Mark items addressed

Each fix reduces the queue. Progress is mechanical and visible.

### 4. Verification (Re-check Phase)

Critics re-verify:
- Is the issue actually fixed?
- Did the fix introduce new issues?
- Is the worklist drained?

This catches regressions and incomplete fixes.

### 5. Closure (Completion)

When the worklist is empty (or remaining items are accepted):
- Produce a summary
- Move to next phase
- Document what was addressed and what was deferred

---

## Worklist Item Structure

A good worklist item has everything needed to find, understand, fix, and verify the issue.

### Example Item

```markdown
### W-001: Missing null check in auth handler

**Severity:** HIGH
**Location:** src/auth.rs:45-48
**Issue:** User object accessed without null check after database lookup
**Impact:** Potential null pointer exception on invalid session
**Fix guidance:** Add `if user.is_none() { return Err(...) }` before line 46
**Verification:** Run `cargo test auth::test_invalid_session`
**Status:** pending
```

### Required Fields

| Field | Purpose |
|-------|---------|
| **ID** | Unique identifier for tracking (W-001, W-002, etc.) |
| **Severity** | HIGH/MEDIUM/LOW — guides priority |
| **Location** | Exact file and line — enables direct navigation |
| **Issue** | What's wrong — the actual problem |
| **Status** | pending/in_progress/fixed/wontfix — current state |

### Optional Fields

| Field | Purpose |
|-------|---------|
| **Impact** | Why this matters — helps with prioritization |
| **Fix guidance** | How to address — accelerates the fixer |
| **Verification** | How to confirm closure — enables automated checks |

---

## Fix-Forward Mechanics

### The Routing Decision

When an issue is found, route based on type:

| Issue Type | Route To | Example |
|------------|----------|---------|
| Logic/semantic error | code-implementer | Wrong algorithm |
| Missing test coverage | test-author | Untested edge case |
| Formatting/lint | auto-linter (skill) | Style violations |
| Documentation gap | doc-writer | Missing docstring |
| Security issue | code-implementer + critic | Injection vulnerability |

### Mechanical vs Semantic

**Mechanical issues** (deterministic fix):
- Formatting
- Import ordering
- Trailing whitespace
- Simple lint rules

Route to deterministic fixer (skill). Don't waste smart tokens.

**Semantic issues** (judgment required):
- Wrong logic
- Missing edge cases
- Architectural problems

Route to appropriate agent. May require iteration.

### The Fix-Forward Default

Don't block on fixable issues. Route to fix.

```
Issue found → Severity HIGH? → Route to fixer → Continue
                            ↓
             Severity LOW? → Add to worklist → Continue
                            ↓
             Mechanical? → Auto-fix → Continue
```

Only block for:
- Mechanical failure (tooling broken)
- Non-derivable decision (need human input)
- Unsafe publish boundary (secrets detected)

**The bias is toward action.** Most issues can be fixed without human intervention. Route them to the appropriate fixer and continue.

---

## The Drain Loop

```python
while worklist.has_pending_items():
    item = worklist.next_by_priority()
    fixer = route_to_fixer(item)
    result = fixer.fix(item)

    if result.success:
        item.status = "fixed"
    else:
        item.add_note(result.blocker)

    # Re-verify periodically
    if should_recheck():
        critic.verify_fixes()
        # May add new items discovered during fixes
```

### Termination Conditions

Stop draining when:
- All HIGH/MEDIUM items fixed
- Remaining items accepted as known issues
- Iteration budget exhausted (checkpoint, continue later)

### New Items During Drain

Fixes may reveal new issues. Handle by:
- Adding to worklist (same structure)
- Continuing the drain loop
- Tracking "waves" if needed

This is normal. The worklist is a living document, not a static list.

---

## Worklist Anti-Patterns

### Vague Items

```
# Bad
"Auth seems broken"

# Good
"W-001: Missing session validation in auth.rs:45"
```

Vague items can't be tracked, prioritized, or verified.

### No Location

```
# Bad
"There's a null pointer somewhere"

# Good
"W-002: Null pointer at user.rs:112, `user.name` without check"
```

Without location, the fixer wastes time searching.

### No Verification

```
# Bad
"Fix the race condition"

# Good
"W-003: Race in cache.rs:78. Verify with `cargo test --test race_test`"
```

Without verification, closure is ambiguous.

### Status Not Updated

```
# Bad
Items stay "pending" after being fixed

# Good
Fixer marks "fixed" immediately after addressing
```

Stale status makes progress invisible.

---

## Integration with Flows

### Build Flow (Flow 3)

```
code-implementer → code-critic → (worklist) → fixer → (drain) → test-executor
```

Critics produce worklist. Microloop drains it. Build receipt summarizes.

### Review Flow (Flow 4)

```
pr-feedback-harvester → (worklist) → fixer → (drain) → standards-enforcer
```

PR feedback becomes worklist items. Drain loop addresses. Review receipt summarizes.

### Gate Flow (Flow 5)

```
gate-fixer → (quick fixes) → merge-decider
```

Gate fixer produces mechanical fixes. Quick drain. Gate decision.

---

## The Evidence Trail

Worklists provide an audit trail:

| What's Recorded | Purpose |
|-----------------|---------|
| What was found | Original issues |
| What was fixed | Work completed |
| What was accepted | Known issues / tech debt |
| What verification was done | Proof of closure |

This becomes part of the receipt for later review. If someone asks "why wasn't X caught?", the worklist shows exactly what was found, what was fixed, and what was left.

---

## Summary

**The problem:** Unstructured feedback is hard to track, prioritize, and verify.

**The solution:** Critics produce structured worklists with identified items.

**The mechanism:** Route items to fixers, drain the queue, verify closure.

**The principle:** Critics produce queues. Fixers drain queues. The loop continues until the queue is empty or accepted.

**The evidence:** Worklists provide audit trails of what was found, fixed, and accepted.

---

## See Also

- [adversarial-loops.md](adversarial-loops.md) — The author/critic pattern that produces worklists
- [agent-philosophy.md](agent-philosophy.md) — Agent roles and responsibilities
- [architecture.md](architecture.md) — Microloop pattern in pack design
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
