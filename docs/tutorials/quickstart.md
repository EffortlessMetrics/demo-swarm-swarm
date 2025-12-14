# Quickstart

> Get the pack working in 5 minutes.

**What this is:** A minimal tutorial to install the pack and run your first flow.

**Who it's for:** New users who want to see the pack work before customizing.

**Prereqs:**
- A dedicated swarm clone of your repo (Git-initialized)
- Claude Code installed
- Shell with `bash`, `git`, `jq` available
- Rust 1.89+ or Python 3.8+ (Rust preferred; Python is the fallback)
- Optional: GitHub CLI (`gh`) for GitHub posting (flows still run without it)

**What "done" looks like:** `.runs/<run-id>/signal/signal_receipt.json` exists.

---

## Adopt DemoSwarm in your repo (swarm clone)

Run DemoSwarm in a separate `<repo>-swarm` clone. The pack creates `run/<run-id>` branches and commits `.runs/` as an audit trail in that clone. Open PRs back to your upstream repo when ready.

### 1. Create a swarm clone
```bash
git clone git@github.com:org/your-repo.git your-repo-swarm
cd your-repo-swarm
```

### 2. Copy the pack components
Copy from this repo into the root of your swarm clone:

- `.claude/` (pack)
- `tools/` (vendored tooling sources)
- `scripts/` (bootstrap and shims)

```bash
cp -r /path/to/demo-swarm/.claude .
cp -r /path/to/demo-swarm/tools .
cp -r /path/to/demo-swarm/scripts .
```

If you already have the `demoswarm` tooling installed globally, you can copy only `.claude/` and skip `tools/` + `scripts/`.

### 3. Bootstrap tooling
Compile/install the local tools for your environment (outputs to `.demoswarm/bin/`):

```bash
bash scripts/bootstrap.sh
```

### 4. Open in Claude Code

Open your sandbox repo in Claude Code. It will discover:

- `.claude/commands/` (slash commands)
- `.claude/agents/` (subagents)
- `.claude/skills/` (skills)

---

### 5. (Optional) Customize for your stack

```text
/customize-pack
```

This adapts test/lint commands to your repo. If you're just exploring, you can skip this.

---

## Run Flow 1

```text
/flow-1-signal "Add a demoswarm version CLI subcommand that prints JSON with tool version info"
```

Flow 1 will:

1. Establish the run directory (`.runs/<run-id>/`)
2. Normalize the signal
3. Write requirements and BDD scenarios
4. Compute `signal_receipt.json`
5. Checkpoint artifacts
6. (If `gh` is authenticated) Create/update a GitHub issue

---

## Verify success

Check that artifacts exist:

```bash
ls .runs/*/signal/
# Should show: requirements.md, features/, signal_receipt.json, etc.
```

Check the receipt using the demoswarm CLI:

```bash
# Get receipt status
bash .claude/scripts/demoswarm.sh receipt get \
  --file ".runs/add-a-demoswarm-version-cli-subcommand/signal/signal_receipt.json" \
  --key "status"
# Should return: VERIFIED

# Count requirements generated
bash .claude/scripts/demoswarm.sh count pattern \
  --file ".runs/add-a-demoswarm-version-cli-subcommand/signal/requirements.md" \
  --regex '^### REQ-' \
  --null-if-missing
# Should return a number > 0

# Count BDD scenarios
bash .claude/scripts/demoswarm.sh count bdd \
  --dir ".runs/add-a-demoswarm-version-cli-subcommand/signal/features" \
  --null-if-missing
# Should return a number > 0
```

Or view the full receipt:

```bash
cat .runs/*/signal/signal_receipt.json
# Or with jq: cat .runs/*/signal/signal_receipt.json | jq .
```

If `status` is `UNVERIFIED`, check `blockers` and `missing_required` fields for what's missing.

---

## What just happened?

Flow 1 transformed your free-text input into:

| Artifact | Purpose |
|----------|---------|
| `requirements.md` | Testable requirements (REQ-### markers) |
| `features/*.feature` | BDD scenarios |
| `verification_notes.md` | How BDD maps to requirements |
| `signal_receipt.json` | Flow summary (counts, status, routing) |

The receipt is the source of truth. Reporters read receipts; they don't recompute.

---

## Why this demo goal?

The `demoswarm version` command is the canonical demo because it's **self-referential**: you're building a feature *for the pack's own CLI tooling*. This exercises the pack's core primitives (receipts, mechanical derivation, gating) without inventing a separate "product."

---

## Next steps

| Goal | Command/Doc |
|------|-------------|
| Continue to Plan | `/flow-2-plan` |
| Customize for your stack | [customize-pack.md](../how-to/customize-pack.md) |
| Validate pack contracts | [validation-run.md](validation-run.md) |
| Demo to others | [walkthrough.md](walkthrough.md) |
| Full reference | [CLAUDE.md](../../CLAUDE.md) |

---

## Troubleshooting

### "Receipt status is UNVERIFIED"

Check the `blockers` field in the receipt. Common causes:

- Missing prerequisites (e.g., `jq` not installed)
- Critic flagged issues that need upstream clarification

### "GitHub operations skipped"

This can happen for multiple reasons. Diagnose in order:

1. **Gates blocked?** Check `.runs/<run-id>/<flow>/secrets_status.json`:
   - `safe_to_publish: false` -> secrets detected
   - `proceed_to_github_ops: false` -> repo hygiene issue

2. **Auth missing?** Run `gh auth status`:
   - If not authenticated, GH agents skip gracefully

Flows still work locally - GitHub posting is optional. Local artifacts are always written.

### "Command not found"

Ensure `.claude/` is at repo root and you're in a Claude Code session.
