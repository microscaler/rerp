"""CI automation. Re-exports from brrtrouter_tooling.ci; RERP keeps fix_cargo_paths wrapper for gen name/version."""

from brrtrouter_tooling.ci import (
    compare_versions,
    find_cargo_tomls,
    find_matches,
    get_latest_tag,
    patch_file,
    run_cargo_update,
    run_get_latest_tag,
    run_is_tag,
    run_patch_brrtrouter,
    run_validate_version,
    run_validate_version_cli,
    validate_version,
)

try:
    from brrtrouter_tooling.ci import fix_all_impl_dependencies, update_impl_cargo_dependencies
except ImportError:
    fix_all_impl_dependencies = None  # type: ignore[misc, assignment]
    update_impl_cargo_dependencies = None  # type: ignore[misc, assignment]

from rerp_tooling.ci.fix_cargo_paths import fix_cargo_toml
from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths

__all__ = [
    "compare_versions",
    "find_cargo_tomls",
    "find_matches",
    "fix_all_impl_dependencies",
    "fix_cargo_toml",
    "get_latest_tag",
    "patch_file",
    "run_cargo_update",
    "run_fix_cargo_paths",
    "run_get_latest_tag",
    "run_is_tag",
    "run_patch_brrtrouter",
    "run_validate_version",
    "run_validate_version_cli",
    "update_impl_cargo_dependencies",
    "validate_version",
]
