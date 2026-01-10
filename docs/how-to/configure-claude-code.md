# Configure Claude Code for DemoSwarm

> Set up Claude Code to run the pack at full velocity.

---

## Prerequisites

Before configuring Claude Code:

1. **Claude Code CLI installed**
   ```bash
   # Verify installation
   claude --version
   ```

2. **Repository cloned**
   ```bash
   git clone <your-repo-url>
   cd <your-repo>
   ```

3. **Git configured**
   ```bash
   git config user.name "Your Name"
   git config user.email "your@email.com"
   ```

---

## The Key Setting: Permissions

### What It Does

Claude Code has a permissions system that controls what operations Claude can perform without asking for approval. By default, Claude asks permission for file reads, writes, and command execution.

For DemoSwarm, we configure Claude to bypass these prompts. Claude can:
- Read and write files freely
- Run commands without approval
- Execute multi-step operations autonomously

### Why We Use It

**The sandbox is the boundary, not the prompt.**

We don't constrain Claude's moment-to-moment actions. Instead:
- The repository is the sandbox
- Gates engage at publish boundaries (commit, push, GitHub post)
- Claude has freedom to explore, implement, and iterate within that sandbox

This is exactly how you'd treat a trusted new hire: "Here's your dev environment. Experiment freely. When you're ready to ship, we'll review."

### The Alternative (and Why It Fails)

Approval-per-action systems:
- Interrupt flow constantly
- Reduce Claude to a suggestion engine
- Make iteration expensive
- Treat capability as threat

We'd rather have Claude try 10 things and find what works than ask permission for each step.

---

## The Sandbox Model

### What Makes This Safe

Claude operates in an isolated development environment:

1. **Repo clone isolation** — Claude works on a clone, not production
2. **No production credentials** — The sandbox has no access to real systems
3. **Git is the only path out** — Push is the boundary to the outside world
4. **Gates engage at publish points** — Secrets scanning, repo hygiene checks before anything leaves

### What You Need to Ensure

For the sandbox model to work, you must:

- **No `.env` files with real secrets** — Use placeholder values or `.env.example`
- **No production API keys** — Development/test credentials only
- **Isolated environment** — No production database access, no live service credentials
- **Human reviews PRs** — The merge decision is human-owned

If your dev environment has production access, you don't have a sandbox — you have theater.

---

## Recommended Configuration

### Repo-Level Settings

Create or edit `.claude/settings.json`:

```json
{
  "permissions": {
    "defaultMode": "bypassPermissions",
    "allow": [
      "Bash",
      "Read",
      "Write",
      "Edit",
      "MultiEdit",
      "WebFetch",
      "WebSearch"
    ],
    "ask": [
      "Bash(rm -rf *)",
      "Bash(git push *)",
      "Bash(git reset --hard *)",
      "Bash(git clean -fd*)",
      "Bash(npm publish*)",
      "Bash(docker login*)"
    ],
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(~/.ssh/**)",
      "Read(~/.aws/**)",
      "Write(./.env)",
      "Write(./secrets/**)"
    ]
  }
}
```

This configuration:
- **Allows** normal development operations without prompts
- **Asks** before destructive or publishing operations (push, publish, reset)
- **Denies** access to sensitive credential files

### Global Settings (Alternative)

If you want this configuration across all repos, edit `~/.claude/settings.json`:

```json
{
  "permissions": {
    "defaultMode": "bypassPermissions",
    "allow": [
      "Bash",
      "Read",
      "Write",
      "Edit",
      "MultiEdit",
      "WebFetch",
      "WebSearch"
    ]
  }
}
```

Repo-level settings override global settings, so you can have permissive global defaults with tighter controls on specific repos.

### Minimal Configuration

If you just want to get started quickly:

```json
{
  "permissions": {
    "defaultMode": "bypassPermissions",
    "allow": [
      "Bash",
      "Read",
      "Write",
      "Edit"
    ]
  }
}
```

This enables the core operations. Add more as needed.

---

## Model Selection

### Default Behavior

Claude Code uses Sonnet by default — a good balance of cost and capability for most tasks.

### For Complex Features

When working on architecturally significant features, you can use Opus for deeper reasoning. The pack doesn't control this directly; it's a Claude Code setting.

### Agent-Level Allocation

The pack handles model allocation internally:
- **Cleanup agents** — Lighter models for mechanical tasks (counting, extraction)
- **Creative agents** — Inherit from session default for implementation and critique

You don't need to configure this; the pack manages it.

---

## Verifying Your Setup

### Check Claude Code Installation

```bash
claude --version
```

### Validate Pack Structure

```bash
bash .claude/scripts/pack-check.sh
```

All checks should pass. If not, review errors and fix before proceeding.

### Test a Flow

```bash
# In Claude Code, run:
/flow-1-signal "test feature"
```

If this creates artifacts under `.runs/test-feature/signal/`, your setup is working.

### Verify Permissions

In Claude Code, try:
- Reading a file (should work without prompt)
- Writing a file (should work without prompt)
- Running a test command (should work without prompt)

If you're getting permission prompts, check your `settings.json` configuration.

---

## Common Issues

| Issue | Cause | Fix |
|-------|-------|-----|
| Permission prompts appearing | `permissions.allow` not configured | Add operations to `allow` list in `settings.json` |
| Flows failing to spawn agents | Task operations not allowed | Add `Task` to allow list if using subtask patterns |
| Git operations failing | No git config | Run `git config user.name` and `git config user.email` |
| `gh` commands failing | GitHub CLI not authenticated | Run `gh auth login` |
| Pack commands not found | Not in Claude Code session | Open Claude Code in the repo directory |
| Settings not taking effect | Wrong settings file location | Check `.claude/settings.json` is at repo root |

---

## Security Considerations

### This Configuration Is for Development Sandboxes

The permissive configuration is designed for isolated development environments where:
- No production credentials exist
- No production systems are accessible
- Git push is the only way to affect the outside world
- Human review happens before merge

### Do NOT Use On

- Repos with production secrets in `.env` or config files
- Machines with production database access
- Environments with live API credentials
- Shared development servers with production access

### Your CI/CD Has Its Own Gates

This configuration applies to local development. Your CI/CD pipeline should have:
- Its own secrets scanning
- Its own test requirements
- Its own approval workflows

The pack's gates (secrets-sanitizer, repo-operator) are development-time safety nets, not replacements for CI/CD security.

### Human Review Required

The pack produces evidence for human review. The PR description is the primary interface. Humans own the merge decision.

If reviewers rubber-stamp without reading, the feedback loop breaks. Review is a decision point, not a formality.

---

## If You Don't Want Bypass Permissions

### Alternative: Approval-Per-Operation

Remove the permissions configuration entirely. Claude will ask for approval on each operation.

```json
{
  // No permissions block = default behavior (ask for everything)
}
```

### What Changes

- Claude asks before reading files
- Claude asks before writing files
- Claude asks before running commands
- Much slower iteration

### When This Makes Sense

- You're evaluating the pack and want to see what it does
- You're working in a sensitive environment
- You prefer explicit approval over sandbox trust

### The Pack Still Works

All flows function correctly with approval-per-operation. You'll just be clicking "approve" frequently. The tradeoff is velocity for visibility.

---

## Environment Variables

The pack uses environment variables for some configuration:

```json
{
  "env": {
    "BASH_DEFAULT_TIMEOUT_MS": "1800000",
    "BASH_MAX_TIMEOUT_MS": "7200000"
  }
}
```

These control command timeouts. The defaults are reasonable for most projects. Adjust if you have long-running test suites or builds.

---

## See Also

- [Customize Pack](customize-pack.md) — Configure test/lint/policy commands
- [Troubleshoot](troubleshoot.md) — Debug common failures
- [Operational Philosophy](../explanation/operational-philosophy.md) — Why we run this way
- [The Gate Pattern](../explanation/principles/gate-pattern.md) — Engineering is default-allow, publishing is gated
