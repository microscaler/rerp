"""Validate version to prevent downgrades."""

import re
import sys


def compare_versions(v1: str, v2: str) -> int:
    """Compare two version strings.

    Returns:
        Positive if v1 > v2, negative if v1 < v2, zero if v1 == v2
    """
    # Strip 'v' prefix
    v1 = v1.lstrip("v")
    v2 = v2.lstrip("v")

    # Parse versions (X.Y.Z or X.Y.Z-prerelease)
    def parse_version(v: str) -> tuple[int, int, int, str | None]:
        m = re.match(r"^(\d+)\.(\d+)\.(\d+)(?:-([\w.-]+))?$", v)
        if not m:
            msg = f"Invalid version format: {v}"
            raise ValueError(msg)
        return (int(m.group(1)), int(m.group(2)), int(m.group(3)), m.group(4))

    try:
        major1, minor1, patch1, prerelease1 = parse_version(v1)
        major2, minor2, patch2, prerelease2 = parse_version(v2)
    except ValueError as e:
        msg = f"Version comparison error: {e}"
        raise SystemExit(msg) from e

    # Compare major.minor.patch
    if major1 != major2:
        return major1 - major2
    if minor1 != minor2:
        return minor1 - minor2
    if patch1 != patch2:
        return patch1 - patch2

    # Same base version, compare prerelease
    # Full release (no prerelease) is greater than any prerelease
    if prerelease1 is None and prerelease2 is not None:
        return 1
    if prerelease1 is not None and prerelease2 is None:
        return -1
    if prerelease1 is None and prerelease2 is None:
        return 0

    # Both have prerelease, compare lexicographically
    # For rc.N format, extract number for better comparison
    rc_match1 = re.match(r"^rc\.(\d+)$", prerelease1)
    rc_match2 = re.match(r"^rc\.(\d+)$", prerelease2)

    if rc_match1 and rc_match2:
        return int(rc_match1.group(1)) - int(rc_match2.group(1))

    # Fallback to string comparison
    if prerelease1 < prerelease2:
        return -1
    if prerelease1 > prerelease2:
        return 1
    return 0


def validate_version(current: str, latest: str | None, allow_same: bool = False) -> int:
    """Validate current version is greater than latest (or equal if allow_same).

    Args:
        current: Current version to validate
        latest: Latest version from GitHub (None if no releases exist)
        allow_same: Allow same version (for patch releases)

    Returns:
        0 if valid, raises SystemExit if invalid
    """
    if latest is None:
        # First release, always valid
        return 0

    cmp = compare_versions(current, latest)

    if cmp > 0:
        # Upgrade, valid
        return 0

    if cmp == 0:
        if allow_same:
            return 0
        msg = (
            f"Version {current} is not greater than latest release {latest}. "
            "Use --allow-same to allow same version (for patch releases)."
        )
        print(msg, file=sys.stderr)
        raise SystemExit(1)

    # cmp < 0, downgrade
    msg = (
        f"Version downgrade detected: current={current}, latest={latest}. "
        "Cannot release a version lower than the latest release."
    )
    print(msg, file=sys.stderr)
    raise SystemExit(1)


def run() -> int:
    """CLI entry point for validate-version command."""
    import argparse

    parser = argparse.ArgumentParser(description="Validate version to prevent downgrades")
    parser.add_argument("--current", required=True, help="Current version to validate")
    parser.add_argument("--latest", help="Latest version from GitHub (or use GITHUB_API)")
    parser.add_argument("--allow-same", action="store_true", help="Allow same version")

    args = parser.parse_args()

    latest = args.latest
    if not latest:
        # Try to get from environment or GitHub API
        import os

        from rerp_tooling.ci.get_latest_tag import get_latest_tag

        repo = os.environ.get("GITHUB_REPOSITORY", "")
        token = os.environ.get("GITHUB_TOKEN", "")

        if repo and token:
            latest = get_latest_tag(repo, token)
        else:
            msg = "--latest required or set GITHUB_REPOSITORY and GITHUB_TOKEN"
            raise SystemExit(msg)

    try:
        validate_version(args.current, latest, allow_same=args.allow_same)
        return 0
    except SystemExit as e:
        # SystemExit.code is the exit code (if set), otherwise args[0] if int, else 1
        code = getattr(e, "code", None)
        if code is None and e.args and isinstance(e.args[0], int):
            code = e.args[0]
        return code if code is not None else 1
