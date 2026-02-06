# PR Prep Skill

Prepare a branch for pull request submission.

## Checklist
Run through and report status:

### 1. Branch Hygiene
```bash
git fetch origin
git log --oneline origin/main..HEAD
git status
```
- [ ] All changes committed
- [ ] No untracked files that should be committed
- [ ] Commits are logical and atomic

### 2. Code Quality
- [ ] Tests pass
- [ ] Linter clean
- [ ] No debug code left behind
- [ ] No commented-out code
- [ ] No TODO comments for this PR's scope

### 3. Documentation
- [ ] Code comments where logic is non-obvious
- [ ] README updated if public API changed
- [ ] CHANGELOG updated if applicable

### 4. Evidence Collection
```bash
git diff --stat origin/main..HEAD
```

## Output
Report a prep summary:

| Check | Status | Notes |
|-------|--------|-------|
| Commits | ✓/✗ | count, any need squashing? |
| Tests | ✓/✗ | pass/fail count |
| Lint | ✓/✗ | warning count |
| Docs | ✓/✗ | any updates needed? |
| Debug code | ✓/✗ | any found? |

## If Issues Found
List each issue and whether to:
- Fix now (blocking)
- Note in PR (non-blocking)
- Ignore (false positive)
