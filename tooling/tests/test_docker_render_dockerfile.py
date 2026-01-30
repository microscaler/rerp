"""Tests for rerp docker render-dockerfile and template rendering (Dockerfile.template + --service)."""

from pathlib import Path

from rerp_tooling.docker.render_dockerfile import (
    render_dockerfile_template,
    render_dockerfile_to_path,
)


def _make_openapi_spec(project_root: Path, suite: str, service: str, port: int = 8002) -> None:
    d = project_root / "openapi" / suite / service
    d.mkdir(parents=True)
    (d / "openapi.yaml").write_text(
        f"openapi: 3.1.0\ninfo: {{}}\nservers:\n  - url: http://localhost:{port}/api/v1/{suite}/{service}\n"
    )


def test_render_dockerfile_template_derives_port_and_binary(tmp_path: Path) -> None:
    _make_openapi_spec(tmp_path, "accounting", "invoice", port=8002)
    template = tmp_path / "Dockerfile.template"
    template.write_text(
        "FROM alpine\nCOPY ./build_artifacts/${TARGETARCH}/{{binary_name}} /app/{{binary_name}}\n"
        "COPY ./microservices/{{system}}/{{module}}/impl/config /app/config\n"
        "EXPOSE {{port}}\n"
    )
    out = render_dockerfile_template(tmp_path, Path("Dockerfile.template"), "invoice")
    assert "invoice" in out
    assert "8002" in out
    assert "{{binary_name}}" not in out
    assert "{{port}}" not in out
    assert "/app/invoice" in out
    assert "microservices/accounting/invoice" in out


def test_render_dockerfile_to_path_writes_file(tmp_path: Path) -> None:
    _make_openapi_spec(tmp_path, "accounting", "general-ledger", port=8001)
    template = tmp_path / "Dockerfile.template"
    template.write_text("EXPOSE {{port}}\n")
    out_path = tmp_path / "docker" / "microservices" / "Dockerfile.general-ledger"
    render_dockerfile_to_path(
        tmp_path, Path("Dockerfile.template"), "general-ledger", output_path=out_path
    )
    assert out_path.read_text() == "EXPOSE 8001\n"
