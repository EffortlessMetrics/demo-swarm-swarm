# Sample Work Plan

This is an example of a work plan artifact produced by the work-planner agent in Flow 2 (Plan). It demonstrates task decomposition with proper marker syntax for mechanical counting.

---

## Work Plan: feat-session-timeout

**Run ID:** feat-session-timeout
**Created:** 2024-01-15T10:00:00Z
**ADR:** ADR-007 (Session Management Strategy)

---

## Overview

Implement configurable session timeout functionality with automatic cleanup of expired sessions.

**Total subtasks:** 8
**Estimated effort:** 2-3 days

---

## Implementation Tasks

### IMPL_001: SessionStore Protocol

**Priority:** P0 (Foundation)
**Estimated effort:** 2h
**Dependencies:** None

Define the abstract session storage interface to enable future backend swaps.

**Deliverables:**
- `src/auth/session_store.py` - Protocol definition
- Type hints for Session, SessionID, Timestamp

**Acceptance criteria:**
- AC-1: Protocol defines create, read, update, delete, cleanup operations
- AC-2: Backend-agnostic (no Redis/SQL-specific types)
- AC-3: Type-safe with full annotations

---

### IMPL_002: InMemorySessionStore

**Priority:** P0 (Foundation)
**Estimated effort:** 3h
**Dependencies:** IMPL_001

Implement the in-memory session store for MVP.

**Deliverables:**
- `src/auth/memory_store.py` - InMemorySessionStore class
- Thread-safe implementation with locks

**Acceptance criteria:**
- AC-1: Implements SessionStore protocol
- AC-2: Thread-safe for concurrent access
- AC-3: Cleanup method removes expired sessions atomically

**Notes:** Per OQ-BUILD-001, using in-memory for MVP with documented upgrade path to Redis.

---

### IMPL_003: SessionManager Core

**Priority:** P0 (Core Feature)
**Estimated effort:** 4h
**Dependencies:** IMPL_001, IMPL_002

Implement the main session manager with timeout logic.

**Deliverables:**
- `src/auth/session_manager.py` - SessionManager class
- Configuration integration for timeout values

**Acceptance criteria:**
- AC-1: Creates sessions with configurable TTL
- AC-2: Validates sessions against expiry timestamp
- AC-3: Refreshes session timestamp on access (if configured)
- AC-4: Integrates with config system for timeout values

**Hotspot:** This is the core business logic - review carefully.

---

### IMPL_004: Session Cleanup Middleware

**Priority:** P1 (Integration)
**Estimated effort:** 2h
**Dependencies:** IMPL_003

Add middleware to cleanup expired sessions on each request.

**Deliverables:**
- `src/auth/middleware.py` - SessionCleanupMiddleware
- Integration with request pipeline

**Acceptance criteria:**
- AC-1: Runs cleanup on configurable interval (not every request)
- AC-2: Non-blocking - does not delay request processing
- AC-3: Logs cleanup statistics at DEBUG level

**Notes:** Per OQ-BUILD-002, using per-request check for simplicity.

---

### IMPL_005: Configuration Schema

**Priority:** P1 (Integration)
**Estimated effort:** 1h
**Dependencies:** None

Add session timeout configuration options.

**Deliverables:**
- `src/config/session.py` - Session configuration schema
- Default values and validation

**Acceptance criteria:**
- AC-1: `session.timeout_seconds` with default 3600 (1 hour)
- AC-2: `session.cleanup_interval_seconds` with default 300 (5 min)
- AC-3: Validation for minimum values (60s timeout, 30s cleanup)

**Contract endpoint:** CE_CONFIG_SESSION

---

## Test Tasks

### TEST_001: Unit Tests - SessionStore

**Priority:** P0
**Estimated effort:** 2h
**Dependencies:** IMPL_001, IMPL_002

**Deliverables:**
- `tests/auth/test_session_store.py`

**Coverage targets:**
- COV_UNIT_STORE: 90%+ line coverage on session_store.py
- COV_UNIT_MEMORY: 90%+ line coverage on memory_store.py

**Scenarios:**
- SC-001: Create session stores timestamp correctly
- SC-002: Read expired session returns None
- SC-003: Cleanup removes only expired sessions
- SC-004: Concurrent access is safe

---

### TEST_002: Unit Tests - SessionManager

**Priority:** P0
**Estimated effort:** 3h
**Dependencies:** IMPL_003

**Deliverables:**
- `tests/auth/test_session_manager.py`

**Coverage targets:**
- COV_UNIT_MANAGER: 90%+ line coverage on session_manager.py

**Scenarios:**
- SC-005: New session has correct expiry
- SC-006: Expired session fails validation
- SC-007: Session refresh updates timestamp
- SC-008: Timeout respects configuration

**BDD mapping:** Maps to `features/session_timeout.feature`

---

### TEST_003: Integration Tests

**Priority:** P1
**Estimated effort:** 2h
**Dependencies:** IMPL_004, TEST_001, TEST_002

**Deliverables:**
- `tests/integration/test_session_flow.py`

**Coverage targets:**
- COV_INT_SESSION: End-to-end session lifecycle

**Scenarios:**
- SC-009: Full session lifecycle (create, use, expire, cleanup)
- SC-010: Middleware integrates correctly with request pipeline
- SC-011: Configuration changes take effect

---

## Documentation Tasks

### DOC_001: API Documentation

**Priority:** P2
**Estimated effort:** 1h
**Dependencies:** IMPL_003, IMPL_005

**Deliverables:**
- Docstrings on all public methods
- `docs/api/sessions.md` - API reference

---

## Task Dependency Graph

```
IMPL_001 (Protocol)
    |
    +---> IMPL_002 (MemoryStore)
    |         |
    |         +---> IMPL_003 (SessionManager)
    |                   |
    |                   +---> IMPL_004 (Middleware)
    |                   |
    |                   +---> TEST_002
    |
    +---> TEST_001

IMPL_005 (Config) ---> IMPL_003

TEST_001 + TEST_002 ---> TEST_003

IMPL_003 + IMPL_005 ---> DOC_001
```

---

## Risk Notes

| Task     | Risk                        | Mitigation                          |
| -------- | --------------------------- | ----------------------------------- |
| IMPL_003 | Race condition in cleanup   | Use atomic operations, add locks    |
| IMPL_004 | Performance impact          | Make cleanup async, batch deletions |
| TEST_003 | Flaky timing tests          | Use time mocking, avoid real delays |

---

## Marker Summary

**Implementation markers:**
- IMPL_001, IMPL_002, IMPL_003, IMPL_004, IMPL_005

**Test markers:**
- TEST_001, TEST_002, TEST_003

**Documentation markers:**
- DOC_001

**Coverage targets:**
- COV_UNIT_STORE, COV_UNIT_MEMORY, COV_UNIT_MANAGER, COV_INT_SESSION

**Contract endpoints:**
- CE_CONFIG_SESSION

---

## Handoff

**What I did:** Decomposed the session timeout feature into 8 subtasks with dependencies, acceptance criteria, and risk notes. Tasks are ordered by priority with foundation work first.

**What's left:** Implementation and testing. All tasks have clear acceptance criteria and dependencies.

**Recommendation:** Proceed to Build (Flow 3). Start with IMPL_001 (Protocol) and IMPL_005 (Config) in parallel since they have no dependencies. Then proceed through the dependency graph.
