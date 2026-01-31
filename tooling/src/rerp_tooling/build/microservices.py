"""Build microservices/accounting workspace (host_aware for jemalloc opt-in; brrtrouter_tooling for single-package)."""

from __future__ import annotations

import sys
from pathlib import Path

from brrtrouter_tooling.build import build_package_with_options
from brrtrouter_tooling.gen import run_gen_if_missing_for_suite

from rerp_tooling.build.constants import get_package_names
from rerp_tooling.build.host_aware import run as run_host_aware
from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths
from rerp_tooling.discovery import suite_sub_service_names


def _fix_cargo_paths_callback(cargo_toml_path: Path, project_root: Path | None) -> None:
    run_fix_cargo_paths(cargo_toml_path, project_root=project_root)


def _gen_package_name_for_service(project_root: Path, _suite: str, service_name: str) -> str | None:
    """Gen crate [package].name for (suite, service_name). Used by run_gen_if_missing_for_suite."""
    pkg = get_package_names(project_root).get(service_name)
    return f"{pkg}_gen" if pkg else None


def run_accounting_gen_if_missing(project_root: Path) -> None:
    """Generate gen crates for accounting suite if missing (via brrtrouter_tooling)."""
    run_gen_if_missing_for_suite(
        project_root,
        "accounting",
        workspace_dir="microservices",
        get_service_names_fn=lambda root, suite: list(suite_sub_service_names(root, suite)),
        fix_cargo_paths_fn=_fix_cargo_paths_callback,
        package_name_for_service=lambda _s, sn: _gen_package_name_for_service(project_root, _s, sn),
    )


def build_microservices_workspace(project_root: Path, arch: str, release: bool) -> int:
    """Build microservices/ workspace. arch: amd64|arm64|arm7. Uses jemalloc for amd64/arm64 (CI opt-in). Returns 0/1."""
    run_accounting_gen_if_missing(project_root)
    return run_host_aware(
        target="workspace",
        arch=arch,
        extra_args=None,
        project_root=project_root,
        release=release,
    )


def build_microservice(project_root: Path, name: str, release: bool) -> int:
    """Build one accounting microservice. name e.g. general-ledger. Returns 0/1."""
    package_names = get_package_names(project_root)
    pkg = package_names.get(name)
    if not pkg:
        print(
            f"❌ unknown service: {name}. Valid: {', '.join(package_names)}",
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
