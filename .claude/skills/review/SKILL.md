# Review Skill

Code review with evidence-based feedback. Produces a recommendation, not a verdict; does not fix issues or merge. In the flow model, code-critic handles this; this skill is for ad-hoc review.

## Review Checklist

### 1. Scope
- [ ] Changes match stated intent
- [ ] No unrelated changes mixed in
- [ ] Size is reviewable (< 400 lines preferred)

### 2. Correctness
- [ ] Logic is sound
- [ ] Edge cases handled
- [ ] Error handling present
- [ ] No obvious bugs

### 3. Testing
- [ ] New code has tests
- [ ] Tests cover happy path
- [ ] Tests cover error cases
- [ ] Tests actually verify behavior (not just run)

### 4. Quality
- [ ] Code is readable
- [ ] Names are clear
- [ ] No unnecessary complexity
- [ ] Follows project patterns

### 5. Safety
- [ ] No security issues (injection, XSS, etc.)
- [ ] No credential exposure
- [ ] No data leaks
- [ ] Safe defaults

## Review Output Format
```markdown
## Summary
<one line: approve/request changes/needs discussion>

## What This Does
<brief description of the change>

## Findings

### Blocking
<issues that must be fixed>

### Suggestions
<improvements, not required>

### Questions
<things to clarify>

### Praise
<what's done well>

## Files Reviewed
| File | Status | Notes |
|------|--------|-------|
```

## Rules
- Read all changed files
- Check test coverage for new code
- Be specific: line numbers, concrete suggestions
- Distinguish blocking vs nice-to-have
