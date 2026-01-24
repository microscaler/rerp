"""Fix operationId casing in OpenAPI YAML: convert camelCase to snake_case.

Matches BRRTRouter linter rules (operation_id_casing): list_pets, get_user, create_asset, etc.
"""

from __future__ import annotations

import re
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


# operationId line: groups (prefix, dq_value, sq_value, unquoted_value, trailing).
_OPID_RE = re.compile(
    r"^(\s*operationId:\s*)"
    r"(?:\"([^\"]*)\"|'([^']*)'|([A-Za-z0-9_\-]+))"
    r"(\s*(?:#.*)?)$"
)


def find_openapi_files(root: Path) -> list[Path]:
    out: list[Path] = []
    for name in ("openapi.yaml", "openapi.yml"):
        out.extend(root.rglob(name))
    return sorted(out)


def process_file(path: Path, dry_run: bool) -> tuple[int, list[tuple[int, str, str]]]:
    """Process one OpenAPI file. Returns (number of replacements, [(line_index, old_val, new_val), ...])."""
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    changes: list[tuple[int, str, str]] = []
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
        new_line = f"{prefix}{new_val}{trailing}\n"
        if not line.endswith("\n"):
            new_line = new_line.rstrip("\n")
        changes.append((i, raw, new_val))
        lines[i] = new_line

    if not changes:
        return 0, []

    if not dry_run:
        path.write_text("".join(lines), encoding="utf-8")

    return len(changes), changes


def run(
    openapi_dir: Path,
    dry_run: bool = False,
    verbose: bool = False,
    rel_to: Path | None = None,
) -> tuple[int, int]:
    """
    Find openapi.yaml/openapi.yml under openapi_dir, fix operationId to snake_case.
    Returns (total_replacements, files_touched).
    """
    if not openapi_dir.is_dir():
        return 0, 0
    base = rel_to or openapi_dir
    files = find_openapi_files(openapi_dir)
    total = 0
    touched = 0
    for p in files:
        n, changes = process_file(p, dry_run=dry_run)
        if n:
            total += n
            touched += 1
            if verbose:
                for i, old_v, new_v in changes:
                    print(f"  {p}:{i + 1}  {old_v!r} -> {new_v!r}")
            else:
                try:
                    rel = p.relative_to(base)
                except ValueError:
                    rel = p
                print(f"  {rel}: {n} operationId(s)")
    return total, touched
