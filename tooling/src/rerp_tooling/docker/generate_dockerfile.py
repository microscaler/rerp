"""Generate service-specific Dockerfile from template (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path
from typing import Optional


def generate_dockerfile(
    system: str,
    module: str,
    port: int = 8000,
    project_root: Optional[Path] = None,
    template_path: Optional[Path] = None,
    output_path: Optional[Path] = None,
) -> Path:
    """Generate Dockerfile for a service. Returns the output path."""
    from brrtrouter_tooling.docker.generate_dockerfile import generate_dockerfile as gen_brt

    return gen_brt(
        system,
        module,
        port=port,
        project_root=project_root,
        template_path=template_path,
        output_path=output_path,
        binary_name_pattern="rerp_{system}_{module}_impl",
    )


def run(system: str, module: str, port: int = 8000, project_root: Optional[Path] = None) -> int:
    """CLI entry: generate Dockerfile. Returns 0."""
    from brrtrouter_tooling.docker.generate_dockerfile import run as run_brt

    return run_brt(
        system,
        module,
        port=port,
        project_root=project_root,
        binary_name_pattern="rerp_{system}_{module}_impl",
    )
