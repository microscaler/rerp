"""Tail Tilt logs for a component. Replaces tail-tilt-logs.sh."""

from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path


def run(component: str, project_root: Path) -> int:
    """Run tilt logs <component> --follow. Returns exit code from tilt logs."""
    if not shutil.which("tilt"):
        print("[ERROR] Tilt is not installed.", file=sys.stderr)
        return 1
    r = subprocess.run(
        ["tilt", "get", "uiresources", "--format", "json"],
        capture_output=True,
        text=True,
    )
    if r.returncode != 0:
        print("[ERROR] Tilt is not running or not connected.", file=sys.stderr)
        return 1
    if f'"name":"{component}"' not in (r.stdout or ""):
        print(
            f"[WARN] Component '{component}' not found. Run: tilt get uiresources",
            file=sys.stderr,
        )
        return 1
    return subprocess.run(["tilt", "logs", component, "--follow"]).returncode
