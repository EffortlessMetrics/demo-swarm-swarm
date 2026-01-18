# Run State

> Run identity, state schemas, and artifact organization.

---

## Directory Structure

All paths are **repo-root-relative**. Run artifacts live under:

```
.runs/
├── index.json                      # Global run index
└── <run-id>/
    ├── run_meta.json               # Per-run metadata
    ├── signal/                     # Flow 1 artifacts
    │   ├── requirements.md
    │   ├── features/*.feature
    │   ├── open_questions.md
    │   └── signal_receipt.json
    ├── plan/                       # Flow 2 artifacts
    │   ├── adr.md
    │   ├── api_contracts.yaml
    │   ├── work_plan.md
    │   └── plan_receipt.json
    ├── build/                      # Flow 3 artifacts
    │   ├── ac_status.json
    │   ├── test_execution.md
    │   └── build_receipt.json
    ├── review/                     # Flow 4 artifacts
    │   ├── review_worklist.md
    │   └── review_receipt.json
    ├── gate/                       # Flow 5 artifacts
    │   ├── merge_decision.md
    │   └── gate_receipt.json
    ├── deploy/                     # Flow 6 artifacts
    │   ├── deployment_decision.md
    │   └── deploy_receipt.json
    └── wisdom/                     # Flow 7 artifacts
        ├── learnings.md
        └── wisdom_receipt.json
```

---

## `.runs/index.json`

Global run index. One entry per `run_id`.

```json
{
  "version": 1,
  "runs": [
    {
      "run_id": "feat-auth",
      "canonical_key": "gh-456",
      "task_key": "feat-auth",
      "task_title": "Add OAuth2 login",
      "issue_number": 456,
      "pr_number": null,
      "updated_at": "2025-12-11T22:15:00Z",
      "status": "VERIFIED",
      "last_flow": "build"
    }
  ]
}
```

### Index Rules

- One entry per `run_id` (upsert, not append)
- Preserve existing ordering; upsert updates in-place. New runs append.
- Keep fields minimal — counts live in receipts
- Only these agents may update `.runs/index.json`:
  - `run-prep`, `signal-run-prep`
  - `<flow>-cleanup`
  - `gh-issue-manager`

---

## `run_meta.json`

Per-run metadata at `.runs/<run-id>/run_meta.json`.

```json
{
  "run_id": "<run-id>",
  "run_id_kind": "GH_ISSUE | LOCAL_ONLY | null",
  "issue_binding": "IMMEDIATE | DEFERRED | null",
  "issue_binding_deferred_reason": "gh_unauth | gh_unavailable | null",
  "canonical_key": "<gh-456 | pr-789 | null>",
  "aliases": ["<run-id>", "<gh-456>", "<branch-name>"],
  "task_key": "<ticket-id | branch-slug | null>",
  "task_title": "<short normalized title>",
  "github_repo": "<owner/repo | null>",
  "github_repo_expected": "<owner/repo | null>",
  "github_repo_actual_at_creation": "<owner/repo | null>",
  "github_ops_allowed": true,
  "repo_mismatch": false,
  "created_at": "<ISO8601>",
  "updated_at": "<ISO8601>",
  "iterations": 1,
  "flows_started": ["signal", "plan"],
  "source": "<branch:name | ticket:id | manual>",
  "issue_number": 456,
  "issue_url": "<url | null>",
  "issue_title": "<string | null>",
  "pr_number": null,
  "supersedes": "<previous-run-id | null>",
  "related_runs": [],
  "base_ref": "<branch-name | null>"
}
```

### Identity Rules

- `run_id` is immutable. **No renames.**
- When a GitHub issue/PR exists, set `canonical_key` and add aliases. Do not rename folders.

### Stacked Run Support

- `base_ref` (optional): The branch this run is based on. Used for diff computation in agents that audit changes (standards-enforcer, coverage-enforcer, etc.). If present, diffs are computed relative to `base_ref`; otherwise agents default to `origin/main`.

---

## Alias Resolution

Runs are found via:

1. `.runs/index.json` (`run_id`, `issue_number`, `canonical_key`)
2. `run_meta.json.aliases[]`

**NOT** by folder name (folders don't rename).

Example: Finding run for issue #456:

- Check `index.json` for `issue_number: 456`
- Or check `canonical_key: "gh-456"`
- Or scan `run_meta.json.aliases[]` for `"gh-456"`

---

## Receipts

Each flow produces a receipt with mechanical counts and quality gate verdicts.

| Flow   | Receipt File                                |
| ------ | ------------------------------------------- |
| Signal | `.runs/<run-id>/signal/signal_receipt.json` |
| Plan   | `.runs/<run-id>/plan/plan_receipt.json`     |
| Build  | `.runs/<run-id>/build/build_receipt.json`   |
| Review | `.runs/<run-id>/review/review_receipt.json` |
| Gate   | `.runs/<run-id>/gate/gate_receipt.json`     |
| Deploy | `.runs/<run-id>/deploy/deploy_receipt.json` |
| Wisdom | `.runs/<run-id>/wisdom/wisdom_receipt.json` |

### Receipt Guarantees

- `counts` are mechanical (grep/wc/parse), never estimated
- `quality_gates` are sourced from agent Machine Summaries (no recomputation)
- Reporters summarize from receipts, not from raw artifacts
- `evidence_sha` is the commit SHA when receipt was generated
- `generated_at` is the ISO8601 timestamp for receipt creation

### Receipts as Logs, Not Gatekeepers

Receipts are historical evidence, not permission slips.

- If `receipt.evidence_sha != git HEAD`, that's normal (ad-hoc fixes expected)
- The receipt is still valid as historical evidence
- Don't BOUNCE or require regeneration just because the SHA drifted
- The git log is the audit trail. The receipt is a summary.

See: [contracts.md](contracts.md) for per-flow receipt schemas.

---

## Publish Surface (Per-Flow)

What secrets-sanitizer scans and repo-operator checkpoints:

| Flow | Publish Surface                                                       |
| ---- | --------------------------------------------------------------------- |
| 1    | `.runs/<run-id>/signal/`, `run_meta.json`, `index.json`               |
| 2    | `.runs/<run-id>/plan/`, `run_meta.json`, `index.json`                 |
| 3    | `.runs/<run-id>/build/`, `run_meta.json`, `index.json`, + code/tests  |
| 4    | `.runs/<run-id>/review/`, `run_meta.json`, `index.json`, + code/tests |
| 5    | `.runs/<run-id>/gate/`, `run_meta.json`, `index.json`                 |
| 6    | `.runs/<run-id>/deploy/`, `run_meta.json`, `index.json`               |
| 7    | `.runs/<run-id>/wisdom/`, `run_meta.json`, `index.json`               |

**Invariant:** secrets-sanitizer scans only the current flow's publish surface, not the entire `.runs/<run-id>/`.

---

## Swarm Repo Model

Recommended: run flows in a dedicated `*-swarm` downstream repo.

```
my-project/           # Human workspace (stays calm)
my-project-swarm/     # Swarm workspace (commits freely)
```

Benefits:

- **Inspectability**: `.runs/` artifacts are committed and reviewable
- **Isolation**: swarm activity doesn't disrupt human development
- **Clean PRs**: open PR from swarm to origin when ready

### Repo Topology (Invariant)

- **Swarm repo (`*-swarm`) is autonomous.** Flows run here end-to-end.
- **Flow 6 (Deploy) merges a run PR into `*-swarm/main`** (the swarm's mainline).
- This pack does **not** merge into the upstream human repo by default. (Upstream export is a separate concern.)

### `.runs/` is Git Content

`.runs/` is committed by default — **do not gitignore it**.

Size discipline:

- Summaries over raw dumps
- No pasting full issue bodies into artifacts
- Keep artifacts "reviewable diff" sized

---

## DevLT Tracking

Receipts may include a `devlt` section for retrospective analysis:

```json
{
  "devlt": {
    "flow_started_at": "2025-12-22T10:00:00Z",
    "flow_completed_at": "2025-12-22T10:45:00Z",
    "human_checkpoints": [
      { "at": "2025-12-22T10:00:00Z", "action": "flow_start" },
      { "at": "2025-12-22T10:30:00Z", "action": "question_answered" },
      { "at": "2025-12-22T10:45:00Z", "action": "flow_approved" }
    ],
    "machine_duration_sec": 2700,
    "human_checkpoint_count": 3,
    "estimated_human_attention_min": 15,
    "estimation_basis": "checkpoint_count * 5min average"
  }
}
```

**Observable vs inferred:**

- Timestamps and counts are **facts** (derived from logs/artifacts)
- `estimated_human_attention_min` is an **inference** (labeled as such)

**Purpose:** DevLT is for retrospective analysis in Flow 7 (Wisdom), not for gating or routing.

---

## See Also

- [contracts.md](contracts.md) — Control-plane blocks, receipt schemas
- [evidence-freshness.md](evidence-freshness.md) — When evidence is stale and when to re-verify
- [stable-markers.md](stable-markers.md) — Marker patterns for counting
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
