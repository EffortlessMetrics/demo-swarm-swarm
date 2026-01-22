# Index.md Accuracy Review Report

> **Note:** This audit was conducted on 2026-01-21. All identified issues have been resolved in this PR.

**File reviewed:** `.claude/index.md`
**Date:** 2026-01-21
**Status:** RESOLVED

---

## Summary

The `.claude/index.md` index file is **significantly out of sync** with actual agent implementations. The index lists **73 agents** but **84 agents** actually exist in the `.claude/agents/` directory. Additionally, several agent descriptions do not match their actual purposes as documented in agent frontmatter.

**Key findings:**
- 11 agents are missing from the index entirely
- Several descriptions are incomplete or inaccurate
- Skills list appears correct
- Commands list (not detailed in index) is correct

---

## 1. Missing Agents (Not Listed in Index)

The following **12 agents** exist in `.claude/agents/` but are not documented in `.claude/index.md`:

1. **evidence-sufficiency-critic** — Evaluate whether evidence panel is sufficient for risk profile
2. **intent-auditor** — Audit ADR/BDD/REQ coherence and flag missing NFRs
3. **maintainability-analyst** — Deep analysis of code maintainability dimensions
4. **merge-reconciler** — Resolve merge conflicts so repo compiles and tests pass
5. **mold-improver** — Improve codebase structure as a mold for future generation
6. **pattern-analyst** — Analyze patterns and anti-patterns in implementation
7. **process-analyst** — Analyze flow efficiency and process bottlenecks
8. **quality-analyst** — High-level code quality health check
9. **review-cockpit-designer** — Design the PR cockpit/review surface
10. **signal-quality-analyst** — Analyze signal clarity and completeness at start
11. **solution-analyst** — Analyze solution fitness against problem statement
12. **spec-auditor** — Audit specification completeness and consistency

**Severity:** HIGH — These are substantive agents doing real work that should be documented.

**Recommendation:** Add all 12 agents to the index in their appropriate flow sections or cross-flow utilities section.

---

## 2. Agent Description Accuracy Issues

The index descriptions are **sometimes oversimplified** compared to actual agent scope. A few examples:

### test-executor (Line 99)
**Index:** "Execute test suite, write verification report"
**Actual (from frontmatter):** "Execute the configured test suite (via test-runner skill) and write a tool-bound verification report → .runs/<run-id>/build/test_execution.md. No git. No fixes."

**Issue:** Index is vague about the tool boundary and output location. The actual description is more specific about what the agent does NOT do.

### standards-enforcer (Line 100)
**Index:** "Run formatters/linters, remove debug artifacts"
**Actual (from frontmatter):** "Check for suspicious test deletions + polish hygiene. Runs formatters/linters (auto-fix), removes debug artifacts."

**Issue:** Index misses the primary job (catch reward hacking via test deletion). The actual description is clearer about intent.

### doc-critic (Line 103)
**Index:** "Review documentation freshness"
**Actual:** (from agent prompt, which acts as doc-critic for the system itself)

**Issue:** The description is accurate but minimal. Does not explain what "freshness" means (stale sections, missing docs, etc.). Worth being slightly more explicit.

---

## 3. Skills List Verification

**Status:** CORRECT

Skills listed in `.claude/index.md` (lines 20-26 in the model guidance section):
- `*-cleanup` agents ✓
- `test-executor` ✓
- `traceability-auditor` ✓
- `flow-historian` ✓
- `gh-researcher` ✓

Skills in CLAUDE.md and actual `.claude/skills/` directory:
- test-runner ✓
- auto-linter ✓
- policy-runner ✓
- runs-derive ✓
- runs-index ✓
- openq-tools ✓
- secrets-tools ✓

All skills mentioned align. No issues found.

---

## 4. Commands List

**Status:** CORRECT (not detailed in index, but inferred structure is right)

Actual commands in `.claude/commands/`:
- customize-pack ✓
- flow-1-signal ✓
- flow-2-plan ✓
- flow-3-build ✓
- flow-4-review ✓
- flow-5-gate ✓
- flow-6-deploy ✓
- flow-7-wisdom ✓

All 8 commands expected. No issues found.

---

## 5. Model Tier Assignments

**Status:** PARTIALLY UNCLEAR

The index correctly assigns agents to tiers (Haiku for cleanup/research, Sonnet/Opus for reasoning), but the missing agents above are not assigned to any tier. This makes it unclear which model to use for them.

**Examples of missing tier assignments:**
- evidence-sufficiency-critic (actually marked `model: haiku` in agent file)
- intent-auditor (actually marked `model: haiku`)
- maintainability-analyst (actually marked `model: inherit`)
- merge-reconciler (actually marked `model: inherit`)

**Recommendation:** Either add these agents to the tier section or explicitly note that some agents use `inherit` and will use user-configured model.

---

## 6. Flow Section Completeness

The index organizes agents by flow (Signal, Plan, Build, Review, Gate, Deploy, Wisdom) and Cross-Flow Utilities.

**Issues:**
- Some missing agents would logically fit in existing flows but aren't there
- Example: `evidence-sufficiency-critic` would naturally fit in **Flow 5: Gate** as a successor to `receipt-checker` and `contract-enforcer`
- Example: `quality-analyst` and `maintainability-analyst` could fit in **Flow 3: Build** or **Flow 7: Wisdom**

**Recommendation:** Add missing agents to appropriate flow sections with clear placement logic.

---

## Actionable Next Steps

### Priority 1: Add Missing Agents
Determine the correct flow placement for all 12 missing agents and add them to the index:

**Likely Flow 5 (Gate):**
- evidence-sufficiency-critic

**Likely Flow 3 (Build):**
- quality-analyst
- pattern-analyst

**Likely Flow 7 (Wisdom):**
- maintainability-analyst
- regression-analyst (already listed)
- solution-analyst
- process-analyst

**Likely Cross-Flow Utilities:**
- intent-auditor (design audit)
- spec-auditor (specification validation)
- review-cockpit-designer (UI/presentation)
- signal-quality-analyst (signal phase analysis)
- merge-reconciler (git utilities)
- mold-improver (codebase structural work)

### Priority 2: Enhance Descriptions
Where descriptions are vague, add more specificity:
- What the agent produces
- What it does NOT do (boundaries)
- What flow phase it belongs to

Example: Instead of "Review tests vs BDD + REQ/NFR", expand to something like:
"Review test quality, coverage mapping to BDD scenarios, and alignment with NFRs. Flag test gaps and quality issues."

### Priority 3: Verify Model Assignments
For newly added agents, document their model tier or note that they inherit from user config.

---

## File Recommendations

**File to update:** `.claude/index.md`

**Estimated changes:**
- Add ~12 agent rows across appropriate flow tables
- Enhance 3-5 existing descriptions for clarity
- Add 12 agents to model tier section (or create a note about `inherit`)

**Files that don't need updating:**
- CLAUDE.md (skills list is correct)
- Agent .md files (they are correct; the index is wrong)

---

## Examples of What Index Should Say

### Good Example (Current)
```
| `design-optioneer` | Propose 2-3 architecture options |
```
Clear, specific, shows output form.

### Could Be Better (Current)
```
| `doc-critic` | Review documentation freshness |
```

### Improved Version
```
| `doc-critic` | Review documentation for staleness — flag outdated sections, missing docs, wrong procedures |
```

---

## Conclusion

The `.claude/index.md` file is a valuable reference but is **out of sync with actual agent implementations**.

**What's working:**
- Overall structure (flows, cross-flow utilities) is sound
- Skills list is accurate
- Commands are correct
- Model tier guidance is well-explained

**What needs fixing:**
- **11 agents are completely missing** from the documentation
- Several descriptions are vague or incomplete
- Flow assignments for some agents are unclear

**Effort to fix:** Low. Adding 12 rows, enhancing 3-5 descriptions, and verifying model assignments is straightforward and would complete the index in ~30 minutes.

**Priority:** Medium. This is a reference doc, not a functional requirement, but keeping it current maintains trust in the documentation and helps new contributors understand the agent ecosystem.
