"""Pre-commit hook logic: delegate to brrtrouter_tooling.pre_commit (RERP: just fmt-rust, entities)."""

from __future__ import annotations

import sys
from pathlib import Path

from brrtrouter_tooling.pre_commit import run_workspace_fmt


def run_microservices_fmt(project_root: Path) -> int:
    """If microservices/ has changed vs HEAD, run just fmt-rust; check microservices/ and entities/."""
    return run_workspace_fmt(
        project_root,
        workspace_dir="microservices",
        fmt_argv=["just", "fmt-rust"],
        extra_check_dirs=["microservices/", "entities/"],
    )


def run_pre_commit(args: object, project_root: Path) -> None:
    pc_cmd = getattr(args, "pre_commit_cmd", None)
    if pc_cmd == "microservices-fmt":
        sys.exit(run_microservices_fmt(project_root))
    print("rerp pre-commit: use subcommand 'microservices-fmt'", file=sys.stderr)
    sys.exit(1)
