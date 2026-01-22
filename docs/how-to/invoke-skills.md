# How to Invoke Skills

This guide explains how to invoke DemoSwarm skills correctly, including the invocation contracts, fallback behavior, and common patterns.

## The Shim Pattern

Skills are invoked through a central shim script that handles implementation resolution:

```bash
bash .claude/scripts/demoswarm.sh <command> [options]
```

**Never invoke skill implementations directly.** The shim provides:

- Consistent resolution across environments
- Automatic fallback to available implementations
- Stable behavior regardless of PATH configuration

## Resolution Order

The shim resolves implementations in this order:

1. **Rust binary (repo-local):** `.demoswarm/bin/demoswarm` - Preferred
2. **Rust binary (global):** `demoswarm` on PATH
3. **Cargo run (dev):** Only if `tools/demoswarm-runs-tools` exists
4. **Python fallback (legacy):** `.claude/skills/runs-derive/fallback/runs_tools.py`

If no implementation is found, the shim prints installation instructions and exits with code 1.

## Contract Principles

All skill commands follow these principles:

| Principle | Rule |
|-----------|------|
| **Null over guess** | Missing file/pattern returns `null`, not `0` |
| **Single scalar stdout** | Each command prints exactly one value |
| **Exit code 0** | Always exit 0 (errors expressed via `null`) |
| **No git/GitHub** | These helpers never touch git or gh |
| **Repo-root paths** | All paths are repo-root-relative |

### Why Exit Code 0?

Skills always exit 0 because:

- Errors are expressed in stdout (`null`, status strings)
- This allows shell scripts to continue without `set -e` failures
- Callers parse stdout, not exit codes

For debugging, set `DEMOSWARM_STRICT=1` to get non-zero exit codes on errors. Note: The shim unsets this variable to keep agent behavior stable.

## Command Families

Skills are organized into command families by purpose:

### Counting and Extraction (runs-derive)

For reading data from `.runs/` artifacts:

```bash
# Count patterns
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/feat-auth/signal/requirements.md" \
  --regex '^### REQ-'

# Count BDD scenarios
bash .claude/scripts/demoswarm.sh count bdd \
  --dir ".runs/feat-auth/signal/features"

# Extract Machine Summary field
bash .claude/scripts/demoswarm.sh ms get \
  --file ".runs/feat-auth/build/code_critique.md" \
  --section "## Machine Summary" \
  --key "status"

# Extract YAML field
bash .claude/scripts/demoswarm.sh yaml get \
  --file ".runs/feat-auth/gate/merge_decision.md" \
  --key "status"

# Get current timestamp
bash .claude/scripts/demoswarm.sh time now
```

### Index Management (runs-index)

For updating `.runs/index.json`:

```bash
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index ".runs/index.json" \
  --run-id "feat-auth" \
  --status "VERIFIED" \
  --last-flow "signal" \
  --updated-at "$(bash .claude/scripts/demoswarm.sh time now)"
```

### Open Questions (openq-tools)

For managing the open questions register:

```bash
# Get next available QID
bash .claude/scripts/demoswarm.sh openq next-id \
  --file ".runs/feat-auth/signal/open_questions.md" \
  --prefix "SIG"

# Append a question (auto-generates QID)
bash .claude/scripts/demoswarm.sh openq append \
  --file ".runs/feat-auth/signal/open_questions.md" \
  --prefix "SIG" \
  --question "What is the session timeout?" \
  --default "30 minutes" \
  --impact "Affects user experience vs security"
```

### Secrets Scanning (secrets-tools)

For publish gate security:

```bash
# Scan for secrets
bash .claude/scripts/demoswarm.sh secrets scan \
  --path ".runs/feat-auth/build" \
  --output ".runs/feat-auth/build/secrets_scan.json"

# Redact specific secret type
bash .claude/scripts/demoswarm.sh secrets redact \
  --file ".runs/feat-auth/build/config.md" \
  --type "github-token"
```

## Common Patterns

### Handling Null Values

Skills return `null` (the literal string) for missing data:

```bash
REQ_COUNT=$(bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/${RUN_ID}/signal/requirements.md" \
  --regex '^### REQ-')

if [[ "$REQ_COUNT" == "null" ]]; then
  echo "Requirements file missing or unparseable"
  BLOCKERS+=("requirements.md missing")
else
  echo "Found $REQ_COUNT requirements"
fi
```

### Chaining Commands

Use command substitution for timestamps and sequential operations:

```bash
# Update index with current timestamp
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index ".runs/index.json" \
  --run-id "$RUN_ID" \
  --status "VERIFIED" \
  --last-flow "build" \
  --updated-at "$(bash .claude/scripts/demoswarm.sh time now)"
```

### Checking Multiple Files

Run parallel checks and aggregate results:

```bash
# Check multiple critiques
REQ_STATUS=$(bash .claude/scripts/demoswarm.sh ms get \
  --file ".runs/${RUN_ID}/signal/requirements_critique.md" \
  --section "## Machine Summary" --key "status")

BDD_STATUS=$(bash .claude/scripts/demoswarm.sh ms get \
  --file ".runs/${RUN_ID}/signal/bdd_critique.md" \
  --section "## Machine Summary" --key "status")

if [[ "$REQ_STATUS" == "VERIFIED" && "$BDD_STATUS" == "VERIFIED" ]]; then
  echo "All critiques passed"
fi
```

### Secrets Scanning Workflow

The standard pattern for publish gate scanning:

```bash
SCAN_OUTPUT=".runs/${RUN_ID}/${FLOW}/secrets_scan.json"
STATUS=$(bash .claude/scripts/demoswarm.sh secrets scan \
  --path ".runs/${RUN_ID}/${FLOW}" \
  --output "$SCAN_OUTPUT")

case "$STATUS" in
  "CLEAN")
    echo "No secrets found, safe to publish"
    ;;
  "SECRETS_FOUND")
    echo "Secrets detected, redaction required"
    # Read findings and redact
    ;;
  "SCAN_PATH_MISSING")
    echo "Scan path does not exist"
    ;;
esac
```

## Skill Ownership

Different skills own different command families:

| Skill | Commands |
|-------|----------|
| runs-derive | `count`, `ms get`, `yaml get/count-items`, `inv get`, `line get`, `receipts count`, `receipt get`, `openapi count-paths`, `time now` |
| runs-index | `index upsert-status` |
| openq-tools | `openq next-id`, `openq append` |
| secrets-tools | `secrets scan`, `secrets redact` |

## Which Agents Use Which Skills

| Skill | Primary Users | Usage Context |
|-------|---------------|---------------|
| runs-derive | cleanup agents | Extracting counts for receipts |
| runs-index | run-prep, cleanup agents | Updating run status |
| openq-tools | clarifier, orchestrators | Registering questions |
| secrets-tools | secrets-sanitizer | Publish gate |
| test-runner | test-executor, test-author | Test execution |
| auto-linter | standards-enforcer | Code hygiene |
| policy-runner | policy-analyst | Compliance verification |

## Installation

### Installing the Rust Implementation (Preferred)

```bash
cargo install --path tools/demoswarm-runs-tools --root .demoswarm
```

This installs the `demoswarm` binary to `.demoswarm/bin/`.

### Verifying Installation

```bash
# Should print a timestamp
bash .claude/scripts/demoswarm.sh time now

# Should print a count or null
bash .claude/scripts/demoswarm.sh count pattern --file CLAUDE.md --regex '^#'
```

## Troubleshooting

### "demoswarm: no implementation found"

The shim couldn't find any implementation. Install the Rust binary:

```bash
cargo install --path tools/demoswarm-runs-tools --root .demoswarm
```

### Unexpected `null` Results

Check that:
1. The file path is correct and repo-root-relative
2. The file exists
3. The pattern/key matches the expected format

### Permission Errors

The shim should handle permissions automatically. If issues persist:
1. Check file permissions on the target files
2. Ensure the shim script is executable: `chmod +x .claude/scripts/demoswarm.sh`

### Debugging Mode

For detailed error information, use strict mode (for manual debugging only):

```bash
DEMOSWARM_STRICT=1 .demoswarm/bin/demoswarm count pattern --file missing.md --regex 'x'
```

Note: Never set `DEMOSWARM_STRICT` in agent code. The shim explicitly unsets it.

## See Also

- [Skills Index](../reference/skills-index.md) - Complete skill reference
- [DemoSwarm CLI Reference](../reference/demoswarm-cli.md) - Full command documentation
- [Working with Receipts](working-with-receipts.md) - Using skills in cleanup agents
