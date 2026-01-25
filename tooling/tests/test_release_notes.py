"""Tests for rerp_tooling.release.notes."""

import json
from pathlib import Path
from unittest.mock import patch

import pytest

from rerp_tooling.release.notes import (
    DEFAULT_TEMPLATE,
    _call_anthropic,
    _call_openai,
    _get_commits_since,
    _get_previous_tag,
    _load_template,
    run,
)

# Basic JSON responses for mocked HTTP: validate provider→API path and parsing.
OPENAI_BASIC = {"choices": [{"message": {"content": "OpenAI basic response for 1.0.0"}}]}
ANTHROPIC_BASIC = {"content": [{"type": "text", "text": "Anthropic basic response for 1.0.0"}]}


def _fake_http_resp(payload: dict):
    class _Resp:
        def read(self) -> bytes:
            return json.dumps(payload).encode()

        def __enter__(self) -> "_Resp":
            return self

        def __exit__(self, *a: object) -> None:
            pass

    return _Resp()


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

    def test_returns_none_when_stdout_empty_or_whitespace(self, tmp_path: Path) -> None:
        from types import SimpleNamespace

        with patch("subprocess.run") as m:
            m.return_value = SimpleNamespace(stdout="\n  \n")
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

    def test_raises_system_exit_on_invalid_ref(self, tmp_path: Path) -> None:
        import subprocess

        err = subprocess.CalledProcessError(
            128,
            ["git", "log", "v99..HEAD", "--pretty=format:%s"],
            stderr="fatal: bad revision 'v99..HEAD'\n",
        )
        with patch("subprocess.run", side_effect=err), pytest.raises(SystemExit) as exc_info:
            _get_commits_since(tmp_path, "v99")
        assert exc_info.value.code == 1


class TestLoadTemplate:
    def test_returns_default_when_none(self) -> None:
        assert _load_template(None) == DEFAULT_TEMPLATE

    def test_returns_default_when_path_not_file(self, tmp_path: Path) -> None:
        assert _load_template(tmp_path / "missing.md") == DEFAULT_TEMPLATE

    def test_returns_file_content(self, tmp_path: Path) -> None:
        p = tmp_path / "t.md"
        p.write_text("# Custom\n")
        assert _load_template(p) == "# Custom\n"

    def test_returns_default_when_path_is_directory(self, tmp_path: Path) -> None:
        (tmp_path / "adir").mkdir()
        assert _load_template(tmp_path / "adir") == DEFAULT_TEMPLATE


class TestRun:
    def test_fails_without_previous_tag(self, tmp_path: Path) -> None:
        with patch("rerp_tooling.release.notes._get_previous_tag", return_value=None):
            rc = run(tmp_path, "1.0.0", since_tag=None)
        assert rc == 1

    def test_succeeds_with_since_tag_and_mocked_openai(self, tmp_path: Path) -> None:
        out = tmp_path / "out.md"
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: x"]),
            patch(
                "rerp_tooling.release.notes._call_openai", return_value="# Release v1.0.0\n\nDone."
            ),
            patch.dict("os.environ", {}, clear=False),
        ):
            rc = run(tmp_path, "1.0.0", since_tag="v0.9.0", output_path=out, provider="openai")
        assert rc == 0
        assert "Release v1.0.0" in out.read_text()

    def test_succeeds_stdout_when_no_output_path(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        with (
            patch("rerp_tooling.release.notes._get_previous_tag", return_value="v0.9.0"),
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch("rerp_tooling.release.notes._call_openai", return_value="Notes here"),
        ):
            rc = run(tmp_path, "1.0.0", output_path=None, provider="openai")
        assert rc == 0
        assert "Notes here" in capsys.readouterr().out

    def test_run_provider_anthropic_uses_call_anthropic(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: x"]),
            patch(
                "rerp_tooling.release.notes._call_anthropic",
                return_value="# Release v1.0.0\n\nClaude notes.",
            ),
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
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch(
                "rerp_tooling.release.notes._call_anthropic",
                return_value="From Anthropic via env",
            ),
            patch.dict("os.environ", {"RELEASE_NOTES_PROVIDER": "anthropic"}, clear=False),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=out,
                provider=None,
            )
        assert rc == 0
        assert "From Anthropic via env" in out.read_text()

    def test_run_provider_anthropic_fails_without_anthropic_api_key(self, tmp_path: Path) -> None:
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch.dict("os.environ", {"ANTHROPIC_API_KEY": ""}, clear=False),
            pytest.raises(SystemExit) as exc_info,
        ):
            run(tmp_path, "1.0.0", since_tag="v0.9.0", provider="anthropic")
        assert exc_info.value.code == 1

    def test_run_returns_1_when_empty_body(self, tmp_path: Path) -> None:
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch("rerp_tooling.release.notes._call_openai", return_value=""),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=tmp_path / "out.md",
                provider="openai",
            )
        assert rc == 1

    def test_run_fails_on_invalid_provider(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        with patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]):
            rc = run(tmp_path, "1.0.0", since_tag="v0.9.0", output_path=None, provider="claude")
        assert rc == 1
        err = capsys.readouterr().err
        assert "Invalid provider 'claude'" in err
        assert "openai" in err and "anthropic" in err

    def test_run_fails_on_invalid_provider_via_env(
        self, tmp_path: Path, capsys: pytest.CaptureFixture[str]
    ) -> None:
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch.dict("os.environ", {"RELEASE_NOTES_PROVIDER": "claude"}, clear=False),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=None,
                provider=None,
            )
        assert rc == 1
        assert "Invalid provider 'claude'" in capsys.readouterr().err

    def test_run_returns_1_when_body_whitespace_only(self, tmp_path: Path) -> None:
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch("rerp_tooling.release.notes._call_openai", return_value="   \n  \t  "),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=tmp_path / "out.md",
                provider="openai",
            )
        assert rc == 1

    def test_run_openai_fails_without_openai_api_key(self, tmp_path: Path) -> None:
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["a"]),
            patch.dict("os.environ", {"OPENAI_API_KEY": ""}, clear=False),
            pytest.raises(SystemExit) as exc_info,
        ):
            run(tmp_path, "1.0.0", since_tag="v0.9.0", provider="openai")
        assert exc_info.value.code == 1

    def test_run_output_path_creates_parent_dirs(self, tmp_path: Path) -> None:
        out = tmp_path / "a" / "b" / "out.md"
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["x"]),
            patch(
                "rerp_tooling.release.notes._call_openai",
                return_value="# Release v1.0.0\n\nDone.",
            ),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=out,
                provider="openai",
            )
        assert rc == 0
        assert out.is_file()
        assert "Release v1.0.0" in out.read_text()

    def test_run_openai_path_mocked_http(self, tmp_path: Path) -> None:
        """Provider openai: full path through _call_openai and JSON parsing."""
        out = tmp_path / "out.md"
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: x"]),
            patch("urllib.request.urlopen", return_value=_fake_http_resp(OPENAI_BASIC)),
            patch.dict("os.environ", {"OPENAI_API_KEY": "sk-fake"}, clear=False),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=out,
                provider="openai",
            )
        assert rc == 0
        assert "OpenAI basic response for 1.0.0" in out.read_text()

    def test_run_anthropic_path_mocked_http(self, tmp_path: Path) -> None:
        """Provider anthropic: full path through _call_anthropic and JSON parsing."""
        out = tmp_path / "out.md"
        with (
            patch("rerp_tooling.release.notes._get_commits_since", return_value=["feat: x"]),
            patch("urllib.request.urlopen", return_value=_fake_http_resp(ANTHROPIC_BASIC)),
            patch.dict("os.environ", {"ANTHROPIC_API_KEY": "skant-fake"}, clear=False),
        ):
            rc = run(
                tmp_path,
                "1.0.0",
                since_tag="v0.9.0",
                output_path=out,
                provider="anthropic",
            )
        assert rc == 0
        assert "Anthropic basic response for 1.0.0" in out.read_text()


class TestCallOpenai:
    def test_returns_text_from_choices_message_content(self) -> None:
        payload = {
            "choices": [{"message": {"content": "# Release v1.0.0\n\nOpenAI."}}],
        }

        class FakeResp:
            def read(self) -> bytes:
                return json.dumps(payload).encode()

            def __enter__(self) -> "FakeResp":
                return self

            def __exit__(self, *a: object) -> None:
                pass

        with (
            patch.dict("os.environ", {"OPENAI_API_KEY": "test-key"}, clear=False),
            patch("urllib.request.urlopen", return_value=FakeResp()),
        ):
            got = _call_openai(
                ["feat: x"],
                "Format here",
                "1.0.0",
                "gpt-4o-mini",
            )
        assert got == "# Release v1.0.0\n\nOpenAI."

    def test_strips_markdown_code_fence(self) -> None:
        payload = {
            "choices": [
                {
                    "message": {
                        "content": "```markdown\n# Release v2.0.0\n\nDone.\n```",
                    }
                }
            ],
        }

        class FakeResp:
            def read(self) -> bytes:
                return json.dumps(payload).encode()

            def __enter__(self) -> "FakeResp":
                return self

            def __exit__(self, *a: object) -> None:
                pass

        with (
            patch.dict("os.environ", {"OPENAI_API_KEY": "k"}, clear=False),
            patch("urllib.request.urlopen", return_value=FakeResp()),
        ):
            got = _call_openai([], "F", "2.0.0", "gpt-4o-mini")
        assert got == "# Release v2.0.0\n\nDone."

    def test_raises_without_openai_api_key(self) -> None:
        with (
            patch.dict("os.environ", {"OPENAI_API_KEY": ""}, clear=False),
            pytest.raises(SystemExit) as exc_info,
        ):
            _call_openai([], "F", "1.0.0", "gpt-4o-mini")
        assert exc_info.value.code == 1


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

        with (
            patch.dict("os.environ", {"ANTHROPIC_API_KEY": "test-key"}, clear=False),
            patch("urllib.request.urlopen", return_value=FakeResp()),
        ):
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

        with (
            patch.dict("os.environ", {"ANTHROPIC_API_KEY": "k"}, clear=False),
            patch("urllib.request.urlopen", return_value=FakeResp()),
        ):
            got = _call_anthropic([], "F", "2.0.0", "claude-sonnet-4-5-20250929")
        assert got == "# Release v2.0.0\n\nDone."
