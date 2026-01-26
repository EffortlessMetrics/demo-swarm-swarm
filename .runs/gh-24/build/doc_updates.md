# Documentation Updates for gh-24

## Handoff

**What I did:** Added explicit phrasing guidance to prevent reviewer confusion about "7 flows" vs command/skill counts. Updated 3 documentation files with canonical phrasing and a clarifying table.

**What's left:** Nothing. All acceptance criteria from issue #24 are addressed.

**Recommendation:** Route to doc-critic for review of documentation accuracy and completeness.

## Inputs Used

- GitHub issue #24 (viewed via `gh issue view 24`)
- `CLAUDE.md` (read for current structure)
- `docs/explanation/architecture.md` (read for current structure)
- `docs/reference/glossary.md` (read for existing terminology)
- `.claude/commands/` directory listing (8 files verified)
- `.claude/skills/` directory listing (7 skills verified)

## Files Updated

| File | Change Type | Summary |
| ---- | ----------- | ------- |
| `docs/explanation/architecture.md` | updated | Added "7 logical flows exposed via 8 slash commands" canonical phrasing; added "Flows vs Commands vs Skills" table with counts and when-to-use guidance |
| `CLAUDE.md` | updated | Added canonical phrasing and "Counts clarified" section under The Seven Flows |
| `docs/reference/glossary.md` | updated | Added "Command" entry with counts clarification, distinguishing flows/commands/skills |

## Deferred / Not Updated (and why)

None. All files mentioned in the issue were updated.

## Mismatches Found (if any)

**Issue text vs actual counts:** The issue mentioned "10 command files" but the actual count is 8 (7 flow commands + customize-pack). Updated documentation with correct count of 8.

## Assumptions Made

- **Assumed the "10 command files" in the issue was outdated.** Verified actual count is 8 command files in `.claude/commands/`. Updated docs with accurate count. Impact: Documentation is accurate rather than matching potentially stale issue text.

## Evidence

Verified counts via directory listing:
- Commands: 8 files in `.claude/commands/` (flow-1 through flow-7 + customize-pack)
- Skills: 7 directories in `.claude/skills/` (auto-linter, openq-tools, policy-runner, runs-derive, runs-index, secrets-tools, test-runner)
- Flows: 7 logical stages (Signal, Plan, Build, Review, Gate, Deploy, Wisdom)

## Canonical Phrasing Added

**Primary phrase:** "7 logical flows exposed via 8 slash commands"

This phrasing appears in:
1. `docs/explanation/architecture.md` line 246
2. `CLAUDE.md` line 134
3. `docs/reference/glossary.md` (in Command entry)

## Summary Table Added

The "Flows vs Commands vs Skills" table in `docs/explanation/architecture.md` provides:

| Concept | Count | What It Is | How to Invoke |
| ------- | ----- | ---------- | ------------- |
| Flow | 7 | Logical SDLC stage | Via its slash command |
| Command | 8 | Slash command file | `/flow-N-name` or `/customize-pack` |
| Skill | 7 | Mechanical tool | `/skill-name` or via agent |
