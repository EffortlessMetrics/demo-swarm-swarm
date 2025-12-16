# Troubleshoot

> Debug common failures and unexpected behavior.

**Goal:** Fix problems when flows don't work as expected.

---

## Quick diagnosis

Before diving in:

```bash
# Run canonical validation
bash .claude/scripts/pack-check.sh

# Check GitHub auth (if expecting GH ops)
gh auth status
```

---

## "Nothing posted to GitHub"

**Symptom:** Flow completed but no issue/comment appeared.

**Diagnosis:** Check the two gates.

### Step 1: Check secrets gate

Look in `.runs/<run-id>/<flow>/secrets_status.json`:

```json
{
  "status": "CLEAN",
  "safe_to_publish": true,
  ...
}
```

If `safe_to_publish: false`:
- Secrets were detected
- Check `secrets_scan.md` for what was flagged
- Fix the source, don't just redact

### Step 2: Check repo hygiene gate

Look for repo-operator's result in the flow output or `git_status.md`:

```yaml
## Repo Operator Result
proceed_to_github_ops: true
```

If `proceed_to_github_ops: false`:
- Anomaly detected (dirty tree outside allowlist)
- Check `anomaly_paths` field
- Review `git_status.md` for details

### Step 3: Check `gh` availability (separate concern)

```bash
gh auth status
```

If not authenticated, GH agents (`gh-issue-manager`, `gh-reporter`) skip gracefully.

**Important:** This is separate from the two gates above:
- **Gates** determine whether GH ops *should* run (control plane)
- **Auth** determines whether GH ops *can* run (availability)

Both gates can pass but GH ops still skip if `gh` isn't authenticated. The flow completes UNVERIFIED (external observability unavailable).

---

## "Counts are null in receipt"

**Symptom:** `<flow>_receipt.json` has `null` for some counts.

**Diagnosis:** This is often expected, not a failure.

Counts are null when:
- Upstream artifacts don't exist (out-of-order start)
- Stable markers aren't present in artifacts
- Artifacts are ambiguous (cleanup couldn't derive mechanically)

**Fix:** Check `cleanup_report.md` for derivation details. If markers are missing, the producer artifact needs stable marker prefixes.

---

## "Reseal loop won't terminate"

**Symptom:** secrets-sanitizer keeps modifying files; flow doesn't complete.

**Diagnosis:** Reseal is stuck.

**Normal behavior:**
```
cleanup → sanitizer → modified_files: true → cleanup → sanitizer → modified_files: false → done
```

**Stuck behavior:**
```
cleanup → sanitizer → modified_files: true → cleanup → sanitizer → modified_files: true → ...
```

**Fix:**
1. Check what's being modified (look at `secrets_scan.md`)
2. If legitimate (e.g., sensitive patterns in generated content), use safe-bail:
   - `repo-operator checkpoint_mode: local_only`
   - Flow completes UNVERIFIED with evidence
3. If a bug, report it

---

## "Microloop won't terminate"

**Symptom:** Critic keeps saying issues exist; author/implementer can't fix them.

**Diagnosis:** Route on the critic's `recommended_action` first (control plane wins).

Microloops route as follows:
- `status: CANNOT_PROCEED` → stop (FIX_ENV)
- `recommended_action: BOUNCE` → follow `route_to_flow/route_to_agent`
- `recommended_action: ESCALATE` → stop microloop; record evidence
- `recommended_action: RERUN` → rerun the specified agent (default author/implementer)
- `recommended_action: PROCEED` → proceed even if UNVERIFIED
- If `recommended_action` absent: use `can_further_iteration_help` as tie-breaker (`no` → proceed; `yes` → rerun)

If a critic keeps saying `RERUN` but the issues are unfixable locally, treat it as `ESCALATE` with crisp blockers and move to the next flow.

---

## "Anomaly detected"

**Symptom:** repo-operator reports `COMPLETED_WITH_ANOMALY`.

**Diagnosis:** Unexpected paths exist outside the allowlist.

**What happens:**
- Allowlist is committed (audit trail preserved)
- `proceed_to_github_ops: false`
- Flow completes UNVERIFIED

**Fix:**
1. Check `anomaly_paths` in Repo Operator Result
2. Review `git_status.md` for details
3. Decide:
   - Add paths to allowlist (if intentional)
   - Remove unexpected files
   - Rerun the flow

---

## "CANNOT_PROCEED"

**Symptom:** Receipt or Machine Summary shows `status: CANNOT_PROCEED`.

**Diagnosis:** Mechanical failure only.

CANNOT_PROCEED means:
- IO error (can't read/write files)
- Permission error
- Tooling failure (e.g., `jq` not installed)

It does **not** mean:
- Quality issues (that's UNVERIFIED)
- Missing artifacts (that's UNVERIFIED with `missing_required`)

**Fix:** Fix the environment/tooling, then rerun.

---

## "Can't find run by issue number"

**Symptom:** Flow can't locate an existing run.

**Diagnosis:** Check alias resolution.

Runs are found via:
1. `.runs/index.json` (`run_id`, `issue_number`, `canonical_key`)
2. `run_meta.json.aliases[]`

**NOT** by folder name (folders don't rename).

**Fix:**
- Check `.runs/index.json` for the run entry
- Check `canonical_key` matches expected format (`gh-<issue_number>`)
- Use explicit run-id if alias resolution fails

---

## "Command not found"

**Symptom:** `/flow-1-signal` doesn't work.

**Diagnosis:** Pack not discovered.

Ensure:
- `.claude/` is at repo root
- You're in a Claude Code session
- `.claude/commands/` contains flow files

---

## "Receipt doesn't match Git state"

**Symptom:** Receipt counts don't reflect actual files.

**Diagnosis:** Possible reseal issue or commit timing.

Receipts are sealed **after** reseal converges. If:
- Modified files weren't restaged after sanitization
- Commit happened before reseal completed

**Fix:** Rerun the flow. The reseal pattern should handle this.

---

## Getting help

If you've tried the above and are still stuck:

1. Run `bash .claude/scripts/pack-check.sh` (must pass)
2. Gather:
   - `<flow>_receipt.json`
   - `secrets_status.json`
   - `git_status.md` (if present)
   - Any Gate Result / Repo Operator Result blocks
3. Open an issue with the above

See [SUPPORT.md](../../SUPPORT.md) for issue guidelines.
