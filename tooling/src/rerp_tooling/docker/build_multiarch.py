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
    """Build binaries, copy, generate Dockerfile from template, buildx base+images, manifest, optional push. Returns 0 or 1."""
    root = project_root

    # Use template with build args - no need to generate content, use template directly
    template_path = root / "docker" / "microservices" / "Dockerfile.template"
    if not template_path.exists():
        print(f"❌ Template not found: {template_path}", file=sys.stderr)
        return 1

    # Determine port (default to 8000, could be enhanced to read from port registry)
    port = 8000
    binary_name = f"rerp_{system}_{module.replace('-', '_')}_impl"

    # Build args for template
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

    # 4. Build base images per arch if needed
    import os

    owner = (
        os.environ.get("GHCR_OWNER") or os.environ.get("GITHUB_REPOSITORY_OWNER") or "microscaler"
    )

    print("🔨 Building base images for all architectures...")
    for arch in ["amd64", "arm64", "arm7"]:
        platform = ARCH_PLATFORMS[arch]
        base_img_ghcr = f"ghcr.io/{owner}/rerp-base:{arch}"
        base_img_local = f"rerp-base:{arch}"

        # Check both local and GHCR tags
        check_ghcr = subprocess.run(
            ["docker", "images", "-q", base_img_ghcr],
            capture_output=True,
            text=True,
            cwd=str(root),
        )
        check_local = subprocess.run(
            ["docker", "images", "-q", base_img_local],
            capture_output=True,
            text=True,
            cwd=str(root),
        )

        if not (check_ghcr.stdout and check_ghcr.stdout.strip()) and not (
            check_local.stdout and check_local.stdout.strip()
        ):
            print(f"  Pulling base image for {arch} from GHCR: {base_img_ghcr}")
            pull_result = subprocess.run(
                ["docker", "pull", base_img_ghcr],
                capture_output=True,
                text=True,
                cwd=str(root),
            )
            if pull_result.returncode != 0:
                print(f"  Pull failed, building base image for {arch} locally...")
                subprocess.run(
                    [
                        "docker",
                        "buildx",
                        "build",
                        "--platform",
                        platform,
                        "--tag",
                        base_img_local,
                        "--tag",
                        base_img_ghcr,
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

    for arch in ["amd64", "arm64", "arm7"]:
        platform = ARCH_PLATFORMS[arch]
        arch_tag = f"{image_name}:{tag}-{arch}"
        image_tags.append(arch_tag)

        # For multi-arch, we need to modify the base image and COPY path
        # Read template and modify for this architecture
        # The template uses ARG BASE_IMAGE=ghcr.io/microscaler/rerp-base:latest
        # We need to replace both the ARG default and ensure FROM uses the arch-specific tag
        base_image_default = f"ghcr.io/{owner}/rerp-base:latest"
        base_image_arch = f"ghcr.io/{owner}/rerp-base:{arch}"

        template_content = template_path.read_text()
        # Replace the default base image in ARG with arch-specific one
        mod = template_content.replace(
            f"ARG BASE_IMAGE={base_image_default}", f"ARG BASE_IMAGE={base_image_arch}"
        )
        mod = mod.replace(
            "./build_artifacts/${TARGETARCH}/",
            f"./build_artifacts/{system}_{module}/{arch}/",
        )
        arch_df = root / "docker" / "microservices" / f"Dockerfile.{system}_{module}.{arch}"
        arch_df.write_text(mod)
        try:
            # Use build args with the modified Dockerfile
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
                    *build_args,
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
