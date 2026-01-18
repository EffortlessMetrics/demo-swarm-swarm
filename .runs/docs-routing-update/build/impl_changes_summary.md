# Implementation Changes Summary for docs-routing-update

## Handoff

**What I did:** Updated all six how-to docs in `docs/how-to/` to align with natural language routing. Removed references to parsing structured routing blocks (`recommended_action`, `route_to_flow`, `route_to_agent`) for live routing decisions. Added clarification that these fields exist in receipts for audit purposes only, derived from prose handoffs.

**What's left:** Nothing. All specified files have been updated.

**Recommendation:** Route to **code-critic** for review of documentation changes.

## What Changed

- **add-an-agent.md**: Replaced the "Control-plane routing" and "Control-plane return" sections with "Handoff Targets" and prose handoff structure. Updated the Output Contract section to emphasize handoffs over Machine Summary blocks. Revised the Critic Pattern and Cleanup Agent Pattern examples to use prose handoffs. Updated the checklist and minimal examples.

- **create-a-flow.md**: Updated the Microloop Template to reference prose handoffs instead of "control-plane Result". Changed the Control-Plane Blocks section to clarify that domain agents use prose handoffs while only boolean gates (secrets-sanitizer, repo-operator) use structured blocks. Rewrote the Routing Logic Template to describe prose-based routing. Added a note to Receipt Schema explaining routing fields are for audit only.

- **troubleshoot.md**: Updated the Quick Diagnosis Ladder entry for "Microloop won't stop" to reference prose handoffs. Rewrote the "Microloop won't terminate" section to explain prose-based routing instead of field-based parsing.

- **working-with-receipts.md**: Updated the introduction to clarify receipts are for audit, not routing. Replaced "Using Receipts for Routing" section with "Using Receipts" which emphasizes the audit purpose. Added explanatory notes to Common Receipt Patterns showing how routing fields are derived from prose.

- **working-with-microloops.md**: Updated all routing-related sections to reference prose handoffs. Rewrote the "Routing on the Result Block" section as "Routing on the Handoff" with natural language routing rules. Updated the Example showing prose handoff format. Revised Anti-Patterns to include "Parsing Structured Fields for Routing" as an anti-pattern. Updated the Microloop Template.

- **adapt-to-non-github.md**: Updated the "Status/routing enums" section to clarify these are audit vocabulary for receipts, not routing contracts.

## REQ/NFR -> Implementation Map

| ID  | Implementation Pointer                   | Notes                                                          |
| --- | ---------------------------------------- | -------------------------------------------------------------- |
| N/A | `docs/how-to/add-an-agent.md`            | Removed control-plane routing sections, added handoff patterns |
| N/A | `docs/how-to/create-a-flow.md`           | Updated microloop and routing templates for prose handoffs     |
| N/A | `docs/how-to/troubleshoot.md`            | Updated microloop troubleshooting for prose-based routing      |
| N/A | `docs/how-to/working-with-receipts.md`   | Clarified receipts are audit logs, not routing inputs          |
| N/A | `docs/how-to/working-with-microloops.md` | Complete rewrite of routing sections for prose handoffs        |
| N/A | `docs/how-to/adapt-to-non-github.md`     | Clarified enum values are for receipts only                    |

## Tests

- Test-runner result: N/A (documentation changes only)
- Remaining failures: None

## Known Issues / Handoffs

- None. All specified files have been updated consistently.

## Assumptions Made

- Assumed the contracts.md file in docs/reference/ already reflects the new pattern correctly (verified by reading it - it does).
- Assumed the goal is to remove teaching of structured routing block parsing, while preserving the structured blocks themselves for audit purposes in receipts.
