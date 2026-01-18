# Release Checklist

> Ship a new pack version.

---

## Version Strategy

Pack version, repo tags, and Rust tool versions are aligned:

| Component     | Location                                | Example             |
| ------------- | --------------------------------------- | ------------------- |
| Pack release  | Git tag                                 | `v1.0.0`            |
| pack-check    | `tools/demoswarm-pack-check/Cargo.toml` | `version = "1.0.0"` |
| demoswarm CLI | `tools/demoswarm-runs-tools/Cargo.toml` | `version = "1.0.0"` |
| CHANGELOG     | `CHANGELOG.md`                          | `## [1.0.0]`        |

All four must match for a release. The `[Unreleased]` section in CHANGELOG.md tracks changes between releases.

---

## Before release

### 1. Validation

```bash
# Must pass
bash .claude/scripts/pack-check.sh
```

### 2. Toy run

Run the validation suite:

- Toy Run A (Flows 1→4)
- Toy Run B (out-of-order)

---

## Release Process

### 1. Bump Versions

1. Update `version` in `tools/demoswarm-pack-check/Cargo.toml`
2. Update `version` in `tools/demoswarm-runs-tools/Cargo.toml`
3. Update `CHANGELOG.md` (move Unreleased to Version)

### 2. Commit & Tag

```bash
git commit -am "chore: prepare release vX.Y.Z"
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin main vX.Y.Z
```

### 3. Create GitHub Release

Use the tag to create a release with release notes.

### 4. Post-Release

Bump to next dev version (optional but recommended for Cargo).
