# GitHub Research for align-doc-ownership

## Search Inputs

**Derived from run_meta.json:**
- `run_id`: align-doc-ownership
- `task_title`: Normalize language and ownership boundaries across DemoSwarm pack
- No linked GitHub issue (this is an internal alignment project)

**Search terms used:**
- "documentation alignment"
- "doc drift"
- "pack consistency"
- "CLAUDE.md"
- "flow command"
- "agent doc"
- "ownership"
- "boundary"
- "cleanup"

**Scope:**
- Primary: EffortlessMetrics/demo-swarm-staging (origin)
- Secondary: EffortlessMetrics/demo-swarm-dev (upstream)

---

## Access & Limitations

| Aspect | Status |
|--------|--------|
| GitHub CLI (`gh`) | Available and authenticated |
| Repository | EffortlessMetrics/demo-swarm-staging |
| Upstream | EffortlessMetrics/demo-swarm-dev |
| Search capability | Full (issues, PRs, commits) |
| Rate limits | No issues encountered |

**Note:** The staging repo has no issues or PRs directly. All issue/PR activity is on the upstream dev repo.

---

## Related Issues

| # | Title | State | Relevance |
|---|-------|-------|-----------|
| (none in staging repo) | - | - | - |

**Upstream repo (demo-swarm-dev) issues:**

No issues directly addressing "doc ownership" or "language normalization" were found. The issue tracker is primarily focused on Flow Studio UX work.

---

## Related PRs

| # | Title | State | Relevance |
|---|-------|-------|-----------|
| 48 | docs: v2.3.2 polish, evaluation guides, and repo organization | MERGED | High |
| 35 | docs: align UX manifest docs with implementation | MERGED | Medium |
| 16 | docs(runbooks): align template with example structure | MERGED | Medium |
| 25 | docs: normalize links and add proof-of-life to UX handover | MERGED | Low |

### PR Details

**PR #48 (High relevance):**
- Moved 25+ legacy docs to `docs/archive/`
- Added evaluation checklist, support docs, release notes
- Established pattern: archive stale docs rather than delete
- Precedent: reorganization is acceptable when it improves discoverability

**PR #35 (Medium relevance):**
- Aligned UX manifest docs with implementation
- Pattern: docs should match implementation state
- Precedent: updating docs to reflect actual vs aspirational state

**PR #16 (Medium relevance):**
- Aligned runbook template with example structure
- Pattern: templates should match real examples
- Precedent: DRY principle applies to doc structure

---

## Related Discussions

No discussions found. The repo does not appear to have GitHub Discussions enabled, or there are no discussions related to the search terms.

---

## Decisions / Constraints Extracted

From commit history and PR patterns:

1. **Commit be0c81a** (recent): "Update context-loader, mutator, and work-planner documentation for clarity and consistency"
   - Ongoing effort to improve doc clarity
   - Pattern: agent docs are being actively refined

2. **Commit 186ea53** (recent): "Improve documentation clarity and consistency across various flow and agent files"
   - Explicit focus on cross-cutting consistency
   - Signals this alignment work is a continuation, not a new initiative

3. **Commit b759005**: "enhance skill descriptions and add skill ownership checks for better clarity and enforcement"
   - Skill docs are expected to own CLI flag/contract truth
   - Enforcement mechanisms exist (pack-check)

4. **Commit 68f123e**: "Add doc drift check workflow and update documentation references"
   - Doc drift detection is already a concern
   - Tooling exists to catch inconsistencies

5. **Commit 963eb64**: "Standardize status sections across agent documentation with clear completion states"
   - Agent docs have already undergone standardization passes
   - Status sections have defined structure

---

## Prior Art Pointers (Local Codebase)

### Flow Commands (`.claude/commands/*.md`)
- 7 flow command files
- Currently orchestration-level (agent lists, station descriptions)
- Minimal skill invocation detail in flow commands (good alignment)

### Agent Docs (`.claude/agents/*.md`)
- 55 agent files
- Heavy operational detail in cleanup agents (build-cleanup, deploy-cleanup, etc.)
- Extensive `demoswarm.sh` invocation examples embedded
- Pattern: agents own operational/invocation specifics

### Skill Docs (`.claude/skills/*/SKILL.md`)
- 7 skill files
- Own CLI flag and contract truth
- Detailed command reference with examples
- Pattern: skills are single source of truth for CLI behavior

### CLAUDE.md
- High-level overview + policy
- Skills table is summary-level (command | purpose)
- Does not duplicate CLI flag details from skills

### Potential Overlaps/Inconsistencies Identified:

| Location | Content Type | Potential Issue |
|----------|--------------|-----------------|
| Agent cleanup docs | CLI invocation examples | May duplicate skill docs |
| CLAUDE.md Skills section | Command table | Could drift from skill docs |
| Agent docs | Status definitions | Already standardized (commit 963eb64) |

---

## Implications for Flow 1

### Constraints for Requirements

1. **Existing alignment precedent**: Recent commits (be0c81a, 186ea53) show active alignment work. This run should continue that trajectory, not contradict prior decisions.

2. **Skill ownership is established**: Skill docs own CLI truth (commit b759005). Requirements should reinforce, not change this boundary.

3. **Archive vs delete pattern**: PR #48 established archiving stale docs. If this run moves content, archiving may be preferred.

4. **pack-check exists**: Tooling for drift detection exists. Requirements could leverage this for enforcement.

### Risks from Prior Attempts

- No failed attempts found. Prior alignment work has been incremental and successful.

### Stakeholders

- Pack maintainers (implicit from commit authorship)
- Agents that reference skills (cleanup agents especially)
- Anyone reading CLAUDE.md as entry point

### "Do Not Repeat" Landmines

- Do not rename sections that pack-check relies on
- Do not remove machine-countable markers without updating checks
- Avoid duplicating CLI examples in multiple locations

---

## Assumptions Made to Proceed

1. This run is a continuation of alignment work, not a new direction
2. The "three-tier ownership" model (flow commands -> agents -> skills) is the intended architecture
3. CLAUDE.md is meant to be entry-point level, not deep reference
4. Cleanup agents legitimately need operational detail (they do the work)
5. No external dependencies block this alignment work

---

## Questions / Clarifications Needed

1. **Q: Should cleanup agents reference skill docs rather than embed examples?**
   - Current: agents have full CLI examples inline
   - Alternative: agents say "use runs-derive skill" and link to SKILL.md
   - Impact: reduces duplication but adds indirection

2. **Q: Is the CLAUDE.md Skills table meant to stay summary-level?**
   - Current: one-line per command
   - Alternative: expand with flag details
   - Recommendation: keep summary, defer to skill docs for flags

3. **Q: Should flow commands ever mention specific CLI flags?**
   - Current: flow commands don't mention flags
   - This appears intentional and should be preserved

---

## Inventory (machine countable)

- PR: #48 relevance=High state=merged
- PR: #35 relevance=Medium state=merged
- PR: #16 relevance=Medium state=merged
- PR: #25 relevance=Low state=merged
- CODE_REF: .claude/commands/*.md note=7 flow command files (orchestration level)
- CODE_REF: .claude/agents/*.md note=55 agent files (operational detail)
- CODE_REF: .claude/skills/*/SKILL.md note=7 skill files (CLI truth)
- CODE_REF: CLAUDE.md note=entry point + policy
- CODE_REF: docs/CUSTOMIZATION.md note=customization guidance

---

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - No direct GitHub issue linked to this alignment project
  - Prior alignment commits are recent (be0c81a, 186ea53) - ensure requirements don't contradict
```
