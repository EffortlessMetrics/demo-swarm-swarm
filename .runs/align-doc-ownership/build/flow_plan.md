# Flow 3: Build for align-doc-ownership

## Planned Steps

- [x] run-prep (establish run directory)
- [x] repo-operator (git prep)
- [x] context-loader (load context)
- [x] clarifier (document ambiguities)
- [x] test-author / test-critic (microloop)
- [ ] code-implementer / code-critic (microloop)
- [ ] mutator / fixer (hardening)
- [ ] doc-writer (polish)
- [ ] self-reviewer (review)
- [ ] build-cleanup (write receipt, update index)
- [ ] repo-operator (stage changes)
- [ ] secrets-sanitizer (publish gate)
- [ ] build-cleanup reseal + repo-operator restage (if modified)
- [ ] repo-operator (commit - if gate passes)
- [ ] gh-issue-manager (update issue board)
- [ ] gh-reporter (post summary)

## Progress Notes

**2025-12-13T07:47**: Run infrastructure established. run-prep completed successfully.

- Run ID: align-doc-ownership
- Build directory: `.runs/align-doc-ownership/build/`
- Prior flows: signal, plan (both complete)
- Iteration: 3

**2025-12-13T07:48**: Context loaded and clarifier completed.

- Subtask ST-001 (Flow 1 Signal) selected for implementation
- Open questions resolved with defaults from user guidance
- Test specification complete

## Task Context

**ADR Decision**: OPT-002 (Pragmatic Enforcement)

- Add pack-check rules for major boundary violations
- Allow minimal inline examples in agent docs when skill docs have coverage gaps
- Normalize CLAUDE.md to summary-level entry point

**Subtasks (from work_plan.md)**:
| ID | Title | Status |
|----|-------|--------|
| ST-001 | Align Flow 1 (Signal) documentation | TODO |
| ST-002 | Align Flow 2 (Plan) documentation | TODO |
| ST-003 | Align Flow 3 (Build) documentation | TODO |
| ST-004 | Align Flow 4 (Gate) + cross-cutting enforcement + CLAUDE.md | TODO |
| ST-005 | Align Flow 5 (Deploy) documentation | TODO |
| ST-006 | Align Flow 6 (Wisdom) + validation run | TODO |

**Parallelization**: ST-001, ST-002, ST-003, ST-005 can run in parallel (distinct file sets).
ST-004 depends on ST-001-003. ST-006 depends on all others.

## Special Notes from User Input

The user has provided detailed guidance on two boundary violations that need enforcement:

1. **Skills named in flow commands** - Current REQ-001/AC-1 says "No flow command file contains ... direct skill-name invocations". Any text like "Computes counts mechanically (**via runs-derive skill**)..." is non-compliant. Replace with: "Computes counts mechanically (never estimates)."

2. **Agent output filenames in flow step lists** - Lines like "work-planner → `subtasks.yaml`" are drift magnets. Flow step lists should be "agent + purpose" only; file-path specifics belong in agent docs.

**Enforcement approach**:

- Flows: **no skill names** anywhere; use invariant wording ("computed mechanically; never estimates")
- Flows: **no agent output file paths** in step lists; step lists are agent + purpose only
- Agent docs: own output filenames/paths; if multiple artifacts, declare which is canonical vs derived
- pack-check: add narrow regex enforcement for both rules
