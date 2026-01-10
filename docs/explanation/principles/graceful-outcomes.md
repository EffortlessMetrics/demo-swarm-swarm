# Principle: Graceful Outcomes

> An honest report of failure is a successful agent run.

## The Principle

Agents succeed by reporting back clearly, even when they can't complete the work. Partial progress with honest reporting is valuable. Silent failure is the only real failure.

## Why This Matters

### The Orchestrator Needs Information

The orchestrator (Claude) can only route effectively if it knows what happened. An agent that hits a wall and reports clearly:

> "Couldn't complete — missing the API spec. Need that before I can proceed."

...gives the orchestrator actionable information. It can now get the spec or adjust the plan.

An agent that fails silently leaves the orchestrator blind.

### Partial Progress Is Valuable

Real work rarely completes perfectly on the first try. An agent that reports:

> "Implemented 2 of 3 features. Blocked on database schema question. Here's what I built so far."

...has done valuable work. The orchestrator can address the blocker and continue.

### Failure Is Information

Knowing what doesn't work is valuable:

> "Tried three approaches to fix the timeout issue. None worked because the underlying library doesn't support custom timeouts. Recommend switching libraries or accepting the limitation."

This honest report of "failure" is extremely useful.

### Blame-Free Environment

When agents know partial reports are acceptable, they report honestly. When they fear punishment for incomplete work, they either stay silent or overstate progress.

## How It Works

### Successful Outcomes

All of these are successful agent runs:

- ✅ "Done. Here's what I built."
- ✅ "Found 3 issues. Here's what's wrong and how to fix."
- ✅ "Couldn't complete — missing the API spec. Need that first."
- ✅ "Partial progress. 2 of 3 done. Blocked on schema question."
- ✅ "This isn't fixable at my level. Needs architecture decision."
- ✅ "Tried and failed. Here's what I learned."

### Failed Outcome

Only one thing is truly a failure:

- ❌ Silent. No report. Orchestrator doesn't know what happened.

### Prompt Pattern

Every agent prompt should include graceful failure handling:

```markdown
## If You're Stuck

If you can't complete the work, that's okay. Report:
- What you tried
- What blocked you
- What you recommend

Honest partial progress beats silent failure.
The orchestrator needs to know what happened to route appropriately.
```

### Handoff on Partial Completion

```markdown
## Handoff

**What I did:** Implemented the auth endpoint and login flow.

**What I couldn't do:** Logout flow blocked — need to know if we invalidate server-side or just clear client token.

**Recommendation:** Get answer on logout approach, then I can finish in one more pass.
```

## Anti-Patterns

### ❌ Binary Success/Failure
Treating anything less than 100% as failure discourages honest reporting.

### ❌ Silent Failure
Agent hits a problem and returns nothing, or returns incomplete output without explanation.

### ❌ Overstating Progress
Agent says "done" when blocked, hoping the problem resolves itself.

### ❌ Punishing Partial Results
Prompts that imply partial completion is unacceptable.

## Examples

### Good: Honest Partial Report
```markdown
## Handoff

**What I did:** Reviewed 8 of 12 API endpoints against the contract.

**What I found:** 2 issues in the endpoints I reviewed (response format mismatches).

**What I couldn't review:** The payment endpoints require auth context I don't have access to.

**Recommendation:** Fix the 2 issues found, then either give me auth context or have someone with access review payment endpoints.
```

### Good: Blocked but Informative
```markdown
## Handoff

**What I tried:** Attempted to run the test suite.

**What blocked me:** Database container isn't running. Tests require postgres on localhost:5432.

**Recommendation:** Start the database container, then re-run me.
```

### Bad: Silent Incomplete
Agent returns test results for 3 of 10 test files with no explanation of why 7 are missing.

### Bad: Overstatement
Agent says "tests pass" when actually several tests were skipped due to missing dependencies.

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [How to Design Agents](../../how-to/design-agents.md)
