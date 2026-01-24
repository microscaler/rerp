"""Build rerp-base from docker/base/Dockerfile. Local counterpart to base-images.yml; --push requires login."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


def run(project_root: Path, push: bool = False, dry_run: bool = False) -> int:
    """Build docker/base/Dockerfile as rerp-base:latest. With --push, also push to ghcr.io/$OWNER/rerp-base:latest. Returns 0 or 1."""
    dockerfile = project_root / "docker" / "base" / "Dockerfile"
    if not dockerfile.exists():
        print(f"❌ {dockerfile} not found", file=sys.stderr)
        return 1
    owner = os.environ.get("GHCR_OWNER") or os.environ.get("GITHUB_REPOSITORY_OWNER")
    if push and not owner:
        print("❌ For --push, set GHCR_OWNER or GITHUB_REPOSITORY_OWNER", file=sys.stderr)
        return 1
    tag_local = "rerp-base:latest"
    tag_remote = f"ghcr.io/{owner}/rerp-base:latest" if owner else None
    if dry_run:
        if push and tag_remote:
            print(
                f"[dry-run] would: docker buildx build -f {dockerfile} -t {tag_local} -t {tag_remote} --platform linux/amd64,linux/arm64 --push ."
            )
        else:
            print(f"[dry-run] would: docker build -f {dockerfile} -t {tag_local} .")
        return 0
    if push and tag_remote:
        cmd = [
            "docker",
            "buildx",
            "build",
            "-f",
            str(dockerfile),
            "-t",
            tag_local,
            "-t",
            tag_remote,
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
            "-t",
            tag_local,
            str(project_root),
        ]
    r = subprocess.run(cmd, cwd=str(project_root))
    if r.returncode != 0:
        return 1
    print(f"✅ Built {tag_local}" + (f", pushed {tag_remote}" if push and tag_remote else ""))
    return 0
