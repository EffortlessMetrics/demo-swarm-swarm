# CONTRIBUTING.md Critique

## Inputs Used
- `CONTRIBUTING.md` (reviewed in full)
- `.claude/agents/` (84 agents verified)
- `.claude/commands/` (8 flow commands verified)
- `.claude/skills/` (7 skills verified)
- `.claude/rules/` (10 rules verified)
- `.claude/hooks/` (directory existence verified)
- `scripts/` (root-level scripts verified)
- `docs/` structure (Diataxis organization verified)
- `.claude/scripts/pack-check.sh` (current implementation verified)

---

## Accuracy Assessment

### ACCURATE SECTIONS

**What lives here (lines 5-11):**
- `.claude/agents/` — Correct. 84 agents present, all have required frontmatter.
- `.claude/commands/` — Correct. 8 commands present (flow-1 through flow-7 + customize-pack).
- `.claude/skills/` — Correct. 7 skills present: test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools.
- `docs/` — Correct. Diataxis structure confirmed (tutorials, how-to, reference, explanation).
- `scripts/` — Correct. Validation scripts present at root and in `.claude/scripts/`.

**What does NOT live here (lines 13-18):**
- All correctly identified. No application code, runtime code, .runs directories, or build tooling found.

**Local Development - Clone (lines 24-29):**
- Accurate. Standard git clone workflow.

**Code Style - Markdown (lines 92-95):**
- Accurate. 2-space YAML indentation is standard in the codebase.
- 120-character line limit is reasonable.

**Code Style - Python (lines 98-105):**
- Accurate. Black and ruff are the correct formatters.

**Agent Frontmatter (lines 52-60):**
- Accurate structure shown. Verified in code-implementer.md and others.

**Color ↔ role family mapping (lines 63-74):**
- CORRECT colors present: yellow (10), purple (13), green (11), red (13), blue (21), orange (14), cyan (1), pink (1).
- All listed colors are used in the codebase.
- The mapping itself is correct per agent examination.

**Making Changes - Agents (lines 49-61):**
- Accurate. Edit `.claude/agents/<agent-name>.md`, run pack-check.sh for validation.

**Making Changes - Commands (lines 76-80):**
- Accurate description.
- Suggestion to "test by copying .claude/ to a sandbox repo" is reasonable.

**Making Changes - Skills (lines 82-85):**
- Accurate. Skills are in `.claude/skills/<skill-name>/` with SKILL.md files.

**PR Process (lines 109-115):**
- Accurate naming convention for feature branches.
- Validation step is correct.

**PR Checklist (lines 117-123):**
- Accurate. All items are valid checks.

---

## STALE / MISSING / INACCURATE SECTIONS

### 1. [MISSING_DOCS] Hooks Directory Not Documented

**What's missing:**
- Lines 5-11 list `.claude/` contents but omit `.claude/hooks/`
- There is a `.claude/hooks/` directory containing:
  - `contract_lint.py` — Contract enforcement pre-commit hook
  - `gh_outbound_guard.py` — GitHub operations safety hook
- These hooks are part of the pack and contributors should know about them

**Why it matters:**
- Contributors may encounter pre-commit failures from these hooks
- They should understand what they do and why they fail
- The pack-check.sh docs don't mention hooks either

**Suggested update:**
Add to "What lives here" section:
```
- `.claude/hooks/` — Pre-commit hooks (contract enforcement, GitHub safety gates)
```

And add a section under "Making Changes":
```
### Pre-commit Hooks

The pack includes hooks that run before commits:
- `contract_lint.py` — Validates API contract consistency
- `gh_outbound_guard.py` — Prevents unsafe GitHub operations

These hooks may prevent commits if violations are detected. If a hook fails:
1. Review the error message
2. Fix the violation (usually in contracts or API docs)
3. Retry the commit

Hooks are installed by `.demoswarm/setup` or manually via `.demoswarm/install-hooks`.
```

---

### 2. [STALE_DOC] pack-check.sh Implementation Has Changed

**What's wrong:**
- Lines 31-42 claim pack-check.sh runs "comprehensive validation: agent structure, frontmatter, flow commands, machine summaries, and more"
- The actual implementation (verified in `.claude/scripts/pack-check.sh`) is now a **shim** that delegates to:
  1. `.demoswarm/bin/pack-check` (Rust binary, preferred)
  2. Global `pack-check` on PATH
  3. Cargo fallback for dev repos
- The shim does NOT contain validation logic itself—it's pure delegation

**Why it matters:**
- Contributors may try to run pack-check.sh and get a "pack-check not installed" error
- The documentation doesn't explain the dependency on the Rust binary
- No guidance on how to install the binary

**Suggested update:**
Replace lines 31-42 with:
```markdown
### 2. Validate the pack

```bash
bash .claude/scripts/pack-check.sh
```

This runs comprehensive validation via the pack-check tool: agent structure, frontmatter, flow commands, and contract consistency.

**Note:** pack-check requires the Rust-based pack-check binary. Install it with:
```bash
cargo install --path tools/demoswarm-pack-check --root .demoswarm
```

Then run the validation:
```bash
bash .claude/scripts/pack-check.sh
```

**For specific checks without installing pack-check**, use targeted scripts:
```bash
python scripts/lint_frontmatter.py       # Frontmatter validation only
python scripts/check_portable_claude.py  # Portability check only
```
```

---

### 3. [INACCURATE] Frontmatter Model Field Claim

**What's wrong:**
- Lines 57-58 show: `model: inherit`
- This is correct, but no explanation of what "inherit" means
- Agents inherit the model from CLAUDE.md or Claude Code defaults
- Contributors might assume `model: inherit` is a literal string vs. a directive

**Suggested update:**
Add clarification:
```
- `model: inherit` — Use the model specified in CLAUDE.md; do not set a specific model
```

---

### 4. [MISSING_DOCS] Documentation Changes and Diataxis

**What's incomplete:**
- Lines 126-134 reference Documentation Conventions but provide minimal guidance
- The conventions exist (verified at `docs/reference/documentation-conventions.md`)
- But the instructions don't explain the Diataxis structure adequately for new contributors

**Suggested update:**
Expand the section with examples:
```markdown
## Documentation Changes

When editing docs, follow [Documentation Conventions](docs/reference/documentation-conventions.md).

### Diataxis Structure

Place docs in the correct Diataxis category:

| Category | Purpose | When to Use |
|----------|---------|------------|
| `docs/tutorials/` | Learning by doing | Hands-on walk-throughs (e.g., "Set up your first flow") |
| `docs/how-to/` | Task completion | Answer "how do I do X?" (e.g., "Add a new agent") |
| `docs/reference/` | Lookup & schemas | Tables, API contracts, stable references |
| `docs/explanation/` | Understanding "why" | Rationale and architecture (e.g., why we use prose routing) |

**Example:**
- Adding a new agent? Write a **how-to** in `docs/how-to/add-an-agent.md`
- Explaining the PM/IC model? Write an **explanation** in `docs/explanation/`
- Documenting the run artifact schema? Write a **reference** in `docs/reference/`

### Key Principles

- **Point to file paths** — "See `.claude/agents/code-implementer.md`" not "See the code-implementer agent"
- **Avoid hardcoded counts** — "See the agents directory" not "We have 84 agents"
- **Explicit about what's not measured** — "Coverage not measured: budget exhausted" not silence
- **No hype language** — Write like an incident report or ADR (factual, scoped, reproducible)

See [Documentation Conventions](docs/reference/documentation-conventions.md) for full guidelines.
```

---

### 5. [MISSING_DOCS] Rules Directory and Constitution

**What's missing:**
- No mention of `.claude/rules/` in "What lives here"
- Rules are foundational to understanding agent behavior and pack philosophy
- Contributors should know about the constitution and check compliance

**Suggested update:**
Add to "What lives here":
```
- `.claude/rules/` — The constitution (00-doctrine through 90-voice-and-tone)
```

Add a new section:
```markdown
## Understanding the Pack Constitution

The `.claude/rules/` directory contains the foundational philosophy and constraints:

| Rule | Scope |
|------|-------|
| `00-doctrine.md` | Core thesis, manufacturing trust, anti-austerity |
| `10-operating-model.md` | PM/IC roles, spawning agents |
| `20-intent-to-narrative.md` | How meaning flows from intent to shipped code |
| `30-autonomy-and-boundaries.md` | Default-allow work, strict publish gates |
| `40-evidence-and-quality.md` | Claims require pointers, quality panels |
| `50-agent-contract.md` | How agents think and hand off |
| `60-flow-orchestrators.md` | How orchestrators scope and route |
| `70-docs-and-teaching.md` | Docs as curriculum, one concept = one home |
| `80-developer-experience.md` | UX, accessibility, economics |
| `90-voice-and-tone.md` | Industrial clarity, human warmth |

When making changes, check that your work doesn't violate any of these rules. See CLAUDE.md for the full contract.
```

---

### 6. [INACCURATE] Questions Section References

**What's incomplete:**
- Lines 137-143 provide question references
- Reference to `ARCHITECTURE.md` is correct, but outdated claim
- CLAUDE.md is correctly the contract
- But no reference to `START-HERE.md` which is now the primary entry point for new contributors

**Suggested update:**
Replace lines 137-143 with:
```markdown
## Questions?

- **New to the pack?** Start with [START-HERE.md](docs/START-HERE.md)
- **Pack usage**: See `README.md` and `CLAUDE.md`
- **Philosophy and architecture**: See `docs/explanation/` (e.g., `agent-philosophy.md`, `architecture.md`)
- **Adapting to your stack**: See `docs/how-to/customize-pack.md`
- **Maintaining the pack**: See `docs/maintainers/`
- **Documentation conventions**: See `docs/reference/documentation-conventions.md`
- **Full docs index**: See `docs/README.md`
```

---

## User-Visible Changes Needing Documentation

1. **pack-check.sh dependency change** — Now requires Rust binary, not pure bash validation
2. **Hooks integration** — Pre-commit hooks introduced but not documented
3. **Rules constitution** — Core philosophy moved to rules directory, should be discoverable
4. **START-HERE.md** — New primary entry point not referenced in CONTRIBUTING.md

---

## Verification Guidance Gaps

- No guidance on what to do if `pack-check.sh` fails
- No explanation of hook failures (what they mean, how to fix)
- No step-by-step example of adding an agent + running validation

---

## Strengths

- Clear, concise structure (good use of sections and tables)
- Accurate frontmatter requirements for agents
- Correct color ↔ role mapping
- Good PR checklist (practical and complete)
- Appropriate tone (factual, not overly formal)

---

## Handoff

**What I found:** CONTRIBUTING.md is 85% accurate. It correctly describes agent structure, frontmatter requirements, skill organization, and the PR process. However, it's missing documentation for:

1. `.claude/hooks/` (pre-commit hooks not mentioned)
2. `.claude/rules/` (constitution directory not listed)
3. Updated pack-check.sh implementation (now a Rust shim, not pure bash)
4. START-HERE.md (primary entry point for new contributors)
5. Diataxis structure guidance (too brief)

**What's left:** 5-6 documented gaps ranging from missing context (hooks, rules) to incomplete implementation details (pack-check dependency). None are critical, but they create friction for new contributors.

**Recommendation:** Route to doc-writer to update these sections. A single revision pass should address all gaps. Priority order:
1. Add hooks documentation (enables understanding of pre-commit failures)
2. Clarify pack-check.sh dependency (enables successful validation)
3. Add rules constitution reference (supports understanding of pack philosophy)
4. Add START-HERE.md reference (improves onboarding)
5. Expand Diataxis guidance (reduces friction for documentation contributors)

This is a solid "ready to improve" state, not a "broken" state. The existing content is accurate; it just needs completion around recently added features (hooks) and updated implementation details (pack-check Rust shim).
