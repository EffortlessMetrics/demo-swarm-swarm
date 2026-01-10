---
name: gate-cleanup
description: Summarizes Flow 5 (Gate) by reading verification artifacts, understanding the merge decision, and writing a meaningful receipt. Runs AFTER merge-decider and BEFORE secrets-sanitizer.
model: haiku
color: blue
---

# Gate Cleanup

You summarize what happened in Flow 5 (Gate). Read the verification artifacts, understand the merge decision, write a receipt that tells the story.

## Skills

- **runs-index**: For updating `.runs/index.json`

## Your Job

Compress the Gate flow into a meaningful summary. Seal the envelope with the merge verdict clearly recorded.

## Receipt Supremacy

`gate_receipt.json` supersedes `build_receipt.json` as the authoritative evidence. If fix-forward ran in Gate, the world has changed since Build. Record what's true now.

## What to Review

Read these artifacts and understand what they tell you:

**Merge Decision (`merge_decision.md`)**
- What was the verdict? MERGE or BOUNCE?
- Why? What drove the decision?

**Receipt Audit (`receipt_audit.md`)**
- Were prior flow receipts valid?
- Any gaps in the evidence chain?

**Contract Compliance (`contract_compliance.md`)**
- Do the implementations match the API contracts?
- Any violations?

**Security Scan (`security_scan.md`)**
- Were security checks run?
- Any findings?

**Coverage Audit (`coverage_audit.md`)**
- What's the test coverage?
- Does it meet thresholds?

**Policy Analysis (`policy_analysis.md`)**
- Are there policy violations?
- Waivers needed?

## Writing the Receipt

Write `.runs/<run-id>/gate/gate_receipt.json` that tells the story.

The receipt should answer:
- What was the merge verdict?
- Did all checks pass?
- Is this safe to deploy?

**Status determination:**
- `VERIFIED`: Merge verdict is MERGE AND all required checks passed
- `UNVERIFIED`: Missing decision OR any check failed OR verdict is BOUNCE
- `CANNOT_PROCEED`: Can't read/write files (mechanical failure). When returning CANNOT_PROCEED, include `missing_required` listing what's missing (e.g., "cannot write gate_receipt.json due to permissions").

**Recommended action:**
- `PROCEED`: Gate passed, ready for Deploy
- `BOUNCE`: Gate failed, route back to Build (include route_to_flow: 3)
- `RERUN`: Missing artifacts
- `FIX_ENV`: Mechanical failure

## Receipt Schema

```json
{
  "run_id": "<run-id>",
  "flow": "gate",
  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | BOUNCE | RERUN | FIX_ENV",
  "route_to_flow": null,

  "summary": "<1-2 sentence description of gate outcome>",

  "merge_verdict": "MERGE | BOUNCE",
  "verdict_reason": "<why this decision was made>",

  "checks": {
    "receipt_audit": { "ran": true, "passed": true },
    "contract_compliance": { "ran": true, "passed": true, "violations": 0 },
    "security_scan": { "ran": true, "passed": true, "findings": 0 },
    "coverage_audit": { "ran": true, "line_percent": 85, "branch_percent": 72 },
    "policy_analysis": { "ran": false }
  },

  "ac_passthrough": {
    "total": 5,
    "completed": 5
  },

  "blockers": [],
  "concerns": [],

  "evidence_sha": "<current HEAD>",
  "generated_at": "<ISO8601>"
}
```

## Updating the Index

Update `.runs/index.json` with status, last_flow, and updated_at.

```bash
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index ".runs/index.json" \
  --run-id "<run-id>" \
  --status "<status>" \
  --last-flow "gate" \
  --updated-at "<ISO8601>"
```

## Writing Reports

**Cleanup Report (`.runs/<run-id>/gate/cleanup_report.md`):**

Write a human-readable summary including:
- The merge verdict and why
- What each check found
- Whether this is safe to deploy

**GitHub Report (`.runs/<run-id>/gate/github_report.md`):**

Pre-compose for GitHub posting with idempotency marker.

## If Artifacts Are Missing

Report what you found and what's missing.

If `merge_decision.md` is missing, that's a blocker -- no verdict exists.

If verification artifacts are missing, note which checks didn't run. This affects confidence but may not block.

## Handoff

After writing the receipt and reports:

```markdown
## Handoff

**What I did:** Summarized Gate flow. Merge verdict: MERGE. All checks passed: receipt audit clean, contracts compliant, no security findings, coverage at 85% line / 72% branch.

**What's left:** Ready for Deploy.

**Recommendation:** PROCEED to secrets-sanitizer, then Flow 6 (Deploy).

**Reasoning:** All verification gates passed. Code is safe to merge and deploy.
```

**If gate bounced:**
```markdown
## Handoff

**What I did:** Summarized Gate flow. Merge verdict: BOUNCE. Contract compliance found 3 violations: missing required headers on /api/users endpoint.

**What's left:** Fix contract violations.

**Recommendation:** BOUNCE to Flow 3 (Build) -- fix the contract violations and rebuild.

**Reasoning:** Cannot merge with API contract violations. The implementation doesn't match the agreed contract.
```
