"""Derive package names, binary names, and ports from OpenAPI layout and specs (no hardcoding)."""

from __future__ import annotations

from pathlib import Path

from .sources import (
    discover_bff_suite_config,
    discover_helm,
    discover_openapi_suite_microservice_localhost,
)
from .suites import iter_suite_services


def get_package_names(project_root: Path) -> dict[str, str]:
    """Cargo [package].name per service: derived from openapi/{suite}/{service}/openapi.yaml.

    Returns dict mapping service name (e.g. 'general-ledger') to package name
    (e.g. 'rerp_accounting_general_ledger'). Key = directory name under openapi/{suite}/.
    """
    out: dict[str, str] = {}
    for suite, service_name in iter_suite_services(project_root):
        snake = service_name.replace("-", "_")
        out[service_name] = f"rerp_{suite}_{snake}"
    return out


def get_binary_names(project_root: Path) -> dict[str, str]:
    """Binary name in build_artifacts/ and Dockerfile COPY: derived from service name.

    Returns dict mapping service name (e.g. 'general-ledger') to artifact name
    (e.g. 'general_ledger'). Key = directory name under openapi/{suite}/.
    """
    out: dict[str, str] = {}
    for _suite, service_name in iter_suite_services(project_root):
        out[service_name] = service_name.replace("-", "_")
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
