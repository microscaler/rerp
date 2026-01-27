"""Pre-commit hook logic: microservices-fmt (cargo fmt when microservices/ changes)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


def _run(cmd: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        cmd,
        cwd=cwd,
        capture_output=True,
        text=True,
    )


def run_microservices_fmt(project_root: Path) -> int:
    """If microservices/ has changed vs HEAD, run cargo fmt in microservices and entities.
    If fmt changes any file, exit 1 and ask user to add and recommit.
    """
    # Any change under microservices/ (vs HEAD) triggers fmt
    r = _run(
        ["git", "diff", "--name-only", "HEAD", "--", "microservices/"],
        cwd=project_root,
    )
    if r.returncode != 0:
        print("git diff failed; skipping microservices-fmt", file=sys.stderr)
        return 0
    if not r.stdout.strip():
        return 0

    # Run cargo fmt in microservices and entities (just fmt-rust)
    r = _run(["just", "fmt-rust"], cwd=project_root)
    if r.returncode != 0:
        print(f"just fmt-rust failed: {r.stderr}", file=sys.stderr)
        return 1

    # If fmt changed anything, fail so user can add and recommit
    for d in ("microservices/", "entities/"):
        r = _run(["git", "diff", "--exit-code", "--", d], cwd=project_root)
        if r.returncode != 0:
            print(
                f"cargo fmt changed {d}. Please run: git add microservices entities && git commit",
                file=sys.stderr,
            )
            return 1
    return 0


def run_pre_commit(args: object, project_root: Path) -> None:
    pc_cmd = getattr(args, "pre_commit_cmd", None)
    if pc_cmd == "microservices-fmt":
        sys.exit(run_microservices_fmt(project_root))
    print("rerp pre-commit: use subcommand 'microservices-fmt'", file=sys.stderr)
    sys.exit(1)
