"""Build microservices/accounting workspace (delegate to brrtrouter_tooling; RERP package list + gen)."""

from __future__ import annotations

import sys
from pathlib import Path

from brrtrouter_tooling.build import (
    build_package_with_options,
    build_workspace_with_options,
)

from rerp_tooling.build.constants import PACKAGE_NAMES
from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths
from rerp_tooling.discovery import suite_sub_service_names


def _fix_cargo_paths_callback(cargo_toml_path: Path, project_root: Path | None) -> None:
    run_fix_cargo_paths(cargo_toml_path, project_root=project_root)


def run_accounting_gen_if_missing(project_root: Path) -> None:
    """Generate gen crates for accounting suite if missing (via brrtrouter_tooling)."""
    from brrtrouter_tooling.gen.regenerate import run_gen_if_missing_for_suite

    run_gen_if_missing_for_suite(
        project_root,
        "accounting",
        workspace_dir="microservices",
        get_service_names_fn=lambda root, suite: list(suite_sub_service_names(root, suite)),
        fix_cargo_paths_fn=_fix_cargo_paths_callback,
    )


def build_microservices_workspace(project_root: Path, arch: str, release: bool) -> int:
    """Build microservices/ workspace. arch: amd64|arm64|arm7. Returns 0/1."""
    return build_workspace_with_options(
        project_root,
        workspace_dir="microservices",
        arch=arch,
        release=release,
        gen_if_missing_callback=run_accounting_gen_if_missing,
    )


def build_microservice(project_root: Path, name: str, release: bool) -> int:
    """Build one accounting microservice. name e.g. general-ledger. Returns 0/1."""
    pkg = PACKAGE_NAMES.get(name)
    if not pkg:
        print(
            f"❌ unknown service: {name}. Valid: {', '.join(PACKAGE_NAMES)}",
            file=sys.stderr,
        )
        return 1
    return build_package_with_options(
        project_root,
        workspace_dir="microservices",
        package_name=pkg,
        arch="amd64",
        release=release,
        gen_if_missing_callback=run_accounting_gen_if_missing,
    )
