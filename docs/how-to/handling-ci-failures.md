# Handling CI Failures

> What to do when GitHub Actions or CodeRabbit report failures on your PR.

**Goal:** Diagnose CI failures quickly and route to the correct fix path.

**Prereqs:** Understanding of [troubleshoot.md](troubleshoot.md) and [bounded fix-forward](../explanation/bounded-fix-forward.md).

---

## Quick Diagnosis

When CI fails, start here:

| Failure Type | Where to Look | Likely Fix |
|--------------|---------------|------------|
| `lint` job failed | GitHub Actions > lint | Fix frontmatter or forbidden patterns |
| `pack-check` job failed | GitHub Actions > pack-check | Fix pack structure issues |
| `demoswarm-smoke` job failed | GitHub Actions > demoswarm-smoke | Fix CLI or Rust tooling |
| `runs-tools-tests` job failed | GitHub Actions > runs-tools-tests | Fix Rust tests |
| `cargo-audit` job failed | GitHub Actions > cargo-audit | Update vulnerable dependencies |
| `doc-drift` job failed | GitHub Actions > doc-drift | Update stale doc references |
| CodeRabbit comment | PR comments | Address feedback per type |

---

## Understanding GitHub Actions Jobs

The pack CI runs six jobs defined in `.github/workflows/pack.yml`:

### 1. `lint` Job

**What it checks:**
- **Portable .claude shape** (`check_portable_claude.py`): Ensures `.claude/` contains no harness-specific references (Flow Studio, localhost:5000, etc.)
- **Frontmatter validation** (`lint_frontmatter.py`): Verifies agents have `name:` and `description:`, commands have `description:`, skills have `SKILL.md` with required fields
- **Agent prompt validation** (`validate_agent_prompts.py`): Validates agent structure, detects duplicate sections, verifies handoff targets exist, checks skill invocations

**Common failures:**

| Error | Cause | Fix |
|-------|-------|-----|
| "Found harness-specific references" | `.claude/` files contain forbidden patterns | Remove Flow Studio, swarm/, or other harness references |
| "missing 'name:' in frontmatter" | Agent file missing required YAML field | Add `name:` to frontmatter |
| "missing 'description:' in frontmatter" | Agent/command/skill missing description | Add `description:` to frontmatter |
| "missing SKILL.md" | Skill directory without required file | Create `SKILL.md` with name and description |
| "missing required section 'Handoff Targets'" | Agent missing required section | Add `## Handoff Targets` section |
| "duplicate section '## Output'" | Agent has repeated `##` header | Remove duplicate section header |
| "references non-existent agent" | Handoff target doesn't exist | Fix reference or create the missing agent |
| "references non-existent skill" | Skill invocation references missing skill | Fix reference or create the missing skill |

**How to fix locally:**

```bash
# Check portable shape
python scripts/check_portable_claude.py

# Check frontmatter
python scripts/lint_frontmatter.py

# Validate agent prompts
python scripts/validate_agent_prompts.py
```

### 2. `pack-check` Job

**What it checks:**
- Full pack validation via Rust-based pack-check tool
- Structure, contracts, agent definitions, flow definitions
- Cross-references and consistency

**Common failures:**

| Error | Cause | Fix |
|-------|-------|-----|
| "Agent X not found" | Referenced agent doesn't exist | Create the agent or fix the reference |
| "Flow Y missing required station" | Flow definition incomplete | Add missing station to flow |
| "Contract violation" | Agent output doesn't match schema | Fix agent output or update contract |
| "Broken link" | Doc references non-existent file | Fix the link or create the target |

**How to fix locally:**

```bash
# Run pack-check
bash .claude/scripts/pack-check.sh

# If not installed, install first:
cargo install --path tools/demoswarm-pack-check --root .demoswarm
```

### 3. `demoswarm-smoke` Job

**What it checks:**
- CLI tool installation works
- Basic commands execute (`time now`, `count pattern`)

**Common failures:**

| Error | Cause | Fix |
|-------|-------|-----|
| "cargo install failed" | Rust compilation error | Fix Rust code in `tools/demoswarm-runs-tools/` |
| "command not found" | CLI binary not built correctly | Check Cargo.toml and binary targets |
| "smoke test failed" | CLI command returns error | Debug the specific failing command |

**How to fix locally:**

```bash
# Install CLI
cargo install --path tools/demoswarm-runs-tools --root .demoswarm

# Run smoke tests
bash .claude/scripts/demoswarm.sh time now
bash .claude/scripts/demoswarm.sh count pattern --file CLAUDE.md --regex '^#'
```

### 4. `runs-tools-tests` Job

**What it checks:**
- Rust unit tests for the CLI tooling crate (`tools/demoswarm-runs-tools/`)

**Common failures:**

| Error | Cause | Fix |
|-------|-------|-----|
| "test X failed" | Test assertion failed | Fix the code or update the test |
| "compilation error" | Rust syntax/type error | Fix the Rust code |

**How to fix locally:**

```bash
cargo test --manifest-path tools/demoswarm-runs-tools/Cargo.toml
```

### 5. `cargo-audit` Job

**What it checks:**
- Security vulnerabilities (CVEs) in Rust dependencies for both tools:
  - `tools/demoswarm-pack-check/`
  - `tools/demoswarm-runs-tools/`

**Common failures:**

| Error | Cause | Fix |
|-------|-------|-----|
| "RUSTSEC-YYYY-NNNN" | Known vulnerability in dependency | Update the affected crate |
| "Crate X has N vulnerabilities" | Multiple CVEs in one dependency | Update to patched version |
| "unmaintained crate" | Dependency no longer maintained | Consider replacing or accept risk |

**How to fix locally:**

```bash
# Check for vulnerabilities
cargo audit --manifest-path tools/demoswarm-pack-check/Cargo.toml
cargo audit --manifest-path tools/demoswarm-runs-tools/Cargo.toml

# Update a specific dependency
cargo update -p <crate-name>

# Update all dependencies
cargo update
```

**Note:** If a vulnerability exists but no patch is available, you may need to:
1. Wait for upstream fix
2. Pin to a specific version with known-safe behavior
3. Replace the dependency with an alternative

### 6. `doc-drift` Job

**What it checks:**
- Stale skill name references (legacy monolith skill was split into `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`)
- Old CLI interface patterns (deprecated flags)
- Required docs exist (quickstart, releasing guide, CLI reference, etc.)
- Legacy Python fallback not promoted over Rust CLI

**Common failures:**

| Error | Cause | Fix |
|-------|-------|-----|
| "stale skill-name references" | Old monolith skill name in docs | Update to `runs-derive`, `runs-index`, `openq-tools`, or `secrets-tools` |
| "legacy Open Questions flags" | Deprecated flag usage | Update to current CLI interface |
| "yaml count-items flag mismatch" | Wrong flag name | Use `--item-regex` instead |
| "inv get flag mismatch" | Wrong flag name | Use `--marker` instead |
| "Missing required doc" | Required file doesn't exist | Create the required documentation |

**How to fix locally:**

```bash
bash scripts/check-doc-drift.sh
```

---

## Interpreting CodeRabbit Feedback

CodeRabbit provides automated code review comments. Types of feedback:

### Code Quality Issues

| Issue Type | Example | How to Address |
|------------|---------|----------------|
| **Style** | "Consider using early return" | Route to fixer or ignore if minor |
| **Complexity** | "Function too long" | Route to code-implementer for refactor |
| **Documentation** | "Missing docstring" | Route to doc-writer |
| **Security** | "Potential injection" | MUST address before merge |
| **Performance** | "Consider caching" | Evaluate, address if material |

### Suggested Changes

CodeRabbit may suggest specific code changes:
1. **Review the suggestion** - Is it correct and aligned with intent?
2. **If helpful:** Apply via the "Commit suggestion" button or route to code-implementer
3. **If not helpful:** Reply explaining why (for CodeRabbit learning)

### False Positives

If CodeRabbit flags something incorrectly:
1. **Reply to the comment** explaining why it's not an issue
2. **Continue with merge** if the PR is otherwise ready
3. **Note patterns** that produce false positives for future reference

---

## Common Failure Patterns and Resolutions

### Pattern 1: Lint Failures

**Symptom:** `lint` job fails on PR.

**Resolution:**

```bash
# Run auto-linter skill
/auto-linter
```

Or manually:

```bash
# Check what's failing
python scripts/check_portable_claude.py
python scripts/lint_frontmatter.py

# Fix the identified issues
# Then commit
```

### Pattern 2: Test Failures

**Symptom:** `runs-tools-tests` job fails.

**Resolution:**

1. Read the test output to identify failing tests
2. Determine if the issue is in code or tests:
   - **Code bug:** Route to code-implementer
   - **Test bug:** Route to test-author
   - **Both:** Fix code first, then tests

```bash
# Run tests locally to reproduce
cargo test --manifest-path tools/demoswarm-runs-tools/Cargo.toml

# Run specific failing test
cargo test --manifest-path tools/demoswarm-runs-tools/Cargo.toml test_name
```

### Pattern 3: Pack-Check Failures

**Symptom:** `pack-check` job fails.

**Resolution:**

```bash
# Run pack-check to see errors
bash .claude/scripts/pack-check.sh

# Common fixes:
# - Add missing frontmatter fields
# - Fix broken cross-references
# - Ensure agents exist before referencing them
# - Match schemas to actual outputs
```

### Pattern 4: Doc Drift

**Symptom:** `doc-drift` job fails with stale references.

**Resolution:**

```bash
# Identify stale references
bash scripts/check-doc-drift.sh

# Common updates:
# - legacy monolith skill name -> appropriate new skill name
# - Old CLI flags -> current flag names
# - Missing docs -> create required files
```

---

## Decision Tree: Fix vs Bounce

When CI fails, decide whether to fix forward or bounce to an earlier flow:

```
CI Failure
    |
    v
+----------------------------------+
| What type of failure?            |
+----------------------------------+
    |
    +---> Lint/Format (mechanical)        --> Fix Forward (auto-linter)
    |
    +---> Test failure (new tests)        --> Fix Forward (code-implementer)
    |
    +---> Test failure (existing tests)   --> Evaluate: regression?
    |           |
    |           +---> Minor regression    --> Fix Forward (code-implementer)
    |           +---> Major regression    --> Bounce to Build (Flow 3)
    |
    +---> Pack-check (structure)          --> Evaluate scope
    |           |
    |           +---> Missing frontmatter --> Fix Forward (add fields)
    |           +---> Contract mismatch   --> Bounce to Plan (Flow 2)
    |           +---> Missing agent       --> Bounce to Build (Flow 3)
    |
    +---> Doc drift (stale refs)          --> Fix Forward (doc-writer)
    |
    +---> Compilation error               --> Bounce to Build (Flow 3)
```

### Use Fix-Forward (Flow 4/5) When

- **Mechanical issues:** Lint, format, import sorting
- **Minor test fixes:** Edge cases, assertion tweaks
- **Documentation gaps:** Missing docstrings, stale references
- **Frontmatter issues:** Adding required fields
- **Deterministic repairs:** Standard tools can fix automatically

### Bounce to Build (Flow 3) When

- **Logic errors:** Code doesn't do what it should
- **Missing implementation:** Feature not complete
- **Regression:** Existing tests now fail
- **Contract violations:** API doesn't match spec

### Bounce to Plan (Flow 2) When

- **Design flaws:** Architecture doesn't support requirements
- **Contract conflicts:** API design needs rethinking
- **Scope issues:** Requirements unclear or conflicting

---

## Using fix-forward-runner

For mechanical fixes at Gate (Flow 5), the fix-forward-runner executes prescribed fixes:

### When fix-forward-runner Applies

1. **gate-fixer** diagnoses issues and creates a fix plan
2. Plan specifies `fix_forward_eligible: true`
3. **fix-forward-runner** executes the plan exactly

### Example Fix-Forward Plan

```yaml
fix_forward_eligible: true
scope: [FORMAT, LINT_AUTOFIX]
rationale: "All findings are formatter-fixable"

apply_steps:
  - id: FF-APPLY-001
    purpose: "Apply Python formatter"
    command: "ruff format src/ tests/"

verify_steps:
  - id: FF-VERIFY-001
    purpose: "Verify clean"
    command: "ruff check src/ tests/"

change_scope:
  allowed_globs: ["src/**/*.py", "tests/**/*.py"]
  deny_globs: [".runs/**", ".github/**"]
```

### What fix-forward-runner Does

1. Parses the plan from `gate_fix_summary.md`
2. Records baseline (current HEAD, uncommitted changes)
3. Executes `apply_steps` exactly as written
4. Enforces scope (fails if files outside `allowed_globs` are touched)
5. Runs `verify_steps` to confirm fixes worked
6. Writes `fix_forward_report.md` with full audit trail

### After fix-forward-runner Completes

- **Success with changes:** Route to repo-operator to commit, then receipt-checker to reseal
- **Success, no changes:** Continue to merge-decider
- **Plan ineligible:** Route to merge-decider (document why)
- **Execution failed:** Route to code-implementer for manual fix

---

## Quick Reference

| CI Job | Common Fix | Command |
|--------|------------|---------|
| lint | Fix frontmatter, remove harness refs, fix agent structure | `python scripts/lint_frontmatter.py && python scripts/validate_agent_prompts.py` |
| pack-check | Fix pack structure | `bash .claude/scripts/pack-check.sh` |
| demoswarm-smoke | Fix CLI/Rust code | `bash .claude/scripts/demoswarm.sh time now` |
| runs-tools-tests | Fix Rust tests | `cargo test --manifest-path tools/demoswarm-runs-tools/Cargo.toml` |
| cargo-audit | Update vulnerable deps | `cargo audit && cargo update -p <crate>` |
| doc-drift | Update stale references | `bash scripts/check-doc-drift.sh` |

---

## See Also

- [Troubleshoot](troubleshoot.md) - General troubleshooting guide
- [Bounded Fix-Forward](../explanation/bounded-fix-forward.md) - When to fix forward vs bounce
- [Orchestrator Decision Tree](orchestrator-decision-tree.md) - Routing decisions
- [Failure Recovery](failure-recovery.md) - Nuclear delete/restart procedures
- [Review a Swarm PR](review-a-swarm-pr.md) - PR review process
