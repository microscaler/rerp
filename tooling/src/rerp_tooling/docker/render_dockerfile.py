"""Render Dockerfile.template with service/system/module/port/binary_name (no per-service Dockerfiles)."""

from __future__ import annotations

import tempfile
from pathlib import Path

from rerp_tooling.build.constants import get_binary_names, get_service_ports
from rerp_tooling.discovery.suites import service_to_suite


def render_dockerfile_template(
    project_root: Path,
    template_path: Path,
    service_name: str,
    system: str | None = None,
) -> str:
    """Render Dockerfile.template with {{service_name}}, {{system}}, {{module}}, {{port}}, {{binary_name}}.

    service_name: e.g. 'invoice', 'general-ledger' (directory name under openapi/{suite}/).
    system: e.g. 'accounting'. If None, derived from discovery (service_to_suite).
    """
    system = system or service_to_suite(project_root, service_name) or "accounting"
    ports = get_service_ports(project_root)
    binaries = get_binary_names(project_root)
    port = ports.get(service_name, "8080")
    binary_name = binaries.get(service_name, service_name.replace("-", "_"))

    template_file = template_path if template_path.is_absolute() else project_root / template_path
    template_content = template_file.read_text()
    return (
        template_content.replace("{{service_name}}", service_name)
        .replace("{{system}}", system)
        .replace("{{module}}", service_name)
        .replace("{{port}}", port)
        .replace("{{binary_name}}", binary_name)
    )


def render_dockerfile_to_path(
    project_root: Path,
    template_path: Path,
    service_name: str,
    output_path: Path,
    system: str | None = None,
) -> None:
    """Render template and write to output_path (for CI: render then docker build -f output_path)."""
    content = render_dockerfile_template(project_root, template_path, service_name, system=system)
    out = project_root / output_path if not output_path.is_absolute() else output_path
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(content)


def render_dockerfile_to_temp(
    project_root: Path,
    template_path: Path,
    service_name: str,
    system: str | None = None,
) -> Path:
    """Render template to a temp file; caller must unlink when done."""
    content = render_dockerfile_template(project_root, template_path, service_name, system=system)
    with tempfile.NamedTemporaryFile(
        mode="w",
        prefix="Dockerfile.",
        suffix=".tmp",
        delete=False,
    ) as f:
        f.write(content)
    return Path(f.name)
