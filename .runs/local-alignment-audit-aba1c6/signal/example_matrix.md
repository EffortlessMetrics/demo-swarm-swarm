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
| REQ-001 | Yes | Yes | No | 5 | Flow count references in public docs |
| REQ-002 | Yes | Yes | No | 5 | Flow overlap semantics documentation |
| REQ-003 | Yes | No | Yes | 4 | Flow 7 purpose and usage |
| REQ-004 | Yes | No | No | 3 | CLAUDE.md flow table updates |
| REQ-005 | Yes | Yes | Yes | 5 | Test count documentation accuracy |
| REQ-006 | Yes | Yes | Yes | 5 | Security posture documentation |
| REQ-007 | Yes | Yes | Yes | 5 | Agent color coding clarification |

## Scenario Index

| REQ | Scenario | Feature File | Tags |
|-----|----------|--------------|------|
| REQ-001 | README references seven flows | features/flow_count_alignment.feature | @REQ-001 @smoke |
| REQ-001 | DEMO_RUN references seven flows with enumeration | features/flow_count_alignment.feature | @REQ-001 |
| REQ-001 | Architecture documentation references seven flows | features/flow_count_alignment.feature | @REQ-001 |
| REQ-001 | CHANGELOG clarifies actual command count | features/flow_count_alignment.feature | @REQ-001 |
| REQ-001 | No stale flow count references remain in public documentation | features/flow_count_alignment.feature | @REQ-001 @edge |
| REQ-002 | Documentation explains flow-4-gate vs flow-4-review | features/flow_overlap_documentation.feature | @REQ-002 @smoke |
| REQ-002 | Documentation explains flow-5-gate vs flow-5-deploy | features/flow_overlap_documentation.feature | @REQ-002 |
| REQ-002 | Documentation explains flow-6-deploy vs flow-6-wisdom | features/flow_overlap_documentation.feature | @REQ-002 |
| REQ-002 | Flow variant guidance is actionable | features/flow_overlap_documentation.feature | @REQ-002 @edge |
| REQ-002 | Flow overlap documentation is discoverable | features/flow_overlap_documentation.feature | @REQ-002 |
| REQ-003 | Flow 7 is included in flow enumeration | features/flow_overlap_documentation.feature | @REQ-003 @smoke |
| REQ-003 | Flow 7 purpose is documented | features/flow_overlap_documentation.feature | @REQ-003 |
| REQ-003 | Flow 7 vs Flow 6 wisdom difference is explained | features/flow_overlap_documentation.feature | @REQ-003 |
| REQ-003 | Missing Flow 7 documentation is flagged | features/flow_overlap_documentation.feature | @REQ-003 @error |
| REQ-004 | CLAUDE.md flow table lists all seven flows | features/flow_count_alignment.feature | @REQ-004 @smoke |
| REQ-004 | CLAUDE.md flow table includes variant commands | features/flow_count_alignment.feature | @REQ-004 |
| REQ-004 | CLAUDE.md flow table numbering is consistent | features/flow_count_alignment.feature | @REQ-004 |
| REQ-005 | Documentation references correct passing test count | features/test_count_documentation.feature | @REQ-005 @smoke |
| REQ-005 | Documentation explains filtered tests | features/test_count_documentation.feature | @REQ-005 |
| REQ-005 | Conflicting test count claims are corrected | features/test_count_documentation.feature | @REQ-005 @edge |
| REQ-005 | Test count claims include source reference | features/test_count_documentation.feature | @REQ-005 |
| REQ-005 | Undocumented test count source is flagged | features/test_count_documentation.feature | @REQ-005 @error |
| REQ-006 | Documentation states Rust regex is ReDoS immune | features/security_posture_documentation.feature | @REQ-006 @smoke |
| REQ-006 | Path traversal is documented as known limitation | features/security_posture_documentation.feature | @REQ-006 |
| REQ-006 | Invalid ReDoS vulnerability claim is corrected | features/security_posture_documentation.feature | @REQ-006 @error |
| REQ-006 | Security claims reference code evidence | features/security_posture_documentation.feature | @REQ-006 |
| REQ-006 | Security claims are verifiable by code inspection | features/security_posture_documentation.feature | @REQ-006 @edge |
| REQ-007 | Documentation acknowledges color field existence | features/agent_color_coding.feature | @REQ-007 @smoke |
| REQ-007 | Documentation clarifies color field purpose | features/agent_color_coding.feature | @REQ-007 |
| REQ-007 | Color consumer is documented if functional | features/agent_color_coding.feature | @REQ-007 @edge |
| REQ-007 | Example agent frontmatter includes color field | features/agent_color_coding.feature | @REQ-007 |
| REQ-007 | Incorrect documentation-only claim is corrected | features/agent_color_coding.feature | @REQ-007 @error |

## Gaps

None. All REQ-* requirements have at least one scenario with a primary @REQ-### tag.

## Notes

- Counts are derived mechanically by signal-cleanup; this matrix is for human navigation.
- This is a documentation alignment audit; scenarios describe documentation verification rather than code execution behavior.
- NFR coverage (NFR-DOC-001, NFR-SEC-001, NFR-TRACE-001) is documented in verification_notes.md as these are non-behavioral requirements.
- Open questions OQ-SIG-001 through OQ-SIG-006 exist in open_questions.md with suggested defaults allowing scenarios to proceed.
