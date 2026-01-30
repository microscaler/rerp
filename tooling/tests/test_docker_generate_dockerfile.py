"""Tests for rerp docker generate-dockerfile (delegates to brrtrouter_tooling.docker.generate_dockerfile)."""

from pathlib import Path

import pytest

pytest.importorskip("brrtrouter_tooling")


class TestGenerateDockerfile:
    def test_generates_file_with_substitutions(self, tmp_path: Path):
        from brrtrouter_tooling.docker.generate_dockerfile import generate_dockerfile

        (tmp_path / "docker" / "microservices").mkdir(parents=True)
        tpl = tmp_path / "docker" / "microservices" / "Dockerfile.template"
        tpl.write_text(
            "FROM x\nCOPY {{binary_name}}\nEXPOSE {{port}}\n# {{service_name}} {{system}} {{module}}\n"
        )
        out = generate_dockerfile("auth", "idam", port=8000, project_root=tmp_path)
        assert out == tmp_path / "docker" / "microservices" / "Dockerfile.auth_idam"
        text = out.read_text()
        assert "rerp_auth_idam_impl" in text
        assert "8000" in text
        assert "auth-idam" in text
        assert "auth" in text
        assert "idam" in text

    def test_template_missing_returns_1(self, tmp_path: Path):
        from brrtrouter_tooling.docker.generate_dockerfile import run

        (tmp_path / "docker" / "microservices").mkdir(parents=True)
        # No template: run() catches FileNotFoundError and returns 1
        assert run("x", "y", project_root=tmp_path) == 1


class TestRun:
    def test_run_returns_zero(self, tmp_path: Path):
        from brrtrouter_tooling.docker.generate_dockerfile import run

        (tmp_path / "docker" / "microservices").mkdir(parents=True)
        (tmp_path / "docker" / "microservices" / "Dockerfile.template").write_text("FROM x\n")
        assert run("a", "b", port=9000, project_root=tmp_path) == 0
        out = tmp_path / "docker" / "microservices" / "Dockerfile.a_b"
        assert out.exists()
