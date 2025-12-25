# Quickstart

> Get the pack working in 5 minutes.

**Prereqs:**
- A dedicated swarm clone of your repo (Git-initialized)
- Claude Code installed
- Shell with `bash`, `git`, `jq`
- Rust 1.89+ or Python 3.8+
- Optional: `gh` CLI for GitHub posting

**Done when:** `.runs/<run-id>/signal/signal_receipt.json` exists with `status: VERIFIED`.

---

## 1. Create a swarm clone

```bash
git clone git@github.com:org/your-repo.git your-repo-swarm
cd your-repo-swarm
```

## 2. Copy the pack

```bash
cp -r /path/to/demo-swarm/.claude .
cp -r /path/to/demo-swarm/tools .
cp -r /path/to/demo-swarm/scripts .
```

## 3. Bootstrap tooling

```bash
bash scripts/bootstrap.sh
```

## 4. Open in Claude Code

Open the repo. Claude Code discovers `.claude/commands/`, `.claude/agents/`, `.claude/skills/`.

## 5. (Optional) Customize

```text
/customize-pack
```

Adapts test/lint commands to your repo. Skip if just exploring.

---

## Run Flow 1

```text
/flow-1-signal "Add a health check endpoint"
```

Flow 1 transforms free-text into:
- `requirements.md` — testable requirements
- `features/*.feature` — BDD scenarios
- `signal_receipt.json` — flow summary (counts, status)

---

## Verify

```bash
ls .runs/*/signal/
# requirements.md, features/, signal_receipt.json

cat .runs/*/signal/signal_receipt.json | jq .status
# Should return: "VERIFIED"
```

If `UNVERIFIED`, check `blockers` field in the receipt.

---

## Next steps

| Goal | Command/Doc |
|------|-------------|
| Continue to Plan | `/flow-2-plan` |
| Customize for your stack | [customize-pack.md](../how-to/customize-pack.md) |
| Validate pack contracts | [validation-run.md](validation-run.md) |
| Full reference | [CLAUDE.md](../../CLAUDE.md) |

---

## Troubleshooting

**"Receipt status is UNVERIFIED"**
Check `blockers` field. Common causes: missing prereqs, critic flagged issues.

**"GitHub operations skipped"**
Check `gh auth status`. Flows work locally without GitHub — posting is optional.

**"Command not found"**
Ensure `.claude/` is at repo root and you're in Claude Code.
