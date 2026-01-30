# BRRTRouter ↔ RERP dependency audit

**Goal:** BRRTRouter is the single source of truth and exports a complete public API. RERP imports from that API normally and has no workarounds.

**Current state:** RERP contains workarounds so CI passes when BRRTRouter **main** (GitHub) is behind. Your **local** BRRTRouter already has the code and exports; the gap is BRRTRouter main on GitHub.

---

## 1. BRRTRouter: what must be on `main`

| BRRTRouter module / symbol | Purpose | Status in local BRRTRouter | Action for BRRTRouter main |
|----------------------------|---------|----------------------------|----------------------------|
| **ci** | | | |
| `fix_all_impl_dependencies` | Update impl Cargo deps | ✅ In `ci/__init__.py` | Ensure merged to main |
| `update_impl_cargo_dependencies` | Same | ✅ In `ci/__init__.py` | Ensure merged to main |
| **gen** | | | |
| `regenerate_service` | Regenerate one service from OpenAPI | ✅ In `gen/__init__.py` (from gen.regenerate) | Ensure merged to main |
| `regenerate_suite_services` | Regenerate suite | ✅ In `gen/__init__.py` | Ensure merged to main |
| `run_gen_if_missing_for_suite` | Gen-if-missing for build | ✅ In `gen/__init__.py` | Ensure merged to main |
| **openapi** | | | |
| `check_number_fields` | Decimal/number format checks | ✅ In `openapi/__init__.py` (from check_decimal_formats) | Ensure merged to main |
| **docker** | | | |
| `docker.copy_artifacts` module | `run()`, `validate_build_artifacts()` | ✅ Module exists; exported in `docker/__init__.py` | Ensure merged to main |
| **docker.copy_multiarch** | | | |
| Return 0 when `arch="all"` and ≥1 arch copied | Lenient “all” behavior | ✅ Returns 0 when `any_ok` | Ensure merged to main |

**Single action for BRRTRouter:** Merge/push the branch that contains the above (ci, gen, openapi, docker exports and copy_multiarch behavior) to **main**, so RERP CI’s `ref: main` checkout gets a complete API.

**Docker parameterization (BRRTRouter):** All docker helpers live in BRRTRouter and are parameterized so RERP (and other projects) pass project-specific values: `build_base` (base_image_name, docker_dir, tag_kind_registry, tag_dockerhub), `build_image_simple` (kind_cluster_name, base_image_name, dockerfile), `build_multiarch` (build_cmd, base_image_name, docker_dir, base_image_local), `copy_artifacts` (package_names, binary_names, workspace_dir), `generate_dockerfile` (binary_name_pattern), `unpack_build_bins` (workspace_dir, zip_prefix), `copy_multiarch` (binary_name_fn, workspace_dir). RERP keeps only a thin `docker/copy_artifacts.py` wrapper and `cli/docker.py` that call BRRTRouter with RERP constants (e.g. rerp-base, rerp/base, kind cluster "rerp", rerp-binaries-*, tooling/.venv/bin/rerp build ...).

**Gen package name (BRRTRouter):** `brrtrouter-gen generate` accepts `--package-name` for generated gen crate `[package].name`. BRRTRouter Python: `call_brrtrouter_generate(..., package_name=...)`, `regenerate_service(..., package_name=...)`, `regenerate_suite_services(..., package_name_for_service=...)`, `run_gen_if_missing_for_suite(..., package_name_for_service=...)`. RERP derives gen package names **on the fly** from `get_package_names(project_root)` (discovery from `openapi/{suite}/{service}/openapi.yaml`); no hardcoded `PACKAGE_NAMES`. **All call sites use this flow:** Tiltfile uses `rerp gen suite accounting --service <name>` (no raw brrtrouter-gen); bootstrap uses `call_brrtrouter_generate(..., package_name=...)` with service_name so new crates get the correct name from the start.

**Dependencies config (brrtrouter-dependencies.toml):** BRRTRouter accepts `--dependencies-config` for additional gen Cargo.toml deps. When the OpenAPI spec uses decimal/money (`format: decimal` or `format: money`), brrtrouter-gen **auto-creates** `brrtrouter-dependencies.toml` alongside the spec if it does not exist (with `[conditional] rust_decimal = { detect = "rust_decimal::Decimal", workspace = true }`). Manual creation is only needed if you want to customize the file; existing files are never overwritten. BRRTRouter `regenerate_service` still sets `deps_config_path` when the file exists; RERP can rely on gen creating the file on first run.


---

## 2. RERP: workarounds to remove (after BRRTRouter main is fixed)

| RERP location | Current workaround | Correct behavior after BRRTRouter main is fixed |
|---------------|--------------------|--------------------------------------------------|
| **ci/__init__.py** | `try/except`: set `fix_all_impl_dependencies` and `update_impl_cargo_dependencies` to `None` on ImportError | Import both from `brrtrouter_tooling.ci` in one block; remove try/except |
| **openapi/__init__.py** | `try/except`: set `check_number_fields = None` on ImportError | Import `check_number_fields` with the rest from `brrtrouter_tooling.openapi`; remove try/except |
| **gen/__init__.py** | `try/except`: set `regenerate_service` / `regenerate_suite_services` to `None` when `rerp_tooling.gen.regenerate` fails to load | Import `regenerate_service`, `regenerate_suite_services` from `brrtrouter_tooling.gen`; remove try/except |
| **gen/regenerate.py** | Imports from `brrtrouter_tooling.gen.regenerate` (submodule) | Can keep submodule import, or switch to `from brrtrouter_tooling.gen import ...` for consistency |
| **build/microservices.py** | Lazy import: `from brrtrouter_tooling.gen.regenerate import run_gen_if_missing_for_suite` inside function | Top-level: `from brrtrouter_tooling.gen import run_gen_if_missing_for_suite` |
| **cli/docker.py** | Lazy import of `copy_artifacts` (run, validate_build_artifacts) inside `run_docker()` branches | Top-level: `from brrtrouter_tooling.docker.copy_artifacts import run as run_copy_artifacts_brt` and `validate_build_artifacts as validate_build_artifacts_brt` |
| **cli/gen.py** | Lazy import of `regenerate_service` / `regenerate_suite_services` inside `run_gen()`; exit 1 if None | Top-level: `from rerp_tooling.gen.regenerate import regenerate_service, regenerate_suite_services` (or from `rerp_tooling.gen`); remove None check |
| **tests/test_build_host_aware.py** | `TestBuildMicroservices.setup_method`: `pytest.importorskip("brrtrouter_tooling.gen.regenerate")` | Remove `setup_method` and importorskip; tests always run |
| **tests/test_docker_copy_artifacts.py** | `_skip_if_no_brt_copy_artifacts()` in every test; `pytest.importorskip("brrtrouter_tooling.docker.copy_artifacts")` | Remove helper and all calls; tests always run |
| **tests/test_docker_copy_multiarch.py** | `test_all_copies_all_archs_that_exist`: if `rc != 0` then `pytest.skip(...)` | Remove skip; assert `rc == 0` only |

---

## 3. RERP: keep as-is (not workarounds)

| Item | Reason |
|------|--------|
| **build/constants.py** | Re-exports `get_package_names`, `get_binary_names`, `get_service_ports` from discovery; **no hardcoding** — all derived from openapi layout and specs. |
| **discovery/services.py** | `get_package_names(project_root)`, `get_binary_names(project_root)`, `get_service_ports(project_root)` from `openapi/{suite}/{service}/openapi.yaml` and port sources (openapi servers, bff-suite-config, helm). |
| **ci/fix_cargo_paths.py** | RERP wrapper that passes RERP gen-crate config to `brrtrouter_tooling.ci`; project-specific. |
| **docker/copy_artifacts.py** | Thin wrapper: `run`/`validate_build_artifacts` call `brrtrouter_tooling.docker.copy_artifacts` with discovery-derived `get_package_names(project_root)`/`get_binary_names(project_root)`. All other docker helpers live in BRRTRouter; RERP `cli/docker.py` calls them with RERP params (image names, kind cluster, zip prefix, build_cmd). |
| **gen/regenerate.py** | RERP wrapper that wires RERP `fix_cargo_paths` into BRRTRouter regenerate; project-specific. |

---

## 4. Execution order

1. **BRRTRouter:** Merge the branch that has the full API (ci, gen, openapi, docker exports + copy_multiarch behavior) to **main**.
2. **RERP:** Remove the workarounds in section 2 (in any order; can be one PR).
3. **CI:** Keep `ref: main` for BRRTRouter checkout; no need to pin a commit once main is complete.

---

## 5. Quick reference: RERP imports from BRRTRouter

| RERP imports | BRRTRouter source |
|--------------|-------------------|
| ci: compare_versions, find_cargo_tomls, find_matches, get_latest_tag, patch_file, run_cargo_update, run_get_latest_tag, run_is_tag, run_patch_brrtrouter, run_validate_version, run_validate_version_cli, validate_version | `brrtrouter_tooling.ci` |
| ci: fix_all_impl_dependencies, update_impl_cargo_dependencies | `brrtrouter_tooling.ci` (must be on main) |
| ci: fix_cargo_toml, run (fix_cargo_paths) | `brrtrouter_tooling.ci`, `brrtrouter_tooling.ci.fix_cargo_paths` |
| gen: call_brrtrouter_generate, call_brrtrouter_generate_stubs, find_brrtrouter | `brrtrouter_tooling.gen` |
| gen: regenerate_service, regenerate_suite_services, run_gen_if_missing_for_suite | `brrtrouter_tooling.gen` (must be on main) |
| openapi: check_openapi_dir, find_openapi_files, fix_impl_controller, fix_impl_controllers_dir, fix_operation_id_run, is_snake_case, process_file, to_snake_case, validate_specs | `brrtrouter_tooling.openapi` |
| openapi: check_number_fields | `brrtrouter_tooling.openapi` (must be on main) |
| docker: build_base, build_image_simple, build_multiarch, copy_binary, copy_multiarch, generate_dockerfile, unpack_build_bins | `brrtrouter_tooling.docker.*` (parameterized: base_image_name, docker_dir, kind_cluster_name, base_image_local, zip_prefix, build_cmd, tag_kind_registry, tag_dockerhub, workspace_dir, binary_name_pattern) |
| docker: copy_artifacts run, validate_build_artifacts | `brrtrouter_tooling.docker.copy_artifacts` (takes package_names, binary_names; RERP passes from build/constants) |
