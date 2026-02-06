# Commit Skill

Create a commit with forensic evidence. In the flow model, repo-operator owns git; this skill is for direct invocation outside flows.

## Pre-Commit Verification
Before committing, run and report:
```bash
git status
git diff --stat
```

## Evidence to Collect
1. **Files changed** - list with add/remove line counts
2. **Tests status** - run relevant tests, report pass/fail
3. **Lint status** - run linter, report clean or issues

## Commit Message Format
```
<type>: <short description>

<body - what changed and why>

Evidence:
- Tests: <pass/fail count>
- Lint: <clean/warnings>
- Files: <count> changed, +<added>/-<removed> lines
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`

## Execution
1. Run `git status` and `git diff --stat`
2. Run project test command (detect from CLAUDE.md or use `cargo test`/`npm test`/etc.)
3. Run project lint command
4. Compose commit message with evidence
5. Stage relevant files (not `.env`, credentials, large binaries)
6. Execute commit
7. Show `git log -1 --stat` to confirm

## Rules
- Never use `--no-verify`
- Never force push
- Never push, create PRs, or amend unless explicitly asked
- If tests fail, stop and report - do not commit
