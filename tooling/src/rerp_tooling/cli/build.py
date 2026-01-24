"""`rerp build` (host-aware cargo/cross; microservices/ workspace)."""

import sys
from pathlib import Path

from rerp_tooling.build.host_aware import run as run_host_aware
from rerp_tooling.build.microservices import (
    build_microservice,
    build_microservices_workspace,
)


def run_build(args, project_root: Path) -> None:
    target = args.target
    arch = getattr(args, "arch", None)
    release = getattr(args, "release", False)

    if target == "microservices":
        rc = build_microservices_workspace(project_root, arch=arch or "amd64", release=release)
        sys.exit(rc)
    if target == "microservice":
        if not arch:
            print(
                "rerp build microservice: missing service name (e.g. general-ledger)",
                file=sys.stderr,
            )
            sys.exit(2)
        rc = build_microservice(project_root, name=arch, release=release)
        sys.exit(rc)

    rc = run_host_aware(target=target, arch=arch, extra_args=None, project_root=project_root)
    sys.exit(rc)
