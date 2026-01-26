"""CI-specific automation: patch Cargo for BRRTRouter/lifeguard from git; fix local path deps; check if ref is tag; validate versions."""

from .get_latest_tag import get_latest_tag
from .get_latest_tag import run as run_get_latest_tag
from .is_tag import run as run_is_tag
from .patch_brrtrouter import (
    find_cargo_tomls,
    find_matches,
    patch_file,
    run,
    run_cargo_update,
)
from .validate_version import (
    compare_versions,
    run_validate_version_cli,
    validate_version,
)
from .validate_version import (
    run as run_validate_version,
)

__all__ = [
    "compare_versions",
    "find_cargo_tomls",
    "find_matches",
    "get_latest_tag",
    "patch_file",
    "run",
    "run_cargo_update",
    "run_get_latest_tag",
    "run_is_tag",
    "run_validate_version",
    "run_validate_version_cli",
    "validate_version",
]
