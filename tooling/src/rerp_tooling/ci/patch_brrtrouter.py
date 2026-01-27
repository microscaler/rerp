"""Patch Cargo.toml path deps for BRRTRouter and lifeguard to git. Used in CI."""

import logging
import re
import subprocess
import sys
from pathlib import Path

log = logging.getLogger(__name__)

BRRTRouter_GIT = '{ git = "https://github.com/microscaler/BRRTRouter", branch = "main" }'
LIFEGUARD_GIT = '{ git = "https://github.com/microscaler/lifeguard", branch = "main" }'

PATH_DEP_BRRTRouter = re.compile(
    r'((?:brrtrouter|brrtrouter_macros)\s*=\s*\{\s*path\s*=\s*["\'][^"\']*BRRTRouter[^"\']*["\'][^}]*\})',
    re.MULTILINE,
)
PATH_DEP_LIFEGUARD = re.compile(
    r'((?:lifeguard|lifeguard-derive|lifeguard-migrate)\s*=\s*\{\s*path\s*=\s*["\'][^"\']*lifeguard[^"\']*["\'][^}]*\})',
    re.MULTILINE,
)


def find_cargo_tomls(root: Path) -> list[Path]:
    """All Cargo.toml under root, excluding target, node_modules, .git."""
    exclude = {"target", "node_modules", ".git"}
    out = []
    for p in root.rglob("Cargo.toml"):
        try:
            rel = p.relative_to(root)
        except ValueError:
            continue
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
    """Return [(old_fragment, replacement)] for BRRTRouter and lifeguard path deps."""
    out: list[tuple[str, str]] = []
    for m in PATH_DEP_BRRTRouter.finditer(text):
        full = m.group(1)
        out.append((full, f"{_key_brrtrouter(full)} = {BRRTRouter_GIT}"))
    for m in PATH_DEP_LIFEGUARD.finditer(text):
        full = m.group(1)
        out.append((full, f"{_key_lifeguard(full)} = {LIFEGUARD_GIT}"))
    return out


def patch_file(
    p: Path, *, dry_run: bool = False, audit: bool = False
) -> tuple[bool, list[tuple[str, str]]]:
    """
    Patch one Cargo.toml: replace BRRTRouter/lifeguard path deps with git.
    Returns (changed, [(old, new)]). If audit or dry_run, does not write.
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

    if "BRRTRouter" in text and re.search(r'path\s*=\s*["\'][^"\']*BRRTRouter', text):
        print(
            f"error: {p} still contains path to BRRTRouter after patch; format may have changed",
            file=sys.stderr,
        )
        sys.exit(1)
    if re.search(r'path\s*=\s*["\'][^"\']*lifeguard', text):
        print(
            f"error: {p} still contains path to lifeguard after patch; format may have changed",
            file=sys.stderr,
        )
        sys.exit(1)

    if not dry_run:
        p.write_text(text)
    return True, matches


def run_cargo_update(workspace_dir: Path) -> None:
    """Run cargo update -p for brrtrouter, brrtrouter_macros, lifeguard*."""
    for packages in (
        ["brrtrouter", "brrtrouter_macros"],
        ["lifeguard", "lifeguard-derive", "lifeguard-migrate"],
    ):
        try:
            cmd = ["cargo", "update"] + [x for p in packages for x in ("-p", p)]
            r = subprocess.run(cmd, cwd=workspace_dir, capture_output=True, text=True)
            if r.returncode != 0 and "did not match any packages" in (r.stderr or ""):
                continue
            r.check_returncode()
        except FileNotFoundError as e:
            log.debug("cargo not in PATH, skipping cargo update: %s", e)
        except subprocess.CalledProcessError as e:
            print(
                f"error: cargo update failed in {workspace_dir}: {e.stderr or e}",
                file=sys.stderr,
            )
            sys.exit(1)


def run(root: Path, *, dry_run: bool = False, audit: bool = False) -> None:
    """
    Main entry: find Cargo.toml, patch those with BRRTRouter/lifeguard path deps,
    run cargo update in touched workspace roots. Idempotent.
    """
    cargo_tomls = find_cargo_tomls(root)
    patched_microservices_or_entities = False
    any_changed = False

    for p in cargo_tomls:
        changed, matches = patch_file(p, dry_run=dry_run, audit=audit)
        if not changed:
            continue
        any_changed = True
        try:
            rel = p.relative_to(root)
        except ValueError:
            rel = p
        if "microservices" in rel.parts or "entities" in rel.parts:
            patched_microservices_or_entities = True
        for old, new in matches:
            if audit:
                print(f"  {rel}: {old.strip()!r} -> {new!r}")
            elif dry_run:
                print(f"  {rel}: would replace {old.strip()!r} -> {new!r}")
            else:
                print(f"Patched {rel}")

    if audit:
        n_with = sum(1 for p in cargo_tomls if find_matches(p.read_text()))
        print(
            f"\nAudit: {len(cargo_tomls)} Cargo.toml scanned; {n_with} with BRRTRouter or lifeguard path deps."
        )
        return
    if dry_run:
        print("\nDry-run: would patch. Run without --dry-run to apply.")
        return
    if not any_changed:
        print("No Cargo.toml with BRRTRouter or lifeguard path deps found; nothing to patch.")
        return

    if patched_microservices_or_entities:
        d = root / "microservices"
        if (d / "Cargo.toml").exists():
            run_cargo_update(d)
            print("Ran cargo update in microservices/")
