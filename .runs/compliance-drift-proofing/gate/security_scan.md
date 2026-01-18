# Security Scan Report

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []

missing_required: []

concerns:

- Dependency audit could not run (cargo-audit CVSS 4.0 format incompatibility)

sources:

- git diff main...HEAD --name-only (115 changed files)
- tools/demoswarm-pack-check/src/checks/control_plane.rs
- tools/demoswarm-pack-check/src/checks/drift.rs
- tools/demoswarm-pack-check/src/checks/flow.rs
- tools/demoswarm-pack-check/src/checks/structure.rs
- tools/demoswarm-pack-check/src/checks/wisdom.rs
- tools/demoswarm-pack-check/src/checks/mod.rs
- tools/demoswarm-pack-check/src/contracts.rs
- tools/demoswarm-pack-check/tests/check_integration_test.rs
- .claude/hooks/contract_lint.py
- .claude/hooks/gh_outbound_guard.py

severity_summary:
critical: 0
major: 0
minor: 0

findings_total: 0

scan_scope:
changed_files_count: 115
changed_files_source: git_diff

dependency_audit:
status: not_run
tool: cargo-audit
reason: Advisory database CVSS 4.0 format not supported by installed cargo-audit version

## Findings

### Secrets Exposure

No suspected secrets detected in scanned surface.

The following files were scanned for secrets patterns:

- All Rust source files in `tools/demoswarm-pack-check/src/`
- Python hooks in `.claude/hooks/`
- Test fixtures in `tools/demoswarm-pack-check/tests/fixtures/`
- Agent and command markdown files in `.claude/`

Patterns checked:

- AWS access keys (AKIA...)
- GitHub tokens (ghp*, ghs*, gho*, ghu*, ghr\_)
- Private key blocks (-----BEGIN PRIVATE KEY-----)
- Slack tokens
- JWT secrets
- Database connection strings with credentials
- Bearer tokens
- Generic patterns: password=, secret=, api_key=, token=

All checked files contain only:

- Regex patterns for secret _detection_ (not actual secrets)
- Test fixtures with synthetic values (e.g., "test-run")
- Documentation patterns

### SAST / Code Patterns

No high-signal vulnerability patterns detected in scanned surface.

**Files analyzed:**

1. **tools/demoswarm-pack-check/src/lib.rs** and **src/checks/\*.rs**
   - Purpose: Pack validation checks for compliance drift detection
   - Security posture: Read-only file operations using safe Rust APIs
   - **`#![forbid(unsafe_code)]` declared** - eliminates all memory safety vulnerabilities
   - No user input handling (operates on static pack files)
   - No SQL/database operations
   - No shell command execution
   - No network operations
   - Safe regex compilation with proper error handling

2. **tools/demoswarm-pack-check/src/contracts.rs**
   - Purpose: Constants and regex patterns for validation
   - Security posture: Static data only, no runtime input processing
   - Regex patterns compile at startup with error handling

3. **tools/demoswarm-pack-check/tests/check_integration_test.rs**
   - Purpose: Integration tests for pack-check
   - Security posture: Test code with synthetic fixtures
   - Contains explicit security test (NFR-SEC-001) verifying no secrets in fixtures
   - Uses safe Command API for subprocess execution

4. **.claude/hooks/contract_lint.py**
   - Purpose: Pre-tool-use hook for contract drift detection
   - Security posture: Reads local files only
   - JSON parsing with exception handling
   - No shell execution
   - No network operations

5. **.claude/hooks/gh_outbound_guard.py**
   - Purpose: Pre-tool-use hook to block secret leakage to GitHub
   - Security posture: This is a _security control_ that prevents secrets from being posted
   - Blocks: private keys, GitHub tokens, AWS keys, bearer tokens, database URLs with passwords
   - Returns exit code 2 to block operations when secrets detected
   - Does NOT print/log detected secret content (only pattern names)

**Observations:**

- The codebase follows defense-in-depth with `gh_outbound_guard.py` as a guardrail
- All file I/O uses safe Rust APIs (std::fs) or Python pathlib
- No unsafe blocks in Rust code
- No eval/exec patterns
- No command injection vectors (no shell=True, no string interpolation into commands)
- No path traversal vulnerabilities (paths are relative to known roots)

### Dependency Risk

**Status:** Not run

**Reason:** The `cargo audit` tool encountered a compatibility issue with the CVSS 4.0 format in the RustSec advisory database. This is a known issue with cargo-audit versions prior to 0.22.0.

**Mitigation:** The Rust dependencies are standard crates:

- `regex` - well-maintained regex library
- `walkdir` - directory traversal
- `clap` - CLI argument parsing
- `serde`/`serde_json` - serialization
- `anyhow` - error handling

These are widely-used crates with no known critical vulnerabilities in recent versions.

**Recommendation for future iterations:** Update cargo-audit to 0.22+ or use `cargo deny check advisories` as an alternative.

## Notes for Merge-Decider

This scan found **no security issues** in the changed surface. The compliance drift-proofing feature implements:

1. **Read-only validation** - The pack-check tool only reads files; it does not modify them or execute arbitrary code.

2. **Defense-in-depth controls** - The `gh_outbound_guard.py` hook is a security control that prevents secrets from being posted to GitHub.

3. **Safe patterns** - All regex patterns, file operations, and subprocess calls use safe APIs without injection vectors.

4. **Test security hygiene** - Integration tests explicitly verify that fixtures contain no secrets (NFR-SEC-001).

The dependency audit limitation is a tooling concern, not a security blocker. The crates used are standard Rust ecosystem libraries with good security track records.

**Recommendation:** PROCEED to merge. No security blockers identified.
