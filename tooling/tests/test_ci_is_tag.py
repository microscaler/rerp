"""Tests for rerp_tooling.ci.is_tag (rerp ci is-tag)."""

import os
from io import StringIO
from unittest.mock import patch

from rerp_tooling.ci import run_is_tag as run


class TestIsTag:
    def test_returns_true_for_release_tag(self) -> None:
        with (
            patch.dict(os.environ, {"GITHUB_REF": "refs/tags/v0.39.0"}, clear=False),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "true"

    def test_returns_false_for_branch(self) -> None:
        with (
            patch.dict(os.environ, {"GITHUB_REF": "refs/heads/main"}, clear=False),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "false"

    def test_returns_false_for_pr(self) -> None:
        with (
            patch.dict(os.environ, {"GITHUB_REF": "refs/pull/123/merge"}, clear=False),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "false"

    def test_returns_true_for_rc_tag(self) -> None:
        # RC tags should still be detected as tags (they start with refs/tags/v)
        with (
            patch.dict(os.environ, {"GITHUB_REF": "refs/tags/v0.39.0-rc.2"}, clear=False),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "true"

    def test_returns_false_when_missing(self) -> None:
        with (
            patch.dict(os.environ, {}, clear=True),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "false"

    def test_returns_false_for_non_v_tag(self) -> None:
        with (
            patch.dict(os.environ, {"GITHUB_REF": "refs/tags/0.39.0"}, clear=False),
            patch("sys.stdout", new=StringIO()) as fake_out,
        ):
            result = run()
            assert result == 0
            assert fake_out.getvalue().strip() == "false"
