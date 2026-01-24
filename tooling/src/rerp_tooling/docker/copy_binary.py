"""Copy a binary to dest and write SHA256 to dest.sha256. Replaces copy-microservice-binary-simple.sh."""

from __future__ import annotations

import hashlib
import shutil
import sys
from pathlib import Path


def run(source: Path, dest: Path, binary_name: str, project_root: Path) -> int:
    """Copy source to dest, chmod +x, write {dest}.sha256. Returns 0 or 1."""
    src = project_root / source if not source.is_absolute() else source
    dst = project_root / dest if not dest.is_absolute() else dest
    if not src.exists():
        print(f"❌ Binary not found: {src}", file=sys.stderr)
        print("   Build the service first", file=sys.stderr)
        return 1
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)
    dst.chmod(0o755)
    h = hashlib.sha256(dst.read_bytes()).hexdigest()
    hash_path = Path(str(dst) + ".sha256")
    hash_path.write_text(h)
    print(f"📦 Copying binary: {src.name} -> {dst.relative_to(project_root)}")
    print(f"✅ Binary copied and hash generated: {hash_path}")
    return 0
