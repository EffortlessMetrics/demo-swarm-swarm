# demoswarm-pack-check

A cross-platform replacement for `.claude/scripts/pack-check.sh`.

It validates a DemoSwarm `.claude/` pack for structural and contract consistency (required agents/commands, canonical Machine Summary axis, gate contracts, drift guards, etc.).

## Install

From the repo root:

```bash
cargo install --path tools/demoswarm-pack-check
```

(or run it without installing)

```bash
cargo run --manifest-path tools/demoswarm-pack-check/Cargo.toml -- --repo-root .
```

## Usage

```bash
pack-check
pack-check --repo-root .
pack-check --no-color
pack-check --format json
```

## Exit codes

- `0`: no errors (warnings may still be present)
- `1`: one or more errors
- `2`: tool invocation / IO error (unexpected)

## Notes

- The tool discovers the repo root by walking upward from the current directory until it finds `.claude/`.
- Output is intentionally close to the original shell script to make diffs and operator muscle-memory easy.
