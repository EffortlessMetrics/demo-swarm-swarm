# Skills Index

This document provides a complete reference for all DemoSwarm skills. Skills are specialized tooling capabilities that agents invoke to perform mechanical operations.

## Overview

| Skill | Purpose | Primary Users |
|-------|---------|---------------|
| [test-runner](#test-runner) | Execute tests and capture results | test-executor, test-author |
| [auto-linter](#auto-linter) | Format and lint code | standards-enforcer |
| [policy-runner](#policy-runner) | Run policy-as-code checks | policy-analyst, flow orchestrators |
| [runs-derive](#runs-derive) | Read-only extraction from .runs artifacts | cleanup agents |
| [runs-index](#runs-index) | Update .runs/index.json | run-prep, cleanup agents |
| [openq-tools](#openq-tools) | Open questions register management | clarifier, flow orchestrators |
| [secrets-tools](#secrets-tools) | Secrets scanning and redaction | secrets-sanitizer |

## Invocation Pattern

All skills are invoked via the demoswarm shim:

```bash
bash .claude/scripts/demoswarm.sh <command> [options]
```

Never invoke skill implementations directly. The shim handles resolution and fallback.

See [How to Invoke Skills](../how-to/invoke-skills.md) for detailed usage guidance.

---

## test-runner

**Description:** Run the relevant tests for the current change and summarize results.

**Use in:** Flow 3 (Build), Flow 4 (Gate)

**Allowed Tools:** Bash, Read, Write

### Purpose

Execute the repository's test suite with appropriate scoping and capture structured output. The skill:

- Runs tests (scoped or full)
- Captures raw output and exit codes
- Produces structured summaries with counts
- Handles framework-specific invocation patterns

### What It Does NOT Do

- Write or modify tests (that's test-author)
- Judge test quality (that's test-critic)
- Make decisions about what to test (that's orchestrator)
- Classify failures (that's flakiness-detector)

### Invocation

Test-runner is invoked directly via test framework commands, not through the demoswarm shim. The skill detects the project stack and runs appropriate commands:

```bash
# Rust
cargo test --workspace --tests --color=always

# Python (pytest)
pytest --color=yes -v

# JavaScript (jest)
npm test -- --colors

# Go
go test ./...
```

### Inputs

| Input | Source | Required |
|-------|--------|----------|
| Test command | Detected from stack or `demo-swarm.config.json` | Yes |
| Scope | Caller-provided (files, modules, AC pattern) | Optional |
| Fail-fast flag | Flow context | Optional |

### Outputs

| Output | Location | Content |
|--------|----------|---------|
| `test_output.log` | Working directory | Raw test output |
| `test_summary.md` | Working directory | Parsed summary with counts |

### Summary Format

```markdown
# Test Summary

## Overall Status
- **Result:** PASS | FAIL
- **Exit Code:** <int>
- **Duration:** <time>

## Counts
- Passed: <int>
- Failed: <int>
- Skipped: <int>

## Failing Tests (if any)
- `path::to::test_name` - <short error>
```

### Selection Strategies

| Strategy | Use When | Example |
|----------|----------|---------|
| Full suite | Global hardening, Gate verification | `cargo test --workspace` |
| Changed tests | Focused verification | Tests for files in diff |
| AC-scoped | Microloop verification | `pytest -m "AC_001"` |
| Smoke tests | Quick sanity check | `pytest -m smoke -x` |

---

## auto-linter

**Description:** Run linters/formatters on changed files and apply safe, mechanical fixes.

**Use in:** Flow 3 (Build), Flow 4 (Review)

**Allowed Tools:** Bash, Read, Write

### Purpose

Provide automated code formatting and linting. The skill:

1. Formats code to match project style (safe to auto-apply)
2. Lints code to detect issues and violations (reports findings)
3. Auto-fixes safe issues that are mechanical/non-semantic
4. Reports issues requiring human judgment

### Invocation

Auto-linter is invoked directly via formatting/linting tools:

```bash
# Rust
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings

# JavaScript/TypeScript
npx prettier --write "src/**/*.{js,ts,jsx,tsx}"
npx eslint --fix "src/**/*.{js,ts,jsx,tsx}"

# Python
black src/ tests/
ruff check --fix src/ tests/
```

### Inputs

| Input | Source | Required |
|-------|--------|----------|
| Changed files | Git diff or caller-provided | Optional |
| Project config | `.prettierrc`, `rustfmt.toml`, etc. | Auto-detected |

### Outputs

| Output | Location | Content |
|--------|----------|---------|
| `lint_output.log` | Working directory | Raw tool output |
| `lint_summary.md` | Working directory | Parsed summary |
| Modified files | In-place | Formatted code |

### Safe vs Manual Fixes

**Safe to Auto-Apply:**
- Whitespace, indentation, trailing spaces
- Import sorting, trailing commas
- Quote style, semicolons (where optional)

**Requires Manual Review:**
- Logic changes, error handling
- Type narrowing, dead code removal
- Security-related warnings

---

## policy-runner

**Description:** Run policy-as-code checks (OPA/Conftest) based on the policy_plan.

**Use in:** Flow 2 (Plan), Flow 4 (Review), Flow 5 (Gate)

**Allowed Tools:** Bash, Read

### Purpose

Execute OPA/Conftest/Rego policies and produce structured evidence for compliance verification. The skill:

- Executes policy checks defined in `policy_plan.md`
- Runs OPA/Conftest against target files
- Produces structured pass/fail evidence
- Generates summaries for downstream agents

### What It Does NOT Do

- Grant waivers or exceptions
- Modify policy files
- Make compliance judgments (that's policy-analyst's job)
- Post to GitHub

### Invocation

```bash
# Via shim (when supported)
bash .claude/scripts/demoswarm.sh policy run \
  --plan ".runs/<run-id>/plan/policy_plan.md" \
  --output ".runs/<run-id>/plan/policy_runner_output.log"

# Direct tool invocation
conftest test <path> -p <policy-dir> --output json
opa eval --data <policy.rego> --input <target.json> "data.policy.deny"
```

### Inputs

| Input | Source | Required |
|-------|--------|----------|
| `policy_plan.md` | `.runs/<run-id>/plan/` or `policies/` | Yes |
| Target files | Specified in policy_plan.md | Yes |
| Policy files | `policies/` directory | Yes |

### Outputs

| Output | Location | Content |
|--------|----------|---------|
| `policy_runner_output.log` | Run directory | Raw execution output |
| `policy_runner_summary.md` | Run directory | Structured results table |

### Summary Format

```markdown
# Policy Runner Summary

## Results
| Policy | Target | Status | Violations |
|--------|--------|--------|------------|
| api-security | api_contracts.yaml | PASS | 0 |
| data-retention | schema.md | FAIL | 1 |
```

---

## runs-derive

**Description:** Read-only, deterministic helpers for `.runs/` derivation.

**Use in:** Cleanup agents for mechanical counts/extraction

**Allowed Tools:** Bash, Read

### Purpose

Provide consistent, null-safe extraction of data from `.runs/` artifacts:

- Count markers (REQ/NFR/QID/RSK)
- Extract Machine Summary fields
- Parse YAML blocks
- Count BDD scenarios
- Read receipt fields

### Operating Invariants

- **Null over guess:** Missing file/dir returns `null`, not `0`
- **Present but no matches:** Returns `0`
- **Unparseable/error:** Returns `null`
- **Single scalar stdout:** Always one value
- **Exit code 0:** Always (errors expressed via `null`)

### Invocation

```bash
bash .claude/scripts/demoswarm.sh <command> [options]
```

### Commands

| Command | Purpose | Example |
|---------|---------|---------|
| `count pattern` | Count lines matching regex | `--file <path> --regex '^### REQ-'` |
| `count bdd` | Count BDD scenarios | `--dir <features-dir>` |
| `ms get` | Extract Machine Summary field | `--file <path> --key "status"` |
| `yaml get` | Extract YAML block field | `--file <path> --key <name>` |
| `yaml count-items` | Count items in YAML block | `--file <path> --item-regex <pattern>` |
| `inv get` | Extract inventory marker value | `--file <path> --marker <name>` |
| `line get` | Extract value from prefixed line | `--file <path> --prefix <text>` |
| `receipts count` | Count prior flow receipts | `--run-dir <path>` |
| `receipt get` | Read field from receipt JSON | `--file <path> --key <name>` |
| `openapi count-paths` | Count paths in OpenAPI YAML | `--file <path>` |
| `time now` | Get current UTC timestamp | (no arguments) |

### Example Usage

```bash
# Count functional requirements
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/feat-auth/signal/requirements.md" \
  --regex '^### REQ-' \
  --null-if-missing
# stdout: 5 (or null if missing)

# Get critic status
bash .claude/scripts/demoswarm.sh ms get \
  --file ".runs/feat-auth/signal/requirements_critique.md" \
  --section "## Machine Summary" \
  --key "status"
# stdout: VERIFIED (or null)
```

---

## runs-index

**Description:** Deterministic updates to `.runs/index.json`.

**Use in:** run-prep and cleanup agents only

**Allowed Tools:** Bash, Read, Write

### Purpose

Update run status in the central index with minimal surface:

- Update `status`, `last_flow`, `updated_at` fields
- Preserve all other fields
- Produce stable git diffs

### Operating Invariants

- **Minimal ownership:** Only updates status/last_flow/updated_at
- **Stable diffs:** Upsert by run_id, preserve ordering
- **No creation:** Fails if index.json doesn't exist (run-prep owns creation)
- **Idempotent:** Same args produce same result

### Allowed Users

- run-prep
- signal-run-prep
- *-cleanup agents (signal, plan, build, gate, deploy, wisdom)

### Invocation

```bash
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index ".runs/index.json" \
  --run-id "feat-auth" \
  --status "VERIFIED" \
  --last-flow "signal" \
  --updated-at "$(bash .claude/scripts/demoswarm.sh time now)"
```

### Inputs

| Input | Flag | Required |
|-------|------|----------|
| Index path | `--index` | Yes |
| Run ID | `--run-id` | Yes |
| Status | `--status` | Yes |
| Last flow | `--last-flow` | Yes |
| Timestamp | `--updated-at` | Optional (defaults to now) |

### Outputs

**Stdout:** `ok` | `SKIPPED_MISSING_INDEX` | `SKIPPED_RUN_NOT_FOUND`

---

## openq-tools

**Description:** Open questions register management with sequential QID generation.

**Use in:** clarifier agent, flow orchestrators when questions arise

**Allowed Tools:** Bash, Read, Write

### Purpose

Manage the open questions register (`open_questions.md`):

- Generate sequential QIDs
- Append question entries with context
- Maintain consistent format

### QID Format

Pattern: `OQ-<FLOW>-<NNN>` (e.g., `OQ-SIG-001`, `OQ-PLAN-002`)

| Flow | Code | Example |
|------|------|---------|
| signal | SIG | OQ-SIG-001 |
| plan | PLAN | OQ-PLAN-001 |
| build | BUILD | OQ-BUILD-001 |
| review | REVIEW | OQ-REVIEW-001 |
| gate | GATE | OQ-GATE-001 |
| deploy | DEPLOY | OQ-DEPLOY-001 |
| wisdom | WISDOM | OQ-WISDOM-001 |

### Invocation

```bash
# Generate next QID
bash .claude/scripts/demoswarm.sh openq next-id \
  --file ".runs/feat-auth/signal/open_questions.md" \
  --prefix "SIG"
# stdout: OQ-SIG-003

# Append a question (auto-generates QID)
bash .claude/scripts/demoswarm.sh openq append \
  --file ".runs/feat-auth/signal/open_questions.md" \
  --prefix "SIG" \
  --question "Should authentication use JWT or session cookies?" \
  --default "Use JWT for stateless authentication" \
  --impact "Session cookies require server-side state management"
# stdout: OQ-SIG-003
```

### Commands

| Command | Purpose | Returns |
|---------|---------|---------|
| `openq next-id` | Generate next QID for a flow | QID string |
| `openq append` | Append question entry to file | Assigned QID |

### Entry Format

```markdown
- QID: OQ-SIG-003
  - Q: Should authentication use JWT or session cookies? [OPEN]
  - Suggested default: Use JWT for stateless authentication
  - Impact if different: Session cookies require server-side state management
  - Added: 2025-12-12T10:30:00Z
```

---

## secrets-tools

**Description:** Secrets scanning and redaction for publish gates.

**Use in:** secrets-sanitizer agent (publish gate)

**Allowed Tools:** Bash, Read, Write

### Purpose

Scan for and redact secrets before publishing:

- Scan files for common secret patterns
- Report findings (locations only, never content)
- Redact secrets in-place when needed

### CRITICAL: Never Print Secret Content

This skill has a strict output contract:

1. **NEVER** print matched secret values to stdout, stderr, or any file
2. **NEVER** store raw secret values in JSON or any artifact
3. **Only output:** file path, line number, secret type, redacted snippet
4. **Redacted format:** `<prefix>...<suffix>` (e.g., `ghp_...abcd`)

### Allowed Users

- **Primary:** secrets-sanitizer (publish gate agent)
- **Secondary:** repo-operator (hygiene checks, read-only)
- **Not allowed:** cleanup agents, author agents, critic agents

### Invocation

```bash
# Scan for secrets
bash .claude/scripts/demoswarm.sh secrets scan \
  --path ".runs/feat-auth/signal" \
  --output ".runs/feat-auth/signal/secrets_scan.json"
# stdout: CLEAN | SECRETS_FOUND | SCAN_PATH_MISSING

# Redact a specific type
bash .claude/scripts/demoswarm.sh secrets redact \
  --file ".runs/feat-auth/signal/github_research.md" \
  --type "github-token"
# stdout: ok | FILE_NOT_FOUND | null
```

### Secret Types

| Type | Pattern | Replacement |
|------|---------|-------------|
| `github-token` | `gh[pousr]_[A-Za-z0-9_]{36,}` | `[REDACTED:github-token]` |
| `aws-access-key` | `AKIA[0-9A-Z]{16}` | `[REDACTED:aws-access-key]` |
| `stripe-key` | `sk_live_[0-9a-zA-Z]{24,}` | `[REDACTED:stripe-key]` |
| `private-key` | `-----BEGIN .*PRIVATE KEY-----` | `[REDACTED:private-key]` |
| `jwt-token` | `eyJ[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*` | `[REDACTED:jwt-token]` |

### Output Format

**Scan JSON output:**

```json
{
  "status": "SECRETS_FOUND",
  "findings": [
    {
      "file": ".runs/feat-auth/signal/github_research.md",
      "type": "github-token",
      "lines": "42,87"
    }
  ]
}
```

---

## See Also

- [How to Invoke Skills](../how-to/invoke-skills.md) - Usage guide and contracts
- [DemoSwarm CLI Reference](demoswarm-cli.md) - Full command reference
- [Agent Philosophy](../explanation/agent-philosophy.md) - How agents use skills
