# PR Review Interface

> The PR description is the cockpit display. Receipts are the flight recorder. The diff is spot-check/audit.

---

## Philosophy

Most reviewers will only read the GitHub PR description. If the key information isn't there, they won't find it in `.runs/`.

**Three surfaces:**
1. **PR Brief (description)** — What changed, why, where to look, what was proven
2. **Receipts** — Machine-readable truth with evidence pointers
3. **Diff** — Audit surface for spot-checking hotspots

The PR Brief is the **primary human interface**. Everything else is drill-down.

---

## PR Brief Template

Every flow that produces a PR (Build/Review/Gate) must generate a PR Brief that gets posted to the PR description.

```markdown
## PR Brief

### What changed
- <1–3 bullets: user-visible or contract-visible changes>

### Why
<1 short paragraph: goal + constraints + approach>

### Review map (hotspots)
- `path/to/core_change.rs` — <why it matters>
- `path/to/contract_or_schema.*` — <interface surface>
- `path/to/tests/*` — <verification surface>

### Quality events
- **Interface lock:** <no API/schema drift | breaking + version bump>
- **Boundaries / coupling:** <new module boundary | deps unchanged>
- **Verification depth:** <mutation-on-diff pass | fuzz added | edge-case tests>
- **Security airbag:** <secrets/vulns/unsafe drift: none | details>

### Proof (measured vs not measured)
- Gate: PASS|BOUNCE (evidence: `.runs/.../gate_receipt.json`)
- Tests: <summary> (evidence: `.runs/.../test_execution.md`)
- Mutation/fuzz: <ran | not run + reason>
- **Not measured:** <explicit list — unknown is normal>

### Reproduce
```bash
<one command>  # e.g., just gate / ./scripts/gate.sh
```
```

---

## Hard Rules

1. **No "CI green" without a pointer.** If you claim CI passed, link to evidence.

2. **Unknown is normal.** "Not measured" is better than invented precision. If mutation testing didn't run, say so.

3. **Quality events, not quality claims.** Report what happened (interface locked, boundaries verified, tests added), not abstract assertions.

4. **Hotspots guide review.** The diff might be 500 lines, but only 3 files matter. Point reviewers there.

5. **Receipts are truth.** The PR Brief summarizes; receipts prove. When in doubt, point to the receipt.

---

## Where This Gets Generated

| Flow | Agent | Output |
|------|-------|--------|
| Build | `build-cleanup` | `.runs/<run-id>/build/pr_brief.md` |
| Review | `review-cleanup` | `.runs/<run-id>/review/pr_brief.md` |
| Gate | `gate-cleanup` | `.runs/<run-id>/gate/pr_brief.md` |

The `pr-creator` and `pr-commenter` agents read the latest `pr_brief.md` and post it to the PR description.

---

## Updating the PR Description

The system updates the PR description deterministically:
- Replace only inside stable markers (`<!-- PR_BRIEF_START -->` / `<!-- PR_BRIEF_END -->`)
- Preserve any original human text outside those markers

This allows humans to add context while the swarm updates the evidence section.

---

## Review Time Guidance (Optional)

If you include review time estimates, follow these rules:

- **Basis:** hotspots + proof depth (not line count)
- **Confidence:** low | medium | high
- **Interpretation:** "machine cycles were spent to buy proof, so review stays short"

Example:
```
Review time: 20–40m (estimated; basis: 3 hotspots + mutation pass; confidence: medium)
```

Do NOT invent precision. If you don't have data, don't estimate.

---

## See Also

- [CLAUDE.md](../../CLAUDE.md) — PR Review Interface non-negotiable
- [contracts.md](contracts.md) — Receipt schemas
- [stable-markers.md](stable-markers.md) — Marker prefixes for PR Brief sections
