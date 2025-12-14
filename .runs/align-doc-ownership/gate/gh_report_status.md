# GitHub Report Status

## Posting
posting_status: SKIPPED
reason: Issue #49 does not exist in repository EffortlessMetrics/demo-swarm-staging

## Target
type: issue
number: 49
repository: EffortlessMetrics/demo-swarm-staging

## Comment
comment_id: null

## Content Prepared
Gate verdict BOUNCE comment prepared in github_report.md but not posted due to missing issue.
Summary: MECH-001 formatting violations require cargo fmt; all substantive gates passed.

## Verification
- [ ] Comment visible on GitHub (SKIPPED - issue not found)
- [ ] Links resolve correctly (N/A)

## Machine Summary
status: UNVERIFIED
recommended_action: FIX_ENV
route_to_flow: null
route_to_agent: gh-issue-manager
blockers:
  - Issue #49 not found in repository
missing_required:
  - Valid GitHub issue for posting
concerns:
  - run_meta.json references issue_number: 49 which does not exist
  - GitHub comment not posted; local artifact written for manual review
