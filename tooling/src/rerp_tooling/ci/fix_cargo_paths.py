"""Fix BRRTRouter path dependencies in generated Cargo.toml files.

Updates brrtrouter and brrtrouter_macros path deps to point at the BRRTRouter
repository (sibling of project root: ../BRRTRouter). For use in local dev after
brrtrouter-gen; CI uses rerp ci patch-brrtrouter to switch to git deps instead.
"""

from __future__ import annotations

import os
import re
from pathlib import Path
from typing import Optional


def fix_cargo_toml(cargo_toml_path: Path, project_root: Optional[Path] = None) -> bool:
    """
    Fix BRRTRouter paths in a Cargo.toml file.
    Assumes Cargo.toml at microservices/accounting/{service}/Cargo.toml; project_root
    defaults to 3 levels up from the file's parent. BRRTRouter is project_root.parent / "BRRTRouter".
    Returns True if content was changed.
    """
    if not cargo_toml_path.exists():
        print(f"Warning: {cargo_toml_path} does not exist, skipping")
        return False

    content = cargo_toml_path.read_text()
    original = content

    cargo_toml_dir = cargo_toml_path.parent.resolve()
    if project_root is not None:
        root = Path(project_root).resolve()
    else:
        # microservices/accounting/{service}/ -> accounting -> microservices -> repo root
        root = cargo_toml_dir.parent.parent.parent

    brrtrouter_path = root.parent / "BRRTRouter"
    try:
        rel = Path(os.path.relpath(brrtrouter_path, cargo_toml_dir)).as_posix()
        rel_macros = Path(
            os.path.relpath(brrtrouter_path / "brrtrouter_macros", cargo_toml_dir)
        ).as_posix()
    except ValueError:
        rel = str(brrtrouter_path)
        rel_macros = str(brrtrouter_path / "brrtrouter_macros")

    content = re.sub(
        r'brrtrouter = \{ path = "[^"]+" \}',
        f'brrtrouter = {{ path = "{rel}" }}',
        content,
    )
    content = re.sub(
        r'brrtrouter_macros = \{ path = "[^"]+" \}',
        f'brrtrouter_macros = {{ path = "{rel_macros}" }}',
        content,
    )

    if content != original:
        cargo_toml_path.write_text(content)
        print(f"✅ Fixed paths in {cargo_toml_path}")
        return True
    print(f"Info:  No changes needed in {cargo_toml_path}")
    return False


def run(cargo_toml_path: Path, project_root: Optional[Path] = None) -> int:
    """Run fix for one Cargo.toml. Returns 0."""
    fix_cargo_toml(cargo_toml_path, project_root)
    return 0
