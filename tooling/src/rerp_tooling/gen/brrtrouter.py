"""Shared utilities for calling BRRTRouter commands."""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Optional


def find_brrtrouter(
    project_root: Path, brrtrouter_path: Optional[Path] = None
) -> tuple[Path, Path]:
    """Find BRRTRouter binary and manifest paths.

    Returns (brrtrouter_bin_path, manifest_path)
    Raises FileNotFoundError if BRRTRouter not found.
    """
    if brrtrouter_path is None:
        brrtrouter_path = project_root.parent / "BRRTRouter"

    brrtrouter_bin = brrtrouter_path / "target" / "debug" / "brrtrouter-gen"
    manifest = brrtrouter_path / "Cargo.toml"

    if not manifest.exists():
        msg = f"BRRTRouter not found at {brrtrouter_path}"
        raise FileNotFoundError(msg)

    return brrtrouter_bin, manifest


def call_brrtrouter_generate(
    spec_path: Path,
    output_dir: Path,
    project_root: Path,
    brrtrouter_path: Optional[Path] = None,
    deps_config_path: Optional[Path] = None,
    capture_output: bool = False,
) -> subprocess.CompletedProcess:
    """Call BRRTRouter's generate command to generate gen crate.

    Args:
        spec_path: Path to OpenAPI spec file
        output_dir: Output directory for gen crate
        project_root: RERP project root
        brrtrouter_path: Optional path to BRRTRouter (defaults to ../BRRTRouter)
        deps_config_path: Optional path to brrtrouter-dependencies.toml
        capture_output: Whether to capture stdout/stderr

    Returns:
        CompletedProcess from subprocess.run
    """
    brrtrouter_bin, manifest = find_brrtrouter(project_root, brrtrouter_path)

    deps_config_arg = []
    if deps_config_path and deps_config_path.exists():
        deps_config_arg = ["--dependencies-config", str(deps_config_path)]

    if brrtrouter_bin.exists():
        cmd = [
            str(brrtrouter_bin),
            "generate",
            "--spec",
            str(spec_path),
            "--output",
            str(output_dir),
            "--force",
            *deps_config_arg,
        ]
    else:
        cmd = [
            "cargo",
            "run",
            "--manifest-path",
            str(manifest),
            "--bin",
            "brrtrouter-gen",
            "--",
            "generate",
            "--spec",
            str(spec_path),
            "--output",
            str(output_dir),
            "--force",
            *deps_config_arg,
        ]

    return subprocess.run(
        cmd,
        check=False,  # Let caller handle errors
        cwd=str(project_root),
        capture_output=capture_output,
        text=True,
    )


def call_brrtrouter_generate_stubs(
    spec_path: Path,
    impl_dir: Path,
    component_name: str,
    project_root: Path,
    brrtrouter_path: Optional[Path] = None,
    force: bool = False,
    capture_output: bool = False,
) -> subprocess.CompletedProcess:
    """Call BRRTRouter's generate-stubs command to generate impl crate.

    Args:
        spec_path: Path to OpenAPI spec file
        impl_dir: Output directory for impl crate
        component_name: Component name (gen crate name) for --component-name
        project_root: RERP project root
        brrtrouter_path: Optional path to BRRTRouter (defaults to ../BRRTRouter)
        force: Whether to force overwrite existing stubs
        capture_output: Whether to capture stdout/stderr

    Returns:
        CompletedProcess from subprocess.run
    """
    brrtrouter_bin, manifest = find_brrtrouter(project_root, brrtrouter_path)

    if brrtrouter_bin.exists():
        cmd = [
            str(brrtrouter_bin),
            "generate-stubs",
            "--spec",
            str(spec_path),
            "--output",
            str(impl_dir),
            "--component-name",
            component_name,
        ]
        if force:
            cmd.append("--force")
    else:
        cmd = [
            "cargo",
            "run",
            "--manifest-path",
            str(manifest),
            "--bin",
            "brrtrouter-gen",
            "--",
            "generate-stubs",
            "--spec",
            str(spec_path),
            "--output",
            str(impl_dir),
            "--component-name",
            component_name,
        ]
        if force:
            cmd.append("--force")

    return subprocess.run(
        cmd,
        check=False,  # Let caller handle errors
        cwd=str(project_root),
        capture_output=capture_output,
        text=True,
    )
