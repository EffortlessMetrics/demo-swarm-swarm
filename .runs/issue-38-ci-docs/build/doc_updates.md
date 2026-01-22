# Documentation Updates for issue-38-ci-docs

## Handoff

**What I did:** Created new `docs/how-to/handling-ci-failures.md` guide addressing GitHub Issue #38. Updated `docs/how-to/troubleshoot.md` with a CI Failures section linking to the new guide. Updated `docs/how-to/README.md` index to include the new guide.

**What's left:** Nothing. All acceptance criteria from Issue #38 have been addressed.

**Recommendation:** Route to doc-critic to review documentation for staleness and accuracy. The guide covers all CI jobs from `pack.yml`, includes CodeRabbit interpretation guidance, provides a decision tree for fix vs bounce, and documents the fix-forward-runner usage.

## Inputs Used
- `.github/workflows/pack.yml` (CI job definitions)
- `scripts/check_portable_claude.py` (lint job details)
- `scripts/lint_frontmatter.py` (lint job details)
- `scripts/check-doc-drift.sh` (doc-drift job details)
- `.claude/scripts/pack-check.sh` (pack-check job details)
- `.claude/agents/fix-forward-runner.md` (fix-forward pattern)
- `docs/explanation/bounded-fix-forward.md` (fix-forward philosophy)
- `docs/how-to/troubleshoot.md` (existing troubleshooting guide)
- `docs/how-to/orchestrator-decision-tree.md` (routing decisions)
- `docs/how-to/failure-recovery.md` (recovery patterns)
- `docs/how-to/review-a-swarm-pr.md` (review process)

## Files Updated
| File | Change Type | Summary |
|------|-------------|---------|
| `docs/how-to/handling-ci-failures.md` | created | New guide covering CI job interpretation, CodeRabbit feedback, common failure patterns, fix vs bounce decision tree, and fix-forward-runner usage |
| `docs/how-to/troubleshoot.md` | updated | Added CI Failures section at top with link to new guide |
| `docs/how-to/README.md` | updated | Added entry for handling-ci-failures.md in the how-to index table |

## Deferred / Not Updated (and why)
- None. All planned updates completed.

## Mismatches Found (if any)
- None. All CI jobs documented match `pack.yml` definitions.

## Assumptions Made
- Assumed CodeRabbit feedback patterns based on common code review bot behavior (style, complexity, documentation, security, performance categories). The pack does not have explicit CodeRabbit configuration, so these categories are generic.
- Assumed fix-forward-runner is the primary mechanism for mechanical fixes at Gate based on agent documentation. The guide documents this as the Flow 5 fix path.

## Acceptance Criteria Verification (Issue #38)
- [x] `docs/how-to/handling-ci-failures.md` exists
- [x] Covers all CI job types from pack.yml (lint, pack-check, demoswarm-smoke, runs-tools-tests, doc-drift)
- [x] Includes decision tree for fix vs bounce
- [x] Linked from troubleshoot.md
