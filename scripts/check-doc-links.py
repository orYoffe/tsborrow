#!/usr/bin/env python3
"""Validate repository-local links in root and docs Markdown files."""

from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parent.parent
FILES = [ROOT / "README.md", *sorted((ROOT / "docs").rglob("*.md"))]
LINK = re.compile(r"\[[^\]]+\]\(([^)]+)\)")


def main() -> int:
    missing: list[str] = []
    for document in FILES:
        source = document.read_text(encoding="utf-8")
        for raw_target in LINK.findall(source):
            target = raw_target.split("#", 1)[0]
            if not target or "://" in target or target.startswith("mailto:"):
                continue
            if not (document.parent / target).resolve().exists():
                missing.append(f"{document.relative_to(ROOT)}: {raw_target}")
    if missing:
        print("Broken documentation links:", *missing, sep="\n- ", file=sys.stderr)
        return 1
    print(f"validated local links in {len(FILES)} documentation files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
