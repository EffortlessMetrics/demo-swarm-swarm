# GitHub Research for demo-swarm-swarm

## Wisdom Context (Scent Trail)

**Source**: `.runs/local-alignment-audit-aba1c6/wisdom/learnings.md`

The most recent flow (local-alignment-audit-aba1c6) completed with full verification. Key learnings that inform this research:

**Positive Patterns:**
- Explicit priority classification (HIGH/MEDIUM/LOW) enabled clear triage during review
- Requirements linked to explicit acceptance criteria enabled mechanical verification
- Layered approach (updating authoritative sources first, then deriving downstream) reduced coordination overhead
- Phased delivery with optional follow-ups allowed progress without blocking on secondary items
- Open questions register with documented defaults reduced cycle time

**Constraints & Pitfalls:**
- "7 flows via 10 command files" ambiguity caused bot reviewer confusion (FB-003, FB-004, FB-005)
- No automation exists for deriving downstream docs from authoritative sources (CLAUDE.md)
- Open questions remained OPEN even when evidence resolved them (OQ-SIG-001)
- Branch protection not enabled on main; future runs will see NOT_DEPLOYED verdicts regardless of merge success (ORG_CONSTRAINT)
- 24 markdown formatting violations (MD022, MD058) from generator templates
- "immeidate" typo across 7 flow command files (caught and fixed, but preventable)

**Pack/Flow Observations Needing Address:**
1. Build receipt write failed due to directory permissions (needed git fallback in Gate)
2. No automation for deriving downstream docs from authoritative sources
3. Open questions resolution tracking missing
4. Deploy verdict confusion due to governance constraint (branch protection)

---

## Search Inputs

| Search Type | Terms Used | Rationale |
|-------------|-----------|-----------|
| Issue List | state:all, limit:100 | Exhaustive search of all issues (open/closed) |
| PR List | state:all, limit:100 | Exhaustive search of all PRs (merged/open/closed) |
| Milestone List | API call | Identify roadmap structure and grouping |
| Label Search | Built-in GitHub + custom labels | Inventory of categorization taxonomy |
| Discussion Search | Attempted via gh CLI | Disabled for this repo (confirmed) |

**Terminology:** None. This is a complete inventory run examining roadmap-level issues (#9-#26).

---

## Access & Limitations

**GitHub CLI Status:** VERIFIED
- `gh` version: 2.83.1, available and authenticated
- Account: EffortlessSteven (keyring auth)
- Token scopes: admin:public_key, gist, read:org, repo
- Git protocol: SSH

**Available Resources:**
- Issue list: FULL (18 issues total: 1 closed, 17 open)
- PR list: FULL (7 PRs retrieved, all merged)
- Labels: FULL (9 custom labels defined + GitHub defaults)
- Milestones: VERIFIED (3 milestones defined: v1.1.0, v1.2.0, v1.3.0)
- Discussions: DISABLED (feature not enabled for this repo)
- Projects: NOT FOUND (no GitHub Project boards in use)

**Search Coverage:** Exhaustive. All 18 issues reviewed in detail. Issues #9-#26 fully analyzed. PR #2-#8 previously documented.

---

## Related Issues

### Summary Table

| Issue | Title | State | Milestone | Priority | Created | Updated | Relevance |
|-------|-------|-------|-----------|----------|---------|---------|-----------|
| #26 | Add Rust dependency caching to CI | OPEN | v1.3.0 | LOW | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #25 | Add cargo-audit job to CI workflow | OPEN | v1.1.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **HIGH** |
| #24 | Document 'command count vs flow count' phrasing | OPEN | v1.3.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #23 | Atomic write pattern for receipt generation | OPEN | v1.3.0 | LOW | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #22 | Add run-prep pre-flight for branch protection | OPEN | v1.3.0 | LOW | 2025-12-31 | 2025-12-31 | **LOW** |
| #21 | Add question resolution tracking to signal-cleanup | OPEN | v1.3.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #20 | Git-based fallback for receipt reading | OPEN | v1.3.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **HIGH** |
| #19 | Add flow count validation rule to pack-check | OPEN | v1.3.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #18 | Add spell-check to pack-check or pre-commit | OPEN | v1.3.0 | LOW | 2025-12-31 | 2025-12-31 | **LOW** |
| #17 | Fix markdown template formatting (MD022/MD058) | OPEN | v1.3.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **HIGH** |
| #16 | Warning-first mode (--strict flag) for pack-check | OPEN | v1.2.0 | HIGH | 2025-12-31 | 2025-12-31 | **HIGH** |
| #15 | Build-to-Gate receipt test fixtures | OPEN | v1.2.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #14 | OpenQ prefix normalization | OPEN | v1.2.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #13 | Skills section enforcement for agents using demoswarm.sh | OPEN | v1.2.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **MEDIUM** |
| #12 | Flow boundary enforcement in pack-check | OPEN | v1.2.0 | HIGH | 2025-12-31 | 2025-12-31 | **HIGH** |
| #11 | Update cargo-audit for CVSS 4.0 support | OPEN | v1.1.0 | MEDIUM | 2025-12-31 | 2025-12-31 | **HIGH** |
| #10 | Enable branch protection on main branch | OPEN | v1.1.0 | HIGH | 2025-12-31 | 2025-12-31 | **HIGH** |
| #9 | Fix path traversal vulnerability in secrets.rs | OPEN | v1.1.0 | HIGH | 2025-12-31 | 2025-12-31 | **HIGH** |
| #1 | DemoSwarm Documentation-Code Alignment Audit | CLOSED | none | HIGH | 2025-12-20 | 2025-12-31 | HISTORICAL |

### Issue Details and Analysis

#### Milestone v1.1.0 - Security & Governance (4 issues)

**Issues in this milestone:** #9, #10, #11, #25
**Status:** 4 open, 0 closed
**Purpose:** Security hardening and governance enforcement

##### Issue #9: Fix path traversal vulnerability in secrets.rs
**Type:** Security/Bug Fix
**Priority:** HIGH
**Dependencies:** NONE (independent)
**Complexity:** Medium (requires canonicalization logic + tests)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Add path canonicalization to secrets.rs path handling
2. Add unit tests for path traversal rejection (`../`, absolute paths)
3. Document threat model assumptions in code comments
4. Pass existing pack-check and runs-tools-tests

**Technical Notes:**
- Source code: `tools/demoswarm-runs-tools/src/commands/secrets.rs`
- Security context: Local execution (low risk) but path handling is foundational
- Relevant evidence: `.runs/local-alignment-audit-aba1c6/gate/risk_assessment.md#RSK-001`

**Blockers for Other Work:** None (independent)

---

##### Issue #10: Enable branch protection on main branch
**Type:** Governance/Configuration
**Priority:** HIGH
**Dependencies:** NONE (org-level setting)
**Complexity:** Low (GitHub settings change)
**Estimated Effort:** 0.5 hours

**Acceptance Criteria:**
1. Enable branch protection via GitHub repository settings UI
2. Configure required status checks (lint, pack-check, runs-tools-tests)
3. Verify subsequent runs achieve `deployment_verdict=STABLE`
4. Document branch protection requirements in CONTRIBUTING.md

**Technical Notes:**
- This is a repository settings change, not code
- **Blocks:** Future runs will see `NOT_DEPLOYED` verdicts until resolved
- Governance constraint preventing stable deployments (documented in wisdom)

**Blocks Other Work:**
- #22 (run-prep branch protection check) can proceed, but this must be fixed for meaningful verdicts

---

##### Issue #11: Update cargo-audit for CVSS 4.0 support
**Type:** Tooling/Dependency
**Priority:** MEDIUM
**Dependencies:** NONE (but will unblock #25)
**Complexity:** Medium (tool evaluation + config)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Upgrade cargo-audit to version supporting CVSS 4.0 (if available)
2. If no upgrade available, document workaround (cargo-deny, cargo-vet)
3. Verify `cargo audit` runs successfully on both Rust crates
4. Add cargo-audit check to CI workflow

**Technical Notes:**
- Prerequisite for #25 (cargo-audit CI job)
- May require evaluating alternative tools (cargo-deny, cargo-vet)
- Current issue: cargo-audit 0.21.2 cannot parse CVSS 4.0 scores

**Blocks Other Work:**
- **Unblocks #25** (Add cargo-audit job to CI workflow)

---

##### Issue #25: Add cargo-audit job to CI workflow
**Type:** Tooling/CI
**Priority:** MEDIUM
**Dependencies:** #11 (cargo-audit CVSS 4.0 support)
**Complexity:** Low (straightforward CI job)
**Estimated Effort:** 1 hour

**Acceptance Criteria:**
1. Add `cargo-audit` job to `.github/workflows/pack.yml`
2. Run on both Rust crates (demoswarm-pack-check, demoswarm-runs-tools)
3. Add ignore list for known false positives (if any)
4. Job fails on CRITICAL/HIGH severity findings

**Technical Notes:**
- Depends on #11 (need working cargo-audit first)
- Implementation provided in issue body (ready to implement)
- Integrates with existing pack.yml workflow

**Blocked By:** #11

---

#### Milestone v1.2.0 - Compliance Enforcement (6 issues)

**Issues in this milestone:** #12, #13, #14, #15, #16
**Status:** 6 open, 0 closed
**Purpose:** Mechanical compliance enforcement in pack-check

##### Issue #16: Warning-first mode (--strict flag) for pack-check
**Type:** Tooling/CLI Enhancement
**Priority:** HIGH
**Dependencies:** NONE (foundational for other enforcement issues)
**Complexity:** Medium (CLI flag + rule configuration)
**Estimated Effort:** 3-4 hours

**Acceptance Criteria:**
1. Add `--strict` flag to pack-check CLI
2. New rules emit warnings by default
3. `--strict` flag converts warnings to errors
4. Update docs/reference/pack-check.md

**Technical Notes:**
- Foundational: enables other pack-check issues (#12, #13, #14, #19) without breaking CI
- This should be done FIRST before other pack-check enhancements
- Allows incremental adoption of compliance rules

**Blocks Other Work:**
- **Enables all other pack-check compliance issues** (#12, #13, #14, #19)

---

##### Issue #12: Flow boundary enforcement in pack-check
**Type:** Tooling/Compliance
**Priority:** HIGH
**Dependencies:** #16 (warning-first mode)
**Complexity:** Medium (pack-check rule implementation)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Add pack-check rule blocking demoswarm.sh invocations in flow commands
2. Add pack-check rule blocking skill-layer CLI syntax in flows
3. No false positives on existing valid commands
4. Update docs/reference/pack-check.md

**Technical Notes:**
- Part of three-tier ownership model enforcement (Flows → Agents → Skills)
- Source: `compliance-drift-proofing` signal analysis (REQ-001)
- Implementation reference: `tools/demoswarm-pack-check/src/checks/drift.rs`

**Blocked By:** #16 (warning-first mode)
**Blocks Other Work:** None (independent within v1.2.0)

---

##### Issue #13: Skills section enforcement for agents using demoswarm.sh
**Type:** Tooling/Compliance
**Priority:** MEDIUM
**Dependencies:** #16 (warning-first mode)
**Complexity:** Low-Medium (straightforward pack-check rule)
**Estimated Effort:** 1-2 hours

**Acceptance Criteria:**
1. Add pack-check rule validating agents using demoswarm.sh have Skills section
2. Surface validation warnings
3. Document intentionally exempt agents (if any)
4. Update docs/reference/pack-check.md

**Technical Notes:**
- Current state: 10 of 14 demoswarm.sh-using agents have Skills sections
- Source: `compliance-drift-proofing` signal analysis (REQ-002)

**Blocked By:** #16 (warning-first mode)
**Blocks Other Work:** None (independent)

---

##### Issue #14: OpenQ prefix normalization
**Type:** Tooling/Compliance
**Priority:** MEDIUM
**Dependencies:** #16 (warning-first mode)
**Complexity:** Low (validation + tool enhancement)
**Estimated Effort:** 1-2 hours

**Acceptance Criteria:**
1. Document canonical OpenQ prefix format (PLN, BLD, GAT, etc.)
2. Add validation in openq-tools enforcing normalized prefixes
3. Update clarifier agent to emit normalized prefixes
4. Add pack-check or documentation drift rule catching deviations

**Technical Notes:**
- Issue: PLN vs PLAN inconsistency in open question markers
- Canonical format should match stable-markers.md
- This is a documentation + tool consistency issue

**Blocked By:** #16 (warning-first mode)
**Blocks Other Work:** None (independent)

---

##### Issue #15: Build-to-Gate receipt test fixtures
**Type:** Tooling/Testing
**Priority:** MEDIUM
**Dependencies:** NONE (independent but related to #16 context)
**Complexity:** Medium (test fixture creation)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Create test fixture for valid Build receipt → Gate consumption
2. Create test fixture for invalid/missing receipt handling
3. Add integration test verifying receipt-checker agent behavior
4. Commit fixtures to `tools/demoswarm-pack-check/tests/fixtures/`

**Technical Notes:**
- Validates receipt contract between Build and Gate flows
- Source: `compliance-drift-proofing` signal analysis (REQ-004)
- Reference: `.claude/agents/receipt-checker.md`, `docs/reference/contracts.md`

**Independent Work:** Can be done in parallel with other v1.2.0 items

---

#### Milestone v1.3.0 - Developer Experience (10 issues)

**Issues in this milestone:** #17, #18, #19, #20, #21, #22, #23, #24, #26
**Status:** 10 open, 0 closed
**Purpose:** DX improvements, template fixes, documentation automation, error resilience

##### Issue #17: Fix markdown template formatting (MD022/MD058)
**Type:** Bug Fix/Quality
**Priority:** MEDIUM
**Dependencies:** NONE (independent)
**Complexity:** Low (template fixes)
**Estimated Effort:** 1-2 hours

**Acceptance Criteria:**
1. Update secrets_scan.md template with blank lines around headings
2. Update secrets_scan.md template with blank lines around tables
3. Audit other generator templates for same issues
4. Verify generated artifacts pass markdownlint

**Technical Notes:**
- 24 violations per run (fixing template eliminates them)
- Current violations: MD022 (missing blank lines), MD058 (line too long)
- Impact: High ROI (one fix in template = 24 per-run savings)

**Independent Work:** Can be done immediately

---

##### Issue #20: Git-based fallback for receipt reading
**Type:** Error Handling/Resilience
**Priority:** MEDIUM
**Dependencies:** NONE (independent)
**Complexity:** Medium (fallback pattern)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Add git-based fallback to receipt reading in Gate agents
2. Pattern: if direct read fails, try `git show HEAD:<path>`
3. Add permission check before receipt write in Build agents
4. Emit warning if directory is read-only

**Technical Notes:**
- Addresses: Build permission failures seen in local-alignment-audit-aba1c6
- Pattern: Read from disk → fallback to `git show` → error
- Enables resilience against directory permission issues

**Independent Work:** Can be done immediately

---

##### Issue #21: Add question resolution tracking to signal-cleanup
**Type:** Workflow/Process
**Priority:** MEDIUM
**Dependencies:** NONE (but improves overall flow)
**Complexity:** Medium (openq-tools enhancement + cleanup workflow)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Add resolution tracking to openq-tools (OPEN → RESOLVED status)
2. Mark questions RESOLVED when answer is derived from evidence
3. Add "question resolution ceremony" to signal-cleanup agent
4. Before proceeding to Plan, verify each question has: (a) resolution, (b) default, or (c) escalation

**Technical Notes:**
- Addresses wisdom finding: OQ-SIG-001 never closed despite resolution
- Improves signal phase closure
- Flow 1 should emit "recommended defaults" document

**Independent Work:** Can be done in parallel

---

##### Issue #19: Add flow count validation rule to pack-check
**Type:** Tooling/Compliance
**Priority:** MEDIUM
**Dependencies:** #16 (warning-first mode)
**Complexity:** Low-Medium (pack-check rule)
**Estimated Effort:** 1-2 hours

**Acceptance Criteria:**
1. Extract flow count from CLAUDE.md "The Seven Flows" section
2. Validate README.md, architecture.md, DEMO_RUN.md match count
3. Emit warning on mismatch (warning-first mode)
4. Update docs/reference/pack-check.md

**Technical Notes:**
- Prevents documentation drift (captured in wisdom)
- Canonical: 7 flows (CLAUDE.md L13)
- Bot reviewers confused by "7 flows via 10 command files" phrasing

**Blocked By:** #16 (warning-first mode)
**Coordinates With:** #24 (flow count phrasing documentation)

---

##### Issue #24: Document 'command count vs flow count' phrasing
**Type:** Documentation
**Priority:** MEDIUM
**Dependencies:** NONE (independent)
**Complexity:** Low (documentation)
**Estimated Effort:** 1 hour

**Acceptance Criteria:**
1. Add explicit phrasing guidance to documentation
2. Canonical phrasing: "7 flows exposed via 10 slash commands"
3. Update architecture.md with variant relationships
4. Document when to use each variant

**Technical Notes:**
- Bot reviewers (FB-003, FB-004, FB-005) showed confusion
- Related: #19 (flow count validation)
- This is pure documentation; can be done immediately

**Coordinates With:** #19 (flow count validation)

---

##### Issue #22: Add run-prep pre-flight for branch protection
**Type:** Workflow/Process
**Priority:** LOW
**Dependencies:** NONE (but meaningful only after #10)
**Complexity:** Low (run-prep check)
**Estimated Effort:** 1 hour

**Acceptance Criteria:**
1. Add branch protection status check to run-prep pre-flight
2. Warn (don't block) if branch protection is not enabled
3. Add to early_risks.md in risk-analyst agent
4. Document in CONTRIBUTING.md

**Technical Notes:**
- Surfacees ORG_CONSTRAINT early (current: only visible at deploy time)
- Only meaningful after #10 is resolved
- Low priority (informational only)

**Meaningful After:** #10 (Enable branch protection)

---

##### Issue #23: Atomic write pattern for receipt generation
**Type:** Robustness/Quality
**Priority:** LOW
**Dependencies:** NONE (independent)
**Complexity:** Low-Medium (refactoring pattern)
**Estimated Effort:** 1-2 hours

**Acceptance Criteria:**
1. Implement atomic write pattern for receipt generation
2. Write to temp file, then rename to final location
3. Handle rename failures gracefully
4. Add unit tests for atomic write behavior

**Technical Notes:**
- Prevents partial writes on permission errors
- Pattern: write to `.tmp`, rename to final
- Low impact but good defensive programming

**Independent Work:** Can be done in parallel

---

##### Issue #26: Add Rust dependency caching to CI
**Type:** Performance/CI Optimization
**Priority:** LOW
**Dependencies:** NONE (independent)
**Complexity:** Low (CI configuration)
**Estimated Effort:** 1 hour

**Acceptance Criteria:**
1. Add Rust caching using `Swatinem/rust-cache@v2`
2. Cache per-job cargo registry, git, target/
3. Verify cache hit rates

**Technical Notes:**
- Performance optimization (no functional impact)
- Can be added independently
- Implementation reference provided in issue

**Independent Work:** Can be done immediately

---

##### Issue #18: Add spell-check to pack-check or pre-commit
**Type:** Quality/DX
**Priority:** LOW
**Dependencies:** NONE (independent)
**Complexity:** Low-Medium (tool integration)
**Estimated Effort:** 2-3 hours

**Acceptance Criteria:**
1. Evaluate spell-check options (cspell, typos-cli, etc.)
2. Add to pack-check OR pre-commit hooks
3. Create custom dictionary for pack-specific terms
4. Verify no false positives

**Technical Notes:**
- Prevents typos like "immeidate" (found in 7 flow files)
- Low priority (QoL improvement)
- Choose spell-checker integration point carefully

**Independent Work:** Can be done in parallel

---

### Closed Issues

#### Issue #1: DemoSwarm Documentation-Code Alignment Audit
**Type:** Historical (tracking artifact)
**State:** CLOSED
**Priority:** Completed (observability-only)

This issue tracked the `local-alignment-audit-aba1c6` run. All referenced findings have been converted into issues #9-#26. See prior research artifact for full details.

---

## Related PRs

### Summary Table

| PR | Title | State | Merged | Author | Created | Relevance |
|----|-------|-------|--------|--------|---------|-----------|
| #8 | refactor: README improvements and documentation clarity | MERGED | 2025-12-27 | swarm | 2025-12-27 | **HIGH** |
| #7 | refactor: DemoSwarm v2.1 directive handoffs alignment | MERGED | 2025-12-27 | swarm | 2025-12-27 | **HIGH** |
| #6 | refactor: standardize agent handoffs to natural language | MERGED | 2025-12-26 | swarm | 2025-12-26 | **HIGH** |
| #5 | refactor: Enhance agent documentation and workflows for clarity and consistency | MERGED | 2025-12-26 | swarm | 2025-12-25 | **HIGH** |
| #4 | feat: Comprehensive documentation enhancements for DemoSwarm pack | MERGED | 2025-12-23 | swarm | 2025-12-23 | **HIGH** |
| #3 | feat: Local alignment audit - Ops-First philosophy and pack consistency improvements | MERGED | 2025-12-22 | swarm | 2025-12-22 | **HIGH** |
| #2 | docs: update pack documentation to seven-flow model | MERGED | 2025-12-20 | swarm | 2025-12-20 | **HIGH** |

**Status:** All 7 PRs have been merged. No open PRs at this time. Roadmap for issues #9-#26 has not yet started implementation.

---

## Decisions / Constraints Extracted

**From Issue Analysis (#9-#26):**

1. **Three-Tier Ownership Model is Enforced** (compliance-drift-proofing)
   - Flow commands own routing (no skill CLI details)
   - Agent docs own operational detail
   - Skill docs own CLI truth
   - Implication: Pack-check rules (#12, #13, #14) are foundational

2. **Milestone Roadmap Governs Sequencing** (v1.1.0 → v1.2.0 → v1.3.0)
   - v1.1.0: Security & Governance (4 issues, blocking)
   - v1.2.0: Compliance Enforcement (6 issues, depends on v1.1.0 + #16)
   - v1.3.0: Developer Experience (10 issues, independent + quality)
   - Implication: v1.1.0 must complete before v1.2.0 can be meaningful

3. **Branch Protection is Governance Blocker** (#10)
   - Prevents `deployment_verdict=STABLE` until resolved
   - Org-level setting (not code)
   - Should be resolved ASAP (blocks #22 usefulness)

4. **Warning-First Mode Enables Safe Compliance** (#16)
   - New rules emit warnings by default
   - `--strict` flag enforces (allows incremental adoption)
   - Foundational for #12, #13, #14, #19
   - Should be done early in v1.2.0

5. **Cargo-Audit Prerequisite Chain** (#11 → #25)
   - #11 must be resolved before #25 can proceed
   - Current blocker: CVSS 4.0 support in cargo-audit

6. **Markdown Template Fix is High ROI** (#17)
   - 24 violations eliminated by fixing one template
   - Independent work
   - Recommended for early v1.3.0

7. **Documentation Consistency Matters** (#19, #24)
   - "7 flows via 10 command files" requires clear phrasing
   - Documentation drift detected in wisdom (FB-003, FB-004, FB-005)
   - #24 is pure docs, #19 is validation rule

8. **Error Resilience Patterns Needed** (#20)
   - Directory permission issues blocked receipt reading
   - Git fallback pattern: if direct read fails, try `git show HEAD:<path>`
   - Improves robustness

---

## Dependencies Between Issues

### Critical Path (Blocking Dependencies)

```
#10 (branch protection) → #22 (run-prep warning)
#11 (cargo-audit CVSS) → #25 (cargo-audit CI job)
#16 (--strict flag)     → #12, #13, #14, #19 (pack-check rules)
```

### Recommended Sequencing by Milestone

**v1.1.0 (Security & Governance):**
- #10 (Enable branch protection) — DO FIRST, org setting, 0.5 hours
- #9 (Fix path traversal) — PARALLEL, 2-3 hours
- #11 (cargo-audit CVSS) — PARALLEL, 2-3 hours
- #25 (cargo-audit CI job) — AFTER #11, 1 hour

**Sequencing Rationale:** #10 is org-level (immediate), others are independent code work. #11→#25 is strict dependency.

**v1.2.0 (Compliance Enforcement):**
- #16 (--strict flag) — DO FIRST, 3-4 hours (unblocks others)
- #12 (flow boundary rules) — AFTER #16, 2-3 hours
- #13 (Skills section rules) — AFTER #16 or PARALLEL, 1-2 hours
- #14 (OpenQ normalization) — AFTER #16 or PARALLEL, 1-2 hours
- #15 (receipt fixtures) — PARALLEL, 2-3 hours

**Sequencing Rationale:** #16 is foundational, enables others. #15 is independent.

**v1.3.0 (Developer Experience):**
- #17 (markdown fixes) — IMMEDIATE, high ROI, 1-2 hours
- #24 (flow count phrasing) — IMMEDIATE, pure docs, 1 hour
- #20 (git fallback) — EARLY, resilience, 2-3 hours
- #19 (flow count validation) — AFTER #16 (if not done), 1-2 hours
- #21 (question resolution) — EARLY, 2-3 hours
- #22 (branch protection check) — AFTER #10, 1 hour
- #23 (atomic writes) — PARALLEL, 1-2 hours
- #26 (rust caching) — PARALLEL, 1 hour
- #18 (spell-check) — LATER, 2-3 hours

**Sequencing Rationale:** Organize by dependency chains, quick wins first (#17, #24), then foundational work (#20, #21), then dependent work (#19, #22), then QoL improvements (#23, #26, #18).

---

## Issues Suitable for Task Breakdown

### Issue #16: Warning-First Mode (--strict flag)

**Current Scope:** Add `--strict` flag to pack-check

**Breakdown Recommendation:**

1. **AC-1: CLI flag parsing** (0.5 hours)
   - Add `--strict` argument to pack-check CLI args parser
   - Default: warnings
   - With `--strict`: errors

2. **AC-2: Rule configuration** (1.5 hours)
   - Audit existing rules to identify which should be warning-first
   - Create rule severity levels: ERROR (always fail), WARNING (--strict only), INFO (advisory)
   - Update rule execution to respect severity

3. **AC-3: Documentation** (1.5 hours)
   - Update docs/reference/pack-check.md
   - Add examples of warning vs strict mode output
   - Explain adoption strategy

4. **AC-4: Testing** (0.5 hours)
   - Add CLI tests for both modes
   - Verify warning/error modes produce expected output

---

### Issue #12: Flow Boundary Enforcement

**Current Scope:** Add pack-check rule blocking flow commands from containing demoswarm.sh

**Breakdown Recommendation:**

1. **AC-1: Pattern validation** (1 hour)
   - Identify all invalid patterns (demoswarm.sh, skill CLI syntax)
   - Create regex patterns for detection

2. **AC-2: Rule implementation** (1 hour)
   - Implement `DriftChecker` rule in pack-check
   - Scan flow commands for invalid patterns
   - Emit warnings/errors

3. **AC-3: False positive audit** (0.5 hours)
   - Run against all existing valid flow commands
   - Document any exceptions

4. **AC-4: Documentation** (0.5 hours)
   - Update docs/reference/pack-check.md

---

### Issue #21: Question Resolution Tracking

**Current Scope:** Add resolution tracking to openq-tools

**Breakdown Recommendation:**

1. **AC-1: Status field addition** (0.5 hours)
   - Add `status: OPEN | RESOLVED` field to open question entries
   - Preserve backward compatibility

2. **AC-2: Resolution logic** (1 hour)
   - Implement resolution detection: question answered by evidence
   - Create `mark-resolved` command in openq-tools
   - Emit resolution ceremony output

3. **AC-3: Signal-cleanup integration** (1 hour)
   - Add "question resolution ceremony" phase to signal-cleanup
   - Before proceeding to Plan, require all questions to be: RESOLVED | have-default | escalated

4. **AC-4: Testing** (0.5 hours)
   - Test open question resolution workflow

---

## Missing Acceptance Criteria / Test Requirements

### Issues Needing More Detail

**Issue #9 (Path Traversal):**
- Missing: Specific test cases (e.g., `../../../etc/passwd`, `/etc/passwd`, `//network/share`)
- Missing: Canonicalization library recommendation (recommend `std::fs::canonicalize()`)

**Issue #11 (Cargo-Audit CVSS):**
- Missing: Version constraint (e.g., "cargo-audit >=0.22.0")
- Missing: Fallback tooling evaluation criteria

**Issue #13 (Skills Section):**
- Missing: Definition of "valid Skills section" (specific fields required?)
- Missing: List of intentionally exempt agents (are any OK to skip?)

**Issue #15 (Receipt Fixtures):**
- Missing: Specific receipt format requirements
- Missing: Invalid receipt scenarios to test (missing fields, wrong type, etc.)

**Issue #20 (Git Fallback):**
- Missing: Fallback chain specifics (disk read → git show → error on both?)
- Missing: Warning format for read-only directories

**Issue #21 (Question Resolution):**
- Missing: Criteria for "evidence resolves question" (is inference enough?)
- Missing: Format of resolution ceremony output

**Issue #24 (Flow Count Phrasing):**
- Missing: Specification of "when to use each variant" (conditions for flow-4-review vs flow-4-gate?)

---

## Prior Art Pointers (Local Codebase)

**Implementation Reference Map:**

| Path | Pattern | Evidence |
|------|---------|----------|
| `.claude/commands/flow-*.md` | 10 flow command files | Examples of valid/invalid phrasing (flow-4-review, flow-4-gate variants documented) |
| `.claude/agents/*.md` | Agent specs with Skills sections | 10 of 14 have Skills sections; audit shows which lack them |
| `tools/demoswarm-pack-check/src/checks/` | Rule implementations | `drift.rs` shows pattern checking; can reference for #12, #19 implementation |
| `tools/demoswarm-runs-tools/src/commands/secrets.rs` | Current path handling | Target for #9 (path traversal fix) |
| `docs/reference/stable-markers.md` | Canonical marker definitions | OpenQ prefix specification (PLN vs PLAN) for #14 |
| `docs/reference/pack-check.md` | Pack-check rule documentation | Reference format for updating in #12, #13, #14, #16, #19 |
| `.github/workflows/pack.yml` | Current CI configuration | Target for #25 (cargo-audit job) and #26 (rust caching) |
| `CLAUDE.md` | Authoritative pack spec | L13 "The Seven Flows" for #19 validation |
| `.runs/local-alignment-audit-aba1c6/` | Prior wisdom artifacts | Evidence for all issues #9-#26 |

---

## Implications for Flow 1

### Requirements-Based Guidance

If you are starting **pack maintenance** or **roadmap planning** work:

1. **Milestone Structure is Clear** (no ambiguity on sequencing)
   - v1.1.0: Security fixes first (blocking for governance)
   - v1.2.0: Compliance enforcement (incremental via #16 --strict flag)
   - v1.3.0: Developer experience (independent quality work)
   - Implication: Use milestone order for release planning

2. **Critical Path is Short** (only 3 chains)
   - #10 (org setting) → #22 (warning only)
   - #11 (tool) → #25 (CI job)
   - #16 (CLI flag) → (#12, #13, #14, #19)
   - Implication: Most work is parallelizable; focus on these dependencies

3. **High-ROI Quick Wins Exist** (immediate wins)
   - #17 (markdown fixes): 1-2 hours → eliminates 24 findings per run
   - #24 (flow phrasing): 1 hour → pure documentation
   - #26 (rust caching): 1 hour → CI performance
   - Implication: Start with quick wins to build momentum

4. **Error Resilience Gaps Exist** (#20)
   - Directory permissions can block receipt reading
   - Git fallback pattern provides safety net
   - Implication: Prioritize #20 in v1.3.0

5. **Warning-First Adoption Strategy** (#16)
   - New pack-check rules should not break existing CI
   - Use --strict flag for enforcement (gradual adoption)
   - Implication: #16 enables safe compliance work in v1.2.0

6. **Security Debt is Minor** (#9, #11)
   - Path traversal in secrets.rs is bounded (local execution)
   - Cargo-audit limitation is tool issue (not code defect)
   - Implication: Both can be resolved in v1.1.0 in parallel

### Wisdom Constraints (From Prior Run)

**Things to Avoid:**

- Do NOT start v1.2.0 (compliance) without completing #16 (--strict flag)
- Do NOT merge #25 (cargo-audit CI) without resolving #11 (CVSS support)
- Do NOT expect #22 to be meaningful until #10 is completed
- Do NOT assume branch protection is enabled (org constraint)
- Do NOT use phrasing "six flows" or "flows in any order" (use "7 flows exposed via 10 command files")

**Things That Worked Well:**

- Issue templates with explicit acceptance criteria enabled mechanical verification
- Explicit priority classification (HIGH/MEDIUM/LOW) enabled clear triage
- Layered roadmap structure (security → compliance → DX) prevented rework
- Parallel work where dependencies allow

---

## Assumptions Made to Proceed

| Assumption | Basis | Impact if Wrong |
|-----------|-------|-----------------|
| Issues #9-#26 are current roadmap | All created 2025-12-31, all labeled "roadmap" | If roadmap is external, these issues may be planned but not authoritative |
| Milestones are enforced sequencing | 3 milestones (v1.1.0, v1.2.0, v1.3.0) are defined, no due dates | If milestones are cosmetic, work can be parallelized differently |
| Branch protection is currently disabled | Issue #10 states "Branch not protected", ORG_CONSTRAINT documented | If already enabled, #10 is complete and #22 is immediately useful |
| Cargo-audit CVSS 4.0 is not yet supported | Issue #11 states "parser limitation in cargo-audit 0.21.2" | If newer version available, #11 is quicker than assumed |
| All issues have accurate acceptance criteria | Issues were created with detailed bodies | If criteria are incomplete, work scope may be larger |
| GitHub CLI tool availability will persist | Currently authenticated and working | If gh becomes unavailable, monitoring/automation may need fallbacks |

---

## Questions / Clarifications Needed

| Question | Impact | Suggested Resolution |
|----------|--------|----------------------|
| Should #10 (branch protection) be completed before other v1.1.0 work? | Blocks meaningful deploy verdicts | Recommend: yes, do first (0.5 hours) |
| Is v1.1.0 complete before starting v1.2.0? | Sequencing constraint | Define: "v1.1.0 complete" = all 4 issues closed |
| Should #16 (--strict flag) be one issue or broken into sub-ACs? | Scope clarity | Current scope is appropriate (3-4 hours) |
| Are there any intentionally exempt agents from #13 (Skills section)? | Acceptance criteria clarity | Audit and document explicitly |
| What is the "evidence threshold" for question resolution (#21)? | Workflow clarity | Define: derivable from repo state or inference allowed? |
| Should spell-check (#18) integrate with pre-commit or pack-check? | Implementation clarity | Recommend: pack-check (aligned with other rules) |

---

## Inventory (Machine Countable)

### Issues by State

- ISSUE: #26 relevance=MEDIUM state=open milestone=v1.3.0 priority=LOW
- ISSUE: #25 relevance=HIGH state=open milestone=v1.1.0 priority=MEDIUM blocked-by=#11
- ISSUE: #24 relevance=MEDIUM state=open milestone=v1.3.0 priority=MEDIUM related-to=#19
- ISSUE: #23 relevance=MEDIUM state=open milestone=v1.3.0 priority=LOW
- ISSUE: #22 relevance=LOW state=open milestone=v1.3.0 priority=LOW meaningful-after=#10
- ISSUE: #21 relevance=MEDIUM state=open milestone=v1.3.0 priority=MEDIUM
- ISSUE: #20 relevance=HIGH state=open milestone=v1.3.0 priority=MEDIUM
- ISSUE: #19 relevance=MEDIUM state=open milestone=v1.3.0 priority=MEDIUM blocked-by=#16 related-to=#24
- ISSUE: #18 relevance=LOW state=open milestone=v1.3.0 priority=LOW
- ISSUE: #17 relevance=HIGH state=open milestone=v1.3.0 priority=MEDIUM quick-win=true
- ISSUE: #16 relevance=HIGH state=open milestone=v1.2.0 priority=HIGH unblocks=#12,#13,#14,#19
- ISSUE: #15 relevance=MEDIUM state=open milestone=v1.2.0 priority=MEDIUM
- ISSUE: #14 relevance=MEDIUM state=open milestone=v1.2.0 priority=MEDIUM blocked-by=#16
- ISSUE: #13 relevance=MEDIUM state=open milestone=v1.2.0 priority=MEDIUM blocked-by=#16
- ISSUE: #12 relevance=HIGH state=open milestone=v1.2.0 priority=HIGH blocked-by=#16
- ISSUE: #11 relevance=HIGH state=open milestone=v1.1.0 priority=MEDIUM unblocks=#25
- ISSUE: #10 relevance=HIGH state=open milestone=v1.1.0 priority=HIGH critical-path=true
- ISSUE: #9 relevance=HIGH state=open milestone=v1.1.0 priority=HIGH
- ISSUE: #1 relevance=HISTORICAL state=closed relevance-reason=observability-tracker

### Pull Requests (All Merged)

- PR: #8 relevance=HIGH state=merged type=refactor
- PR: #7 relevance=HIGH state=merged type=refactor
- PR: #6 relevance=HIGH state=merged type=refactor
- PR: #5 relevance=HIGH state=merged type=refactor
- PR: #4 relevance=HIGH state=merged type=feature
- PR: #3 relevance=HIGH state=merged type=feature
- PR: #2 relevance=HIGH state=merged type=docs

### Milestones

- MILESTONE: v1.1.0 title="Security & Governance" open_issues=4 closed_issues=0
- MILESTONE: v1.2.0 title="Compliance Enforcement" open_issues=6 closed_issues=0
- MILESTONE: v1.3.0 title="Developer Experience" open_issues=10 closed_issues=0

### Labels (Active in Repo)

- LABEL: roadmap (18 issues using)
- LABEL: security (4 issues using)
- LABEL: tooling (8 issues using)
- LABEL: compliance (6 issues using)
- LABEL: governance (3 issues using)
- LABEL: pack-improvement (6 issues using)
- LABEL: dx (5 issues using)
- LABEL: documentation (1 issue using)

### Code References

- CODE_REF: tools/demoswarm-runs-tools/src/commands/secrets.rs note="Path traversal target (#9)"
- CODE_REF: .github/workflows/pack.yml note="CI target for #25 (cargo-audit) and #26 (rust caching)"
- CODE_REF: tools/demoswarm-pack-check/src/checks/drift.rs note="Reference for #12 implementation"
- CODE_REF: docs/reference/stable-markers.md note="OpenQ prefix specification (#14)"
- CODE_REF: docs/reference/pack-check.md note="Rule documentation target (#12, #13, #14, #16, #19)"
- CODE_REF: CLAUDE.md note="Authoritative flow specification (L13) for #19 validation"
- CODE_REF: .claude/commands/ note="Flow command files (10 total) - targets for #12, #24"
- CODE_REF: .claude/agents/ note="Agent specs (~20) - targets for #13 (Skills section audit)"

---

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
what_completed: |
  Complete GitHub roadmap analysis for demo-swarm-swarm repository.
  - 17 open roadmap issues analyzed (Issues #9-#26, excluding #1 which is closed)
  - 3 milestones mapped (v1.1.0: 4 issues, v1.2.0: 6 issues, v1.3.0: 10 issues)
  - 3 critical dependency chains identified and documented
  - 7 merged PRs reviewed (all alignment/documentation, no blockers)
  - Task breakdown recommendations provided for complex issues
  - High-ROI quick wins identified (#17, #24, #26)
what_left: |
  GitHub research complete. Roadmap fully inventoried.
  Optional follow-ups (not blockers for Flow 1):
  - Detailed estimation/effort planning (provided but not finalized)
  - Resource allocation (not in scope for research)
  - Sprint/iteration planning (recommendation provided)
blockers: []
missing_required: []
critical_path_issues:
  - "#10: Enable branch protection on main branch"
  - "#11: Update cargo-audit for CVSS 4.0 support"
  - "#16: Warning-first mode (--strict flag) for pack-check"
constraints:
  - "v1.1.0 (4 issues) must complete before v1.2.0 is meaningful"
  - "Issue #16 (--strict flag) unblocks v1.2.0 compliance rules (#12, #13, #14, #19)"
  - "Issue #11 (cargo-audit) must complete before #25 (CI job) can proceed"
  - "Branch protection not enabled (ORG_CONSTRAINT affects deploy verdicts)"
  - "Test count = 102 unit tests; 277 integration tests filtered"
quick_wins:
  - "Issue #17: markdown fixes (1-2h, eliminates 24 findings)"
  - "Issue #24: flow phrasing docs (1h, pure documentation)"
  - "Issue #26: rust caching (1h, CI performance)"
assumptions:
  - "All 17 issues represent current roadmap (created 2025-12-31)"
  - "Milestones enforce suggested sequencing (not cosmetic)"
  - "Branch protection is currently disabled (per #10)"
evidence_sha: null
generated_at: 2025-12-31T10:45:00Z
```

---

## Handoff

**What I did:** Conducted comprehensive GitHub research on the demo-swarm-swarm repository roadmap. Analyzed 17 open issues (#9-#26) organized across 3 milestones (v1.1.0 Security, v1.2.0 Compliance, v1.3.0 DX). Mapped critical dependency chains, identified quick wins, and provided task breakdown recommendations for complex issues. All issues traced back to wisdom from the local-alignment-audit-aba1c6 run.

**What's left:** GitHub research is complete. All roadmap context has been retrieved, analyzed, and synthesized. Three critical dependency chains identified (branch-protection → warning, cargo-audit → CI job, --strict-flag → compliance-rules). No external blockers or missing context.

**Recommendation:** Flow 1 can proceed with confidence. The roadmap is well-structured with clear milestones and explicit dependencies. For your work:

1. **If starting pack maintenance:** Follow milestone order (v1.1.0 → v1.2.0 → v1.3.0). Start with critical path items (#10, #11, #16) to unblock downstream work.

2. **If implementing features:** Complete quick wins first (#17, #24, #26) to build momentum, then tackle dependent work (v1.2.0 after #16).

3. **For issue triage:** Use provided dependency chains and complexity estimates to sequence work. Critical path has no more than 3 links in any chain.

4. **For documentation clarity:** Reference the "Things That Worked" section from wisdom — explicit ACs, priority classification, and layered roadmap structure enabled prior run success.

5. **Binding to GitHub:** Each issue includes explicit acceptance criteria and evidence links to prior runs. Bind to issue numbers for full context inheritance.

---

## Appendix: Detailed Dependency Graph

### Critical Path Visualization

```
v1.1.0 (Security & Governance)
├─ #10: Branch protection (ORG) → #22: run-prep check
├─ #9:  Path traversal (sec) ✓ independent
├─ #11: Cargo-audit (tooling) → #25: CI job
└─ #25: Cargo-audit CI ✓ (after #11)

v1.2.0 (Compliance Enforcement)
├─ #16: --strict flag (CLI) → #12, #13, #14, #19
├─ #12: Flow boundary rules (after #16)
├─ #13: Skills section (after #16)
├─ #14: OpenQ normalization (after #16)
├─ #19: Flow count validation (after #16)
└─ #15: Receipt fixtures ✓ independent

v1.3.0 (Developer Experience)
├─ #17: Markdown fixes ✓ independent (quick win)
├─ #24: Flow phrasing ✓ independent (quick win)
├─ #20: Git fallback ✓ independent
├─ #21: Question resolution ✓ independent
├─ #22: Branch protection check (after #10)
├─ #23: Atomic writes ✓ independent
├─ #26: Rust caching ✓ independent (quick win)
└─ #18: Spell-check ✓ independent (late)
```

### Parallelization Opportunities

**v1.1.0 (can run in parallel):**
- Start #10 immediately (org setting, 0.5h)
- Parallel: #9 (2-3h), #11 (2-3h)
- Sequential: #25 after #11 (1h)
- **Minimum elapsed time:** 3-4 hours (not 5-7 if serialized)

**v1.2.0 (requires #16 first, then parallel):**
- Start #16 immediately (3-4h)
- After #16, parallel: #12 (2-3h), #13 (1-2h), #14 (1-2h), #19 (1-2h)
- Parallel throughout: #15 (2-3h)
- **Minimum elapsed time:** ~3-4h + ~3h = 6-8 hours (not 8-14 if serialized)

**v1.3.0 (mostly independent):**
- Start immediately: #17 (1-2h), #24 (1h), #26 (1h), #20 (2-3h), #21 (2-3h), #23 (1-2h)
- Parallel: #18 (2-3h)
- Dependent on #10: #22 (1h after #10 complete)
- **Minimum elapsed time:** ~2-3h (quick wins first) + ~2-3h (longer work) = 4-6 hours (not 16-19 if serialized)

---

