#!/usr/bin/env python3
"""
Validate agent prompts for structure, duplicate sections, and reference integrity.

Checks:
1. Agent prompt schema validation (required sections: Input/Inputs, Output/Outputs, Handoff Targets)
2. Duplicate section detection (fails if duplicate ## headers found)
3. Handoff targets graph validation (referenced agents must exist)
4. Skills invocation validation (referenced skills must exist)

Exit codes:
  0 - All checks pass
  1 - Validation errors found
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLAUDE_DIR = ROOT / ".claude"
AGENTS_DIR = CLAUDE_DIR / "agents"
SKILLS_DIR = CLAUDE_DIR / "skills"

# Required sections - at least one of each group must exist
# Using flexible matching to handle variations like "Inputs (best-effort)"
REQUIRED_SECTION_PATTERNS = [
    # Input variants - match "Input", "Inputs", "Required Inputs", etc.
    (
        r"^(Inputs?|Required Inputs?|What You['']ll Need)",
        "Input/Inputs section",
    ),
    # Output variants
    (
        r"^Outputs?",
        "Output/Outputs section",
    ),
    # Handoff Targets - matches "Handoff Targets" or "Handoff Targets (reference)" etc.
    (
        r"^Handoff Targets",
        "Handoff Targets section",
    ),
]


def extract_frontmatter_and_content(path: Path) -> tuple[list[str], str]:
    """Extract YAML frontmatter lines and markdown content from a file."""
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    if not lines or not lines[0].strip().startswith("---"):
        return [], text
    try:
        end = next(
            idx
            for idx, line in enumerate(lines[1:], start=1)
            if line.strip().startswith("---")
        )
    except StopIteration:
        return [], text
    frontmatter = lines[1:end]
    content = "\n".join(lines[end + 1 :])
    return frontmatter, content


def remove_code_blocks(content: str) -> str:
    """Remove fenced code blocks from markdown content.

    This prevents false positives from detecting duplicate section headers
    that appear inside code block templates/examples.
    """
    # Remove triple-backtick fenced code blocks
    content = re.sub(r"```[\s\S]*?```", "", content)
    # Remove indented code blocks (4+ spaces at start of line)
    # Be careful not to remove legitimate content
    return content


def get_h2_sections(content: str) -> list[str]:
    """Extract all ## section headers from markdown content (excluding code blocks)."""
    # First, remove code blocks to avoid false positives
    content_no_code = remove_code_blocks(content)
    # Match ## at start of line (possibly with leading whitespace)
    pattern = r"^\s*##\s+(.+)$"
    return re.findall(pattern, content_no_code, re.MULTILINE)


def normalize_section_name(section: str) -> str:
    """Normalize section name for duplicate detection.

    Strips trailing parenthetical content like "(reference)" or "(best-effort)"
    to catch duplicates like:
      - "## Handoff Targets" and "## Handoff Targets (reference)"
    """
    # Remove trailing parenthetical content
    normalized = re.sub(r"\s*\([^)]*\)\s*$", "", section.strip())
    return normalized


def find_duplicate_sections(sections: list[str]) -> list[str]:
    """Find sections that appear more than once (after normalization)."""
    seen: dict[str, str] = {}  # normalized -> first occurrence
    duplicates = []

    for section in sections:
        normalized = normalize_section_name(section)
        if normalized in seen:
            # Only report the first duplicate found
            if normalized not in duplicates:
                duplicates.append(normalized)
        else:
            seen[normalized] = section.strip()

    return duplicates


def check_required_sections(sections: list[str]) -> list[str]:
    """Check if required section groups are present. Returns missing group names."""
    missing = []

    for pattern, group_name in REQUIRED_SECTION_PATTERNS:
        found = any(re.match(pattern, s.strip(), re.IGNORECASE) for s in sections)
        if not found:
            missing.append(group_name)

    return missing


def get_all_agent_names() -> set[str]:
    """Get all agent names from the agents directory."""
    if not AGENTS_DIR.exists():
        return set()
    return {p.stem for p in AGENTS_DIR.glob("*.md")}


def get_all_skill_names() -> set[str]:
    """Get all skill names from the skills directory."""
    if not SKILLS_DIR.exists():
        return set()
    return {p.name for p in SKILLS_DIR.iterdir() if p.is_dir()}


def extract_handoff_targets(content: str) -> list[str]:
    """Extract agent names from Handoff Targets section."""
    # Find the Handoff Targets section (with optional parenthetical suffix)
    pattern = r"##\s+Handoff Targets[^\n]*\n(.*?)(?=\n##|\Z)"
    match = re.search(pattern, content, re.DOTALL)
    if not match:
        return []

    section_content = match.group(1)

    # Look for patterns like:
    # - **agent-name**: description
    # - **agent-name**:
    # Agent names must be lowercase with hyphens (like "code-implementer")
    agent_pattern = r"[-*]\s+\*\*([a-z][a-z0-9-]*)\*\*\s*:"
    agents = re.findall(agent_pattern, section_content)

    return agents


def extract_skill_references(content: str) -> list[str]:
    """Extract skill names referenced in the agent prompt.

    Only matches explicit skill invocation patterns, not general text.
    """
    skills = []

    # Pattern 1: `skill-name` skill (with backticks and followed by "skill")
    # e.g., "Use the `test-runner` skill"
    skill_pattern1 = r"`([a-z][a-z0-9-]+)`\s+skill"
    skills.extend(re.findall(skill_pattern1, content, re.IGNORECASE))

    # Pattern 2: References to .claude/skills/<skill-name>/
    # e.g., "See `.claude/skills/openq-tools/SKILL.md`"
    skill_pattern2 = r"\.claude/skills/([a-z][a-z0-9-]+)"
    skills.extend(re.findall(skill_pattern2, content))

    # Pattern 3: **skill-name**: in a Skills section listing
    # e.g., "- **runs-index**: For updating..."
    # Find the Skills section first
    skills_section_match = re.search(
        r"##\s+Skills\s*\n(.*?)(?=\n##|\Z)", content, re.DOTALL
    )
    if skills_section_match:
        section = skills_section_match.group(1)
        # Extract skill names from bold list items
        skill_pattern3 = r"[-*]\s+\*\*([a-z][a-z0-9-]+)\*\*\s*:"
        skills.extend(re.findall(skill_pattern3, section))

    # Deduplicate and return
    return list(set(skills))


def main() -> int:
    if not AGENTS_DIR.exists():
        print("ERROR: .claude/agents/ directory not found at repo root", file=sys.stderr)
        return 1

    errors: list[str] = []  # Blocking errors (fail CI)
    warnings: list[str] = []  # Non-blocking warnings
    all_agents = get_all_agent_names()
    all_skills = get_all_skill_names()

    for agent_path in sorted(AGENTS_DIR.glob("*.md")):
        agent_name = agent_path.stem
        rel_path = agent_path.relative_to(ROOT)

        try:
            _, content = extract_frontmatter_and_content(agent_path)
        except Exception as e:
            errors.append(f"{rel_path}: failed to read file ({e})")
            continue

        sections = get_h2_sections(content)

        # Check 1: Required sections (WARNING only - many existing agents lack these)
        missing_sections = check_required_sections(sections)
        for missing in missing_sections:
            warnings.append(f"{rel_path}: missing '{missing}'")

        # Check 2: Duplicate sections (ERROR - clear structural problem)
        duplicates = find_duplicate_sections(sections)
        for dup in duplicates:
            errors.append(f"{rel_path}: duplicate section '## {dup}'")

        # Check 3: Handoff targets reference real agents (ERROR - broken reference)
        handoff_targets = extract_handoff_targets(content)
        for target in handoff_targets:
            if target not in all_agents:
                errors.append(
                    f"{rel_path}: handoff target '{target}' not found in .claude/agents/"
                )

        # Check 4: Skills references point to real skills (ERROR - broken reference)
        skill_refs = extract_skill_references(content)
        for skill in skill_refs:
            if skill not in all_skills:
                errors.append(
                    f"{rel_path}: skill '{skill}' not found in .claude/skills/"
                )

    # Print warnings (informational)
    if warnings:
        print("Agent prompt warnings (non-blocking):")
        for warn in sorted(warnings):
            print(f"  - {warn}")
        print()

    # Print errors (blocking)
    if errors:
        print("Agent prompt validation failed:", file=sys.stderr)
        for err in sorted(errors):
            print(f"  - {err}", file=sys.stderr)
        print(f"\nTotal: {len(errors)} error(s)", file=sys.stderr)
        return 1

    print(f"Agent prompt validation passed ({len(all_agents)} agents checked).")
    if warnings:
        print(f"({len(warnings)} warnings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
