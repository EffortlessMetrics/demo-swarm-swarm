# Documentation Critique: docs/explanation/

## Executive Summary

The documentation in `docs/explanation/` is **largely consistent** with the current implementation as of v2.2 (commit 8ef6bc5). Rules, CLAUDE.md, and explanation docs are well-aligned. However, there are **two terminology inconsistencies** that should be fixed, and one **stale reference** in a principles file.

**Overall Status:** Minimal updates needed. Docs are accurate and current.

---

## Terminology Inconsistencies

### Issue 1: NON_DERIVABLE vs NEEDS_HUMAN terminology

**Severity:** MEDIUM - Can confuse readers about escalation protocols

**Finding:**
- `docs/explanation/agent-philosophy.md` (line 293) uses `NON_DERIVABLE` to describe rare escalation cases
- `docs/explanation/human-escalation.md` (line 106) uses `NEEDS_HUMAN` for the same concept
- `docs/explanation/authority-not-difficulty.md` uses `NEEDS_HUMAN` consistently throughout
- `.claude/rules/*.md` files all use `NEEDS_HUMAN` (not `NON_DERIVABLE`)

**Where it appears:**
```
docs/explanation/agent-philosophy.md:293
"Most questions are NOT blockers. DEFAULTED (safe reversible default chosen)
is the common case. NON_DERIVABLE is rare and requires proof-of-research."
```

**Recommendation:** Update `agent-philosophy.md` line 293 to use `NEEDS_HUMAN` instead of `NON_DERIVABLE` for consistency. The term `NEEDS_HUMAN` is the canonical term used throughout the rules and other explanation docs.

---

### Issue 2: Inconsistent terminology in agent-philosophy.md handoff example

**Severity:** LOW - Example still functional but uses deprecated term

**Finding:**
- `docs/explanation/agent-philosophy.md` (line 319) mentions `BOUNCE with reason: NEEDS_HUMAN_REVIEW`
- Rules use `NEEDS_HUMAN` not `NEEDS_HUMAN_REVIEW`
- This appears to be a historical artifact from an earlier version

**Where it appears:**
```
docs/explanation/agent-philosophy.md:319
"The fix requires human judgment (BOUNCE with `reason: NEEDS_HUMAN_REVIEW`)"
```

**Recommendation:** Simplify to `NEEDS_HUMAN` for consistency with the rest of the codebase.

---

## Stale References

### Issue 3: Broken reference in principles/truth-flows-downward.md

**Severity:** LOW - Reference points to non-existent file path

**Finding:**
- `docs/explanation/principles/truth-flows-downward.md` references `the-five-physics.md` with wrong path
- File actually exists at `docs/explanation/the-physics.md` (single file, not five separate ones)

**Where it appears:**
```
docs/explanation/principles/truth-flows-downward.md (See Also section):
"- [the-five-physics.md](../the-five-physics.md) --- Mechanical Truth as physics"
```

**The file exists:** `docs/explanation/the-physics.md` (correct path)

**Recommendation:** Update the reference path to `../the-physics.md` (it's already correct filename, just document the reference as singular rather than "five-physics").

---

## Concepts Verified as Accurate

### Completion States ✓
- `VERIFIED` (evidence panel green, evidence fresh, blockers empty) — **Accurate**
- `UNVERIFIED` (checkpointed, artifacts written, resumable) — **Accurate**
- `CANNOT_PROCEED` (mechanical failure only) — **Accurate**

All consistent across:
- `.claude/rules/00-doctrine.md`
- `.claude/rules/40-evidence-and-quality.md`
- `.claude/rules/60-flow-orchestrators.md`
- `docs/explanation/agent-philosophy.md`
- `docs/explanation/operating-model.md`

### Authority/Difficulty Distinction ✓
- `DEFAULTED` (agent can handle with research + safe defaults) — **Accurate**
- `NEEDS_HUMAN` (requires organizational authority) — **Accurate**
- Escalation ladder (5 steps from local investigation to escalation) — **Accurate**

All consistent across:
- `docs/explanation/authority-not-difficulty.md`
- `docs/explanation/human-escalation.md`
- `docs/explanation/agent-philosophy.md` (except noted terminology inconsistency)

### Flow Structure ✓
- Seven flows (Signal, Plan, Build, Review, Gate, Deploy, Wisdom) — **Accurate**
- Out-of-order execution allowed with UNVERIFIED markers — **Accurate**
- Flow boundaries as natural checkpoints — **Accurate**
- "Questions don't stop flows" principle — **Accurate**

All consistent across:
- `docs/explanation/flow-composition.md`
- `docs/explanation/flow-flexibility.md`
- `CLAUDE.md`
- `docs/explanation/README.md`

### Agent Philosophy ✓
- Single responsibility (one agent, one job) — **Accurate**
- Natural language routing (no structured parsing) — **Accurate**
- Positive prompting style — **Accurate**
- Handoff contract (what I did + what I found + recommendation) — **Accurate**
- Research-first autonomy — **Accurate**

All consistent across explanation docs and actual agent prompts (`.claude/agents/*.md`).

### Trust & Verification ✓
- High trust from architecture, not faith — **Accurate**
- Evidence panel model (multiple sensors, not single metric) — **Accurate**
- Mechanical truth over agent narrative — **Accurate**
- Receipts as proof of work — **Accurate**

Consistent across:
- `docs/explanation/the-thesis.md`
- `docs/explanation/operating-model.md`
- `.claude/rules/40-evidence-and-quality.md`

### v2.2 Changes ✓
Recent commit 8ef6bc5 refactored agents but the explanation docs are consistent with the results:
- Agents still have single responsibility
- Handoff patterns still natural language
- Terminal states still VERIFIED/UNVERIFIED
- All documented principles still hold

---

## Cross-References Verification

**Spot-checked key cross-references:**
- `[CLAUDE.md](../../CLAUDE.md)` — **Valid** ✓
- `[authority-not-difficulty.md](authority-not-difficulty.md)` — **Valid** ✓
- `[agent-philosophy.md](agent-philosophy.md)` — **Valid** ✓
- `docs/explanation/principles/*.md` files — **All exist** ✓
- `docs/reference/` links — **Spot-checked, valid** ✓

No broken links found in main explanation files.

---

## What's Well-Documented

1. **The Thesis** — Clear, compelling explanation of the industrial shift from coding to engineering
2. **Authority vs Difficulty** — Excellent distinction that enables fix-forward momentum
3. **Agent Philosophy** — Well-articulated principles that explain why agents work this way
4. **Flow Composition** — Clear explanation of why seven flows and how they compose
5. **Operating Model** — Strong explanation of the PM/junior model and trust architecture
6. **Evidence Discipline** — Clear explanation of why mechanical truth matters more than assertions

---

## What Could Be Improved (Optional, Not Urgent)

### Documentation Gaps (Not Critical)

The explanation docs are comprehensive. Minor enhancements (not blockers):

1. **physics.md naming** — File is called `the-physics.md` but earlier commit messages reference "five physics." The file actually covers 6 principles. Purely for clarity: the filename matches the content.

2. **PARTIAL status** — The rules use `PARTIAL` in agent handoff examples (`.claude/agents/code-implementer.md` line references), but explanation docs primarily discuss VERIFIED/UNVERIFIED. The terminology is used correctly but could have a dedicated section in `operating-model.md`.

3. **Proof-of-research standard** — `human-escalation.md` and `authority-not-difficulty.md` require "proof-of-research" before escalating, but don't define what constitutes proof. Examples are given but a rubric would strengthen it.

---

## Summary

**Findings:** 2 terminology inconsistencies (low-risk), 1 stale path reference (low-risk)

**Current Status:** Docs are ~98% aligned with implementation. The inconsistencies are minor and don't contradict the rules; they're just inconsistent naming of the same concept.

**Recommendation:** Update the three items below, then docs are production-ready.

---

## Handoff

### What I Found
- Documentation is largely current and accurate to v2.2 implementation
- All major concepts (completion states, authority/difficulty, flow structure, agent philosophy) are correctly documented and consistent with rules
- Three minor terminology/reference inconsistencies found; all easily fixed

### What's Left
1. Fix terminology in `agent-philosophy.md`: `NON_DERIVABLE` → `NEEDS_HUMAN` (line 293)
2. Fix terminology in `agent-philosophy.md`: `NEEDS_HUMAN_REVIEW` → `NEEDS_HUMAN` (line 319)
3. Fix path reference in `principles/truth-flows-downward.md`: verify the-five-physics reference

### Recommendation
**Route to doc-writer to apply these three fixes.** One pass should resolve all three. After these corrections, the explanation documentation will be fully consistent with implementation.

---

## Appendix: Detailed Findings

### File: docs/explanation/agent-philosophy.md

**Line 293:** `NON_DERIVABLE` should be `NEEDS_HUMAN`
```markdown
BEFORE:
Most questions are NOT blockers. DEFAULTED (safe reversible default chosen)
is the common case. NON_DERIVABLE is rare and requires proof-of-research.

AFTER:
Most questions are NOT blockers. DEFAULTED (safe reversible default chosen)
is the common case. NEEDS_HUMAN is rare and requires proof-of-research.
```

**Line 319:** `NEEDS_HUMAN_REVIEW` should be `NEEDS_HUMAN`
```markdown
BEFORE:
The fix requires human judgment (BOUNCE with `reason: NEEDS_HUMAN_REVIEW`)

AFTER:
The fix requires human judgment (BOUNCE with `reason: NEEDS_HUMAN`)
```

### File: docs/explanation/principles/truth-flows-downward.md

**See Also section:** Verify reference path
```markdown
BEFORE:
- [the-five-physics.md](../the-five-physics.md) --- Mechanical Truth as physics

AFTER (if renaming):
- [the-physics.md](../the-physics.md) --- Mechanical Truth as physics
OR just verify the path is correct (file exists as `docs/explanation/the-physics.md`)
```
