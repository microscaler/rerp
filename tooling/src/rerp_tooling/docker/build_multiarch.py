"""Build and push multi-architecture Docker images (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path


def run(
    system: str,
    module: str,
    image_name: str,
    tag: str,
    push: bool,
    project_root: Path,
) -> int:
    """Build binaries, copy, buildx images, manifest, optional push. Returns 0 or 1."""
    from brrtrouter_tooling.docker.build_multiarch import run as run_brt

    build_cmd = [
        "tooling/.venv/bin/rerp",
        "build",
        f"{system}_{module}",
        "all",
    ]
    return run_brt(
        system,
        module,
        image_name,
        tag,
        push,
        project_root,
        build_cmd=build_cmd,
        base_image_name="rerp-base",
    )
