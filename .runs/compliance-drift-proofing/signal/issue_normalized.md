# Normalized Issue (Iteration 2 Rerun)

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
blockers: []
missing_required: []
notes:

- Iteration 2 rerun per orchestrator signal (2025-12-18)
- GitHub issue #8 created and bound (canonical_key: gh-8, issue_binding: IMMEDIATE)
- Initial Flow 1 signal VERIFIED 2025-12-17; domain artifacts remain valid
- PR branch fixed Flow 1 GitHub operations blocking issues (issue_binding: DEFERRED -> IMMEDIATE)
- Rerun focuses on tail steps: cleanup → gates → GitHub ops
- No sensitive content detected

## Summary

Comprehensive mechanical compliance enforcement and drift-proofing for DemoSwarm pack (GitHub issue #8, OPEN). Six functional requirements: (1) flow command boundary validation, (2) Skills section enforcement, (3) OpenQ prefix normalization, (4) Build-to-Gate test scenarios, (5) warning-first mode with --strict, (6) false-positive prevention.

**Iteration 2 Context**: Initial signal VERIFIED 2025-12-17 (6 FReq, 6 NFReq, 39 BDD scenarios, 9 open questions). Issue #8 created 2025-12-18 via PR #7. Rerun to finalize binding and complete GitHub reporting.

## Signal Type

- request_type: feature (compliance/quality infrastructure)
- source_type: github_issue
- canonical_key: gh-8
- issue_number: 8
- issue_url: https://github.com/EffortlessMetrics/demo-swarm-staging/issues/8
- issue_state: OPEN

## Observed vs Expected

- observed: Pack has foundational drift infrastructure (pack-check 29 checks, check-doc-drift.sh 6 guards) but gaps remain: flow commands can contain skill CLI syntax, handshake validation is manual, agents may lack Skills sections, OpenQ prefix inconsistency (PLN vs PLAN), incomplete Build-to-Gate fixtures.
- expected: Automated compliance enforcement detecting drift at authoring time, validated flow boundaries, all demoswarm.sh users have Skills sections, normalized OpenQ prefixes, documented test scenarios.

## Impact

- affected_users: Pack maintainers, agent authors, cleanup agents, flow maintainers, downstream swarm users
- severity: medium (infrastructure/quality improvement)
- frequency: always (structural concern)
- environment: all (documentation and tooling)

## Components Mentioned

- systems/services:
  - pack-check (Rust at tools/demoswarm-pack-check/, drift.rs and structure.rs)
  - check-doc-drift.sh (6 Bash guards)
  - demoswarm CLI (tools/demoswarm-runs-tools/)
  - receipt-checker, clarifier, cleanup agents
- endpoints/paths:
  - .claude/agents/\*.md (55 docs, 14 use demoswarm.sh)
  - .claude/commands/flow-\*.md (6 flow commands)
  - .claude/skills/\*/SKILL.md (7 skill docs)
  - CLAUDE.md, docs/reference/contracts.md, docs/reference/stable-markers.md
- code touch points:
  - tools/demoswarm-pack-check/src/checks/drift.rs (REQ-001-003)
  - tools/demoswarm-pack-check/src/checks/control_plane.rs (enum checks)
  - tools/demoswarm-pack-check/src/checks/structure.rs (REQ-002)
  - tools/demoswarm-pack-check/src/contracts.rs (constants)

## Constraints / Non-negotiables

- Status enum: VERIFIED | UNVERIFIED | CANNOT_PROCEED (frozen)
- Action enum: PROCEED | RERUN | BOUNCE | FIX_ENV (frozen)
- Three-Tier Ownership (from issue #49): Flow (routing), Agent (operational), Skill (CLI), CLAUDE.md (TOC)
- Backward compatibility: no breaking changes without migration path
- No new parallel tools; extend pack-check or check-doc-drift.sh
- Archive-over-delete for file modifications
- OQ-SIG-002 (PLN vs PLAN) remains open; assume PLN canonical
- unknowns:
  - Final decision on OpenQ prefix form (PLN vs PLAN)
  - Agents intentionally exempt from Skills section requirement
  - Cross-agent handshake scope (Build-to-Gate only per REQ-004)
  - Test fixtures committed vs dynamically generated

## Evidence (bounded)

### Requirements Summary

Functional: REQ-001 (flow boundary), REQ-002 (Skills section), REQ-003 (OpenQ validation), REQ-004 (test fixtures), REQ-005 (warning-first), REQ-006 (no false positives)

Non-Functional: NFR-PERF-001 (CI runtime <30s), NFR-REL-001 (deterministic output), NFR-OPS-001 (diagnostic clarity), NFR-COMP-001 (backward compat), NFR-SEC-001 (no secrets), NFR-MAINT-001 (maintainable patterns)

### Key Findings from github_research.md

- Issue #8 just created; comprehensive Flow 1 artifacts from PR #7
- Issue #49 (upstream) bounced at Gate; complexity indicator for ownership enforcement
- pack-check has 29 checks; check-doc-drift.sh has 6 guards; existing infrastructure is substantial
- Canonical enum enforcement already works (checks 28-29)
- OpenQ prefix inconsistency documented (OQ-SIG-002)
- 4 agents potentially missing Skills sections
- Three-Tier Ownership Model is authoritative from issue #49
