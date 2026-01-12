# Your First Swarm Run

> A beginner's guide to running Flow 1 and understanding what happens.

This tutorial walks you through running your first flow in DemoSwarm. By the end, you will have transformed a simple feature idea into testable requirements and BDD scenarios.

**Time required:** 10-15 minutes

**What you will learn:**
- How to start a flow
- What agents do the work
- Where artifacts are stored
- How to verify the run succeeded

---

## Prerequisites

Before starting, make sure you have:

1. **Claude Code installed** - The IDE extension that runs the swarm
2. **A repo with the pack** - Either:
   - Clone this pack directly, or
   - Copy `.claude/` from this pack to your project
3. **Basic shell access** - `bash` and `git` available in your terminal
4. **(Optional) GitHub CLI** - `gh` authenticated for GitHub integration

Not sure if you are ready? Run this quick check:

```bash
# Check git
git --version

# Check bash
bash --version

# Check gh (optional)
gh auth status
```

If `gh` is not authenticated, that is fine. The flow works fully offline; GitHub features will simply be skipped.

---

## Step 1: Open Your Repo in Claude Code

Open your repo in your IDE with Claude Code enabled. Claude Code discovers the pack automatically:
- `.claude/commands/` - Slash commands like `/flow-1-signal`
- `.claude/agents/` - Specialist agents that do the work
- `.claude/skills/` - Tools for testing, linting, etc.

You should see the commands available when you type `/` in the chat.

---

## Step 2: Run Your First Flow

In the Claude Code chat, type:

```
/flow-1-signal "Add a user login feature"
```

Press Enter. That is it. The swarm takes over from here.

**What you just did:** You gave the swarm a "signal" - a natural language description of what you want to build. Flow 1 transforms this fuzzy input into structured, testable artifacts.

---

## Step 3: Watch the Agents Work

After you run the command, you will see Claude Code spawn multiple agents. Here is what each one does:

### Infrastructure Agents (Setting Up)

| Agent | What It Does |
|-------|-------------|
| `gh-issue-resolver` | Creates or finds a GitHub issue for tracking |
| `repo-operator` | Creates a branch for this run |
| `signal-run-prep` | Sets up the `.runs/<run-id>/` directory |

### Research and Analysis Agents

| Agent | What It Does |
|-------|-------------|
| `gh-researcher` | Searches for related issues and prior work |
| `signal-normalizer` | Parses your input into structured form |
| `problem-framer` | Defines the problem clearly (goals, non-goals) |
| `clarifier` | Documents ambiguities and assumptions |

### Requirements Agents (Microloop)

| Agent | What It Does |
|-------|-------------|
| `requirements-author` | Writes testable requirements (REQ-001, etc.) |
| `requirements-critic` | Reviews and improves the requirements |

These two agents work in a loop, refining until the critic is satisfied.

### BDD Agents (Microloop)

| Agent | What It Does |
|-------|-------------|
| `bdd-author` | Creates Gherkin scenarios (Given/When/Then) |
| `bdd-critic` | Reviews scenarios for coverage and testability |

Again, these loop until the critic says the scenarios are ready.

### Assessment and Cleanup Agents

| Agent | What It Does |
|-------|-------------|
| `scope-assessor` | Estimates scope (S/M/L/XL) and identifies risks |
| `risk-analyst` | Performs deeper risk analysis |
| `spec-auditor` | Final quality check across all artifacts |
| `signal-cleanup` | Generates the receipt (summary of what was produced) |

### Publishing Agents

| Agent | What It Does |
|-------|-------------|
| `secrets-sanitizer` | Scans for accidentally committed secrets |
| `repo-operator` | Commits the artifacts |
| `gh-reporter` | Posts a summary to the GitHub issue |

---

## Step 4: Find Your Artifacts

When the flow completes, check your `.runs/` directory:

```bash
ls .runs/
```

You will see a folder with your run ID. Inside:

```
.runs/<run-id>/
├── run_meta.json              # Run metadata
└── signal/
    ├── requirements.md        # Testable requirements
    ├── features/
    │   └── *.feature          # BDD scenarios (Gherkin)
    ├── open_questions.md      # Questions and assumptions
    ├── problem_statement.md   # Problem definition
    ├── early_risks.md         # Identified risks
    ├── scope_estimate.md      # S/M/L/XL estimate
    └── signal_receipt.json    # Flow summary
```

### Key Files to Review

**`requirements.md`** - Your requirements with IDs and acceptance criteria:

```markdown
### REQ-001 User Login

Users can log into the system with email and password.

**Acceptance Criteria:**
- AC-1: User can enter email and password
- AC-2: System validates credentials
- AC-3: Successful login redirects to dashboard
```

**`features/*.feature`** - BDD scenarios in Gherkin format:

```gherkin
Feature: User Login
  As a user
  I want to log in with my credentials
  So that I can access my account

  @REQ-001
  Scenario: Successful login with valid credentials
    Given I am on the login page
    When I enter "user@example.com" as email
    And I enter "validpassword" as password
    And I click the login button
    Then I should be redirected to the dashboard
    And I should see a welcome message

  @REQ-001
  Scenario: Failed login with invalid password
    Given I am on the login page
    When I enter "user@example.com" as email
    And I enter "wrongpassword" as password
    And I click the login button
    Then I should see an error message "Invalid credentials"
    And I should remain on the login page
```

---

## Step 5: Check the Receipt

The receipt tells you if the flow succeeded:

```bash
cat .runs/*/signal/signal_receipt.json | jq .
```

Or if you do not have `jq`:

```bash
cat .runs/*/signal/signal_receipt.json
```

You will see something like:

```json
{
  "flow": "signal",
  "run_id": "gh-123",
  "status": "VERIFIED",
  "counts": {
    "requirements": 4,
    "acceptance_criteria": 12,
    "bdd_scenarios": 8,
    "risks": 3,
    "open_questions": 2
  },
  "quality_gates": {
    "requirements_reviewed": true,
    "bdd_coverage_adequate": true
  },
  "evidence_sha": "abc123...",
  "generated_at": "2025-12-15T10:30:00Z"
}
```

### What the Status Means

| Status | Meaning |
|--------|---------|
| `VERIFIED` | Flow completed successfully. All quality checks passed. |
| `UNVERIFIED` | Flow completed but with issues. Check `blockers` field. |
| `CANNOT_PROCEED` | Mechanical failure (permissions, missing tools). |

**UNVERIFIED is not failure.** It means the flow completed but the critic found issues worth noting. You can still proceed to Flow 2; the issues are documented.

---

## Understanding the Markers

You will see markers like `REQ-001`, `NFR-SECURITY-001`, and `@REQ-001` in the artifacts. These are stable identifiers for mechanical counting.

| Marker | Meaning | Example |
|--------|---------|---------|
| `REQ-###` | Functional requirement | `REQ-001` (User Login) |
| `NFR-DOMAIN-###` | Non-functional requirement | `NFR-SECURITY-001` (Password hashing) |
| `AC-#` | Acceptance criterion | `AC-1: User can enter email` |
| `@REQ-###` | BDD scenario tags | Links scenario to requirement |
| `RSK-###` | Risk identifier | `RSK-001 HIGH SECURITY` |
| `OQ-SIG-###` | Open question | `OQ-SIG-001` (unclear scope) |

These markers let cleanup agents count artifacts without parsing prose. They also create traceability: every BDD scenario should trace back to a requirement.

---

## Common Questions

### Why did it create all these files?

Each file serves a purpose:
- **`requirements.md`** - What to build (testable)
- **`features/*.feature`** - How to verify it (executable specs)
- **`open_questions.md`** - What is unclear (for humans to answer)
- **`early_risks.md`** - What could go wrong (flagged early)

This is the "Signal to Spec" transformation. You gave a vague idea; you get back structured, testable artifacts.

### How do I know if it worked?

Check three things:

1. **Receipt status:** `VERIFIED` or `UNVERIFIED` (both are success states)
2. **Artifacts exist:** `ls .runs/*/signal/` shows files
3. **Content makes sense:** Read `requirements.md` and `features/*.feature`

### What if something went wrong?

Check the receipt's `blockers` field:

```bash
cat .runs/*/signal/signal_receipt.json | jq .blockers
```

Common issues:
- **Missing artifacts** - An agent failed to write. Check `missing_required` field.
- **Critic rejected** - Requirements or BDD did not meet quality bar. This is normal; iterate.
- **GitHub unavailable** - `gh` not authenticated. Local artifacts still work.

### Can I run it again?

Yes. Flow 1 is designed to be re-run. Running `/flow-1-signal` on an existing run-id:
- Preserves the existing directory
- Agents read and refine existing artifacts
- Each run improves the output

### What do I do with open questions?

Review `.runs/<run-id>/signal/open_questions.md`. You can:
1. Answer the questions directly (edit the file)
2. Note them for later (address in Flow 2)
3. Provide more signal (re-run with more details)

---

## What to Do Next

### Option A: Continue to Flow 2 (Plan)

If you are satisfied with the requirements and scenarios:

```
/flow-2-plan
```

Flow 2 takes the Signal artifacts and produces architecture decisions (ADR), API contracts, and a work plan.

### Option B: Iterate on Flow 1

If you want to refine the requirements:

1. Edit `.runs/<run-id>/signal/requirements.md` directly
2. Re-run `/flow-1-signal` with more specific input
3. Answer open questions in `open_questions.md`

### Option C: Review First

Before proceeding, humans should check:
- Are these the right requirements?
- Do the BDD scenarios cover expected behavior?
- Are the identified risks acceptable?
- Can any open questions be answered now?

The swarm produced the artifacts. You decide if they are correct.

---

## Troubleshooting

### "Command not found" when typing /flow-1-signal

Make sure:
1. You are in Claude Code (not regular terminal)
2. `.claude/` directory exists at repo root
3. Claude Code has discovered the pack (check available commands)

### Flow hangs or takes a long time

This is normal for the first run. The flow:
- Creates directories
- Runs multiple agent loops (requirements, BDD)
- May make GitHub API calls

Wait for completion. If it exceeds 30 minutes, check for error messages.

### "GitHub operations skipped"

This happens when `gh` is not authenticated. It is not an error. Local artifacts are still created. To enable GitHub features:

```bash
gh auth login
```

### Receipt shows CANNOT_PROCEED

This is a mechanical failure (I/O, permissions, tooling broken). Check:
- Can you write to `.runs/`?
- Do you have `bash` and `git` available?
- Check `missing_required` field in receipt for specifics.

### Receipt shows UNVERIFIED

This is not an error. It means the flow completed but the critic found issues. Check:
- `blockers` field for what was flagged
- `quality_gates` to see which checks did not pass

You can still proceed; the issues are documented for human review.

---

## Next Steps

| Goal | Command/Doc |
|------|-------------|
| Continue to design | `/flow-2-plan` |
| See all 7 flows | [Walkthrough](walkthrough.md) |
| Customize for your stack | [customize-pack.md](../how-to/customize-pack.md) |
| Understand the full system | [CLAUDE.md](../../CLAUDE.md) |

---

## Summary

You just ran your first swarm flow. Here is what happened:

1. **You gave a signal** - A natural language feature request
2. **Agents did the work** - Requirements, BDD, risk analysis
3. **Artifacts were produced** - In `.runs/<run-id>/signal/`
4. **A receipt was generated** - Summarizing what was built

The swarm transformed your vague idea into structured, testable specifications. From here, Flow 2 designs the solution, Flow 3 builds it, and subsequent flows review, gate, and deploy it.

Welcome to the swarm.
