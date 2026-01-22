# Maintainers Documentation Review

## Inputs Used
- `docs/maintainers/README.md`
- `docs/maintainers/release-checklist.md`
- `docs/maintainers/handover.md`
- `docs/maintainers/ADR-runner-bounded-reseal.md`
- `docs/maintainers/validation-log.md`
- `docs/tutorials/validation-run.md`
- `CONTRIBUTING.md`
- `docs/how-to/maintain-the-pack.md`
- `tools/demoswarm-pack-check/Cargo.toml` (version: 1.0.1)
- `tools/demoswarm-runs-tools/Cargo.toml` (version: 1.0.1)
- `CHANGELOG.md` (v1.0.1 - 2025-12-13)

---

## Maintenance Procedures Documentation

### Coverage Assessment

**Well-documented:**
- Release process (release-checklist.md): Clear version alignment, commit/tag commands, GitHub release creation
- Pack-check validation (handover.md): Proper reference to pack-check.sh and expected location
- Validation runs (validation-run.md): Comprehensive multi-stage validation with explicit verification checkpoints
- Agent/flow/skill addition (maintain-the-pack.md, add-an-agent.md, create-a-flow.md): Detailed checklists and philosophy
- Contributing workflow (CONTRIBUTING.md): Entry point for new contributors is clear and accessible

**Gaps identified:**

1. **[MISSING_DOC] DOC-MAINT-001: Breaking Change Handling**
   - Surface: CHANGELOG.md, release-checklist.md, handover.md
   - Missing: No guidance on how to document or communicate breaking changes to pack users
   - What's needed: Upgrade path for users running older pack versions, deprecation policies, migration guides
   - Suggested addition: "Breaking Changes" section in release-checklist.md with examples
   - Route to: doc-writer

2. **[MISSING_DOC] DOC-MAINT-002: Bug Triage and Hotfix Procedures**
   - Surface: docs/maintainers/
   - Missing: No documented process for handling critical bugs discovered post-release
   - What's needed: Hotfix branch naming, emergency validation procedures, backport policies
   - Suggested addition: New doc `docs/maintainers/emergency-procedures.md`
   - Route to: doc-writer

3. **[MISSING_DOC] DOC-MAINT-003: Dependency Update Process**
   - Surface: release-checklist.md, CONTRIBUTING.md
   - Missing: How to safely update Rust dependencies (Cargo.toml versions), when to bump MSRV, testing strategy
   - What's needed: Procedure for updating Cargo dependencies without breaking compatibility
   - Suggested addition: New doc `docs/maintainers/dependency-updates.md`
   - Route to: doc-writer

---

## Upgrade/Migration Documentation

### Assessment

**Status:** No formal upgrade/migration guides exist

**Finding:** The pack is early in its lifecycle (v1.0.0 → v1.0.1). The CHANGELOG.md correctly documents changes but lacks structured upgrade paths.

**Gaps:**

1. **[MISSING_DOC] DOC-MAINT-004: Minor Version Upgrade Guide**
   - Surface: None (should be docs/how-to/upgrade-pack.md)
   - Why missing: v1.0.0 → v1.0.1 involved doc-only changes; no actual upgrade complexity yet
   - What users need: Instructions for updating `.claude/` in existing target repos
   - Suggested addition: `docs/how-to/upgrade-pack.md` with version matrix and compatibility table
   - Route to: doc-writer

2. **[MISSING_DOC] DOC-MAINT-005: Customization State Preservation**
   - Surface: docs/how-to/customize-pack.md (exists), but no guidance on applying pack updates to customized repos
   - What's needed: "How to upgrade your customized pack" guide (merge conflict scenarios, preservation of local edits)
   - Route to: doc-writer

3. **[STALE_DOC] ADR: Runner-bounded reseal loops (ADR-runner-bounded-reseal.md)**
   - Date: 2025-12-14 (very recent)
   - Assessment: Current and accurate
   - Consequence note: "Future cap changes require only CLI config/release, not doc edits" - correctly documented
   - No action needed

---

## Contribution Guidance Accuracy

### Assessment

**Strengths:**
- CONTRIBUTING.md provides clear entry point with pack scope definition
- Color ↔ role family mapping (table) is accurate and useful
- Frontmatter requirements match agent structure expectations
- Code style guidance includes Python (black, ruff) and Markdown conventions

**Issues found:**

1. **[STALE_DOC] DOC-MAINT-006: Deprecated Python Validation Scripts**
   - File: CONTRIBUTING.md (lines 39-43)
   - Claims: "Additional targeted scripts (for specific checks): python scripts/lint_frontmatter.py / python scripts/check_portable_claude.py"
   - Reality: No `scripts/` directory found in repo; validation is now done via bash .claude/scripts/pack-check.sh
   - Suggested update: Remove Python script references; clarify that pack-check.sh is the single source of truth
   - Route to: doc-writer

2. **[VERIFICATION_MISMATCH] DOC-MAINT-007: PR Checklist References Lint Script**
   - File: CONTRIBUTING.md (line 119)
   - Checklist item: "Agent frontmatter is valid (run lint script)"
   - Reality: Users should run `bash .claude/scripts/pack-check.sh`, not a separate lint script
   - Suggested update: Change to "Agent frontmatter is valid (bash .claude/scripts/pack-check.sh validates)"
   - Route to: doc-writer

3. **[ACCURACY_CHECK] Color Mapping (CONTRIBUTING.md)**
   - Verified: All colors match role families in CLAUDE.md ✓
   - Agent files checked: Spot-check confirms colors are correctly assigned
   - No action needed

4. **[MISSING_DOC] DOC-MAINT-008: Frontmatter Required Fields Location**
   - Surface: CONTRIBUTING.md (lines 53-60)
   - Example given: `name`, `description`, `color`, `model`
   - Issue: No reference to where the canonical frontmatter schema lives (if anywhere)
   - What's needed: Pointer to either `.claude/rules/` or a reference schema for new maintainers
   - Suggested addition: Link to canonical frontmatter spec (if it exists as a structured doc)
   - Route to: doc-writer

---

## Release Process Accuracy

### Assessment

**Release checklist (release-checklist.md) status:**

1. **Version Alignment [VERIFIED]**
   - Claims: Pack, pack-check, demoswarm CLI, and CHANGELOG must all match
   - Reality check:
     - CHANGELOG.md: v1.0.1 ✓
     - tools/demoswarm-pack-check/Cargo.toml: version = "1.0.1" ✓
     - tools/demoswarm-runs-tools/Cargo.toml: version = "1.0.1" ✓
   - All in sync, no action needed

2. **Validation Instructions [VERIFIED]**
   - Claim: "Run bash .claude/scripts/pack-check.sh"
   - Reality: Script exists and is executable ✓
   - No action needed

3. **Toy Run References [VERIFIED but could clarify]**
   - Lines 34-35 reference "Toy Run A" and "Toy Run B"
   - Full spec lives in: docs/tutorials/validation-run.md
   - Issue: No direct link in release-checklist.md to validation-run.md
   - Suggested enhancement: Add link: "See [validation-run.md](../tutorials/validation-run.md) for full toy run specification"
   - Route to: doc-writer (enhancement, not critical)

4. **Post-Release Dev Version Bump [OPTIONAL]**
   - Line 57: "(optional but recommended for Cargo)"
   - Assessment: Reasonable guidance; however, should note that this is NOT REQUIRED for the git tag itself
   - No action needed (already marked optional)

---

## Handover Documentation

### Assessment

**handover.md status:**

1. **Sanity Check Command [VERIFIED]**
   - Claim: `bash .claude/scripts/pack-check.sh`
   - Reality: Script exists and is the correct entry point ✓
   - No action needed

2. **Contract Change Procedures [VERIFIED but could be more concrete]**
   - Step 1-6 are correct and align with philosophy
   - Issue: No concrete example of a contract change provided
   - What would help: Worked example showing how to change a status value or agent field
   - Suggested addition: Add example under "Common drift patterns" with before/after
   - Route to: doc-writer (enhancement)

3. **Feel Test [VERIFIED but procedurally vague]**
   - Lines 63-68 describe 4 scenarios
   - Issue: These are narrative descriptions, not step-by-step procedures
   - What's needed: Either reference to actual automation (if it exists) or clearer manual steps
   - Suggested update: Clarify whether these are manual validation checks or automated tests
   - Route to: code-implementer or orchestrator (unclear if automated)

4. **Key Resources Links [VERIFIED]**
   - All 6 links point to files that exist ✓
   - validation-run.md is correctly referenced
   - No action needed

---

## Validation Log

### Assessment

**validation-log.md status:**

1. **Recording Format [VERIFIED]**
   - Template provided is clear and machine-scannable
   - Current entries follow the format
   - No action needed

2. **Latest Entry Assessment**
   - Date: 2025-12-14 (current)
   - Status: PARTIAL for interactive flows, but PASSED for pack-check
   - Note: Explicitly explains why interactive flows are PARTIAL (agent cannot trigger flows)
   - Assessment: Honest and clear ✓
   - No action needed

3. **Historical Entries [VERIFIED]**
   - 2025-12-12 entry: Documentation audit run
   - Both entries provide context and commit references
   - No action needed

---

## User-Visible Changes Needing Documentation

Based on CHANGELOG.md v1.0.1:

1. **Documentation Cleanup Changes**
   - Updated REPO_MAP.md, CONTRIBUTING.md, DEMO_RUN.md, PR template
   - Issue: None of these are mentioned in docs/maintainers/ as areas maintainers should be aware of
   - Suggested action: Add reference in handover.md to these "external" docs that may diverge
   - Route to: doc-writer

2. **Agent Count Updates**
   - Changed to "50+" to avoid drift
   - Issue: No guidance on when/how to update agent count in future releases
   - Suggested addition: Note in maintain-the-pack.md about hardcoded counts
   - Route to: doc-writer

---

## Strengths of Current Documentation

1. **Pack-check as Single Source of Truth**: Well-documented across multiple files; validation is clear
2. **Validation Run Comprehensiveness**: validation-run.md is thorough with clear verification checkpoints
3. **Agent Philosophy Integration**: maintain-the-pack.md deeply integrates CLAUDE.md principles
4. **Clear Entry Points**: README.md, handover.md, CONTRIBUTING.md all provide appropriate starting points
5. **Version Alignment Enforcement**: Release checklist makes version sync explicit and enforced

---

## Verification Instructions Accuracy

### Checked References

| Procedure | Command | Status |
|-----------|---------|--------|
| Pack validation | `bash .claude/scripts/pack-check.sh` | ✓ Verified (script exists, executable) |
| Release commit | `git commit -am "chore: prepare release vX.Y.Z"` | ✓ Correct syntax |
| Release tag | `git tag -a vX.Y.Z -m "Release vX.Y.Z"` | ✓ Correct syntax |
| Python scripts | `black scripts/ && ruff check scripts/` | ⚠ Scripts may not exist (see DOC-MAINT-006) |
| Cargo validation | Version fields in Cargo.toml | ✓ Verified |

---

## Documentation Gaps Summary

| ID | Type | Severity | Topic |
|----|------|----------|-------|
| DOC-MAINT-001 | MISSING_DOC | Medium | Breaking change communication process |
| DOC-MAINT-002 | MISSING_DOC | Medium | Bug triage and hotfix procedures |
| DOC-MAINT-003 | MISSING_DOC | Low | Rust dependency update process |
| DOC-MAINT-004 | MISSING_DOC | Low | Minor version upgrade guide (for users) |
| DOC-MAINT-005 | MISSING_DOC | Medium | Upgrading customized packs |
| DOC-MAINT-006 | STALE_DOC | High | Deprecated Python validation scripts |
| DOC-MAINT-007 | VERIFICATION_MISMATCH | High | PR checklist references non-existent lint script |
| DOC-MAINT-008 | MISSING_DOC | Low | Frontmatter schema reference |

---

## Handoff

**What I found:**

Maintenance documentation is comprehensive and accurate for day-to-day operations (validation, release process, handover). Version alignment is correct (all v1.0.1). However, two critical issues were discovered:

1. **CONTRIBUTING.md contains outdated references** to Python validation scripts (`lint_frontmatter.py`, `check_portable_claude.py`) that no longer exist. These should be removed and replaced with bash .claude/scripts/pack-check.sh references.

2. **Missing upgrade/migration guidance** for users updating from older pack versions. This is not urgent (pack is early in lifecycle) but should be documented before v1.1.0.

3. **Missing operational procedures** for handling breaking changes, emergency hotfixes, and dependency updates.

**What's left:**

- 2 stale references in CONTRIBUTING.md need immediate correction
- 5 new guides needed before v1.1.0 release (breaking changes, hotfix procedures, dependency updates, upgrade guide, customization preservation)
- 1 enhancement: Add concrete example to handover.md for contract changes

**Recommendation:**

Route to doc-writer to:
1. **IMMEDIATE:** Fix CONTRIBUTING.md (remove Python script references, update PR checklist) — DOC-MAINT-006, DOC-MAINT-007
2. **BEFORE v1.1.0:** Add breaking change policy and upgrade guides — DOC-MAINT-001, DOC-MAINT-004, DOC-MAINT-005
3. **NICE-TO-HAVE:** Add operational procedures for bugs, hotfixes, and dependencies — DOC-MAINT-002, DOC-MAINT-003

All documentation links are correct and files exist. Validation procedures are accurate and current.
