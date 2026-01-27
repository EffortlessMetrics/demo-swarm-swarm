## 2024-05-22 - Improving Generated Report Accessibility
**Learning:** For command-line or agent-driven tools without a traditional frontend, the "User Interface" is often the generated reports (Markdown). These reports can get very long, making navigation difficult for keyboard and screen reader users.
**Action:** Always include "Back to Top" links and status emojis in long generated Markdown reports to improve scannability and accessibility.

## 2024-05-24 - Agent Definitions as UI Source
**Learning:** In DemoSwarm, the `.claude/agents/*.md` files act as the "source code" for the user interface (the generated Markdown reports). Modifying these prompts is the direct way to implement UX changes.
**Action:** When asked for UX improvements in this repo, look for inconsistencies in agent output definitions.
