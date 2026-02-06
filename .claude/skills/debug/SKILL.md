# Debug Skill

Systematic debugging workflow.

## Phase 1: Reproduce
Before debugging, confirm the issue:
```
1. Get exact reproduction steps
2. Run them, capture output
3. Confirm failure matches reported issue
```

If can't reproduce: stop and clarify with user.

## Phase 2: Isolate
Narrow down the problem:
```
1. Identify the failing component/function (use Grep to search for error messages)
2. Find the smallest reproduction case
3. Add logging/instrumentation if needed
4. Trace execution to failure point
```

## Phase 3: Diagnose
Understand root cause:
```
1. Read the code at failure point
2. Check recent changes (git log -p)
3. Verify assumptions about inputs
4. Identify the actual vs expected behavior
```

## Phase 4: Fix
Apply minimal fix:
```
1. Change only what's necessary (use Edit for targeted changes)
2. Don't refactor while fixing
3. Add a test that fails without fix
4. Verify test passes with fix
```

## Phase 5: Verify
Confirm fix is complete:
```
1. Run full test suite
2. Re-run original reproduction
3. Check for regression in related areas
```

## Output Format
```markdown
## Issue
<description>

## Reproduction
<steps and output>

## Root Cause
<what's actually wrong>

## Fix
<what was changed and why>

## Verification
- Reproduction: ✓/✗
- New test: ✓/✗
- Test suite: ✓/✗
```

## Rules
- Ground every assumption in tool output
- Never guess - verify each assumption
- One fix at a time
- Test before and after
- Don't expand scope
