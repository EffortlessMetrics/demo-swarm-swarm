# Flow Composition

> Seven flows that compose into a complete SDLC.

## The Seven Flows

| Flow | Name   | Purpose                                 |
| ---- | ------ | --------------------------------------- |
| 1    | Signal | Shape the problem -> requirements + BDD |
| 2    | Plan   | Design the solution -> ADR + work plan  |
| 3    | Build  | Implement the solution -> working code  |
| 4    | Review | Harvest feedback -> resolve worklist    |
| 5    | Gate   | Verify readiness -> merge decision      |
| 6    | Deploy | Ship to production -> verification      |
| 7    | Wisdom | Learn from the run -> insights          |

## Why Seven Flows

### Separation of Concerns

Each flow has a distinct purpose:

- Signal: WHAT problem are we solving?
- Plan: HOW will we solve it?
- Build: Make it work
- Review: Make it right
- Gate: Verify it's ready
- Deploy: Ship it
- Wisdom: Learn from it

### Natural Checkpoints

Flows create natural pause points:

- End of Signal: Do we understand the problem?
- End of Plan: Do we have a good design?
- End of Build: Does the code work?
- End of Review: Is feedback addressed?
- End of Gate: Should we ship?

### Resumability

Each flow produces artifacts. If interrupted:

- Resume where you left off
- Don't lose progress
- State is on disk

## How Flows Chain

### The Happy Path

```
Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom
```

Each flow:

1. Reads artifacts from previous flows
2. Does its work
3. Produces artifacts for next flow
4. Reports completion

### Bouncing Between Flows

Sometimes you need to go back:

```
Build discovers design flaw
    |
    v
Bounce to Plan (local resolution attempted first)
    |
    v
Plan fixes design
    |
    v
Return to Build
```

Bounces should be rare. Most issues resolve locally.

### Skipping Flows

For small changes, you can skip:

- Bug fix? Signal -> Build -> Gate (skip Plan)
- Docs only? Signal -> Build -> Review (skip Plan, Gate)
- Hotfix? Build -> Gate -> Deploy (skip Signal, Plan, Review)

The system handles this:

- Missing upstream artifacts noted as assumptions
- Outcomes marked UNVERIFIED where appropriate
- Proceed anyway

## Flow Dependencies

### What Each Flow Needs

| Flow   | Required From              | Optional From               |
| ------ | -------------------------- | --------------------------- |
| Plan   | Signal: requirements.md    | Signal: BDD features        |
| Build  | Plan: adr.md, work_plan.md | Plan: api_contracts.yaml    |
| Review | Build: PR exists           | Gate: partial results       |
| Gate   | Build: build_receipt.json  | Review: review_receipt.json |
| Deploy | Gate: MERGE verdict        | -                           |
| Wisdom | Any: run artifacts         | All: complete run           |

### Graceful Degradation

Missing upstream? The flow:

1. Notes what's missing
2. Documents assumptions
3. Proceeds best-effort
4. Marks outcome UNVERIFIED

You can always run with incomplete upstream. You just get less verification.

## Out-of-Order Execution

### Why It's Allowed

Real development isn't always linear:

- Prototyping before full requirements
- Fixing bugs without design docs
- Deploying hotfixes quickly

### How It Works

Start any flow. The system:

1. Checks for expected upstream artifacts
2. Notes what's missing
3. Proceeds with documented assumptions
4. Produces whatever artifacts it can

### The Trade-off

Out-of-order = less verification:

- No requirements? Can't verify implementation matches intent
- No design? Can't verify architecture is sound
- No review? Might miss feedback

You're trading thoroughness for speed. Sometimes that's right.

## Flow Artifacts

### Signal Produces

- `requirements.md` - What we're building
- `features/*.feature` - BDD scenarios
- `signal_receipt.json` - Flow summary

### Plan Produces

- `adr.md` - Architecture decision
- `api_contracts.yaml` - Interface definitions
- `work_plan.md` - Implementation breakdown
- `plan_receipt.json` - Flow summary

### Build Produces

- Code changes
- Test changes
- `build_receipt.json` - Flow summary
- Draft PR

### Review Produces

- `review_worklist.md` - Feedback items
- Fixes applied
- `review_receipt.json` - Flow summary

### Gate Produces

- `merge_decision.md` - Ship/no-ship
- `gate_receipt.json` - Flow summary

### Deploy Produces

- `deployment_decision.md` - Deploy outcome
- `verification_report.md` - Health checks
- `deploy_receipt.json` - Flow summary

### Wisdom Produces

- `learnings.md` - Insights
- `wisdom_receipt.json` - Flow summary

## Iteration Patterns

### Single Pass

Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom

Standard for well-understood work.

### Design Iteration

Signal -> Plan <-> (design critique loop) -> Build -> ...

For complex architecture decisions.

### Build Iteration

... -> Build <-> (writer/critic loop) -> Review -> ...

For implementation refinement.

### Multi-Cycle

Full flow -> Wisdom -> (apply learnings) -> Signal -> ...

For ongoing development.

## When to Use Each Flow

| Situation     | Flows to Run                    |
| ------------- | ------------------------------- |
| New feature   | All 7                           |
| Bug fix       | Signal -> Build -> Gate         |
| Hotfix        | Build -> Gate -> Deploy         |
| Refactor      | Signal -> Plan -> Build -> Gate |
| Docs update   | Signal -> Build -> Review       |
| Investigation | Signal (partial)                |

## See Also

- [The Gate Pattern](principles/gate-pattern.md) - Boundary verification
- [Local Resolution](principles/local-resolution.md) - When to bounce
