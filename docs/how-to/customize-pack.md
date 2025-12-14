# Customizing DemoSwarm

This guide covers how to adapt the DemoSwarm **Claude Code pack** to your stack, repo layout, and tooling.

The pack is designed to be **layout-neutral**:
- durable artifacts always live under `.runs/<run-id>/...`
- code/test/doc locations are **project-defined**
- Flow 3 scope is constrained by `subtask_context_manifest.json`, not by assuming `src/` or `tests/`

---

## What you typically customize

1) **Commands** (tests, lint, policy)
- `.claude/skills/test-runner/SKILL.md`
- `.claude/skills/auto-linter/SKILL.md`
- `.claude/skills/policy-runner/SKILL.md`

2) **Repo layout + intended change surface** (Flow 3)
- Use `/customize-pack` (recommended) and ensure `context-loader` produces a correct `.runs/<run-id>/build/subtask_context_manifest.json`.

3) **GitHub integration** (optional)
- If `gh` is missing/unauthenticated, the pack still produces local artifacts.
- Reporting ops (`gh-issue-manager`, `gh-reporter`) are designed to **SKIP** when GH tooling/auth isn't available.
- Release ops in Flow 5 (merge/tag/release) require working GitHub integration if you want automation.

---

## Prerequisites

### Required tools (for pack-check and mechanical agents)

| Tool | Purpose | Required |
|------|---------|----------|
| `bash` | Pack scripts + shell-driven skills | Yes |
| `git` | Local repo operations (repo-operator) | Yes |

Notes:
- Some scripts may rely on **bash 4+** features. If you hit issues on macOS default bash, install a newer bash and rerun.
- Mechanical operations (counting, extraction) use the **`demoswarm` CLI** via shims, not raw shell utilities.

### Optional tools

| Tool | Purpose | Fallback |
|------|---------|----------|
| `jq` | JSON pretty-printing | `python -m json.tool` |
| `gh` | GitHub issues/comments + (Flow 5) GitHub-native release ops | Reporting ops SKIPPED; release ops require human handling |
| `curl` | Smoke/health checks (Flow 5 smoke-verifier) | Smoke checks become artifact-only / N/A |

---

## Platform support

### Linux / macOS
Works out of the box with standard Unix utilities.

macOS note:
- Differences like BSD `sed` vs GNU `sed` can matter if you copy/paste shell snippets. Prefer portable patterns (no `sed -i` assumptions).

### Windows
Recommended: **WSL2** for best compatibility.

Alternatives (more brittle):
- Git Bash (works for many cases; ensure `jq` is installed)
- PowerShell + Unix tools (scoop/choco), but expect edge cases

---

## Adapting to your stack

### Test command

Edit `.claude/skills/test-runner/SKILL.md` to match your repo:

**Rust**
```bash
cargo test --workspace --tests
```

**Node.js**
```bash
pnpm test
# or npm test / yarn test
```

**Python**
```bash
python -m pytest -q
# or pytest -q
```

**Go**
```bash
go test ./...
```

### Lint/format command

Edit `.claude/skills/auto-linter/SKILL.md`:

**Rust**
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

**Node.js**
```bash
pnpm lint
pnpm format
```

**Python**
```bash
ruff check --fix .
ruff format .
```

### Policy checks

Edit `.claude/skills/policy-runner/SKILL.md` to run *your* checks:

```bash
./scripts/check-policies.sh
```

If you don't have policy-as-code yet, keep it as a no-op or a lightweight sanity check. The important part is: **the skill must not lie** about what it ran.

---

## Repo layout and Flow 3 scope

### The pack does not assume `src/` / `tests/`

Avoid editing agent prompts to "teach" hardcoded paths.

Instead:

* Use the Flow 3 context mechanism:
  * `context-loader` writes `.runs/<run-id>/build/subtask_context_manifest.json`
  * implementers/fixer/doc-writer treat that manifest as the scope boundary
* If a repo has multiple apps/packages:
  * ensure the manifest points at the correct package(s) for the subtask

### BDD features are run-local

Flow 1 BDD artifacts always live under:

* `.runs/<run-id>/signal/features/*.feature`

(They are not stored in repo-root `features/` by default.)

---

## Receipts and mechanical counting (important)

The plan is to keep receipts **layout-neutral** and **mechanical**.

Do:

* Prefer counts derived from `.runs/` artifacts (summaries + inventory markers).
* Keep stable marker prefixes intact in summary artifacts (e.g., inventory sections).

Do not:

* "Fix" receipts by scanning repo folders like `tests/` or `src/` unless you're intentionally making the pack repo-specific.

If you need different counts:

* adjust the *producer artifacts* to emit stable markers the cleanup agents can count (preferred),
* or adjust cleanup patterns to count from `.runs/<run-id>/...` artifacts (acceptable),
* avoid repo scanning patterns as the default.

---

## Git provider adaptation (advanced)

The pack ships with GitHub-oriented agents (`gh-*`) and Flow 5 release ops that assume GitHub.

If you're on GitLab/Azure/Bitbucket, you have two sane options:

1. **Skip GH integration**

* run flows locally
* keep artifacts and receipts
* handle issues/merge/release manually in your provider

2. **Fork integration agents + update flow commands**

* create equivalents (e.g., `glab-issue-manager`, `glab-reporter`, `glab-researcher`)
* update `.claude/commands/flow-*.md` to reference them
* keep the two-gate rule and safe-bail semantics unchanged

Treat "swap commands in place" as insufficient unless you also update contracts, gating language, and pack-check.

---

## CI/CD integration

Default:

* Flow 5 `deploy-monitor` is written for GitHub Actions (via `gh`).

If you use another CI system:

* adapt `deploy-monitor` to query your CI for run status (read-only) and write the same `verification_report.md` contract.
* keep it evidence-first: URLs + concise summaries.

---

## Quick Start: `/customize-pack`

Run:

```text
/customize-pack
```

Use it to:

* set test/lint/policy commands in skill files
* configure how Flow 3 discovers scope (manifest expectations)

The command should also write a receipt of what it changed (path should be shown by the command output). Keep that receipt in the repo.

---

## Verifying your customization

After customizing, verify the pack works with your settings:

### Run pack validation

```bash
bash .claude/scripts/pack-check.sh
```

All checks should pass. If not, review the errors and adjust customizations.

### Run a toy flow and check receipts

```text
/flow-1-signal test-custom "Test my customization"
```

Then verify artifacts were created correctly:

```bash
# Check receipt status
bash .claude/scripts/demoswarm.sh receipt get \
  --file ".runs/test-custom/signal/signal_receipt.json" \
  --key "status"
# Should return: VERIFIED

# Check counts were derived
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/test-custom/signal/requirements.md" \
  --regex '^### REQ-' \
  --null-if-missing
# Should return a number > 0
```

If receipts show `UNVERIFIED` or counts are `null`, check `cleanup_report.md` for derivation details.

---

## Troubleshooting

### `jq: command not found`

Install jq.

* macOS: `brew install jq`
* Ubuntu/Debian: `apt install jq`
* Windows (scoop): `scoop install jq`

### `sed` portability problems

Avoid `sed -i` assumptions. Prefer portable patterns or handle macOS/Linux differences explicitly.

### `gh: command not found` / `gh: not authenticated`

* Reporting ops will be SKIPPED and local artifacts still written.
* Flow 5 release ops won't run automatically without working GitHub tooling/auth.

Install/auth (GitHub CLI docs):

```bash
# macOS
brew install gh
gh auth login
```

---

## Quick reference: files you'll edit most

| To customize…               | Edit                                                                          |
| --------------------------- | ----------------------------------------------------------------------------- |
| Test command                | `.claude/skills/test-runner/SKILL.md`                                         |
| Lint/format command         | `.claude/skills/auto-linter/SKILL.md`                                         |
| Policy checks               | `.claude/skills/policy-runner/SKILL.md`                                       |
| Flow 3 scope discovery      | `/customize-pack` + `context-loader` / manifest expectations                  |
| GitHub integration behavior | `gh-issue-manager`, `gh-reporter`, `gh-researcher` + flow commands (advanced) |
