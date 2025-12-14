# Why Two Gates

> Why GitHub operations need both secrets and repo hygiene gates.

---

## The problem

Before posting to GitHub or pushing code:
- Is the content safe to publish? (secrets)
- Is the repo state correct? (hygiene)

These are **independent concerns**. Both must pass.

---

## Two gates defined

### Gate 1: Secrets gate

**Source:** `secrets-sanitizer` Gate Result

**Field:** `safe_to_publish: true | false`

**What it checks:**
- Publish surface scanned for secrets
- Sensitive patterns detected/redacted
- No hardcoded credentials

**Failure mode:**
- Secrets detected
- Unable to safely redact
- Upstream fix required

### Gate 2: Repo hygiene gate

**Source:** `repo-operator` Repo Operator Result

**Field:** `proceed_to_github_ops: true | false`

**What it checks:**
- Only expected paths in commit
- No anomalous files outside allowlist
- Staging/commit operations succeeded

**Failure mode:**
- Unexpected files detected
- Git operation failed
- Anomaly in working tree

---

## Why both are needed

### Scenario A: Secrets clean, hygiene bad

- No secrets in artifacts ✓
- Unexpected file in staging ✗

Risk: Push unexpected content to GitHub.

**Result:** Skip GH ops.

### Scenario B: Hygiene clean, secrets bad

- Only expected files staged ✓
- Secret detected in artifact ✗

Risk: Expose sensitive data.

**Result:** Skip GH ops.

### Scenario C: Both pass

- Content is safe ✓
- Repo state is correct ✓

**Result:** Proceed with GH ops.

---

## How flows use the gates

```yaml
# After cleanup and sanitizer...

IF secrets Gate Result safe_to_publish: false
  THEN skip GH ops
  THEN complete flow UNVERIFIED with evidence

ELSE IF repo-operator proceed_to_github_ops: false
  THEN skip GH ops
  THEN complete flow UNVERIFIED with evidence

ELSE
  CALL gh-issue-manager
  CALL gh-reporter
```

Both gates are checked before **any** GitHub operation:
- Issue creation
- Status board updates
- Comment posting
- Pushing
- Merge (Flow 5)

---

## Why not one combined gate?

### Separation of concerns

Secrets scanning and repo hygiene are different skills:
- secrets-sanitizer knows about secret patterns
- repo-operator knows about Git state

Combining them would blur responsibilities.

### Independent failures

Each gate can fail independently:
- Secrets issue → fix content
- Hygiene issue → fix staging/allowlist

Clear separation makes debugging easier.

### Different remediation

Secrets issues may require:
- Redaction
- Upstream code change
- Environment variable externalization

Hygiene issues may require:
- Removing unexpected files
- Adjusting allowlist
- Fixing Git state

---

## Safe-bail pattern

When gates fail but flow should complete:

```yaml
repo-operator checkpoint_mode: local_only
```

This:
- Commits locally
- Never pushes
- Forces `proceed_to_github_ops: false`
- Flow completes UNVERIFIED with evidence

Use when:
- Reseal doesn't converge
- Anomaly is intentional but shouldn't push
- Local-only operation is preferred

---

## See also

- [why-reseal.md](why-reseal.md) — Receipt correctness
- [architecture.md](architecture.md) — Overall pack design
- [contracts.md](../reference/contracts.md) — Gate Result schema
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
