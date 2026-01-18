# Feedback Actions (Run local-alignment-audit-aba1c6)

## Outcome Snapshot

- issue_drafts: 4
- suggestions: 5
- inputs_present:
  - learnings: no
  - regressions: yes
  - artifact_audit: yes

---

## Flow 1 - Signal (Proposed edits)

- [ ] SUG-001: Add branch protection status to early_risks.md template
  - evidence: wisdom/regression_report.md#Recommended Next
  - proposed_change: `.claude/agents/risk-analyst.md` + Add guidance to check branch protection status during Signal risk identification; prompt agents to flag ORG_CONSTRAINT risks early when GitHub ops are allowed but governance is unknown

---

## Flow 2 - Plan (Proposed edits)

- [ ] SUG-002: Add governance verification step to observability_spec template
  - evidence: deploy/deployment_decision.md#Rationale
  - proposed_change: `.claude/agents/observability-specifier.md` + Add "Governance Observability" section template that prompts authors to document branch protection requirements and required status checks

---

## Flow 3 - Build (Issue drafts + suggestions)

- ISSUE: ISSUE-DRAFT-001: Address path traversal risk in secrets.rs (RSK-001)
  - target_flow: 3
  - labels: security, hardening, deferred
  - summary: Path traversal vulnerability identified in `tools/demoswarm-runs-tools/src/commands/secrets.rs`. The path handling lacks canonicalization which could allow scanning/redacting files outside `.runs/` boundary if untrusted input reaches path argument. Deferred from documentation-only run local-alignment-audit-aba1c6.
  - acceptance_criteria:
    - [ ] Add path canonicalization to secrets.rs path handling
    - [ ] Add unit tests for path traversal rejection (e.g., `../`, absolute paths outside boundary)
    - [ ] Document threat model assumptions in code comments
    - [ ] Pass existing pack-check and runs-tools-tests
  - evidence:
    - gate/risk_assessment.md#RSK-001: Path Traversal in secrets.rs (Deferred)
    - gate/security_scan.md#Notes for Merge-Decider

- ISSUE: ISSUE-DRAFT-002: Update cargo-audit tooling for CVSS 4.0 support
  - target_flow: 3
  - labels: tooling, dependencies, maintenance
  - summary: cargo audit could not run due to CVSS 4.0 parser limitation in cargo-audit 0.21.2. This prevented dependency vulnerability scanning during the run. Update tooling to support newer advisory database format.
  - acceptance_criteria:
    - [ ] Upgrade cargo-audit to version supporting CVSS 4.0 (if available)
    - [ ] If no upgrade available, document workaround or alternative tooling
    - [ ] Verify `cargo audit` runs successfully on both Rust tool crates
    - [ ] Add cargo-audit check to pack.yml CI workflow
  - evidence:
    - gate/security_scan.md#Dependency Risk
    - wisdom/regression_report.md#Recommended Next

- [ ] SUG-003: Add permission handling documentation to build-cleanup agent
  - evidence: wisdom/artifact_audit.md#Matrix
  - proposed_change: `.claude/agents/build-cleanup.md` + Add note about potential directory permission issues on Windows and document workaround (verify via git ls-files) when direct file reads fail

---

## Flow 5 - Gate (Issue drafts)

- ISSUE: ISSUE-DRAFT-003: Enable branch protection on main branch
  - target_flow: 5
  - labels: governance, org-constraint, repository-settings
  - summary: GitHub branch protection is not enabled on main branch. The API returned "Branch not protected" (HTTP 404). This prevents governance enforcement verification and caused deployment_verdict=NOT_DEPLOYED despite successful merge. This is an organizational/repository settings issue, not a code defect.
  - acceptance_criteria:
    - [ ] Enable branch protection on main branch via GitHub repository settings
    - [ ] Configure required status checks to include Pack CI workflow jobs (lint, pack-check, runs-tools-tests)
    - [ ] Verify subsequent runs achieve deployment_verdict=STABLE
    - [ ] Document branch protection requirements in CONTRIBUTING.md
  - evidence:
    - deploy/deployment_decision.md#Rationale
    - deploy/deployment_decision.md#failed_checks
    - wisdom/regression_report.md#Recommended Next

---

## Flow 6 - Deploy (Proposed edits)

- [ ] SUG-004: Clarify NOT_DEPLOYED vs STABLE distinction in deploy-decider prompt
  - evidence: deploy/deployment_decision.md#Rationale
  - proposed_change: `.claude/agents/deploy-decider.md` + Add explicit guidance that NOT_DEPLOYED due to ORG_CONSTRAINT is a governance limitation, not a deployment failure; clarify that merge operation can succeed while governance verification fails

---

## Flow 7 - Wisdom (Issue drafts + suggestions)

- ISSUE: ISSUE-DRAFT-004: Close Issue #1 after Wisdom flow completion
  - target_flow: 7
  - labels: run-closure, github-ops
  - summary: Issue #1 (DemoSwarm Documentation-Code Alignment Audit) remains OPEN. All work items are complete, PR #2 is merged, and release tag v1.0.0-local-alignment-audit-aba1c6 exists. Issue should be closed with final status update via gh-issue-manager after Wisdom flow completes.
  - acceptance_criteria:
    - [ ] Post final Wisdom flow comment to Issue #1 via gh-reporter
    - [ ] Close Issue #1 via gh-issue-manager with status=COMPLETED
    - [ ] Verify issue closure reflects in run_meta.json
  - evidence:
    - wisdom/regression_report.md#Issue Correlation
    - wisdom/flow_history.json#run_context

- [ ] SUG-005: Add learnings.md creation to learning-synthesizer agent prompt
  - evidence: wisdom/ directory (learnings.md not found)
  - proposed_change: `.claude/agents/learning-synthesizer.md` + Verify that learnings.md is always created even when no regressions found; document that feedback-applier depends on this file for PACK_OBS markers

---

## Pack/Flow Improvements

No `PACK_OBS` markers were found in learnings.md (file was not created for this run). The following observations are derived from artifact analysis:

- [ ] SUG-006: Add file permission resilience to wisdom artifact auditor
  - evidence: wisdom/artifact_audit.md#Note on build_receipt
  - proposed_change: `.claude/agents/artifact-auditor.md` + Document fallback pattern (git ls-files + bash read) for permission-denied scenarios; add guidance to mark as UNREADABLE rather than failing

---

## Cross-cutting (Optional)

(None identified for this run)

---

## Issues Created

None. (Drafts only; no GitHub side effects.)

---

## Actions Deferred

- **cargo audit CVSS 4.0 workaround**
  - reason: External tooling limitation; requires investigation into cargo-audit release schedule or alternative tools (cargo-deny, cargo-vet)

- **Windows permission issue root cause**
  - reason: Build directory permission errors appear environment-specific; needs investigation on Windows development environments before prescribing fix

---

## Inventory (machine countable)

- ISSUE_DRAFT: ISSUE-DRAFT-001 target_flow=3 labels="security, hardening, deferred"
- ISSUE_DRAFT: ISSUE-DRAFT-002 target_flow=3 labels="tooling, dependencies, maintenance"
- ISSUE_DRAFT: ISSUE-DRAFT-003 target_flow=5 labels="governance, org-constraint, repository-settings"
- ISSUE_DRAFT: ISSUE-DRAFT-004 target_flow=7 labels="run-closure, github-ops"
- SUGGESTION: SUG-001 target_flow=1
- SUGGESTION: SUG-002 target_flow=2
- SUGGESTION: SUG-003 target_flow=3
- SUGGESTION: SUG-004 target_flow=6
- SUGGESTION: SUG-005 target_flow=7
- SUGGESTION: SUG-006 target_flow=7

---

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - "learnings.md was not created by learning-synthesizer; feedback derived from regression_report.md and artifact_audit.md only"
  - "No Build hardening worklists present (mutation_report.md, fuzz_report.md, flakiness_report.md not found)"
```
