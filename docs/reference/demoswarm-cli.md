# DemoSwarm CLI Reference

This document specifies the CLI helper surface for mechanical operations in `.runs/` artifacts. All cleanup agents must use the demoswarm shim for these operations.

## Contract Summary

| Principle            | Rule                                        |
| -------------------- | ------------------------------------------- |
| Null over guess      | Missing file/pattern -> `null`, not `0`     |
| Single scalar stdout | Each command prints exactly one value       |
| Exit code 0          | Always exit 0 (errors expressed via `null`) |
| No git/GitHub        | These helpers never touch git or gh         |
| Repo-root paths      | All paths are repo-root-relative            |

**Strict mode (optional):** set `DEMOSWARM_STRICT=1` to return non-zero exit codes on parse/exec errors. Agents never set this; it is for human debugging only. Note: `.claude/scripts/demoswarm.sh` unsets `DEMOSWARM_STRICT` to keep agent behavior stable; use direct `demoswarm ...` invocation for strict debugging.

## Exit Codes

| Exit Code | Mode | Meaning |
| --------- | ---- | ------- |
| 0 | Normal | Success OR error (errors expressed via `null` output) |
| 0 | Strict | Success only |
| 2 | Strict | Parse error or execution error |

**Key principle:** In normal mode, the CLI **always** exits 0. Errors are communicated via the `null` sentinel in stdout. This ensures agents using the shim never encounter unexpected process failures.

**When to use strict mode:**
- Human debugging of command syntax
- CI/CD scripts that need exit code signaling
- Never in agent automation (the shim unsets `DEMOSWARM_STRICT`)

## Error Handling

### Output Signals

All commands communicate errors via stdout values, not exit codes:

| Output | Meaning |
| ------ | ------- |
| `null` | Error occurred (file missing, parse error, invalid input) |
| `0` | Valid result: zero matches/items found |
| Integer | Valid result: count of matches/items |
| String | Valid result: extracted value |
| `CLEAN` | Secrets scan: no secrets found |
| `SECRETS_FOUND` | Secrets scan: secrets detected |
| `ok` | Write operation succeeded |
| `SKIPPED_MISSING_INDEX` | Index file does not exist |
| `PATTERN_ERROR` | Secrets scan: invalid regex in patterns file |
| `FILE_NOT_FOUND` | File does not exist (redact command) |
| `SCAN_PATH_MISSING` | Scan path does not exist |

### Common Error Scenarios

| Scenario | Command Output | How to Handle |
| -------- | -------------- | ------------- |
| File does not exist | `null` | Check if file path is correct; file may not be created yet |
| Directory does not exist | `null` | Verify the run directory exists |
| Invalid regex pattern | `null` | Check regex syntax (ERE format required) |
| No YAML block in file | `null` | File may not contain fenced ` ```yaml ``` ` block |
| Key not found | `null` | Verify key name matches exactly (case-sensitive) |
| Template leak detected | `null` | Value contains `\|` or `<` characters (unfilled template) |
| JSON parse error | `null` | Receipt file may be malformed JSON |

### Debugging Tips

1. **Use strict mode for debugging:**
   ```bash
   DEMOSWARM_STRICT=1 demoswarm count pattern --file x --regex y
   # Exit code 2 + error message if something is wrong
   ```

2. **Check file existence first:**
   ```bash
   test -f ".runs/feat-auth/signal/requirements.md" && \
     bash .claude/scripts/demoswarm.sh count pattern \
       --file ".runs/feat-auth/signal/requirements.md" \
       --regex '^### REQ-'
   ```

3. **Validate regex separately:**
   ```bash
   echo "test line" | grep -E '^test' >/dev/null && echo "regex valid"
   ```

## Architecture

### Shim Invocation (Required)

Agents **always invoke via the shim**:

```bash
bash .claude/scripts/demoswarm.sh <command> [OPTIONS]
```

Never invoke the binary directly or manipulate PATH.

### Resolution Order

The shim resolves in order:

1. `.demoswarm/bin/demoswarm` (Rust, repo-local install, preferred)
2. `demoswarm` on PATH (global install)
3. `cargo run` fallback (dev only, if `tools/` exists)
4. Python fallback (legacy)

### Implementations

| Implementation   | Location                | Notes                        |
| ---------------- | ----------------------- | ---------------------------- |
| Rust (demoswarm) | `.demoswarm/bin/`       | Preferred; install via cargo |
| Python           | `.claude/skills/*/bin/` | Legacy fallback              |

## Command Reference

### Global Options

All commands accept the following global option:

| Flag       | Description                                                              |
| ---------- | ------------------------------------------------------------------------ |
| `--strict` | Enable strict mode: return exit code 2 on errors instead of 0            |

The `--strict` flag can be placed anywhere on the command line and takes precedence over the `DEMOSWARM_STRICT` environment variable.

---

### count pattern

Count lines matching a regex in a file.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh count pattern --file <path> --regex <ere> [--fallback-regex <ere>] [--null-if-missing] [--null-if-zero]
```

**Arguments:**

| Flag                     | Required | Description                                                                           |
| ------------------------ | -------- | ------------------------------------------------------------------------------------- |
| `--file <path>`          | Yes      | File to search                                                                        |
| `--regex <ere>`          | Yes      | Extended regex pattern                                                                |
| `--fallback-regex <ere>` | No       | Try if primary returns 0                                                              |
| `--null-if-missing`      | No       | Provided for API consistency; functionally redundant (missing already returns `null`) |
| `--null-if-zero`         | No       | Return `null` instead of `0` when no matches                                          |

**Stdout:** `null` | integer

**Semantics:**

- File missing -> `null`
- File exists, pattern matches N lines -> `N`
- File exists, no matches -> `0`
- File exists, grep error -> `null`
- With `--fallback-regex`: if primary regex returns `0`, try fallback
- With `--null-if-zero`: `null` instead of `0` when no matches

**Example:**

```bash
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/feat-auth/signal/requirements.md" \
  --regex '^### REQ-'
# stdout: 5 (or null)
```

---

### count bdd

Count BDD scenarios across feature files.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh count bdd --dir <path> [--null-if-missing]
```

**Arguments:**

| Flag                | Required | Description                                                                           |
| ------------------- | -------- | ------------------------------------------------------------------------------------- |
| `--dir <path>`      | Yes      | Features directory                                                                    |
| `--null-if-missing` | No       | Provided for API consistency; functionally redundant (missing already returns `null`) |

**Stdout:** `null` | integer

**Semantics:**

- Dir missing -> `null`
- Dir exists, no `.feature` files -> `0`
- Counts lines matching `^\s*(Scenario:|Scenario Outline:)`

**Example:**

```bash
bash .claude/scripts/demoswarm.sh count bdd \
  --dir ".runs/feat-auth/signal/features"
# stdout: 12 (or null)
```

---

### ms get

Extract a field from a `## Machine Summary` block.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh ms get --file <path> --section <header> --key <name> [--null-if-missing]
```

**Arguments:**

| Flag                 | Required | Description                                  |
| -------------------- | -------- | -------------------------------------------- |
| `--file <path>`      | Yes      | Markdown file                                |
| `--section <header>` | Yes      | Section header (always `## Machine Summary`) |
| `--key <name>`       | Yes      | Field name (e.g., `status`)                  |

**Stdout:** `null` | string (the field value)

**Semantics:**

- File missing -> `null`
- Section not found -> `null`
- Key not found in section -> `null`
- Value contains `|` or `<` (template leak) -> `null`
- Otherwise -> the value (first word after `key:`)

**Example:**

```bash
bash .claude/scripts/demoswarm.sh ms get \
  --file ".runs/feat-auth/signal/requirements_critique.md" \
  --section "## Machine Summary" \
  --key "status"
# stdout: VERIFIED (or null)
```

---

### yaml get

Extract a field from a fenced YAML block at the start of a file.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh yaml get --file <path> --key <name> [--null-if-missing]
```

**Arguments:**

| Flag            | Required | Description                   |
| --------------- | -------- | ----------------------------- |
| `--file <path>` | Yes      | Markdown file with YAML block |
| `--key <name>`  | Yes      | YAML key to extract           |

**Stdout:** `null` | string

**Semantics:**

- File missing -> `null`
- No YAML block (no ` ```yaml ``` `) -> `null`
- Key not in YAML -> `null`
- Otherwise -> the value

**Example:**

```bash
bash .claude/scripts/demoswarm.sh yaml get \
  --file ".runs/feat-auth/deploy/deployment_decision.md" \
  --key "deployment_verdict"
# stdout: STABLE (or null)
```

---

### yaml count-items

Count items matching a pattern in a YAML block.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh yaml count-items --file <path> --item-regex <ere> [--null-if-missing]
```

**Arguments:**

| Flag                 | Required | Description                   |
| -------------------- | -------- | ----------------------------- |
| `--file <path>`      | Yes      | Markdown file with YAML block |
| `--item-regex <ere>` | Yes      | Pattern to count within YAML  |

**Stdout:** `null` | integer

**Semantics:**

- File missing -> `null`
- No YAML block -> `null`
- Counts lines matching pattern within YAML block only
- POSIX character classes like `[[:space:]]` are supported

**Example:**

```bash
bash .claude/scripts/demoswarm.sh yaml count-items \
  --file ".runs/feat-auth/deploy/deployment_decision.md" \
  --item-regex '^[[:space:]]*- check:'
# stdout: 3 (or null)
```

---

### inv get

Extract value from an inventory marker line.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh inv get --file <path> --marker <name> [--null-if-missing]
```

**Arguments:**

| Flag              | Required | Description                           |
| ----------------- | -------- | ------------------------------------- |
| `--file <path>`   | Yes      | File to search                        |
| `--marker <name>` | Yes      | Marker prefix (e.g., `DEP_CI_SIGNAL`) |

**Stdout:** `null` | string

**Semantics:**

- Looks for `^- <MARKER>: <value>`
- Returns first match's value
- File missing -> `null`
- No matching marker -> `null`

**Example:**

```bash
bash .claude/scripts/demoswarm.sh inv get \
  --file ".runs/feat-auth/deploy/verification_report.md" \
  --marker "DEP_CI_SIGNAL"
# stdout: PASS (or null)
```

---

### line get

Extract value from a line with a known prefix.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh line get --file <path> --prefix <text> [--null-if-missing]
```

**Arguments:**

| Flag                | Required | Description                                                                           |
| ------------------- | -------- | ------------------------------------------------------------------------------------- |
| `--file <path>`     | Yes      | File to search                                                                        |
| `--prefix <text>`   | Yes      | Line prefix (e.g., `Mutation Score:`)                                                 |
| `--null-if-missing` | No       | Provided for API consistency; functionally redundant (missing already returns `null`) |

**Stdout:** `null` | string

**Semantics:**

- File missing -> `null`
- No matching prefix -> `null`
- Empty value after prefix -> `null`
- Otherwise returns trimmed value after prefix

**Example:**

```bash
bash .claude/scripts/demoswarm.sh line get \
  --file ".runs/feat-auth/build/mutation_report.md" \
  --prefix "Mutation Score:"
# stdout: 85% (or null)
```

---

### receipts count

Count prior flow receipts in a run directory.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh receipts count --run-dir <path> [--null-if-missing]
```

**Arguments:**

| Flag                | Required | Description                                                                           |
| ------------------- | -------- | ------------------------------------------------------------------------------------- |
| `--run-dir <path>`  | Yes      | Run directory path                                                                    |
| `--null-if-missing` | No       | Provided for API consistency; functionally redundant (missing already returns `null`) |

**Stdout:** `null` | integer

**Semantics:**

- Counts known receipt files (signal, plan, build, gate, deploy, wisdom)
- Max is 6
- Run directory missing -> `null`

**Example:**

```bash
bash .claude/scripts/demoswarm.sh receipts count \
  --run-dir ".runs/feat-auth"
# stdout: 4 (or null)
```

---

### receipt get

Read a field from a receipt JSON file.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh receipt get --file <path> --key <name> [--null-if-missing]
```

**Arguments:**

| Flag                | Required | Description                                                                           |
| ------------------- | -------- | ------------------------------------------------------------------------------------- |
| `--file <path>`     | Yes      | Receipt JSON file                                                                     |
| `--key <name>`      | Yes      | Top-level key to extract                                                              |
| `--null-if-missing` | No       | Provided for API consistency; functionally redundant (missing already returns `null`) |

**Stdout:** `null` | string

**Semantics:**

- Uses discovery protocol: tries direct file read first, then `git show HEAD:<path>` fallback
- File missing AND git fallback fails -> `null`
- JSON parse error -> `null`
- Key not found -> `null`
- Returns only scalar values (strings, numbers, booleans); arrays/objects return `null`
- Logs discovery method to stderr (`discovery_method: direct_read` or `discovery_method: git_show`)

**Example:**

```bash
bash .claude/scripts/demoswarm.sh receipt get \
  --file ".runs/feat-auth/gate/gate_receipt.json" \
  --key "merge_verdict"
# stdout: MERGE (or null)
# stderr: discovery_method: direct_read
```

---

### openapi count-paths

Count API paths in an OpenAPI YAML file.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh openapi count-paths --file <path> [--null-if-missing]
```

**Arguments:**

| Flag                | Required | Description                                                                           |
| ------------------- | -------- | ------------------------------------------------------------------------------------- |
| `--file <path>`     | Yes      | OpenAPI YAML file                                                                     |
| `--null-if-missing` | No       | Provided for API consistency; functionally redundant (missing already returns `null`) |

**Stdout:** `null` | integer

**Semantics:**

- Counts top-level keys under `paths:`
- Best-effort; returns `null` if unparseable or paths section missing
- File missing -> `null`

**Example:**

```bash
bash .claude/scripts/demoswarm.sh openapi count-paths \
  --file ".runs/feat-auth/plan/api_contracts.yaml"
# stdout: 8 (or null)
```

---

### index upsert-status

Update a run's status in `.runs/index.json`.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index <path> --run-id <id> --status <status> --last-flow <flow> [--updated-at <iso>]
```

**Arguments:**

| Flag                 | Required | Description                                  |
| -------------------- | -------- | -------------------------------------------- |
| `--index <path>`     | Yes      | Path to index.json                           |
| `--run-id <id>`      | Yes      | Run ID to update                             |
| `--status <status>`  | Yes      | VERIFIED, UNVERIFIED, or CANNOT_PROCEED      |
| `--last-flow <flow>` | Yes      | signal, plan, build, gate, deploy, or wisdom |
| `--updated-at <iso>` | No       | ISO8601 timestamp (defaults to now)          |

**Stdout:** `ok` | `SKIPPED_MISSING_INDEX` | `null`

**Semantics:**

- Index missing -> print `SKIPPED_MISSING_INDEX`, exit 0
- If run_id exists: updates `status`, `last_flow`, `updated_at` (preserves all other fields)
- If run_id does not exist: creates new entry with provided values
- Keeps `runs[]` sorted by `run_id`
- Atomic write (temp file + rename)
- JSON parse error -> `null`

**Example:**

```bash
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index ".runs/index.json" \
  --run-id "feat-auth" \
  --status "VERIFIED" \
  --last-flow "signal" \
  --updated-at "2025-01-15T10:30:00Z"
# stdout: ok
```

---

### time now

Print current UTC timestamp in ISO8601 format.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh time now
```

**Arguments:** None

**Stdout:** ISO8601 timestamp

**Example:**

```bash
bash .claude/scripts/demoswarm.sh time now
# stdout: 2025-01-15T10:30:00Z
```

---

## Utility Commands

These are used by skills or scripts, not typically called directly by agents.

### openq next-id

Generate the next open question ID in sequence.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh openq next-id --file <path> --prefix <prefix>
```

**Arguments:**

| Flag                | Required | Description                              |
| ------------------- | -------- | ---------------------------------------- |
| `--file <path>`     | Yes      | Path to open_questions.md file           |
| `--prefix <prefix>` | Yes      | ID prefix (e.g., `SIG`, `PLAN`, `BUILD`) |

**Stdout:** `OQ-<PREFIX>-NNN` (e.g., `OQ-SIG-001`)

**Semantics:**

- Scans file for existing IDs matching `OQ-<PREFIX>-NNN` pattern
- Returns next sequential ID (zero-padded to 3 digits)
- If file missing or no existing IDs -> returns `OQ-<PREFIX>-001`
- Never returns `null`; always generates a valid ID

**Example:**

```bash
bash .claude/scripts/demoswarm.sh openq next-id \
  --file ".runs/feat-auth/signal/open_questions.md" \
  --prefix "SIG"
# stdout: OQ-SIG-003 (if OQ-SIG-001 and OQ-SIG-002 exist)
```

---

### openq append

Append an open question entry to a file.

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh openq append \
  --file <path> --prefix <prefix> --question <text> \
  --default <text> --impact <text>
```

**Arguments:**

| Flag                | Required | Description                               |
| ------------------- | -------- | ----------------------------------------- |
| `--file <path>`     | Yes      | Path to open_questions.md file            |
| `--prefix <prefix>` | Yes      | ID prefix (e.g., `SIG`, `PLAN`, `BUILD`)  |
| `--question <text>` | Yes      | The open question text                    |
| `--default <text>`  | Yes      | Suggested default answer/assumption       |
| `--impact <text>`   | Yes      | Impact description if assumption is wrong |

**Stdout:** The generated QID (e.g., `OQ-SIG-003`)

**Semantics:**

- Generates next ID using `openq next-id` logic
- Appends formatted entry with: QID, question, suggested default, impact, and ISO8601 timestamp
- Creates file if it does not exist
- Returns the assigned QID on success

**Example:**

```bash
bash .claude/scripts/demoswarm.sh openq append \
  --file ".runs/feat-auth/signal/open_questions.md" \
  --prefix "SIG" \
  --question "Should OAuth tokens expire after 24h or 7d?" \
  --default "24 hours for security" \
  --impact "User experience vs security tradeoff"
# stdout: OQ-SIG-003
```

---

### secrets scan

Scan for secrets (locations only, never content).

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh secrets scan --path <path> --output <path> [--patterns-file <path>] [-v|--verbose]
```

**Arguments:**

| Flag                     | Required | Description                                            |
| ------------------------ | -------- | ------------------------------------------------------ |
| `--path <path>`          | Yes      | File or directory to scan                              |
| `--output <path>`        | Yes      | Path to write JSON findings                            |
| `--patterns-file <path>` | No       | JSON/YAML file with additional secret patterns         |
| `-v`, `--verbose`        | No       | Log skipped paths with reasons to stderr               |

**Stdout:** `CLEAN` | `SECRETS_FOUND` | `SCAN_PATH_MISSING` | `PATTERN_ERROR`

**Semantics:**

- Scans recursively for common secret patterns (tokens, keys, credentials)
- Writes JSON findings to `--output` path (not stdout)
- Stdout returns only the status signal
- Automatically excludes: `.git`, `target`, `node_modules`, `.demoswarm` directories
- Never prints secret content (only file locations and types)
- Custom patterns from `--patterns-file` are merged with built-in patterns (built-in first)
- If `--patterns-file` contains invalid regex, returns `PATTERN_ERROR`

**Built-in patterns:**

| Type             | Pattern matches                            |
| ---------------- | ------------------------------------------ |
| `github-token`   | GitHub tokens (ghp_*, gho_*, ghu_*, etc.)  |
| `aws-access-key` | AWS access key IDs (AKIA*)                 |
| `stripe-key`     | Stripe API keys (sk_live_*, sk_test_*)     |
| `private-key`    | PEM private key blocks                     |
| `jwt-token`      | JSON Web Tokens (eyJ*)                     |

**Custom patterns file format (JSON):**

```json
{
  "patterns": [
    { "pattern": "my-secret-[a-z0-9]+", "type": "my-custom-secret" }
  ]
}
```

**Output JSON format:**

```json
{
  "status": "CLEAN|SECRETS_FOUND|SCAN_PATH_MISSING|PATTERN_ERROR",
  "findings": [
    { "file": "path/to/file", "type": "github-token", "lines": "5,12" }
  ],
  "skipped_count": 0
}
```

**Example:**

```bash
bash .claude/scripts/demoswarm.sh secrets scan \
  --path ".runs/feat-auth/build" \
  --output ".runs/feat-auth/build/secrets_findings.json"
# stdout: CLEAN (or SECRETS_FOUND)
# findings written to .runs/feat-auth/build/secrets_findings.json
```

---

### secrets redact

Redact a specific secret type in a file (in-place).

**Usage:**

```bash
bash .claude/scripts/demoswarm.sh secrets redact --file <path> --type <type> [--patterns-file <path>]
```

**Arguments:**

| Flag                     | Required | Description                                              |
| ------------------------ | -------- | -------------------------------------------------------- |
| `--file <path>`          | Yes      | File to redact secrets from                              |
| `--type <type>`          | Yes      | Type of secret to redact                                 |
| `--patterns-file <path>` | No       | JSON/YAML file with additional patterns (for custom types) |

**Valid `--type` values (built-in):**

| Type             | Description                                          |
| ---------------- | ---------------------------------------------------- |
| `github-token`   | GitHub personal access tokens (ghp_*, gho_*, etc.)   |
| `aws-access-key` | AWS access key IDs (AKIA*)                           |
| `stripe-key`     | Stripe API keys (sk_live_*, sk_test_*)               |
| `jwt-token`      | JSON Web Tokens (eyJ*)                               |
| `private-key`    | Private key blocks (-----BEGIN * PRIVATE KEY-----)   |

Custom types can be used when providing a `--patterns-file` that defines the type.

**Stdout:** `ok` | `FILE_NOT_FOUND` | `null`

**Semantics:**

- Redacts in-place (modifies the file directly)
- Replaces secret content with `[REDACTED:<type>]` placeholder
- Returns `ok` on successful redaction
- Returns `FILE_NOT_FOUND` if file does not exist
- Returns `null` on other errors (unknown type, pattern file errors)
- For `private-key` type, uses line-based block replacement (BEGIN to END)

**Example:**

```bash
bash .claude/scripts/demoswarm.sh secrets redact \
  --file ".runs/feat-auth/build/config_sample.md" \
  --type "github-token"
# stdout: ok
# File now contains [REDACTED:github-token] where tokens were
```

**Example with custom pattern:**

```bash
bash .claude/scripts/demoswarm.sh secrets redact \
  --file ".runs/feat-auth/build/config.md" \
  --type "my-custom-secret" \
  --patterns-file ".claude/config/secret-patterns.json"
# stdout: ok
```

---

## Installation

### Rust (preferred)

```bash
cargo install --path tools/demoswarm-runs-tools --root .demoswarm
```

This installs the `demoswarm` binary to `.demoswarm/bin/`.

### Verification

```bash
bash .claude/scripts/demoswarm.sh time now
bash .claude/scripts/demoswarm.sh count pattern --file CLAUDE.md --regex '^#'
```

---

## Skill Ownership

Different skills own different command families:

| Skill           | Commands Owned                                                                                                                       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `runs-derive`   | `count`, `ms get`, `yaml get/count-items`, `inv get`, `line get`, `receipts count`, `receipt get`, `openapi count-paths`, `time now` |
| `runs-index`    | `index upsert-status`                                                                                                                |
| `openq-tools`   | `openq next-id`, `openq append`                                                                                                      |
| `secrets-tools` | `secrets scan`, `secrets redact`                                                                                                     |

See individual skill docs for usage contracts:

- `.claude/skills/runs-derive/SKILL.md`
- `.claude/skills/runs-index/SKILL.md`
- `.claude/skills/openq-tools/SKILL.md`
- `.claude/skills/secrets-tools/SKILL.md`
