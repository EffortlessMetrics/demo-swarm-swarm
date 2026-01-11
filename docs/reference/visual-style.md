# Visual Style

> Standards for diagrams in DemoSwarm documentation.

---

## Purpose

Diagrams should clarify, not decorate. Every diagram must:

- **Answer a specific question** - What flow does this change? How do these components connect?
- **Be small enough to parse in 30 seconds** - If it takes longer, split it
- **Link to source artifacts when making claims** - Diagrams that describe behavior should point to the authoritative source

---

## Mermaid Conventions

### Flow Direction

| Direction | Use For |
|-----------|---------|
| Top-to-bottom (`TB`) | Sequential flows, agent chains, decision trees |
| Left-to-right (`LR`) | Data transformation pipelines, compressor patterns |

```mermaid
flowchart TB
    A[Signal] --> B[Plan]
    B --> C[Build]
    C --> D[Review]
```

### Node Naming

- Use descriptive labels, not abbreviations (`requirements-author` not `req-auth`)
- Keep labels under 4 words
- Sentence case preferred (`Test executor` not `TEST EXECUTOR`)

### Node Shapes

Use shapes consistently for semantic meaning:

```mermaid
flowchart LR
    A[Rectangle: Process/Action]
    B([Stadium: Start/End])
    C{Diamond: Decision}
    D[(Cylinder: Storage/Data)]
    E{{Hexagon: External System}}
```

| Shape | Meaning | Example |
|-------|---------|---------|
| Rectangle `[text]` | Process or action | `[code-implementer]` |
| Stadium `([text])` | Start or end point | `([Request])` |
| Diamond `{text}` | Decision point | `{Tests pass?}` |
| Cylinder `[(text)]` | Storage or data | `[(requirements.md)]` |
| Hexagon `{{text}}` | External system | `{{GitHub API}}` |

### Edge Labels

- Keep short (1-3 words)
- Use for conditions or transformations
- Omit if the relationship is obvious

```mermaid
flowchart TB
    A{Tests pass?}
    A -->|Yes| B[Proceed to gate]
    A -->|No| C[Route to fixer]
```

---

## Semantic Palette

Use consistent colors for semantic meaning. Colors supplement labels; do not rely on color alone.

| Color | Hex | Meaning | Use For |
|-------|-----|---------|---------|
| Blue | `#4A90D9` | Work plane | Internal operations, building, testing |
| Green | `#7CB342` | Success | Verified, clean, passing |
| Orange | `#F5A623` | Warning | Needs attention, partial success |
| Red | `#D94A4A` | Blocked | Failed, critical, stopped |
| Purple | `#9C27B0` | Publish plane | External boundaries, GitHub ops |
| Gray | `#9E9E9E` | Inactive | Historical, optional, skipped |

### Applying Colors in Mermaid

```mermaid
flowchart LR
    A[Internal work]:::workPlane
    B[Publish gate]:::publishPlane
    C[Verified]:::success

    A --> B --> C

    classDef workPlane fill:#4A90D9,color:#fff
    classDef publishPlane fill:#9C27B0,color:#fff
    classDef success fill:#7CB342,color:#fff
```

### Example: Two-Plane Visualization

```mermaid
flowchart TB
    subgraph Work["Work Plane (default-allow)"]
        A[Read files]:::workPlane
        B[Write code]:::workPlane
        C[Run tests]:::workPlane
    end

    subgraph Publish["Publish Plane (gated)"]
        D{Secrets scan}:::publishPlane
        E[Push]:::publishPlane
    end

    A --> B --> C --> D
    D -->|Pass| E
    D -->|Fail| F[Block]:::blocked

    classDef workPlane fill:#4A90D9,color:#fff
    classDef publishPlane fill:#9C27B0,color:#fff
    classDef blocked fill:#D94A4A,color:#fff
```

---

## Diagram Types

### Flow Diagrams

Show sequence of operations.

**Use for:** Agent routing, flow progression, decision trees

**Keep to:** 5-10 nodes max

```mermaid
flowchart TB
    A[Test fails] --> B{Flaky?}
    B -->|Yes| C[flakiness-detector]
    B -->|No| D{Test bug?}
    D -->|Yes| E[test-author]
    D -->|No| F[fixer]
```

### Sequence Diagrams

Show interactions over time.

**Use for:** Agent handoffs, API calls, multi-party coordination

**Limit to:** 4-5 participants

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant I as code-implementer
    participant C as code-critic

    O->>I: Implement AC-1
    I->>I: Write code
    I-->>O: Result: COMPLETE
    O->>C: Critique
    C->>C: Review code
    C-->>O: Result: VERIFIED
```

### State Diagrams

Show state transitions.

**Use for:** Run states, PR states, gate decisions

**Keep to:** 5-7 states max

```mermaid
stateDiagram-v2
    [*] --> Draft: PR created
    Draft --> Ready: Review complete
    Ready --> Merged: Gate passes
    Ready --> Draft: Feedback received
    Merged --> [*]
```

### Architecture Diagrams

Show structure.

**Use for:** System overview, component relationships

**Use subgraphs for grouping**

```mermaid
flowchart TB
    subgraph Signal["Flow 1: Signal"]
        S1[requirements-author]
        S2[bdd-author]
    end

    subgraph Plan["Flow 2: Plan"]
        P1[adr-author]
        P2[work-planner]
    end

    S1 --> P1
    S2 --> P2
```

---

## Size Budgets

| Diagram Type | Max Nodes | Max Edges | If Larger... |
|--------------|-----------|-----------|--------------|
| Flow | 10 | 15 | Split into sub-diagrams |
| Sequence | 5 participants | 12 messages | Split into phases |
| State | 7 | 12 | Hierarchical states |
| Architecture | 12 | 20 | Use subgraphs |

**No hairballs.** If you cannot parse it in 30 seconds, it is too complex. Split it or remove detail.

---

## Canonical Diagram Types

Three diagram patterns recur throughout DemoSwarm documentation. Use these as templates for consistency.

### Intent-to-Narrative Pipeline (LR)

Shows transformation from user intent through processing to observable outcomes. Use left-to-right (`LR`) orientation.

**Use for:** Feature flows, signal-to-artifact pipelines, data transformations

```mermaid
flowchart LR
    A([User intent]):::intent --> B[Signal]:::workPlane
    B --> C[Plan]:::workPlane
    C --> D[Build]:::workPlane
    D --> E[(Artifact)]:::evidence
    E --> F([Narrative]):::evidence

    classDef intent fill:#4A90D9,color:#fff
    classDef workPlane fill:#9E9E9E,color:#fff
    classDef evidence fill:#7CB342,color:#fff
```

**Pattern:** Intent (blue) flows through work (gray) to produce evidence (green).

### Truth Arbitration Stack (TD)

Shows authority hierarchy where higher layers override lower layers. Use top-to-bottom (`TD`) orientation.

**Use for:** Configuration precedence, trust hierarchies, verification stacks

```mermaid
flowchart TD
    A[Human decision]:::halt --> B[Gate receipt]:::evidence
    B --> C[Test results]:::evidence
    C --> D[Agent claims]:::workPlane
    D --> E[LLM output]:::workPlane

    classDef halt fill:#D94A4A,color:#fff
    classDef evidence fill:#7CB342,color:#fff
    classDef workPlane fill:#9E9E9E,color:#fff
```

**Pattern:** Higher = more authoritative. Red at top indicates halting authority.

### Boundary Model (Work vs Publish)

Shows the two-plane architecture: default-allow work plane and gated publish plane. Uses subgraphs to separate domains.

**Use for:** Permission models, gate explanations, security boundaries

```mermaid
flowchart TB
    subgraph Work["Work Plane"]
        direction LR
        A[Read]:::workPlane --> B[Write]:::workPlane
        B --> C[Test]:::workPlane
    end

    subgraph Publish["Publish Plane"]
        D{Gate}:::boundary --> E[Push]:::boundary
    end

    C --> D
    D -->|Fail| F[Block]:::halt

    classDef workPlane fill:#9E9E9E,color:#fff
    classDef boundary fill:#F5A623,color:#000
    classDef halt fill:#D94A4A,color:#fff
```

**Pattern:** Work is gray (neutral, default-allow). Boundaries are orange (attention required). Halting is red.

---

## Color Semantics

Colors communicate meaning. Use sparingly and consistently.

| Semantic | Color | Hex | When to Use |
|----------|-------|-----|-------------|
| Intent | Blue | `#4A90D9` | User requests, goals, inputs that drive work |
| Work plane | Gray | `#9E9E9E` | Internal operations, building, iteration (default-allow) |
| Evidence | Green | `#7CB342` | Verified artifacts, passing tests, proof |
| Boundaries | Orange | `#F5A623` | Gates, attention points, transitions between planes |
| Halting | Red | `#D94A4A` | Blocked, failed, stopped, human-required decisions |

### Usage Guidelines

1. **One color per semantic** - Do not use blue for both intent and work
2. **Gray is default** - Most nodes should be gray; color highlights exceptions
3. **Red means stop** - Reserve red for genuine halting conditions, not warnings
4. **Orange draws attention** - Use for boundaries that require verification
5. **Green confirms** - Use only for verified/evidenced states, not hopes

### Mermaid Class Definitions

Standard class definitions for consistent styling:

```mermaid
classDef intent fill:#4A90D9,color:#fff
classDef workPlane fill:#9E9E9E,color:#fff
classDef evidence fill:#7CB342,color:#fff
classDef boundary fill:#F5A623,color:#000
classDef halt fill:#D94A4A,color:#fff
```

---

## Section Budgets

Beyond node/edge limits, apply these constraints:

### One Diagram Per Section

Each documentation section should contain at most one diagram. If you need multiple diagrams:
- Split into subsections
- Question whether all diagrams are necessary
- Consider if prose would suffice

### Avoid Universe Diagrams

Do not create diagrams that show "everything." Signs of a universe diagram:
- More than 3 subgraphs
- Nodes that require scrolling to see
- Legend longer than the diagram
- "See diagram for overview" as the only explanation

**Instead:** Create focused diagrams for specific questions. Link between documents for broader context.

### Complexity Budget

| If you have... | Then limit to... |
|----------------|------------------|
| 1 subgraph | 8 nodes |
| 2 subgraphs | 6 nodes per subgraph |
| 3 subgraphs | 4 nodes per subgraph |
| More subgraphs | Split into multiple diagrams |

---

## Diagram Discipline

### Every Diagram Must Have

1. **A title** - What question does this answer?
2. **Context** - Where does this fit? (preceding prose)
3. **Source links** - If it makes claims about artifacts (e.g., "tests run in this order"), link to the source

### Diagrams Should Not

- **Duplicate prose** - If the text already explains it, the diagram should show structure, not repeat content
- **Claim unverified behavior** - Do not diagram what "should" happen; diagram what does happen
- **Use colors randomly** - Every color must have semantic meaning (see palette above)
- **Include every detail** - That is what documentation is for; diagrams show shape and flow

### ASCII Art Alternative

For simple flows in code comments or when Mermaid is not available:

```
┌──────────────────────┐
│  requirements-author │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│    requirements.md   │
└──────────┬───────────┘
           ▼
┌──────────────────────┐
│   bdd-author         │
└──────────────────────┘
```

See [agent-data-flows.md](agent-data-flows.md) for examples of ASCII art diagrams.

---

## Accessibility

- **Do not rely solely on color** - Always include text labels
- **Ensure sufficient contrast** - Use white text (`color:#fff`) on colored backgrounds
- **Provide text descriptions for complex diagrams** - A sentence or two explaining what the diagram shows

---

## Tools

| Tool | When to Use |
|------|-------------|
| Mermaid | Primary choice; renders in GitHub, docs sites |
| ASCII art | Simple flows in code comments, fallback when Mermaid unavailable |

**Avoid:** External image files (cannot version, cannot search, break on path changes)

---

## Examples in This Repository

Diagrams in use that follow these conventions:

| Document | Diagram Type | Purpose |
|----------|--------------|---------|
| [pr-as-review-surface.md](../explanation/pr-as-review-surface.md) | Flow | Shows flow progression |
| [agent-data-flows.md](agent-data-flows.md) | ASCII art | Producer-consumer relationships |
| [architecture.md](../explanation/architecture.md) | ASCII art | Compressor pattern illustration |

---

## Quick Reference

```mermaid
flowchart TB
    subgraph "Node Shapes"
        A[Process]
        B([Start/End])
        C{Decision}
        D[(Data)]
        E{{External}}
    end
```

```mermaid
flowchart LR
    subgraph "Color Classes"
        W[work]:::workPlane
        P[publish]:::publishPlane
        S[success]:::success
        X[blocked]:::blocked
    end

    classDef workPlane fill:#4A90D9,color:#fff
    classDef publishPlane fill:#9C27B0,color:#fff
    classDef success fill:#7CB342,color:#fff
    classDef blocked fill:#D94A4A,color:#fff
```

---

## See Also

- [documentation-conventions.md](documentation-conventions.md) - Writing style for docs
- [agent-data-flows.md](agent-data-flows.md) - ASCII diagram examples
- [pr-as-review-surface.md](../explanation/pr-as-review-surface.md) - PR visualization patterns
- [architecture.md](../explanation/architecture.md) - System diagrams
