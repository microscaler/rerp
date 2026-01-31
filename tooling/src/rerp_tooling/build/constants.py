"""RERP build: package names, binary names, and ports derived from OpenAPI (no hardcoding)."""

from __future__ import annotations

from pathlib import Path
from typing import Optional

from rerp_tooling.discovery.services import (
    get_binary_names as _get_binary_names,
    get_package_names as _get_package_names,
    get_service_ports as _get_service_ports,
)


def get_package_names(project_root: Path, suite: Optional[str] = None) -> dict[str, str]:
    """Cargo [package].name per service. Derived from openapi/{suite}/{service}/openapi.yaml.

    When suite is set, only include services from that suite; when None, include all suites.
    """
    return _get_package_names(project_root, suite=suite)


def get_binary_names(project_root: Path, suite: Optional[str] = None) -> dict[str, str]:
    """Binary name in build_artifacts/ and Dockerfile COPY. Derived from service name.

    When suite is set, only include services from that suite; when None, include all suites.
    """
    return _get_binary_names(project_root, suite=suite)


def get_service_ports(project_root: Path) -> dict[str, str]:
    """HTTP port per service. From openapi servers, then bff-suite-config, then helm."""
    return _get_service_ports(project_root)
