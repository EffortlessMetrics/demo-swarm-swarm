# Quickstart

> Get the pack working in 5 minutes.

**Prereqs:**

- Git-initialized repo (swarm clone recommended)
- Claude Code installed
- Shell with `bash`, `git`
- Rust 1.70+ (for CLI tools)
- Optional: `gh` CLI for GitHub integration

**Done when:** `.runs/<run-id>/signal/signal_receipt.json` exists with `status: VERIFIED`.

---

## 1. Set Up

### Create a swarm clone

```bash
git clone git@github.com:org/your-repo.git your-repo-swarm
cd your-repo-swarm
```

### Copy the pack

```bash
cp -r /path/to/demo-swarm/.claude .
cp -r /path/to/demo-swarm/tools .
cp -r /path/to/demo-swarm/scripts .
```

### Bootstrap tooling

```bash
bash scripts/bootstrap.sh
```

This installs `pack-check` and `demoswarm` CLI tools to `.demoswarm/bin/`.

---

## 2. Open in Claude Code

Open the repo. Claude Code discovers `.claude/commands/`, `.claude/agents/`, `.claude/skills/`.

Optional: run `/customize-pack` to adapt test/lint commands to your stack.

---

## 3. Run Flow 1

```text
/flow-1-signal "Add a health check endpoint"
```

Flow 1 transforms free-text into:

- `requirements.md` — testable requirements
- `features/*.feature` — BDD scenarios
- `signal_receipt.json` — flow summary

---

## 4. Verify

```bash
ls .runs/*/signal/
# requirements.md, features/, signal_receipt.json

cat .runs/*/signal/signal_receipt.json | jq .status
# Should return: "VERIFIED"
```

If `UNVERIFIED`, check `blockers` field in the receipt.

---

## Without GitHub

The pack works fully offline. When `gh` is not authenticated:

- GitHub agents (`gh-issue-manager`, `gh-reporter`) are **SKIPPED**, not failed
- Local artifacts are still written to `.runs/<run-id>/`
- Git operations create local branches/commits but don't push

To verify CLI tooling works against artifacts:

```bash
# Count requirements
bash .claude/scripts/demoswarm.sh count pattern \
  --file .runs/*/signal/requirements.md \
  --regex '^### REQ-'

# Count BDD scenarios
bash .claude/scripts/demoswarm.sh count bdd \
  --dir .runs/*/signal/features

# Read receipt status
bash .claude/scripts/demoswarm.sh receipt get \
  --file .runs/*/signal/signal_receipt.json \
  --key status
```

---

## Next Steps

| Goal                     | Command/Doc                                      |
| ------------------------ | ------------------------------------------------ |
| Continue to Plan         | `/flow-2-plan`                                   |
| Run all 7 flows          | [Walkthrough](walkthrough.md)                    |
| Customize for your stack | [customize-pack.md](../how-to/customize-pack.md) |
| Validate pack contracts  | [validation-run.md](validation-run.md)           |

---

## Troubleshooting

**"Receipt status is UNVERIFIED"**
Check `blockers` field. Common causes: missing prereqs, critic flagged issues. This is not failure—it means the flow completed with documented gaps.

**"CANNOT_PROCEED in receipt"**
Mechanical failure (I/O, permissions, tooling). Check `missing_required` field.

**"GitHub operations skipped"**
Check `gh auth status`. Flows work locally—GitHub posting is optional.

**"Command not found"**
Ensure `.claude/` is at repo root and you're in Claude Code.

**"pack-check failed"**
Run `bash scripts/bootstrap.sh` to install CLI tools.
