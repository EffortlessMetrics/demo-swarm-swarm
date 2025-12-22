```yaml
schema_version: deployment_decision_v1
deployment_verdict: NOT_DEPLOYED
gate_verdict: MERGE
default_branch: main

verification:
  ci_workflows: PASS
  branch_protection: FAIL
  runtime_verification: PASS
  pre_commit: N/A
  documentation: PASS

failed_checks:
  - check: branch_protection
    status: FAIL
    reason: "GitHub API returned 404 'Branch not protected' for main branch; no required status checks configured"

recommended_actions:
  - "Enable branch protection on main branch via GitHub repo settings"
  - "Configure required status checks to include Pack CI workflow jobs (lint, pack-check, runs-tools-tests)"
  - "This is an org-level constraint; the run artifacts are complete and merge succeeded"
```

# Deployment Decision

## Evidence

* Gate: `gate/merge_decision.md` - verdict: MERGE, status: VERIFIED
* CI workflows: `.github/workflows/pack.yml` (5 jobs: lint, pack-check, demoswarm-smoke, runs-tools-tests, doc-drift)
* Branch protection: gh API repos/EffortlessMetrics/demo-swarm-swarm/branches/main/protection - returned 404 "Branch not protected"
* Runtime verification: `deploy/verification_report.md` - status: VERIFIED, smoke_signal: STABLE, all post-merge checks passed
* Pre-commit: No `.pre-commit-config.yaml` file present (N/A)
* Documentation: README.md + CONTRIBUTING.md provide clear dev/CI instructions (PASS)

## Rationale

The Gate verdict is MERGE with all checks passing. The PR was successfully merged to main (commit `ed9b9c98b7a353a29671d489148fef3ba08d933e`) and tagged (`v1.0.0-local-alignment-audit-aba1c6`). Runtime verification confirms:
- Merge commit present in main branch history
- Release tag created and pushed
- All 7 flow command files verified present
- No "six flows" documentation drift remaining
- Pack structure and contracts verified intact

**However**, branch protection is not enabled on the main branch. The GitHub API explicitly returned "Branch not protected" (HTTP 404). This means:
- Required status checks are not enforced
- PRs can be merged without CI passing
- The governance posture cannot be verified as enforced

Per the deploy-decider operating invariants, governance enforcement must be verifiable for a STABLE verdict. Since branch protection is a critical check and it returned FAIL, the verdict is NOT_DEPLOYED despite the successful merge operation.

**Important context:** This is an org-level/repo-settings constraint, not a defect in the run artifacts. The merge operation completed successfully, all run artifacts are complete, and the code changes are correct. The NOT_DEPLOYED verdict reflects that governance enforcement cannot be verified, not that the deployment failed.

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_station: null
route_to_agent: null
blockers:
  - "ORG_CONSTRAINT: Branch protection not enabled on main branch; required status checks not configured"
missing_required: []
concerns:
  - "Merge operation completed successfully but governance enforcement is not verifiable"
  - "CI workflows exist and would run on push/PR, but are not required to pass before merge"
  - "Consider enabling branch protection before next production deployment"
