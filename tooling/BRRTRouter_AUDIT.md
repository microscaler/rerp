# RERP Tooling → BRRTRouter Consistency Audit

This document identifies what in RERP tooling is **already delegated** to BRRTRouter, what is **thin wrapper** (project config only), and what **should move to BRRTRouter** so Microscaler projects share one implementation.

---

## Already delegated (no action)

| RERP module | Delegates to BRRTRouter | RERP-only |
|-------------|-------------------------|-----------|
| `ports.py` | `brrtrouter_tooling.ports` | — |
| `discovery/__init__.py` | `brrtrouter_tooling.discovery` | — |
| `openapi/__init__.py` | `brrtrouter_tooling.openapi` | — |
| `bff/__init__.py`, `bff/generate_system.py` | `brrtrouter_tooling.bff` | — |
| `bootstrap/__init__.py` | `brrtrouter_tooling.bootstrap` | — |
| `build/host_aware.py` | `brrtrouter_tooling.build.host_aware` | — |
| `ci/__init__.py` | `brrtrouter_tooling.ci` (plus RERP `fix_cargo_paths`) | — |
| `docker/copy_artifacts.py` | `brrtrouter_tooling.docker.copy_artifacts` | `BINARY_NAMES`, `PACKAGE_NAMES` (config) |
| `tilt/setup.py`, `setup_kind_registry.py`, `setup_persistent_volumes.py`, `logs.py` | `brrtrouter_tooling.tilt.*` | — |
| `cli/release.py` | `brrtrouter_tooling.release.*` | — |
| `cli/pre_commit.py` | `brrtrouter_tooling.pre_commit` | — |

---

## Thin wrappers (config only – acceptable)

These only pass RERP-specific constants; logic lives in BRRTRouter. No different implementations.

| RERP file | Passes to BRRTRouter | Purpose |
|-----------|----------------------|---------|
| `docker/build_base.py` | `base_image_name="rerp-base"` | Project image name |
| `docker/build_image_simple.py` | `base_image_name="rerp-base"`, `kind_cluster_name="rerp"` | Project image + Kind cluster |
| `docker/copy_multiarch.py` | `workspace_dir="microservices"` | Workspace path |
| `docker/generate_dockerfile.py` | `binary_name_pattern="rerp_{system}_{module}_impl"` | Binary naming |
| `docker/unpack_build_bins.py` | `workspace_dir="microservices"`, `zip_prefix="rerp-binaries-"` | Workspace + zip prefix |
| `tilt/teardown.py` | `tilt_service_names`, `_container_name`, `_image_rmi_list`, `STATIC_CONTAINERS`, `VOLUME_NAMES` | Project container/volume naming |

**Optional improvement:** Centralise these in a single RERP config (e.g. `RERP_DOCKER_CONFIG`, `RERP_TILT_CONFIG`) and have the CLI pass it to BRRTRouter, so there are fewer small wrapper files. Not required for consistency.

---

## Logic to move to BRRTRouter (avoid divergence)

### 1. **CI: gen crate name/version (RERP `ci/fix_cargo_paths.py`)**

**Current:** RERP calls `brrt_fix_cargo_toml`, then applies **RERP-specific** logic: gen crate name `rerp_accounting_{service}_gen`, version `0.1.3`, `[lib]` block. Other Microscaler projects will have similar but different names/versions.

**Risk:** Each project implements its own post-fix → inconsistent behaviour and duplication.

**Move to BRRTRouter:**

- In `brrtrouter_tooling.ci.fix_cargo_paths` (or a new helper), add optional **post-fix** support:
  - Either: `gen_crate_name_fn(project_root, cargo_toml_path) -> (name, version) | None` (if not None, apply name/version/\[lib\]).
  - Or: `gen_crate_config(suite: str, name_pattern: str, version: str)` passed into `fix_cargo_toml`.
- RERP then: `fix_cargo_toml(..., gen_crate_config=("accounting", "rerp_accounting_{service}_gen", "0.1.3"))` and removes the local gen-crate block from `fix_cargo_paths.py`.

**Result:** One implementation in BRRTRouter; projects only pass config.

---

### 2. **Gen: regenerate suite/service (RERP `gen/regenerate.py`)**

**Current:** RERP has `regenerate_service` / `regenerate_suite_services` with **hardcoded paths**: `openapi/{suite}/{service}/openapi.yaml`, `microservices/{suite}/{service}/gen`, BFF at `openapi/{suite}/openapi_bff.yaml`, and calls RERP’s `run_fix_cargo_paths` after each generate.

**Risk:** Another Microscaler project will reimplement the same flow with different paths or naming → divergence.

**Move to BRRTRouter:**

- In `brrtrouter_tooling.gen`, add:
  - `regenerate_service(project_root, suite, service_name, *, spec_path_fn=None, output_dir_fn=None, deps_config_path_fn=None, fix_cargo_paths_fn=None)`  
    Default path logic: `openapi/{suite}/{service}/openapi.yaml`, `microservices/{suite}/{service}/gen`, etc. Allow overrides via callbacks.
  - `regenerate_suite_services(project_root, suite, service_names, ...)` that loops and calls `regenerate_service`.
- BFF vs non-BFF can be detected by path existence or a small helper (e.g. `is_bff_spec(project_root, suite, service_name)`).
- RERP then: calls `brrtrouter_tooling.gen.regenerate_service` / `regenerate_suite_services` with RERP’s `fix_cargo_paths_fn` (or gen_crate config from item 1). RERP’s `gen/regenerate.py` becomes a thin wrapper (paths + fix_cargo_paths wiring).

**Result:** One “regenerate from OpenAPI” flow in BRRTRouter; projects only pass path and post-gen fix.

---

### 3. **Build: gen-if-missing for suite (RERP `build/microservices.py`)**

**Current:** RERP has `run_accounting_gen_if_missing(project_root)`: discovers services for suite `"accounting"`, calls `call_brrtrouter_generate` per service, then `run_fix_cargo_paths`. Used as `gen_if_missing_callback` for `build_workspace_with_options`.

**Risk:** Other projects will need “gen if missing for my suite” with their own discovery and fix_cargo_paths → copy-paste of the same pattern.

**Move to BRRTRouter:**

- In `brrtrouter_tooling.build` or `brrtrouter_tooling.gen`, add:
  - `run_gen_if_missing_for_suite(project_root, suite, *, get_service_names_fn, spec_path_fn, output_dir_fn, deps_config_path_fn, fix_cargo_paths_fn)`  
    So any project can pass: suite name, how to list services, where specs/output live, and how to fix Cargo.toml.
- RERP then: `gen_if_missing_callback = lambda root: run_gen_if_missing_for_suite(root, "accounting", get_service_names_fn=suite_sub_service_names, ..., fix_cargo_paths_fn=run_fix_cargo_paths)`. RERP’s `run_accounting_gen_if_missing` is removed or becomes a one-liner.

**Result:** One “gen if missing” pattern in BRRTRouter; projects pass suite + discovery + fix.

---

## Summary table

| Area | Current RERP | Action |
|------|----------------|--------|
| **copy_artifacts** | Delegates to BRRTRouter; only `BINARY_NAMES`/`PACKAGE_NAMES` | None |
| **Docker/tilt wrappers** | Thin wrappers (config only) | Optional: centralise config |
| **ci/fix_cargo_paths** | Extends BRRTRouter with gen crate name/version/\[lib\] | **Move** gen-crate logic to BRRTRouter (parameterised) |
| **gen/regenerate** | Full local flow (paths + fix_cargo_paths) | **Move** regenerate_suite/service to BRRTRouter (parameterised) |
| **build/microservices** | Local `run_accounting_gen_if_missing` | **Move** “gen if missing for suite” to BRRTRouter (parameterised) |

---

## Recommended order of work (DONE)

1. **BRRTRouter: gen crate post-fix in `fix_cargo_paths`** ✅  
   Added optional `gen_crate_config: tuple[str, str] | None` (name_pattern, version). RERP: uses it; local gen-crate logic removed.

2. **BRRTRouter: `regenerate_service` / `regenerate_suite_services` in `gen`** ✅  
   Added `gen/regenerate.py` with default paths and optional `fix_cargo_paths_fn`. RERP: delegates to brrtrouter_tooling.gen.

3. **BRRTRouter: `run_gen_if_missing_for_suite` in gen** ✅  
   Generic “gen if missing”; run_accounting_gen_if_missing is a thin wrapper.

After this, the only RERP-specific pieces are **config** (names, paths, constants); all behaviour lives in BRRTRouter and is shared across Microscaler projects.
