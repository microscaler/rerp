"""TDD: tests for rerp_tooling.openapi.validate (rerp openapi validate)."""

from pathlib import Path

import yaml


def _write_spec(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as f:
        yaml.dump(data, f, default_flow_style=False, sort_keys=False)


class TestValidateSpecs:
    """Unit tests for validate_specs(openapi_dir) -> list[tuple[Path, Exception]]."""

    def test_openapi_dir_missing_returns_empty(self, tmp_path: Path):
        from rerp_tooling.openapi import validate_specs

        missing = tmp_path / "nonexistent"
        assert not missing.exists()
        errs = validate_specs(missing)
        assert errs == []

    def test_openapi_dir_empty_returns_empty(self, tmp_openapi_dir):
        from rerp_tooling.openapi import validate_specs

        _, openapi_dir = tmp_openapi_dir
        errs = validate_specs(openapi_dir)
        assert errs == []

    def test_one_valid_spec_returns_empty(self, tmp_openapi_dir):
        from rerp_tooling.openapi import validate_specs

        _, openapi_dir = tmp_openapi_dir
        (openapi_dir / "acct" / "gl").mkdir(parents=True)
        _write_spec(
            openapi_dir / "acct" / "gl" / "openapi.yaml",
            {"openapi": "3.0.3", "info": {"title": "x", "version": "1"}, "paths": {}},
        )
        errs = validate_specs(openapi_dir)
        assert errs == []

    def test_one_invalid_yaml_returns_one_error(self, tmp_openapi_dir):
        from rerp_tooling.openapi import validate_specs

        _, openapi_dir = tmp_openapi_dir
        (openapi_dir / "bad").mkdir(parents=True)
        spec = openapi_dir / "bad" / "openapi.yaml"
        spec.parent.mkdir(parents=True, exist_ok=True)
        spec.write_text("openapi: 3.0.3\n  bad: indent\n")
        errs = validate_specs(openapi_dir)
        assert len(errs) == 1
        assert errs[0][0] == spec
        assert errs[0][1] is not None

    def test_one_non_dict_yaml_returns_one_error(self, tmp_openapi_dir):
        from rerp_tooling.openapi import validate_specs

        _, openapi_dir = tmp_openapi_dir
        (openapi_dir / "list").mkdir(parents=True)
        _write_spec(openapi_dir / "list" / "openapi.yaml", [1, 2, 3])  # type: ignore[arg-type]
        errs = validate_specs(openapi_dir)
        assert len(errs) == 1
        assert "list" in str(errs[0][0])

    def test_two_specs_one_invalid_returns_one_error(self, tmp_openapi_dir):
        from rerp_tooling.openapi import validate_specs

        _, openapi_dir = tmp_openapi_dir
        (openapi_dir / "a").mkdir(parents=True)
        (openapi_dir / "b").mkdir(parents=True)
        _write_spec(
            openapi_dir / "a" / "openapi.yaml",
            {"openapi": "3.0.3", "info": {"title": "a", "version": "1"}, "paths": {}},
        )
        (openapi_dir / "b" / "openapi.yaml").write_text("{ key:   # unclosed flow mapping\n")
        errs = validate_specs(openapi_dir)
        assert len(errs) == 1
        assert "b" in str(errs[0][0])

    def test_nested_rglob_finds_openapi_yaml(self, tmp_openapi_dir):
        from rerp_tooling.openapi import validate_specs

        _, openapi_dir = tmp_openapi_dir
        (openapi_dir / "x" / "y" / "z").mkdir(parents=True)
        _write_spec(
            openapi_dir / "x" / "y" / "z" / "openapi.yaml",
            {"openapi": "3.0.3", "info": {"title": "z", "version": "1"}, "paths": {}},
        )
        errs = validate_specs(openapi_dir)
        assert errs == []
