# Learnings from Run: local-alignment-audit-aba1c6

## Outcome Snapshot

- Gate verdict: MERGE (all 11 receipt checks passed, 35/35 ACs complete, 0 security findings)
- Deploy outcome: NOT_DEPLOYED (ORG_CONSTRAINT: branch protection not enabled; merge operation succeeded)
- Regression count: 0 (documentation-only run, no code regressions possible)
- Run duration: 41 hours 13 minutes across 7 flows
- PR: #2 merged to main (commit ed9b9c98b7a353a29671d489148fef3ba08d933e)
- Tag: v1.0.0-local-alignment-audit-aba1c6

---

## Learning: Requirements

### What Worked

- **Explicit priority classification**: REQ-001 through REQ-007 marked as HIGH/MEDIUM/LOW enabled clear triage during review. Critical work (flow count alignment) was prioritized correctly.
- **Assumption tracking with impact analysis**: Each ASM-* included "impact if wrong" statements, enabling Gate flow to validate assumptions without re-litigating them.
- **Open questions register (OQ-SIG-*)**: 6 open questions were tracked; the swarm proceeded with documented defaults rather than blocking on human input. This reduced cycle time significantly.
- **Requirements linked to acceptance criteria**: Every REQ had explicit ACs (35 total), enabling mechanical verification in Build and Gate flows.

### What Didn't

- **OQ-SIG-001 (six vs seven flows) remained open throughout**: The core ambiguity was documented but never formally resolved. The swarm assumed "seven flows" was canonical based on CLAUDE.md authority, which proved correct, but this could have caused rework if wrong.
- **NFR-DOC-001 partial satisfaction risk**: The ADR acknowledged that secondary docs might remain stale if time-constrained. This ambiguity persisted through Gate but was ultimately resolved via the style sweep.

### Recommendation

- **Close open questions at flow boundaries**: Flow 1 should emit a "recommended defaults" document that subsequent flows can reference. If a question is answered by evidence during the run, mark it RESOLVED in the register rather than leaving it OPEN.
- **Add a "question resolution ceremony" to signal-cleanup**: Before proceeding to Plan, verify each open question has either: (a) been resolved by evidence, (b) been assigned a documented default, or (c) been escalated as a human-decision blocker.

### Evidence

- `.runs/local-alignment-audit-aba1c6/signal/requirements.md`: OQ-SIG-001, ASM-001 through ASM-005
- `.runs/local-alignment-audit-aba1c6/signal/open_questions.md`: 6 questions registered
- `.runs/local-alignment-audit-aba1c6/plan/adr.md`: ASM-001 stated CLAUDE.md is authoritative

---

## Learning: Design

### What Worked

- **Layered approach (OPT-003) reduced coordination overhead**: Updating authoritative sources first (CLAUDE.md, architecture.md), then deriving downstream docs, prevented conflicting edits and enabled clean diffs.
- **Phased ADR with incremental merge option**: Phases 1-2 were merge-critical; Phases 3-4 were optional follow-ups. This allowed the run to proceed without blocking on secondary documentation.
- **5 decision drivers with explicit requirement bindings**: Each DRIVER referenced specific REQs and NFRs, making traceability mechanical rather than interpretive.
- **Reactive test fixture strategy**: Phase 4 "only if pack-check fails" avoided premature changes to structure.rs. Pack-check passed without fixture updates.

### What Didn't

- **10 command files vs 7 flows ambiguity persisted into Review**: The distinction between "7 flows" and "10 command files" caused confusion for bot reviewers (FB-003, FB-004, FB-005). The ADR mentioned variants but did not explicitly resolve the documentation phrasing question.
- **No explicit "derive from authoritative source" automation**: Each downstream doc update was manual. If a future run modifies CLAUDE.md, downstream docs could drift again.

### Recommendation

- **Add explicit phrasing guidance to ADR for multi-path architectures**: When a system has N logical flows implemented by M command files (where M > N), the ADR should specify canonical phrasing (e.g., "7 flows exposed via 10 slash commands").
- **Consider pack-check enhancement for documentation derivation**: A future pack-check rule could validate that public doc flow counts match CLAUDE.md authoritative count.

### Evidence

- `.runs/local-alignment-audit-aba1c6/plan/adr.md`: OPT-003 chosen, Phases 1-4 defined
- `.runs/local-alignment-audit-aba1c6/plan/design_options.md`: 3 options evaluated
- `.runs/local-alignment-audit-aba1c6/review/pr_feedback.md`: FB-003, FB-004, FB-005 (command count confusion)

---

## Learning: Build

### Test Quality

- **Pack-check served as verification mechanism**: 53 structural assertions passed with 2 advisory warnings (QID patterns, non-blocking).
- **BDD scenarios (32) provided specification coverage**: All 7 REQs were covered by feature files with explicit AC mappings.
- **No code changes = no unit test delta**: This was a documentation-only run; existing pack-check tests validated structural integrity without requiring new test authoring.

### Iteration Patterns

- **Review feedback drove one significant rework (RW-001)**: The api_contracts.yaml referenced deleted command files. This was caught by Gemini Code Assist and resolved in a single iteration.
- **24 MINOR markdown formatting items resolved in bulk**: Style sweep was efficient; all items addressed in one pass via fixer routing.
- **Permission issue caused build_receipt CANNOT_PROCEED**: The build directory had permission constraints during receipt write. This was a mechanical failure, not a content defect. All artifacts were verified via git fallback.

### Recommendation

- **Add git-based fallback to receipt reading in Gate flow**: When direct file read fails due to permissions, Gate flow should attempt `git show HEAD:<path>` before marking as missing_required.
- **Template improvements for secrets_scan.md**: Recurring MD022/MD058 violations suggest the generator template needs blank lines around headings/tables. Fix once in the template to eliminate 24 repeat findings per run.
- **Fix typo "immeidate" in flow command templates**: This typo appeared in 7 flow command files. Corrected in this run but should have been caught by spell-check integration.

### Evidence

- `.runs/local-alignment-audit-aba1c6/gate/receipt_audit.md`: Build receipt CANNOT_PROCEED noted, git fallback used
- `.runs/local-alignment-audit-aba1c6/review/review_worklist.md`: 30 items total, 29 resolved, 1 skipped
- `.runs/local-alignment-audit-aba1c6/review/pr_feedback.md`: FB-026 (typo), FB-027 (template formatting)

---

## Assumptions

| Assumption | Held? | Evidence |
|-----------|-------|----------|
| ASM-001: CLAUDE.md is authoritative for flow architecture | Yes | 7 flows model validated; no contradicting evidence found |
| ASM-002: Flow variants are intentional re-entry points | Yes | OPT-003 succeeded; variants work as designed |
| ASM-003: 102 passing unit tests is authoritative count | Unknown | Test count was not re-verified in this run (docs-only) |
| ASM-004: Path traversal is awareness issue, not exploitable | Yes | RSK-001 deferred to future security hardening; no immediate exploit vector identified |
| ASM-005: Agent color coding is advisory, not schema-enforced | Yes | No schema validation failures encountered |
| ASM-006: Pack-check uses string literals, not semantic assertions | Yes | Pack-check passed without test fixture updates |

---

## Surprises

- **Bot reviewer confusion about command count vs flow count**: Both Gemini Code Assist and CodeRabbit flagged ambiguous phrasing around "7 flows" vs "10 command files". The pack has 7 logical flows but 10 slash command files (including variants). This was not anticipated as a review friction point.

- **Branch protection not enabled on main**: The deploy-decider correctly identified this as ORG_CONSTRAINT, but the swarm assumed branch protection would be present. Future runs targeting repos without branch protection will see NOT_DEPLOYED verdicts even when merge succeeds.

- **Build directory permissions blocked receipt write**: A mechanical failure (CANNOT_PROCEED) occurred during build receipt writing due to directory permissions. The git fallback mechanism in Gate was essential for validation.

- **CI workflow did not trigger post-merge**: Pack CI workflow was configured correctly but had not executed by the time verification_report was written. This is within GitHub Actions SLA but created an UNKNOWN ci_signal.

---

## Pack/Flow Observations

Friction, gaps, or improvement opportunities noticed during this run:

- PACK_OBS: Build receipt write failed due to directory permissions, requiring git fallback in Gate
  - source: gate/receipt_audit.md (L17, L40-41)
  - suggested_change: Add permission check before receipt write; emit warning if directory is read-only; consider atomic write pattern

- PACK_OBS: 24 markdown formatting violations (MD022, MD034, MD058) across generated artifacts
  - source: review/pr_feedback.md (FB-007 through FB-030)
  - suggested_change: Update secrets_scan.md template and other generators to emit blank lines around headings/tables per markdownlint rules

- PACK_OBS: "immeidate" typo in 7 flow command files
  - source: review/pr_feedback.md (FB-026)
  - suggested_change: Add spell-check to pack-check or pre-commit hooks; fixed in this run but was preventable

- PACK_OBS: No automation for deriving downstream docs from authoritative sources
  - source: plan/adr.md (Negative consequences section)
  - suggested_change: Consider pack-check rule that validates public doc flow counts against CLAUDE.md section

- PACK_OBS: Open questions remain OPEN even when evidence resolves them
  - source: signal/requirements.md (OQ-SIG-001 never closed despite proceeding with seven-flow model)
  - suggested_change: Add resolution tracking to openq-tools; mark questions RESOLVED when answer is derived from evidence

- PACK_OBS: Deploy verdict NOT_DEPLOYED due to missing branch protection, even though merge succeeded
  - source: deploy/deployment_decision.md (branch_protection: FAIL)
  - suggested_change: Document this governance constraint in run setup; add branch protection check to run-prep pre-flight

---

## Actions

- ACTION: Update secrets_scan.md generator template to include blank lines before headings and around tables (fixes MD022/MD058 pattern)
- ACTION: Add spell-check integration to pack-check or pre-commit hooks to catch typos like "immeidate"
- ACTION: Add pack-check rule to validate that public docs flow count matches CLAUDE.md "The Seven Flows" section count
- ACTION: Enhance run-prep pre-flight to check branch protection status and warn if not enforced
- ACTION: Add question resolution tracking to signal-cleanup; mark OQ entries RESOLVED when evidence provides answer
- ACTION: Document "command count vs flow count" phrasing guidance in pack documentation (7 flows via 10 command files)
- ACTION: Add git-based fallback to build receipt reading when directory permissions fail
- ACTION: Consider atomic write pattern for receipt generation to avoid partial writes on permission errors

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
  - "Build receipt CANNOT_PROCEED was a permissions artifact; all content verified via git fallback"
  - "cargo audit could not run due to CVSS 4.0 parser limitation (external tooling issue)"
  - "RSK-001 (path traversal in secrets.rs) deferred to future security hardening run"
  - "CI workflow status was UNKNOWN at verification time (within GitHub Actions SLA)"
```

---

## Learning Synthesizer Result

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - "Build receipt CANNOT_PROCEED was a permissions artifact; verified via git fallback"
  - "Branch protection not enabled; NOT_DEPLOYED is governance constraint, not content defect"
```
