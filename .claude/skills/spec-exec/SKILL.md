# Spec Exec Skill

Execute a provided specification completely in one shot.

## Input
A structured specification that defines:
- What to build
- Acceptance criteria
- Constraints

The spec defines WHAT. Your job is HOW.

## Execution Rules

### No Deviation
1. **No clarifying questions** - the spec is complete
2. **No scope expansion** - exactly what's specified
3. **No partial completion** - finish or report blockers
4. **No gold-plating** - don't add unrequested features

### Verification Required
After each implementation unit, verify via exit codes and test output, not assumptions:
1. Run tests
2. Verify lint
3. Check build
4. Only then continue

### Sub-Agent Usage
Use Task tool for:
- Parallel independent work
- Exploration that doesn't need your context
- Bounded subtasks with clear deliverables

Keep main context clean. Delegate aggressively.

## Output Format
At completion:
```markdown
## Spec Executed
<brief summary>

## Files Created/Modified
| File | Action | Lines |
|------|--------|-------|

## Commands Run
| Command | Exit Code |
|---------|-----------|

## Test Results
- Total: <count>
- Passed: <count>
- Failed: <count>

## Blockers Encountered
<any issues that prevented completion>

## Deviations from Spec
<any necessary changes, with justification>
```

## Rules
- Complete the entire spec
- Verify at each step
- Report blockers immediately
- Don't interpret - execute
