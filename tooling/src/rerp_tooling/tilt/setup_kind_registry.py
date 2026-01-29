"""RERP: delegate to brrtrouter_tooling.tilt.setup_kind_registry."""

from pathlib import Path

from brrtrouter_tooling.tilt.setup_kind_registry import run as _run


def run(project_root: Path) -> int:
    return _run(project_root)
