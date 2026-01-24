"""Tests for rerp_tooling.docker.build_image_simple (rerp docker build-image-simple)."""

from pathlib import Path
from unittest.mock import MagicMock, patch


class TestBuildImageSimple:
    def test_hash_missing_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.build_image_simple import run

        (tmp_path / "art").write_bytes(b"x")
        (tmp_path / "Dockerfile").write_text("FROM alpine\n")
        assert (
            run(
                "img",
                Path("Dockerfile"),
                Path("missing.sha256"),
                Path("art"),
                tmp_path,
            )
            == 1
        )

    def test_artifact_missing_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.build_image_simple import run

        (tmp_path / "h.sha256").write_text("a" * 64)
        (tmp_path / "Dockerfile").write_text("FROM alpine\n")
        assert run("img", Path("Dockerfile"), Path("h.sha256"), Path("missing"), tmp_path) == 1

    def test_dockerfile_missing_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.build_image_simple import run

        (tmp_path / "h.sha256").write_text("a" * 64)
        (tmp_path / "art").write_bytes(b"x")
        assert (
            run(
                "img",
                Path("missing"),
                Path("h.sha256"),
                Path("art"),
                tmp_path,
            )
            == 1
        )

    def test_docker_build_fails_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.build_image_simple import run

        (tmp_path / "h.sha256").write_text("a" * 64)
        (tmp_path / "art").write_bytes(b"x")
        (tmp_path / "Dockerfile").write_text("FROM alpine\n")
        with patch("subprocess.run") as m:
            m.return_value = MagicMock(returncode=1)
            assert (
                run(
                    "img",
                    Path("Dockerfile"),
                    Path("h.sha256"),
                    Path("art"),
                    tmp_path,
                )
                == 1
            )
            m.assert_called_once()
            assert m.call_args[0][0][:2] == ["docker", "build"]

    def test_docker_build_ok_push_ok_returns_0(self, tmp_path: Path):
        from rerp_tooling.docker.build_image_simple import run

        (tmp_path / "h.sha256").write_text("a" * 64)
        (tmp_path / "art").write_bytes(b"x")
        (tmp_path / "Dockerfile").write_text("FROM alpine\n")
        with patch("subprocess.run") as m:
            m.return_value = MagicMock(returncode=0)
            assert (
                run(
                    "img",
                    Path("Dockerfile"),
                    Path("h.sha256"),
                    Path("art"),
                    tmp_path,
                )
                == 0
            )
            assert m.call_count >= 2
            assert m.call_args_list[0][0][0][:2] == ["docker", "build"]
            assert m.call_args_list[1][0][0][:2] == ["docker", "push"]

    def test_docker_build_ok_push_fail_kind_ok_returns_0(self, tmp_path: Path):
        from rerp_tooling.docker.build_image_simple import run

        (tmp_path / "h.sha256").write_text("a" * 64)
        (tmp_path / "art").write_bytes(b"x")
        (tmp_path / "Dockerfile").write_text("FROM alpine\n")
        with patch("subprocess.run") as m:
            m.side_effect = [
                MagicMock(returncode=0),
                MagicMock(returncode=1, stdout="", stderr="connection refused"),
                MagicMock(returncode=0),
            ]
            assert (
                run(
                    "img",
                    Path("Dockerfile"),
                    Path("h.sha256"),
                    Path("art"),
                    tmp_path,
                )
                == 0
            )
            assert m.call_count == 3
            assert m.call_args_list[1][0][0][:2] == ["docker", "push"]
            assert "kind" in m.call_args_list[2][0][0]
