"""`rerp docker` subcommands: generate-dockerfile, copy-artifacts, copy-binary, build-image-simple, copy-multiarch, build-multiarch, build-base."""

import sys
from pathlib import Path

from brrtrouter_tooling.docker.copy_artifacts import (
    run as run_copy_artifacts_brt,
)
from brrtrouter_tooling.docker.copy_artifacts import (
    validate_build_artifacts as validate_build_artifacts_brt,
)

from rerp_tooling.build.constants import PACKAGE_NAMES
from rerp_tooling.docker.build_base import run as run_build_base
from rerp_tooling.docker.build_image_simple import run as run_build_image_simple
from rerp_tooling.docker.build_multiarch import run as run_build_multiarch
from rerp_tooling.docker.copy_artifacts import BINARY_NAMES
from rerp_tooling.docker.copy_binary import run as run_copy_binary
from rerp_tooling.docker.copy_multiarch import run as run_copy_multiarch
from rerp_tooling.docker.generate_dockerfile import run as run_generate_dockerfile
from rerp_tooling.docker.unpack_build_bins import run as run_unpack_build_bins


def run_docker(args, project_root: Path) -> None:
    if args.docker_cmd == "generate-dockerfile":
        run_generate_dockerfile(
            args.system,
            args.module,
            port=getattr(args, "port", 8000),
            project_root=project_root,
        )
        sys.exit(0)
    if args.docker_cmd == "unpack-build-bins":
        inp = Path(args.input_dir)
        inp = (project_root / inp) if not inp.is_absolute() else inp
        rc = run_unpack_build_bins(inp, project_root)
        sys.exit(rc)
    if args.docker_cmd == "validate-build-artifacts":
        rc = validate_build_artifacts_brt(project_root, BINARY_NAMES)
        sys.exit(rc)
    if args.docker_cmd == "copy-artifacts":
        rc = run_copy_artifacts_brt(
            args.arch,
            project_root,
            package_names=PACKAGE_NAMES,
            binary_names=BINARY_NAMES,
        )
        sys.exit(rc)
    if args.docker_cmd == "build-base":
        rc = run_build_base(project_root, push=args.push, dry_run=args.dry_run)
        sys.exit(rc)
    if args.docker_cmd == "copy-binary":
        rc = run_copy_binary(
            Path(args.source),
            Path(args.dest),
            args.binary_name,
            project_root,
        )
        sys.exit(rc)
    if args.docker_cmd == "build-image-simple":
        rc = run_build_image_simple(
            args.image_name,
            Path(args.dockerfile),
            Path(args.hash_path),
            Path(args.artifact_path),
            project_root,
        )
        sys.exit(rc)
    if args.docker_cmd == "copy-multiarch":
        rc = run_copy_multiarch(
            args.system,
            args.module,
            getattr(args, "arch", "all"),
            project_root,
        )
        sys.exit(rc)
    if args.docker_cmd == "build-multiarch":
        rc = run_build_multiarch(
            args.system,
            args.module,
            args.image_name,
            getattr(args, "tag", "latest"),
            getattr(args, "push", False),
            project_root,
        )
        sys.exit(rc)
    sys.exit(1)
