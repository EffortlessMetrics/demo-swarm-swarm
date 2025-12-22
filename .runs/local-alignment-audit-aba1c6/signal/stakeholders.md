# Stakeholders

## Primary

- **Pack integrators**: Rely on documentation to understand flow architecture; currently confused by "six flows" vs. "seven flows" discrepancy and undocumented flow variants
- **Documentation maintainers**: Responsible for keeping README.md, DEMO_RUN.md, architecture.md, and CHANGELOG.md in sync with CLAUDE.md source of truth

## Secondary

- **Security reviewers**: Assess pack security posture; need accurate claims about ReDoS immunity and path traversal limitations to avoid false positives/negatives in security assessments
- **Test reviewers / compliance auditors**: Validate test coverage claims against actual test execution; need accurate test counts (102 passing vs. claimed numbers) to trust quality posture

## Consulted

- **Pack architects**: Input needed on whether flow variants (flow-4-gate vs. flow-4-review) are intentional re-entry patterns or legacy artifacts; impacts how overlap semantics are documented
- **Tooling maintainers**: Consulted on whether agent color coding is advisory or schema-validated; determines documentation scope for REQ-007

## Informed

- **Downstream tooling (pack-check)**: Must be informed of seven-flow model to ensure validation rules align; wisdom.rs checks depend on correct flow enumeration
- **Future pack consumers**: Benefit from accurate documentation without needing to reconcile conflicting sources

## Notes

- CLAUDE.md is the authoritative source for flow count (stated as "repo-level policy + shared contracts")
- No external system integrations exist for this work; all changes are documentation-only
- Agent color coding consumers (if any) are unknown; requires investigation or assumption
