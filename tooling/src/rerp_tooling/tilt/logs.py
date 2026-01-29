"""RERP: delegate to brrtrouter_tooling.tilt.logs."""

from pathlib import Path

from brrtrouter_tooling.tilt.logs import run as _run


def run(component: str, project_root: Path) -> int:
    return _run(component, project_root)
