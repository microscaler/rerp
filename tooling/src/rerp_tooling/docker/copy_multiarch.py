"""Copy component binaries for all or one arch to build_artifacts/{system}_{module}/{arch}. Replaces copy-multiarch-binary.sh."""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path

from rerp_tooling.build.host_aware import ARCH_TARGETS


def _binary_name(system: str, module: str) -> str:
    return f"rerp_{system}_{module.replace('-', '_')}_impl"


def run(system: str, module: str, arch: str, project_root: Path) -> int:
    """Copy from microservices/target/{triple}/release/ to build_artifacts/{system}_{module}/{arch}. Returns 0 or 1."""
    if arch == "all":
        archs = ["amd64", "arm64", "arm7"]
    elif arch in ARCH_TARGETS:
        archs = [arch]
    else:
        print(
            f"❌ Unknown architecture: {arch}. Use amd64, arm64, arm7, or all.",
            file=sys.stderr,
        )
        return 1

    root = project_root
    binary_name = _binary_name(system, module)
    microservices_target = root / "microservices" / "target"
    base_dest = root / "build_artifacts" / f"{system}_{module}"
    any_ok = False

    for a in archs:
        triple = ARCH_TARGETS[a]
        src = microservices_target / triple / "release" / binary_name
        dest_dir = base_dest / a
        dest_bin = dest_dir / binary_name
        hash_path = dest_dir / f"{binary_name}.sha256"

        if not src.exists():
            print(f"❌ Binary not found: {src}", file=sys.stderr)
            print(
                f"   Build first: tooling/.venv/bin/rerp build {system}_{module} {a}",
                file=sys.stderr,
            )
            continue

        dest_dir.mkdir(parents=True, exist_ok=True)
        dest_bin.write_bytes(src.read_bytes())
        dest_bin.chmod(0o755)
        hash_path.write_text(hashlib.sha256(dest_bin.read_bytes()).hexdigest())
        print(f"✅ {a} binary copied and hash generated: {hash_path.relative_to(root)}")
        any_ok = True

    if not any_ok:
        return 1
    print("🎉 All requested binaries copied!")
    return 0
