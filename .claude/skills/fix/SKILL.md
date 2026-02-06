# Fix Skill

Bug fix workflow with verification. In the flow model, fixer handles critic-identified issues; this skill is for ad-hoc bugfixes outside flows (for investigation-first work, use `/debug`).

## Input
Specify one of:
- Issue/bug description
- Error message
- Failing test
- Reproduction steps

## Workflow

### 1. Understand
```
- What is the expected behavior?
- What is the actual behavior?
- When did it start failing? (git bisect if needed)
```

### 2. Reproduce
```
- Create minimal reproduction
- Capture exact error/output
- Confirm it's reproducible
```

### 3. Locate
```
- Find the code responsible
- Trace from symptom to cause
- Identify root cause (not just symptom)
```

### 4. Fix
```
- Minimal change to fix root cause
- Don't fix adjacent issues (note them instead)
- Don't refactor while fixing
```

### 5. Test
```
- Add test that fails without fix
- Verify test passes with fix
- Run full test suite
- Re-run original reproduction
```

### 6. Document
```
- Commit message explains the bug and fix
- Note if related issues exist
```

## Output Format
```markdown
## Bug
<description>

## Root Cause
<what was actually wrong>

## Fix
<what was changed>

## Evidence
- Reproduction: <before/after>
- New test: <name>
- Test suite: <pass/fail count>

## Related Issues
<any adjacent problems noticed but not fixed>
```

## Rules
- One bug, one fix. Never expand scope. Never refactor while fixing.
- Always add a regression test
- Note but don't fix adjacent issues
