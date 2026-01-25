"""Bump version in all Cargo.toml [package] and [workspace.package] sections.

Source of truth: components/Cargo.toml [workspace.package].version.
Walks the entire repo (rglob) for Cargo.toml; excludes target, .git, .venv, node_modules, build, dist.
Accepts version = \"v0.1.0\" or \"0.1.0\" when reading; always writes \"X.Y.Z\" (no \"v\") to Cargo.toml.
Tags remain \"vX.Y.Z\" (workflow adds \"v\" when creating the git tag).
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

# Sections that define a package/workspace version we own (not [dependencies]).
VERSION_SECTIONS = ("package", "workspace.package")

# Paths to skip when walking for Cargo.toml (e.g. target, deps, vendored).
SKIP_PARTS = ("target", ".git", ".venv", "node_modules", "build", "dist")


def _read_current(components_toml: Path) -> str:
    """Read version from components/Cargo.toml [workspace.package].version. Raises on parse error."""
    text = components_toml.read_text()
    in_sec = False
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("["):
            in_sec = s.strip("[]").strip() == "workspace.package"
            continue
        if in_sec:
            m = re.match(r'^\s*version\s*=\s*"v?(\d+\.\d+\.\d+)"', line)
            if m:
                return m.group(1)  # Always return canonical X.Y.Z (no "v") for Cargo.toml
    msg = "Could not find [workspace.package].version in components/Cargo.toml"
    raise SystemExit(msg)


def _next_version(old: str, bump: str) -> str:
    """Compute next semver from old (X.Y.Z or vX.Y.Z) and bump (patch|minor|major). Always returns X.Y.Z."""
    old = old.lstrip("v")
    parts = old.split(".")
    if len(parts) != 3 or not all(p.isdigit() for p in parts):
        msg = f"Invalid version in components/Cargo.toml: {old}"
        raise SystemExit(msg)
    x, y, z = int(parts[0]), int(parts[1]), int(parts[2])
    b = (bump or "patch").lower()
    if b == "patch":
        z += 1
    elif b == "minor":
        y += 1
        z = 0
    elif b == "major":
        x += 1
        y = z = 0
    else:
        msg = f"Unknown bump: {bump}. Use patch, minor, or major."
        raise SystemExit(msg)
    return f"{x}.{y}.{z}"


def _replace_in_file(path: Path, old: str, new: str) -> bool:
    """Replace version = \"old\" with version = \"new\" only in [package] or [workspace.package]. Returns True if changed."""
    text = path.read_text()
    lines = text.splitlines(keepends=True)
    out: list[str] = []
    in_sec = False
    replaced = False
    for line in lines:
        s = line.strip()
        if s.startswith("["):
            in_sec = s.strip("[]").strip() in VERSION_SECTIONS
            out.append(line)
            continue
        if in_sec and not replaced:
            # Match version = "0.1.0" or version = "v0.1.0"; replace with version = "X.Y.Z" (always no "v")
            pat = r'(\s*version\s*=\s*")v?' + re.escape(old) + r'"'
            if re.search(pat, line):
                new_line = re.sub(pat, lambda m: m.group(1) + new + '"', line, count=1)
                out.append(new_line)
                replaced = True
                continue
        out.append(line)
    if replaced:
        path.write_text("".join(out))
    return replaced


def _cargo_toml_paths(project_root: Path) -> list[Path]:
    """All Cargo.toml under project_root (root, components/, entities/, microservices/, etc.), excluding SKIP_PARTS."""
    out: list[Path] = []
    for p in project_root.rglob("Cargo.toml"):
        if any(part in p.parts for part in SKIP_PARTS):
            continue
        try:
            if not p.is_file():
                continue
        except OSError:
            continue
        out.append(p)
    return sorted(out)


def run(project_root: Path, bump: str) -> int:
    """Bump version: read from components/Cargo.toml, walk all Cargo.toml, replace in [package]/[workspace.package]. Returns 0 or 1."""
    components_toml = project_root / "components" / "Cargo.toml"
    if not components_toml.is_file():
        print("components/Cargo.toml not found", file=sys.stderr)
        return 1

    old = _read_current(components_toml)
    new = _next_version(old, bump)

    updated: list[Path] = []
    for p in _cargo_toml_paths(project_root):
        try:
            if _replace_in_file(p, old, new):
                updated.append(p.relative_to(project_root))
        except Exception as e:
            print(f"Error updating {p}: {e}", file=sys.stderr)
            return 1

    if not updated:
        print(f"No Cargo.toml had [package]/[workspace.package].version = {old!r}", file=sys.stderr)
        return 1

    print(f"Bumped {old} -> {new} ({bump}); updated {len(updated)} file(s)")
    for u in updated:
        print(f"  {u}")

    # GitHub Actions: append to GITHUB_OUTPUT for step outputs
    go = os.environ.get("GITHUB_OUTPUT")
    if go:
        with open(go, "a") as f:
            f.write(f"version={new}\n")

    return 0
