"""RERP: delegate to brrtrouter_tooling.tilt.teardown with RERP service names and naming."""

from pathlib import Path

from brrtrouter_tooling.tilt.teardown import run as _run

from rerp_tooling.discovery import tilt_service_names

STATIC_CONTAINERS = ["postgres-dev", "redis-dev", "prometheus-dev", "grafana-dev"]
VOLUME_NAMES = ["postgres_data", "redis_data", "prometheus_data", "grafana_data"]


def _container_name(s: str) -> str:
    return f"rerp-{s}-dev"


def _image_rmi_list(s: str) -> list[str]:
    return [
        f"rerp-accounting-{s}:latest",
        f"localhost:5001/rerp-accounting-{s}:tilt",
    ]


def run(
    project_root: Path,
    remove_images: bool = False,
    remove_volumes: bool = False,
    system_prune: bool = False,
) -> int:
    return _run(
        project_root,
        tilt_service_names(project_root),
        container_name_fn=_container_name,
        image_rmi_list_fn=_image_rmi_list,
        static_containers=STATIC_CONTAINERS,
        volume_names=VOLUME_NAMES,
        remove_images=remove_images,
        remove_volumes=remove_volumes,
        system_prune=system_prune,
    )
