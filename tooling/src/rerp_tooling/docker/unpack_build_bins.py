"""Extract rerp-binaries-*.zip from the Multi-Arch job into microservices/target. Replaces manual unzip for local container builds."""

from __future__ import annotations

import sys
import zipfile
from pathlib import Path

# Zips from Build Multi-Arch: rerp-binaries-{amd64,arm64,arm7}.zip
# Each contains {triple}/release/rerp_*_impl (triple = x86_64-unknown-linux-musl, aarch64-unknown-linux-musl, armv7-unknown-linux-musleabihf)
# We extract into microservices/target/ so copy-multiarch and build-multiarch find them.


def run(input_dir: Path, project_root: Path) -> int:
    """Extract rerp-binaries-*.zip from input_dir into project_root/microservices/target. Returns 0 or 1."""
    if not input_dir.is_dir():
        print(f"❌ Input directory not found: {input_dir}", file=sys.stderr)
        return 1

    dest = project_root / "microservices" / "target"
    dest.mkdir(parents=True, exist_ok=True)

    # Artifact names from ci.yml: rerp-binaries-amd64, rerp-binaries-arm64, rerp-binaries-arm7
    zips = [
        input_dir / "rerp-binaries-amd64.zip",
        input_dir / "rerp-binaries-arm64.zip",
        input_dir / "rerp-binaries-arm7.zip",
    ]
    found = [z for z in zips if z.exists()]
    if not found:
        print(
            f"❌ No rerp-binaries-*.zip in {input_dir}. Expected: rerp-binaries-amd64.zip, rerp-binaries-arm64.zip, rerp-binaries-arm7.zip",
            file=sys.stderr,
        )
        return 1

    for z in found:
        count = 0
        with zipfile.ZipFile(z, "r") as zh:
            for name in zh.namelist():
                if "/" not in name or name.startswith("/") or ".." in name:
                    continue
                zh.extract(name, dest)
                count += 1
                if name.endswith("_impl") and not name.endswith(".d"):
                    (dest / name).chmod(0o755)
        print(f"📦 Extracted {z.name} -> {dest} ({count} entries)")

    print(f"✅ Unpacked into {dest.relative_to(project_root)}")
    print(
        "   Next: rerp docker copy-multiarch <system> <module> all  (then build-multiarch or your Docker workflow)"
    )
    return 0
