# Documentation Updates for issue-35-status-semantics

## Handoff

**What I did:** Added new "Status Semantics" section to CLAUDE.md and standardized status terminology across all 7 cleanup agents to consistently use VERIFIED/UNVERIFIED/PARTIAL/CANNOT_PROCEED with clear guidance on when to use each.

**What's left:** Nothing. All cleanup agents have been updated with consistent status semantics.

**Recommendation:** Route to doc-critic to verify the documentation changes are accurate and complete, then route to repo-operator to commit the changes.

## Inputs Used

- `CLAUDE.md` (existing content)
- `.claude/agents/build-cleanup.md`
- `.claude/agents/review-cleanup.md`
- `.claude/agents/deploy-cleanup.md`
- `.claude/agents/plan-cleanup.md`
- `.claude/agents/wisdom-cleanup.md`
- `.claude/agents/gate-cleanup.md`
- `.claude/agents/signal-cleanup.md`
- `.claude/rules/00-doctrine.md`
- `.claude/rules/40-evidence-and-quality.md`

## Files Updated

| File | Change Type | Summary |
| ---- | ----------- | ------- |
| `CLAUDE.md` | updated | Added new "Status Semantics" section after Architecture Principles with table defining VERIFIED/UNVERIFIED/PARTIAL/CANNOT_PROCEED and guidance on when to use each |
| `.claude/agents/review-cleanup.md` | updated | Standardized status section to use code-formatted statuses, added explicit note that PARTIAL is unique to Flow 4, updated receipt schema to include status field, updated handoff examples |
| `.claude/agents/plan-cleanup.md` | updated | Changed "Assessing completion" to "Status determination" with code-formatted statuses, added note that Plan does not use PARTIAL |
| `.claude/agents/deploy-cleanup.md` | updated | Changed "Completion states" to "Status determination" with code-formatted statuses, added status field to receipt schema, added note that Deploy does not use PARTIAL |
| `.claude/agents/wisdom-cleanup.md` | updated | Changed "Completion states" to "Status determination" with code-formatted statuses, added status field to receipt schema, updated "Partial completion" references to "UNVERIFIED" |
| `.claude/agents/build-cleanup.md` | no change | Already uses correct VERIFIED/UNVERIFIED/CANNOT_PROCEED terminology |
| `.claude/agents/gate-cleanup.md` | no change | Already uses correct VERIFIED/UNVERIFIED/CANNOT_PROCEED terminology |
| `.claude/agents/signal-cleanup.md` | no change | Already uses correct VERIFIED/UNVERIFIED/CANNOT_PROCEED terminology |

## Deferred / Not Updated (and why)

- None. All cleanup agents have been reviewed and updated as needed.

## Mismatches Found (if any)

- None. The existing doctrine in `00-doctrine.md` and `40-evidence-and-quality.md` already defined three statuses (VERIFIED/UNVERIFIED/CANNOT_PROCEED). The only addition needed was documenting PARTIAL for unbounded loops, which was already implicitly used in review-cleanup.

## Assumptions Made

- **PARTIAL is exclusively for Flow 4:** Based on the issue description and review-cleanup.md's explicit note that "Flow 4 has unbounded loops", I documented that PARTIAL should only be used for flows with unbounded iteration loops. Currently only Flow 4 (Review) qualifies.

## Key Changes Summary

### CLAUDE.md - New Status Semantics Section

```markdown
## Status Semantics

Cleanup agents report completion using these statuses:

| Status           | Meaning                                                                        | When to Use                                      |
| ---------------- | ------------------------------------------------------------------------------ | ------------------------------------------------ |
| `VERIFIED`       | Converged. Evidence panel green, evidence fresh, blockers empty.               | Work complete, all checks passed                 |
| `UNVERIFIED`     | Not converged, but checkpointed. Artifacts written, state captured, resumable. | Missing verification, contradictions, blockers   |
| `PARTIAL`        | Progress made in unbounded loop. More iterations needed.                       | Flow 4 worklists, iterative refinement only      |
| `CANNOT_PROCEED` | Mechanical failure. Tooling broken, permissions missing, infra down.           | Environment issues preventing any work           |

**Key distinctions:**

- **UNVERIFIED vs PARTIAL:** Use `PARTIAL` only for flows with unbounded iteration loops (like Flow 4 review worklist processing). For all other incomplete work (missing artifacts, failed checks, contradictions), use `UNVERIFIED`.

- **UNVERIFIED is not failure.** It's honest state that enables routing. A receipt with `UNVERIFIED` and documented gaps is more valuable than blocking.

- **PARTIAL is a checkpoint.** When Flow 4 exhausts context mid-worklist, `PARTIAL` means "real progress made, more to do, rerun to continue."
```

### Cleanup Agent Updates

Each cleanup agent now has:
1. A "Status determination" section with code-formatted status values
2. Explicit note about whether PARTIAL applies to that flow
3. Receipt schema with explicit status field
4. Handoff examples using proper status terminology
