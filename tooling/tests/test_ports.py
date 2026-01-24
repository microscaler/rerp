"""Tests for rerp_tooling.ports (PortRegistry, validate, reconcile, fix_duplicates)."""

from pathlib import Path


class TestPortRegistry:
    def test_load_empty_creates_default(self, tmp_path: Path):
        from rerp_tooling.ports import PortRegistry

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        assert reg.registry["version"] == "1.0"
        assert "assignments" in reg.registry
        assert reg.registry["assignments"] == {}

    def test_assign_and_get(self, tmp_path: Path):
        from rerp_tooling.ports import PortRegistry

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        port, is_new = reg.assign_port("svc-a")
        assert is_new is True
        assert port >= 8001
        assert reg.get_port("svc-a") == port

    def test_assign_idempotent_without_force(self, tmp_path: Path):
        from rerp_tooling.ports import PortRegistry

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        p1, n1 = reg.assign_port("x")
        p2, n2 = reg.assign_port("x")
        assert p1 == p2
        assert n1 is True
        assert n2 is False

    def test_list_assignments(self, tmp_path: Path):
        from rerp_tooling.ports import PortRegistry

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        reg.assign_port("a")
        reg.assign_port("b")
        lst = reg.list_assignments()
        assert set(lst.keys()) == {"a", "b"}

    def test_release_port(self, tmp_path: Path):
        from rerp_tooling.ports import PortRegistry

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        port, _ = reg.assign_port("x")
        got = reg.release_port("x")
        assert got == port
        assert reg.get_port("x") is None


class TestValidate:
    def test_validate_no_sources_ok(self, tmp_path: Path, capsys):
        from rerp_tooling.ports import PortRegistry, validate

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        rc = validate(reg, tmp_path, json_out=False)
        out, _ = capsys.readouterr()
        assert rc == 0
        assert "No port conflicts" in out or "✅" in out

    def test_validate_json_out(self, tmp_path: Path, capsys):
        from rerp_tooling.ports import PortRegistry, validate

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        rc = validate(reg, tmp_path, json_out=True)
        out, _ = capsys.readouterr()
        assert "ok" in out and "errors" in out
        assert rc == 0


class TestReconcile:
    def test_reconcile_adds_helm_only_service(self, tmp_path: Path, capsys):
        from rerp_tooling.ports import PortRegistry, reconcile

        (tmp_path / "helm" / "rerp-microservice" / "values").mkdir(parents=True)
        (tmp_path / "helm" / "rerp-microservice" / "values" / "new-svc.yaml").write_text(
            "service:\n  name: new-svc\n  port: 8010\n"
        )
        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        rc = reconcile(reg, tmp_path, update_configs=False)
        out, _ = capsys.readouterr()
        assert rc == 0
        assert "Added to registry" in out or "new-svc" in out
        assert reg.get_port("new-svc") == 8010


class TestFixDuplicates:
    def test_fix_duplicates_none(self, tmp_path: Path, capsys):
        from rerp_tooling.ports import PortRegistry, fix_duplicates

        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        rc = fix_duplicates(reg, tmp_path, dry_run=False)
        out, _ = capsys.readouterr()
        assert rc == 0
        assert "No duplicate" in out

    def test_fix_duplicates_dry_run(self, tmp_path: Path, capsys):
        from rerp_tooling.ports import PortRegistry, fix_duplicates

        (tmp_path / "helm" / "rerp-microservice" / "values").mkdir(parents=True)
        (tmp_path / "helm" / "rerp-microservice" / "values" / "a.yaml").write_text(
            "service:\n  name: a\n  port: 8001\n"
        )
        (tmp_path / "helm" / "rerp-microservice" / "values" / "b.yaml").write_text(
            "service:\n  name: b\n  port: 8001\n"
        )
        reg_path = tmp_path / "reg.json"
        reg = PortRegistry(reg_path, tmp_path)
        rc = fix_duplicates(reg, tmp_path, dry_run=True)
        out, _ = capsys.readouterr()
        assert rc == 0
        assert "duplicate" in out.lower() or "Resolving" in out
