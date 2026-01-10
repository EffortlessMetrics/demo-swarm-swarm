# pack-check Reference

> Structural validation for the pack.

---

## Canonical invocation

All pack validation flows through the shim. Use these two forms:

**Human / logs / CI:**

```bash
bash .claude/scripts/pack-check.sh --no-color
```

**Machine routing / artifacts:**

```bash
bash .claude/scripts/pack-check.sh --format json
```

This is the **single source of truth** for pack portability and contract drift.

---

## What pack-check validates

### Structure

- Agent files exist in `.claude/agents/`
- Command files exist in `.claude/commands/`
- Skill files exist in `.claude/skills/`
- Required frontmatter fields present

### Enums

- Status axis: `VERIFIED | UNVERIFIED | CANNOT_PROCEED`
- Recommended action: `PROCEED | RERUN | BOUNCE | FIX_ENV`
- No bespoke status strings

### Machine Summary blocks

- Presence in critics and verifiers
- Correct heading casing (`## Machine Summary`)
- Required fields present

### Control-plane blocks

- Gate Result schema includes `modified_files`
- Repo Operator Result schema complete
- PACK-CONTRACT markers present

### Behavioral rules

- No "See CLAUDE.md > ..." substitutions in flow commands
- No hardcoded paths (e.g., `src/`, `tests/`)
- Routing fields use correct names (`route_to_agent`, not `route_to`)

### Mechanical operations enforcement

- Cleanup agents must use `demoswarm.sh` shim for mechanical ops
- No bespoke `grep|sed|awk|jq` pipelines in cleanup agents
- Ownership boundaries: `runs-index` for index writes, `secrets-tools` for secrets ops, `openq-tools` for questions

See [demoswarm-cli.md](demoswarm-cli.md) for the shim surface and command contracts.

### Flow boundary enforcement (check 52)

- Flow commands must NOT contain `demoswarm.sh` invocations
- Flow commands must NOT contain skill CLI subcommands (count, ms, yaml, index, receipt, receipts, openapi, line, inv, time, openq, secrets)
- Enforces three-tier ownership: flow commands delegate to agents; agents use skills

**Violations reported as warnings** (not errors). Use `--strict` to elevate to errors.

### OpenQ prefix validation (check 53)

- QID patterns in `.runs/**/open_questions.md` must use canonical flow codes
- Valid format: `OQ-<FLOW>-<NNN>` where:
  - `<FLOW>` is one of: SIG, PLAN, BUILD, REVIEW, GATE, DEPLOY, WISDOM
  - `<NNN>` is a three-digit zero-padded number (001-999)
- Reports warnings for non-canonical codes (PLN instead of PLAN, BLD instead of BUILD)

**Violations reported as warnings** (not errors). Use `--strict` to elevate to errors.

### Wisdom markers

- Regression pattern: `^### REG-[0-9]{3}:`
- Single-line, anchored

---

## Expected output

Green run:

```
[PASS] Agent structure valid
[PASS] Command structure valid
[PASS] Status enums canonical
[PASS] Machine Summary blocks present
[PASS] Gate Result schema valid
...
All checks passed.
```

Red run:

```
[FAIL] Missing Machine Summary in: design-critic.md
[FAIL] Non-canonical status: "BLOCKED" in flow-3-build.md
...
3 errors, 1 warning
```

---

## JSON output schema

When using `--format json`, pack-check outputs:

```json
{
  "schema_version": 1,
  "repo_root": "/path/to/repo",
  "errors": 2,
  "warnings": 1,
  "counts": {
    "agents": 52,
    "commands": 7,
    "skills": 4
  },
  "diagnostics": [
    {
      "level": "fail",
      "check_id": 3,
      "check_title": "Checking Machine Summary blocks...",
      "message": "Missing Machine Summary in: design-critic.md"
    }
  ]
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `schema_version` | `number` | Always `1` (for future compatibility) |
| `repo_root` | `string` | Absolute path to validated repo |
| `errors` | `number` | Count of failures |
| `warnings` | `number` | Count of warnings |
| `counts.agents` | `number` | Number of agent files found |
| `counts.commands` | `number` | Number of command files found |
| `counts.skills` | `number` | Number of skill files found |
| `diagnostics[]` | `array` | Non-pass diagnostics only |
| `diagnostics[].level` | `"warn" \| "fail"` | Severity |
| `diagnostics[].check_id` | `number` | Check number (matches text output) |
| `diagnostics[].check_title` | `string` | Check section title |
| `diagnostics[].message` | `string` | Specific diagnostic message |

**Interpretation:**

- Exit code is authoritative: `0` = pass, non-zero = fail
- Use `errors` and `warnings` for routing decisions
- `diagnostics[]` contains only failures/warnings (passes filtered out)
- Summarize results; do not paste full output into artifacts

---

## CI integration

`.github/workflows/pack.yml` runs:

```yaml
- name: Validate pack
  run: bash .claude/scripts/pack-check.sh --no-color
```

CI validates:
- Pack structure and contracts
- Agent frontmatter
- Optional: targeted scripts

CI does **not** validate:
- Your target repo's build/tests
- Actual flow execution

---

## Targeted scripts (optional)

Additional scripts for focused checks:

```bash
# Frontmatter validation only
python scripts/lint_frontmatter.py

# Portability check only
python scripts/check_portable_claude.py
```

These are supplements, not replacements for pack-check.

---

## When validation fails

### Contract drift

**Symptom:** pack-check fails on enum or schema check.

**Fix:** Update the drifted file to use canonical values. Don't add new values without updating pack-check.

### Missing markers

**Symptom:** pack-check fails on marker presence.

**Fix:** Add the required marker/section to the agent or command file.

### Heading casing

**Symptom:** `Machine Summary Block` vs `Machine Summary` mismatch.

**Fix:** Use exact casing as defined in contracts.

### Flow boundary violation (check 52)

**Symptom:** pack-check warns about `demoswarm.sh` or skill CLI subcommands in flow commands.

**Fix:** Move the CLI invocation to the appropriate agent. Flow commands should delegate to agents (e.g., call `context-loader` or `build-cleanup`), not invoke skill-layer CLI directly.

### OpenQ prefix invalid (check 53)

**Symptom:** pack-check warns about non-canonical QID flow codes.

**Fix:** Update QIDs to use canonical flow codes:
- `SIGNAL` -> `SIG`
- `PLN` -> `PLAN`
- `BLD` -> `BUILD`
- `REV` -> `REVIEW`
- `GAT` -> `GATE`
- `DEP` -> `DEPLOY`
- `WIS` -> `WISDOM`

---

## Adding new checks

When adding a pack contract:

1. Add the check to `tools/demoswarm-pack-check/src/checks/`
2. Document the contract in `CLAUDE.md`
3. Update relevant agent/command files
4. Rebuild: `cargo install --path tools/demoswarm-pack-check --root .demoswarm`
5. Verify with `bash .claude/scripts/pack-check.sh --no-color`

---

## See also

- [pack-check-scope.md](pack-check-scope.md) — What pack-check enforces vs refuses to enforce
- [contracts.md](contracts.md) — What pack-check validates
- [stable-markers.md](stable-markers.md) — Marker patterns
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
