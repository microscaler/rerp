import hashlib
from pathlib import Path

from rerp_tooling import runtime


def _service(root: Path, suite: str, service: str, package: str, binary: str) -> None:
    spec = root / "openapi" / suite / service / "openapi.yaml"
    spec.parent.mkdir(parents=True)
    spec.write_text("openapi: 3.1.0\ninfo: {title: test, version: '1'}\npaths: {}\n")
    gen = root / "microservices" / suite / service / "gen"
    (gen / "doc").mkdir(parents=True)
    (gen / "static_site").mkdir()
    (gen / "config").mkdir()
    (gen / "Cargo.toml").write_text(f'[package]\nname = "{package}_gen"\nversion = "0.1.0"\n')
    impl = root / "microservices" / suite / service / "impl"
    impl.mkdir(parents=True)
    (impl / "Cargo.toml").write_text(
        f'''[package]
name = "{package}"
version = "0.1.0"

[[bin]]
name = "{binary}"
path = "src/main.rs"
'''
    )


def test_describe_service_reads_package_and_binary_independently(tmp_path: Path) -> None:
    _service(tmp_path, "documents", "render", "rerp_documents_render", "render")

    descriptor = runtime.describe_service(tmp_path, "documents", "render")

    assert descriptor["package_name"] == "rerp_documents_render"
    assert descriptor["binary_name"] == "render"
    assert descriptor["resource_name"] == "documents-render"
    assert descriptor["config_dir"] == "microservices/documents/render/gen/config"


def test_discover_services_includes_suite_level_bff_contract(tmp_path: Path) -> None:
    service_root = tmp_path / "microservices" / "accounting" / "bff"
    (service_root / "impl").mkdir(parents=True)
    (service_root / "gen").mkdir(parents=True)
    (service_root / "impl" / "Cargo.toml").write_text(
        '[[bin]]\nname = "accounting_bff"\n\n[package]\nname = "rerp_accounting_bff"\nversion = "0.1.0"\n'
    )
    (service_root / "gen" / "Cargo.toml").write_text(
        '[package]\nname = "rerp_accounting_bff_gen"\nversion = "0.1.0"\n'
    )
    suite_openapi = tmp_path / "openapi" / "accounting"
    suite_openapi.mkdir(parents=True)
    (suite_openapi / "openapi_bff.yaml").write_text("openapi: 3.1.0\n")
    values = tmp_path / "helm" / "rerp-microservice" / "values"
    values.mkdir(parents=True)
    (values / "bff.yaml").write_text("service: {}\n")

    descriptors = runtime.discover_services(tmp_path)

    assert [item["service"] for item in descriptors] == ["bff"]
    assert descriptors[0]["spec_path"] == "openapi/accounting/openapi_bff.yaml"


def test_discovery_requires_suite_qualified_helm_values_outside_accounting(
    tmp_path: Path,
) -> None:
    _service(tmp_path, "accounting", "invoice", "rerp_accounting_invoice", "invoice")
    _service(tmp_path, "documents", "render", "rerp_documents_render", "render")
    values = tmp_path / "helm" / "rerp-microservice" / "values"
    values.mkdir(parents=True)
    (values / "invoice.yaml").write_text("service: {name: invoice}\n")
    (values / "documents-render.yaml").write_text("service: {name: documents-render}\n")

    services = runtime.discover_services(tmp_path)

    assert [(item["suite"], item["service"]) for item in services] == [
        ("accounting", "invoice"),
        ("documents", "render"),
    ]


def test_image_build_uses_narrow_staged_context(tmp_path: Path, monkeypatch) -> None:
    _service(tmp_path, "accounting", "invoice", "rerp_accounting_invoice", "invoice")
    artifact = tmp_path / "build_artifacts" / "amd64" / "accounting" / "invoice"
    artifact.parent.mkdir(parents=True)
    artifact.write_bytes(b"binary")
    digest = hashlib.sha256(artifact.read_bytes()).hexdigest()
    digest_file = artifact.with_suffix(".sha256")
    digest_file.write_text(digest + "\n")
    dockerfile = tmp_path / "docker" / "microservices" / "Dockerfile"
    dockerfile.parent.mkdir(parents=True)
    dockerfile.write_text("FROM scratch\nCOPY service /app/service\n")
    observed = {}

    def fake_run(command, cwd, check):
        context = Path(command[-1])
        observed["command"] = command
        observed["files"] = sorted(
            path.relative_to(context).as_posix() for path in context.rglob("*") if path.is_file()
        )

        class Result:
            returncode = 0

        return Result()

    monkeypatch.setattr(runtime.subprocess, "run", fake_run)

    result = runtime.build_service_image(
        tmp_path,
        "localhost:5001/rerp-accounting-invoice",
        dockerfile,
        digest_file,
        artifact,
        "accounting",
        "invoice",
    )

    assert result == 0
    assert observed["command"][0:2] == ["docker", "build"]
    assert "localhost:5001/rerp-accounting-invoice:tilt" in observed["command"]
    assert observed["files"] == ["amd64/service"]


def test_stage_multiarch_context_selects_each_architecture_binary(tmp_path: Path) -> None:
    _service(tmp_path, "accounting", "invoice", "rerp_accounting_invoice", "invoice")
    dockerfile = tmp_path / "docker" / "microservices" / "Dockerfile"
    dockerfile.parent.mkdir(parents=True)
    dockerfile.write_text("ARG TARGETARCH\nFROM scratch\nCOPY ${TARGETARCH}/service /app/service\n")
    artifacts = tmp_path / "build_artifacts"
    for architecture in ("amd64", "arm64", "arm"):
        artifact = artifacts / architecture / "invoice"
        artifact.parent.mkdir(parents=True)
        artifact.write_bytes(architecture.encode())
        Path(f"{artifact}.sha256").write_text(hashlib.sha256(artifact.read_bytes()).hexdigest())

    assert (
        runtime.stage_multiarch_context(
            tmp_path, ".docker-context/invoice", artifacts, "accounting", "invoice"
        )
        == 0
    )

    context = tmp_path / ".docker-context" / "invoice"
    assert (context / "amd64" / "service").read_bytes() == b"amd64"
    assert (context / "arm64" / "service").read_bytes() == b"arm64"
    assert (context / "arm" / "service").read_bytes() == b"arm"
    assert (context / "Dockerfile").is_file()


def test_base_build_uses_only_the_canonical_local_tag(tmp_path: Path, monkeypatch) -> None:
    dockerfile = tmp_path / "docker" / "base" / "Dockerfile"
    dockerfile.parent.mkdir(parents=True)
    dockerfile.write_text("FROM scratch\n")
    observed = []

    def fake_run(command, cwd, check):
        observed.append((command, cwd, check))

        class Result:
            returncode = 0

        return Result()

    monkeypatch.setattr(runtime.subprocess, "run", fake_run)

    assert runtime.build_base_image(tmp_path) == 0
    assert observed == [
        (
            [
                "docker",
                "build",
                "--tag",
                "rerp-base:latest",
                "--file",
                str(dockerfile),
                str(tmp_path),
            ],
            tmp_path,
            False,
        )
    ]


def test_image_build_rejects_stale_hash_before_docker(tmp_path: Path, monkeypatch) -> None:
    _service(tmp_path, "accounting", "invoice", "rerp_accounting_invoice", "invoice")
    artifact = tmp_path / "invoice"
    artifact.write_bytes(b"new")
    digest_file = tmp_path / "invoice.sha256"
    digest_file.write_text("0" * 64)
    dockerfile = tmp_path / "Dockerfile"
    dockerfile.write_text("FROM scratch\n")
    monkeypatch.setattr(
        runtime.subprocess,
        "run",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError("docker must not run")),
    )

    assert (
        runtime.build_service_image(
            tmp_path,
            "image",
            dockerfile,
            digest_file,
            artifact,
            "accounting",
            "invoice",
        )
        == 1
    )
