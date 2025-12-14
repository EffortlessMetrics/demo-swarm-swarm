# Example Matrix

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

## Coverage Summary

| Requirement | Happy Path | Edge Cases | Error Cases | Scenario Count | Notes |
|-------------|------------|------------|-------------|----------------|-------|
| REQ-001 | Yes | Yes | Yes | 3 | Flow command boundary enforcement |
| REQ-002 | Yes | Yes | Yes | 6 | Agent doc consistency (enums, skills, outputs) |
| REQ-003 | Yes | Yes | No | 3 | Skill doc ownership - no error mode (skill docs are authoritative) |
| REQ-004 | Yes | No | Yes | 4 | CLAUDE.md scope normalization |
| REQ-005 | Yes | Yes | No | 7 | Subtask partitioning - no error mode (partitioning is work plan) |
| REQ-006 | Yes | No | Yes | 4 | Validation run recording |
| REQ-007 | Yes | Yes | Yes | 4 | Archive-over-delete pattern |

**Total Scenarios:** 31

## Scenario Index

| REQ | Scenario | Feature File | Tags |
|-----|----------|--------------|------|
| REQ-001 | Flow commands contain only orchestration content | features/doc-ownership.feature | @REQ-001 @smoke |
| REQ-001 | Flow commands avoid CLI flag syntax | features/doc-ownership.feature | @REQ-001 @edge |
| REQ-001 | Detection of skill plumbing in flow command triggers pack-check failure | features/doc-ownership.feature | @REQ-001 @error |
| REQ-002 | Agent docs use canonical status enum values | features/doc-ownership.feature | @REQ-002 @smoke |
| REQ-002 | Agent docs use canonical recommended_action enum values | features/doc-ownership.feature | @REQ-002 |
| REQ-002 | Agent that invokes skills includes Skills section | features/doc-ownership.feature | @REQ-002 |
| REQ-002 | Agent docs with file-write rules use explicit format | features/doc-ownership.feature | @REQ-002 |
| REQ-002 | Agent docs reference skill docs for CLI details | features/doc-ownership.feature | @REQ-002 @edge |
| REQ-002 | Agent with invalid status enum fails pack-check | features/doc-ownership.feature | @REQ-002 @error |
| REQ-003 | Skill doc contains complete CLI command reference | features/doc-ownership.feature | @REQ-003 @smoke |
| REQ-003 | Skill doc contains runnable examples | features/doc-ownership.feature | @REQ-003 |
| REQ-003 | CLI details migrate from CLAUDE.md to skill docs | features/doc-ownership.feature | @REQ-003 @edge |
| REQ-004 | CLAUDE.md Skills table is summary-level only | features/doc-ownership.feature | @REQ-004 @smoke |
| REQ-004 | CLAUDE.md does not duplicate skill doc content | features/doc-ownership.feature | @REQ-004 |
| REQ-004 | CLAUDE.md references skill docs for detailed usage | features/doc-ownership.feature | @REQ-004 |
| REQ-004 | Detection of duplicated CLI flags in CLAUDE.md triggers doc-drift failure | features/doc-ownership.feature | @REQ-004 @error |
| REQ-005 | ST-001 covers Flow 1 (Signal) documentation | features/doc-ownership.feature | @REQ-005 @smoke |
| REQ-005 | ST-002 covers Flow 2 (Plan) documentation | features/doc-ownership.feature | @REQ-005 |
| REQ-005 | ST-003 covers Flow 3 (Build) documentation | features/doc-ownership.feature | @REQ-005 |
| REQ-005 | ST-004 covers Flow 4 plus cross-cutting concerns | features/doc-ownership.feature | @REQ-005 |
| REQ-005 | ST-005 covers Flow 5 (Deploy) documentation | features/doc-ownership.feature | @REQ-005 |
| REQ-005 | ST-006 covers Flow 6 (Wisdom) plus validation | features/doc-ownership.feature | @REQ-005 |
| REQ-005 | Subtasks have distinct touches patterns to minimize conflicts | features/doc-ownership.feature | @REQ-005 @edge |
| REQ-006 | Validation run is recorded after alignment completion | features/doc-ownership.feature | @REQ-006 @smoke |
| REQ-006 | Validation log entry includes required fields | features/doc-ownership.feature | @REQ-006 |
| REQ-006 | Validation run not recorded if pack-check fails | features/doc-ownership.feature | @REQ-006 @error |
| REQ-006 | Validation run not recorded if doc-drift fails | features/doc-ownership.feature | @REQ-006 @error |
| REQ-007 | Moved content retains reference to new location | features/doc-ownership.feature | @REQ-007 @smoke |
| REQ-007 | Removed content is archived not deleted | features/doc-ownership.feature | @REQ-007 |
| REQ-007 | Content moves are documented in PR description | features/doc-ownership.feature | @REQ-007 @edge |
| REQ-007 | Direct deletion without archive is flagged in review | features/doc-ownership.feature | @REQ-007 @error |

## Gaps (if any)

- REQ-003: No error case scenario because skill docs are the authoritative source; there is no "skill doc failure" mode within the alignment work itself. Skill doc presence and completeness are validated by the happy path and edge scenarios.
- REQ-005: No error case scenario because subtask partitioning is a work plan concern, not a runtime behavior. Partitioning validity is verified by edge case (non-overlapping patterns).

## Notes

- Counts are derived mechanically by signal-cleanup; this matrix is for human navigation.
- All 7 functional requirements have at least one happy path scenario.
- Error scenarios focus on pack-check and doc-drift validation failures, which are the primary automated enforcement mechanisms.
- NFRs (NFR-MAINT-001, NFR-TEST-001, NFR-REGR-001) are non-behavioral and covered in verification_notes.md.
