"""CI automation. Re-exports from brrtrouter_tooling.ci when available; RERP keeps fix_cargo_paths."""

from rerp_tooling.ci.fix_cargo_paths import fix_cargo_toml
from rerp_tooling.ci.fix_cargo_paths import run as run_fix_cargo_paths

try:
    from brrtrouter_tooling.ci import (
        compare_versions,
        find_cargo_tomls,
        find_matches,
        fix_all_impl_dependencies,
        get_latest_tag,
        patch_file,
        run_cargo_update,
        run_get_latest_tag,
        run_is_tag,
        run_patch_brrtrouter,
        run_validate_version,
        run_validate_version_cli,
        update_impl_cargo_dependencies,
        validate_version,
    )

    _BRRT = True
except ImportError:
    _BRRT = False

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

if _BRRT:
    pass  # names already bound above
else:
    # Stub missing names so "from rerp_tooling.ci import X" does not break; callers use submodules.
    compare_versions = None  # type: ignore[assignment]
    find_cargo_tomls = None  # type: ignore[assignment]
    find_matches = None  # type: ignore[assignment]
    fix_all_impl_dependencies = None  # type: ignore[assignment]
    get_latest_tag = None  # type: ignore[assignment]
    patch_file = None  # type: ignore[assignment]
    run_cargo_update = None  # type: ignore[assignment]
    run_get_latest_tag = None  # type: ignore[assignment]
    run_is_tag = None  # type: ignore[assignment]
    run_patch_brrtrouter = None  # type: ignore[assignment]
    run_validate_version = None  # type: ignore[assignment]
    run_validate_version_cli = None  # type: ignore[assignment]
    update_impl_cargo_dependencies = None  # type: ignore[assignment]
    validate_version = None  # type: ignore[assignment]
