# Build Cleanup Report for align-doc-ownership

## Machine Summary
```yaml
status: UNVERIFIED
recommended_action: RERUN
route_to_flow: null
route_to_agent: null
blockers:
  - "self_reviewer status is UNVERIFIED"
missing_required: []
concerns:
  - "mutation_score could not be extracted (no 'Mutation Score:' line found in mutation_report.md)"
```

## Artifact Verification

| Artifact | Status |
| -------- | ------ |
| self_review.md | Present (required) |
| test_changes_summary.md | Present (required) |
| impl_changes_summary.md | Present (required) |
| flow_plan.md | Present (optional) |
| subtask_context_manifest.json | Present (optional) |
| open_questions.md | Present (optional) |
| test_critique.md | Present (optional) |
| code_critique.md | Present (optional) |
| mutation_report.md | Present (optional) |
| fix_summary.md | Present (optional) |
| doc_updates.md | Present (optional) |

## Counts Derived

| Metric | Value | Source | Method |
| ------ | ----: | ------ | ------ |
| tests_written | 27 | test_changes_summary.md | `demoswarm count pattern --regex "^- "` |
| files_changed | 22 | impl_changes_summary.md | `demoswarm count pattern --regex "^- "` |
| mutation_score | null | mutation_report.md | `demoswarm line get --prefix "Mutation Score:"` - no matching line found |
| open_questions | 5 | open_questions.md | `demoswarm count pattern --regex "^- QID:"` |

## Quality Gates

| Gate | Status | Source | Method |
| ---- | ------ | ------ | ------ |
| test_critic | VERIFIED | test_critique.md | `demoswarm ms get --section "## Machine Summary" --key "status"` |
| code_critic | VERIFIED | code_critique.md | `demoswarm ms get --section "## Machine Summary" --key "status"` |
| self_reviewer | UNVERIFIED | self_review.md | `demoswarm ms get --section "## Machine Summary" --key "status"` |

## Index Update

* updated: yes
* fields: status, last_flow, updated_at
* notes: Updated status from VERIFIED to UNVERIFIED (self_reviewer gate is UNVERIFIED). last_flow remains "build". updated_at set to 2025-12-13T08:25:02Z.

## Status Derivation

The receipt status is **UNVERIFIED** because:
1. All required artifacts are present (self_review.md, test_changes_summary.md)
2. However, the self_reviewer quality gate reports status: UNVERIFIED
3. Per pack contract: any quality gate UNVERIFIED => overall UNVERIFIED

The recommended_action is **RERUN** because:
1. A quality gate is UNVERIFIED (not CANNOT_PROCEED)
2. No required artifacts are missing
3. RERUN indicates stay in Flow 3 to address issues

## Next Steps

The self_reviewer gate is UNVERIFIED. Review `.runs/align-doc-ownership/build/self_review.md` to understand what issues were identified and address them before proceeding to Flow 4 (Gate).
