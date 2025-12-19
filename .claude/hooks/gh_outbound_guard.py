#!/usr/bin/env python3
import json
import re
import sys
from typing import Any

PATTERNS = [
    ("private-key", re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----")),
    ("github-token", re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{36,}\b")),
    ("aws-access-key", re.compile(r"\bAKIA[0-9A-Z]{16}\b")),
    ("bearer-token", re.compile(r"(?i)\bBearer\s+[A-Za-z0-9._-]{20,}\b")),
    ("db-url-password", re.compile(r"\b(postgres|mysql|mongodb)://[^:\s]+:[^@\s]+@")),
]


def walk_strings(x: Any):
    if isinstance(x, str):
        yield x
    elif isinstance(x, dict):
        for v in x.values():
            yield from walk_strings(v)
    elif isinstance(x, list):
        for v in x:
            yield from walk_strings(v)


def main() -> int:
    try:
        payload = json.load(sys.stdin)
    except Exception:
        return 0  # if we can't parse, don't brick tooling

    tool = payload.get("tool_name", "")
    tool_input = payload.get("tool_input", {})

    texts = []
    if tool == "Bash":
        cmd = tool_input.get("command", "")
        # Only guard likely GH-write commands, to avoid noisy false blocks
        if "gh " not in cmd or ("issue" not in cmd and "api" not in cmd):
            return 0
        texts.append(cmd)
    elif isinstance(tool, str) and tool.startswith("mcp__github__"):
        texts.extend(list(walk_strings(tool_input)))
    else:
        return 0

    hits = []
    for t in texts:
        for name, rx in PATTERNS:
            if rx.search(t):
                hits.append(name)

    if hits:
        hits = sorted(set(hits))
        print(
            "Blocked outbound GitHub write: potential secret pattern(s) detected: "
            + ", ".join(hits)
            + ". Remove the value from the issue/comment body or replace with [REDACTED:<type>].",
            file=sys.stderr,
        )
        return 2  # blocks PreToolUse

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
