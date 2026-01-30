"""`rerp docker` subcommands: delegate to brrtrouter_tooling.docker with RERP-specific config."""

import sys
from pathlib import Path

from brrtrouter_tooling.docker.build_base import run as run_build_base_brt
from brrtrouter_tooling.docker.build_image_simple import run as run_build_image_simple_brt
from brrtrouter_tooling.docker.build_multiarch import run as run_build_multiarch_brt
from brrtrouter_tooling.docker.copy_artifacts import run as run_copy_artifacts_brt
from brrtrouter_tooling.docker.copy_artifacts import (
    validate_build_artifacts as validate_build_artifacts_brt,
)
from brrtrouter_tooling.docker.copy_binary import run as run_copy_binary_brt
from brrtrouter_tooling.docker.copy_multiarch import run as run_copy_multiarch_brt
from brrtrouter_tooling.docker.generate_dockerfile import run as run_generate_dockerfile_brt
from brrtrouter_tooling.docker.unpack_build_bins import run as run_unpack_build_bins_brt

from rerp_tooling.build.constants import PACKAGE_NAMES
from rerp_tooling.docker.copy_artifacts import BINARY_NAMES


def run_docker(args, project_root: Path) -> None:
    if args.docker_cmd == "generate-dockerfile":
        run_generate_dockerfile_brt(
            args.system,
            args.module,
            port=getattr(args, "port", 8000),
            project_root=project_root,
        )
        sys.exit(0)
    if args.docker_cmd == "unpack-build-bins":
        inp = Path(args.input_dir)
        inp = (project_root / inp) if not inp.is_absolute() else inp
        rc = run_unpack_build_bins_brt(inp, project_root)
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
        rc = run_build_base_brt(
            project_root, push=args.push, dry_run=args.dry_run, base_image_name="rerp-base"
        )
        sys.exit(rc)
    if args.docker_cmd == "copy-binary":
        rc = run_copy_binary_brt(
            Path(args.source),
            Path(args.dest),
            args.binary_name,
            project_root,
        )
        sys.exit(rc)
    if args.docker_cmd == "build-image-simple":
        dockerfile = (
            Path(args.dockerfile) if hasattr(args, "dockerfile") and args.dockerfile else None
        )
        system = getattr(args, "system", None)
        module = getattr(args, "module", None)
        port = getattr(args, "port", None)
        binary_name = getattr(args, "binary_name", None)

        rc = run_build_image_simple_brt(
            args.image_name,
            Path(args.hash_path),
            Path(args.artifact_path),
            project_root,
            system=system,
            module=module,
            port=port,
            binary_name=binary_name,
            dockerfile=dockerfile,
            base_image_name="rerp-base",
            kind_cluster_name="rerp",
        )
        sys.exit(rc)
    if args.docker_cmd == "copy-multiarch":
        rc = run_copy_multiarch_brt(
            args.system,
            args.module,
            getattr(args, "arch", "all"),
            project_root,
        )
        sys.exit(rc)
    if args.docker_cmd == "build-multiarch":
        build_cmd = [
            "tooling/.venv/bin/rerp",
            "build",
            f"{args.system}_{args.module}",
            "all",
        ]
        rc = run_build_multiarch_brt(
            args.system,
            args.module,
            args.image_name,
            getattr(args, "tag", "latest"),
            getattr(args, "push", False),
            project_root,
            build_cmd=build_cmd,
            base_image_name="rerp-base",
        )
        sys.exit(rc)
    sys.exit(1)
