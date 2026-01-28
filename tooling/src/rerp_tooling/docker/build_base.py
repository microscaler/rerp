"""Build rerp-base from docker/base/Dockerfile. Local counterpart to base-images.yml; --push requires login."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

# Matches tooling/src/rerp_tooling/tilt/setup_kind_registry.REG_PORT
KIND_REGISTRY_PORT = "5001"


def run(project_root: Path, push: bool = False, dry_run: bool = False) -> int:
    """Build docker/base/Dockerfile and tag with: local (rerp-base:latest), ghcr.io, local Kind registry (localhost:5001), and optionally docker.io. With --push, push to remote registries. Returns 0 or 1."""
    dockerfile = project_root / "docker" / "base" / "Dockerfile"
    if not dockerfile.exists():
        print(f"❌ {dockerfile} not found", file=sys.stderr)
        return 1
    owner = (
        os.environ.get("GHCR_OWNER") or os.environ.get("GITHUB_REPOSITORY_OWNER") or "microscaler"
    )
    dockerhub_org = os.environ.get("DOCKERHUB_ORG") or os.environ.get("DOCKERHUB_OWNER")
    if push and (os.environ.get("GHCR_OWNER") or os.environ.get("GITHUB_REPOSITORY_OWNER")) is None:
        print("❌ For --push, set GHCR_OWNER or GITHUB_REPOSITORY_OWNER", file=sys.stderr)
        return 1
    tag_local = "rerp-base:latest"
    tag_ghcr = f"ghcr.io/{owner}/rerp-base:latest"
    tag_kind = f"localhost:{KIND_REGISTRY_PORT}/rerp-base:latest"
    tag_dockerhub = f"docker.io/{dockerhub_org}/rerp-base:latest" if dockerhub_org else None
    tags = [tag_local, tag_ghcr, tag_kind]
    if tag_dockerhub:
        tags.append(tag_dockerhub)
    if dry_run:
        tag_args = " ".join(f"-t {t}" for t in tags)
        if push:
            print(
                f"[dry-run] would: docker buildx build -f {dockerfile} {tag_args} --platform linux/amd64,linux/arm64 --push ."
            )
        else:
            print(f"[dry-run] would: docker build -f {dockerfile} {tag_args} .")
        return 0
    if push:
        cmd = [
            "docker",
            "buildx",
            "build",
            "-f",
            str(dockerfile),
            *[x for t in tags for x in ("-t", t)],
            "--platform",
            "linux/amd64,linux/arm64",
            "--push",
            str(project_root),
        ]
    else:
        cmd = [
            "docker",
            "build",
            "-f",
            str(dockerfile),
            *[x for t in tags for x in ("-t", t)],
            str(project_root),
        ]
    r = subprocess.run(cmd, cwd=str(project_root))
    if r.returncode != 0:
        return 1
    tagged = ", ".join(tags)
    pushed = ", pushed to remotes" if push else ""
    print(f"✅ Built and tagged: {tagged}{pushed}")
    return 0
