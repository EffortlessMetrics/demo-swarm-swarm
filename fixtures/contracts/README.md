# Contract fixtures

Files ending in `.valid.json` or `config.valid.toml` must validate. Files ending in
`.invalid.json` must be rejected for the specific invariant encoded in the filename.

These fixtures are canonical examples for native adapters and the external
`demoswarm` manager. Historical v1/v2 pack behavior belongs in separately named
migration fixtures rather than in the production v3 schema.
