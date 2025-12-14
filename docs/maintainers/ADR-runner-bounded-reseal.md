# ADR: Runner-bounded reseal loops

- **Status:** Accepted
- **Date:** 2025-12-14
- **Decision Makers:** Steven Zimmerman

## Context

Reseal cycles (`<cleanup> → secrets-sanitizer`) appear in Flows 1–6. Prior docs encoded a numeric cap (e.g., “at most 2 reseals”), which consumed agent context and created the risk of divergent limits between docs and the actual runner/CLI behavior.

## Decision

Move reseal loop bounding out of the flow docs and into the runner/CLI:

- Flow docs describe the convergence loop and safe-bail behavior but omit any numeric cap.
- The CLI enforces a bounded reseal count and emits the stop signal to the orchestrator/agents when the bound is reached.
- On runner-triggered stop: append an evidence note to `secrets_scan.md`, perform a local-only checkpoint (if `safe_to_commit: true`), and skip all GitHub operations for the run.

## Consequences

- Single source of truth for the cap (the CLI), eliminating doc/behavior drift.
- Agents do not spend context tokens on count limits; they just honor the runner’s stop signal.
- Future cap changes require only CLI config/release, not doc edits.
- Safe-bail remains consistent: evidence note → local-only checkpoint → no GH ops, leaving a clear audit trail.
