#!/usr/bin/env python3
"""Rust lint gate for the microservices workspace (CI: checks.yml `rust-lint`; policy: docs/RUST_LINTS.md).

1. Every crate compiles, generated ones included: `cargo check --workspace --all-targets`.
2. Every crate that opts into the workspace lints (`[lints] workspace = true`) is clippy-clean with -D warnings.
   Generated `gen/` crates keep clippy's defaults; they are fixed in BRRTRouter's templates, not here.
3. Every hand-written crate in a strict suite has opted in, so a new crate cannot slip past step 2.

Usage: python3 tooling/scripts/lint_rust.py [--no-check] [--list]
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKSPACE = ROOT / "microservices"
STRICT_SUITES = ("crm", "accounting")
# Opted-in crates the clippy step skips, with the reason. Keep this short and dated; each entry is debt.
SKIP_CLIPPY = {
    # clippy's -D warnings reaches workspace dependencies too, and brrtrouter-gen emits unused imports in BFF
    # handlers (BRRTRouter bug, 2026-11-09). cargo check still covers the crate.
    "rerp_accounting_bff": "its generated dependency accounting/bff/gen has unused imports (BRRTRouter generator)",
}
OPTED_IN = re.compile(r"^\[lints\]\s*\n\s*workspace\s*=\s*true\s*$", re.MULTILINE)


def members() -> list[tuple[str, Path]]:
    out = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        cwd=WORKSPACE,
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    meta = json.loads(out)
    ids = set(meta["workspace_members"])
    return [
        (p["name"], Path(p["manifest_path"]).parent) for p in meta["packages"] if p["id"] in ids
    ]


def is_generated(crate_dir: Path) -> bool:
    return crate_dir.name == "gen"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--no-check", action="store_true", help="skip the workspace-wide cargo check"
    )
    parser.add_argument("--list", action="store_true", help="print the opted-in crates and exit")
    args = parser.parse_args()

    strict, missing = [], []
    for name, crate_dir in members():
        rel = crate_dir.relative_to(WORKSPACE)
        if OPTED_IN.search((crate_dir / "Cargo.toml").read_text()):
            strict.append(name)
        elif rel.parts[0] in STRICT_SUITES and not is_generated(crate_dir):
            missing.append(str(rel))
    strict.sort()

    if args.list:
        print("\n".join(strict))
        return 0
    if missing:
        print(
            "These hand-written crates do not opt into the workspace lints; add `[lints]\\nworkspace = true`:"
        )
        for rel in missing:
            print(f"  microservices/{rel}/Cargo.toml")
        return 1

    if not args.no_check:
        print("== cargo check --workspace --all-targets", flush=True)
        if subprocess.run(
            ["cargo", "check", "--workspace", "--all-targets"], cwd=WORKSPACE
        ).returncode:
            return 1

    for name, why in SKIP_CLIPPY.items():
        if name in strict:
            strict.remove(name)
            print(f"   skipping clippy for {name}: {why}")
    print(f"== cargo clippy -D warnings on {len(strict)} hand-written crates", flush=True)
    cmd = ["cargo", "clippy", "--all-targets"]
    for name in strict:
        cmd += ["-p", name]
    cmd += ["--", "-D", "warnings"]
    return subprocess.run(cmd, cwd=WORKSPACE).returncode


if __name__ == "__main__":
    sys.exit(main())
