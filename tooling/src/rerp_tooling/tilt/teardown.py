"""Tilt teardown: tilt down, stop/rm containers, optional images/volumes/prune. Replaces teardown-tilt.sh."""

from __future__ import annotations

import subprocess
from pathlib import Path

from rerp_tooling.discovery import tilt_service_names


def run(
    project_root: Path,
    remove_images: bool = False,
    remove_volumes: bool = False,
    system_prune: bool = False,
) -> int:
    """Stop Tilt, containers, optionally remove images/volumes, optionally docker system prune. Returns 0."""
    subprocess.run(["pkill", "-f", "tilt up"], capture_output=True)
    subprocess.run(["tilt", "down"], capture_output=True)
    for c in ["postgres-dev", "redis-dev", "prometheus-dev", "grafana-dev"]:
        subprocess.run(["docker", "stop", c], capture_output=True)
        subprocess.run(["docker", "rm", c], capture_output=True)
    for s in tilt_service_names(project_root):
        subprocess.run(["docker", "stop", f"rerp-{s}-dev"], capture_output=True)
        subprocess.run(["docker", "rm", f"rerp-{s}-dev"], capture_output=True)
    if remove_images:
        for s in tilt_service_names(project_root):
            subprocess.run(["docker", "rmi", f"rerp-accounting-{s}:latest"], capture_output=True)
            subprocess.run(
                ["docker", "rmi", f"localhost:5001/rerp-accounting-{s}:tilt"], capture_output=True
            )
    if remove_volumes:
        for v in ["postgres_data", "redis_data", "prometheus_data", "grafana_data"]:
            subprocess.run(["docker", "volume", "rm", v], capture_output=True)
    subprocess.run(["docker", "network", "prune", "-f"], capture_output=True)
    if system_prune:
        subprocess.run(["docker", "system", "prune", "-f"], capture_output=True)
    print("Teardown complete!")
    return 0
