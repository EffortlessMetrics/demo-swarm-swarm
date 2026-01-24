# gh-reporter Accessibility Improvements

**Date**: 2026-01
**Related PRs**: #70 (Jules), replacement PR (maintainer takeover)

## Learning

The gh-reporter agent generates Markdown comments on GitHub issues that serve as the "UI" for this system. Accessibility improvements to the prompt directly improve the UX of all generated reports.

## Observations

1. **Builds on existing cockpit UX work** - The patterns from `2026-01-cockpit-ux.md` (navigation links, semantic status indicators) were not yet applied to gh-reporter specifically

2. **Flow summaries need navigation aids** - Reports can be long; "Jump to" links help keyboard users and small-screen reviewers find sections quickly

3. **Status indicators help scanning** - Visual status (emoji + text) makes it faster to scan for pass/fail states without reading full text

## Implementation

Applied to `.claude/agents/gh-reporter.md`:

- Added accessibility guidelines section in Step 3 (comment body building)
- Added "Jump to" navigation examples in Flow 1 and Flow 3 summary guidance
- Updated status reporting examples to use visual indicators (`✅ VERIFIED`, `⚠️ PARTIAL`, `❌ FAIL`)
- Added Hard Rule #8 enforcing accessibility requirements

## Consistency with Existing Patterns

This change applies and extends the patterns documented in `2026-01-cockpit-ux.md`:
- ✅ = positive/passing state
- ⚠️ = warning/partial state (extension of the cockpit-ux status palette, which uses ⚪ for neutral/skipped/N/A)
- ❌ = negative/failing state
- Navigation links for keyboard accessibility
