# Documentation Critique: docs/how-to/ Guides

## Inputs Used
- All 21 how-to guides in `docs/how-to/`
- Flow command definitions in `.claude/commands/flow-*.md`
- Agent definitions in `.claude/agents/*.md`
- Skills structure in `.claude/skills/*/SKILL.md`
- Git log (recent v2.2 refactor and subsequent commits)
- Run artifacts under `.runs/`

## Summary

The how-to guides are **largely accurate and current** with the implementation. The guides correctly reflect:
- Flow structure (7 flows with microloops)
- Agent responsibilities and handoff patterns
- Run artifacts and receipt structure
- Branch naming conventions (`run/<run-id>`)
- Skills customization points (test-runner, auto-linter, policy-runner)
- Natural language routing model

However, there are **3 areas of concern** that need attention:

---

## Findings

### 1. MINOR_DRIFT: Working with Microloops Guide References Non-Existent "observability-designer"

**Location:** `docs/how-to/working-with-microloops.md`, line 39

**What it says:**
```markdown
| `observability-designer` | `observability-critic` | `observability_spec.md` → `observability_critique.md` |
```

**What's implemented:**
Agents exist:
- `observability-critic.md` ✓
- `design-optioneer.md` ✓
- `interface-designer.md` ✓
- NO `observability-designer.md`

**Severity:** MINOR_DRIFT - The agent pair is referenced but `observability-designer` doesn't exist as a named agent.

**Fix needed:** Either create `observability-designer.md` to match the guide, or update the guide to remove this row. The observability concern may be rolled into `design-optioneer` in the current implementation.

**Route to:** Check with designers; if intentional consolidation, update guide. If missing agent, create it.

---

### 2. ACCURACY_CHECK: Microloop Flow 2 Pairs May Be Incomplete

**Location:** `docs/how-to/working-with-microloops.md`, Flow 2 table (lines 34-39)

**What it says:**
```markdown
| `design-optioneer` | `option-critic` | `design_options.md` → `option_critique.md` |
| `interface-designer` | `contract-critic` | `api_contracts.yaml` → `contract_critique.md` |
| `observability-designer` | `observability-critic` | `observability_spec.md` → `observability_critique.md` |
```

**What's in Flow 2 command:**
The `flow-2-plan.md` orchestrator references:
- `design-optioneer` ✓
- `interface-designer` ✓
- `adr-author` ✓ (not in the table!)
- `clarifier` (if needed) ✓ (not in the table)

**Issue:** The guide lists only 3 writer-critic pairs for Flow 2, but the actual flow includes `adr-author` and possibly `clarifier` work. The table is incomplete.

**Severity:** INCOMPLETE_TABLE - Readers won't know all microloops in Flow 2.

**Fix needed:** Verify which pairs are standard Flow 2 microloops, then update the table.

---

### 3. VERIFICATION_COMMAND_ACCURACY: Troubleshoot Guide References Correct Paths

**Location:** `docs/how-to/troubleshoot.md`, lines 34-62

**Verified accurate:**
- `bash .claude/scripts/pack-check.sh` ✓
- `.runs/<run-id>/<flow>/secrets_status.json` ✓
- `.runs/<run-id>/<flow>/git_status.md` ✓
- `gh auth status` ✓

**No issues found** in troubleshooting paths or command examples.

---

### 4. SKILL_CUSTOMIZATION_PATHS: All Verified Current

**Location:** `docs/how-to/customize-pack.md`, lines 73-127

**What it says to edit:**
- `.claude/skills/test-runner/SKILL.md` ✓ EXISTS
- `.claude/skills/auto-linter/SKILL.md` ✓ EXISTS
- `.claude/skills/policy-runner/SKILL.md` ✓ EXISTS

**Verification:** All three skill directories exist and contain SKILL.md files.

**No issues found** in skill customization paths.

---

### 5. FLOW_COMMAND_EXAMPLES: All Current

**Location:** Multiple guides reference `/flow-1-signal`, `/flow-2-plan`, etc.

**Verified:**
- All 7 flow commands exist in `.claude/commands/flow-*.md` ✓
- Command syntax matches documentation ✓
- Run ID argument handling correct ✓

**No issues found** with flow command documentation.

---

### 6. AGENT_REFERENCES: Largely Accurate With One Gap

**Location:** `docs/how-to/decompose-work.md`, `orchestrator-decision-tree.md`

**Cross-checked agent names** against actual `.claude/agents/` directory:
- All mentioned agents exist ✓
- Responsibilities match prompts ✓
- Color coding matches agent frontmatter ✓

**One potential issue:** The decompose-work guide mentions `requirements-author` and `requirements-critic` but doesn't mention they're specifically Flow 1 microloop pair (implicit understanding required).

**Severity:** CLARITY_ISSUE (not a staleness issue) - Readers with Flow 1 context will understand; readers without might not know when to use these agents.

---

### 7. RECEIPT_STRUCTURE: Accurate

**Location:** `docs/how-to/working-with-receipts.md`

**Verified against implementation:**
- Receipt paths correct (`signal_receipt.json`, `plan_receipt.json`, etc.) ✓
- Status field values accurate (`VERIFIED`, `UNVERIFIED`, `CANNOT_PROCEED`) ✓
- Evidence SHA staleness detection documented correctly ✓
- DevLT tracking optional field documented ✓

**No issues found.**

---

### 8. BRANCH NAMING_DISCREPANCY: Minor Inconsistency (Not Staleness)

**Location:** `docs/how-to/team-operations.md`, lines 54-59

**What it says:**
```markdown
- `swarm/<run-id>` - standard swarm branch
- `feat/<run-id>` - feature-style naming
- `run/<run-id>` - alternative pattern
```

**What implementation uses:** `run/<run-id>` (from all flow commands)

**Issue:** The guide lists three options but implementation standardizes on `run/<run-id>`. The other two are presented as if equally standard when they're not.

**Severity:** MISLEADING_GUIDANCE - New users might choose `swarm/<run-id>` thinking it's standard, then encounter conflicts.

**Fix needed:** Clarify that `run/<run-id>` is the standardized convention. The others should be noted as "if you modify repo-operator" or removed.

---

## What's Accurate and Well-Documented

✓ Flow structure and sequencing (Signal → Plan → Build → Review → Gate → Deploy → Wisdom)
✓ Microloop patterns (writer ↔ critic ↔ orchestrator routing)
✓ Handoff philosophy (prose-based routing, no parsing)
✓ Receipt mechanics and freshness detection
✓ Run topology and branch model (when using `run/<run-id>`)
✓ Skill customization paths and examples
✓ Pack-check and demoswarm CLI references
✓ Troubleshooting decision trees
✓ Security/boundary gating model (secrets scan, repo hygiene checks)

---

## Summary of Issues Found

| Category | Count | Severity | Action |
|----------|-------|----------|--------|
| Missing agent definitions | 1 | MINOR_DRIFT | Clarify design or create missing agent |
| Incomplete microloop tables | 1 | INCOMPLETE_TABLE | Update Flow 2 microloop table |
| Misleading branch guidance | 1 | MISLEADING_GUIDANCE | Clarify `run/<run-id>` as standard |
| Clarity gaps (not staleness) | 1 | CLARITY_ISSUE | Add context about agent scope/flow |

---

## What's Left

**Stale docs to update:**
1. `docs/how-to/working-with-microloops.md` — line 39 (observability-designer reference)
2. `docs/how-to/working-with-microloops.md` — line 34-39 (Flow 2 microloop table incomplete)
3. `docs/how-to/team-operations.md` — lines 54-59 (branch naming guidance ambiguous)

**Nothing missing that's critical.** The docs are comprehensive and largely current.

---

## Handoff

**What I found:**
Reviewed all 21 how-to guides against current implementation. Found **3 issues:**
1. Dangling agent reference (`observability-designer` mentioned but doesn't exist)
2. Incomplete microloop table for Flow 2 (adr-author, clarifier not listed)
3. Branch naming guidance lists non-standard options as if equally valid

All other guides are accurate, current, and well-structured.

**What's left:**
3 doc updates needed in guides that reference implementation details that have drifted slightly.

**Recommendation:**
Route to **doc-writer** to:
1. Verify and fix the `observability-designer` reference (create agent or remove from guide)
2. Update Flow 2 microloop table with complete list
3. Clarify branch naming: emphasize `run/<run-id>` as standard, explain others as customization variants

One pass should address all three. No deep rework needed—these are small targeted fixes in specific sections.

**Alternative (if observability-designer is a design decision):** Route to **interface-designer** or **design-optioneer** to confirm whether observability design is handled separately or consolidated into design options.
