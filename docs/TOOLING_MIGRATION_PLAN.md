# RERP Tooling Migration Plan

**Goal:** Tooling is the **singular** implementation for automation in RERP. No other scripts in the repo when finished. GitHub Actions that need automation call `rerp` subcommands.

**Principle:** Do **not** move/lift‑and‑shift Python from `./scripts`. **Break down** each script into Python modules in `tooling`, **fully tested** (TDD: tests first, then implementation), then expose as `rerp` subcommands.

---

## 1. Script → Module and `rerp` Subcommand Map

| Script | Domain | Module(s) | `rerp` subcommand | Notes |
|--------|--------|-----------|-------------------|-------|
| `assign-port.py` | ports | `discovery`, `ports` | `rerp ports` | ✅ Done. Add full tests. |
| `patch-brrtrouter-for-ci.py` | ci / cargo | `ci.patch_brrtrouter` | `rerp ci patch-brrtrouter` | Find Cargo.toml, replace path deps with git, cargo update |
| **Inline OpenAPI validate (CI)** | openapi | `openapi.validate` | `rerp openapi validate` | Replace inline Python in ci.yml |
| `generate_system_bff.py` | bff | `bff.generate_system` | `rerp bff generate-system` | ✅ Done. Directory-based discovery, merge specs. |
| `generate_bff_spec.py` | bff | `bff.generate_spec` | `rerp bff generate-spec` | Fallback; bff-generator + bff-suite-config preferred |
| `host-aware-build.py` | build (components) | `build.cargo` (arch, target, zigbuild/cross), `build.components` | `rerp build components workspace`, `rerp build components <crate>` | `--arch amd64\|arm64\|arm7\|all` |
| `build-microservice.sh` | build (microservices) | `build.microservice` | `rerp build microservice workspace`, `rerp build microservice <name>` | Uses discovery; brrtrouter-gen if missing |
| `build-and-push-microservice-containers.sh` | build | `build.push_containers` | `rerp build push-containers [--copy-only=ARCH] [TAG] [extra]` | Discovery-driven |
| `build-microservice-docker-simple.sh` | docker | `docker.build` | `rerp docker build` (or internal to push-containers) | |
| `generate-dockerfile.py` | docker | `docker.render` | `rerp docker render` | Template from discovery |
| `fix_cargo_toml_paths.py` | cargo | `cargo.fix_paths` | (library only; used by `rerp brrtrouter gen`) | |
| `bootstrap_microservice.py` | bootstrap | `bootstrap.microservice` | `rerp bootstrap microservice` | --suite, uses docker render + tiltfile |
| `fix_operation_id_casing.py` | openapi | `openapi.fix_operation_id_casing` | `rerp openapi fix-operation-id-casing` | |
| `generate_*_openapi.py` (4 scripts) | openapi | `openapi.generate_*` | `rerp openapi generate-*` | Consolidate into coherent `rerp openapi` subcommands |
| `setup-tilt.sh` | tilt | `tilt.setup` | `rerp tilt setup` | |
| `teardown-tilt.sh` | tilt | `tilt.teardown` | `rerp tilt teardown` | |
| `setup-kind-registry.sh` | k8s | `k8s.kind_registry` | `rerp k8s kind-registry` | |
| `setup-persistent-volumes.sh` | k8s | `k8s.persistent_volumes` | `rerp k8s persistent-volumes` | |
| `tail-tilt-logs.sh` | tilt | `tilt.logs` | `rerp tilt logs` | |
| `copy-*-binary*.sh`, `build-*-docker*.sh` (components) | build/docker | Fold into `build.push_containers`, `docker.build` | — | Or deprecate if microservices-only |

**Data (not code):** `port-registry.json` → project root or `RERP_PORT_REGISTRY`. `scripts/requirements.txt` → removed; tooling’s `pyproject.toml` is the only Python deps. `README*.md` → migrate into `tooling/README.md` or `docs/`.

---

## 2. GitHub Actions → `rerp` Commands

CI today uses:

| CI step | Current | Becomes |
|---------|---------|---------|
| Install deps | `pip install -r scripts/requirements.txt` | `pip install -e ./tooling` (or `.[dev]` when tests in CI) |
| OpenAPI validate | Inline `python3 -c "import yaml; ..."` | `rerp openapi validate` |
| BFF generate (dry run) | `python3 scripts/generate_system_bff.py` | `rerp bff generate-system` |
| Ports validate | `./scripts/assign-port.py validate` | `rerp ports validate` |
| Patch Cargo (BRRTRouter, lifeguard) | `python3 scripts/patch-brrtrouter-for-ci.py` | `rerp ci patch-brrtrouter` |
| Build (components) | `python3 scripts/host-aware-build.py workspace amd64` | `rerp build components workspace --arch amd64` |
| Build (components crate) | `python3 scripts/host-aware-build.py auth_idam amd64` | `rerp build components auth_idam --arch amd64` |
| Build (microservices) | `./scripts/build-microservice.sh workspace --arch amd64 --release` | `rerp build microservice workspace --arch amd64 --release` |
| Copy binaries | `./scripts/build-and-push-microservice-containers.sh --copy-only=amd64` | `rerp build push-containers --copy-only=amd64` |
| Build and push | `./scripts/build-and-push-microservice-containers.sh TAG [extra]` | `rerp build push-containers TAG [extra]` |

When migration is done, `scripts/` is **removed**. `tooling/` is the only automation. Any future CI that needs custom logic adds a `rerp` subcommand.

---

## 3. TDD Workflow

For each script (or logical group):

1. **Tests first:** In `tooling/tests/`, add `test_<domain>_<feature>.py` with unit tests for the **module API** (e.g. `validate_specs(openapi_dir) -> list[tuple[Path, Exception]]`). Use `tmp_path` or fixtures; avoid depending on real `openapi/` except in integration tests.
2. **Implement module:** In `tooling/src/rerp_tooling/<domain>/`, implement the logic. No CLI yet.
3. **CLI:** Add `rerp <subcommand>` in `cli/` that calls the module. Thin layer.
4. **Integration:** Optional `tests/integration/` that run against the real repo (e.g. `rerp ports validate`); can be opt‑in or in a separate CI job.

---

## 4. Implementation Order

1. **Infra:** pytest, `conftest.py`, and tests for existing `discovery` and `ports`.
2. **`rerp openapi validate`** (replaces CI inline) — TDD: `openapi.validate`, then `rerp openapi validate`.
3. **`rerp ci patch-brrtrouter`** — TDD: `ci.patch_brrtrouter`, then `rerp ci patch-brrtrouter`.
4. **`rerp bff generate-system`** — TDD: `bff.generate_system`, then CLI.
5. **`rerp build components`** (from host-aware-build) — TDD: `build.cargo` (arch/target, zigbuild/cross), `build.components`, then CLI.
6. **`rerp build microservice`** — TDD: `build.microservice` + discovery, then CLI.
7. **`rerp build push-containers`** — TDD: `build.push_containers`, then CLI.
8. **`rerp docker render`** — TDD: `docker.render` + discovery `iter_services`, then CLI.
9. **Tilt, k8s, bootstrap, openapi generate/fix** — Same pattern.
10. **Remove `./scripts`**, update CI to use only `rerp`, delete `scripts/`.

---

## 5. Test and CI for Tooling

- **`tooling/tests/`** — Unit and integration tests. Run: `cd tooling && .venv/bin/pytest tests/ -v` or with coverage: `--cov=rerp_tooling --cov-report=term-missing`.
- **CI:** A **`tooling`** job runs first (build, test, coverage). It creates `tooling/.venv`, `pip install -e ./tooling[dev]`, and `pytest tests/ -v --cov=rerp_tooling --cov-report=term-missing`. All other jobs that use `rerp` depend on `tooling`, create their own `tooling/.venv`, install `./tooling` (or `[dev]`), and run `tooling/.venv/bin/rerp`.

---

## 7. Phase 1 Done (Current)

- **`rerp ports`** — from assign-port (discovery + ports). Tests: discovery, ports (registry assign/list/release).
- **`rerp openapi validate`** — new module `openapi.validate`, CLI. Replaces inline Python in CI. Tests: validate_specs.
- **`rerp ci patch-brrtrouter`** — new module `ci.patch_brrtrouter`, CLI. Replaces `scripts/patch-brrtrouter-for-ci.py`. Tests: find_cargo_tomls, find_matches, patch_file.

**CI updated (`.github/workflows/ci.yml`):**

- **tooling** (runs first): create `tooling/.venv`, `pip install -e ./tooling[dev]`, `pytest tests/ -v --cov=rerp_tooling --cov-report=term-missing`. No other job runs until this passes.
- **validate-openapi** (needs: tooling): `tooling/.venv` + `pip install -e ./tooling`, `tooling/.venv/bin/rerp openapi validate`, `tooling/.venv/bin/rerp bff generate-system`.
- **validate-ports** (needs: validate-openapi): `tooling/.venv` + `pip install -e ./tooling`, `tooling/.venv/bin/rerp ports validate`.
- **build-and-test, build-multiarch, build-push-containers:** `tooling/.venv` + `pip install -e ./tooling`, `tooling/.venv/bin/rerp ci patch-brrtrouter`. `host-aware-build.py`, `build-microservice.sh`, `build-and-push-microservice-containers.sh` unchanged for now.

**Next:** `rerp build components`, `rerp build microservice`, `rerp build push-containers`, etc., per §4.

---

## 6. port-registry.json and Other Data

- **port-registry.json:** Default `project_root/port-registry.json`; fallback `project_root/scripts/port-registry.json` during migration. Override `RERP_PORT_REGISTRY`. When `scripts/` is removed, only project root (or explicit path).
- **README-port-registry.md:** Content moves into `tooling/README.md` or `docs/port-registry.md`.

---

*This plan is the source of truth for “what replaces what” and how we implement (TDD, no lift‑and‑shift).*
