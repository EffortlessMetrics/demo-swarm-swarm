# demoswarm-runs-tools

Deterministic helpers for `.runs/` operations in DemoSwarm packs.

## Installation

```bash
# Install to repo-local bin
cargo install --path tools/demoswarm-runs-tools --root .demoswarm

# Or build directly
cargo build --release -p demoswarm-runs-tools
```

## Usage

### Subcommand invocation

```bash
demoswarm count pattern --file "..." --regex "..."
demoswarm ms get --file "..." --section "## Machine Summary" --key status
demoswarm yaml get --file "..." --key deployment_verdict
demoswarm index upsert-status --index ".runs/index.json" --run-id "..." --status "..." --last-flow "..."
demoswarm time now
```

### Multicall invocation

When the binary is installed/copied as `runs_*` names, it routes automatically:

```bash
runs_count_pattern --file "..." --regex "..."
runs_extract_machine_field --file "..." --section "..." --key "..."
runs_iso_now
```

## Contract

All commands follow the scalar stdout contract:

- **stdout**: Single scalar (`null`, integer, or string)
- **exit code**: Always 0 (errors expressed via `null` stdout)
- **stderr**: Optional diagnostics

## Commands

| Command | Python equivalent |
|---------|-------------------|
| `count pattern` | `runs_count_pattern` |
| `count bdd` | `runs_count_bdd_scenarios` |
| `ms get` | `runs_extract_machine_field` |
| `yaml get` | `runs_extract_yaml_block_field` |
| `yaml count-items` | `runs_count_yaml_block_items` |
| `inv get` | `runs_extract_inventory_marker` |
| `line get` | `runs_extract_line_value` |
| `receipts count` | `runs_count_existing_receipts` |
| `receipt get` | `runs_read_receipt_field` |
| `openapi count-paths` | `runs_count_openapi_paths` |
| `index upsert-status` | `runs_index_upsert_status` |
| `time now` | `runs_iso_now` |
