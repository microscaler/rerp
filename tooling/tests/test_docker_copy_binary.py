"""Tests for rerp_tooling.docker.copy_binary (rerp docker copy-binary)."""

from pathlib import Path


class TestCopyBinary:
    def test_source_missing_returns_1(self, tmp_path: Path):
        from rerp_tooling.docker.copy_binary import run

        assert run(Path("missing"), Path("out/b"), "b", tmp_path) == 1

    def test_success_copies_chmod_and_writes_sha256(self, tmp_path: Path):
        from rerp_tooling.docker.copy_binary import run

        src = tmp_path / "src" / "bin"
        src.parent.mkdir(parents=True)
        src.write_bytes(b"binary content")
        dest = Path("out") / "bin"
        assert run(Path("src/bin"), dest, "bin", tmp_path) == 0
        out_bin = tmp_path / "out" / "bin"
        out_hash = tmp_path / "out" / "bin.sha256"
        assert out_bin.exists()
        assert out_bin.stat().st_mode & 0o111
        assert out_hash.exists()
        import hashlib

        assert out_hash.read_text() == hashlib.sha256(out_bin.read_bytes()).hexdigest()
