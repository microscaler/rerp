"""Tests for rerp_tooling.docker.copy_multiarch (rerp docker copy-multiarch)."""

from pathlib import Path


class TestCopyMultiarch:
    def test_unknown_arch_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_multiarch import run

        assert run("auth", "idam", "x64", tmp_path) == 1

    def test_missing_binary_continues_no_copies(self, tmp_path: Path):
        from rerp_tooling.docker.copy_multiarch import run

        # No components/target/.../release/rerp_auth_idam_impl
        assert run("auth", "idam", "amd64", tmp_path) == 1
        assert not (tmp_path / "build_artifacts").exists()

    def test_success_copies_and_writes_sha256(self, tmp_path: Path):
        from rerp_tooling.docker.copy_multiarch import run

        triple = "x86_64-unknown-linux-musl"
        src = tmp_path / "components" / "target" / triple / "release"
        src.mkdir(parents=True)
        (src / "rerp_auth_idam_impl").write_bytes(b"binary")
        assert run("auth", "idam", "amd64", tmp_path) == 0
        dest = tmp_path / "build_artifacts" / "auth_idam" / "amd64"
        assert (dest / "rerp_auth_idam_impl").exists()
        assert (dest / "rerp_auth_idam_impl.sha256").exists()
        import hashlib

        assert (dest / "rerp_auth_idam_impl.sha256").read_text() == hashlib.sha256(
            b"binary"
        ).hexdigest()

    def test_all_copies_all_archs_that_exist(self, tmp_path: Path):
        from rerp_tooling.docker.copy_multiarch import run

        for triple in ["x86_64-unknown-linux-musl", "aarch64-unknown-linux-musl"]:
            src = tmp_path / "components" / "target" / triple / "release"
            src.mkdir(parents=True)
            (src / "rerp_auth_idam_impl").write_bytes(b"x")
        # arm7 missing
        assert run("auth", "idam", "all", tmp_path) == 0
        assert (
            tmp_path / "build_artifacts" / "auth_idam" / "amd64" / "rerp_auth_idam_impl"
        ).exists()
        assert (
            tmp_path / "build_artifacts" / "auth_idam" / "arm64" / "rerp_auth_idam_impl"
        ).exists()
        assert not (tmp_path / "build_artifacts" / "auth_idam" / "arm7").exists()
