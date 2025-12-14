# Stakeholders

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
blockers: []
missing_required: []

## Primary

- **Pack maintainers**: Primary stakeholders. Blurred boundaries increase cognitive load when updating docs and risk introducing contradictions. They will benefit from clear ownership rules and reduced drift risk. Source: problem_statement.md (Who Is Affected).

## Secondary

- **Agent implementations**: Agents that reference skill tooling may have stale or duplicated invocation examples if skill docs are the source of truth. 55 agent files require audit for consistency. Source: problem_statement.md (Who Is Affected), context_brief.md (55 files to audit).

- **Flow orchestrators**: Flow commands that embed skill plumbing become coupled to implementation details they should not own. 6 flow command files must be cleaned of skill plumbing. Source: problem_statement.md (Who Is Affected).

## Consulted

- **Skill doc authors**: Need to confirm that skill docs contain complete CLI reference and all flag documentation. 7 skill docs serve as single source of CLI truth. Source: context_brief.md (Skill Docs).

- **pack-check / tooling maintainers**: May need to extend pack-check with boundary-enforcement drift checks. Rust development may be required if checks are non-trivial. Source: problem_statement.md (Constraints), open_questions.md (OQ-SIG-006).

## Informed

- **New contributors / onboarding**: Unclear ownership makes it harder to understand where to find or update authoritative information. Clear three-tier model improves discoverability. Source: problem_statement.md (Who Is Affected).

- **CI/CD pipeline consumers**: Validation tooling (pack-check, doc-drift) will gain new rules; existing checks must continue passing. Source: requirements.md (NFR-TEST-001).

## Notes

- **55 agent files, 6 flow commands, 7 skill docs, 1 CLAUDE.md**: This is the scope of files requiring alignment. The subtask partitioning (ST-001 through ST-006) minimizes merge conflict risk by flow.

- **Dependency on validation run**: Toy Run A/B through flows 1-4 must succeed and be recorded in `docs/maintainers/validation-log.md` before work is considered complete.

- **Prior alignment commits**: Commits be0c81a and 186ea53 established the trajectory; this work continues that direction.

- **Archive-over-delete pattern**: PR #48 established the pattern; moved/removed content should be archived, not deleted.
