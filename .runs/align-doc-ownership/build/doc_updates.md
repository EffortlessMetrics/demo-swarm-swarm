# Documentation Updates for align-doc-ownership

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_agent: null
route_to_flow: null
docs_updated: 5
blockers: []
missing_required: []
```

## Documentation Changes Made

### Flow Commands (5 files)

The following flow commands were updated to remove skill plumbing references:

| File                                | Change                                            |
| ----------------------------------- | ------------------------------------------------- |
| `.claude/commands/flow-1-signal.md` | Removed "(via runs-derive skill—never estimates)" |
| `.claude/commands/flow-2-plan.md`   | Removed "(via runs-derive skill—never estimates)" |
| `.claude/commands/flow-3-build.md`  | Removed "(via runs-derive skill—never estimates)" |
| `.claude/commands/flow-4-gate.md`   | Removed "(via runs-derive skill—never estimates)" |
| `.claude/commands/flow-6-wisdom.md` | Removed "(via runs-derive skill—never estimates)" |

The new wording is: "Computes counts mechanically (never estimates)"

This change aligns with REQ-001: Flow commands must contain no skill plumbing.

### Pack-Check Documentation

The pack-check tool was updated with new boundary enforcement checks:

- Check 45: Flow Skill Plumbing boundary detection
- Check 46: Missing Skills section detection
- Check 47: Flow output path pattern detection (advisory)

These checks are self-documenting via their output messages.

## Documentation Not Changed

The following were explicitly NOT changed per the work plan scope:

- CLAUDE.md normalization (deferred to ST-004)
- Agent doc updates (deferred to ST-001 through ST-006 individual subtasks)
- Skill doc updates (not needed - skill docs are already complete)

## Verification

- All changed files pass pack-check
- No drift detected between documentation and implementation
- Changes are consistent with ADR OPT-002 (Pragmatic Enforcement)

## Inventory

- DOC_UPDATE: flow-1-signal.md
- DOC_UPDATE: flow-2-plan.md
- DOC_UPDATE: flow-3-build.md
- DOC_UPDATE: flow-4-gate.md
- DOC_UPDATE: flow-6-wisdom.md
