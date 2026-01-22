# PR Cockpit UX Improvements

**Date**: 2026-01
**Related PRs**: #63, #64, #65, #66 (consolidated into single replacement)

## Learning

When improving the UX of AI-generated content (like PR descriptions), we are effectively "prompt engineering" for UX. The "code" is the prompt template.

## Observations

1. **Navigation aids matter** - Jump links at the top help keyboard/screen reader users navigate text-heavy reports

2. **Status indicators need both icon AND text** - Per accessibility rules ("don't rely on color alone"), use patterns like `✅ PASS` not just `PASS` or just `✅`

3. **Decorative vs semantic** - Status indicators that convey state (pass/fail/neutral) are semantic; emoji that just decorate (📊 for coverage) add noise without meaning

## Implementation

- Added navigation links with explicit anchor matching guidance
- Added semantic status indicators (✅/❌/⚪) paired with text
- Rationalized the indicator vocabulary:
  - ✅ = positive/passing state
  - ❌ = negative/failing state
  - ⚪ = neutral/skipped/N/A
- Extended accessibility guidance in design principles
