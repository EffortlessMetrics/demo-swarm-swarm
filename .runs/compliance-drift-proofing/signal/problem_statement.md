# Problem Statement

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - Upstream issue #49 (align-doc-ownership) bounced at Gate; indicates prior implementation challenges
  - OpenQ prefix inconsistency (PLN vs PLAN) remains open (OQ-SIG-002); affects REQ-003 implementation
  - Skills section audit reveals 4 of 14 demoswarm.sh-using agents may be missing required sections
  - Semantic compliance validation (cross-agent logic) is harder to validate mechanically than syntactic rules

confidence: High

## The Problem

The DemoSwarm pack currently lacks comprehensive, mechanical enforcement of its compliance contracts. While foundational infrastructure exists (pack-check validates status/action enums with 29 checks, check-doc-drift.sh catches stale references with 6 guards), significant gaps remain: (1) flow commands can still contain skill-layer CLI details despite the three-tier ownership model, (2) cross-agent handshake validation between flows (e.g., Build receipt to Gate) depends on manual review rather than automated checks, (3) agents using demoswarm.sh may lack required Skills sections, and (4) documentation patterns rely on regexes that must evolve manually as the pack changes.

These gaps mean that drift can occur silently between documentation, contracts, and runtime behavior. When agents produce output that technically parses but violates pack conventions, the error surfaces late (at Gate or later) rather than at authoring time. This increases maintenance burden and reduces confidence in the pack's reliability.

The problem is compounded by the incomplete state of prior alignment work (issue #49 bounced at Gate), suggesting that enforcement mechanisms are technically achievable but implementation is non-trivial. A warning-first approach (REQ-005) is planned to enable incremental adoption without breaking existing CI.

## Who Is Affected

- **Pack maintainers**: Must manually verify contract compliance; lack early feedback when drift occurs
- **Agent authors**: Produce output without immediate validation; discover violations late in the flow
- **Cleanup agents** (signal-cleanup, plan-cleanup, build-cleanup, etc.): Depend on stable markers for counts; drift in marker patterns breaks receipts
- **Flow command maintainers**: No automated guard against accidentally embedding skill plumbing
- **Clarifier agent**: Generates OpenQ IDs with flow prefixes; must emit normalized prefixes per REQ-003
- **Downstream swarm users**: Inherit quality debt when pack contracts are not enforced

## Constraints

- **Canonical status enum**: Must remain `VERIFIED | UNVERIFIED | CANNOT_PROCEED` (frozen, no new values)
- **Canonical action enum**: Must remain `PROCEED | RERUN | BOUNCE | FIX_ENV` (frozen, no new values)
- **Three-tier ownership** (authoritative from issue #49): Flow commands own routing; agent docs own operational detail; skill docs own CLI truth; CLAUDE.md is table of contents only
- **Backward compatibility**: New checks must not break existing valid artifacts without a migration path (NFR-COMP-001)
- **Tooling integration**: New validation should extend pack-check and/or check-doc-drift.sh, not create parallel infrastructure
- **Archive-over-delete**: File changes preserve history rather than removing content
- **pack-check is Rust-based**: Extensions require Rust (drift.rs, control_plane.rs, structure.rs) or must be implemented in check-doc-drift.sh (Bash)
- **CI runtime constraint**: pack-check must complete in under 30 seconds (NFR-PERF-001)

## Non-Goals

- **Creating new enforcement tooling from scratch**: This work extends existing pack-check and check-doc-drift.sh infrastructure
- **Merging pack-check and check-doc-drift.sh into a single tool**: Consolidation is a separate concern; this work adds checks to either tool as appropriate
- **Implementing semantic/behavioral compliance validation**: This work focuses on syntactic/structural checks that can be mechanically verified
- **Resolving upstream issue #49 (align-doc-ownership)**: That issue's Gate bounce is context, not scope; this work complements but does not supersede it
- **Full test fixture coverage for all receipt scenarios**: Test fixtures (REQ-004) focus on Build-to-Gate handshake as the most critical handoff
- **Changing existing OpenQ IDs**: REQ-003 normalizes future generation, not retroactive cleanup

## Success Looks Like

- Flow commands are mechanically verified to not contain demoswarm.sh invocations or skill-layer CLI syntax (REQ-001)
- All agents using demoswarm.sh have a Skills section (or a validation warning surfaces this gap) (REQ-002)
- OpenQ prefixes follow a single normalized pattern (assumed: PLN, BLD, GAT, DEP, WIS per stable-markers.md) and validation catches deviations (REQ-003)
- Build-to-Gate receipt handshake has at least one documented test scenario demonstrating contract compliance (REQ-004)
- New pack-check rules run in warnings-first mode by default; --strict flag enables enforcement (REQ-005)
- No false positives: existing valid agent docs, flow commands, and skill docs continue to pass validation (REQ-006)
- Documentation does not regress: pack-check.md updated with new rules

## Known Context

### Run Identity

- **run_id**: compliance-drift-proofing
- **canonical_key**: gh-8
- **issue_number**: 8
- **issue_url**: https://github.com/EffortlessMetrics/demo-swarm-staging/issues/8
- **issue_state**: OPEN
- **iteration**: 2 (rerun per orchestrator signal, 2025-12-18)
- **prior status**: Signal VERIFIED (2025-12-17), 6 FReq, 6 NFReq, 39 BDD scenarios, 9 open questions

### Relevant Modules/Files

- `tools/demoswarm-pack-check/src/checks/drift.rs` (14 existing drift checks; foundation for REQ-001, REQ-002)
- `tools/demoswarm-pack-check/src/checks/control_plane.rs` (15 control-plane checks; enum enforcement baseline)
- `tools/demoswarm-pack-check/src/checks/structure.rs` (structural checks; can extend for REQ-002)
- `tools/demoswarm-pack-check/src/contracts.rs` (canonical constants; update for REQ-003 OpenQ codes)
- `scripts/check-doc-drift.sh` (6 Bash-based drift guards; possible venue for REQ-003)
- `.claude/agents/receipt-checker.md` (Build-to-Gate handshake validation; source for REQ-004)
- `.claude/agents/clarifier.md` (OpenQ ID generation with flow prefixes; consumer of REQ-003 contract)
- `.claude/skills/openq-tools/SKILL.md` (openq next-id command; authority for OpenQ prefix format)
- `docs/reference/contracts.md` (receipt schemas, Machine Summary contract; authority for all contracts)
- `docs/reference/stable-markers.md` (marker patterns: SIG, PLN, BLD, GAT, DEP, WIS)
- `docs/reference/pack-check.md` (pack-check rule documentation; to be updated with new rules)

### Prior Art / Related Issues

- **Issue #8 (this run)**: "DemoSwarm Compliance Enforcement & Drift-Proofing Analysis" - OPEN, Signal VERIFIED
- **Issue #49 (upstream)**: "Align doc ownership boundaries across pack" - OPEN, Gate BOUNCE; parent work establishing three-tier ownership model
- **PR #7 (merged)**: Flow 1 signal checkpoint with comprehensive artifacts (8661 additions)
- **PR #6 (merged)**: Updated contracts.md with current receipt/Machine Summary schemas
- **Run align-doc-ownership**: Prior run at UNVERIFIED build, Gate BOUNCE; overlapping concerns with this work
- **pack-check checks 28-35**: Already validate status/action enums and routing fields

### Infrastructure Baseline

| Component | Current State | Target REQ |
|-----------|--------------|------------|
| Status enum validation | check 28 | REQ-006 (no false positives) |
| Action enum validation | check 29 | REQ-006 (no false positives) |
| Drift guards (Bash) | 6 guards | Foundation |
| Receipt validation | receipt-checker agent | REQ-004 (test fixtures) |
| OpenQ prefixes | clarifier + openq-tools | REQ-003 (normalization) |
| Skills sections | 10 of 14 users have them | REQ-002 (enforcement) |
| Flow boundaries | NOT enforced | REQ-001 (new check) |
| Warning-first mode | NOT implemented | REQ-005 (--strict flag) |

## Assumptions Made to Proceed

- **ASM-1**: The upstream issue #49 design direction (three-tier ownership) is authoritative and will not be reversed.
  - *If wrong*: Ownership boundary enforcement would need redesign.

- **ASM-2**: pack-check (Rust) is the preferred venue for new structural validation rules.
  - *If wrong*: More checks would go into check-doc-drift.sh instead, affecting implementation approach.

- **ASM-3**: OpenQ prefix normalization should use the abbreviated form (PLN, BLD, GAT, etc.) consistent with stable-markers.md.
  - *If wrong*: Would need to update stable-markers.md and openq-tools/SKILL.md to use long-form (PLAN, BUILD, GATE).

- **ASM-4**: Warning-before-failure is acceptable for new rules to enable incremental adoption.
  - *If wrong*: All new checks would need immediate enforcement, requiring more careful rollout.

- **ASM-5**: The 4 agents using demoswarm.sh without Skills sections are gaps to fix, not intentional exceptions.
  - *If wrong*: Need to document which agents are intentionally exempt.

- **ASM-6**: Prior Flow 1 research and requirements are correct and do not need major revision.
  - *If wrong*: Iteration 2 would surface new requirements or constraints not captured in iteration 1.

## Questions / Clarifications Needed

- Q: Should new pack-check rules be warnings first or immediate failures? Suggested default: warnings first (--strict flag for failures).

- Q: Is the PLN vs PLAN discrepancy intentional (abbreviation for marker brevity) or a documentation error? Suggested default: PLN is canonical (matches stable-markers.md). Open question: OQ-SIG-002.

- Q: Should cross-agent handshake validation extend beyond Build-to-Gate (e.g., Signal-to-Plan, Plan-to-Build)? Suggested default: No, focus on Build-to-Gate as the most critical handoff.

- Q: Which agents using demoswarm.sh are intentionally exempt from Skills section requirements (if any)? Suggested default: None; all 14 should have Skills sections.

- Q: Should flow boundary enforcement be a pack-check rule (Rust) or check-doc-drift.sh rule (Bash)? Suggested default: pack-check (consistent with other structural validation).

- Q: Should test fixtures (REQ-004) be committed to the repo or generated dynamically during CI? Suggested default: Committed to `tools/demoswarm-pack-check/tests/fixtures/` for stability and reviewability.
