"""`rerp ci` subcommands: patch-brrtrouter, fix-cargo-paths, is-tag, get-latest-tag, validate-version."""

import sys
from pathlib import Path

from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths
from rerp_tooling.ci.get_latest_tag import run as run_get_latest_tag
from rerp_tooling.ci.is_tag import run as run_is_tag
from rerp_tooling.ci.patch_brrtrouter import run as run_patch_brrtrouter
from rerp_tooling.ci.validate_version import run_validate_version_cli


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
    if args.ci_cmd == "get-latest-tag":
        rc = run_get_latest_tag()
        sys.exit(rc)
    if args.ci_cmd == "validate-version":
        current = getattr(args, "current", None)
        latest = getattr(args, "latest", None)
        allow_same = getattr(args, "allow_same", False)

        rc = run_validate_version_cli(current=current, latest=latest, allow_same=allow_same)
        sys.exit(rc)
    sys.exit(1)
