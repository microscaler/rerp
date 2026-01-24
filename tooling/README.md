# RERP Tooling

RERP-specific development tooling: a single `rerp` CLI with subcommands for ports, build, docker, tilt, brrtrouter, and bff. Replaces the ad‑hoc `./scripts` with a proper Python package, shared discovery, and a coherent surface.

## Setup

From the **repo root**:

```bash
just init
```

This creates `tooling/.venv` and installs `rerp` (editable, with dev deps). The justfile owns `init` (not tooling) to avoid circular setup. Run `just init` before first `tilt up`; Tilt’s `build-tooling` resource expects `.venv` to exist.

- `just build-tooling` — rebuild after editing tooling. Tilt runs this automatically when `tooling/src` or `tooling/pyproject.toml` changes (ignores `*.pyc`, `__pycache__`).
- `just venv` — start an interactive shell with `tooling/.venv` sourced and activated (`exit` to leave).

Manual setup (if not using just):

```bash
cd tooling
python3 -m venv .venv
.venv/bin/pip install -e ".[dev]"
```

Or with `uv`:

```bash
cd tooling && uv venv && uv pip install -e ".[dev]"
```

## Usage

**Ports**
- **`rerp ports assign <service>`** — Assign next available port (or `--port N`, `--update-configs`)
- **`rerp ports list`** — List all port assignments
- **`rerp ports validate`** — Scan registry, helm, kind, Tiltfile, bff-suite-config; report conflicts
- **`rerp ports update-configs <service>`** — Update helm, kind, openapi servers for a service
- **`rerp ports reconcile [--update-configs]`** — Add helm-only services to the registry
- **`rerp ports fix-duplicates [--dry-run]`** — Resolve duplicate helm ports; prefer suite (BFF + bff-suite-config)

**OpenAPI**
- **`rerp openapi validate [--openapi-dir DIR]`** — Validate all `openapi.yaml` under `openapi/` (replaces CI inline).
- **`rerp openapi fix-operation-id-casing [--openapi-dir DIR] [--dry-run] [--verbose]`** — Convert operationId from camelCase to snake_case. Replaces `scripts/fix_operation_id_casing.py`.

**CI**
- **`rerp ci patch-brrtrouter [--dry-run] [--audit]`** — Replace path deps for BRRTRouter and lifeguard with git; run `cargo update`. Replaces `scripts/patch-brrtrouter-for-ci.py`.
- **`rerp ci fix-cargo-paths PATH`** — Fix brrtrouter/brrtrouter_macros path deps in a Cargo.toml to ../BRRTRouter (local dev after brrtrouter-gen). Replaces `scripts/fix_cargo_toml_paths.py`.

**BFF**
- **`rerp bff generate-system [--system NAME] [--output PATH] [--openapi-dir DIR]`** — Merge `openapi/{system}/{service}/openapi.yaml` into system BFF at `openapi/{system}/openapi.yaml`. No `--system` ⇒ all systems with sub-services. Replaces `scripts/generate_system_bff.py`.

**Docker**
- **`rerp docker generate-dockerfile <system> <module> [--port N]`** — Generate `docker/microservices/Dockerfile.{system}_{module}` from template. Replaces `scripts/generate-dockerfile.py`.
- **`rerp docker copy-artifacts <arch>`** — Copy microservice binaries from `microservices/target/{triple}/release/` to `build_artifacts/{amd64|arm64|arm}`. Use after `rerp build microservices <arch> --release`. Replaces `--copy-only` of `build-and-push-microservice-containers.sh`. **Push is in GHA only** (`docker/build-push-action` per service).
- **`rerp docker copy-binary <source> <dest> <binary_name>`** — Copy a binary to dest, chmod +x, write `{dest}.sha256`. Used by Tilt’s accounting flow. Replaces `copy-microservice-binary-simple.sh`.
- **`rerp docker build-image-simple <image_name> <dockerfile> <hash_path> <artifact_path>`** — Ensure hash/artifact/dockerfile exist; `docker build -t {image_name}:tilt`; push to registry or `kind load`. Used by Tilt. Replaces `build-microservice-docker-simple.sh`.
- **`rerp docker copy-multiarch <system> <module> [arch]`** — Copy component binaries from `components/target/{triple}/release/` to `build_artifacts/{system}_{module}/{arch}/`. `arch`: amd64, arm64, arm7, or all. Replaces `copy-multiarch-binary.sh`.
- **`rerp docker build-multiarch <system> <module> <image_name> [--tag N] [--push]`** — Build for all archs, copy, generate Dockerfile if missing, buildx base+images, manifest; optional push. Replaces `build-multiarch-docker.sh`.
- **`rerp docker build-base [--push] [--dry-run]`** — Build `docker/base/Dockerfile` as `rerp-base:latest`. `--push` pushes to `ghcr.io/$GHCR_OWNER/rerp-base:latest` (requires login). **Publishing** uses `.github/workflows/base-images.yml` (change detection, registry cache).

**Build**
- **`rerp build <target> [arch]`** — Host-aware build: `workspace` or `<system>_<module>` (e.g. `auth_idam`). Optional `arch`: `amd64`, `arm64`, `arm7`, or `all` (default: host). Replaces `scripts/host-aware-build.py`.
- **`rerp build microservices [arch] [--release]`** — Build `microservices/` workspace (accounting). `arch`: amd64, arm64, arm7 (default amd64). Replaces `scripts/build-microservice.sh workspace`.
- **`rerp build microservice <name> [--release]`** — Build one accounting microservice (e.g. general-ledger). Replaces `scripts/build-microservice.sh <name>`.

**Bootstrap**
- **`rerp bootstrap microservice <name> [port]`** — Bootstrap accounting microservice from `openapi/accounting/<name>/openapi.yaml`: BRRTRouter codegen, Dockerfile, config, workspace Cargo.toml, Tiltfile. Port from registry or `[port]`. Replaces `scripts/bootstrap_microservice.py`.

**Tilt**
- **`rerp tilt setup-kind-registry`** — Create/start `kind-registry` (localhost:5001) and connect to the `kind` network. Run after `kind create cluster`. Replaces `setup-kind-registry.sh`.
- **`rerp tilt setup-persistent-volumes`** — Apply `k8s/data/persistent-volumes.yaml` and `k8s/monitoring/persistent-volumes.yaml`. Replaces `setup-persistent-volumes.sh`.
- **`rerp tilt setup`** — Create dirs and Docker volumes; check docker/tilt. Replaces `setup-tilt.sh`.
- **`rerp tilt teardown [--remove-images] [--remove-volumes] [--system-prune]`** — Tilt down, stop containers; optional cleanup. Replaces `teardown-tilt.sh`.
- **`rerp tilt logs <component>`** — Tail Tilt logs for a component. Replaces `tail-tilt-logs.sh`.

Run from the **repo root** so `port-registry.json` is found (at project root). Override with:

- `RERP_PROJECT_ROOT` — project root (default: `.` or cwd)
- `RERP_PORT_REGISTRY` — path to `port-registry.json`

## Relation to Microscaler Farm

RERP tooling is **not** a replacement for Farm. Use **Farm** for git, test, lint, env, agents; use **`rerp`** for RERP structure: ports, microservice build, Docker render, Tilt, brrtrouter, BFF spec. Example: `farm git preflight` and `rerp ports validate`.

## Tests

```bash
cd tooling && .venv/bin/pip install -e ".[dev]" && .venv/bin/pytest tests/ -v
```

With coverage (as in CI):

```bash
cd tooling && .venv/bin/pytest tests/ -v --cov=rerp_tooling --cov-report=term-missing
```

## Layout

- `src/rerp_tooling/` — package
  - `discovery/` — suites, BFFs, `iter_bffs`, and port sources for validate
  - `ports.py` — `PortRegistry`, validate, reconcile, fix_duplicates
  - `openapi/` — `validate_specs` for `rerp openapi validate`; `fix_operation_id` for `rerp openapi fix-operation-id-casing`
  - `ci/` — `patch_brrtrouter` for `rerp ci patch-brrtrouter`; `fix_cargo_paths` for `rerp ci fix-cargo-paths`
  - `bff/` — `discover_sub_services`, `generate_system_bff_spec`, `list_systems_with_sub_services` for `rerp bff generate-system`
  - `cli/` — `rerp` entry and `ports`, `openapi`, `ci`, `bff`, `docker`, `build` subcommands
  - `docker/` — `generate_dockerfile`, `copy_artifacts`, `copy_binary`, `copy_multiarch`, `build_image_simple`, `build_multiarch`, `build_base` for `rerp docker generate-dockerfile`, `copy-artifacts`, `copy-binary`, `copy-multiarch`, `build-image-simple`, `build-multiarch`, `build-base`
  - `tilt/` — `setup_kind_registry`, `setup_persistent_volumes`, `setup`, `teardown`, `logs` for `rerp tilt setup-kind-registry`, `setup-persistent-volumes`, `setup`, `teardown`, `logs`
  - `build/` — `host_aware` for `rerp build` (workspace or \<system\>_\<module\> [arch])
  - `bootstrap/` — `microservice` for `rerp bootstrap microservice`
- `tests/` — unit tests (TDD). No lift‑and‑shift; scripts are broken into modules and tested.
