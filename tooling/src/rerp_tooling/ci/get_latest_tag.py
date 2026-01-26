"""Get latest release tag from GitHub API."""

import json
import os
import sys
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen


def get_latest_tag(repo: str, token: str) -> str | None:
    """Get latest release tag from GitHub API.

    Args:
        repo: Repository in format "owner/repo"
        token: GitHub token for authentication

    Returns:
        Version string without 'v' prefix (e.g., "0.39.0") or None if no releases exist
    """
    url = f"https://api.github.com/repos/{repo}/releases/latest"
    headers = {
        "Accept": "application/vnd.github.v3+json",
        "Authorization": f"token {token}",
        "User-Agent": "rerp-tooling",
    }

    req = Request(url, headers=headers)

    try:
        with urlopen(req) as response:
            data = json.loads(response.read().decode())
            tag_name = data.get("tag_name", "")
            # Strip 'v' prefix if present
            return tag_name.lstrip("v") if tag_name else None
    except HTTPError as e:
        if e.code == 404:
            # No releases exist yet
            return None
        raise
    except URLError as e:
        msg = f"Failed to fetch latest release from GitHub: {e}"
        raise SystemExit(msg) from e


def run() -> int:
    """Get latest tag and print to stdout. Returns 0 on success, 1 on error."""
    repo = os.environ.get("GITHUB_REPOSITORY", "")
    token = os.environ.get("GITHUB_TOKEN", "")

    if not repo:
        print("", file=sys.stderr)
        return 1

    if not token:
        print("", file=sys.stderr)
        return 1

    try:
        latest = get_latest_tag(repo, token)
        if latest:
            print(latest)
        return 0
    except (SystemExit, HTTPError, URLError) as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
