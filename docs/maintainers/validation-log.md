# Validation Log

> Record of pack validation runs.

---

## How to record

After running validation (see [validation-run.md](../tutorials/validation-run.md)):

```markdown
## Validation: YYYY-MM-DD

- pack-check: PASSED | FAILED
- Toy Run A (Flows 1–4): PASSED | PARTIAL | FAILED
- Toy Run B (out-of-order Plan): PASSED | FAILED
- Reseal test: PASSED | SKIPPED | FAILED
- GH idempotency: PASSED | SKIPPED | N/A

Run IDs: <comma-separated>
Commit: <sha>
Notes: <any relevant context>
```

---

## Log

<!-- Add new entries at the top -->

## Validation: 2025-12-14

- pack-check: PASSED (49/49 checks, 0 errors, 0 warnings)
- Toy Run A (Flows 1–4): PARTIAL (manual CLI/contract verification)
- Toy Run B (out-of-order Plan): PARTIAL (manual CLI/contract verification)
- Reseal test: SKIPPED
- GH idempotency: SKIPPED

Run IDs: val-a, val-b
Commit: N/A (pre-release)
Notes: Release 1.0.1 preparation. Updated documentation and versioning. Validated pack structure and CLI tooling (demoswarm, pack-check). Full interactive flow validation skipped as agent cannot trigger flows, but underlying machinery is verified.

## Validation: 2025-12-12

- pack-check: PASSED (46/46 checks, 0 errors, 0 warnings)
- Toy Run A (Flows 1–4): SKIPPED (pre-release documentation audit)
- Toy Run B (out-of-order Plan): SKIPPED (pre-release documentation audit)
- Reseal test: SKIPPED
- GH idempotency: SKIPPED

Run IDs: N/A (documentation audit run)
Commit: 7a94529
Notes: Pre-release audit fixing CLI documentation drift (openq-tools, secrets-tools SKILL.md), adding FOSS hygiene files (SECURITY.md, CODE_OF_CONDUCT.md), completing CLAUDE.md command reference.
