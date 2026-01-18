# Sample Code Critique

This is an example of a code critique artifact produced by the code-critic agent. Note the severity markers (`[CRITICAL]`, `[MAJOR]`, `[MINOR]`, `[SUGGESTION]`) which are stable markers that can be counted mechanically.

---

## Code Critique: feat-session-timeout

## Summary

Implementation is mostly sound. Found one critical issue requiring fix before merge, one major issue that should be addressed, two minor issues, and one suggestion.

**Severity counts:** 1 CRITICAL, 1 MAJOR, 2 MINOR, 1 SUGGESTION

---

## Findings

### [CRITICAL] SQL injection vulnerability in session lookup

**File:** `src/auth/session_manager.py:52-54`

```python
# Current (vulnerable)
def get_session(self, sid: str) -> Session:
    query = f"SELECT * FROM sessions WHERE id = '{sid}'"
    return self.db.execute(query).fetchone()
```

**Issue:** Session ID is interpolated directly into SQL query without parameterization.

**Impact:** Attacker can inject arbitrary SQL via session ID cookie.

**Required fix:** Use parameterized query:

```python
query = "SELECT * FROM sessions WHERE id = ?"
return self.db.execute(query, (sid,)).fetchone()
```

**Blocks merge:** Yes - security vulnerability

---

### [MAJOR] Race condition in concurrent session cleanup

**File:** `src/auth/session_manager.py:78-85`

```python
def cleanup_expired(self):
    expired = self.find_expired_sessions()  # Query
    for session in expired:
        self.delete_session(session.id)     # Separate delete
```

**Issue:** Time-of-check to time-of-use race. Session could be refreshed between `find_expired_sessions()` and `delete_session()`.

**Impact:** Under concurrent load, a valid session could be incorrectly deleted.

**Suggested fix:** Use atomic DELETE with timestamp condition:

```python
def cleanup_expired(self):
    self.db.execute(
        "DELETE FROM sessions WHERE expires_at < ?",
        (datetime.utcnow(),)
    )
```

**Risk if deferred:** Medium - data loss possible under concurrent load

---

### [MINOR] Logging could include session ID

**File:** `src/auth/session_manager.py:67`

```python
logger.info("Session expired")
```

**Issue:** Log message doesn't include session ID for debugging.

**Suggestion:** Add session ID: `logger.info(f"Session {sid} expired")`

**Risk:** Low - debugging convenience only

---

### [MINOR] Config validation missing upper bound

**File:** `src/config/defaults.py:23-27`

```python
SESSION_CONFIG = {
    "timeout_seconds": {
        "type": "int",
        "min": 60,
        # No max defined
    }
}
```

**Issue:** `timeout_seconds` has no maximum, could be set to years.

**Suggestion:** Add reasonable max (e.g., 86400 for 24h):

```python
"timeout_seconds": {"type": "int", "min": 60, "max": 86400}
```

**Risk:** Low - operator error, not security

---

### [SUGGESTION] Consider lazy cleanup

**File:** `src/auth/session_manager.py:82`

**Observation:** Cleanup runs on every request via middleware.

**Alternative:** Background task or lazy cleanup on access would reduce per-request overhead.

**Trade-off:** Current approach is simpler, fine for moderate load (<1000 concurrent sessions). Refactor if scaling becomes a concern.

---

## Verdict

**Cannot merge until CRITICAL is fixed.** The SQL injection must be addressed before this code ships.

MAJOR race condition should ideally be fixed in this PR but could be deferred with explicit risk acceptance. MINOR issues are non-blocking.

---

## Evidence Pointers

| Finding                   | File:Line                           | Severity   |
| ------------------------- | ----------------------------------- | ---------- |
| SQL injection             | `src/auth/session_manager.py:52-54` | CRITICAL   |
| Race condition            | `src/auth/session_manager.py:78-85` | MAJOR      |
| Missing session ID in log | `src/auth/session_manager.py:67`    | MINOR      |
| Config validation gap     | `src/config/defaults.py:23-27`      | MINOR      |
| Cleanup strategy          | `src/auth/session_manager.py:82`    | SUGGESTION |

---

## Handoff

**What I did:** Reviewed session timeout implementation against security and correctness criteria. Found 1 CRITICAL (SQL injection), 1 MAJOR (race condition), 2 MINOR, 1 SUGGESTION.

**What's left:** CRITICAL must be fixed. MAJOR should be fixed. MINOR/SUGGESTION are optional.

**Recommendation:** Route to **fixer** for the SQL injection fix (mechanical, well-defined). The fixer should also address the race condition if time permits. After fixes land, re-run code-critic to verify the CRITICAL is resolved before proceeding to gate.
