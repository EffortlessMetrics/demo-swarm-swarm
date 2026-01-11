# Implementation Changes Summary for reseal-pattern-impl

## Handoff

**What I did:** Added the `reseal-if-modified` pattern to the 4 flow commands that were missing it (flow-1-signal.md, flow-2-plan.md, flow-3-build.md, flow-6-deploy.md).

**What's left:** Nothing. All 4 flows now have the pattern documented.

**Recommendation:** Route to code-critic to verify the pattern additions are consistent with the existing pattern in flow-4-review.md, flow-5-gate.md, and flow-7-wisdom.md.

## What Changed

* **Flow command documentation:** Added the `reseal-if-modified` pattern to ensure receipts are regenerated when artifacts are modified during feedback loops or fix operations. The pattern was placed at the natural point in each flow where modifications might occur (after auditor/critic feedback loops, before final cleanup).

* **Pattern consistency:** Each flow now documents when to reseal:
  - Flow 1 (Signal): After spec-auditor feedback causes requirements-author or bdd-author reruns
  - Flow 2 (Plan): After design-critic feedback causes interface-designer or adr-author reruns
  - Flow 3 (Build): After self-reviewer identifies issues requiring fixer/implementer/test-author fixes
  - Flow 6 (Deploy): After deploy-decider recommends retrying merge operations

## REQ/NFR -> Implementation Map

| ID | Implementation Pointer | Notes |
|----|------------------------|-------|
| N/A | `.claude/commands/flow-1-signal.md::Step 9b` | Added reseal guidance after spec-auditor feedback loop |
| N/A | `.claude/commands/flow-2-plan.md::Step 10` | Added reseal guidance after design-critic feedback loop |
| N/A | `.claude/commands/flow-3-build.md::Step 7` | Added reseal guidance after self-reviewer feedback loop |
| N/A | `.claude/commands/flow-6-deploy.md::Step 4` | Added reseal guidance after deploy-decider retry scenarios |

## Tests

* Test-runner result: N/A (documentation-only changes)
* Remaining failures: None

## Known Issues / Handoffs

* None. The pattern is now documented consistently across all 7 flows.

## Assumptions Made

* The pattern wording follows the style established in flow-4-review.md, flow-5-gate.md, and flow-7-wisdom.md (using `build-cleanup` for resealing in those flows)
* Each flow's cleanup agent is the appropriate agent to call for resealing (signal-cleanup, plan-cleanup, build-cleanup, deploy-cleanup respectively)
