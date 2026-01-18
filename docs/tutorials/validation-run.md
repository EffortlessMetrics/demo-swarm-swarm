# Validation Run

This guide validates the DemoSwarm **pack-only** repo by running two small scenarios.

The goal is not "does my app build", but "does the pack contract hold":

- receipts are mechanical
- `.runs/index.json` updates are stable
- reseal-if-modified converges (or safe-bails correctly)
- two-gate external ops are enforced
- out-of-order flow start creates an issue with "Signal pending" banner (when GH is available)
- idempotent issue reporting (one comment per flow) when GH is available

---

## Prerequisites

Required:

1. A sandbox repo (fresh clone is ideal)
2. DemoSwarm pack installed (`.claude/` present)
3. Run `/customize-pack` if you haven't (or confirm skills are configured)
4. Shell tooling for pack-check: `bash`, `git`, `grep`, `awk`, `sed`, `wc`, `jq`

Optional (for GitHub integration validation):

- `gh` installed + authenticated (`gh auth status`)

Note: If `gh` is not available/authenticated, GH steps are expected to be **SKIPPED** and local artifacts should still be written.

---

## Preflight: Pack validation

From repo root:

```bash
bash .claude/scripts/pack-check.sh
```

This should be green before you trust validation runs.

---

## Canonical Demo Goal

All validation runs use the same demo goal:

> Add a `demoswarm version` CLI subcommand that prints JSON with tool version info.

This goal is self-referential (it's a feature for the pack's own CLI tooling), deterministic, and exercises the pack's core primitives without inventing a separate "product."

---

## Validation Run A: Normal order (Flow 1 → 2 → 3 → 4 → 5)

Use a deterministic run-id so reruns are comparable:

```text
/flow-1-signal val-a "Add a demoswarm version CLI subcommand that prints JSON with tool version info. Constraints: Must work via bash .claude/scripts/demoswarm.sh version. Output is JSON to stdout (no logs/no extra text). No network calls; no GitHub required. Include at least: demoswarm_version, pack_version (if available), git_sha (optional/null-safe)."
```

### A1) After Flow 1 completes, verify (local)

- [ ] `.runs/val-a/run_meta.json` exists and contains `run_id: val-a` (or equivalent field)
- [ ] `.runs/val-a/signal/signal_receipt.json` exists
- [ ] `.runs/val-a/signal/requirements.md` contains REQ- identifiers
- [ ] `.runs/val-a/signal/features/` contains at least one `*.feature`
- [ ] `.runs/index.json` entry for `val-a` exists with `last_flow: "signal"`

Optional (if GH available and gates passed):

- [ ] A GitHub issue exists for the run
- [ ] Issue body contains a status board
- [ ] There is a Flow 1 comment (or a recorded "posted/updated" status artifact)

---

Run Flow 2 using the same run-id:

```text
/flow-2-plan val-a
```

### A2) After Flow 2 completes, verify (local)

- [ ] `.runs/val-a/plan/plan_receipt.json` exists
- [ ] `.runs/val-a/plan/adr.md` exists
- [ ] `.runs/val-a/plan/api_contracts.yaml` exists
- [ ] `.runs/val-a/plan/observability_spec.md` exists
- [ ] `.runs/index.json` updated for `val-a` with `last_flow: "plan"`

Optional (if GH available and gates passed):

- [ ] Issue status board updated
- [ ] Flow 2 comment exists and is separate from Flow 1 comment (one-per-flow behavior)

---

Run Flow 3:

```text
/flow-3-build val-a
```

### A3) After Flow 3 completes, verify (local)

- [ ] `.runs/val-a/build/build_receipt.json` exists
- [ ] `.runs/val-a/build/self_review.md` exists
- [ ] `.runs/val-a/build/impl_changes_summary.md` exists
- [ ] `.runs/val-a/build/test_changes_summary.md` exists (or the flow documented why it is absent)
- [ ] `.runs/index.json` updated for `val-a` with `last_flow: "build"`
- [ ] If reseal occurred: `.runs/val-a/build/cleanup_report.md` shows a reseal cycle occurred (evidence, not vibes)

**Feature-specific verification:**

- [ ] `demoswarm version` works via the shim: `bash .claude/scripts/demoswarm.sh version`
- [ ] Output is valid JSON with expected keys (`demoswarm_version`, `pack_version`, `git_sha`)

Optional (if GH available and gates passed):

- [ ] Flow 3 comment exists/updated

---

Run Flow 4 (Review):

```text
/flow-4-review val-a
```

### A4) After Flow 4 completes, verify (local)

- [ ] `.runs/val-a/review/review_receipt.json` exists
- [ ] `.runs/val-a/review/review_worklist.md` exists (or documented as empty)
- [ ] `.runs/index.json` updated for `val-a` with `last_flow: "review"`

---

Run Flow 5 (Gate):

```text
/flow-5-gate val-a
```

### A5) After Flow 5 completes, verify (local)

- [ ] `.runs/val-a/gate/gate_receipt.json` exists
- [ ] `.runs/val-a/gate/merge_decision.md` exists and contains MERGE/BOUNCE (with reason)
- [ ] `.runs/val-a/gate/contract_compliance.md` exists
- [ ] `.runs/val-a/gate/coverage_audit.md` exists
- [ ] `.runs/index.json` updated for `val-a` with `last_flow: "gate"`

Optional (if GH available and gates passed):

- [ ] Flow 5 comment exists/updated with verdict

---

## Validation Run B: Out-of-order start (Flow 2 without Flow 1)

Use a separate deterministic run-id:

```text
/flow-2-plan val-b
```

### B1) After Flow 2 completes, verify (local)

- [ ] `.runs/val-b/run_meta.json` exists
- [ ] `.runs/val-b/plan/plan_receipt.json` exists
- [ ] Plan artifacts exist with explicit assumptions (because Signal inputs are missing)
- [ ] `.runs/index.json` has an entry for `val-b`

Optional (if GH available and gates passed):

- [ ] GitHub issue exists
- [ ] Issue body includes a **Signal pending** banner/note
- [ ] The issue does not imply Signal was auto-run; it should instruct humans to run `/flow-1-signal` to backfill

---

Optional backfill:

```text
/flow-1-signal val-b "Backfill: Add demoswarm version CLI subcommand"
```

Verify:

- [ ] `.runs/val-b/signal/` now exists with Signal artifacts
- [ ] Issue is updated accordingly (banner removed/updated, Flow 1 comment posted/updated)

---

## Reseal behavior test (optional, local-only)

This validates the reseal loop without needing GH.

1. Add a mock "secret-like" string to a run-local artifact (avoid real-looking tokens):

```bash
printf '\nDEMO_SECRET=REDACT_ME\n' >> ".runs/val-a/signal/test_secret.md"
```

1. Run the reseal sequence by rerunning Flow 1 (simplest way to exercise the flow's own reseal logic):

```text
/flow-1-signal val-a "Reseal test run"
```

Verify:

- [ ] `.runs/val-a/signal/secrets_status.json` indicates `modified_files: true` occurred at least once
- [ ] The secret-like string is redacted in the artifact
- [ ] `signal_receipt.json` reflects the final post-redaction state (resealed)

Note: If reseal fails to converge, safe-bail must force `repo-operator checkpoint_mode: local_only` and prevent GH ops.

---

## GitHub comment idempotency (optional; requires GH)

Goal: one comment per flow, updated on rerun (not duplicated).

1. Run Flow 1 for `val-a` (already done)
2. Rerun Flow 1 again:

```text
/flow-1-signal val-a "Idempotency rerun"
```

Verify:

- [ ] The Flow 1 comment is updated (same "slot"), not duplicated
- [ ] The flow directory contains a local record artifact of the post (e.g., `github_report.md`) and a posting status artifact (e.g., `gh_report_status.md`)
- [ ] If the implementation uses a stored comment ID, it should be stable across reruns (do not rely on a specific filename unless explicitly contracted)

---

## Expected Output (Shape)

### Receipts

Every `<flow>_receipt.json` should include at least:

```json
{
  "run_id": "<run-id>",
  "flow": "<flow-name>",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | RERUN | BOUNCE | FIX_ENV",
  "route_to_flow": null,
  "route_to_agent": null,
  "missing_required": [],
  "counts": {},
  "completed_at": "<ISO8601>"
}
```

**Note on routing fields:** The `recommended_action`, `route_to_flow`, and `route_to_agent` fields exist for **audit trail purposes**. Cleanup agents derive these values from the agent's prose handoff. Orchestrators route based on reading handoffs directly, not by parsing these fields.

### Index entry

`.runs/index.json` should reflect:

```json
{
  "run_id": "<run-id>",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "last_flow": "<most-recent-flow>",
  "updated_at": "<ISO8601>"
}
```

---

## Troubleshooting

### "Receipt has null counts"

Expected when:

- upstream artifacts don't exist (out-of-order start), or
- stable markers aren't present / are ambiguous

Check the flow's `cleanup_report.md` for derivation details.

### "No issue created / no comment posted"

Check:

- `gh auth status` (if you expected GH integration)
- the secrets gate (`safe_to_publish: false` blocks external ops)
- repo-operator gate (`proceed_to_github_ops: false` blocks external ops)
- flow-local GH status artifacts for the skip reason

### "Reseal didn't happen"

Check:

- `secrets_status.json` for `modified_files: true`
- that the flow command includes reseal logic (cleanup ↔ sanitizer)

---

## Recording results (maintainer hygiene)

After successful validation:

1. Keep `.runs/val-a/` and `.runs/val-b/` (optional, but useful)
2. Record a short log in `docs/maintainers/validation-log.md`:

```md
## Validation: <date>

- pack-check: PASSED
- Validation Run A (Flows 1–5): PASSED
- Validation Run B (out-of-order Plan): PASSED
- Reseal test: PASSED (optional)
- GH idempotency: PASSED (optional)

Run IDs: val-a, val-b
Commit: <sha>
```
