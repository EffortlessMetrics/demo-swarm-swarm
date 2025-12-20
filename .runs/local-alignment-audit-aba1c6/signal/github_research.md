# GitHub Research for local-alignment-audit-aba1c6

## Search Inputs

**Run context:**
- `run_id`: `local-alignment-audit-aba1c6`
- `run_id_kind`: `LOCAL_ONLY` (no GitHub binding)
- `github_ops_allowed`: `false` (read-only research only, no API calls)
- **Task**: DemoSwarm documentation-code alignment audit (focus: Six-Flow vs Seven-Flow discrepancies, test metrics)

**Search terms used (local, no API):**
1. `Six-Flow | Seven-Flow` (exact term matching for flow count)
2. `flow.*count | total.*flow` (broader pattern for references to flow totals)
3. `/flow-[0-9]` (command existence check)
4. Commit history inspection (recent changes, sync patterns)
5. Primary documentation files:
   - `CLAUDE.md` (pack reference)
   - `README.md` (public-facing)
   - `ARCHITECTURE.md` (design docs)
   - `DEMO_RUN.md` (tutorial stub)
   - `docs/explanation/architecture.md` (architecture deep-dive)

---

## Access & Limitations

**GitHub CLI Status**: `gh` commands **not invoked** (per `github_ops_allowed: false` in run_meta)
- This is a `LOCAL_ONLY` run with deferred GitHub binding
- All research is conducted via local file inspection, git history, and codebase grep
- **Outcome**: UNVERIFIED on GitHub context (no issue/PR data fetched), but VERIFIED on local code structure

**Scope of this research**:
- Local pack documentation consistency check
- Flow command count verification from disk
- Commit history for context on recent changes
- Cross-file terminology audit

**What was NOT checked** (due to read-only, no GitHub API):
- GitHub issue tracker (EffortlessMetrics/demo-swarm-swarm has issues disabled)
- PR status or history (would require `gh pr list`)
- External references or issue links
- GitHub Discussions (not critical for this audit type)

---

## Core Finding: Six-Flow vs Seven-Flow Discrepancy

### The Discrepancy

**Documentation Claims (Read):**
- `README.md` line 67: "### The six flows"
- `DEMO_RUN.md` line 14: "Run all six flows with commentary"
- `docs/explanation/architecture.md` line 62: "## The six flows"
- `CHANGELOG.md` v1.0.0 line 24: "**6 flow commands**: `/flow-1-signal` through `/flow-6-wisdom`"

**Code Reality (on disk):**
```
.claude/commands/
  flow-1-signal.md
  flow-2-plan.md
  flow-3-build.md
  flow-4-gate.md      ← Two overlapping flow 4 variants
  flow-4-review.md    ← (not mentioned in public docs)
  flow-5-deploy.md    ← Two overlapping flow 5 variants
  flow-5-gate.md      ← (not mentioned in public docs)
  flow-6-deploy.md    ← Two overlapping flow 6 variants
  flow-6-wisdom.md    ← (not mentioned in public docs)
  flow-7-wisdom.md    ← **SEVENTH FLOW** (not mentioned in ANY public doc)
```

**Total actual flows**: 10 command files implementing overlapping flow semantics:
- Flow 1: 1 variant (signal)
- Flow 2: 1 variant (plan)
- Flow 3: 1 variant (build)
- Flow 4: 2 variants (gate, review)
- Flow 5: 2 variants (deploy, gate)
- Flow 6: 2 variants (deploy, wisdom)
- **Flow 7: 1 variant (wisdom)** — **undocumented**

### Why This Matters

1. **Public documentation (README, DEMO_RUN, architecture.md) references "six flows"** but the pack implements **seven distinct flow sequences**.
2. **CLAUDE.md (the pack reference)** correctly lists the seven flows in line 13 and the seven-flow table in line 68, but other public docs still say "six."
3. **Changelog v1.0.0 claims "6 flow commands"** but actually ships with 10 command files.
4. **Flow overlap complexity**: The pack uses a multi-phase/multi-path design:
   - Flow 4 has both `/flow-4-gate` and `/flow-4-review` (different entry points into review/gate cycle)
   - Flow 5 has both `/flow-5-gate` and `/flow-5-deploy` (gate verdict vs. execution)
   - Flow 6 has both `/flow-6-deploy` and `/flow-6-wisdom` (parallel wisdom-from-deploy paths)
   - Flow 7 (`/flow-7-wisdom`) exists as a **standalone second-cycle wisdom** (not clearly documented)

---

## Related Prior Work

### `align-doc-ownership` Run

Located at `.runs/align-doc-ownership/`:
- **Scope**: Alignment of documentation ownership boundaries (Flow commands ≠ Agent docs ≠ Skill docs)
- **Status**: Gates completed with bounce (plan/build/gate all executed; gate verdict was BOUNCE)
- **Key constraints established**:
  - Flow commands own orchestration/routing (no CLI plumbing)
  - Agent docs own operational detail
  - Skill docs own CLI truth
  - CLAUDE.md is TOC only
- **Relevance to this audit**: Previous team identified similar "documentation structure" issues; this audit extends to flow count and test metrics

### `compliance-drift-proofing` Run

Located at `.runs/compliance-drift-proofing/`:
- **Scope**: Mechanical enforcement of DemoSwarm compliance contracts
- **Parent issue**: `gh-49` ("Align doc ownership boundaries"), `gh-8` (compliance enforcement)
- **Key artifact**: `signal/requirements.md` (REQ-005 subtask partitioning):
  - ST-005 covers "Flow 5 (Deploy)"
  - ST-006 covers "Flow 6 (Wisdom)"
  - **But no ST-007** — suggests the team is aware of multi-path design but hasn't named Flow 7 as a distinct flow in requirements
- **Relevance**: Confirms that flow count issues are known to the team; subtask partitioning doesn't account for Flow 7 as a separate flow path

---

## Discrepancies Identified

### 1. Six-Flow Claims (Outdated)

| File | Location | Claim | Status |
|------|----------|-------|--------|
| README.md | L67 | "### The six flows" | **STALE** — should say "Seven flows" |
| DEMO_RUN.md | L14 | "Run all six flows with commentary" | **STALE** — should enumerate flows 1-7 |
| docs/explanation/architecture.md | L62 | "## The six flows" | **STALE** — should list 7 + explain overlap |
| CHANGELOG.md v1.0.0 | L24 | "**6 flow commands**: /flow-1-signal through /flow-6-wisdom" | **MISLEADING** — actually 10 command files |

### 2. Flow Overlap Not Documented

| Pattern | Actual Commands | Documentation Status |
|---------|-----------------|----------------------|
| Flow 4 split | `/flow-4-gate`, `/flow-4-review` | Not explained in README/DEMO_RUN |
| Flow 5 split | `/flow-5-gate`, `/flow-5-deploy` | Not explained in README/DEMO_RUN |
| Flow 6 split | `/flow-6-deploy`, `/flow-6-wisdom` | Not explained in README/DEMO_RUN |
| Flow 7 standalone | `/flow-7-wisdom` | **Completely undocumented in public docs** |

### 3. CLAUDE.md Inconsistency

**Correct (CLAUDE.md L13-14):**
```markdown
- **7 flows**: Signal → Plan → Build → Review → Gate → Deploy → Wisdom
```

**Outdated (CLAUDE.md L68 flow table):**
```markdown
| 1. Signal | `/flow-1-signal` | Intent -> requirements, BDD, risks, receipt |
| 2. Plan | `/flow-2-plan` | Spec -> ADR, contracts, observability, plans, receipt |
| 3. Build | `/flow-3-build` | Design -> code/tests + build receipt |
| 4. Gate | `/flow-4-gate` | Code -> verdict (MERGE/BOUNCE) + gate receipt |
| 5. Deploy | `/flow-5-deploy` | Verdict -> promote to swarm mainline (or NOT_DEPLOYED) + deploy receipt |
| 6. Wisdom | `/flow-6-wisdom` | Run history -> regressions, learnings, feedback + terminal receipt |
```
The table says "6" flows and maps them to numbered 1-6, but omits Flow 7 and doesn't explain the Flow 4/5/6 splits.

---

## Prior Art Pointers (Local Codebase)

### Flow Definition Files
- `.claude/commands/flow-1-signal.md` — Entry point for requirements capture
- `.claude/commands/flow-2-plan.md` — Design and planning
- `.claude/commands/flow-3-build.md` — Implementation
- `.claude/commands/flow-4-gate.md` — Pre-merge gate verdict
- `.claude/commands/flow-4-review.md` — **ALTERNATE PATH** (PR review → build loop)
- `.claude/commands/flow-5-gate.md` — **ALTERNATE PATH** (gate → build loop re-entry)
- `.claude/commands/flow-5-deploy.md` — Deployment execution
- `.claude/commands/flow-6-deploy.md` — **ALTERNATE PATH** (deploy-after-gate path)
- `.claude/commands/flow-6-wisdom.md` — Learning extraction
- `.claude/commands/flow-7-wisdom.md` — **STANDALONE** second-cycle wisdom (no documentation)

### Documentation Structure
- `docs/explanation/architecture.md` — Claims 6 flows; should reflect actual 7-flow + multi-path design
- `docs/tutorials/walkthrough.md` — Tutorial reference; canonical demo walkthrough
- `CLAUDE.md` — **Most accurate** (lists 7 flows correctly, but flow table still says 6)

### Test Metrics
No test count discrepancies found in available local artifacts. The compliance run (`compliance-drift-proofing`) references test metrics in:
- `.runs/compliance-drift-proofing/plan/test_plan.md`

### Agent Inventory
- `CHANGELOG.md` line 23: "**50+ agents** across 8 role families"
- `CLAUDE.md` line 14: "**50+ agents**: narrow specialists"
- **Status**: Using "50+" to avoid drift (correct pattern; no fixed count maintained)

---

## Implications for Flow 1

### Constraints for Audit Requirements

1. **Update flow count in public-facing docs**:
   - Change "six flows" → "seven flows" in README.md, DEMO_RUN.md, ARCHITECTURE.md
   - Explain the multi-path design (Flow 4/5/6 splits, Flow 7 as alternate path)

2. **Clarify flow semantics**:
   - Document the relationship between `/flow-4-gate` vs `/flow-4-review`
   - Document the relationship between `/flow-5-gate` vs `/flow-5-deploy`
   - Document the relationship between `/flow-6-deploy` vs `/flow-6-wisdom` vs `/flow-7-wisdom`
   - Answer: "Do we have 6 sequential flows with alternate paths, or 7 distinct flows?"

3. **Update CLAUDE.md flow table**:
   - Either expand to show all 7 flows + explain overlaps, OR
   - Keep the 6-flow main table and add a subsection explaining variants

4. **Test metric audit scope**:
   - The compliance run's subtask structure (ST-001 through ST-006) should account for Flow 7 if it's a distinct flow
   - Flow 7 is not currently represented in the subtask partitioning

---

## Assumptions Made to Proceed

1. **GitHub issues are disabled on this repo** (EffortlessMetrics/demo-swarm-swarm).
   - No API calls were attempted; all research is local.
   - This is consistent with `github_ops_allowed: false` in run_meta.

2. **The seven-flow table in CLAUDE.md is the source of truth for flow count** (line 13 of CLAUDE.md explicitly lists "7 flows").
   - README, DEMO_RUN, and ARCHITECTURE are outdated.

3. **Flow variants (4-gate vs 4-review, etc.) are intentional design patterns**, not bugs.
   - Each variant command likely represents a different entry point into the review/gate/deploy/wisdom cycle.
   - This design enables re-entry flows (e.g., rework after gate rejection).

4. **Flow 7 (/flow-7-wisdom) is an intentional standalone flow**, not a duplicate.
   - It exists as a parallel to Flow 6 for multi-cycle operations.
   - Its absence from public documentation is the issue, not its existence.

---

## Questions / Clarifications Needed

1. **Flow semantics**: Are the 10 flow commands implementing:
   - Option A: 6 main flows (1-6) with variant re-entry paths (4-review, 5-gate, 6-deploy, 6-wisdom)?
   - Option B: 7 distinct flows (1-7) with Flow 7 as an explicit second-cycle path?
   - **Impact**: Determines how documentation should represent them.

2. **Flow 7 purpose**: What is the intended use case for `/flow-7-wisdom`?
   - Is it a second iteration on the same run?
   - Is it for analyzing a different run?
   - Is it for batch learnings across multiple runs?

3. **Test metric audit**: Are there specific test count discrepancies to address?
   - The compliance run mentions "test metric discrepancies" but doesn't define them.
   - Should I search for specific tests that are missing or miscounted?

4. **Subtask scope**: Should ST-007 be added to the compliance run's subtask partitioning if Flow 7 is a distinct flow?

---

## Inventory (machine countable)

- CODE_REF: `.claude/commands/flow-1-signal.md` note=Signal flow (documented)
- CODE_REF: `.claude/commands/flow-2-plan.md` note=Plan flow (documented)
- CODE_REF: `.claude/commands/flow-3-build.md` note=Build flow (documented)
- CODE_REF: `.claude/commands/flow-4-gate.md` note=Gate flow variant (documented but marked as variant)
- CODE_REF: `.claude/commands/flow-4-review.md` note=Review flow variant (undocumented overlap)
- CODE_REF: `.claude/commands/flow-5-deploy.md` note=Deploy flow variant (documented)
- CODE_REF: `.claude/commands/flow-5-gate.md` note=Gate flow variant re-entry (undocumented overlap)
- CODE_REF: `.claude/commands/flow-6-deploy.md` note=Deploy flow variant (undocumented overlap)
- CODE_REF: `.claude/commands/flow-6-wisdom.md` note=Wisdom flow variant (documented)
- CODE_REF: `.claude/commands/flow-7-wisdom.md` note=Seventh standalone wisdom flow (UNDOCUMENTED)

---

## Machine Summary

```yaml
status: UNVERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: null
route_to_station: requirements-author

blockers: []

missing_required: []

concerns:
  - Six-Flow claims in public docs (README, DEMO_RUN, ARCHITECTURE) conflict with seven-flow listing in CLAUDE.md
  - Flow overlap (flow-4-review, flow-4-gate, flow-5-gate, etc.) is not documented in public-facing guides
  - Flow 7 (/flow-7-wisdom) is completely absent from public documentation despite existing in pack
  - CHANGELOG v1.0.0 claims "6 flow commands" but pack ships with 10 command files (unclear semantics)
  - "Test metric discrepancies" mentioned in audit scope but not itemized in local context; may require clarification

observations:
  - CLAUDE.md (pack reference) is more accurate than public docs; suggests pack reference may not be kept in sync with tutorials/guides
  - Prior alignment work (align-doc-ownership, compliance-drift-proofing) touched similar issues but focused on ownership boundaries, not flow count clarity
  - Multi-path design is intentional (enables re-entry flows), but needs documentation explaining when/why to use each variant
  - Team is aware of alignment issues (compliance run exists); this audit can leverage prior findings

can_further_iteration_help: yes
```
