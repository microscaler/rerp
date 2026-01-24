"""Build Docker image and push (or kind load). Replaces build-microservice-docker-simple.sh."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


def run(
    image_name: str,
    dockerfile: Path,
    hash_path: Path,
    artifact_path: Path,
    project_root: Path,
) -> int:
    """Verify hash, artifact, dockerfile; docker build -t image:tilt; push or kind load. Returns 0 or 1."""
    root = project_root
    h, a, d = Path(hash_path), Path(artifact_path), Path(dockerfile)
    h = root / h if not h.is_absolute() else h
    a = root / a if not a.is_absolute() else a
    d = root / d if not d.is_absolute() else d
    if not h.exists():
        print(f"❌ Hash file not found: {h}", file=sys.stderr)
        print("   This indicates copy script has not completed yet", file=sys.stderr)
        return 1
    if not a.exists():
        print(f"❌ Artifact not found: {a}", file=sys.stderr)
        return 1
    if not d.exists():
        print(f"❌ Dockerfile not found: {d}", file=sys.stderr)
        return 1
    tag = f"{image_name}:tilt"
    build = subprocess.run(
        ["docker", "build", "-t", tag, "--rm", "--force-rm", "-f", str(d), "."],
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
