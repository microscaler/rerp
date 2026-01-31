"""RERP copy-artifacts: wrap brrtrouter_tooling.docker.copy_artifacts with discovery-derived names."""

from __future__ import annotations

from pathlib import Path
from typing import Optional

from rerp_tooling.build.constants import get_binary_names, get_package_names

__all__ = ["run", "validate_build_artifacts"]


def run(arch: str, project_root: Path, suite: Optional[str] = None) -> int:
    """Copy binaries to build_artifacts/{arch}. Names derived from openapi layout.

    When suite is set, only copy binaries for that suite; when None, all suites (for matrix later).
    """
    from brrtrouter_tooling.docker.copy_artifacts import run as _run_copy_artifacts

    return _run_copy_artifacts(
        arch,
        project_root,
        package_names=get_package_names(project_root, suite=suite),
        binary_names=get_binary_names(project_root, suite=suite),
    )


def validate_build_artifacts(project_root: Path, suite: Optional[str] = None) -> int:
    """Validate build_artifacts contain expected binaries. Names derived from openapi layout.

    When suite is set, only validate binaries for that suite; when None, all suites.
    """
    from brrtrouter_tooling.docker.copy_artifacts import (
        validate_build_artifacts as _validate_build_artifacts,
    )

    return _validate_build_artifacts(project_root, get_binary_names(project_root, suite=suite))
