#!/usr/bin/env python3
"""
Fix operationId casing in OpenAPI YAML files: convert camelCase to snake_case.

Matches BRRTRouter linter rules (operation_id_casing) and the petstore example:
- list_pets, get_user, create_asset, etc.

Usage:
  python fix_operation_id_casing.py [ROOT] [--dry-run] [--verbose]

ROOT: Directory to search for openapi.yaml/openapi.yml (default: current dir)
--dry-run: Print changes without writing
--verbose: Print each file and conversion
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


def is_snake_case(s: str) -> bool:
    """Match BRRTRouter linter: non-empty, first is lower or '_', all lower/digit/underscore."""
    if not s:
        return False
    if s[0] != "_" and not s[0].islower():
        return False
    return all(c.islower() or c.isdigit() or c == "_" for c in s)


def to_snake_case(s: str) -> str:
    """Port of BRRTRouter src/linter.rs to_snake_case. Converts camelCase, kebab-case, spaces."""
    result: list[str] = []
    for ch in s:
        if ch.isupper():
            if result and result[-1] != "_":
                result.append("_")
            result.append(ch.lower())
        elif ch.islower() or ch.isdigit():
            result.append(ch)
        elif ch in "- ":
            if result and result[-1] != "_":
                result.append("_")
        else:
            result.append(ch)
    return "".join(result)


# Matches: "  operationId: value" or '  operationId: "value"' or "  operationId: 'value'"
# Captures: (prefix, dq_content or None, sq_content or None, unquoted or None, trailing)
_OPID_RE = re.compile(
    r"^(\s*operationId:\s*)"
    r"(?:\"([^\"]*)\"|'([^']*)'|([A-Za-z0-9_\-]+))"
    r"(\s*(?:#.*)?)$"
)


def process_file(path: Path, dry_run: bool, verbose: bool) -> int:
    """Process one OpenAPI file. Returns number of replacements."""
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    changes: list[tuple[int, str, str]] = []  # (line_index, old_val, new_val)
    for i, line in enumerate(lines):
        m = _OPID_RE.match(line.rstrip("\n\r"))
        if not m:
            continue
        raw = m.group(2) or m.group(3) or m.group(4)
        if is_snake_case(raw):
            continue
        new_val = to_snake_case(raw)
        if new_val == raw:
            continue
        prefix, _, _, _, trailing = m.groups()
        # Emit unquoted for snake_case (safe)
        new_line = f"{prefix}{new_val}{trailing}\n"
        if not line.endswith("\n"):
            new_line = new_line.rstrip("\n")
        changes.append((i, raw, new_val))
        lines[i] = new_line

    if not changes:
        return 0

    if verbose:
        for i, old_v, new_v in changes:
            print(f"  {path}:{i+1}  {old_v!r} -> {new_v!r}")

    if not dry_run:
        path.write_text("".join(lines), encoding="utf-8")

    return len(changes)


def find_openapi_files(root: Path) -> list[Path]:
    out: list[Path] = []
    for name in ("openapi.yaml", "openapi.yml"):
        out.extend(root.rglob(name))
    return sorted(out)


def main() -> int:
    ap = argparse.ArgumentParser(
        description="Convert operationId from camelCase to snake_case in OpenAPI YAML files."
    )
    ap.add_argument(
        "root",
        nargs="?",
        default=".",
        help="Root directory to search (default: current directory)",
    )
    ap.add_argument("--dry-run", action="store_true", help="Only print changes, do not write")
    ap.add_argument("--verbose", "-v", action="store_true", help="Print each file and conversion")
    args = ap.parse_args()

    root = Path(args.root).resolve()
    if not root.is_dir():
        print(f"Not a directory: {root}", file=sys.stderr)
        return 1

    files = find_openapi_files(root)
    if not files:
        print(f"No openapi.yaml or openapi.yml under {root}")
        return 0

    total = 0
    touched = 0
    for p in files:
        n = process_file(p, dry_run=args.dry_run, verbose=args.verbose)
        if n:
            total += n
            touched += 1
            if not args.verbose:
                print(f"  {p.relative_to(root)}: {n} operationId(s)")

    if touched:
        mode = "[DRY-RUN] " if args.dry_run else ""
        print(f"{mode}Updated {touched} file(s), {total} operationId(s) converted to snake_case.")
    else:
        print("No operationId casing changes needed.")

    return 0


if __name__ == "__main__":
    sys.exit(main())
