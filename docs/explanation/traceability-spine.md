# Traceability Spine

> From "why" to "done" — how requirements flow through the system.

---

## The Problem This Solves

Six months from now, someone asks: "Why does this code exist?"

Without traceability, you're stuck:
- Reading commit messages that say "fix auth"
- Grepping for comments that may not exist
- Hoping someone remembers the original discussion

With traceability, you follow the chain:

```
Problem -> Requirement -> BDD Scenario -> Test -> Code -> Critique -> Receipt -> Merge Decision
```

Each link points to the next. Each artifact references its source. The whole journey is auditable.

---

## The Traceability Chain

Every merged change should be traceable through these artifacts:

| Stage | Artifact | Location | Answers |
|-------|----------|----------|---------|
| Problem | `problem_statement.md` | `.runs/<id>/signal/` | Why are we doing this? |
| Requirements | `requirements.md` | `.runs/<id>/signal/` | What must be true? |
| BDD Scenarios | `*.feature` | `.runs/<id>/signal/features/` | How do we verify it? |
| Design | `adr.md`, `api_contracts.yaml` | `.runs/<id>/plan/` | How will we build it? |
| Tests | Test files | Project `tests/` dir | Does it work? |
| Code | Source files | Project `src/` dir | The implementation |
| Critiques | `*_critique.md` | `.runs/<id>/build/` | Is it good enough? |
| Receipt | `build_receipt.json` | `.runs/<id>/build/` | What was verified? |
| Decision | `merge_decision.md` | `.runs/<id>/gate/` | Should it ship? |

The chain runs forward (requirement drives code) and backward (code traces to requirement). Both directions are useful.

---

## Marker Threading

Requirements use stable markers (REQ-001, NFR-001). These markers thread through the entire chain:

```
REQ-001 (requirements.md)
  |
  v
Scenario: REQ-001 - User can log in (login.feature)
  |
  v
def test_user_login_req001(): (test_auth.py)
  |
  v
class LoginHandler: # Implements REQ-001 (auth.py)
  |
  v
"REQ-001: covered by 3 scenarios, 5 tests" (build_receipt.json)
  |
  v
"All REQs traced and verified" (merge_decision.md)
```

The marker `REQ-001` appears in every artifact. Grep finds all references. Impact analysis becomes mechanical.

### Marker Discipline

Markers must be:
- **Stable** — REQ-001 stays REQ-001 forever
- **Grep-able** — Anchored patterns, no wrapping
- **Referenced forward** — Downstream artifacts cite upstream markers

See [stable-markers.md](../reference/stable-markers.md) for the complete marker vocabulary.

---

## What Traceability Enables

### Impact Analysis

Change a requirement? Find all affected code and tests.

```bash
# Find everything that references REQ-003
grep -r "REQ-003" .runs/ src/ tests/
```

This returns:
- The original requirement definition
- Scenarios that verify it
- Tests that exercise it
- Code that implements it
- Receipts that recorded it

### Coverage Verification

Every REQ should have tests. Every test should trace to a REQ.

```bash
# Requirements defined
grep -c "^### REQ-" .runs/feat-auth/signal/requirements.md

# Requirements covered in tests
grep -c "REQ-" tests/
```

If the counts don't match, something is missing.

### Audit Trail

When REQ-001 breaks in production:

1. Find the merge that touched REQ-001
2. Find the build receipt for that run
3. Check what tests existed and what they verified
4. See the critique that approved it
5. See the gate decision that merged it

The chain explains how a bug got past verification. That explanation is how you prevent the next one.

### Regression Detection

When a test starts failing, the marker tells you which requirement is at risk:

```
FAILED test_user_login_req001
```

Immediately you know: REQ-001 is broken. Check what changed. Check if the requirement still applies. Fix the code or update the requirement.

---

## Traceability by Flow

Each flow adds links to the chain.

### Flow 1 (Signal)

**Input:** Issue, idea, bug report

**Output:**
- Problem statement (the "why")
- Requirements (REQ-001, NFR-001)
- BDD scenarios (reference REQs)

**Traceability established:**
- Each REQ/NFR gets a stable marker
- Scenarios cite the markers they verify
- The problem statement explains what drove the requirements

### Flow 2 (Plan)

**Input:** Requirements and scenarios

**Output:**
- ADR (design decisions)
- API contracts
- Work plan

**Traceability established:**
- ADR decisions reference REQs they support
- Contracts describe interfaces that implement REQs
- Work plan maps tasks to requirements

### Flow 3 (Build)

**Input:** Plan artifacts

**Output:**
- Code implementing the design
- Tests verifying the scenarios
- Critiques evaluating quality
- Receipt summarizing coverage

**Traceability established:**
- Tests cite scenarios/REQs in names or comments
- Code comments reference REQs where non-obvious
- Critiques reference specific issues with specific code
- Receipt summarizes which REQs were verified

### Flow 4 (Review)

**Input:** PR and feedback

**Output:**
- Review worklist (issues to fix)
- Fixes applied

**Traceability established:**
- Worklist items link to critique findings or PR comments
- Each fix traces to the feedback that requested it

### Flow 5 (Gate)

**Input:** Build receipt and evidence

**Output:**
- Merge decision (MERGE or BOUNCE)
- Gate receipt

**Traceability established:**
- Decision cites which REQs verified
- Decision cites which surfaces checked (security, coverage, etc.)
- Bounced decisions explain what's missing

### Flow 6 (Deploy)

**Input:** Merged code

**Output:**
- Deployment decision
- Verification report

**Traceability established:**
- Report confirms deployed SHA matches verified SHA
- Deployment events are timestamped and logged

### Flow 7 (Wisdom)

**Input:** All prior artifacts

**Output:**
- Learnings
- Feedback actions

**Traceability established:**
- Learnings cite specific runs, decisions, outcomes
- Feedback actions trace to the events that triggered them

---

## One Requirement's Journey

Here's how REQ-001 flows through the entire system:

```
+-------------------------------------------------------------+
| REQ-001: User can log in with email and password            |
| Location: .runs/feat-auth/signal/requirements.md            |
+-------------------------------------------------------------+
         |
         v
+-------------------------------------------------------------+
| Scenario: REQ-001 - Successful login                        |
|   Given a registered user                                   |
|   When they submit valid credentials                        |
|   Then they receive a session token                         |
| Location: .runs/feat-auth/signal/features/login.feature     |
+-------------------------------------------------------------+
         |
         v
+-------------------------------------------------------------+
| def test_successful_login():                                |
|     """Verifies REQ-001 - user receives session token"""    |
|     ...                                                     |
| Location: tests/test_auth.py:42                             |
+-------------------------------------------------------------+
         |
         v
+-------------------------------------------------------------+
| class LoginHandler:                                         |
|     """Implements REQ-001: authenticate and issue token"""  |
|     def authenticate(self, email, password): ...            |
| Location: src/auth/login.py:15                              |
+-------------------------------------------------------------+
         |
         v
+-------------------------------------------------------------+
| code_critique.md:                                           |
|   "LoginHandler is sound. Token expiry matches ADR-005."    |
| Location: .runs/feat-auth/build/code_critique.md            |
+-------------------------------------------------------------+
         |
         v
+-------------------------------------------------------------+
| build_receipt.json:                                         |
|   "REQ-001: covered by 3 scenarios, 5 tests, all passing"   |
| Location: .runs/feat-auth/build/build_receipt.json          |
+-------------------------------------------------------------+
         |
         v
+-------------------------------------------------------------+
| merge_decision.md:                                          |
|   Verdict: MERGE                                            |
|   "All requirements verified. REQ-001 fully traced."        |
| Location: .runs/feat-auth/gate/merge_decision.md            |
+-------------------------------------------------------------+
```

At each stage, you can follow the chain forward or backward. The marker threads everything together.

---

## Traceability Gaps Are Information

Perfect traceability is ideal. Imperfect traceability is reality.

When gaps exist, document them:

```markdown
## Coverage Gaps
- REQ-003 has no automated test (requires hardware integration)
  - Risk: MEDIUM
  - Mitigation: Manual verification documented in test_plan.md
- NFR-SECURITY-002 relies on external audit (not in-repo)
  - Risk: HIGH if audit expires
  - Mitigation: Ticket filed to integrate audit tooling
```

A gap you know about is safer than false confidence.

### Anti-Patterns

**Fake traceability:**
```markdown
REQ-001: Fully covered  # No pointer to evidence
```

**Gap hiding:**
```markdown
All requirements verified  # When some weren't
```

**Stale references:**
```markdown
See test_auth.py  # But test_auth.py was deleted
```

The value of traceability is accuracy. False traceability is worse than none because it creates unjustified confidence.

---

## Practical Patterns

### In Requirements

```markdown
### REQ-001: User authentication

**Description:** Users can authenticate with email and password.

**Acceptance Criteria:**
- AC-1: Valid credentials return a session token
- AC-2: Invalid credentials return 401

**Verification:** BDD scenarios in `login.feature`
```

### In Tests

```python
def test_valid_login_req001():
    """
    REQ-001/AC-1: Valid credentials return a session token.
    """
    result = login("user@example.com", "password123")
    assert result.token is not None
```

### In Code

```python
class LoginHandler:
    """
    Implements REQ-001: User authentication.

    Design: ADR-005 (JWT tokens with 24h expiry)
    """
    def authenticate(self, email: str, password: str) -> Session:
        ...
```

### In Receipts

```json
{
  "requirements_coverage": {
    "REQ-001": {"status": "covered", "tests": 5, "scenarios": 3},
    "REQ-002": {"status": "covered", "tests": 2, "scenarios": 1},
    "REQ-003": {"status": "partial", "tests": 0, "scenarios": 1, "note": "manual verification only"}
  }
}
```

---

## Why This Matters

Traceability isn't bureaucracy. It's the answer to:

- "Why did we build this?" (Follow chain backward to problem)
- "Is this tested?" (Follow chain forward to tests)
- "What broke?" (Follow marker to all references)
- "Can we change this?" (Impact analysis via grep)
- "Who decided this?" (ADR and merge decision)

Without traceability, these questions require archaeology. With it, they require grep.

---

## See Also

- [stable-markers.md](../reference/stable-markers.md) — Marker patterns for counting and tracing
- [run-state.md](../reference/run-state.md) — Where artifacts live
- [contracts.md](../reference/contracts.md) — Receipt schemas
- [claims-and-evidence.md](claims-and-evidence.md) — Evidence over trust
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
