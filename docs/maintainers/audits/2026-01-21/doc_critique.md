# Documentation Critique Report

## Inputs Reviewed

- `.claude/agents/` (84 agent files)
- `docs/reference/agents-index.md` (master agent listing)
- `docs/reference/contracts.md` (control-plane blocks, schemas)
- `docs/reference/flow-comparison.md` (flow summaries and agent counts)
- `docs/reference/routing-table.md` (routing patterns)
- `docs/reference/demoswarm-cli.md` (CLI reference)
- `docs/reference/pr-quality-scorecard.md` (review surfaces)
- `docs/reference/pr-review-interface.md` (PR cockpit design)

---

## Stale / Missing / Inaccurate Documentation

### [MISSING_DOC] DOC-CRIT-001: Agents Index Incomplete
- **File/surface:** `docs/reference/agents-index.md`
- **Issue:** Claims 61 agents in "Listed" count but `.claude/agents/` contains 84 agents
- **Missing agents (12 total):**
  - `evidence-sufficiency-critic` - Critic agent for evidence quality
  - `intent-auditor` - Verification agent for intent/specification traceability
  - `maintainability-analyst` - Analytics agent for code maintainability
  - `merge-reconciler` - Handles git merge conflict resolution
  - `mold-improver` - Refactoring/codebase quality agent
  - `pattern-analyst` - Analytics agent for codebase patterns
  - `process-analyst` - Analytics agent for workflow analysis
  - `quality-analyst` - Analytics agent for quality assessment
  - `review-cockpit-designer` - Designs PR review surfaces
  - `signal-quality-analyst` - Analyzes signal quality in Flow 1
  - `solution-analyst` - Analytics agent for solution validation
  - `spec-auditor` - Verification agent for spec completeness
- **Impact:** Orchestrators and users cannot reference complete agent list; critical agents missing from flow diagrams
- **Suggested update:** Add 12 missing agents to appropriate color family sections with descriptions and primary flow involvement
- **Route to:** doc-writer to update agents-index.md

### [STALE_DOC] DOC-CRIT-002: Agent Index References Non-Existent Agents
- **File/surface:** `docs/reference/agents-index.md` line 112
- **Issue:** References agent `mutator` which does not exist in `.claude/agents/`
- **Detail:** Should be `mutation-auditor` per actual file: `.claude/agents/mutation-auditor.md`
- **Impact:** Routing confusion; users searching for "mutator" won't find the actual agent
- **Suggested update:** Replace `mutator` with `mutation-auditor`; ensure description matches actual agent behavior
- **Route to:** doc-writer

### [STALE_DOC] DOC-CRIT-003: Agent Index Lists Model Selection Options Incorrectly
- **File/surface:** `docs/reference/agents-index.md` lines 185-189
- **Issue:** "Agent Model Selection" section lists `haiku`, `sonnet`, `inherit`, and `model` as agent values, but these are not agents—they are model selection constants
- **Detail:** These appear in the "Additional Agents" table when they should not; they are YAML frontmatter options, not agents themselves
- **Impact:** Confuses readers about what constitutes an "agent"
- **Suggested update:** Move model selection guidance to a separate subsection or link to [model-allocation.md](model-allocation.md); remove from agents table
- **Route to:** doc-writer

### [MISSING_DOC] DOC-CRIT-004: Flow-to-Agent Mapping Incomplete
- **File/surface:** `docs/reference/agents-index.md` Flow-to-Agent Mapping section (lines 193-223)
- **Issue:** 12 missing agents not referenced in any flow diagram; unclear where these agents are invoked
- **Detail:** Examples:
  - `intent-auditor`, `evidence-sufficiency-critic` - Unclear which flows invoke them
  - `spec-auditor` - Not shown in Flow 2 plan diagram
  - `signal-quality-analyst` - Not shown in Flow 1 signal diagram
  - `merge-reconciler` - Not referenced (routing table mentions it at routing-table.md:46 but not indexed in flows)
  - `review-cockpit-designer` - Not shown in Flow 3 or Flow 4
  - All 12 missing agents lack flow placement
- **Impact:** New users cannot understand when/why these agents are invoked
- **Suggested update:** Add missing agents to flow diagrams with clear positioning and rationale
- **Route to:** doc-writer

### [STALE_DOC] DOC-CRIT-005: Agent Family Counts Likely Incorrect
- **File/surface:** `docs/reference/agents-index.md` Quick Reference table (lines 11-21)
- **Issue:** Agent count claims (61 total, with breakdown by family: Yellow 7, Purple 9, Green 7, Red 11, Blue 13, Orange 11, Cyan 2, Pink 1) do not match 84 actual agents
- **Detail:** The 12 missing agents likely belong to existing families (most likely: Blue +5, Orange +4, Red +2, Purple +1), making color family counts inaccurate
- **Impact:** Readers cannot rely on this summary table for understanding pack scope
- **Suggested update:** Recount all agents by color family; update summary table with accurate totals
- **Route to:** doc-writer

### [ACCURACY_QUESTION] DOC-CRIT-006: Routing Table Completeness
- **File/surface:** `docs/reference/routing-table.md` line 46
- **Issue:** References `merge-reconciler` in routing patterns but this agent is not listed in agents-index.md
- **Status:** Agent exists in `.claude/agents/merge-reconciler.md` but documentation is out of sync
- **Suggested update:** Ensure merge-reconciler appears in agents-index.md with role description
- **Route to:** doc-writer

### [ACCURACY_NOTE] DOC-CRIT-007: Flow Comparison Agent Counts Unverified
- **File/surface:** `docs/reference/flow-comparison.md` lines 22-31
- **Issue:** Claims agent counts per flow (e.g., Flow 1: 17 agents, Flow 2: 19 agents) but with 23 agents missing from index, actual counts are likely different
- **Detail:** Cannot verify accuracy without complete agent index
- **Status:** Likely STALE but requires full agent-to-flow mapping to confirm
- **Suggested update:** After agents-index.md is complete, re-verify agent counts per flow
- **Route to:** doc-writer (after agents-index.md fix)

### [VERIFICATION_MISMATCH] DOC-CRIT-008: demoswarm-cli.md May Reference Removed Commands
- **File/surface:** `docs/reference/demoswarm-cli.md`
- **Issue:** Provides detailed CLI reference but no verification that all documented commands exist in current implementation
- **Detail:** Document claims resolve order (lines 31-35) and multiple implementation options, but no output from `bash .claude/scripts/demoswarm.sh --help` provided to verify
- **Status:** Cannot determine staleness without running verification
- **Suggested update:** Run demoswarm.sh help output and compare against documented commands; verify all examples work
- **Route to:** test-executor or code-critic to verify CLI docs match implementation

---

## Accuracy Check: Schemas and Contracts

### [VERIFIED] DOC-CRIT-009: contracts.md Status Enums Appear Current
- **File/surface:** `docs/reference/contracts.md` lines 15-42
- **Finding:** Status axis (VERIFIED | UNVERIFIED | CANNOT_PROCEED) and recommended action enums (PROCEED | RERUN | BOUNCE | FIX_ENV) are referenced throughout CLAUDE.md and match established terminology in rules
- **Status:** ACCURATE (no staleness detected)

### [VERIFIED] DOC-CRIT-010: Control-Plane Blocks Documentation Current
- **File/surface:** `docs/reference/contracts.md` lines 46-99
- **Finding:** Gate Result (secrets-sanitizer) and Repo Operator Result schemas are detailed and referenced in CLAUDE.md
- **Status:** ACCURATE (referenced in rule 30-autonomy-and-boundaries.md)

---

## User-Visible Changes Needing Documentation

### [FEATURE_ADDITION] DOC-CRIT-011: v2.2 Refactor Added 12 New Agents
- **Context:** v2.2 commit (8ef6bc5) added comprehensive agent refactor but agents-index.md was not updated
- **Missing documentation for:**
  - New verification agents (intent-auditor, evidence-sufficiency-critic, spec-auditor)
  - New analytics agents (quality-analyst, solution-analyst, pattern-analyst, process-analyst, signal-quality-analyst, maintainability-analyst)
  - New cross-cutting agents (merge-reconciler, review-cockpit-designer)
- **Suggested update:** Create summary of v2.2 agent additions with role descriptions
- **Route to:** doc-writer

---

## Documentation Strengths

1. **contracts.md** is well-structured and provides clear enum definitions with good examples
2. **pr-quality-scorecard.md** accurately describes the five quality surfaces with good inspection points
3. **routing-table.md** provides useful patterns for common routing decisions
4. **pr-review-interface.md** gives clear guidance on PR cockpit design
5. **flow-comparison.md** structure is good (though agent counts may need verification)

---

## Verification Guidance Gaps

### [INFO_GAP] DOC-CRIT-012: No Verification Instructions for Agent Index
- **Surface:** `docs/reference/agents-index.md`
- **Gap:** No documented way to verify that agent list is current
- **Missing:** No script or procedure listed to check `.claude/agents/` against agents-index.md
- **Suggested update:** Add section: "To verify completeness: `ls .claude/agents/ | wc -l` should match claimed agent count"
- **Route to:** doc-writer

### [INFO_GAP] DOC-CRIT-013: No Staleness Detection for Flow Diagrams
- **Surface:** `docs/reference/agents-index.md` Flow-to-Agent Mapping section
- **Gap:** No guidance for readers on how to verify that flow sequences are current
- **Suggested update:** Add verification note: "Cross-check these sequences with actual flow commands in `.claude/commands/flow-*.md`"
- **Route to:** doc-writer

---

## Summary of Findings

### What I Found

**Significant staleness in agents-index.md:**
- 12 agents (14% of pack) are missing from the master index
- 1 agent name is incorrect (mutator → mutation-auditor)
- Model selection constants incorrectly listed as agents
- Agent family counts (61 vs 84) are off by 23 agents
- Flow-to-agent mapping incomplete—missing agents have no documented flow placement
- No verification instructions for index freshness

**Likely cascading impacts:**
- flow-comparison.md agent counts per flow (lines 22-31) are probably inaccurate
- Orchestrators and new users cannot reference complete routing guide
- v2.2 refactor added 12 agents but documentation was not updated

**Verified as current:**
- contracts.md enums and control-plane blocks are accurate
- pr-quality-scorecard.md five surfaces are correct
- routing-table.md patterns are sound
- CLAUDE.md references are consistent

### What's Left

1. **agents-index.md** needs complete recount and update
2. **flow-comparison.md** agent counts need re-verification (depends on agents-index.md)
3. **demoswarm-cli.md** needs verification that documented commands exist
4. **Verification procedures** need to be documented (how to detect future staleness)

### Recommendation

**Route to doc-writer.** Three specific tasks:

1. **Priority 1:** Update `agents-index.md` to include all 84 agents with color families, descriptions, and flow placement (DOC-CRIT-001, 002, 003, 004, 005)
2. **Priority 2:** Re-verify `flow-comparison.md` agent counts after agents-index.md is complete (DOC-CRIT-007)
3. **Priority 3:** Add verification guidance section to agents-index.md to prevent future staleness (DOC-CRIT-012)

After doc-writer completes these updates, run self-reviewer to verify all surfaces are synchronized and accurate.
