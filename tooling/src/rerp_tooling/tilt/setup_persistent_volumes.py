"""RERP: delegate to brrtrouter_tooling.tilt.setup_persistent_volumes (default pv paths)."""

from pathlib import Path

from brrtrouter_tooling.tilt.setup_persistent_volumes import run as _run


def run(project_root: Path) -> int:
    return _run(project_root)  # uses default pv_paths (k8s/data, k8s/monitoring)
