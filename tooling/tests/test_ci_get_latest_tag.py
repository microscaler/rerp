"""Tests for rerp_tooling.ci.get_latest_tag (rerp ci get-latest-tag)."""

import json
import os
from io import StringIO
from unittest.mock import Mock, patch

from rerp_tooling.ci.get_latest_tag import get_latest_tag, run


def _fake_urlopen_success(tag_name: str):
    """Create a mock urlopen that returns successful response."""
    mock_response = Mock()
    mock_response.read.return_value = json.dumps({"tag_name": tag_name}).encode()
    mock_response.__enter__ = Mock(return_value=mock_response)
    mock_response.__exit__ = Mock(return_value=None)
    return mock_response


def _fake_urlopen_404():
    """Create a mock urlopen that returns 404."""
    from urllib.error import HTTPError

    mock_response = Mock()
    error = HTTPError("url", 404, "Not Found", {}, None)
    mock_response.__enter__ = Mock(side_effect=error)
    mock_response.__exit__ = Mock(return_value=None)
    return error


class TestGetLatestTag:
    def test_returns_latest_tag_from_github_api(self) -> None:
        with patch(
            "rerp_tooling.ci.get_latest_tag.urlopen", return_value=_fake_urlopen_success("v0.39.0")
        ):
            result = get_latest_tag("owner/repo", "token")
            assert result == "0.39.0"

    def test_strips_v_prefix(self) -> None:
        with patch(
            "rerp_tooling.ci.get_latest_tag.urlopen", return_value=_fake_urlopen_success("v1.2.3")
        ):
            result = get_latest_tag("owner/repo", "token")
            assert result == "1.2.3"

    def test_handles_tag_without_v_prefix(self) -> None:
        with patch(
            "rerp_tooling.ci.get_latest_tag.urlopen", return_value=_fake_urlopen_success("0.39.0")
        ):
            result = get_latest_tag("owner/repo", "token")
            assert result == "0.39.0"

    def test_handles_rc_tags(self) -> None:
        with patch(
            "rerp_tooling.ci.get_latest_tag.urlopen",
            return_value=_fake_urlopen_success("v0.39.0-rc.2"),
        ):
            result = get_latest_tag("owner/repo", "token")
            assert result == "0.39.0-rc.2"

    def test_returns_none_when_no_releases(self) -> None:
        with patch("rerp_tooling.ci.get_latest_tag.urlopen", side_effect=_fake_urlopen_404()):
            result = get_latest_tag("owner/repo", "token")
            assert result is None

    def test_run_prints_version_to_stdout(self) -> None:
        with (
            patch(
                "rerp_tooling.ci.get_latest_tag.urlopen",
                return_value=_fake_urlopen_success("v0.39.0"),
            ),
            patch.dict(
                os.environ,
                {"GITHUB_REPOSITORY": "owner/repo", "GITHUB_TOKEN": "token"},
                clear=False,
            ),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "0.39.0"

    def test_run_handles_no_releases(self) -> None:
        with (
            patch("rerp_tooling.ci.get_latest_tag.urlopen", side_effect=_fake_urlopen_404()),
            patch.dict(
                os.environ,
                {"GITHUB_REPOSITORY": "owner/repo", "GITHUB_TOKEN": "token"},
                clear=False,
            ),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == ""
