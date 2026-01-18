# Sample PR Description (Cockpit Format)

This is an example of a well-structured PR description. The PR description is the primary interface for reviewers.

---

## PR Brief

### What changed

- New `SessionManager` class handling timeout logic
- Config option `session.timeout_seconds` (default: 3600)
- Automatic cleanup of expired sessions on each request

### Why

User sessions currently persist indefinitely, creating a security risk. This change adds configurable timeouts with conservative defaults, following the ADR-007 decision to favor simplicity over background cleanup processes.

### Review map (hotspots)

- `src/auth/session_manager.py:45-89` - Core timeout logic (most important)
- `src/config/defaults.py:23-31` - New config schema with validation
- `tests/auth/test_session_timeout.py` - Verification coverage

### Quality events

- **Interface lock:** Config schema additive only; no API changes
- **Boundaries:** Single new module; no new dependencies
- **Verification depth:** 8 BDD scenarios added; mutation on diff: 94%
- **Security airbag:** No secrets; no vulnerable deps

### Quality Scorecard

| Surface         | Status   | Notes                              |
| --------------- | -------- | ---------------------------------- |
| Correctness     | measured | 8 BDD scenarios pass               |
| Verification    | measured | 94% mutation score on new code     |
| Boundaries      | clean    | config schema additive only        |
| Maintainability | noted    | 3 hotspots identified above        |
| Explanation     | complete | ADR-007 documents timeout strategy |

**Status values:** `measured` (automated), `partial` (some measured), `estimated` (inferred), `noted` (human observation), `clean` (no changes), `unknown` (gap documented)

### Proof (measured vs not measured)

- Gate: PASS (evidence: `.runs/feat-session-timeout/gate/gate_receipt.json`)
- Tests: 142/142 passed (evidence: `.runs/feat-session-timeout/build/test_execution.md`)
- Mutation: 94% on new code (evidence: `.runs/feat-session-timeout/build/mutation_report.md`)
- **Not measured:** Load testing under concurrent session expiry (documented in open questions)

### Reproduce

```bash
./scripts/gate.sh  # or: just gate
```

---

## Evidence Pointers

| Artifact        | Location                                              |
| --------------- | ----------------------------------------------------- |
| Test results    | `.runs/feat-session-timeout/build/test_execution.md`  |
| Code critique   | `.runs/feat-session-timeout/build/code_critique.md`   |
| Mutation report | `.runs/feat-session-timeout/build/mutation_report.md` |
| ADR             | `.runs/feat-session-timeout/plan/adr.md`              |
| Gate receipt    | `.runs/feat-session-timeout/gate/gate_receipt.json`   |

---

## Handoff

**What I did:** Implemented session timeout with configurable TTL. All acceptance criteria pass. Code critique found 2 MINOR issues (non-blocking).

**What's left:** Nothing blocking merge. MINOR code findings can be addressed in follow-up PR.

**Recommendation:** Route to gate for final checks. The 2 MINOR findings are documented but non-blocking per policy.
