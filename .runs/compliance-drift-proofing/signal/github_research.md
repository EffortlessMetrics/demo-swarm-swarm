# GitHub Research for compliance-drift-proofing

## Search Inputs

Terms derived from `run_meta.json` and orchestrator signal:

- **Canonical key**: `gh-8` (GitHub issue just created)
- **Issue number**: 8 (DemoSwarm Compliance Enforcement & Drift-Proofing Analysis)
- **Aliases**: `compliance-drift-proofing`, `gh-8`
- **Keywords searched**: `pack-check`, `drift`, `compliance`, `enforcement`, `alignment`, `ownership`, `check-doc-drift`
- **Repository scope**: `EffortlessMetrics/demo-swarm-staging` (primary)

## Access & Limitations

- **gh CLI**: Available and authenticated (account: EffortlessSteven)
- **Token scopes**: `admin:public_key`, `gist`, `read:org`, `repo` (sufficient)
- **Rate limits**: Not encountered during research
- **Discussions**: Not accessed (not critical for this work type)
- **Upstream repo**: `EffortlessMetrics/demo-swarm-dev` accessible; issue #49 context available

## Related Issues

| # | Title | State | Repo | Relevance |
|---|-------|-------|------|-----------|
| 8 | DemoSwarm Compliance Enforcement & Drift-Proofing Analysis | OPEN | demo-swarm-staging | **Critical (this run)** |
| 49 | Align doc ownership boundaries across pack | OPEN | demo-swarm-dev | **High (parent work)** |

### Issue Details

**Issue #8 (staging, this run)** - "DemoSwarm Compliance Enforcement & Drift-Proofing Analysis"

- **Status**: Just created (2025-12-17, Signal=VERIFIED per receipt)
- **What it addresses**: Implement comprehensive mechanical enforcement of DemoSwarm compliance contracts to prevent silent drift between documentation, contracts, and runtime behavior
- **Key gaps being addressed**:
  1. Flow commands can contain skill-layer CLI details despite three-tier ownership model
  2. Cross-agent handshake validation (Build receipt to Gate) depends on manual review
  3. Agents using demoswarm.sh may lack required Skills sections
  4. Documentation patterns rely on regexes that must evolve manually
- **Why it matters**: Drift occurs silently; errors surface late (at Gate or later) rather than at authoring time, increasing maintenance burden
- **Current status**: Signal=VERIFIED, requirements/features documented, ready for Flow 1 re-run per orchestrator signal

**Issue #49 (upstream: demo-swarm-dev)** - "Align doc ownership boundaries across pack"

- **Status**: OPEN, prior work that bounced at Gate (signal=VERIFIED, plan=VERIFIED, build=UNVERIFIED, gate=BOUNCE)
- **What it does**: Normalizes language and ownership boundaries across the DemoSwarm pack's three documentation tiers (flow commands, agent docs, skill docs)
- **Decision/constraint established**:
  - Flow commands own orchestration and routing (no skill plumbing)
  - Agent docs own operational detail (inputs/outputs, Machine Summary contract)
  - Skill docs own CLI truth (command syntax, flags, examples)
  - CLAUDE.md is a table of contents only
- **Why it matters**: This is the **parent work item** for compliance enforcement (issue #8 extends and strengthens enforcement mechanisms from #49)
- **Gate bounce reason**: Implementation challenges (likely related to flow command validation complexity); issue #8 incorporates learnings from this bounce

## Related PRs

| # | Title | State | Repo | Relevance |
|---|-------|-------|------|-----------|
| 7 | signal: checkpoint compliance-drift-proofing Flow 1 | MERGED | demo-swarm-staging | **Critical (this run)** |
| 6 | docs: update contracts reference | MERGED | demo-swarm-staging | **High** |
| 5 | Align docs with repo and Diátaxis | OPEN | demo-swarm-staging | **Medium** |
| 1 | flow(signal): checkpoint align-doc-ownership artifacts | MERGED | demo-swarm-staging | **Medium** |

### PR Details

**PR #7 (staging)** - "signal: checkpoint compliance-drift-proofing Flow 1"

- **Status**: MERGED (2025-12-18)
- **Scope**: 8661 additions, 2230 deletions (large Signal-phase commit)
- **Contents**: Flow 1 artifacts (requirements.md, features, risk_assessment.md, problem_statement.md, open_questions.md, etc.)
- **Reviewers**: copilot-pull-request-reviewer, gemini-code-assist (commented)
- **Relevance**: Contains all Signal-phase output for this run; represents the current problem formulation and constraints

**PR #6 (staging)** - "docs: update contracts reference"

- **Status**: MERGED
- **Scope**: Updated `docs/reference/contracts.md` (1081 additions, 709 deletions)
- **What it defines**: Baseline contract schema for:
  - Receipt JSON structures (signal, plan, build, gate, deploy, wisdom)
  - Machine Summary control-plane block (status, recommended_action, route_to_flow, route_to_agent, blockers, etc.)
  - Stable markers for mechanical counting (ISSUE, PR, DISCUSSION, CODE_REF prefixes)
  - Gate Result and Repo Operator Result canonical blocks
- **Relevance**: Defines the contracts that new enforcement rules must validate against; establishes the "source of truth" for compliance checks

**PR #5 (staging)** - "Align docs with repo and Diátaxis"

- **Status**: OPEN
- **Scope**: 181 additions, 980 deletions (mostly cleanup)
- **What it does**: Consolidates documentation following Diataxis framework; removes stubs; updates docs/README.md index
- **Key changes**: Tutorials consolidated (quickstart, walkthrough, validation-run); removed first-run.md, demo-run.md, validation.md, toy-run.md
- **Relevance**: May affect documentation structure that drift checks reference; provides cleaner baseline for enforcement rules

**PR #1 (staging)** - "flow(signal): checkpoint align-doc-ownership artifacts"

- **Status**: MERGED
- **Scope**: 11102 additions (large artifact set from issue #49 work)
- **Relevance**: Contains prior art for signal/plan/build/gate flow artifacts related to ownership boundaries; documents issues that led to Gate bounce

## Related Discussions

No GitHub discussions accessed (not critical for this compliance/engineering work type).

## Decisions / Constraints Extracted

From the research and related artifacts, the following decisions and constraints are relevant:

### 1. Three-Tier Ownership Model (from issue #49, authoritative)

- **Flow commands**: Routing and orchestration only; must not contain skill-layer CLI syntax or demoswarm.sh invocations
- **Agent docs**: Operational detail (inputs, outputs, control-plane contracts, routing decisions)
- **Skill docs**: CLI truth (command syntax, flags, examples, subcommands)
- **CLAUDE.md**: Quick-start and table of contents only

**Implication**: Enforcement must mechanically validate this boundary (REQ-001, REQ-002, REQ-003).

### 2. Canonical Enum Enforcement (from contracts.md, pack-check control_plane.rs)

Already enforced by existing pack-check checks:
- **Status axis**: `VERIFIED | UNVERIFIED | CANNOT_PROCEED` (check 28)
- **Recommended action**: `PROCEED | RERUN | BOUNCE | FIX_ENV` (check 29)
- **Route fields**: `route_to_flow: 1|2|3|4|5|6|null`, `route_to_agent: <name>|null` (checks 31-35)

**Implication**: New rules must not introduce new enum values; extend existing validation infrastructure.

### 3. OpenQ Prefix Contract (from openq-tools/SKILL.md, stable-markers.md)

Current state has **documented inconsistency** (OQ-SIG-002 open):
- **Canonical format**: `OQ-<FLOW>-<NNN>` (e.g., `OQ-SIG-001`, `OQ-BUILD-042`)
- **Flow codes (abbreviated)**: `SIG`, `PLN`, `BLD`, `GAT`, `DEP`, `WIS` (per stable-markers.md)
- **Flow codes (alternate)**: Some docs say `PLAN`, `BUILD`, `GATE` (full names)
- **Requirement**: REQ-003 assumes PLN is canonical; open question OQ-SIG-002 asks if this is intentional

**Implication**: REQ-003 validation must normalize to PLN/BLD/GAT/DEP; documentation inconsistency must be resolved.

### 4. Pack-Check Validation Scope (from drift.rs, control_plane.rs)

**Existing checks** (14 drift checks + 15 control-plane checks):
- Status/action enum validation (checks 3, 4, 16-35)
- Banned pattern detection (checks 7, 8, 14, 23, 30, 38-49)
- Shim enforcement (checks 45, 47, 48)
- Skill ownership (check 46)
- Flow boundary enforcement (check 40: `See CLAUDE.md` substitution check)

**Gaps identified** (to be addressed by REQ-001 through REQ-006):
- No explicit validation that flow commands don't contain demoswarm.sh
- No validation that agents using demoswarm.sh have Skills sections
- No validation of OpenQ prefix patterns in open_questions.md files
- No test fixtures for Build-to-Gate receipt handshake

### 5. Check-Doc-Drift.sh Scope (from script inspection)

**Existing Bash-based checks** (6 checks):
1. Stale runs tooling reference (skills were split; update to current skill names)
2. Legacy OpenQ CLI usage (update to the current OpenQ tool interface)
3. Legacy secrets CLI usage (update to the current secrets tool interface)
4. yaml count-items interface drift (update to current matching flag)
5. inv get interface drift (update to current marker-based interface)
6. (Check 6 implied by context)

**Implication**: check-doc-drift.sh handles interface drift; pack-check handles structural/ownership drift.

### 6. Receipt Validation (from receipt-checker.md)

- Build receipt is validated for JSON parse, contract fields, test grounding
- Cross-checks exist but could be strengthened (REQ-004)

**Implication**: Test fixtures needed to document Build-to-Gate handshake contract.

### 7. Warning-First Approach (from problem_statement.md, requirements.md)

- New rules should produce warnings by default (not hard failures)
- `--strict` flag can elevate warnings to errors
- This allows incremental adoption without breaking existing CI (NFR-COMP-001)

**Implication**: REQ-005 defines warning-first mode with --strict override.

## Prior Art Pointers (Local Codebase)

| Path | Note | Key Insight |
|------|------|------------|
| `tools/demoswarm-pack-check/src/checks/drift.rs` | 14 existing drift checks (IDs 7,8,14,23,30,38-49) | Foundation for REQ-001 flow boundary check and REQ-002 skills validation |
| `tools/demoswarm-pack-check/src/checks/control_plane.rs` | 15 control-plane checks (IDs 3,4,16-35) | Machine Summary and canonical enum validation (already working) |
| `tools/demoswarm-pack-check/src/checks/structure.rs` | Structural checks for file presence, format | Can be extended for Skills section validation (REQ-002) |
| `tools/demoswarm-pack-check/src/contracts.rs` | Canonical enums, required agents, skill ownership | Update required for OpenQ flow codes constant (REQ-003) |
| `scripts/check-doc-drift.sh` | 6 Bash-based doc drift guards | Possible venue for REQ-003 if not handled in pack-check |
| `.claude/agents/receipt-checker.md` | Build receipt validation agent | Source for understanding Build-to-Gate contract (REQ-004 documentation) |
| `.claude/agents/clarifier.md` | OpenQ ID generation with flow prefixes | Consumers of OpenQ prefix contract; must align with REQ-003 |
| `.claude/skills/openq-tools/SKILL.md` | openq next-id, append commands | Authority on OpenQ prefix format; must be canonical reference |
| `docs/reference/contracts.md` | Receipt schemas, Machine Summary contract, stable markers | Authority on all contracts; REQ-004 test fixtures should reference this |
| `docs/reference/stable-markers.md` | Marker patterns for counting (ISSUE, PR, DISCUSSION, CODE_REF) | Defines OpenQ flow code abbreviations (SIG, PLN, BLD, etc.) |
| `docs/reference/pack-check.md` | pack-check rule documentation | Should be updated with new rules (REQ-001 through REQ-005) |
| `.runs/gh-8/signal/` | Current run artifacts | Problem statement, requirements, risk assessment, open_questions.md |

## Implications for Flow 1 (Signal Phase Re-Run)

### Constraints Requirements Must Respect

1. **Three-Tier Ownership Inviolable**: No flow command files may invoke demoswarm.sh or use skill CLI syntax
2. **Backward Compatibility Non-Negotiable**: New validation rules must not break existing valid artifacts without clear migration path (per NFR-COMP-001)
3. **Enum Contract Frozen**: No new Machine Summary status/action values; must use: `{VERIFIED, UNVERIFIED, CANNOT_PROCEED}` and `{PROCEED, RERUN, BOUNCE, FIX_ENV}`
4. **Existing pack-check Integration**: New checks must extend drift.rs, control_plane.rs, or structure.rs; do not create parallel tooling
5. **OpenQ Prefix Consistency Required**: Decision needed on PLN vs PLAN before implementation (OQ-SIG-002)
6. **Receipt Contracts Authoritative**: docs/reference/contracts.md is the single source of truth for receipt/Machine Summary schemas

### Risks from Prior Attempts

1. **Issue #49 Gate BOUNCE**: Prior alignment work bounced at Gate, suggesting implementation complexity:
   - Possible root causes: flow boundary enforcement harder than expected, side effects on existing artifacts, stakeholder consensus on ownership model needed
   - Mitigation: This work uses warning-first approach (REQ-005) to avoid immediate failures; includes documented test scenarios (REQ-004)

2. **OpenQ Prefix Inconsistency**: Current docs use both `PLAN` and `PLN`; creates confusion in marker patterns
   - Mitigation: OQ-SIG-002 is an open question; REQ-003 implementation depends on resolution

3. **Flow Boundary Leakage Historical**: Skill plumbing has crept into flows in past commits
   - Mitigation: REQ-001 adds mechanical detection; pack-check failure on CI will prevent regression

4. **Skills Section Gaps**: Some agents using demoswarm.sh may lack Skills sections (4 potential gaps noted in problem_statement.md)
   - Mitigation: REQ-002 surfaces the gaps; remediation path documented in problem_statement.md

### Stakeholders Hinted by Prior Issues/PRs

- **Pack maintainers** (via pack-check rules): Must validate compliance; will benefit from early feedback
- **Agent authors** (across all flows): Will need to add/update Skills sections (if missing per REQ-002)
- **Cleanup agents** (signal-cleanup, plan-cleanup, build-cleanup, etc.): Depend on stable markers for counts; drift in marker patterns breaks receipts (REQ-003 prevents this)
- **Flow command maintainers**: Will need to validate no skill plumbing (REQ-001)
- **Clarifier agent** (OpenQ ID generation): Will need to emit normalized prefixes (PLN, BLD, etc. per REQ-003)
- **Downstream swarm users**: Will inherit pack quality improvements

### Do Not Repeat Landmines

1. **No backward-incompatible validation**: All new rules must have warnings-first mode; existing artifacts must pass without remediation
2. **No CLI reference duplication**: Flow commands must not repeat skill command syntax; REQ-001 prevents this
3. **No enum invention**: Must use canonical sets only (VERIFIED, UNVERIFIED, CANNOT_PROCEED; PROCEED, RERUN, BOUNCE, FIX_ENV)
4. **No marker pattern changes without producer/consumer updates**: REQ-003 assumes harmonization of PLN vs PLAN; requires decisions before implementation
5. **No test fixtures with real secrets**: REQ-004 test fixtures must use dummy/sanitized data only (NFR-SEC-001)

## Assumptions Made to Proceed

1. **ASM-1**: The upstream issue #49 design direction (three-tier ownership) is authoritative and will not be reversed
   - *Consequence*: Ownership boundary enforcement is in-scope and foundational

2. **ASM-2**: pack-check (Rust) is the preferred venue for new structural validation rules
   - *Consequence*: REQ-001, REQ-002, REQ-003 will be implemented in drift.rs and structure.rs, not check-doc-drift.sh (unless performance/maintainability analysis suggests otherwise)

3. **ASM-3**: OpenQ prefix normalization should use the abbreviated form (PLN, BLD, GAT, etc.) consistent with stable-markers.md
   - *Consequence*: REQ-003 enforces PLN (not PLAN); open question OQ-SIG-002 assumes this resolution

4. **ASM-4**: Warning-before-failure is acceptable for new rules to enable incremental adoption
   - *Consequence*: REQ-005 defines `--strict` flag for opt-in enforcement; CI can run without --strict during rollout period

5. **ASM-5**: The 4 agents potentially missing Skills sections are gaps to fix, not intentional exceptions
   - *Consequence*: REQ-002 will surface them; remediation path is to add Skills sections to each

6. **ASM-6**: Prior Flow 1 research and requirements are correct and will not need major revision
   - *Consequence*: Flow 1 re-run per orchestrator signal will update artifacts but not fundamentally change scope

## Questions / Clarifications Needed

1. **Q: Should new pack-check rules be warnings first or immediate failures?**
   - **Current assumption** (per REQ-005): Warnings first; `--strict` flag enables enforcement
   - **Suggest validation**: Confirm with pack maintainers and CI team

2. **Q: Is the PLN vs PLAN discrepancy intentional (abbreviation for marker brevity) or a documentation error?**
   - **Current assumption** (per REQ-003): PLN is canonical (matches stable-markers.md abbreviations: SIG, PLN, BLD, GAT, DEP, WIS)
   - **Open question**: OQ-SIG-002; should be resolved before Flow 2 (Plan phase)

3. **Q: Should cross-agent handshake validation extend beyond Build-to-Gate (e.g., Signal-to-Plan, Plan-to-Build)?**
   - **Current assumption** (per REQ-004): No; focus on Build-to-Gate as the most critical handoff
   - **Suggest validation**: Confirm scope with Flow 2 planning team

4. **Q: Which agents using demoswarm.sh are intentionally exempt from Skills section requirements (if any)?**
   - **Current assumption** (per REQ-002): None; all agents using demoswarm.sh should have Skills sections
   - **Suggest enumeration**: Confirm list from problem_statement.md context_brief.md before implementation

5. **Q: Should flow boundary enforcement be a pack-check rule (Rust) or check-doc-drift.sh rule (Bash)?**
   - **Current assumption** (per REQ-001): pack-check rule (consistent with other structural validation)
   - **Suggest validation**: Confirm with Rust/Bash maintainers; performance impact analysis if Bash chosen

6. **Q: Should test fixtures (REQ-004) be committed to the repo or generated dynamically during CI?**
   - **Current assumption**: Committed to `tools/demoswarm-pack-check/tests/fixtures/` for stability and reviewability
   - **Suggest validation**: Confirm storage location with pack maintainers

## Inventory (machine countable)

- ISSUE: #8 relevance=Critical state=open
- ISSUE: #49 relevance=High state=open
- PR: #7 relevance=Critical state=merged
- PR: #6 relevance=High state=merged
- PR: #5 relevance=Medium state=open
- PR: #1 relevance=Medium state=merged
- CODE_REF: tools/demoswarm-pack-check/src/checks/drift.rs note=14 existing drift checks (foundation for REQ-001, REQ-002)
- CODE_REF: tools/demoswarm-pack-check/src/checks/control_plane.rs note=15 control-plane checks (Machine Summary, enum validation)
- CODE_REF: tools/demoswarm-pack-check/src/checks/structure.rs note=Structural checks (can extend for REQ-002)
- CODE_REF: tools/demoswarm-pack-check/src/contracts.rs note=Canonical enums and constants (update for REQ-003)
- CODE_REF: scripts/check-doc-drift.sh note=6 Bash-based drift guards (possible venue for REQ-003)
- CODE_REF: .claude/agents/receipt-checker.md note=Build receipt validation (source for REQ-004 documentation)
- CODE_REF: .claude/agents/clarifier.md note=OpenQ ID generation (consumer of REQ-003 contract)
- CODE_REF: .claude/skills/openq-tools/SKILL.md note=OpenQ prefix specification (authority for REQ-003)
- CODE_REF: docs/reference/contracts.md note=Receipt/Machine Summary schemas (authority for all contracts)
- CODE_REF: docs/reference/stable-markers.md note=Marker patterns including OpenQ flow codes
- CODE_REF: docs/reference/pack-check.md note=pack-check rule documentation (to be updated with new rules)
- CODE_REF: .runs/gh-8/signal/ note=Current run artifacts (problem_statement.md, requirements.md, risk_assessment.md)

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: null

blockers: []
missing_required: []

concerns:
  - "Upstream issue #49 Gate bounced; indicates implementation complexity in ownership boundary enforcement"
  - "OpenQ prefix inconsistency (PLN vs PLAN) is documented (OQ-SIG-002) but unresolved; blocks REQ-003 implementation"
  - "4 agents potentially missing Skills sections (per problem_statement.md); enumeration assumed, should be validated"
  - "Prior Flow 1 artifacts exist and are comprehensive; re-run per orchestrator signal will update but likely not change scope"
```

## Summary

Issue #8 represents a critical compliance enforcement initiative that extends prior work from issue #49. The research confirms:

1. **GitHub context is well-established**: Issue #8 is properly created, with comprehensive Signal-phase artifacts from PR #7; upstream issue #49 provides parent design direction
2. **Prior art is substantial**: pack-check infrastructure includes 29 existing checks; check-doc-drift.sh provides 6 Bash guards; receipt validation and OpenQ tools are in place
3. **Requirements are detailed and testable**: 6 functional requirements (REQ-001 through REQ-006) and 6 non-functional requirements (NFR-PERF-001, NFR-REL-001, NFR-OPS-001, NFR-COMP-001, NFR-SEC-001, NFR-MAINT-001) are documented with acceptance criteria
4. **Key decisions are documented**: Three-tier ownership model is authoritative; warning-first approach is planned; OpenQ prefix normalization is open (OQ-SIG-002)
5. **Risks are identified**: Gate bounce from issue #49 suggests implementation complexity; OpenQ prefix inconsistency must be resolved before implementation
6. **Stakeholders are identifiable**: Pack maintainers, agent authors, cleanup agents, flow command maintainers, clarifier agent, and downstream users will be impacted

**Recommendation**: Proceed with Flow 1 re-run per orchestrator signal. Update problem_statement.md and requirements.md to reflect GitHub context (issue #8 is now created and linked). Escalate OQ-SIG-002 (PLN vs PLAN) to Flow 2 (Plan phase) for explicit decision before design begins.
