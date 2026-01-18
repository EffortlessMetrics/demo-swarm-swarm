# Why Reseal

> Receipt correctness and the reseal pattern.

---

## The problem

Receipts must reflect the final committed state.

Timeline without reseal:

1. Cleanup writes receipt (counts artifacts)
2. Secrets-sanitizer scans
3. Sanitizer redacts/modifies artifacts
4. Commit happens
5. **Receipt is stale** (counted pre-modification state)

---

## The reseal pattern

```
cleanup → sanitizer → modified_files: true?
  YES → cleanup → sanitizer → modified_files: true?
    YES → ... (repeat)
    NO → done
  NO → done
```

In words:

1. Cleanup computes receipt
2. Sanitizer scans publish surface
3. If sanitizer modified files, receipt is stale
4. Rerun cleanup (recompute receipt)
5. Rerun sanitizer (verify no more modifications)
6. Repeat until `modified_files: false`

---

## Why this matters

### Receipt accuracy

Receipts are the source of truth. Reporters read receipts; they don't recompute.

If a receipt counts 5 requirements but sanitizer redacted one, the count is wrong.

Reseal ensures receipts match the final tree.

### Commit consistency

The commit should contain:

- Artifacts in their final form
- Receipt that accurately describes them

Without reseal, you might commit:

- Pre-redaction artifacts (bad)
- Stale receipt (misleading)

---

## How it works in practice

### Normal case (no modifications)

```
cleanup → receipt written
sanitizer → modified_files: false
→ done (receipt is accurate)
```

### Modification case

```
cleanup → receipt written
sanitizer → redacts secret → modified_files: true
cleanup → receipt rewritten (counts post-redaction)
sanitizer → modified_files: false
→ done (receipt is accurate)
```

### Convergence

Normally converges in 1-2 cycles:

- First cleanup: original state
- Sanitizer modifies
- Second cleanup: post-modification state
- Sanitizer: nothing to modify

---

## Non-convergence (stuck loop)

If modifications keep happening:

```
cleanup → sanitizer → modified → cleanup → sanitizer → modified → ...
```

Causes:

- Sanitizer modifies something that cleanup re-generates
- Pathological pattern in artifacts

### Solution: safe-bail

```yaml
repo-operator checkpoint_mode: local_only
```

This:

- Commits locally
- Never pushes
- Forces `proceed_to_github_ops: false`
- Flow completes UNVERIFIED with evidence

Better to complete locally than loop forever.

---

## Flow 3 special case

Flow 3 has extra complexity:

- Build produces code/tests
- Sanitizer might modify code/tests
- Must restage after modification

Sequence:

```
build-cleanup → sanitizer → modified_files: true?
  YES → restage → build-cleanup → sanitizer → ...
  NO → commit
```

Without restage, the commit wouldn't include sanitizer's changes.

---

## What flows check

```yaml
## Gate Result
modified_files: true | false
```

This field drives the reseal loop.

---

## Design rationale

### Why not just recompute receipts later?

Receipts should be sealed at flow completion. Lazy recomputation:

- Adds complexity
- Creates "which version?" ambiguity
- Breaks receipts-first reporting

### Why not skip sanitizer?

Security. Every publish surface must be scanned.

### Why not modify in place and count after?

Sanitizer doesn't know receipt structure. Cleanup knows how to count.

Separation of concerns.

---

## See also

- [why-two-gates.md](why-two-gates.md) — GitHub ops gating
- [architecture.md](architecture.md) — Overall pack design
- [troubleshoot.md](../how-to/troubleshoot.md) — Debugging reseal issues
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
