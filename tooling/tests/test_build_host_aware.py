"""TDD: tests for rerp_tooling.build.host_aware (rerp build)."""

from pathlib import Path
from unittest.mock import patch

import pytest


class TestArchTargets:
    def test_arch_targets_mapping(self):
        from rerp_tooling.build.host_aware import ARCH_TARGETS

        assert ARCH_TARGETS["amd64"] == "x86_64-unknown-linux-musl"
        assert ARCH_TARGETS["arm64"] == "aarch64-unknown-linux-musl"
        assert ARCH_TARGETS["arm7"] == "armv7-unknown-linux-musleabihf"


class TestDetectHostArchitecture:
    def test_x86_64_returns_amd64(self):
        from rerp_tooling.build.host_aware import detect_host_architecture

        with patch("platform.machine", return_value="x86_64"):
            assert detect_host_architecture() == "amd64"

    def test_amd64_returns_amd64(self):
        from rerp_tooling.build.host_aware import detect_host_architecture

        with patch("platform.machine", return_value="amd64"):
            assert detect_host_architecture() == "amd64"

    def test_aarch64_returns_arm64(self):
        from rerp_tooling.build.host_aware import detect_host_architecture

        with patch("platform.machine", return_value="aarch64"):
            assert detect_host_architecture() == "arm64"

    def test_arm64_returns_arm64(self):
        from rerp_tooling.build.host_aware import detect_host_architecture

        with patch("platform.machine", return_value="arm64"):
            assert detect_host_architecture() == "arm64"

    def test_unknown_falls_back_to_amd64(self):
        from rerp_tooling.build.host_aware import detect_host_architecture

        with patch("platform.machine", return_value="unknown"):
            assert detect_host_architecture() == "amd64"


class TestShouldUseZigbuild:
    def test_darwin_returns_true(self):
        from rerp_tooling.build.host_aware import should_use_zigbuild

        with (
            patch("platform.system", return_value="Darwin"),
            patch("platform.machine", return_value="x86_64"),
        ):
            assert should_use_zigbuild() is True

    def test_linux_x86_64_returns_false(self):
        from rerp_tooling.build.host_aware import should_use_zigbuild

        with (
            patch("platform.system", return_value="Linux"),
            patch("platform.machine", return_value="x86_64"),
        ):
            assert should_use_zigbuild() is False

    def test_linux_arm64_returns_true(self):
        from rerp_tooling.build.host_aware import should_use_zigbuild

        with (
            patch("platform.system", return_value="Linux"),
            patch("platform.machine", return_value="aarch64"),
        ):
            assert should_use_zigbuild() is True


class TestShouldUseCross:
    def test_env_set_returns_true(self, monkeypatch):
        from rerp_tooling.build.host_aware import should_use_cross

        monkeypatch.setenv("RERP_USE_CROSS", "1")
        assert should_use_cross() is True

    def test_env_unset_returns_false(self, monkeypatch):
        from rerp_tooling.build.host_aware import should_use_cross

        monkeypatch.delenv("RERP_USE_CROSS", raising=False)
        assert should_use_cross() is False


class TestDetermineArchitectures:
    def test_all_returns_three_archs(self):
        from rerp_tooling.build.host_aware import _determine_architectures

        assert _determine_architectures("all") == ["amd64", "arm64", "arm7"]

    def test_amd64_returns_single(self):
        from rerp_tooling.build.host_aware import _determine_architectures

        assert _determine_architectures("amd64") == ["amd64"]

    def test_arm64_returns_single(self):
        from rerp_tooling.build.host_aware import _determine_architectures

        assert _determine_architectures("arm64") == ["arm64"]

    def test_arm7_returns_single(self):
        from rerp_tooling.build.host_aware import _determine_architectures

        assert _determine_architectures("arm7") == ["arm7"]

    def test_none_uses_host(self):
        from rerp_tooling.build.host_aware import _determine_architectures

        with patch("rerp_tooling.build.host_aware.detect_host_architecture", return_value="amd64"):
            assert _determine_architectures(None) == ["amd64"]

    def test_unknown_exits(self):
        from rerp_tooling.build.host_aware import _determine_architectures

        with pytest.raises(SystemExit):
            _determine_architectures("unknown")


class TestRun:
    def test_run_returns_one_when_microservices_missing(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.build.host_aware import run

        monkeypatch.setenv("RERP_USE_CROSS", "1")  # skip rustup in _build_for_arch
        # No microservices/Cargo.toml
        rc = run("workspace", arch="amd64", project_root=tmp_path)
        assert rc == 1

    def test_run_returns_one_when_service_crate_missing(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.build.host_aware import run

        monkeypatch.setenv("RERP_USE_CROSS", "1")
        (tmp_path / "microservices").mkdir()
        (tmp_path / "microservices" / "Cargo.toml").write_text("[workspace]\n")
        # auth_idam → microservices/auth/idam/impl must exist for _build_service
        rc = run("auth_idam", arch="amd64", project_root=tmp_path)
        assert rc == 1


class TestBuildMicroservices:
    def test_build_microservice_unknown_name_returns_1(self, tmp_path: Path):
        from rerp_tooling.build.microservices import build_microservice

        rc = build_microservice(tmp_path, "unknown-svc", release=False)
        assert rc == 1

    def test_build_microservices_workspace_missing_manifest_returns_1(self, tmp_path: Path):
        from rerp_tooling.build.microservices import build_microservices_workspace

        rc = build_microservices_workspace(tmp_path, "amd64", release=False)
        assert rc == 1

    def test_run_accounting_gen_if_missing_noop_when_probe_exists(self, tmp_path: Path):
        from rerp_tooling.build.microservices import run_accounting_gen_if_missing

        (tmp_path / "microservices" / "accounting" / "general-ledger").mkdir(parents=True)
        (tmp_path / "microservices" / "accounting" / "general-ledger" / "Cargo.toml").write_text("")
        run_accounting_gen_if_missing(tmp_path)  # no raise, no subprocess

    def test_build_microservices_workspace_success_mocked(self, tmp_path: Path, monkeypatch):
        from rerp_tooling.build.microservices import build_microservices_workspace

        (tmp_path / "microservices").mkdir(parents=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text("[workspace]\n")
        (tmp_path / "microservices" / "accounting" / "general-ledger").mkdir(parents=True)
        (tmp_path / "microservices" / "accounting" / "general-ledger" / "Cargo.toml").write_text("")
        monkeypatch.setenv("RERP_USE_CROSS", "")
        with patch(
            "rerp_tooling.build.microservices.build_workspace_with_options",
            return_value=0,
        ) as m_build:
            rc = build_microservices_workspace(tmp_path, "amd64", release=False)
        assert rc == 0
        assert m_build.called

    def test_build_microservices_workspace_arm7_disables_jemalloc(
        self, tmp_path: Path, monkeypatch
    ):
        """armv7 is passed through to brrtrouter_tooling.build (which disables jemalloc)."""
        from rerp_tooling.build.microservices import build_microservices_workspace

        (tmp_path / "microservices").mkdir(parents=True)
        (tmp_path / "microservices" / "Cargo.toml").write_text("[workspace]\n")
        (tmp_path / "microservices" / "accounting" / "general-ledger").mkdir(parents=True)
        (tmp_path / "microservices" / "accounting" / "general-ledger" / "Cargo.toml").write_text("")
        monkeypatch.setenv("RERP_USE_CROSS", "1")
        with patch(
            "rerp_tooling.build.microservices.build_workspace_with_options",
            return_value=0,
        ) as m_build:
            rc = build_microservices_workspace(tmp_path, "arm7", release=True)
        assert rc == 0
        assert m_build.called
        kwargs = m_build.call_args[1]
        assert kwargs.get("arch") == "arm7"
        assert kwargs.get("release") is True
