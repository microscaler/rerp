"""Get latest release tag from GitHub API."""

import json
import os
import sys
import time
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen


def _fibonacci_backoff_sequence(max_total_seconds: int = 300) -> list[int]:
    """Generate Fibonacci backoff sequence up to max_total_seconds.

    Returns list of wait times in seconds: [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...]
    Stops when cumulative sum would exceed max_total_seconds.
    """
    sequence = []
    total = 0
    a, b = 1, 1
    while total + a <= max_total_seconds:
        sequence.append(a)
        total += a
        a, b = b, a + b
    return sequence


def get_latest_tag(repo: str, token: str, max_retries: int = 20) -> str | None:
    """Get latest release tag from GitHub API with retry logic.

    Uses Fibonacci backoff (1, 1, 2, 3, 5, 8, 13, ... seconds) up to 300 seconds total.
    Retries on HTTPError (except 404) and URLError. Logs each retry attempt.

    Args:
        repo: Repository in format "owner/repo"
        token: GitHub token for authentication
        max_retries: Maximum number of retry attempts (default: 20)

    Returns:
        Version string without 'v' prefix (e.g., "0.39.0") or None if no releases exist

    Raises:
        SystemExit: If all retries are exhausted or non-retryable error occurs
    """
    url = f"https://api.github.com/repos/{repo}/releases/latest"
    headers = {
        "Accept": "application/vnd.github.v3+json",
        "Authorization": f"token {token}",
        "User-Agent": "rerp-tooling",
    }

    backoff_sequence = _fibonacci_backoff_sequence(max_total_seconds=300)
    last_error = None

    for attempt in range(max_retries):
        req = Request(url, headers=headers)

        try:
            with urlopen(req) as response:
                data = json.loads(response.read().decode())
                tag_name = data.get("tag_name", "")
                # Strip 'v' prefix if present
                return tag_name.lstrip("v") if tag_name else None
        except HTTPError as e:
            if e.code == 404:
                # No releases exist yet - not an error, return None
                return None
            # Retry on other HTTP errors (rate limiting, server errors, etc.)
            last_error = e
            if attempt < len(backoff_sequence):
                wait_time = backoff_sequence[attempt]
                print(
                    f"Retry {attempt + 1}/{max_retries}: HTTP {e.code} error, waiting {wait_time}s before retry...",
                    file=sys.stderr,
                )
                time.sleep(wait_time)
            else:
                # Out of backoff sequence, use last wait time
                wait_time = backoff_sequence[-1] if backoff_sequence else 1
                print(
                    f"Retry {attempt + 1}/{max_retries}: HTTP {e.code} error, waiting {wait_time}s before retry...",
                    file=sys.stderr,
                )
                time.sleep(wait_time)
        except URLError as e:
            # Retry on network errors
            last_error = e
            if attempt < len(backoff_sequence):
                wait_time = backoff_sequence[attempt]
                print(
                    f"Retry {attempt + 1}/{max_retries}: Network error ({e}), waiting {wait_time}s before retry...",
                    file=sys.stderr,
                )
                time.sleep(wait_time)
            else:
                # Out of backoff sequence, use last wait time
                wait_time = backoff_sequence[-1] if backoff_sequence else 1
                print(
                    f"Retry {attempt + 1}/{max_retries}: Network error ({e}), waiting {wait_time}s before retry...",
                    file=sys.stderr,
                )
                time.sleep(wait_time)

    # All retries exhausted
    if isinstance(last_error, HTTPError):
        msg = f"Failed to fetch latest release from GitHub after {max_retries} retries: HTTP {last_error.code} {last_error.reason}"
    else:
        msg = (
            f"Failed to fetch latest release from GitHub after {max_retries} retries: {last_error}"
        )
    raise SystemExit(msg) from last_error


def run() -> int:
    """Get latest tag and print to stdout. Returns 0 on success, 1 on error."""
    repo = os.environ.get("GITHUB_REPOSITORY", "")
    token = os.environ.get("GITHUB_TOKEN", "")

    if not repo:
        print("Error: GITHUB_REPOSITORY environment variable is required", file=sys.stderr)
        return 1

    if not token:
        print("Error: GITHUB_TOKEN environment variable is required", file=sys.stderr)
        return 1

    try:
        latest = get_latest_tag(repo, token)
        if latest:
            print(latest)
        return 0
    except (SystemExit, HTTPError, URLError) as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1
