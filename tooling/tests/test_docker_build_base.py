"""Tests for rerp_tooling.docker.build_base (rerp docker build-base)."""

from pathlib import Path


class TestBuildBase:
    def test_dockerfile_missing_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.build_base import run

        (tmp_path / "docker").mkdir(parents=True)
        # No docker/base/Dockerfile
        assert run(tmp_path, push=False, dry_run=False) == 1

    def test_dry_run_returns_0_and_does_not_invoke_docker(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.docker.build_base import run

        (tmp_path / "docker" / "base").mkdir(parents=True)
        (tmp_path / "docker" / "base" / "Dockerfile").write_text("FROM alpine\n")
        seen = []

        def fake_run(cmd, *a, **kw):
            seen.append(cmd)
            return type("R", (), {"returncode": 0})()

        monkeypatch.setattr("rerp_tooling.docker.build_base.subprocess.run", fake_run)
        assert run(tmp_path, push=False, dry_run=True) == 0
        assert not seen

    def test_push_without_owner_returns_1(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.docker.build_base import run

        (tmp_path / "docker" / "base").mkdir(parents=True)
        (tmp_path / "docker" / "base" / "Dockerfile").write_text("FROM alpine\n")
        monkeypatch.delenv("GHCR_OWNER", raising=False)
        monkeypatch.delenv("GITHUB_REPOSITORY_OWNER", raising=False)
        assert run(tmp_path, push=True, dry_run=False) == 1

    def test_build_invokes_docker_and_returns_0(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.docker.build_base import run

        (tmp_path / "docker" / "base").mkdir(parents=True)
        (tmp_path / "docker" / "base" / "Dockerfile").write_text("FROM alpine\n")
        calls = []

        def fake_run(cmd, **kw):
            calls.append(cmd)
            return type("R", (), {"returncode": 0})()

        monkeypatch.setattr("rerp_tooling.docker.build_base.subprocess.run", fake_run)
        assert run(tmp_path, push=False, dry_run=False) == 0
        assert len(calls) == 1
        assert calls[0][:2] == ["docker", "build"]

    def test_build_failure_returns_1(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.docker.build_base import run

        (tmp_path / "docker" / "base").mkdir(parents=True)
        (tmp_path / "docker" / "base" / "Dockerfile").write_text("FROM alpine\n")

        def fake_run(*args, **kw):
            return type("R", (), {"returncode": 1})()

        monkeypatch.setattr("rerp_tooling.docker.build_base.subprocess.run", fake_run)
        assert run(tmp_path, push=False, dry_run=False) == 1
