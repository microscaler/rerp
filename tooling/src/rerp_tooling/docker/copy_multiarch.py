"""Copy component binaries for all or one arch (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path


def run(system: str, module: str, arch: str, project_root: Path) -> int:
    """Copy from microservices/target to build_artifacts/{system}_{module}/{arch}. Returns 0 or 1."""
    from brrtrouter_tooling.docker.copy_multiarch import run as run_brt

    return run_brt(system, module, arch, project_root, workspace_dir="microservices")
