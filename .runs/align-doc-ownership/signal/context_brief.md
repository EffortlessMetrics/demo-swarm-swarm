# Context Brief

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: problem-framer
route_to_flow: 1
blockers: []
missing_required: []
notes:
  - keywords searched: "pack-check", "doc-drift", "runs-derive", "demoswarm.sh"
  - exclusions applied: ".runs/, .git/"
  - Found substantial existing infrastructure for the work

## Related Runs (best-effort)
- No prior runs found with similar task titles or matching keywords.

## Likely Code Touch Points (best-effort)

### Flow Commands (primary targets)
- `.claude/commands/flow-1-signal.md` -- Flow 1 orchestration; check for skill plumbing
- `.claude/commands/flow-2-plan.md` -- Flow 2 orchestration; check for skill plumbing
- `.claude/commands/flow-3-build.md` -- Flow 3 orchestration; check for skill plumbing
- `.claude/commands/flow-4-gate.md` -- Flow 4 orchestration; ST-004 includes cross-cutting enforcement
- `.claude/commands/flow-5-deploy.md` -- Flow 5 orchestration; check for skill plumbing
- `.claude/commands/flow-6-wisdom.md` -- Flow 6 orchestration; check for skill plumbing

### Agent Docs (55 files to audit for consistency)
- `.claude/agents/*.md` -- All 55 agent docs need audit for:
  - status/recommended_action enum consistency
  - "write exactly N files" rules
  - Skills section presence and naming

### Skill Docs (source of CLI truth)
- `.claude/skills/runs-derive/SKILL.md` -- Read-only derivations skill
- `.claude/skills/runs-index/SKILL.md` -- Index write operations
- `.claude/skills/openq-tools/SKILL.md` -- Open questions tooling
- `.claude/skills/secrets-tools/SKILL.md` -- Secrets scanning/redaction
- `.claude/skills/test-runner/SKILL.md` -- Test execution
- `.claude/skills/auto-linter/SKILL.md` -- Formatting and linting
- `.claude/skills/policy-runner/SKILL.md` -- Policy-as-code checks

### Pack-level Documentation
- `CLAUDE.md` -- Needs cleanup to be "table of contents + quick start", not deep reference

### Validation Infrastructure
- `scripts/check-doc-drift.sh` -- Existing drift checker; may need new boundary checks
- `tools/demoswarm-pack-check/` -- Pack-check tool; may need new drift rules
- `docs/maintainers/validation-log.md` -- Must record final Toy Run A/B

## Docs / Prior Art
- `docs/maintainers/validation-log.md` -- Existing validation log structure with example entries
- `scripts/check-doc-drift.sh` -- Existing drift checks: stale skill names, old CLI flags, required docs
- `tools/demoswarm-pack-check/src/checks/drift.rs` -- Rust-based drift checking implementation

## Risks Spotted Early (non-binding)
- **Scope creep risk (inference)**: 55 agent files + 6 flow files + 7 skill files is substantial; the subtask partitioning helps but merge conflicts remain possible
- **pack-check extension risk (inference)**: Adding new boundary-enforcement checks to pack-check may require Rust development in the tool crate
- **Validation dependency (inference)**: The Toy Run A/B requirement means the work cannot be considered complete until flows 1-4 execute successfully end-to-end
- **Cross-cutting enforcement concentration (inference)**: ST-004 is heavier than others (owns Gate + cross-cutting + CLAUDE.md); may need more time
