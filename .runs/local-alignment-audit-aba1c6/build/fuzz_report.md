# Fuzz Report

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
counts:
  crashes: null
budget_seconds: null
duration_seconds: 0
fuzz_command: null

## Run Notes
- Tool/config selection: No `demo-swarm.config.json` found in repo root; fuzz harness not configured. Fuzzing skipped.
- Exit status: null (skipped)
- Limits: Fuzzing is only applicable when a fuzz harness is explicitly configured via `demo-swarm.config.json` with `fuzz.command` and optionally `fuzz.budget_seconds`. This run is a documentation-only audit (no production code changes).

## Crash Worklist (prioritized)
None. Fuzzing was not executed.

## Inventory (machine countable)
- FUZZ_CRASH: 0
- FUZZ_HARNESS_CONFIGURED: false
