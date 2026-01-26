"""`rerp ci` subcommands: patch-brrtrouter, fix-cargo-paths, is-tag."""

import sys
from pathlib import Path

from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths
from rerp_tooling.ci.is_tag import run as run_is_tag
from rerp_tooling.ci.patch_brrtrouter import run as run_patch_brrtrouter


def run_ci(args, project_root: Path) -> None:
    if args.ci_cmd == "patch-brrtrouter":
        run_patch_brrtrouter(
            project_root,
            dry_run=getattr(args, "dry_run", False),
            audit=getattr(args, "audit", False),
        )
        sys.exit(0)
    if args.ci_cmd == "fix-cargo-paths":
        run_fix_cargo_paths(Path(args.cargo_toml), project_root=project_root)
        sys.exit(0)
    if args.ci_cmd == "is-tag":
        rc = run_is_tag()
        sys.exit(rc)
    sys.exit(1)
