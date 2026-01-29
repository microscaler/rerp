"""Build rerp-base from docker/base/Dockerfile (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path


def run(project_root: Path, push: bool = False, dry_run: bool = False) -> int:
    """Build docker/base/Dockerfile as rerp-base:latest. Returns 0 or 1."""
    from brrtrouter_tooling.docker.build_base import run as run_brt

    return run_brt(
        project_root,
        push=push,
        dry_run=dry_run,
        base_image_name="rerp-base",
    )
