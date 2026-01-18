# Git Status

## Status: COMPLETED_WITH_ANOMALY

## Operation: checkpoint

## Before

- Branch: run/compliance-drift-proofing
- Head: bacacfe4648937f261a33371f4f50454c1898bfd
- Porcelain: modified gate artifacts, untracked temp files

## Allowlist

- .runs/compliance-drift-proofing/gate/
- .runs/compliance-drift-proofing/run_meta.json
- .runs/index.json

## Unexpected Paths (untracked outside allowlist)

- `C\357\200\272CodeSwarmdemo-swarm-staging.runscompliance-drift-proofingbuildtest_output.log` (untracked - malformed path artifact)
- test_output.log (untracked - temp file)
- test_output_unit.log (untracked - temp file)
- tools/demoswarm-pack-check/coverage/tarpaulin-report.html (untracked - coverage artifact)
- tools/demoswarm-pack-check/test_changes_summary.md (untracked - temp file)
- tools/demoswarm-pack-check/test_output.log (untracked - temp file)
- tools/demoswarm-pack-check/test_output_integration.log (untracked - temp file)
- tools/demoswarm-pack-check/test_output_unit.log (untracked - temp file)
- tools/demoswarm-pack-check/test_summary.md (untracked - temp file)

## Actions Taken

- Reset staging area
- Staged allowlist files only (16 files)
- Committed allowlist with message: "gate(compliance-drift-proofing): BOUNCE verdict - reseal required"
- Push skipped due to anomaly (unexpected untracked files outside allowlist)

## After

- Branch: run/compliance-drift-proofing
- Head: 3fd894b5ae57f48045195cf4cca659513d9288fd
- Porcelain: 9 untracked files outside allowlist

## Notes

- Tighten-only safety applied: anomaly detected, push skipped
- Anomaly files appear to be temporary test/coverage artifacts that should be cleaned up or gitignored
- Recommend running reconcile_anomaly to clean temp files before retry
