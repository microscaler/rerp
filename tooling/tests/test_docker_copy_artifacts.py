"""Tests for rerp_tooling.docker.copy_artifacts (rerp docker copy-artifacts, validate-build-artifacts)."""

from pathlib import Path

import pytest

pytest.importorskip("brrtrouter_tooling")


def _make_openapi_layout(project_root: Path, services: list[tuple[str, str]]) -> None:
    """Create openapi/{suite}/{service}/openapi.yaml so discovery returns names. services = [(suite, name), ...]."""
    for suite, name in services:
        d = project_root / "openapi" / suite / name
        d.mkdir(parents=True)
        (d / "openapi.yaml").write_text(
            f"openapi: 3.1.0\ninfo: {{}}\nservers:\n  - url: http://localhost:8001/api/v1/{suite}/{name}\n"
        )


class TestValidateBuildArtifacts:
    def test_missing_dir_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_artifacts import validate_build_artifacts

        assert validate_build_artifacts(tmp_path) == 1

    def test_missing_binary_in_dir_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_artifacts import validate_build_artifacts

        _make_openapi_layout(
            tmp_path, [("accounting", "general-ledger"), ("accounting", "invoice")]
        )
        for arch in ("amd64", "arm64", "arm"):
            (tmp_path / "build_artifacts" / arch).mkdir(parents=True)
        (tmp_path / "build_artifacts" / "amd64" / "invoice").write_bytes(b"")
        # general_ledger missing in amd64
        assert validate_build_artifacts(tmp_path) == 1

    def test_all_present_returns_0(self, tmp_path: Path):
        from rerp_tooling.build.constants import get_binary_names
        from rerp_tooling.docker.copy_artifacts import validate_build_artifacts

        _make_openapi_layout(
            tmp_path, [("accounting", "general-ledger"), ("accounting", "invoice")]
        )
        binary_names = get_binary_names(tmp_path)
        for arch in ("amd64", "arm64", "arm"):
            d = tmp_path / "build_artifacts" / arch
            d.mkdir(parents=True)
            for name in binary_names.values():
                (d / name).write_bytes(b"\x7fELF")
        assert validate_build_artifacts(tmp_path) == 0


class TestCopyArtifacts:
    def test_unknown_arch_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_artifacts import run

        assert run("x64", tmp_path) == 1

    def test_missing_binary_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_artifacts import run

        # Discovery expects one service; we do not create its binary so run fails
        _make_openapi_layout(tmp_path, [("accounting", "general-ledger")])
        triple = "x86_64-unknown-linux-musl"
        (tmp_path / "microservices" / "target" / triple / "release").mkdir(parents=True)
        assert run("amd64", tmp_path) == 1

    def test_copies_all_to_build_artifacts_amd64(self, tmp_path: Path):
        from rerp_tooling.build.constants import get_binary_names, get_package_names
        from rerp_tooling.docker.copy_artifacts import run

        _make_openapi_layout(
            tmp_path, [("accounting", "general-ledger"), ("accounting", "invoice")]
        )
        package_names = get_package_names(tmp_path)
        binary_names = get_binary_names(tmp_path)
        triple = "x86_64-unknown-linux-musl"
        rel = tmp_path / "microservices" / "target" / triple / "release"
        rel.mkdir(parents=True)
        for pkg in package_names.values():
            (rel / pkg).write_bytes(b"\x7fELF")
        assert run("amd64", tmp_path) == 0
        out = tmp_path / "build_artifacts" / "amd64"
        assert out.is_dir()
        for bin_name in binary_names.values():
            p = out / bin_name
            assert p.exists(), f"missing {bin_name}"
            assert p.stat().st_mode & 0o111

    def test_arm7_uses_artifact_dir_arm(self, tmp_path: Path):
        from rerp_tooling.build.constants import get_package_names
        from rerp_tooling.docker.copy_artifacts import run

        _make_openapi_layout(tmp_path, [("accounting", "general-ledger")])
        package_names = get_package_names(tmp_path)
        triple = "armv7-unknown-linux-musleabihf"
        rel = tmp_path / "microservices" / "target" / triple / "release"
        rel.mkdir(parents=True)
        for pkg in package_names.values():
            (rel / pkg).write_bytes(b"\x7fELF")
        assert run("arm7", tmp_path) == 0
        assert (tmp_path / "build_artifacts" / "arm").is_dir()
        assert (tmp_path / "build_artifacts" / "arm" / "general_ledger").exists()
