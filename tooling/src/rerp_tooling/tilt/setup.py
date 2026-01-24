"""Tilt-only setup: create dirs, docker volumes, check deps, print instructions. Replaces setup-tilt.sh."""

from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path


def run(project_root: Path) -> int:
    """Create dirs, docker volumes; check docker/tilt; print help. Returns 0 or 1."""
    for p in [
        "docker/prometheus",
        "docker/grafana/dashboards",
        "docker/grafana/datasources",
        "openapi/accounting",
        "microservices/accounting",
        "k8s/microservices",
        "k8s/data",
    ]:
        (project_root / p).mkdir(parents=True, exist_ok=True)
    for v in ["postgres_data", "redis_data", "prometheus_data", "grafana_data"]:
        if shutil.which("docker"):
            subprocess.run(["docker", "volume", "create", v], capture_output=True)
    for cmd in ["docker", "tilt"]:
        if not shutil.which(cmd):
            print(
                f"[ERROR] {cmd} is not installed. Please install it first.",
                file=sys.stderr,
            )
            return 1
    print("Setup complete! 🎉")
    print("To start: just up  or  just up-k8s  or  tilt up")
    return 0
