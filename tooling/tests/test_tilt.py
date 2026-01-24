"""Tests for rerp_tooling.tilt (rerp tilt setup-kind-registry, setup-persistent-volumes, logs)."""

from pathlib import Path
from unittest.mock import MagicMock, patch


class TestSetupKindRegistry:
    def test_returns_0_when_network_and_registry_ok(self, tmp_path: Path):
        from rerp_tooling.tilt.setup_kind_registry import run

        with patch("subprocess.run") as m:
            # docker inspect (no container) -> fail; then we'd docker run. To avoid that, make inspect succeed (container exists)
            # docker inspect -f State.Running -> "true"
            # docker network inspect kind -> ok
            # docker inspect -f json .NetworkSettings.Networks.kind -> "{}" (not "null", so already connected)
            # kubectl cluster-info -> ok; kubectl apply -> ok
            m.side_effect = [
                MagicMock(returncode=0),
                MagicMock(returncode=0, stdout="true"),
                MagicMock(returncode=0),
                MagicMock(returncode=0, stdout="{}"),
                MagicMock(returncode=0),
                MagicMock(returncode=0),
            ]
            assert run(tmp_path) == 0

    def test_returns_1_when_kind_network_missing(self, tmp_path: Path):
        from rerp_tooling.tilt.setup_kind_registry import run

        with patch("subprocess.run") as m:
            m.side_effect = [
                MagicMock(returncode=0),
                MagicMock(returncode=0, stdout="true"),
                MagicMock(returncode=1),  # docker network inspect kind fails
            ]
            assert run(tmp_path) == 1


class TestSetupPersistentVolumes:
    def test_returns_1_when_kubectl_not_connected(self, tmp_path: Path):
        from rerp_tooling.tilt.setup_persistent_volumes import run

        with patch("subprocess.run") as m:
            m.return_value = MagicMock(returncode=1)
            assert run(tmp_path) == 1

    def test_returns_0_when_no_pv_files(self, tmp_path: Path):
        from rerp_tooling.tilt.setup_persistent_volumes import run

        with patch("subprocess.run") as m:
            m.return_value = MagicMock(returncode=0, stdout="")
            assert run(tmp_path) == 0


class TestLogs:
    def test_returns_1_when_tilt_not_in_path(self, tmp_path: Path):
        from rerp_tooling.tilt.logs import run

        with patch("shutil.which", return_value=None):
            assert run("general-ledger", tmp_path) == 1

    def test_returns_1_when_tilt_get_fails(self, tmp_path: Path):
        from rerp_tooling.tilt.logs import run

        with patch("shutil.which", return_value="/usr/bin/tilt"), patch("subprocess.run") as m:
            m.side_effect = [
                MagicMock(returncode=1),
            ]
            assert run("general-ledger", tmp_path) == 1

    def test_returns_1_when_component_not_found(self, tmp_path: Path):
        from rerp_tooling.tilt.logs import run

        with patch("shutil.which", return_value="/usr/bin/tilt"), patch("subprocess.run") as m:
            m.side_effect = [
                MagicMock(returncode=0, stdout='[{"name":"other"}]'),
            ]
            assert run("general-ledger", tmp_path) == 1

    def test_returns_tilt_logs_exit_code_when_ok(self, tmp_path: Path):
        from rerp_tooling.tilt.logs import run

        with patch("shutil.which", return_value="/usr/bin/tilt"), patch("subprocess.run") as m:
            m.side_effect = [
                MagicMock(returncode=0, stdout='[{"name":"general-ledger"}]'),
                MagicMock(returncode=0),
            ]
            assert run("general-ledger", tmp_path) == 0
            assert m.call_count == 2
            assert m.call_args_list[1][0][0][:2] == ["tilt", "logs"]


class TestSetup:
    def test_returns_1_when_docker_not_in_path(self, tmp_path: Path):
        from rerp_tooling.tilt.setup import run

        with patch("shutil.which") as m:
            m.return_value = (
                None  # docker and tilt which checks both fail if we return None for "docker" first
            )
            # for cmd in ["docker","tilt"]: first which("docker") -> None, return 1
            assert run(tmp_path) == 1

    def test_returns_1_when_tilt_not_in_path(self, tmp_path: Path):
        from rerp_tooling.tilt.setup import run

        with patch("shutil.which") as m:
            # which("docker") x4 in volume loop, which("docker") then which("tilt") in for cmd
            m.side_effect = ["/usr/bin/docker"] * 5 + [None]  # tilt -> None
            with patch("subprocess.run", return_value=MagicMock(returncode=0)):
                assert run(tmp_path) == 1

    def test_returns_0_creates_dirs_and_volumes_mocked(self, tmp_path: Path):
        from rerp_tooling.tilt.setup import run

        with patch("shutil.which", return_value="/usr/bin/docker"):
            with patch("subprocess.run", return_value=MagicMock(returncode=0)):
                assert run(tmp_path) == 0
            for p in [
                "docker/prometheus",
                "openapi/accounting",
                "microservices/accounting",
                "k8s/data",
            ]:
                assert (tmp_path / p).is_dir()


class TestTeardown:
    def test_returns_0_with_mocked_subprocess(self, tmp_path: Path):
        from rerp_tooling.tilt.teardown import run

        with patch("subprocess.run", return_value=MagicMock(returncode=0)):
            assert run(tmp_path, remove_images=False, remove_volumes=False, system_prune=False) == 0

    def test_returns_0_with_remove_flags_mocked(self, tmp_path: Path):
        from rerp_tooling.tilt.teardown import run

        with patch("subprocess.run", return_value=MagicMock(returncode=0)):
            assert run(tmp_path, remove_images=True, remove_volumes=True, system_prune=True) == 0

    def test_uses_tilt_service_names_for_containers_and_images(self, tmp_path: Path):
        from rerp_tooling.tilt.teardown import run

        (tmp_path / "openapi" / "acct").mkdir(parents=True)
        (tmp_path / "openapi" / "acct" / "bff-suite-config.yaml").write_text(
            "bff_service_name: bff\nservices:\n  edi: {}\n  financial-reports: {}"
        )
        with patch("subprocess.run", return_value=MagicMock(returncode=0)) as m:
            run(tmp_path, remove_images=True, remove_volumes=False, system_prune=False)
        calls = [c[0][0] for c in m.call_args_list]
        # Container stop/rm: rerp-{name}-dev for edi, financial-reports, bff
        assert ["docker", "stop", "rerp-bff-dev"] in [
            c[:3] for c in calls if c[:2] == ["docker", "stop"] and len(c) > 2
        ]
        assert ["docker", "stop", "rerp-edi-dev"] in [
            c[:3] for c in calls if c[:2] == ["docker", "stop"] and len(c) > 2
        ]
        assert ["docker", "stop", "rerp-financial-reports-dev"] in [
            c[:3] for c in calls if c[:2] == ["docker", "stop"] and len(c) > 2
        ]
        # Image rmi: rerp-accounting-{name}:latest and localhost:5001/rerp-accounting-{name}:tilt
        rmi_calls = [c for c in calls if c[:2] == ["docker", "rmi"] and len(c) > 2]
        rmi_args = [c[2] for c in rmi_calls]
        assert "rerp-accounting-bff:latest" in rmi_args
        assert "rerp-accounting-edi:latest" in rmi_args
        assert "localhost:5001/rerp-accounting-financial-reports:tilt" in rmi_args
