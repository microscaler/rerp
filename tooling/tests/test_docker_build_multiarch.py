"""Tests for rerp_tooling.docker.build_multiarch (rerp docker build-multiarch)."""

from pathlib import Path
from unittest.mock import MagicMock, patch


class TestBuildMultiarch:
    def test_returns_1_when_rerp_build_fails(self, tmp_path: Path):
        from rerp_tooling.docker.build_multiarch import run

        with patch("subprocess.run") as m:
            m.return_value = MagicMock(returncode=1)
            rc = run("auth", "idam", "img", "latest", False, tmp_path)
        assert rc == 1
        assert m.call_count >= 1
        assert m.call_args[0][0][:3] == ["tooling/.venv/bin/rerp", "build", "auth_idam"]
