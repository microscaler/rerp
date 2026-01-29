"""Copy binary to dest and write SHA256 (delegate to brrtrouter_tooling)."""

from __future__ import annotations

from pathlib import Path


def run(source: Path, dest: Path, binary_name: str, project_root: Path) -> int:
    """Copy source to dest, chmod +x, write {dest}.sha256. Returns 0 or 1."""
    from brrtrouter_tooling.docker.copy_binary import run as run_brt

    return run_brt(source, dest, binary_name, project_root)
