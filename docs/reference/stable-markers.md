# Stable Markers Reference

> Marker prefixes used for mechanical counting in receipts.

---

## Why markers exist

Receipts derive counts mechanically (grep/wc/parse). Stable markers make this possible without:
- Parsing prose
- Guessing structure
- Recomputing from source

**Rule:** If you need a count, add a marker to the producer artifact.

---

## Marker conventions

- **Single-line, anchored:** Markers must be on their own line, anchored at start
- **No wrapping:** Don't wrap marker lines (breaks grep patterns)
- **Prefix-based:** Counts are derived by counting prefix occurrences

---

## Requirements markers

**Producer:** `requirements-author`

| Marker | Pattern | Example |
|--------|---------|---------|
| Requirement | `### REQ-###` | `### REQ-001` |
| Acceptance criteria | `- AC-#:` | `- AC-1: User can log in` |
| NFR | `### NFR-<DOMAIN>-###` | `### NFR-SECURITY-001` |
| Metric | `- MET-#:` | `- MET-1: Response time < 200ms` |

---

## Risk markers

**Producer:** `scope-assessor`, `risk-analyst`

| Marker | Pattern | Example |
|--------|---------|---------|
| Risk | `RSK-### [SEVERITY] [CATEGORY]` | `RSK-001 HIGH SECURITY` |

Severity: `CRITICAL | HIGH | MEDIUM | LOW`
Category: `SECURITY | COMPLIANCE | DATA | PERFORMANCE | OPS`

---

## Open questions markers

**Producer:** `clarifier`

| Marker | Pattern | Example |
|--------|---------|---------|
| Question ID | `[QID-<flow>-###]` | `[QID-SIG-001]` |

Flow prefixes: `SIG`, `PLN`, `BLD`, `GAT`, `DEP`, `WIS`

---

## Inventory markers (Plan artifacts)

**Producer:** Various Plan agents

| Marker | Pattern | Example |
|--------|---------|---------|
| Implementation item | `IMPL_*:` | `IMPL_ENDPOINT:` |
| Documentation item | `DOC_*:` | `DOC_README:` |
| Contract item | `CE_*:` | `CE_API_HEALTH:` |
| Coverage item | `COV_*:` | `COV_UNIT_HEALTH:` |
| Dependency | `DEP_*:` | `DEP_AUTH_LIB:` |

---

## Wisdom markers

**Producer:** `regression-analyst`, `learning-synthesizer`, `feedback-applier`

| Marker | Pattern | Example |
|--------|---------|---------|
| Regression | `^### REG-[0-9]{3}:` | `### REG-001: Performance degradation` |
| Learning | `^## Learning: ` | `## Learning: Caching improves latency` |
| Action | `^- ISSUE: ` | `- ISSUE: Add rate limiting` |

**Note:** These patterns are anchored (`^`) and require exact casing.

---

## BDD markers

**Producer:** `bdd-author`

| Marker | Pattern | Example |
|--------|---------|---------|
| Feature file | `*.feature` | `login.feature` |
| Scenario | `Scenario:` | `Scenario: User logs in successfully` |

---

## Test markers

**Producer:** `test-author`, `test-strategist`

| Marker | Pattern | Example |
|--------|---------|---------|
| Test type | `TYPE_*:` | `TYPE_UNIT:` |
| Coverage target | `TARGET_*:` | `TARGET_90_PERCENT:` |

---

## Adding new markers

When you need a new count:

1. **Add marker to producer:** Have the authoring agent emit the marker
2. **Document the pattern:** Add it to this reference
3. **Update cleanup grep:** Adjust cleanup agent's count derivation
4. **Update pack-check:** If it's a contract marker

**Don't:**
- Invent ad-hoc patterns in cleanup agents
- Parse prose to extract counts
- Rely on heading structure alone

---

## Counting examples

Use the `demoswarm` CLI via shim (preferred):

```bash
# Count requirements
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/<run-id>/signal/requirements.md" \
  --regex '^### REQ-' \
  --null-if-missing

# Count regressions
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/<run-id>/wisdom/regression_report.md" \
  --regex '^### REG-[0-9]{3}:' \
  --null-if-missing

# Count open questions
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/<run-id>/signal/open_questions.md" \
  --regex '\[QID-' \
  --null-if-missing

# Count BDD scenarios
bash .claude/scripts/demoswarm.sh count bdd \
  --dir ".runs/<run-id>/signal/features" \
  --null-if-missing
```

The shim provides null-safety: missing file → `null`, no matches → `0`.

---

## See also

- [contracts.md](contracts.md) — Control-plane blocks
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
