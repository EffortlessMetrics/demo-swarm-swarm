# Context Brief (Iteration 2 Updated)

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
blockers: []
missing_required: []
notes:

- Iteration 2 rerun context (2025-12-18): GitHub issue #8 now bound
- Prior artifacts from iteration 1 (2025-12-17) remain VERIFIED
- Keywords searched: pack-check, check-doc-drift, enum, receipt-checker, openq, Skills
- Exclusions: .runs/, .git/
- Substantial existing infrastructure found; upstream work at #49

## Run Identity Context

- run_id_kind: GH_ISSUE
- issue_binding: IMMEDIATE (updated from DEFERRED in iteration 1)
- canonical_key: gh-8
- github_ops_allowed: true
- repo_expected: EffortlessMetrics/demo-swarm-staging
- repo_actual_at_creation: EffortlessMetrics/demo-swarm-staging

## Related Runs

- align-doc-ownership: Addresses ownership boundaries, enum consistency, flow boundaries
  - Status: last_flow=gate, verdict=BOUNCE (complexity indicator)
  - Path: .runs/align-doc-ownership/signal/issue_normalized.md

## Likely Code Touch Points

### Pack-Check Validation (Rust)

- tools/demoswarm-pack-check/src/checks/drift.rs (14 checks, foundation for REQ-001-003)
- tools/demoswarm-pack-check/src/checks/control_plane.rs (enum checks 28-29, baseline)
- tools/demoswarm-pack-check/src/checks/structure.rs (file checks, REQ-002 extension)
- tools/demoswarm-pack-check/src/contracts.rs (constants, REQ-003 OpenQ codes)

### Drift Detection (Bash)

- scripts/check-doc-drift.sh (6 guards: stale skills, old flags, marker drift)

### OpenQ Infrastructure

- .claude/agents/clarifier.md (ID generation with flow prefixes)
- .claude/skills/openq-tools/SKILL.md (openq next-id, append)
- tools/demoswarm-runs-tools/src/commands/openq.rs (Rust impl)

### Receipt Validation

- .claude/agents/receipt-checker.md (Build receipt validation, REQ-004 source)

### Skills Enforcement Targets

- 10 agents have Skills sections (confirmed)
- 14 agents use demoswarm.sh (4 potentially missing Skills)

### Flow Commands

- .claude/commands/flow-\*.md (6 files, REQ-001 enforcement target)

## Prior Art / Docs

- .runs/align-doc-ownership/ (Issue #49, Gate bounce reference)
- docs/reference/contracts.md (AUTHORITY: receipt/Machine Summary schemas)
- docs/reference/stable-markers.md (OpenQ codes: SIG, PLN, BLD, GAT, DEP, WIS)
- CLAUDE.md (pack policy, canonical enums, three-tier ownership)

## Infrastructure Baseline

| Component          | Current           | REQ Target |
| ------------------ | ----------------- | ---------- |
| Status enum        | check 28          | REQ-006    |
| Action enum        | check 29          | REQ-006    |
| Drift (Bash)       | 6 guards          | Foundation |
| Receipt validation | receipt-checker   | REQ-004    |
| OpenQ prefixes     | clarifier + tools | REQ-003    |
| Skills sections    | 10/14 users       | REQ-002    |
| Flow boundaries    | NOT enforced      | REQ-001    |
| Warning-first      | NOT implemented   | REQ-005    |

## Key Risks

- RSK-001 (HIGH): Issue #49 Gate bounce indicates complexity in ownership enforcement
  - Mitigation: REQ-005 warning-first approach
- RSK-002 (MEDIUM): OpenQ PLN vs PLAN unresolved (OQ-SIG-002)
  - Mitigation: Escalate to Flow 2
- RSK-003 (MEDIUM): 4 agents potentially missing Skills sections
  - Mitigation: REQ-002 surfaces gaps
- RSK-004 (MEDIUM): Regex maintenance burden
  - Mitigation: NFR-MAINT-001 constants approach
- RSK-005 (MEDIUM): Flow boundary leakage historical
  - Mitigation: REQ-001 mechanical detection

## Critical Decisions

1. Three-Tier Ownership (AUTHORITATIVE from #49): Flow (routing), Agent (operational), Skill (CLI)
2. Canonical Enums (FROZEN): VERIFIED|UNVERIFIED|CANNOT_PROCEED, PROCEED|RERUN|BOUNCE|FIX_ENV
3. OpenQ Prefix (OPEN): PLN vs PLAN per OQ-SIG-002
4. Warning-First (PLANNED): REQ-005 with --strict flag
5. Tool Separation: pack-check and check-doc-drift.sh remain distinct

## Inventory (machine countable)

- ISSUE: #8 (this run) Critical/open
- ISSUE: #49 (upstream) High/open
- PR: #7 merged (Signal, 8661 additions)
- PR: #6 merged (contracts.md)
- PR: #5 open (docs)
- PR: #1 merged (artifacts)
- CODE_REF: drift.rs (14 checks)
- CODE_REF: control_plane.rs (15 checks)
- CODE_REF: structure.rs
- CODE_REF: contracts.rs
- CODE_REF: check-doc-drift.sh (6 guards)
- CODE_REF: receipt-checker.md
- CODE_REF: clarifier.md
- CODE_REF: openq-tools/SKILL.md
- CODE_REF: docs/reference/contracts.md
- CODE_REF: docs/reference/stable-markers.md
