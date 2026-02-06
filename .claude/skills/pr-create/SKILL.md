# PR Create Skill

Create a pull request with forensic evidence. For full SDLC work, start with `/flow-1-signal` (then `/flow-2-plan` → `/flow-3-build`).

## Pre-PR Verification
```bash
git status
git log --oneline main..HEAD
git diff --stat main..HEAD
```

## Evidence to Collect
1. **Commits included** - list all commits in the PR
2. **Files changed** - diff stat against base branch
3. **Tests status** - full test suite results
4. **Lint status** - clean or list issues

## PR Description Format
```markdown
## Summary
<1-3 bullet points describing the change>

## Changes
<files changed with brief description of each>

## Evidence
- **Commits**: <count>
- **Tests**: <pass/fail count>
- **Lint**: <clean/warnings>
- **Files**: <count> changed, +<added>/-<removed> lines

## Test Plan
<how to verify this works>

## Unknowns
<what wasn't tested, edge cases, risks>
```

## Execution
1. Verify branch is up to date with remote
2. Run full test suite
3. Run linter
4. Collect diff stats
5. Create PR with `gh pr create`
6. Report PR URL

## Rules
- Never create a PR with failing tests or without a test plan
- Always include test plan
- Always acknowledge unknowns
- Push branch before creating PR if needed
