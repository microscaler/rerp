"""Build Docker image and push or kind load (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path
from typing import Optional


def run(
    image_name: str,
    hash_path: Path,
    artifact_path: Path,
    project_root: Path,
    system: Optional[str] = None,
    module: Optional[str] = None,
    port: Optional[int] = None,
    binary_name: Optional[str] = None,
    dockerfile: Optional[Path] = None,
) -> int:
    """Build Docker image. Returns 0 on success, 1 on error."""
    from brrtrouter_tooling.docker.build_image_simple import run as run_brt

    return run_brt(
        image_name,
        hash_path,
        artifact_path,
        project_root,
        system=system,
        module=module,
        port=port,
        binary_name=binary_name,
        dockerfile=dockerfile,
        base_image_name="rerp-base",
        kind_cluster_name="rerp",
    )
