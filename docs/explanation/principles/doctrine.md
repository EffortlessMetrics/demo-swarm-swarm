# The Doctrine

> The operating constitution for agent-driven development.

---

## The Doctrine

We run with high autonomy inside a sandbox because generation and iteration are cheap. Safety and trust come from boundaries: evidence, critique loops, and gated publishing. Agents are single-responsibility ICs called to do work or compress context. The orchestrator routes on handoff prose, not parsed fields. Artifacts exist to carry meaning across time and make review cheap. If an artifact is not worth reading, do not write it. PARTIAL is a successful outcome when reported honestly with next steps.

---

## Core Tenets

**Default-allow engineering, gated publishing.**
Inside the sandbox, agents explore and iterate without permission. Gates engage only at publish boundaries: commit, push, merge, deploy. The sandbox is the security boundary, not the prompt.

**Machine time is cheap, human attention is expensive.**
Let agents try ten approaches to find what works. Reserve human judgment for decisions that matter: should we ship this? Is this the right approach? Does this solve the problem?

**Agents are ICs; orchestrator is PM.**
Agents do focused cognitive work: investigate, reason, implement, critique. Orchestrators scope tasks, route work, and make sequencing decisions. Communication is natural language throughout.

**Small scoped loops beat big prompts.**
Tight feedback loops (implement, test, critique, fix) produce better results than monolithic prompts that try to get everything right on the first pass. Iteration is the path to quality.

**Permissions are not safety; boundaries are safety.**
Approval-per-action provides no security benefit inside a sandbox. Real safety comes from architecture: isolated environments, gated publishing, secrets scanning, human review at merge.

**Partial is success if honest and actionable.**
An agent that completes 60% and documents what is done, what is blocked, and what to try next has succeeded. Hidden uncertainty behind false completion is the actual failure mode.

**Structure is for evidence, not for routing.**
Artifacts exist because their content matters: receipts tell stories, decisions explain reasoning. The orchestrator routes on natural language handoffs, not parsed control-plane fields.

---

## What This Enables

**High velocity.**
Try multiple approaches rapidly. Run tests without asking. Fix issues immediately. Iterate to working code in a single session. No permission dialogs, no approval workflows, no stop-and-wait cycles.

**Deep focus.**
Maintain context across long operations. Follow a complex implementation through. Handle multi-file changes atomically. No constant interruption or context loss from approval cycles.

**Quality through iteration.**
More iterations equals better code. Try something, see if it works. Run tests, fix failures. Get critique, address issues. Repeat until solid. This is only possible with freedom to iterate.

---

## What This Requires

**A real sandbox.**
Isolated environment with no production access. No credentials in the sandbox. Git is the only path out. Push is the boundary. If the sandbox leaks, you do not have a gate pattern; you have theater.

**Publish gates.**
Secrets scanning before commit. Repo hygiene before push. Evidence review before merge. Governance check before deploy. Gates are fast, consistent, and automated.

**Human merge decisions.**
Humans own the final decision at the merge boundary. The PR cockpit (brief, receipts, evidence pointers) is the primary interface. The diff is for spot-check and audit.

**Evidence trails.**
Artifacts that mean something to the next reader (human or agent). Receipts summarize what happened. Decision documents explain reasoning. Write for comprehension, not for protocol compliance.

---

## When This Breaks Down

**If the sandbox is not real.**
If agents can accidentally affect production from the "sandbox," gates become theater. Real isolation is the foundation everything else rests on.

**If gates are skipped.**
Pushing without secrets scan. Merging without evidence review. Deploying without governance check. "We trust Claude" does not mean "skip the gates." It means trust inside the sandbox, verify at boundaries.

**If nobody reads the PR cockpit.**
The system produces evidence for human review. If reviewers rubber-stamp without reading, the feedback loop breaks. Review is a decision point, not a formality.

**If partial progress is punished.**
Agents under pressure to "complete" will guess to finish. If honest partial reports are treated as failures, agents learn to hide uncertainty. The result is downstream failures from false completion signals.

---

## See Also

- [Operational Philosophy](../operational-philosophy.md) — bypassPermissions and why it works
- [The Gate Pattern](gate-pattern.md) — Engineering is default-allow, publishing is gated
- [Agent Philosophy](../agent-philosophy.md) — How agents think, act, and fail gracefully
