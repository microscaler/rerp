"""Derive package names, binary names, and ports from OpenAPI layout and specs (no hardcoding)."""

from __future__ import annotations

from pathlib import Path
from typing import Optional

from .sources import (
    discover_bff_suite_config,
    discover_helm,
    discover_openapi_suite_microservice_localhost,
)
from .suites import iter_bffs, iter_suite_services


def get_package_names(project_root: Path, suite: Optional[str] = None) -> dict[str, str]:
    """Cargo [package].name per service: from openapi/{suite}/{service}/openapi.yaml and BFFs.

    Returns dict mapping service name (e.g. 'general-ledger', 'bff') to package name
    (e.g. 'rerp_accounting_general_ledger', 'rerp_accounting_bff'). Includes BFF services
    from bff-suite-config so brrtrouter-gen receives --package-name and writes correct
    [package].name in gen/Cargo.toml (e.g. rerp_accounting_bff_gen).

    When suite is set, only include services from that suite; when None, include all suites.
    """
    out: dict[str, str] = {}
    for s, service_name in iter_suite_services(project_root, suite=suite):
        snake = service_name.replace("-", "_")
        out[service_name] = f"rerp_{s}_{snake}"
    for bff_svc, s in iter_bffs(project_root, suite=suite):
        snake = bff_svc.replace("-", "_")
        out[bff_svc] = f"rerp_{s}_{snake}"
    return out


def get_binary_names(project_root: Path, suite: Optional[str] = None) -> dict[str, str]:
    """Binary name in build_artifacts/ and Dockerfile COPY: from service name (incl. BFF).

    Returns dict mapping service name (e.g. 'general-ledger', 'bff') to artifact name
    (e.g. 'general_ledger', 'bff').

    When suite is set, only include services from that suite; when None, include all suites.
    """
    out: dict[str, str] = {}
    for _s, service_name in iter_suite_services(project_root, suite=suite):
        out[service_name] = service_name.replace("-", "_")
    for bff_svc, _ in iter_bffs(project_root, suite=suite):
        out[bff_svc] = bff_svc.replace("-", "_")
    return out


def get_service_ports(project_root: Path) -> dict[str, str]:
    """HTTP port per service: from openapi servers (localhost), then bff-suite-config, then helm.

    Returns dict mapping service name to port string (e.g. '8001'). No hardcoding.
    """
    out: dict[str, str] = {}
    # Primary: openapi/{suite}/{name}/openapi.yaml servers[].url localhost:PORT
    for name, (_suite, port) in discover_openapi_suite_microservice_localhost(project_root).items():
        out[name] = str(port)
    # Fallbacks: bff-suite-config services.*.port, then helm values service.port
    for name, port in discover_bff_suite_config(project_root).items():
        out.setdefault(name, str(port))
    for name, port in discover_helm(project_root).items():
        out.setdefault(name, str(port))
    return out
