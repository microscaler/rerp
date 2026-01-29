"""Extract rerp-binaries-*.zip into microservices/target (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path


def run(input_dir: Path, project_root: Path) -> int:
    """Extract rerp-binaries-*.zip from input_dir into project_root/microservices/target. Returns 0 or 1."""
    from brrtrouter_tooling.docker.unpack_build_bins import run as run_brt

    return run_brt(
        input_dir,
        project_root,
        workspace_dir="microservices",
        zip_prefix="rerp-binaries-",
    )
