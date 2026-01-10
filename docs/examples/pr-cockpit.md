# Sample PR Description (Cockpit Format)

This is an example of a well-structured PR description. The PR description is the primary interface for reviewers.

---

## Summary

Add user session timeout with configurable TTL.

## What Changed

- New `SessionManager` class handling timeout logic
- Config option `session.timeout_seconds` (default: 3600)
- Automatic cleanup of expired sessions

## Hotspots (review these)

- `src/auth/session_manager.py:45-89` - timeout logic
- `src/config/defaults.py:23` - new config key
- `tests/auth/test_session_timeout.py` - new test coverage

## Quality Scorecard

| Surface | Status | Notes |
|---------|--------|-------|
| Correctness | PASS | 8 BDD scenarios pass |
| Verification | PASS | 94% line coverage on new code |
| Boundaries | PASS | config schema updated |
| Maintainability | PASS | single new class, documented |
| Explanation | PASS | ADR-007 explains timeout strategy |

## Evidence Pointers

- Test results: `.runs/feat-session-timeout/build/test_execution.md`
- Code critique: `.runs/feat-session-timeout/build/code_critique.md`
- ADR: `.runs/feat-session-timeout/plan/adr.md`

## Not Measured

- Mutation testing (not configured for this repo)
- Load testing under concurrent session expiry
