"""`rerp bootstrap microservice` — Bootstrap a microservice from OpenAPI."""

import sys
from pathlib import Path

from rerp_tooling.bootstrap.microservice import run_bootstrap_microservice


def run_bootstrap(args, project_root: Path) -> None:
    service_name = args.service_name
    port = getattr(args, "port", None)
    rc = run_bootstrap_microservice(service_name, port, project_root)
    sys.exit(rc)
