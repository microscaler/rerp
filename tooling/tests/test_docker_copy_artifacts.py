"""Tests for rerp_tooling.docker.copy_artifacts (rerp docker copy-artifacts)."""

from pathlib import Path


class TestCopyArtifacts:
    def test_unknown_arch_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_artifacts import run

        assert run("x64", tmp_path) == 1

    def test_missing_binary_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_artifacts import run

        triple = "x86_64-unknown-linux-musl"
        (tmp_path / "microservices" / "target" / triple / "release").mkdir(parents=True)
        # None of the package binaries exist
        assert run("amd64", tmp_path) == 1

    def test_copies_all_to_build_artifacts_amd64(self, tmp_path: Path):
        from rerp_tooling.build.microservices import PACKAGE_NAMES
        from rerp_tooling.docker.copy_artifacts import BINARY_NAMES, run

        triple = "x86_64-unknown-linux-musl"
        rel = tmp_path / "microservices" / "target" / triple / "release"
        rel.mkdir(parents=True)
        for pkg in PACKAGE_NAMES.values():
            (rel / pkg).write_bytes(b"\x7fELF")
        assert run("amd64", tmp_path) == 0
        out = tmp_path / "build_artifacts" / "amd64"
        assert out.is_dir()
        for bin_name in BINARY_NAMES.values():
            p = out / bin_name
            assert p.exists(), f"missing {bin_name}"
            assert p.stat().st_mode & 0o111

    def test_arm7_uses_artifact_dir_arm(self, tmp_path: Path):
        from rerp_tooling.build.microservices import PACKAGE_NAMES
        from rerp_tooling.docker.copy_artifacts import run

        triple = "armv7-unknown-linux-musleabihf"
        rel = tmp_path / "microservices" / "target" / triple / "release"
        rel.mkdir(parents=True)
        for pkg in PACKAGE_NAMES.values():
            (rel / pkg).write_bytes(b"\x7fELF")
        assert run("arm7", tmp_path) == 0
        assert (tmp_path / "build_artifacts" / "arm").is_dir()
        assert (tmp_path / "build_artifacts" / "arm" / "general_ledger").exists()
