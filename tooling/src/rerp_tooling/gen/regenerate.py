"""Regenerate services from OpenAPI specs: delegate to brrtrouter_tooling with RERP fix_cargo_paths."""

from pathlib import Path
from typing import Optional

from brrtrouter_tooling.gen import (
    regenerate_service as brrt_regenerate_service,
)
from brrtrouter_tooling.gen import (
    regenerate_suite_services as brrt_regenerate_suite_services,
)

from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths


def _fix_cargo_paths_callback(cargo_toml_path: Path, project_root: Optional[Path]) -> None:
    run_fix_cargo_paths(cargo_toml_path, project_root=project_root)


def regenerate_service(
    project_root: Path,
    suite: str,
    service_name: str,
    brrtrouter_path: Optional[Path] = None,
) -> int:
    """Regenerate a single service. Returns 0 on success, 1 on error."""
    return brrt_regenerate_service(
        project_root,
        suite,
        service_name,
        brrtrouter_path=brrtrouter_path,
        fix_cargo_paths_fn=_fix_cargo_paths_callback,
    )


def regenerate_suite_services(
    project_root: Path,
    suite: str,
    service_names: list[str],
    brrtrouter_path: Optional[Path] = None,
) -> int:
    """Regenerate all services in a suite. Returns 0 if all succeed, 1 if any fail."""
    return brrt_regenerate_suite_services(
        project_root,
        suite,
        service_names,
        brrtrouter_path=brrtrouter_path,
        fix_cargo_paths_fn=_fix_cargo_paths_callback,
    )
