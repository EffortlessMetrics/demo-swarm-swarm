# Contributing to demo-swarm

Thanks for contributing to the demo-swarm pack. This is a **pack-only repository**—it contains agents, commands, and skills that get copied into target repos.

## What lives here

- `.claude/agents/` — Subagent definitions
- `.claude/commands/` — Slash command orchestrators (7 flows + customize)
- `.claude/skills/` — Skills (test-runner, auto-linter, policy-runner, runs-derive, runs-index, openq-tools, secrets-tools)
- `docs/` — Documentation (Diataxis structure)
- `scripts/` — Validation scripts

## What does NOT live here

- Application code (`src/`, `tests/`, etc.)
- Runtime/harness/UI code
- `.runs/` directories (created in target repos)
- Makefile or build tooling

---

## Local Development

### 1. Clone the repo

```bash
git clone <repo-url>
cd demo-swarm
```

### 2. Validate the pack

```bash
bash .claude/scripts/pack-check.sh
```

This runs comprehensive validation: agent structure, frontmatter, flow commands, machine summaries, and more.

**Additional targeted scripts** (for specific checks):
```bash
python scripts/lint_frontmatter.py       # Frontmatter validation only
python scripts/check_portable_claude.py  # Portability check only
```

---

## Making Changes

### Adding or editing agents

1. Edit files in `.claude/agents/<agent-name>.md`
2. Ensure frontmatter includes required fields:
   ```yaml
   ---
   name: agent-name
   description: What the agent does
   color: <color matching role family>
   model: inherit
   ---
   ```
3. Run `bash .claude/scripts/pack-check.sh` to validate

### Color ↔ role family mapping

| Color | Role Family |
|-------|-------------|
| yellow | shaping |
| purple | spec/design |
| green | implementation |
| red | critic |
| blue | verification |
| orange | analytics |
| pink | reporter |
| cyan | infra |

### Adding or editing commands

1. Edit files in `.claude/commands/<command-name>.md`
2. Commands are slash commands that orchestrate agent workflows
3. Test by copying `.claude/` to a sandbox repo and running the command

### Adding or editing skills

1. Edit `SKILL.md` files in `.claude/skills/<skill-name>/`
2. Skills are invoked by agents; they're tools, not agents

---

## Code Style

### Markdown (agents, commands, docs)

- Use GitHub-flavored markdown
- Use 2-space indentation for YAML
- Keep lines under 120 characters where practical

### Python (scripts)

```bash
# Format
black scripts/

# Lint
ruff check scripts/
```

---

## Pull Request Process

1. Create a feature branch: `git checkout -b feat/your-change`
2. Make your changes
3. Run validation: `bash .claude/scripts/pack-check.sh`
4. Commit with a clear message: `git commit -m "feat: add foo-agent"`
5. Push and open a PR

### PR Checklist

- [ ] Agent frontmatter is valid (run lint script)
- [ ] Colors match role families
- [ ] Documentation updated if needed
- [ ] No hardcoded paths (use `.runs/<run-id>/<flow>/` pattern)

---

## Documentation Changes

When editing docs, follow [Documentation Conventions](docs/reference/documentation-conventions.md):

- Use correct Diataxis categories (tutorials, how-tos, reference, explanation)
- Point to file paths, not abstractions
- Use stable terminology (swarm mainline, upstream, deploy)
- Avoid hardcoded counts and absolute claims

---

## Questions?

- **Pack usage**: See `README.md` and `CLAUDE.md`
- **Adapting to your stack**: See `docs/how-to/customize-pack.md`
- **Architecture**: See `ARCHITECTURE.md` or `docs/explanation/architecture.md`
- **Documentation conventions**: See `docs/reference/documentation-conventions.md`
- **Full docs index**: See `docs/README.md`
