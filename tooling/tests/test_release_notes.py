"""Tests for rerp_tooling.release.notes."""

import json
from pathlib import Path
from unittest.mock import patch

import pytest

from rerp_tooling.release.notes import (
    DEFAULT_TEMPLATE,
    _call_anthropic,
    _get_commits_since,
    _get_previous_tag,
    _load_template,
    run,
)


class TestGetPreviousTag:
    def test_returns_tag(self, tmp_path: Path) -> None:
        from types import SimpleNamespace

        with patch("subprocess.run") as m:
            m.return_value = SimpleNamespace(stdout="v1.2.3\n")
            result = _get_previous_tag(tmp_path)
        assert result == "v1.2.3"

    def test_returns_none_on_error(self, tmp_path: Path) -> None:
        import subprocess

        with patch("subprocess.run", side_effect=subprocess.CalledProcessError(1, "git")):
            assert _get_previous_tag(tmp_path) is None


class TestGetCommitsSince:
    def test_returns_list(self, tmp_path: Path) -> None:
        from types import SimpleNamespace

        with patch("subprocess.run") as m:
            m.return_value = SimpleNamespace(stdout="feat: x\nfix: y\n")
            got = _get_commits_since(tmp_path, "v1.0.0")
        assert got == ["feat: x", "fix: y"]

    def test_returns_empty_when_no_commits(self, tmp_path: Path) -> None:
        from types import SimpleNamespace

        with patch("subprocess.run") as m:
            m.return_value = SimpleNamespace(stdout="")
            got = _get_commits_since(tmp_path, "v1.0.0")
        assert got == []


class TestLoadTemplate:
    def test_returns_default_when_none(self) -> None:
        assert _load_template(None) == DEFAULT_TEMPLATE

    def test_returns_default_when_path_not_file(self, tmp_path: Path) -> None:
        assert _load_template(tmp_path / "missing.md") == DEFAULT_TEMPLATE

    def test_returns_file_content(self, tmp_path: Path) -> None:
        p = tmp_path / "t.md"
        p.write_text("# Custom\n")
        assert _load_template(p) == "# Custom\n"


class TestRun:
    def test_fails_without_previous_tag(self, tmp_path: Path) -> None:
        with patch("rerp_tooling.release.notes._get_previous_tag", return_value=None):
            rc = run(tmp_path, "1.0.0", since_tag=None)
        assert rc == 1

    def test_succeeds_with_since_tag_and_mocked_openai(self, tmp_path: Path) -> None:
        out = tmp_path / "out.md"
        with patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: x"]):
            with patch("rerp_tooling.release.notes._call_openai", return_value="# Release v1.0.0\n\nDone."):
                with patch.dict("os.environ", {}, clear=False):
                    rc = run(tmp_path, "1.0.0", since_tag="v0.9.0", output_path=out, provider="openai")
        assert rc == 0
        assert "Release v1.0.0" in out.read_text()

    def test_succeeds_stdout_when_no_output_path(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        with patch("rerp_tooling.release.notes._get_previous_tag", return_value="v0.9.0"):
            with patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]):
                with patch("rerp_tooling.release.notes._call_openai", return_value="Notes here"):
                    rc = run(tmp_path, "1.0.0", output_path=None, provider="openai")
        assert rc == 0
        assert "Notes here" in capsys.readouterr().out

    def test_run_provider_anthropic_uses_call_anthropic(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        with patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: x"]):
            with patch(
                "rerp_tooling.release.notes._call_anthropic",
                return_value="# Release v1.0.0\n\nClaude notes.",
            ):
                rc = run(
                    tmp_path,
                    "1.0.0",
                    since_tag="v0.9.0",
                    output_path=None,
                    provider="anthropic",
                )
        assert rc == 0
        assert "Claude notes" in capsys.readouterr().out

    def test_run_respects_release_notes_provider_env(self, tmp_path: Path) -> None:
        out = tmp_path / "out.md"
        with patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]):
            with patch(
                "rerp_tooling.release.notes._call_anthropic",
                return_value="From Anthropic via env",
            ):
                with patch.dict("os.environ", {"RELEASE_NOTES_PROVIDER": "anthropic"}, clear=False):
                    rc = run(
                        tmp_path,
                        "1.0.0",
                        since_tag="v0.9.0",
                        output_path=out,
                        provider=None,
                    )
        assert rc == 0
        assert "From Anthropic via env" in out.read_text()

    def test_run_provider_anthropic_fails_without_anthropic_api_key(
        self, tmp_path: Path
    ) -> None:
        with patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]):
            with patch.dict("os.environ", {"ANTHROPIC_API_KEY": ""}, clear=False):
                with pytest.raises(SystemExit) as exc_info:
                    run(tmp_path, "1.0.0", since_tag="v0.9.0", provider="anthropic")
        assert exc_info.value.code == 1

    def test_run_returns_1_when_empty_body(self, tmp_path: Path) -> None:
        with patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]):
            with patch("rerp_tooling.release.notes._call_openai", return_value=""):
                rc = run(tmp_path, "1.0.0", since_tag="v0.9.0", output_path=tmp_path / "out.md", provider="openai")
        assert rc == 1


class TestCallAnthropic:
    def test_returns_text_from_content_block(self) -> None:
        payload = {
            "content": [{"type": "text", "text": "# Release v1.0.0\n\nAnthropic."}],
        }

        class FakeResp:
            def read(self) -> bytes:
                return json.dumps(payload).encode()

            def __enter__(self) -> "FakeResp":
                return self

            def __exit__(self, *a: object) -> None:
                pass

        with patch.dict("os.environ", {"ANTHROPIC_API_KEY": "test-key"}, clear=False):
            with patch("urllib.request.urlopen", return_value=FakeResp()):
                got = _call_anthropic(
                    ["feat: x"],
                    "Format here",
                    "1.0.0",
                    "claude-sonnet-4-5-20250929",
                )
        assert got == "# Release v1.0.0\n\nAnthropic."

    def test_strips_markdown_code_fence(self) -> None:
        payload = {
            "content": [
                {
                    "type": "text",
                    "text": "```markdown\n# Release v2.0.0\n\nDone.\n```",
                },
            ],
        }

        class FakeResp:
            def read(self) -> bytes:
                return json.dumps(payload).encode()

            def __enter__(self) -> "FakeResp":
                return self

            def __exit__(self, *a: object) -> None:
                pass

        with patch.dict("os.environ", {"ANTHROPIC_API_KEY": "k"}, clear=False):
            with patch("urllib.request.urlopen", return_value=FakeResp()):
                got = _call_anthropic([], "F", "2.0.0", "claude-sonnet-4-5-20250929")
        assert got == "# Release v2.0.0\n\nDone."
