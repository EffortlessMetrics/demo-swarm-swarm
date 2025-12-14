#!/usr/bin/env python3
"""
Check that .claude/ contains only portable content.

This script ensures the pack stays clean of harness-specific references
that would confuse users who copy .claude/ into their own repos.

Exit codes:
  0 - All checks pass
  1 - Found forbidden patterns
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]

# Simple substring patterns that indicate harness/Flow Studio coupling
FORBIDDEN_SUBSTRINGS = [
    "Flow Studio",      # Flow Studio UI
    "flow-studio",      # Flow Studio references
    "flow_studio",      # Flow Studio snake_case
    "localhost:5000",   # Flow Studio default port
    "UIID",             # Flow Studio UI identifiers
    "validate_swarm",   # Harness validation script
    "FR-OP-",           # Harness operationalization FRs (FR-OP-001..005)
    "selftest",         # Harness selftest system
    "swarm-ops",        # Harness ops agent
]

# Regex patterns for path-segment matching (avoids false positives like .demoswarm/)
# Match "swarm/" only when it's a directory segment, not part of "demoswarm/"
FORBIDDEN_PATTERNS_REGEX = [
    # Matches /swarm/, \swarm\, or start-of-string swarm/ — but NOT demoswarm/
    (re.compile(r'(?<![A-Za-z0-9])swarm[/\\]', re.IGNORECASE), "swarm/"),
]

# Patterns that are OK (functional requirement examples, not harness-specific)
ALLOWED_EXCEPTIONS = [
    "FR-001",  # Example FR IDs in requirements templates
    "FR-002",
    "FR-003",
    "FR-004",
    "FR-005",
    "FR-006",
]


def main() -> int:
    errors: list[tuple[pathlib.Path, str, int, str]] = []

    for path in ROOT.glob(".claude/**/*"):
        if not path.is_file():
            continue

        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            # Skip binary files
            continue

        lines = text.split("\n")
        for line_num, line in enumerate(lines, 1):
            # Check simple substring patterns
            for pattern in FORBIDDEN_SUBSTRINGS:
                if pattern in line:
                    # Check if this is an allowed exception
                    is_exception = any(exc in line for exc in ALLOWED_EXCEPTIONS)
                    if not is_exception:
                        rel = path.relative_to(ROOT)
                        errors.append((rel, pattern, line_num, line.strip()[:80]))

            # Check regex patterns (for segment-aware path matching)
            for regex, pattern_name in FORBIDDEN_PATTERNS_REGEX:
                if regex.search(line):
                    # Check if this is an allowed exception
                    is_exception = any(exc in line for exc in ALLOWED_EXCEPTIONS)
                    if not is_exception:
                        rel = path.relative_to(ROOT)
                        errors.append((rel, pattern_name, line_num, line.strip()[:80]))

    if errors:
        print("Found harness-specific references in portable .claude files:\n")
        for rel, pattern, line_num, snippet in errors:
            print(f"  {rel}:{line_num}")
            print(f"    Pattern: {pattern}")
            print(f"    Line: {snippet}")
            print()
        print(f"Total: {len(errors)} violation(s)")
        return 1

    print("Portable .claude check passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
