"""Tests for rerp docker unpack-build-bins (delegates to brrtrouter_tooling.docker.unpack_build_bins)."""

import zipfile
from pathlib import Path

import pytest

pytest.importorskip("brrtrouter_tooling")


class TestUnpackBuildBins:
    def test_missing_input_dir_returns_1(self, tmp_path: Path):
        from brrtrouter_tooling.docker.unpack_build_bins import run

        assert run(tmp_path / "nonexistent", tmp_path) == 1

    def test_no_zips_returns_1(self, tmp_path: Path):
        from brrtrouter_tooling.docker.unpack_build_bins import run

        (tmp_path / "empty").mkdir()
        assert run(tmp_path / "empty", tmp_path) == 1

    def test_extracts_into_microservices_target(self, tmp_path: Path):
        from brrtrouter_tooling.docker.unpack_build_bins import run

        ind = tmp_path / "zips"
        ind.mkdir()
        triple = "x86_64-unknown-linux-musl"
        with zipfile.ZipFile(ind / "rerp-binaries-amd64.zip", "w") as z:
            z.writestr(f"{triple}/release/rerp_ai_core_impl", b"\x7fELF")
        assert run(ind, tmp_path) == 0
        dest = tmp_path / "microservices" / "target" / triple / "release" / "rerp_ai_core_impl"
        assert dest.exists()
        assert dest.stat().st_mode & 0o111
