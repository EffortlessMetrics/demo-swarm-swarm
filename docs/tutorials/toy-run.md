# Toy Run: Flow 1 Without GitHub

A quick guide to running Flow 1 (Signal) end-to-end and seeing artifacts generated, without requiring GitHub authentication.

---

## Purpose

This guide shows how to:

- Run a single flow to see the pack in action
- Understand the artifact structure in `.runs/`
- Verify the pack works before integrating with GitHub

The demo goal (`demoswarm version`) is intentionally self-referential: it's a feature *for this pack's tooling*, exercising the exact machinery the pack is built around.

---

## Prerequisites

1. **Rust toolchain** - Install from [rustup.rs](https://rustup.rs/)

2. **Run bootstrap** - From repo root:

   ```bash
   bash scripts/bootstrap.sh
   ```

   This installs `pack-check` and `demoswarm` CLI tools to `.demoswarm/bin/`.

3. **Shell utilities** - `bash`, `git` (required)

4. **Optional: `jq`** - For pretty-printing JSON. Fallback:

   ```bash
   python -m json.tool < file.json
   ```

GitHub CLI (`gh`) is **not required** for this guide. GH agents will be SKIPPED (not failed).

---

## Running Flow 1

From repo root, in Claude Code:

```text
/flow-1-signal toy-run "Add a demoswarm version CLI subcommand that prints JSON with tool version info. Constraints: Must work via bash .claude/scripts/demoswarm.sh version. Output is JSON to stdout (no logs/no extra text). No network calls; no GitHub required. Include at least: demoswarm_version, pack_version (if available), git_sha (optional/null-safe)."
```

The flow will:

1. Create run infrastructure in `.runs/toy-run/`
2. Analyze the feature request
3. Generate requirements, BDD features, and verification notes
4. Produce a receipt summarizing the flow outcome
5. SKIP GitHub operations (no `gh` auth)

---

## Expected Artifacts

After Flow 1 completes, you should see artifacts in `.runs/toy-run/signal/`.

### Minimum Success Proof (must exist)

```
.runs/
  index.json                     # Global run index (updated)
  toy-run/
    run_meta.json                # Run identity and metadata
    signal/
      signal_receipt.json        # Mechanical counts and status
      requirements.md            # REQ-* identifiers
      features/
        *.feature                # At least one BDD scenario file
```

If these exist and `signal_receipt.json` shows `status: VERIFIED`, the flow succeeded.

### Common Additional Artifacts

These are produced by various agents and may or may not appear depending on flow progression:

```
signal/
  problem_statement.md           # Normalized problem framing
  requirements_critique.md       # Harsh review of requirements (contains Machine Summary)
  bdd_critique.md                # Harsh review of BDD scenarios (contains Machine Summary)
  verification_notes.md          # Verification criteria
  stakeholders.md                # Identified stakeholders
  early_risks.md                 # Early risk identification
  scope_estimate.md              # T-shirt size estimate
  open_questions.md              # Ambiguities and defaults
  cleanup_report.md              # Flow cleanup summary
  flow_plan.md                   # Durable flow state
  gh_issue_status.md             # GitHub issue status (SKIPPED if no gh auth)
  secrets_status.json            # Secrets scan result
  git_status.md                  # Repo operator status
```

### Key Files to Inspect

**`.runs/toy-run/signal/signal_receipt.json`**

```json
{
  "run_id": "toy-run",
  "flow": "signal",
  "status": "VERIFIED",
  "recommended_action": "PROCEED",
  "counts": {
    "requirements": 3,
    "features": 1,
    "scenarios": 4
  },
  "completed_at": "2025-..."
}
```

**`.runs/toy-run/run_meta.json`**

```json
{
  "run_id": "toy-run",
  "task_title": "Add demoswarm version CLI subcommand",
  "created_at": "2025-...",
  "flows_started": ["signal"],
  "source": "manual"
}
```

**`.runs/index.json`**

```json
{
  "version": 1,
  "runs": [
    {
      "run_id": "toy-run",
      "status": "VERIFIED",
      "last_flow": "signal",
      "updated_at": "2025-..."
    }
  ]
}
```

---

## GitHub Operations: SKIPPED Without Auth

When `gh` is not authenticated, these agents are **SKIPPED** (not failed):

- `gh-issue-manager` (issue creation/update)
- `gh-reporter` (status comments)

Look for `gh_issue_status.md` or similar files indicating **SKIPPED** status.

The flow still completes successfully with local artifacts. This is expected behavior—the pack is designed to work offline.

### Separate Concern: Secrets Gate

The **secrets gate** (`safe_to_publish`) is independent of GitHub auth. Even without `gh`:

- Secrets scanning still runs
- `safe_to_publish: true` means no secrets found in artifacts
- `safe_to_publish: false` means secrets were detected (unrelated to GH auth)

The **repo hygiene gate** (`proceed_to_github_ops`) determines whether GH operations should run, but without `gh` auth, GH agents are skipped regardless.

## Git Side Effects

Flow 1 creates **local git activity** as part of its audit trail:

- Creates a run branch: `run/toy-run`
- May create checkpoint commits for artifacts

This is local only. Without `gh auth`, nothing is pushed to GitHub. Your local repo will have a new branch and possibly new commits, but remote operations are skipped.

---

## Inspecting Results

### Check the Receipt

```bash
cat .runs/toy-run/signal/signal_receipt.json | jq .
# Or without jq:
python -m json.tool < .runs/toy-run/signal/signal_receipt.json
```

Key fields:

- `status`: VERIFIED means flow completed successfully
- `recommended_action`: PROCEED means ready for next flow
- `counts`: Mechanical counts of artifacts produced

### Check Machine Summaries

Critic artifacts (not `requirements.md` itself) contain Machine Summary blocks:

```bash
bash .claude/scripts/demoswarm.sh ms get \
  --file .runs/toy-run/signal/requirements_critique.md \
  --section "## Machine Summary" \
  --key status
```

(If `requirements_critique.md` doesn't exist, try `bdd_critique.md`.)

### Verify Index Updated

```bash
cat .runs/index.json | jq '.runs[] | select(.run_id == "toy-run")'
# Or without jq:
python -c "import json; d=json.load(open('.runs/index.json')); print([r for r in d['runs'] if r['run_id']=='toy-run'])"
```

---

## Sanity Checks: Verify Tooling Works

After the flow completes, run these commands to confirm the `demoswarm` CLI is working against your generated artifacts:

### Count Requirements

```bash
bash .claude/scripts/demoswarm.sh count pattern \
  --file .runs/toy-run/signal/requirements.md \
  --regex '^### REQ-'
```

Should return a number > 0 if requirements were generated.

### Count BDD Scenarios

```bash
bash .claude/scripts/demoswarm.sh count bdd \
  --dir .runs/toy-run/signal/features
```

Should return a number > 0 if feature files were generated.

### Read Receipt Field

```bash
bash .claude/scripts/demoswarm.sh receipt get \
  --file .runs/toy-run/signal/signal_receipt.json \
  --key status
```

Should return `VERIFIED` (or `UNVERIFIED` if the flow completed with gaps).

These checks prove the Rust tooling is installed and working against real artifacts.

---

## Continuing Through Flows 2–5

After Flow 1 succeeds, continue the demo:

```text
/flow-2-plan toy-run
/flow-3-build toy-run
/flow-4-review toy-run
/flow-5-gate toy-run
```

### Full Demo Proof Points

After Flows 1–5 complete:

- `*_receipt.json` exists for each flow (signal, plan, build, review, gate)
- Gate artifacts show tests/lint ran
- If the feature was implemented: `demoswarm version` works via the shim and returns parseable JSON

```bash
# Verify the implemented feature (after Flow 3)
bash .claude/scripts/demoswarm.sh version
# Should output JSON with demoswarm_version, pack_version, git_sha
```

---

## Cleanup

### `.runs/` is Committed by Default

The `.runs/` directory is **not gitignored**. This is intentional:

- Artifacts are inspectable and reviewable
- Receipts provide audit trail
- Flow state persists across sessions

### Removing a Toy Run

If you want to clean up:

```bash
rm -rf .runs/toy-run
# Then manually remove the entry from .runs/index.json
```

Or keep it as a reference for how artifacts should look.

---

## Next Steps

After seeing Flow 1 work:

1. **Continue the flow sequence**: `/flow-2-plan toy-run` (uses Signal outputs)
2. **Enable GitHub**: `gh auth login` then rerun to see GH integration
3. **Customize for your stack**: `/customize-pack` to set test/lint commands
4. **See full validation guide**: [validation-run.md](validation-run.md)

---

## Troubleshooting

### "pack-check failed"

Run bootstrap first:

```bash
bash scripts/bootstrap.sh
```

### "No artifacts created"

Check that you ran from repo root. All paths are repo-root-relative.

### "Receipt shows UNVERIFIED"

This is not a failure. UNVERIFIED means the flow completed but with gaps or missing inputs. Check `blockers` in the receipt for details.

### "CANNOT_PROCEED in receipt"

This indicates mechanical failure (I/O, permissions, tooling). Check `missing_required` for the specific issue.
