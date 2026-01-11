# Routing Table

> When an agent hits a problem, where does work go next?

---

## The Core Principle

**"Blocked" almost never means "stop." It means "route to the agent that handles this."**

Agents do not escalate to humans mid-flow. They:
1. Try to resolve locally
2. If they cannot, route to another agent
3. Document the situation
4. Questions get queued (surfaced to GH/docs) but do not stop the flow

True halts are rare. Most "blocks" are just routing decisions.

---

## The Routing Discipline

When an agent encounters something outside its scope:

| Do | Do Not |
|----|--------|
| Finish what you can | Stop at the first obstacle |
| Route to the right specialist | Guess outside your domain |
| Queue questions, continue with assumptions | Escalate mid-flow |
| Document what happened and what you assumed | Leave gaps unexplained |

---

## Common Routing Patterns

| When This Happens | Route To | Why |
|-------------------|----------|-----|
| Lint/format failures | `auto-linter` (skill) | Mechanical fix |
| Test failures | `test-author` or `fixer` | Test needs update or code needs fix |
| Type errors | `fixer` | Mechanical fix |
| Missing dependency | `code-implementer` | Implementation gap |
| Unclear requirement | `clarifier` (queues question) | Needs human input eventually |
| Design ambiguity | `design-optioneer` | Generate options |
| Security concern | `risk-analyst` | Assess and document |
| Git operation needed | `repo-operator` | Sole owner of git |
| Merge conflict | `merge-reconciler` | Resolve conflicts, then back to repo-operator |
| Critique finding | `fixer` | Address the issue |
| Contract mismatch | `interface-designer` or `fixer` | Design or implementation gap |
| Secrets detected | `secrets-sanitizer` | Must clean before publish |
| Missing test coverage | `test-author` | Write the tests |
| Documentation gap | `doc-writer` | Update docs |
| BDD scenario issue | `bdd-author` | Revise scenarios |
| Requirements unclear | `requirements-author` or `clarifier` | Revise or clarify |

---

## Routing by Agent Family

### Critics (Red)

Critics **never fix**. They find issues and report them.

| Agent | Routes findings to | Routes structural issues to |
|-------|-------------------|----------------------------|
| `code-critic` | `fixer` | `code-implementer` (if redesign needed) |
| `test-critic` | `test-author` | `test-strategist` (if test plan wrong) |
| `requirements-critic` | `requirements-author` | `problem-framer` (if scope wrong) |
| `bdd-critic` | `bdd-author` | `requirements-author` (if reqs wrong) |
| `design-critic` | `design-optioneer` or `adr-author` | Back to Flow 2 start |
| `contract-critic` | `interface-designer` | Back to Flow 2 |
| `doc-critic` | `doc-writer` | `code-implementer` (if behavior wrong) |

### Authors (Purple/Green)

Authors **create artifacts** in their domain.

| Agent | Routes implementation needs to | Routes ambiguity to |
|-------|-------------------------------|---------------------|
| `requirements-author` | `bdd-author` (next step) | `clarifier` |
| `bdd-author` | `test-author` (Flow 3) | `clarifier` |
| `test-author` | `code-implementer` (if code needs changing) | `clarifier` |
| `design-optioneer` | `adr-author` (next step) | `clarifier` |
| `interface-designer` | `code-implementer` (Flow 3) | `design-optioneer` |

### Implementers (Green)

Implementers **write and modify code**.

| Agent | Routes test needs to | Routes git operations to | Routes critiques to |
|-------|---------------------|-------------------------|---------------------|
| `code-implementer` | `test-author` | `repo-operator` | Self (fix) or `fixer` |
| `fixer` | `test-author` | `repo-operator` | Self |
| `test-author` | Self | `repo-operator` | Self |
| `doc-writer` | N/A | `repo-operator` | Self |

### Operators (Cyan/Green)

Operators **handle external system interactions**.

| Agent | Routes content decisions to | Routes failures to |
|-------|----------------------------|-------------------|
| `repo-operator` | Requesting agent | `fixer` (code issues) or human (at boundary) |
| `gh-issue-manager` | Requesting agent | `clarifier` (if issue unclear) |
| `gh-reporter` | Requesting agent | N/A (report-only) |

### Cleanup Agents (Blue)

Cleanup agents **summarize and verify state**.

| Agent | Routes incomplete work to | Routes to next flow if |
|-------|--------------------------|------------------------|
| `signal-cleanup` | Owning agent in Flow 1 | All artifacts exist |
| `plan-cleanup` | Owning agent in Flow 2 | All artifacts exist |
| `build-cleanup` | Owning agent in Flow 3 | Tests pass, artifacts exist |
| `review-cleanup` | `fixer` | Worklist addressed |
| `gate-cleanup` | Depends on verdict | MERGE or BOUNCE decided |
| `deploy-cleanup` | `deploy-monitor` | Deployment verified |
| `wisdom-cleanup` | N/A | Learnings extracted |

### Gate Agents (Blue/Red)

Gate agents **make decisions based on evidence**.

| Agent | If pass | If fail |
|-------|---------|---------|
| `merge-decider` | Proceed to Gate cleanup | BOUNCE with reason |
| `deploy-decider` | Proceed to deploy | Block deployment |
| `secrets-sanitizer` | Allow publish | Block publish, attempt fix |

---

## Question Routing (The Escalation Path)

Questions do not stop flows. They get queued.

### The Flow

```
1. Agent encounters uncertainty
          |
          v
2. Agent tries to resolve:
   - Search code, check docs, look at precedent
          |
          v
3. If unresolved, agent chooses:
   - Safe default (if exists) -> document assumption -> continue
   - No safe default -> mark NEEDS_HUMAN -> queue question
          |
          v
4. Question recorded in open_questions.md:
   - What was tried
   - What was assumed (if defaulted)
   - Risk if wrong
   - Who should answer
          |
          v
5. Flow continues with assumption
          |
          v
6. At flow boundary, cleanup summarizes open questions
          |
          v
7. Human reviews questions with flow output
          |
          v
8. If answer changes things:
   Route back to relevant agent in next iteration
```

### open_questions.md Entry Format

```markdown
## OQ-BUILD-007: Session timeout default

- **Question:** What should the session timeout be?
- **Tried:** Searched codebase, found no existing timeout config
- **Defaulted to:** 30 minutes (industry standard)
- **Risk if wrong:** User sessions may expire too soon or too late
- **Who should answer:** Product owner or security team
```

---

## True Halts (Rare)

A flow should only halt when:

| Condition | Why it halts |
|-----------|--------------|
| Secrets detected and cannot be cleaned | Cannot publish with exposed credentials |
| External system completely unavailable | Cannot proceed (e.g., GitHub down) |
| Circular dependency between agents | Needs human arbitration |
| Safety/compliance hard stop | Policy requires human sign-off |

Even these often resolve to "route to X" rather than "stop everything."

---

## Routing Syntax in Handoffs

When an agent hands off with a routing recommendation:

```markdown
## Handoff

**What I did:** Implemented the session timeout feature.

**What's left:**
- Tests are failing on edge case (1 failure in test_session_edge_cases.py)
- Unclear whether timeout should be hard or soft (defaulted to hard, queued question)

**Recommendation:** Route to fixer for the test failure. The timeout question
is documented in open_questions.md as OQ-BUILD-007; continue with hard timeout
assumption for now.
```

The orchestrator reads this and routes accordingly. No parsing required.

---

## Routing Decision Tree

```
Agent hits obstacle
       |
       v
Can I resolve it myself?
       |
   +---+---+
   |       |
  Yes      No
   |       |
   v       v
Do it   Is there an agent for this?
           |
       +---+---+
       |       |
      Yes      No
       |       |
       v       v
   Route    Is it safe to default?
   to it        |
            +---+---+
            |       |
           Yes      No
            |       |
            v       v
        Default   Queue question
        + doc     + continue
```

### Text Version

1. **Can I resolve it myself?**
   - Yes: Do it.
   - No: Continue to step 2.

2. **Is there an agent for this?**
   - Yes: Route to that agent.
   - No: Continue to step 3.

3. **Is it safe to default?**
   - Yes: Default, document it, continue.
   - No: Queue question, continue with NEEDS_HUMAN flag.

---

## Agent Lookup by Problem Type

Quick reference for common problems:

| Problem Type | First Choice | Second Choice |
|--------------|--------------|---------------|
| **Code quality** | `fixer` | `code-implementer` |
| **Test failures** | `fixer` | `test-author` |
| **Missing tests** | `test-author` | - |
| **Type errors** | `fixer` | - |
| **Lint errors** | `auto-linter` | `fixer` |
| **Format issues** | `auto-linter` | - |
| **Import errors** | `fixer` | `code-implementer` |
| **Missing docs** | `doc-writer` | - |
| **Stale comments** | `fixer` | `doc-writer` |
| **Git conflicts** | `repo-operator` | - |
| **Branch issues** | `repo-operator` | - |
| **Secrets exposed** | `secrets-sanitizer` | - |
| **Design unclear** | `design-optioneer` | `clarifier` |
| **Requirements unclear** | `clarifier` | `requirements-author` |
| **Contract violation** | `interface-designer` | `code-implementer` |
| **Coverage gap** | `test-author` | - |
| **Security issue** | `risk-analyst` | `security-scanner` |
| **Flaky test** | `flakiness-detector` | `test-author` |
| **Mutation survivors** | `mutation-auditor` | `test-author` |

---

## Anti-Patterns

### Do Not

- **Stop mid-flow to ask human a question** - Queue it and continue
- **Guess without documenting the guess** - Assumptions must be recorded
- **Route in circles (A to B to A)** - Detect loops and break them
- **Route to an agent that does not exist** - Check the agents index
- **Treat every uncertainty as a blocker** - Most have safe defaults

### Do

- **Finish what you can before routing** - Partial progress is valuable
- **Document what you tried** - Next agent needs context
- **Make reversible defaults when safe** - Continue with assumptions
- **Queue questions for boundary review** - Humans see them at checkpoints
- **Trust the next agent to handle their domain** - Single responsibility

---

## Flow Boundary Behavior

At flow boundaries (signal-cleanup, plan-cleanup, etc.):

1. **Collect open questions** from the flow
2. **Summarize** what was completed vs what is pending
3. **Write receipt** with honest status
4. **Report** to orchestrator with routing recommendation

The orchestrator then decides:
- Continue to next flow
- Rerun current flow
- Wait for human input (at boundary, not mid-flow)

---

## Routing Cheat Sheet

```
Code broken?           -> fixer
Tests broken?          -> fixer or test-author
Tests missing?         -> test-author
Design wrong?          -> design-optioneer
Requirements wrong?    -> requirements-author
Git needed?            -> repo-operator
Secret found?          -> secrets-sanitizer
Question unanswered?   -> clarifier (queues it)
Docs stale?            -> doc-writer
Contract violated?     -> interface-designer or fixer
```

---

## See Also

- [agents-index.md](agents-index.md) - Master listing of all agents
- [contracts.md](contracts.md) - Handoff patterns and control-plane blocks
- [agent-philosophy.md](../explanation/agent-philosophy.md) - How agents think and act
- [human-escalation.md](../explanation/human-escalation.md) - When humans get involved
- [CLAUDE.md](../../CLAUDE.md) - Pack reference
