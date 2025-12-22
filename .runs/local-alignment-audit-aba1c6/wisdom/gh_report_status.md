# GitHub Report Status

## Posting
posting_status: POSTED
reason: null
publish_mode: RESTRICTED
link_style: PATHS_ONLY
publish_surface: NOT_PUSHED

## Target
type: issue
number: 1
repository: EffortlessMetrics/demo-swarm-swarm

## Comment
comment_id: 3679628254
comment_url: https://github.com/EffortlessMetrics/demo-swarm-swarm/issues/1#issuecomment-3679628254

## Content Posted

**Flow 7 (Wisdom) summary** with:
- Run completion status (all 7 flows executed)
- Machine-derived counts from wisdom_receipt.json:
  - learnings_extracted: 3
  - feedback_actions_created: 4
  - suggestions_created: 6
  - regressions_found: 0
  - pack_observations: 6
- Flow status table (Signal/Plan/Review/Gate/Deploy/Wisdom = VERIFIED; Build = CANNOT_PROCEED)
- Quality gates status (all 7 quality gates VERIFIED)
- Final run status: VERIFIED
- Deployment verdict: NOT_DEPLOYED (governance constraint)
- Publishing mode: RESTRICTED (paths only, machine counts only; no human-authored markdown or blob links)
- Local artifact paths for detailed insights

## Verification
- [x] Comment posted to issue #1
- [x] Comment ID captured: 3679628254
- [x] Idempotency marker present: <!-- DEMOSWARM_RUN:local-alignment-audit-aba1c6 FLOW:wisdom -->
- [x] RESTRICTED mode compliance: paths only, machine-derived counts only
- [x] No secrets/tokens included
- [x] No large diffs or code blocks
- [x] Links are artifact paths, not blob links

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - publish_surface: NOT_PUSHED (repo anomaly detected by repo-operator; governance constraint, not a code defect)
  - Build CANNOT_PROCEED: Wisdom flow proceeded best-effort with alternative inputs; all quality gates passed
