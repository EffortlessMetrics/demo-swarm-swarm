#!/usr/bin/env python3
"""
Pack contract linter.

Checks for common control-plane drifts:
- No ESCALATE in control-plane vocab.
- Repo Operator Result blocks include publish_surface.
- Gate Result blocks keep the closed recommended_action enum.
- Flow docs contain publish-blocked guidance (no implicit skip).
- Flow docs only reference declared agent names (frontmatter `name:`).

Hook behavior:
- When invoked as a Claude Code PreToolUse hook, this linter runs only when the tool input references `.claude/`.
  Otherwise it exits 0 silently to avoid interrupting normal work.
"""

from __future__ import annotations

import json
import re
import sys
import unicodedata
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[2]
CLAUDE_ROOT = REPO_ROOT / ".claude"


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def iter_invisible_chars(text: str):
    for line_no, line in enumerate(text.splitlines(), 1):
        for col_no, ch in enumerate(line, 1):
            if ch in ("\n", "\r"):
                continue
            cat = unicodedata.category(ch)
            if cat in {"Cc", "Cf"}:
                yield line_no, col_no, ch, cat


def walk_strings(x: Any):
    if isinstance(x, str):
        yield x
    elif isinstance(x, dict):
        for v in x.values():
            yield from walk_strings(v)
    elif isinstance(x, list):
        for v in x:
            yield from walk_strings(v)


def maybe_load_hook_payload() -> dict[str, Any] | None:
    if sys.stdin is None or sys.stdin.isatty():
        return None
    try:
        payload = json.load(sys.stdin)
    except Exception:
        return None
    return payload if isinstance(payload, dict) else None


def should_run_for_payload(payload: dict[str, Any] | None) -> bool:
    if payload is None:
        return True  # manual run

    tool = payload.get("tool_name", "")
    tool_input = payload.get("tool_input", {})

    texts: list[str] = []
    if tool == "Bash" and isinstance(tool_input, dict):
        texts.append(str(tool_input.get("command", "")))
    else:
        texts.extend(str(s) for s in walk_strings(tool_input))

    return any((".claude/" in t) or (".claude\\" in t) for t in texts)


def main() -> int:
    payload = maybe_load_hook_payload()
    if not should_run_for_payload(payload):
        return 0

    errors: list[str] = []
    md_files = list(CLAUDE_ROOT.rglob("*.md"))
    agent_md_files = list((CLAUDE_ROOT / "agents").glob("*.md"))

    # 1) ESCALATE vocabulary purge (only in control-plane contexts)
    escalate_pattern = re.compile(
        r'^\s*recommended_action:\s*.*\bESCALATE\b'
        r'|'
        r'\brecommended_action\b[^|]*\bESCALATE\b'  # enum definitions like "PROCEED | ESCALATE"
        r'|'
        r'`ESCALATE`',  # code-formatted references to the action
        re.MULTILINE
    )
    for path in md_files:
        text = read_text(path)
        if escalate_pattern.search(text):
            errors.append(f"{path}: contains forbidden control-plane vocab 'ESCALATE' in action context")

    # 2) Repo Operator Result must include publish_surface
    repo_block = re.compile(r"## Repo Operator Result.*?(?=##|$)", re.DOTALL)
    for path in md_files:
        text = read_text(path)
        for match in repo_block.finditer(text):
            if "publish_surface" not in match.group(0):
                errors.append(f"{path}: Repo Operator Result block missing publish_surface")

    # 3) Gate Result recommended_action enum must be closed
    gate_block = re.compile(r"## Gate Result.*?(?=##|$)", re.DOTALL)
    for path in md_files:
        text = read_text(path)
        for match in gate_block.finditer(text):
            line_match = re.search(r"recommended_action:\s*(.+)", match.group(0))
            if not line_match:
                errors.append(f"{path}: Gate Result block missing recommended_action line")
                continue
            line = line_match.group(1)
            if "PROCEED" not in line or "RERUN" not in line or "BOUNCE" not in line or "FIX_ENV" not in line:
                errors.append(f"{path}: Gate Result recommended_action enum is not closed (PROCEED|RERUN|BOUNCE|FIX_ENV)")

    # 4) Flow docs must state publish-blocked semantics (no skip)
    flow_docs = sorted((CLAUDE_ROOT / "commands").glob("flow-*-*.md"))
    for path in flow_docs:
        text = read_text(path)
        if "Publish blocked" not in text and "publish blocked" not in text:
            errors.append(f"{path}: missing publish-blocked -> RESTRICTED guidance")

    # 5) Command docs must not contain invisible/control characters.
    command_docs = sorted((CLAUDE_ROOT / "commands").glob("*.md"))
    bad_bytes = set(range(0x00, 0x20)) - {0x0A, 0x0D}  # allow LF/CR only
    bad_bytes.add(0x7F)  # DEL
    for path in command_docs:
        data = path.read_bytes()
        offenders = sorted({b for b in data if b in bad_bytes})
        if offenders:
            errors.append(f"{path}: contains ASCII control bytes {', '.join(hex(b) for b in offenders)}")

        # Also catch Unicode format controls (e.g., zero-width spaces) when UTF-8 decodes.
        text = data.decode("utf-8", errors="replace")
        for line_no, col_no, ch, cat in iter_invisible_chars(text):
            if ch == "\t":
                continue
            errors.append(
                f"{path}:{line_no}:{col_no}: contains {cat} char U+{ord(ch):04X} ({unicodedata.name(ch, 'UNKNOWN')})"
            )
            break  # one per file is enough

    # 6) Flow docs must reference declared agent names (frontmatter `name:`).
    def extract_frontmatter_name(text: str) -> str | None:
        lines = text.splitlines()
        if not lines:
            return None
        try:
            start = next(i for i, line in enumerate(lines) if line.strip() == "---")
        except StopIteration:
            return None
        try:
            end = next(i for i in range(start + 1, len(lines)) if lines[i].strip() == "---")
        except StopIteration:
            return None
        for line in lines[start + 1 : end]:
            if line.startswith("name:"):
                name = line[len("name:") :].strip()
                return name or None
        return None

    agent_names: set[str] = set()
    for path in agent_md_files:
        name = extract_frontmatter_name(read_text(path))
        if not name:
            errors.append(f"{path}: missing frontmatter name:")
            continue
        agent_names.add(name)

        # Optional smell: filename != name
        if path.stem != name:
            print(f"[contract-lint] WARNING: {path}: filename '{path.stem}' != frontmatter name '{name}'")

    def is_candidate_agent_token(token: str) -> bool:
        if token in agent_names:
            return True

        # Heuristic: avoid flagging hyphenated prose like "no-op" or "fix-forward" by requiring
        # a common agent suffix when the token is not declared.
        suffixes = (
            "-analyst",
            "-analyzer",
            "-assessor",
            "-auditor",
            "-author",
            "-checker",
            "-cleanup",
            "-critic",
            "-customizer",
            "-decider",
            "-designer",
            "-enforcer",
            "-executor",
            "-fixer",
            "-framer",
            "-historian",
            "-loader",
            "-manager",
            "-monitor",
            "-normalizer",
            "-operator",
            "-planner",
            "-prep",
            "-reporter",
            "-researcher",
            "-resolver",
            "-runner",
            "-sanitizer",
            "-synthesizer",
            "-triager",
            "-verifier",
        )
        return "-" in token and token.endswith(suffixes)

    def parse_agent_sequence_prefix(s: str) -> list[str]:
        s = s.lstrip()
        if not s:
            return []

        def is_agent_char(ch: str) -> bool:
            return ch.isascii() and (ch.islower() or ch.isdigit() or ch == "-")

        i = 0
        while i < len(s) and is_agent_char(s[i]):
            i += 1
        first = s[:i]
        if not first or not is_candidate_agent_token(first):
            return []

        out = [first]
        while i < len(s):
            j = i
            while j < len(s) and s[j].isspace():
                j += 1
            if j == i or j >= len(s):
                break
            sep = s[j]
            if sep not in ("/", "+"):
                break
            k = j + 1
            while k < len(s) and s[k].isspace():
                k += 1
            if k == j + 1 or k >= len(s):
                break
            start2 = k
            while k < len(s) and is_agent_char(s[k]):
                k += 1
            token = s[start2:k]
            if token and is_candidate_agent_token(token):
                out.append(token)
            i = k
        return out

    import re as _re

    re_checkbox = _re.compile(r"^\s*-\s*\[\s*[xX ]\s*\]\s*(.+)$")
    re_order = _re.compile(r"^\s*\d+\)\s*(.+)$")
    re_plus = _re.compile(r"\+\s+([a-z][a-z0-9-]+)")
    re_arrow = _re.compile(r"->\s+([a-z][a-z0-9-]+)")
    re_parens = _re.compile(r"\(([^)]*)\)")
    re_token = _re.compile(r"[a-z][a-z0-9-]+")

    IGNORE_TOKENS = {"run-id"}

    def extract_agent_refs(task_text: str) -> set[str]:
        refs: set[str] = set()

        leading = parse_agent_sequence_prefix(task_text)
        refs.update(leading)

        for m in re_plus.finditer(task_text):
            refs.add(m.group(1))
        for m in re_arrow.finditer(task_text):
            refs.add(m.group(1))

        # Only treat parentheticals as agent refs if the line doesn't already start with an agent.
        if not leading:
            for m in re_parens.finditer(task_text):
                inner = m.group(1)
                for t in re_token.findall(inner):
                    if is_candidate_agent_token(t):
                        refs.add(t)

        return {r for r in refs if r not in IGNORE_TOKENS}

    for path in flow_docs:
        text = read_text(path)
        for line_no, line in enumerate(text.splitlines(), 1):
            m = re_checkbox.match(line) or re_order.match(line)
            if not m:
                continue
            for token in extract_agent_refs(m.group(1)):
                if token not in agent_names and is_candidate_agent_token(token):
                    errors.append(f"{path}:{line_no}: references missing agent name '{token}'")

    if errors:
        for e in errors:
            print(f"[contract-lint] {e}")
        return 1

    print("contract-lint: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
