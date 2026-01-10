# Implementation Changes Summary for critic-refactor

## Implementation Facts
work_status: COMPLETED
tests_run: no
tests_passed: unknown

## What Changed

Rewrote all 9 critic agent prompts to align with Claude-native design principles:

* **Structure overhaul:** Replaced harness-era patterns (Machine Summary YAML blocks, status enums, route_to_agent fields) with a cleaner template structure: Your Job, What You'll Need, What You Produce, What to Look For, Writing Your Critique, Tips, If You're Stuck, Handoff.

* **Positive prompting:** Converted negative constraints ("Do NOT", "NEVER", "must NOT") to positive guidance and tips. Tells critics what TO do rather than what to avoid.

* **Single responsibility clarity:** Each critic now has a one-sentence job description at the top that makes their single purpose clear.

* **Graceful outcomes:** Added "If You're Stuck" sections with explicit guidance that partial progress is success. Blockers are findings to report, not failures.

* **Substantive critiques:** Added "Writing Your Critique" sections with sparse vs. rich examples showing how to write findings that explain what's wrong, why it matters, and how to fix it.

* **Improved handoffs:** Replaced Machine Summary blocks with prose-based Handoff sections using the three-part structure: What I found, What's left, Recommendation.

## Files Changed

| File | Change |
|------|--------|
| `.claude/agents/requirements-critic.md` | Full rewrite |
| `.claude/agents/bdd-critic.md` | Full rewrite |
| `.claude/agents/design-critic.md` | Full rewrite |
| `.claude/agents/contract-critic.md` | Full rewrite |
| `.claude/agents/observability-critic.md` | Full rewrite |
| `.claude/agents/code-critic.md` | Full rewrite |
| `.claude/agents/test-critic.md` | Full rewrite |
| `.claude/agents/doc-critic.md` | Full rewrite |
| `.claude/agents/option-critic.md` | Full rewrite |

## Patterns Applied

### Removed (harness-era)
- `## Machine Summary` YAML blocks
- `status: VERIFIED | UNVERIFIED | CANNOT_PROCEED` enums
- `recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV` enums
- `route_to_agent:` / `route_to_flow:` fields
- `blockers: []` / `concerns: []` as YAML arrays
- `can_further_iteration_help:` field
- `observations:` field

### Added (Claude-native)
- `## Your Job` - single clear sentence
- `## What You'll Need` - inputs listed with context notes
- `## What You Produce` - output file(s)
- `## What to Look For` - positive guidance on domain expertise
- `## Writing Your Critique` - sparse vs. rich examples
- `## Tips` - helpful guidance (not constraints)
- `## If You're Stuck` - graceful blockers handling
- `## Handoff` - three-part prose structure with examples

## Known Issues / Handoffs

None. All 9 critics successfully rewritten.

## Assumptions Made

* **Kept domain expertise:** Preserved what each critic actually checks for (testability, traceability, contract validity, etc.) while restructuring the prompt format.
* **Kept severity levels:** Retained CRITICAL/MAJOR/MINOR severity classifications as they provide useful signal.
* **Kept issue ID prefixes:** Retained patterns like `BDD-CRIT-001`, `DC-MAJ-002` for machine countability.

## Inventory
- IMPL_REQ_IMPLEMENTED: critic-refactor
- IMPL_TESTS_RUN: no
- IMPL_TESTS_PASSED: unknown

## Handoff

**What I did:** Rewrote all 9 critic agent prompts (.claude/agents/*-critic.md) to align with Claude-native design principles. Removed harness-era Machine Summary blocks and status enums. Added positive prompting with "Your Job", "Tips", and "If You're Stuck" sections. Improved handoff structure with three-part prose format.

**What's left:** Nothing - all 9 critics have been updated.

**Recommendation:** Review the updated critic prompts and verify they maintain the domain expertise while following the new structure. Consider testing with a sample run to ensure handoffs work correctly.
