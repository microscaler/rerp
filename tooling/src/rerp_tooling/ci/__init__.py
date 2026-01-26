"""CI-specific automation: patch Cargo for BRRTRouter/lifeguard from git; fix local path deps; check if ref is tag."""

from .is_tag import run as run_is_tag
from .patch_brrtrouter import (
    find_cargo_tomls,
    find_matches,
    patch_file,
    run,
    run_cargo_update,
)

__all__ = [
    "find_cargo_tomls",
    "find_matches",
    "patch_file",
    "run",
    "run_cargo_update",
    "run_is_tag",
]
