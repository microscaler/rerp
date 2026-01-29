"""TDD: tests for rerp_tooling.openapi.fix_operation_id (rerp openapi fix-operation-id-casing)."""

from pathlib import Path


class TestIsSnakeCase:
    def test_empty_false(self):
        from rerp_tooling.openapi import is_snake_case

        assert is_snake_case("") is False

    def test_starts_upper_false(self):
        from rerp_tooling.openapi import is_snake_case

        assert is_snake_case("ListPets") is False
        assert is_snake_case("GetUser") is False

    def test_valid_snake_true(self):
        from rerp_tooling.openapi import is_snake_case

        assert is_snake_case("list_pets") is True
        assert is_snake_case("get_user") is True
        assert is_snake_case("create_asset") is True
        assert is_snake_case("abc_123") is True
        assert is_snake_case("_private") is True
        assert is_snake_case("a") is True


class TestToSnakeCase:
    def test_camel_case(self):
        from rerp_tooling.openapi import to_snake_case

        assert to_snake_case("listPets") == "list_pets"
        assert to_snake_case("getUser") == "get_user"
        assert to_snake_case("createAsset") == "create_asset"

    def test_kebab_and_spaces(self):
        from rerp_tooling.openapi import to_snake_case

        assert to_snake_case("kebab-case") == "kebab_case"
        assert " " not in to_snake_case("get User")

    def test_already_snake_unchanged(self):
        from rerp_tooling.openapi import to_snake_case

        assert to_snake_case("list_pets") == "list_pets"


class TestFindOpenapiFiles:
    def test_finds_yaml_and_yml(self, tmp_path: Path):
        from rerp_tooling.openapi import find_openapi_files

        (tmp_path / "a").mkdir()
        (tmp_path / "b" / "c").mkdir(parents=True)
        (tmp_path / "a" / "openapi.yaml").write_text("x: 1")
        (tmp_path / "b" / "c" / "openapi.yml").write_text("y: 2")
        out = find_openapi_files(tmp_path)
        assert len(out) == 2
        assert any("openapi.yaml" in str(p) for p in out)
        assert any("openapi.yml" in str(p) for p in out)

    def test_empty_dir_returns_empty(self, tmp_path: Path):
        from rerp_tooling.openapi import find_openapi_files

        assert find_openapi_files(tmp_path) == []


class TestProcessFile:
    def test_converts_camel_and_writes(self, tmp_path: Path):
        from rerp_tooling.openapi import process_file

        spec = tmp_path / "openapi.yaml"
        spec.write_text("paths:\n  /pets:\n    get:\n      operationId: listPets\n")
        n, ch = process_file(spec, dry_run=False)
        assert n == 1
        assert ch == [(3, "listPets", "list_pets")]
        assert "list_pets" in spec.read_text()
        assert "listPets" not in spec.read_text()

    def test_dry_run_does_not_write(self, tmp_path: Path):
        from rerp_tooling.openapi import process_file

        spec = tmp_path / "openapi.yaml"
        orig = "paths:\n  /pets:\n    get:\n      operationId: listPets\n"
        spec.write_text(orig)
        n, _ = process_file(spec, dry_run=True)
        assert n == 1
        assert spec.read_text() == orig

    def test_leaves_snake_case_alone(self, tmp_path: Path):
        from rerp_tooling.openapi import process_file

        spec = tmp_path / "openapi.yaml"
        orig = "paths:\n  /pets:\n    get:\n      operationId: list_pets\n"
        spec.write_text(orig)
        n, ch = process_file(spec, dry_run=False)
        assert n == 0
        assert ch == []
        assert spec.read_text() == orig

    def test_quoted_operation_id(self, tmp_path: Path):
        from rerp_tooling.openapi import process_file

        spec = tmp_path / "openapi.yaml"
        spec.write_text('paths:\n  /x:\n    get:\n      operationId: "getStuff"\n')
        n, ch = process_file(spec, dry_run=False)
        assert n == 1
        assert ch[0][1] == "getStuff" and ch[0][2] == "get_stuff"
        assert "get_stuff" in spec.read_text()


class TestRun:
    def test_missing_dir_returns_0_0(self, tmp_path: Path):
        from rerp_tooling.openapi import fix_operation_id_run

        d = tmp_path / "missing"
        assert not d.exists()
        t, u = fix_operation_id_run(d, dry_run=False, verbose=False)
        assert t == 0 and u == 0

    def test_no_openapi_files_returns_0_0(self, tmp_path: Path):
        from rerp_tooling.openapi import fix_operation_id_run

        tmp_path.mkdir(exist_ok=True)
        (tmp_path / "readme.txt").write_text("hi")
        t, u = fix_operation_id_run(tmp_path, dry_run=False, verbose=False)
        assert t == 0 and u == 0

    def test_one_file_with_changes_returns_total_and_touched(self, tmp_path: Path):
        from rerp_tooling.openapi import fix_operation_id_run

        (tmp_path / "svc").mkdir()
        (tmp_path / "svc" / "openapi.yaml").write_text(
            "paths:\n  /a:\n    get:\n      operationId: getA\n  /b:\n    post:\n      operationId: createB\n"
        )
        t, u = fix_operation_id_run(tmp_path, dry_run=False, verbose=False, rel_to=tmp_path)
        assert t == 2 and u == 1
        text = (tmp_path / "svc" / "openapi.yaml").read_text()
        assert "get_a" in text and "create_b" in text
