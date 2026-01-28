"""Build Docker image and push (or kind load). Replaces build-microservice-docker-simple.sh."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path
from typing import Optional


def run(
    image_name: str,
    hash_path: Path,
    artifact_path: Path,
    project_root: Path,
    system: Optional[str] = None,
    module: Optional[str] = None,
    port: Optional[int] = None,
    binary_name: Optional[str] = None,
    dockerfile: Optional[Path] = None,
) -> int:
    """Build Docker image using template or static Dockerfile.

    If system/module/port/binary_name are provided, generates Dockerfile from template dynamically.
    Otherwise, uses the provided dockerfile path (backward compatibility).

    Returns 0 on success, 1 on error.
    """
    root = project_root
    h = root / hash_path if not hash_path.is_absolute() else hash_path
    a = root / artifact_path if not artifact_path.is_absolute() else artifact_path

    if not h.exists():
        print(f"❌ Hash file not found: {h}", file=sys.stderr)
        print("   This indicates copy script has not completed yet", file=sys.stderr)
        return 1
    if not a.exists():
        print(f"❌ Artifact not found: {a}", file=sys.stderr)
        return 1

    # Use template with build args if parameters provided, otherwise use static file
    use_template = (
        system is not None and module is not None and port is not None and binary_name is not None
    )

    if use_template:
        # Use template directly with build arguments
        template_path = root / "docker" / "microservices" / "Dockerfile.template"
        if not template_path.exists():
            print(f"❌ Template not found: {template_path}", file=sys.stderr)
            return 1

        # Check if base image exists locally or in GHCR
        # Try local first (rerp-base:latest), then GHCR (ghcr.io/microscaler/rerp-base:latest)
        import os

        owner = (
            os.environ.get("GHCR_OWNER")
            or os.environ.get("GITHUB_REPOSITORY_OWNER")
            or "microscaler"
        )
        base_image_local = "rerp-base:latest"
        base_image_ghcr = f"ghcr.io/{owner}/rerp-base:latest"

        check_local = subprocess.run(
            ["docker", "images", "-q", base_image_local],
            capture_output=True,
            text=True,
            cwd=str(root),
        )
        check_ghcr = subprocess.run(
            ["docker", "images", "-q", base_image_ghcr],
            capture_output=True,
            text=True,
            cwd=str(root),
        )

        if not (check_local.stdout and check_local.stdout.strip()) and not (
            check_ghcr.stdout and check_ghcr.stdout.strip()
        ):
            print(f"📦 Base image {base_image_local} or {base_image_ghcr} not found")
            print(f"   Attempting to pull from GHCR: {base_image_ghcr}")
            pull_result = subprocess.run(
                ["docker", "pull", base_image_ghcr],
                capture_output=True,
                text=True,
                cwd=str(root),
            )
            if pull_result.returncode != 0:
                print(f"   Pull failed, building locally as {base_image_local}...")
                from rerp_tooling.docker.build_base import run as run_build_base

                if run_build_base(root, push=False, dry_run=False) != 0:
                    print("❌ Failed to build base image", file=sys.stderr)
                    return 1
                # Tag the local build with the GHCR name for consistency
                tag_result = subprocess.run(
                    ["docker", "tag", base_image_local, base_image_ghcr],
                    capture_output=True,
                    text=True,
                    cwd=str(root),
                )
                if tag_result.returncode == 0:
                    print(f"   Tagged local image as {base_image_ghcr}")

        dockerfile_path = template_path
        build_args = [
            "--build-arg",
            f"SYSTEM={system}",
            "--build-arg",
            f"MODULE={module}",
            "--build-arg",
            f"PORT={port}",
            "--build-arg",
            f"BINARY_NAME={binary_name}",
        ]
    else:
        # Use provided static Dockerfile (backward compatibility)
        if dockerfile is None:
            print(
                "❌ Either provide (system, module, port, binary_name) or dockerfile path",
                file=sys.stderr,
            )
            return 1
        d = root / dockerfile if not dockerfile.is_absolute() else dockerfile
        if not d.exists():
            print(f"❌ Dockerfile not found: {d}", file=sys.stderr)
            return 1
        dockerfile_path = d
        build_args = []

    tag = f"{image_name}:tilt"
    # Note: docker build uses local images by default if available
    # We don't use --pull flag, so it won't try to pull from registry
    build_cmd = [
        "docker",
        "build",
        "-t",
        tag,
        "--rm",
        "--force-rm",
        "-f",
        str(dockerfile_path),
        *build_args,
        ".",
    ]
    build = subprocess.run(
        build_cmd,
        cwd=str(root),
    )

    if build.returncode != 0:
        print("❌ Docker build failed", file=sys.stderr)
        return 1

    push = subprocess.run(["docker", "push", tag], cwd=str(root), capture_output=True, text=True)
    if push.returncode == 0:
        print(f"✅ Docker image pushed to registry: {tag}")
    else:
        print("⚠️  Registry not available at localhost:5001; loading into Kind cluster...")
        kind = subprocess.run(
            ["kind", "load", "docker-image", tag, "--name", "rerp"],
            cwd=str(root),
            capture_output=True,
            text=True,
        )
        if kind.returncode == 0:
            print(f"✅ Image loaded into Kind: {tag}")
        else:
            print(f"⚠️  Could not push or kind load; image tagged as: {tag}")
    print(f"✅ Docker image ready: {tag}")
    return 0
