# Refactor Skill

Safe refactoring with verification.

## Pre-Refactor
Before changing anything:
```bash
# Capture baseline
git status  # must be clean
<run tests> # must pass
```

If tests don't pass, fix them first. Never refactor with failing tests.

## Refactor Types

### Extract
Pull code into new function/module:
1. Identify code to extract
2. Write the new function
3. Replace original with call
4. Run tests
5. Verify behavior unchanged

### Rename
Change names for clarity:
1. Identify all usages
2. Rename systematically
3. Run tests
4. Verify no broken references

### Move
Relocate code to better home:
1. Identify new location
2. Move code
3. Update all imports/references
4. Run tests
5. Verify no broken dependencies

### Simplify
Reduce complexity:
1. Identify complex code
2. Rewrite more simply
3. Run tests
4. Verify behavior unchanged

## Verification After Each Step
```bash
<run tests>
git diff --stat
```

## Output Format
```markdown
## Refactor Goal
<what and why>

## Changes Made
| File | Change | Lines |
|------|--------|-------|

## Verification
- Tests before: ✓ (<count> passed)
- Tests after: ✓ (<count> passed)
- Behavior change: None

## Diff Summary
<git diff --stat output>
```

## Rules
- Improving the mold improves all future generation
- Small steps, test after each
- Never change behavior while refactoring
- Never refactor and add features simultaneously
- If tests fail, revert and retry smaller
