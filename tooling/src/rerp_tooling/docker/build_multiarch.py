"""Build and push multi-architecture Docker images for a component. Replaces build-multiarch-docker.sh."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ARCH_PLATFORMS = {
    "amd64": "linux/amd64",
    "arm64": "linux/arm64",
    "arm7": "linux/arm/v7",
}


def run(
    system: str,
    module: str,
    image_name: str,
    tag: str,
    push: bool,
    project_root: Path,
) -> int:
    """Build binaries, copy, generate Dockerfile if needed, buildx base+images, manifest, optional push. Returns 0 or 1."""
    root = project_root
    dockerfile = root / "docker" / "microservices" / f"Dockerfile.{system}_{module}"

    # 1. Build for all archs
    print("🔨 Building binaries for all architectures...")
    r = subprocess.run(
        ["tooling/.venv/bin/rerp", "build", f"{system}_{module}", "all"],
        cwd=str(root),
    )
    if r.returncode != 0:
        print("❌ rerp build failed", file=sys.stderr)
        return 1

    # 2. Copy binaries
    from rerp_tooling.docker.copy_multiarch import run as copy_run

    if copy_run(system, module, "all", root) != 0:
        return 1

    # 3. Generate Dockerfile if missing
    if not dockerfile.exists():
        print("📝 Generating Dockerfile...")
        r = subprocess.run(
            [
                "tooling/.venv/bin/rerp",
                "docker",
                "generate-dockerfile",
                system,
                module,
                "--port",
                "8000",
            ],
            cwd=str(root),
        )
        if r.returncode != 0:
            return 1

    # 4. Build base images per arch if needed
    print("🔨 Building base images for all architectures...")
    for arch in ["amd64", "arm64", "arm7"]:
        platform = ARCH_PLATFORMS[arch]
        base_img = f"rerp/base:{arch}"
        check = subprocess.run(
            ["docker", "images", "-q", base_img],
            capture_output=True,
            text=True,
            cwd=str(root),
        )
        if not (check.stdout and check.stdout.strip()):
            print(f"  Building base image for {arch}...")
            subprocess.run(
                [
                    "docker",
                    "buildx",
                    "build",
                    "--platform",
                    platform,
                    "--tag",
                    base_img,
                    "--load",
                    "-f",
                    "docker/base/Dockerfile",
                    ".",
                ],
                cwd=str(root),
                check=True,
            )

    # 5. Build image per arch
    print("🔨 Building Docker images for all architectures...")
    image_tags = []
    df_content = dockerfile.read_text()

    for arch in ["amd64", "arm64", "arm7"]:
        platform = ARCH_PLATFORMS[arch]
        arch_tag = f"{image_name}:{tag}-{arch}"
        image_tags.append(arch_tag)

        # Per-arch Dockerfile: FROM rerp/base:arch, COPY from build_artifacts/{system}_{module}/{arch}/
        mod = df_content.replace("rerp/base:latest", f"rerp/base:{arch}")
        mod = mod.replace(
            "./build_artifacts/${TARGETARCH}/",
            f"./build_artifacts/{system}_{module}/{arch}/",
        )
        arch_df = root / "docker" / "microservices" / f"Dockerfile.{system}_{module}.{arch}"
        arch_df.write_text(mod)
        try:
            r = subprocess.run(
                [
                    "docker",
                    "buildx",
                    "build",
                    "--platform",
                    platform,
                    "--tag",
                    arch_tag,
                    "--file",
                    str(arch_df),
                    "--load",
                    ".",
                ],
                cwd=str(root),
            )
            if r.returncode != 0:
                print(f"❌ Docker build failed for {arch}", file=sys.stderr)
                arch_df.unlink(missing_ok=True)
                return 1
            print(f"  ✅ Built: {arch_tag}")
        finally:
            arch_df.unlink(missing_ok=True)

    # 6. Manifest
    manifest_tag = f"{image_name}:{tag}"
    print("🔗 Creating multi-architecture manifest...")
    r = subprocess.run(["docker", "manifest", "create", manifest_tag, *image_tags], cwd=str(root))
    if r.returncode != 0:
        print("❌ docker manifest create failed", file=sys.stderr)
        return 1

    for arch in ["amd64", "arm64", "arm7"]:
        platform = ARCH_PLATFORMS[arch]
        arch_tag = f"{image_name}:{tag}-{arch}"
        pl = platform.split("/")
        subprocess.run(
            [
                "docker",
                "manifest",
                "annotate",
                "--arch",
                pl[1] if len(pl) > 1 else arch,
                "--os",
                pl[0],
                manifest_tag,
                arch_tag,
            ],
            cwd=str(root),
            capture_output=True,
        )

    print(f"✅ Multi-architecture manifest created: {manifest_tag}")

    # 7. Push if requested
    if push:
        print("📤 Pushing images...")
        for t in image_tags:
            subprocess.run(["docker", "push", t], cwd=str(root), check=True)
        subprocess.run(["docker", "manifest", "push", manifest_tag], cwd=str(root), check=True)
        print("✅ All images pushed")
    else:
        print("Info:  Images built locally. Use --push to push.")

    print("🎉 Multi-architecture build complete!")
    return 0
