#!/usr/bin/env python3
"""
Patch Cargo.toml to use BRRTRouter and lifeguard from git instead of local paths.

Used in CI where ../../BRRTRouter, ../../lifeguard (and variants) are not available.
Run from repo root. Idempotent.

  python3 scripts/patch-brrtrouter-for-ci.py         # patch, verify, cargo update
  python3 scripts/patch-brrtrouter-for-ci.py --audit # list Cargo.toml and matches only
  python3 scripts/patch-brrtrouter-for-ci.py --dry-run  # show changes, no write, no cargo

Repo keeps path deps for local dev; CI runs this before build.
"""

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

BRRTRouter_GIT = '{ git = "https://github.com/microscaler/BRRTRouter", branch = "main" }'
LIFEGUARD_GIT = '{ git = "https://github.com/microscaler/lifeguard", branch = "main" }'

# Match brrtrouter or brrtrouter_macros with path = "..../BRRTRouter/..." (any relative path)
PATH_DEP_BRRTRouter = re.compile(
    r'((?:brrtrouter|brrtrouter_macros)\s*=\s*\{\s*path\s*=\s*["\'][^"\']*BRRTRouter[^"\']*["\'][^}]*\})',
    re.MULTILINE,
)
# Match lifeguard, lifeguard-derive, lifeguard-migrate with path = "..../lifeguard/..."
PATH_DEP_LIFEGUARD = re.compile(
    r'((?:lifeguard|lifeguard-derive|lifeguard-migrate)\s*=\s*\{\s*path\s*=\s*["\'][^"\']*lifeguard[^"\']*["\'][^}]*\})',
    re.MULTILINE,
)


def find_cargo_tomls() -> list[Path]:
    """All Cargo.toml under repo, excluding target and typical non-source trees."""
    exclude = {"target", "node_modules", ".git"}
    out = []
    for p in ROOT.rglob("Cargo.toml"):
        rel = p.relative_to(ROOT)
        if any(part in exclude for part in rel.parts):
            continue
        out.append(p)
    return sorted(out)


def _key_brrtrouter(full: str) -> str:
    return "brrtrouter_macros" if "brrtrouter_macros" in full.split("=")[0] else "brrtrouter"


def _key_lifeguard(full: str) -> str:
    s = full.split("=")[0]
    if "lifeguard-migrate" in s:
        return "lifeguard-migrate"
    if "lifeguard-derive" in s:
        return "lifeguard-derive"
    return "lifeguard"


def find_matches(text: str) -> list[tuple[str, str]]:
    """Return list of (old_fragment, replacement)."""
    out = []
    for m in PATH_DEP_BRRTRouter.finditer(text):
        full = m.group(1)
        repl = f"{_key_brrtrouter(full)} = {BRRTRouter_GIT}"
        out.append((full, repl))
    for m in PATH_DEP_LIFEGUARD.finditer(text):
        full = m.group(1)
        repl = f"{_key_lifeguard(full)} = {LIFEGUARD_GIT}"
        out.append((full, repl))
    return out


def patch_file(p: Path, *, dry_run: bool, audit: bool) -> tuple[bool, list[tuple[str, str]]]:
    """
    Patch one Cargo.toml. Returns (changed, list of (old, new) replacements).
    If audit, only gather matches; if dry_run, do not write.
    """
    if not p.exists():
        return False, []
    text = p.read_text().replace("\r\n", "\n").replace("\r", "\n")
    matches = find_matches(text)
    if not matches:
        return False, []

    if audit:
        return True, matches

    for old, new in matches:
        text = text.replace(old, new, 1)

    # Verify no path to BRRTRouter or lifeguard remains (catches format we don't match)
    if "BRRTRouter" in text and re.search(r'path\s*=\s*["\'][^"\']*BRRTRouter', text):
        print(
            f"error: {p.relative_to(ROOT)} still contains path to BRRTRouter after patch; "
            "format may have changed",
            file=sys.stderr,
        )
        sys.exit(1)
    if re.search(r'path\s*=\s*["\'][^"\']*lifeguard', text):
        print(
            f"error: {p.relative_to(ROOT)} still contains path to lifeguard after patch; "
            "format may have changed",
            file=sys.stderr,
        )
        sys.exit(1)

    if not dry_run:
        p.write_text(text)
    return True, matches


def _cargo_update_packages(workspace_dir: Path, packages: list[str]) -> bool:
    """Run cargo update -p for each in packages. Return True if ran successfully, False if skipped (did not match)."""
    try:
        cmd = ["cargo", "update"] + [x for p in packages for x in ("-p", p)]
        r = subprocess.run(cmd, cwd=workspace_dir, capture_output=True, text=True)
        if r.returncode != 0 and "did not match any packages" in (r.stderr or ""):
            return False
        r.check_returncode()
        return True
    except FileNotFoundError:
        print(f"warning: cargo not in PATH, skipping cargo update in {workspace_dir}", file=sys.stderr)
        return False
    except subprocess.CalledProcessError as e:
        print(f"error: cargo update failed in {workspace_dir}: {e.stderr or e}", file=sys.stderr)
        sys.exit(1)


def run_cargo_update(workspace_dir: Path) -> None:
    _cargo_update_packages(workspace_dir, ["brrtrouter", "brrtrouter_macros"])
    _cargo_update_packages(workspace_dir, ["lifeguard", "lifeguard-derive", "lifeguard-migrate"])


def main() -> None:
    audit = "--audit" in sys.argv
    dry_run = "--dry-run" in sys.argv

    cargo_tomls = find_cargo_tomls()
    patched_under_components = False
    patched_under_microservices = False
    patched_under_entities = False
    any_changed = False
    n_with_deps = 0

    for p in cargo_tomls:
        changed, matches = patch_file(p, dry_run=dry_run, audit=audit)
        if not changed:
            continue
        n_with_deps += 1
        any_changed = True
        rel = p.relative_to(ROOT)
        if "components" in rel.parts:
            patched_under_components = True
        if "microservices" in rel.parts:
            patched_under_microservices = True
        if "entities" in rel.parts:
            patched_under_entities = True
        for old, new in matches:
            if audit:
                print(f"  {rel}: {old.strip()!r} -> {new!r}")
            elif dry_run:
                print(f"  {rel}: would replace {old.strip()!r} -> {new!r}")
        if not audit and not dry_run:
            print(f"Patched {rel}")

    if audit:
        print(f"\nAudit: {len(cargo_tomls)} Cargo.toml scanned; {n_with_deps} with BRRTRouter or lifeguard path deps.")
        return

    if dry_run:
        print(f"\nDry-run: would patch {n_with_deps} file(s). Run without --dry-run to apply.")
        return

    if not any_changed:
        print("No Cargo.toml with BRRTRouter or lifeguard path deps found; nothing to patch.")
        return

    # Refresh Cargo.lock in workspace roots we touched
    if patched_under_components:
        d = ROOT / "components"
        if (d / "Cargo.toml").exists():
            run_cargo_update(d)
            print("Ran cargo update in components/")
    if patched_under_microservices or patched_under_entities:
        d = ROOT / "microservices"
        if (d / "Cargo.toml").exists():
            run_cargo_update(d)
            print("Ran cargo update in microservices/")


if __name__ == "__main__":
    main()
