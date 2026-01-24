"""CI-specific automation: patch Cargo for BRRTRouter/lifeguard from git; fix local path deps."""

from .patch_brrtrouter import (
    find_cargo_tomls,
    find_matches,
    patch_file,
    run,
    run_cargo_update,
)

__all__ = ["find_cargo_tomls", "find_matches", "patch_file", "run", "run_cargo_update"]
