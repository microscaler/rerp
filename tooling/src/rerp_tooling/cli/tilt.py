"""`rerp tilt` subcommands: setup-kind-registry, setup-persistent-volumes, setup, teardown, logs."""

import sys
from pathlib import Path

from rerp_tooling.tilt.logs import run as run_logs
from rerp_tooling.tilt.setup import run as run_setup
from rerp_tooling.tilt.setup_kind_registry import run as run_setup_kind_registry
from rerp_tooling.tilt.setup_persistent_volumes import (
    run as run_setup_persistent_volumes,
)
from rerp_tooling.tilt.teardown import run as run_teardown


def run_tilt(args, project_root: Path) -> None:
    t = getattr(args, "tilt_cmd", None)
    if t == "setup-kind-registry":
        sys.exit(run_setup_kind_registry(project_root))
    if t == "setup-persistent-volumes":
        sys.exit(run_setup_persistent_volumes(project_root))
    if t == "setup":
        sys.exit(run_setup(project_root))
    if t == "teardown":
        sys.exit(
            run_teardown(
                project_root,
                remove_images=getattr(args, "remove_images", False),
                remove_volumes=getattr(args, "remove_volumes", False),
                system_prune=getattr(args, "system_prune", False),
            )
        )
    if t == "logs":
        sys.exit(run_logs(args.component, project_root))
    sys.exit(1)
