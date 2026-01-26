"""Check if GITHUB_REF is a release tag (refs/tags/v*)."""

import os


def run() -> int:
    """Check if GITHUB_REF is a release tag. Prints 'true' or 'false' to stdout, returns 0."""
    ref = os.environ.get("GITHUB_REF", "")
    is_tag = "true" if ref.startswith("refs/tags/v") else "false"
    print(is_tag)
    return 0
