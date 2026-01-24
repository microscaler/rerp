"""Setup PersistentVolumes for RERP (k8s/data, k8s/monitoring). Replaces setup-persistent-volumes.sh."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


def run(project_root: Path) -> int:
    """Apply k8s/data/persistent-volumes.yaml and k8s/monitoring/persistent-volumes.yaml. Returns 0 or 1."""
    if subprocess.run(["kubectl", "cluster-info"], capture_output=True).returncode != 0:
        print("❌ Error: Cannot connect to Kubernetes cluster", file=sys.stderr)
        print(
            "   Please ensure your Kind cluster is running: kind get clusters",
            file=sys.stderr,
        )
        return 1
    for label, path in [
        ("data", project_root / "k8s" / "data" / "persistent-volumes.yaml"),
        ("monitoring", project_root / "k8s" / "monitoring" / "persistent-volumes.yaml"),
    ]:
        if path.exists():
            print(f"📦 Creating {label} PersistentVolumes...")
            r = subprocess.run(
                ["kubectl", "apply", "-f", str(path)], capture_output=True, text=True
            )
            if r.returncode != 0 and "AlreadyExists" not in (r.stderr or ""):
                print(f"⚠️  Warning: Some {label} PVs may already exist (this is OK)")
        else:
            print(f"Info:  No {label} PersistentVolumes file found (this is OK for initial setup)")
    print("✅ PersistentVolumes setup complete!")
    r = subprocess.run(["kubectl", "get", "pv"], capture_output=True, text=True)
    if r.returncode == 0:
        print(r.stdout or "No PersistentVolumes found")
    return 0
