"""RERP: delegate to brrtrouter_tooling.tilt.setup with RERP dirs and volumes."""

from pathlib import Path

from brrtrouter_tooling.tilt.setup import run as _run

RERP_DIRS = [
    "docker/prometheus",
    "docker/grafana/dashboards",
    "docker/grafana/datasources",
    "openapi/accounting",
    "microservices/accounting",
    "k8s/microservices",
    "k8s/data",
]
RERP_VOLUMES = ["postgres_data", "redis_data", "prometheus_data", "grafana_data"]


def run(project_root: Path) -> int:
    return _run(project_root, dirs=RERP_DIRS, volumes=RERP_VOLUMES)
