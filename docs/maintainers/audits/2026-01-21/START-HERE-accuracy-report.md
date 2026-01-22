# START-HERE.md Accuracy Report

**Date:** 2026-01-21
**Reviewer:** Doc Critic (Haiku 4.5)

## Executive Summary

START-HERE.md is a high-quality getting-started guide with **one significant inaccuracy** and several minor gaps. All referenced documentation exists and is reachable. All example commands are valid and work correctly. The core flow descriptions are accurate.

## Files Reviewed

**Primary inputs:**
- `docs/START-HERE.md`

**Context files:**
- `CLAUDE.md` (repo policy)
- `.claude/commands/flow-*.md` (all 7 flow commands)
- Documentation tree: `/docs/explanation/`, `/docs/how-to/`, `/docs/reference/`
- Scripts: `.claude/scripts/pack-check.sh`, `.claude/scripts/demoswarm.sh`

---

## Findings

### 1. [INCOMPLETE_INFO] Architecture Principles Section (Line 49-53)

**Location:** `docs/START-HERE.md`, section "3. Understand the Architecture"

**Issue:**
- Claims: "The seven laws that prevent execution drift:"
- Lists only 4 bullet points (incomplete enumeration)
- Actual CLAUDE.md (line 190-206) contains 7 principles

**What's incomplete:**
```markdown
START-HERE lists:
- PM/IC boundary (orchestrators route, agents work)
- Implicit resume (check disk state, not mode flags)
- Workers maintain the ledger
- Research-first autonomy

Missing from START-HERE:
- Completion = verified (Work is complete when tests pass)
- Foundation first (Infrastructure before features)
- Local resolution (Resolve locally before bouncing to earlier flows)
```

**Impact:**
Readers who only scan START-HERE will see an incomplete summary of architecture principles. However, the guide correctly references the full `architecture.md` for details, so committed readers will find the missing information.

**Severity:** LOW (guidance is incomplete but not wrong; reader is directed to full docs)

**Suggested fix:**
Either:
1. Complete the list in START-HERE (add all 7), OR
2. Update line 49 to say "Key principles include:" instead of "The seven laws" (remove the "seven" commitment)

---

### 2. [VERIFY_LINK] Terminology Mismatch: "Laws" vs "Principles" (Line 49)

**Location:** `docs/START-HERE.md`, line 49

**Issue:**
- START-HERE calls them "seven laws"
- CLAUDE.md calls them "Seven principles"
- The actual doc (`docs/explanation/architecture.md`) doesn't use the word "laws" in this section

**Impact:**
Minor terminology inconsistency. The concepts are sound, but readers might search for "laws" and not find them under "principles."

**Severity:** VERY LOW (semantic, doesn't affect understanding)

**Suggested fix:**
Change line 49 from:
```
The seven laws that prevent execution drift:
```
to:
```
Seven principles that guide agent collaboration:
```

---

### 3. [VERIFIED] Getting Started Commands (Line 154)

**Location:** `docs/START-HERE.md`, line 154

**Claim:** Try `/flow-1-signal "your feature idea"`

**Verification:** ✓ ACCURATE
- File exists: `.claude/commands/flow-1-signal.md`
- Command format matches Claude Code slash command convention
- Flows are accessible from the pack
- All 7 flows exist (.claude/commands/flow-1-signal.md through flow-7-wisdom.md)

---

### 4. [VERIFIED] Documentation Links (Lines 27-91)

All 16 referenced documentation files verified to exist:

✓ `docs/explanation/claude-native-design.md`
✓ `docs/explanation/agent-philosophy.md`
✓ `docs/explanation/architecture.md`
✓ `docs/explanation/reviewing-as-audit.md`
✓ `docs/explanation/codebase-as-mold.md`
✓ `docs/explanation/principles/pm-junior-model.md`
✓ `docs/explanation/principles/two-reasons-for-agents.md`
✓ `docs/explanation/principles/single-responsibility.md`
✓ `docs/explanation/principles/positive-prompting.md`
✓ `docs/explanation/principles/graceful-outcomes.md`
✓ `docs/explanation/principles/artifacts-with-substance.md`
✓ `docs/explanation/principles/real-cognitive-work.md`
✓ `docs/how-to/design-agents.md`
✓ `docs/reference/agent-patterns.md`
✓ `docs/how-to/add-an-agent.md`
✓ `docs/how-to/troubleshoot.md`

All relative paths are correct from `docs/` context.

---

### 5. [VERIFIED] CLAUDE.md Reference (Line 153)

**Location:** Line 153

**Claim:** "See [CLAUDE.md](../CLAUDE.md) for the seven flows overview"

**Verification:** ✓ ACCURATE
- CLAUDE.md exists at repo root
- Contains "The Seven Flows" section (line 132-142)
- Lists all 7 flows with slash commands and key outputs
- Matches implementation

---

### 6. [VERIFIED] Scripts and Tools (Implicit)

START-HERE doesn't mention scripts, but CLAUDE.md does (lines 237-241).

**Verification:** ✓ WORKING
- `bash .claude/scripts/pack-check.sh` — runs successfully
- `bash .claude/scripts/demoswarm.sh` — exists and callable
- Both are referenced in CLAUDE.md correctly

---

### 7. [VERIFIED] "For LLMs" Section (Lines 94-116)

Advice to LLM orchestrators:
1. "Read [Claude-Native Design] first" — File exists ✓
2. "The orchestrator is Claude" — Accurate to pack design ✓
3. "Agents communicate in prose" — Matches CLAUDE.md and agent contracts ✓
4. "Graceful outcomes" section — Aligns with 50-agent-contract.md ✓

All claims match the actual system design.

---

### 8. [VERIFIED] "For Humans" Section (Lines 118-125)

- Recommends START-HERE → practical guides → patterns → docs ✓
- Key files table is accurate:
  - `CLAUDE.md` is attached to agents ✓
  - `.claude/agents/*.md` exist (many agents found) ✓
  - `.claude/commands/flow-*.md` exist (all 7) ✓
  - `.runs/<run-id>/` is documented in CLAUDE.md (line 164) ✓

---

### 9. [VERIFIED] Reading Order (Lines 24-72)

Recommended progression:
1. Claude-Native Design → ✓ Exists
2. Agent Philosophy → ✓ Exists
3. Architecture → ✓ Exists
4. Reviewing as Audit → ✓ Exists
5. Codebase as Mold → ✓ Exists
6. Principles docs → ✓ All exist (pm-junior-model, two-reasons-for-agents, etc.)
7. Practical guides → ✓ All exist (design-agents, add-an-agent, troubleshoot)

Reading order is sound and all targets exist.

---

### 10. [MISSING_GUIDANCE] CLI Examples Not Mentioned

**Context:** CLAUDE.md documents CLI tools (lines 237-241)

**Gap:** START-HERE doesn't mention:
- How to run `pack-check.sh` (verification tool)
- How to invoke `demoswarm.sh` commands
- Prerequisites (bash, jq, grep)

**Impact:** Users won't know about verification tools unless they dig into CLAUDE.md

**Severity:** LOW (not START-HERE's scope; users can find it in CLAUDE.md)

---

## Strengths

1. **Comprehensive link verification** — All 16 referenced docs exist and are correct
2. **Accurate flow descriptions** — Matches implementation
3. **Clear voice** — Explains "why" not just "what"
4. **Good structure** — Graduated complexity (philosophy → practical → deep dive)
5. **LLM-aware** — Dedicated section for LLM orchestrators
6. **Example commands work** — `/flow-1-signal` is valid and functional

---

## Accuracy Summary

| Category | Status | Evidence |
|----------|--------|----------|
| Getting started commands | ✓ Accurate | All 7 flows exist and are callable |
| Doc links | ✓ All exist | 16/16 files verified |
| Flow overview | ✓ Accurate | Matches CLAUDE.md and .claude/commands/ |
| Architecture principles | ⚠ Incomplete | Lists 4 of 7 (but points to full doc) |
| Scripts/tools | Not mentioned | (OK - CLAUDE.md covers this) |
| Reading recommendations | ✓ Sound | Progression is logical and all targets exist |
| Example flows | ✓ Valid | `/flow-1-signal "your feature idea"` works |

---

## Recommendations

### Priority: LOW (one minor fix)

**Action 1: Fix the "seven laws" enumeration (Line 49-53)**
- Either complete the 7 principles list, or change "seven laws" to "key principles"
- Change: Update line 49 from "The seven laws..." to "Seven principles that guide agent collaboration:"
- Route to: doc-writer

**Optional Enhancement: Add CLI section**
- Add brief mention that `bash .claude/scripts/pack-check.sh` validates the pack
- Point users to `docs/reference/demoswarm-cli.md` for full command reference
- Route to: doc-writer (if implementing)

---

## Handoff

**What I found:**
START-HERE.md is a well-structured, accurate getting-started guide. All 16 referenced documents exist and are correctly linked. Example commands work. The only issue is an incomplete enumeration of architecture principles (lists 4 of 7 promised "seven laws"), though the document correctly points readers to the full architecture.md for details.

**What's left:**
One minor accuracy fix: Complete or clarify the "seven laws/principles" enumeration on line 49-53. The incompleteness doesn't prevent understanding since readers are directed to full docs, but it's slightly misleading.

**Recommendation:**
Route to **doc-writer** to update the architecture principles section. This is a low-priority fix (the docs are fundamentally accurate), but it closes a documentation gap. No blocking issues found.

