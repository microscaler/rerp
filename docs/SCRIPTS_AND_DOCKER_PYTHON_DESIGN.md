# Scripts Consolidation and Docker Templating — Design

**Status:** Superseded in part by the implemented descriptor/staged-context design
**Date:** 2025-01-24  
**Implementation status:** §7.3 records what is **done** in `./tooling` vs **gaps** in `./scripts`, which callers (CI, Tiltfile, justfile) still use `./scripts`, and the distance to deprecating and deleting `./scripts`.  
**Current decision (2026-07-15):** Service Dockerfile rendering was rejected. RERP now has one parameterized `docker/microservices/Dockerfile`; `rerp_tooling.runtime` discovers suite-scoped service metadata and stages narrow, verified development or multi-architecture release contexts. Sections describing rendered `Dockerfile.*` files are retained as historical analysis, not implementation guidance.

**Scope:** Replace shell scripts in `./scripts` with Python using shared metadata discovery; use one parameterized microservice Dockerfile with staged contexts; **externalise embedded bash in the Tiltfile** into Python where practical. **Transform `./scripts` into `./tooling`**: a pyproject-based, .venv-backed **holistic RERP tooling** package (not a flat dump of ad‑hoc scripts). This tooling is **RERP-specific** and does **not** replace Microscaler Farm.

---

## 1. Objectives

1. **Replace shell scripts with Python** that use the same metadata discovery logic as `assign-port.py`, so adding a new suite does not require editing tooling.
2. **Replace static Dockerfiles** with one parameterized Dockerfile and tooling-staged, service-specific build contexts.
3. **Single source of truth for “what is a service”**: discovery is driven by repo layout (`openapi/`, `microservices/`, `bff-suite-config.yaml`) and Cargo/port-registry, not hardcoded lists.
4. **Transform `./scripts` → `./tooling`**: a proper Python package with `pyproject.toml` and `.venv`, shared reusable libraries, a coherent CLI, and room to grow as RERP’s development workflows evolve.

---

## 1b. From script dump to holistic tooling (and relation to Farm)

### Why not keep `./scripts` as a dump of Python scripts?

A flat `./scripts` of standalone `.py` files leads to:

- **No shared reuse:** Each script imports its own copy of path logic, or duplicates it. `assign-port.py` has discovery helpers; `build_microservice.sh` has a parallel `case` mapping; `generate_bff_spec` has `SERVICE_CONFIG`. There is no single `from rerp_discovery import iter_services` for everyone.
- **Ad‑hoc invocations:** Callers must know whether to run `python3 scripts/assign-port.py` or `./scripts/build-microservice.sh`. There is no unified surface (`rerp ports validate`, `rerp build microservice general-ledger`).
- **Dependency chaos:** Some scripts need `yaml`, others `toml`; versions are unspecified. A `requirements.txt` in `scripts/` is a grab‑bag. No isolated env: deps can clash with system Python or other tools.
- **Hard to test:** Standalone scripts are often written to be run, not imported. Shared logic is buried in `if __name__ == "__main__"` blocks. Unit tests would have to subprocess or mock at the edges.
- **Hard to grow:** Adding a “validate all suites” or “doctor” command means yet another top‑level script. No clear place for `rerp_discovery`, `rerp_docker`, `rerp_ports` as importable modules.

### What we want instead: `./tooling` as holistic RERP tooling

- **One installable package:** `pyproject.toml` in `./tooling`, dependencies declared, versioned. A **`.venv`** (in `./tooling` or at repo root with `pip install -e ./tooling`) isolates RERP tooling deps. One `pip sync` or `uv sync` and all commands have the same deps.
- **Shared, reusable libraries:** `rerp_discovery` (suites, services, ports), `rerp_docker` (templating, render), and eventually `rerp_brrtrouter`, `rerp_tilt` live as packages under `tooling`. Every CLI command `from rerp_tooling.discovery import iter_services` instead of re‑implementing.
- **Unified CLI:** A single entry point (e.g. `rerp`) with subcommands: `rerp ports assign|list|validate|update-configs|reconcile|fix-duplicates`, `rerp build microservice|workspace`, `rerp docker render`, `rerp tilt setup|teardown`, `rerp brrtrouter lint|gen`, `rerp bff spec-gen`, etc. Tiltfile, `just`, and humans run `rerp ...` (or `./tooling/.venv/bin/rerp ...` when the venv is not activated).
- **Testable and maintainable:** Libraries are importable; CLI is thin. Tests can target `iter_services()`, `render_dockerfile(ServiceInfo(...))`, and the `rerp ports` logic without subprocess. New behaviour is added as library + subcommand, not a new script in a flat list.
- **Designed to grow:** As RERP gains HR, sales, or other suites, we add `rerp suites list`, `rerp doctor`, or `rerp openapi validate` in the same tree. No need to invent a new “scripts 2.0” later.

### Relationship to Microscaler Farm

**RERP tooling is not a replacement for Farm.** Farm is cross‑project, platform tooling: git workflows, lint, test, coverage, docker/registry, agents, memory‑bank, env, etc. RERP tooling is **RERP‑specific**:

- **RERP:** suites, BFFs, `openapi/{suite}/`, `microservices/{suite}/`, `bff-suite-config.yaml`, port‑registry, Tilt resources, rendering Dockerfiles and Tiltfile fragments for RERP’s layout. Concepts like `iter_services()`, `ServiceInfo`, and “accounting vs hr suite” are RERP domain.
- **Farm:** `farm git`, `farm test`, `farm lint`, `farm docker`, `farm agent`, etc. Generic and reusable across Microscaler projects.

The two coexist: use **Farm** for git, testing, and cross‑cutting automation; use **RERP tooling** (`rerp`) for anything that depends on RERP’s structure (ports, build of microservices, Docker render, Tilt setup, brrtrouter, BFF spec). Example: `farm git preflight` before commit; `rerp ports validate` and `rerp docker render` as RERP‑specific checks. RERP tooling does not duplicate Farm’s domain; it fills the RERP‑shaped hole.

---

## 2. Current State (Problems)

### 2.1 Scripts with Hardcoded Service/Suite Logic

| Script | Hardcoded data | Purpose |
|--------|----------------|---------|
| `build-microservice.sh` | `ACCOUNTING_SERVICES`, `case` for (service → package_name) | Build one microservice or workspace; run brrtrouter-gen if accounting crates missing |
| `build-and-push-microservice-containers.sh` | `SERVICES`, `PKG`, `BIN` (associative arrays) | `--copy-only=ARCH` and multi-arch build/push for accounting + bff |
| `build-microservice-docker.sh` | `system`/`module` → `binary_name=rerp_{system}_{module}_impl` | Docker build for **components** layout (not used by current Tilt) |
| `build-microservice-docker-simple.sh` | None (takes paths as args) | Docker build; used by Tiltfile with explicit paths |
| `build-multiarch-docker.sh` | `system`/`module` → `binary_name`, arch maps | Multi-arch Docker for **components**; calls `generate-dockerfile.py` |
| `copy-microservice-binary.sh` | `system`/`module` → `rerp_{system}_{module}_impl`, `components/` | Copy from components to build_artifacts |
| `copy-microservice-binary-simple.sh` | None (args) | Copy source→dest + hash; used by Tiltfile |
| `copy-multiarch-binary.sh` | `system`/`module` → `rerp_*_impl`, `components/` | Multi-arch copy for components |
| `teardown-tilt.sh` | `general-ledger invoice ... budget` (no edi, financial-reports, bff) | Stop/rm containers and optional image/volume cleanup |
| `setup-tilt.sh` | `openapi/accounting`, `microservices/accounting` | Create dirs; no service list |
| `tail-tilt-logs.sh` | Example list in help only | `tilt logs`; fine as-is |
| `setup-kind-registry.sh` | None | Kind registry; no service list |
| `setup-persistent-volumes.sh` | None | `kubectl apply` PVs; no service list |
| `generate-dockerfile.py` | `system`/`module` and `components/`-style template | Renders `Dockerfile.{system}_{module}` for **components**; not aligned with `microservices/` layout |
| `generate_bff_spec.py` | `SERVICE_CONFIG` (accounting + ports), `service_type` | Fallback BFF generator; CI uses `bff-generator` + bff-suite-config |
| `generate_system_bff.py` | Walks `openapi/{system}/`; no SERVICE_CONFIG | System BFF; discovery is directory-based |

**Two layouts in use:**

- **Microservices (Tilt, accounting today):** `microservices/{suite}/{service_dir}/`, binary from Cargo `[package]` (or `[[bin]]`), `build_artifacts/amd64/{binary_name}`. `Dockerfile.{service}` uses `microservices/accounting/...` and `build_artifacts/${TARGETARCH}/{binary_name}`.
- **Components (host-aware-build, some scripts):** `components/{system}/{module}_impl`, binary `rerp_{system}_{module}_impl`. Different path and naming.

This design focuses on the **microservices** layout as the primary path. Components-based flows can be phased later.

### 2.2 Tiltfile

- `PACKAGE_NAMES`, `BINARY_NAMES`, `get_service_port`, `ACCOUNTING_SERVICES` are all hardcoded.
- `bff-spec-gen` deps list each `openapi/accounting/{name}/openapi.yaml` explicitly.
- Adding a service requires edits in several places.
- **Embedded scripts:** Several `local_resource` and `custom_build` commands embed inline bash. See **§2.4** and **§5.6**.

### 2.3 Dockerfiles

- **Static:** `Dockerfile.general-ledger`, `Dockerfile.invoice`, …, `Dockerfile.bff` are hand-maintained or one-off generated.
- **Template:** `Dockerfile.template` uses `{{system}}`, `{{module}}`, `{{binary_name}}` and `components/` paths; it does not match the structure of the existing microservice Dockerfiles (e.g. `Dockerfile.invoice` uses `microservices/accounting/`, `alpine:3.19`, `build_artifacts/${TARGETARCH}/invoice`).
- **Inconsistency:** `binary_name` in Dockerfiles (`invoice`, `general_ledger`, `bff`) does not always match Cargo `[package].name` (`invoice_management`, `general_ledger`, `rerp_accounting_backend_for_frontend_api`). `BINARY_NAMES` in the Tiltfile encodes this mapping; it must move into discoverable metadata.

### 2.4 Tiltfile Embedded Scripts (Inventory)

All `local_resource` / `custom_build` that embed inline bash or non-trivial logic. These must be externalised into `./tooling` (as `rerp` subcommands or library-backed CLIs) and reimplemented in Python so the Tiltfile only invokes `rerp ...` (or `./tooling/.venv/bin/rerp ...`); discovery and suite-awareness live in the tooling library.

| Resource | Tiltfile lines | Embedded logic | Calls to external scripts |
|----------|----------------|----------------|---------------------------|
| **create_microservice_build** | 41–146 | None (dead: not invoked). Only invokes `host-aware-build.py`, `copy-microservice-binary.sh`, `generate-dockerfile.py`, `build-microservice-docker.sh`. | Yes |
| **create_microservice_lint** | 154–176 | **Bash:** `set -e`; try `../BRRTRouter/target/debug/brrtrouter-gen lint --spec ./openapi/{spec_file} --fail-on-error` else `cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen -- lint --spec ./openapi/{spec_file} --fail-on-error`. Hardcoded paths to BRRTRouter. | No |
| **create_microservice_gen** | 181–218 | **Bash:** `set -e`; try `brrtrouter-gen generate --spec ./openapi/{spec} --output ./microservices/accounting/{output_dir} --force` else `cargo run ... generate ...`; then `if [ -f ./microservices/accounting/{output_dir}/Cargo.toml ]; then python3 ./scripts/fix_cargo_toml_paths.py ./microservices/accounting/{output_dir}/Cargo.toml; fi`. Hardcoded `microservices/accounting/`. | `fix_cargo_toml_paths.py` only |
| **create_microservice_build_resource** | 268–286 | None. `cmd='./scripts/build-microservice.sh %s'`. | Yes |
| **create_microservice_deployment** (copy) | 303–311 | None. `cmd='./scripts/copy-microservice-binary-simple.sh %s %s %s'`. | Yes |
| **create_microservice_deployment** (docker) | 314–321 | None. `cmd='./scripts/build-microservice-docker-simple.sh %s %s %s %s'`. | Yes |
| **create_microservice_deployment** (custom_build) | 324–336 | **Bash (inline):** `(docker image inspect {image}:tilt >/dev/null 2>&1) \|\| ./scripts/build-microservice-docker-simple.sh {image} {dockerfile} {hash_path} {artifact_path}` then `(docker push $EXPECTED_REF 2>/dev/null \|\| kind load docker-image $EXPECTED_REF --name rerp)`. Uses Tilt env `EXPECTED_REF`. | `build-microservice-docker-simple.sh` |
| **create_microservice_deployment** (live_update) | 331–335 | `run('kill -HUP 1', trigger=[artifact_path])` — Tilt built-in; runs **in-container**. Not host-side; leave in Tiltfile. | — |
| **accounting-all-gens** | 380–385 | `cmd='echo "✅ All accounting codegen complete"'`. Trivial; can stay or become `rerp noop` for symmetry. | No |
| **bff-spec-gen** | 402–423 | **Bash:** `set -e`; `bff-generator generate-spec --config openapi/accounting/bff-suite-config.yaml --output openapi/accounting/openapi_bff.yaml`. Hardcoded `openapi/accounting/`. `deps=[...]` list is also hardcoded (one entry per service). | No |
| **bff-lint** | 327–344 | **Bash:** same pattern as **create_microservice_lint** but `--spec ./openapi/accounting/openapi_bff.yaml`. | No |

**Summary of embedded logic to move into `./tooling` (Python, as `rerp` subcommands or equivalent):**

1. **brrtrouter-gen lint** (create_microservice_lint, bff-lint): try `../BRRTRouter/target/debug/brrtrouter-gen lint`, else `cargo run ... brrtrouter-gen lint`. Paths and BRRTRouter location must be parameterised (or discovered).
2. **brrtrouter-gen generate + fix_cargo_toml_paths** (create_microservice_gen): generate into `microservices/{suite}/{output_dir}`; then `fix_cargo_toml_paths`. Suite and output_dir must come from args or discovery, not `accounting` literal.
3. **custom_build “ensure image and push/load”**: if image doesn’t exist, run build; then `docker push $EXPECTED_REF` or `kind load docker-image $EXPECTED_REF --name rerp`. Must work with Tilt’s `EXPECTED_REF` (env).
4. **bff-generator generate-spec** (bff-spec-gen): run `bff-generator` with `--config` and `--output`. Config/output must be `openapi/{suite}/bff-suite-config.yaml` and `openapi/{suite}/openapi_bff.yaml` from `--suite` or discovery. The `deps` list for the Tilt resource should come from bff-suite-config (or `Tiltfile.generated.star`) so it’s not handwritten.
5. **accounting-all-gens**: keep `echo` or `rerp noop "All accounting codegen complete"`; low priority.

**Not externalised (stay in Tiltfile):**

- **live_update** `run('kill -HUP 1', ...)`: Tilt in-container primitive; no host script.
- **create_microservice_build**: dead (components path); can be removed or repurposed later.

---

## 3. Shared Metadata Discovery (Python Library)

### 3.1 Location and Shape

- **Package:** `tooling/src/rerp_tooling/discovery/` (or `tooling/rerp_tooling/discovery/`) as part of the `rerp_tooling` package. Export as `from rerp_tooling.discovery import iter_services, suites_with_bff, ...`. All RERP tooling (ports CLI, build, docker, brrtrouter, bff, tilt) imports from here.
- **Reuse:** The current `assign-port.py` logic (`_suites_with_bff`, `_bff_suite_config_path`, `_openapi_bff_path`, `_service_to_suite`, `_get_bff_service_name_from_config`, `_iter_bffs`, `_bff_service_to_suite`) moves into `rerp_tooling.discovery`. The `rerp ports` subcommand uses it; no duplicate logic.
- **Refactor:** Extract discovery into `rerp_tooling.discovery`; add `iter_microservice_crates`, `iter_services`, `ServiceInfo`. The former `assign-port.py` becomes the `rerp ports` subcommand, implemented on top of `rerp_tooling.discovery` and `rerp_tooling.ports` (or a single `rerp_tooling.cli.ports` module).

### 3.2 Discovery Primitives

**Suites (with BFF):**

- `suites_with_bff() -> List[str]`: `openapi/*/bff-suite-config.yaml` exists.
- `iter_bffs() -> Iterator[Tuple[str, str]]`: `(bff_registry_name, suite)` from `bff_service_name` in each bff-suite-config.

**Microservice crates (per suite):**

- `iter_microservice_crates() -> Iterator[Tuple[str, str]]`: for each `microservices/{suite}/`:
  - list `microservices/{suite}/*/Cargo.toml` (exclude `bff` if we want to treat BFF separately, or include it). For each: `(suite, service_dir)` where `service_dir` is the directory name (e.g. `general-ledger`, `bff`).
- Alternatively: derive from `bff-suite-config` `services` + BFF. Prefer **walking `microservices/{suite}/`** so crates are discovered even if not yet in bff-suite-config.

**Per-service metadata (suite, service_dir, is_bff):**

For each `(suite, service_dir)`:

1. **Paths:**
   - `microservices_root = microservices/{suite}/{service_dir}/`
   - `cargo_toml = microservices_root / "Cargo.toml"`
2. **From Cargo.toml:**
   - `package_name = [package].name`
   - `cargo_binary_name`: `[[bin]].name` if present, else `[package].name` (file in `target/.../release/`).
3. **Overrides (to align with current BINARY_NAMES / Helm):**
   - From bff-suite-config:
     - If `service_dir == bff_service_name` (compare with `bff_service_name` for this suite): `registry_name = bff_service_name`; optional `metadata.bff_binary_name` override for the artifact/Docker binary name.
     - Else if `service_dir` is in `services`: optional `services.{name}.binary_name`.
   - If no override, `binary_name` (for `build_artifacts/`, `/app/`, Docker) = `cargo_binary_name`. The copy step uses: source `target/.../release/{cargo_binary_name}`, dest `build_artifacts/amd64/{binary_name}`.
4. **Registry (service) name:**
   - BFF: `bff_service_name` from bff-suite-config.
   - Others: `service_dir` (e.g. `general-ledger`). Used in Helm, port-registry, `Dockerfile.{registry_name}`.
5. **Port:**
   - From `port-registry.json` (at **project root** by default; overridable via `RERP_PORT_REGISTRY` or `--registry`) for `registry_name`; if missing, discovery can return `None` and callers (e.g. Docker render) can use a default or fail.

**Concrete API (high level):**

```text
def iter_services() -> Iterator[ServiceInfo]:
    """All services that have a microservices/{suite}/{name}/ Crate and (optionally) port-registry entry."""
    # ServiceInfo: suite, service_dir, registry_name, package_name,
    #              cargo_binary_name (file in target/), binary_name (build_artifacts/, /app/ in Docker),
    #              cargo_toml, microservices_root, port: Optional[int]
```

`ServiceInfo` can be a `dataclass` or `TypedDict`. `iter_services()` should:
- Walk `microservices/*/` for `*/Cargo.toml`.
- For BFF: identify via `bff_service_name` (and `microservices/{suite}/bff` or the dir that matches the BFF crate) and apply `bff_binary_name` if set.
- For others: use `service_dir` as `registry_name`, Cargo for `package_name`/`binary_name`, and overrides from bff-suite-config when available.
- Optionally merge with `port-registry.json` to attach `port`.

### 3.3 BFF Crate Identification

- BFF crate: `microservices/{suite}/bff/` when `bff_service_name == "bff"`; or more generally, the directory name for the BFF can match `bff_service_name` (with hyphens normalized). For “bff” the dir is `bff`.
- If we ever have `hr-bff` → `microservices/hr/hr-bff/`, the same rule applies: `service_dir == "hr-bff"` and `bff_service_name == "hr-bff"`.

### 3.4 Backward Compatibility / Overrides

- **bff-suite-config.yaml:** add optional:
  - `metadata.bff_binary_name` (e.g. `bff`) when the Cargo binary differs from `[package].name`.
  - `services.{name}.binary_name` for backends when needed.
- **Cargo.toml:** prefer adding `[[bin]] name = "bff"` (or `invoice`, etc.) where we want a different binary name, and keep overrides only for migration or special cases.

---

## 4. Docker Templating Tool

### 4.1 Role

- **Infer** the set of services from `rerp_discovery.iter_services()`.
- **Render** `docker/microservices/Dockerfile.{registry_name}` from a **single** template so that new suites/services get a Dockerfile without editing the tool.

### 4.2 Template

- **Path:** `docker/microservices/Dockerfile.template` (replace or coexist with current one; we define a new contract).
- **Variables:** must match what `ServiceInfo` (or the renderer) provides. Proposed:

  | Variable       | Source                         | Example                          |
  |----------------|--------------------------------|----------------------------------|
  | `service_name` | `registry_name` (human)        | `general-ledger`, `bff`         |
  | `binary_name`  | From Cargo + override          | `general_ledger`, `bff`         |
  | `suite`        | From discovery                 | `accounting`                    |
  | `service_dir`  | From discovery                 | `general-ledger`, `bff`         |
  | `port`         | Port registry or default       | `8001`, `8010`                  |

- **Paths in Dockerfile:**
  - `COPY ./build_artifacts/${TARGETARCH}/{{binary_name}} /app/{{binary_name}}`
  - `COPY ./microservices/{{suite}}/{{service_dir}}/config /app/config`
  - `COPY ./microservices/{{suite}}/{{service_dir}}/doc /app/doc`
  - `COPY ./microservices/{{suite}}/{{service_dir}}/static_site /app/static_site`
  - `EXPOSE {{port}}`
  - `ENTRYPOINT ["/app/{{binary_name}}", "--spec", "/app/doc/openapi.yaml", ...]`

- Base image: keep `alpine:3.19` as in current Dockerfiles, or make it a template variable. Multi-arch: keep `ARG TARGETPLATFORM` / `TARGETARCH` as today.

### 4.3 Tool Behaviour

- **CLI:** `rerp docker render` (or `rerp docker-render` as a subcommand under a `rerp docker` group). Implemented in `rerp_tooling` and wired via `pyproject.toml` `[project.scripts]` or `[project.entry-points.'console_scripts']`.
- **Modes:**
  - **Render all:** for each `ServiceInfo` from `iter_services()`, render `docker/microservices/Dockerfile.{registry_name}`. Option `--dry-run` to print only.
  - **Render one:** `--service <registry_name>` to render only that service.
- **Idempotent:** re-run overwrites; no manual edits to generated Dockerfiles.
- **Discovery:** use `iter_services()` from `rerp_tooling.discovery`; port from port-registry when present, otherwise e.g. 8080 or fail (configurable).

### 4.4 When to Run

- **After adding a new microservice crate:** e.g. `microservices/accounting/new-svc/` + `port-registry` + bff-suite-config (if needed) → run `rerp docker render` → new `Dockerfile.new-svc`.
- **CI:** as part of a “generate” or “prebuild” step (e.g. before `docker build`), or in a “validate” that checks generated Dockerfiles match `rerp docker render` output.
- **Tiltfile / just / make:** optional `rerp docker render` (or a `render_dockerfiles` target that runs it) so `tilt up` or `just dev-up` can ensure Dockerfiles exist.

### 4.5 Removal of Static Dockerfiles

- After rollout, **remove** hand-maintained `Dockerfile.general-ledger`, `Dockerfile.invoice`, …, `Dockerfile.bff`.
- **Keep** only `Dockerfile.template` and treat `Dockerfile.*` as generated. Optionally add `docker/microservices/.gitignore` for `Dockerfile.*` and always generate in CI/local, or commit generated Dockerfiles for traceability; the design prefers **committing generated** so `docker build` and CI stay simple and reviewable.

---

## 5. Script Replacements (Shell → Python)

### 5.1 Build and Copy (Microservices Layout)

| Current (shell)               | New (Python)                         | Discovery use                          |
|------------------------------|--------------------------------------|----------------------------------------|
| `build-microservice.sh`      | `rerp build microservice` \| `rerp build workspace` | `iter_services()` for name→package; `iter_microservice_crates` + bff-suite for “run brrtrouter-gen if missing” (from `openapi/{suite}/{name}/openapi.yaml` per bff-suite-config + BFF spec) |
| `build-and-push-microservice-containers.sh` | `rerp build push-containers` | `iter_services()` for SERVICES, PKG, BIN; `--copy-only=ARCH` and multi-arch build/push |
| `copy-microservice-binary-simple.sh` | keep or thin Python wrapper     | No discovery; args only. Tiltfile can keep passing paths, or we provide a `copy_binary.py --service <registry_name>` that uses discovery to get source/dest. |
| `copy-microservice-binary.sh` | deprecate or redirect to components flow | Components layout; out of scope for this design. |
| `copy-multiarch-binary.sh`   | deprecate or `copy_multiarch.py` for components | Same. |

**build_microservice.py (high level):**

- Args: `[service-name | workspace] [--arch amd64|arm64|arm7] [cargo args...]`
- If `workspace`: build `microservices/` with `--workspace`; optionally “gen if missing” by:
  - For each suite in `suites_with_bff()`: for each `name` in bff-suite-config `services` and the BFF, check `microservices/{suite}/{name}/Cargo.toml` (or the BFF path); if missing, run brrtrouter-gen from `openapi/{suite}/{name}/openapi.yaml` (or BFF from `openapi/{suite}/openapi_bff.yaml`) and `fix_cargo_toml_paths.py`. Uses discovery + bff-suite-config only; no “accounting” literal.
- If `service-name`: resolve `service-name` to `(suite, service_dir, package_name)` via discovery; then `cargo build -p package_name` from `microservices/`. Host/arch and zigbuild/cross logic can be shared with `host-aware-build.py` or inlined.

**build_push_containers.py (high level):**

- Args: `[--copy-only=amd64|arm64|arm7] [TAG] [extra_tag]`
- `--copy-only`: for each `ServiceInfo`, copy `microservices/target/{triple}/release/{package_name}` → `build_artifacts/{arch}/{binary_name}` (with `arch_to_triple` / `arch_to_artifact_dir`). No hardcoded SERVICES/PKG/BIN.
- Otherwise: for each `ServiceInfo`, ensure `Dockerfile.{registry_name}` exists (run `rerp docker render` or require pre-run), then `docker buildx build` for `linux/amd64,linux/arm64,linux/arm/v7` as today. Image names: `ghcr.io/{owner}/rerp-{registry_name}`.

### 5.2 Docker

| Current                      | New                                | Discovery use                          |
|-----------------------------|------------------------------------|----------------------------------------|
| `build-microservice-docker.sh` | Deprecate (components) or adapt  | —                                       |
| `build-microservice-docker-simple.sh` | Keep or thin Python wrapper  | No discovery.                          |
| `build-multiarch-docker.sh` | Deprecate or `build_multiarch_docker.py` for components | —                     |
| `generate-dockerfile.py`    | **Replaced** by `rerp docker render` | `iter_services()`; one template for microservices. |

### 5.3 Setup / Teardown / Tilt Helpers

| Current                    | New                         | Discovery use                                      |
|----------------------------|-----------------------------|----------------------------------------------------|
| `setup-tilt.sh`            | `rerp tilt setup`           | Create `openapi/{s}`, `microservices/{s}` for `s in suites_with_bff()`; if none, minimal or create `accounting` as default. Dir creation only; no service list. |
| `teardown-tilt.sh`         | `rerp tilt teardown`        | For each `registry_name` from `iter_services()`: `rerp-{registry_name}-dev`, `localhost:5001/rerp-{registry_name}:tilt`. Optional prompts for images/volumes/prune as now. |
| `tail-tilt-logs.sh`        | `rerp tilt logs` or keep    | Arg: component name. `tilt logs`; no discovery.    |
| `setup-kind-registry.sh`   | `rerp kind registry` (or equivalent) | Logic only; no service list.              |
| `setup-persistent-volumes.sh` | `rerp k8s persistent-volumes` (or equivalent) | Logic only; no service list.        |

### 5.4 BFF / OpenAPI Generation

- `generate_bff_spec.py`: already has `SERVICE_CONFIG` and `service_type`. Prefer **bff-generator + bff-suite-config** as the canonical path (as in Tilt/CI). This script can be retired or restricted to fallback; any remaining discovery should use `bff-suite-config` and port-registry instead of `SERVICE_CONFIG`.
- `generate_system_bff.py`: discovery is directory-based; can be updated to use `suites_with_bff()` and bff-suite-config for ports/paths if it stays.
- **Tiltfile bff-spec-gen deps:** should be generated from bff-suite-config `services` (+ BFF spec path) so adding a service doesn’t require Tiltfile edits. That implies a Tiltfile fragment or a codegen step for the `deps=[...]` list; included in “Tiltfile” below.

### 5.5 Bootstrap

- `bootstrap_microservice.py`: creates crates, Dockerfile, updates Cargo.toml/Tiltfile. It hardcodes `microservices/accounting/`. It should:
  - Take `--suite` and optionally infer from `openapi/{suite}/{name}/openapi.yaml` existing.
  - Use `rerp docker render` to create the Dockerfile (or call the same logic) instead of its own `create_dockerfile()`.
  - Updates to Tiltfile: ideally via a “Tiltfile fragment” or a `tiltfile_append` generated from discovery so the Tiltfile stays suite-agnostic. Detailed Tiltfile generation is a follow-on.

### 5.6 Tiltfile-Embedded Scripts → Python (Externalise into `./tooling` as `rerp` subcommands)

All inline bash in `local_resource` / `custom_build` (§2.4) is replaced by calls to the RERP tooling CLI: `rerp brrtrouter lint`, `rerp brrtrouter gen`, `rerp bff spec-gen`, `rerp tilt ensure-image-and-push`. The Tiltfile passes args (e.g. `--spec`, `--suite`, `--output-dir`); the tooling uses `rerp_tooling.discovery` where paths or service lists must be suite-agnostic. Invoke via `rerp` (when `tooling`’s .venv is on PATH) or `./tooling/.venv/bin/rerp` for a fixed path in Tilt/just.

| Embedded logic (Tiltfile) | `rerp` subcommand (in `./tooling`) | Purpose | Args / behaviour |
|---------------------------|------------------------------------|---------|------------------|
| **create_microservice_lint**, **bff-lint** | `rerp brrtrouter lint` | Run `brrtrouter-gen lint` on an OpenAPI spec. Prefer `../BRRTRouter/target/debug/brrtrouter-gen` if it exists, else `cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen --`. | `--spec <path>`. `--brrtrouter-dir` (default `../BRRTRouter`). |
| **create_microservice_gen** | `rerp brrtrouter gen` | Run `brrtrouter-gen generate`, then `fix_cargo_toml_paths` on the generated `Cargo.toml`. | `--spec <path>`, `--output-dir <dir>` or `--suite <s> --name <n>`. `--brrtrouter-dir`. No `accounting` literal. |
| **bff-spec-gen** | `rerp bff spec-gen` | Run `bff-generator generate-spec` with config/output from `openapi/{suite}/`. Optional `--deps-json` for Tiltfile `deps`. | `--suite <s>`. |
| **custom_build** (ensure image + push/load) | `rerp tilt ensure-image-and-push` | If image missing, run build; then `docker push $EXPECTED_REF` or `kind load`. | `--image-name`, `--dockerfile`, `--hash-path`, `--artifact-path`, `--tag`. Reads `EXPECTED_REF` from env. |
| **accounting-all-gens** | (optional) `rerp noop` | No-op with a message. | `[message]`. Low priority. |

**Tiltfile after externalisation (intended invocations):**  
Use `rerp` when the tooling .venv is on PATH, or `./tooling/.venv/bin/rerp` for an explicit path. Example:

- **create_microservice_lint:**  
  `rerp brrtrouter lint --spec ./openapi/accounting/general-ledger/openapi.yaml`
- **bff-lint:**  
  `rerp brrtrouter lint --spec ./openapi/accounting/openapi_bff.yaml`
- **create_microservice_gen:**  
  `rerp brrtrouter gen --spec ./openapi/accounting/general-ledger/openapi.yaml --output-dir general-ledger` or `--suite accounting --name general-ledger`
- **bff-spec-gen:**  
  `rerp bff spec-gen --suite accounting`. `deps` from `rerp bff spec-gen --suite accounting --deps-json` or from `Tiltfile.generated.star`.
- **custom_build cmd:**  
  `rerp tilt ensure-image-and-push --image-name %s --dockerfile %s --hash-path %s --artifact-path %s` (and `EXPECTED_REF` in env).

**BRRTRouter path:** scripts should accept `--brrtrouter-dir` (default `../BRRTRouter` from repo root) so CI or different clones can override. Logic: if `{brrtrouter_dir}/target/debug/brrtrouter-gen` exists and is executable, use it; else `cargo run --manifest-path {brrtrouter_dir}/Cargo.toml --bin brrtrouter-gen --`.

---

## 6. Tiltfile

- **Goal:** Remove `PACKAGE_NAMES`, `BINARY_NAMES`, `ACCOUNTING_SERVICES`, and hardcoded `get_service_port` / `bff-spec-gen` deps; **replace all embedded bash** (§2.4) with `rerp` subcommands (§5.6).
- **Options:**
  1. **Export from Python:** `rerp tiltfile generate` writes `Tiltfile.generated.star` (or, as fallback, `.tiltfile_data.json`/`.yaml`) with `services`, `package_names`, `binary_names`, `ports`, and `bff_spec_deps`. Tiltfile (Starlark) reads that JSON if Tilt supports it, or a Starlark `load()` of a generated `.star` that defines dicts.
  2. **Generate a Tiltfile fragment:** e.g. `Tiltfile.generated.star` that defines `PACKAGE_NAMES`, `BINARY_NAMES`, `SERVICES`, `get_service_port`, and the list for `bff-spec-gen` deps. The main `Tiltfile` does `load('Tiltfile.generated.star', 'PACKAGE_NAMES', 'BINARY_NAMES', ...)` and uses them. The fragment is generated by `rerp tiltfile generate` from `iter_services()` and bff-suite-config.
  3. **Fully generated Tiltfile:** possible but larger change; not required for first phase.

- **Recommendation:** Option 2: generate `Tiltfile.generated.star` (or `Tiltfile.generated.bzl`) from discovery. The generator runs when:
  - `just dev-up` / `tilt up` (as a prep step), or
  - Manually / in CI when services change.
- **bff-spec-gen deps:** from bff-suite-config: `[openapi/{suite}/{n}/openapi.yaml for n in services]` for each suite that has a BFF; plus `openapi/{suite}/openapi_bff.yaml` as an ignore. No accounting-specific list.

---

## 7. File and CLI Layout

### 7.1 New / Changed Files (Target: `./tooling`)

The **`./scripts`** directory is **replaced by `./tooling`**: a proper Python package with `pyproject.toml`, an isolated `.venv`, and a single `rerp` CLI. All previous scripts become `rerp` subcommands or library code.

```
tooling/
  pyproject.toml            # [project], deps (pyyaml, toml, typer/click, ...), [project.scripts] rerp = "rerp_tooling.cli:main"
  .venv/                    # gitignored; python -m venv .venv && .venv/bin/pip install -e .
  src/
    rerp_tooling/
      __init__.py
      cli/
        __init__.py
        main.py             # entry; subcommands: ports, build, docker, tilt, brrtrouter, bff, tiltfile, noop
        ports.py            # rerp ports assign|list|validate|update-configs|reconcile|fix-duplicates
        build.py            # rerp build microservice|workspace|push-containers
        docker.py           # rerp docker render
        tilt.py             # rerp tilt setup|teardown|ensure-image-and-push
        brrtrouter.py       # rerp brrtrouter lint|gen
        bff.py              # rerp bff spec-gen
        tiltfile.py         # rerp tiltfile generate  (writes Tiltfile.generated.star)
      discovery/
        __init__.py         # iter_services, suites_with_bff, iter_bffs, ServiceInfo, ...
        suites.py
        services.py
        ports.py            # thin wrapper over port-registry.json
      docker/
        __init__.py
        render.py           # render logic (used by rerp docker render)
  tests/
    ...

# Project root (default; overridable via RERP_PORT_REGISTRY or --registry)
port-registry.json

docker/microservices/
  Dockerfile.template       # new contract (microservices layout, variables above)
  # Dockerfile.*            # generated by rerp docker render; commit or .gitignore

# Generated (project root or configured path)
Tiltfile.generated.star     # from rerp tiltfile generate; defines PACKAGE_NAMES, BINARY_NAMES, PORTS, BFF_DEPS, BFF_SPEC_DEPS
```

**CLI usage:** `rerp` when `tooling`'s `.venv` is on PATH, or `./tooling/.venv/bin/rerp` for an explicit path (e.g. in Tiltfile, just, CI). Install: `cd tooling && python -m venv .venv && .venv/bin/pip install -e .` (or `uv sync`), then `rerp --help`.

### 7.2 Migration from `./scripts` to `./tooling` and Deprecations

**Migration:** All logic in `./scripts` is **migrated into `./tooling`** as the `rerp_tooling` package and `rerp` subcommands. The `./scripts` directory is **deprecated** and **removed** once migration is complete; any remaining references (Tiltfile, just, CI, docs) must point to `rerp` or `./tooling/.venv/bin/rerp`.

**Script → `rerp` / library:**
- `assign-port.py` → `rerp ports` (assign, list, validate, update-configs, reconcile, fix-duplicates)
- `build-microservice.sh` → `rerp build microservice` | `rerp build workspace`
- `build-and-push-microservice-containers.sh` → `rerp build push-containers`
- `generate-dockerfile.py` / `render_dockerfiles.py` → `rerp docker render`
- `setup-tilt.sh`, `teardown-tilt.sh`, `setup-kind-registry.sh`, `setup-persistent-volumes.sh` → `rerp tilt setup`, `rerp tilt teardown`, and (in tooling) `rerp kind registry`, `rerp k8s persistent-volumes` or equivalent
- **Tiltfile-embedded** (brrtrouter lint/gen, bff spec-gen, ensure-image-and-push) → `rerp brrtrouter lint`, `rerp brrtrouter gen`, `rerp bff spec-gen`, `rerp tilt ensure-image-and-push`
- Tiltfile data generator → `rerp tiltfile generate`

**Deprecate (components) or later phase:**  
`build-microservice-docker.sh`, `build-multiarch-docker.sh`, `copy-microservice-binary.sh`, `copy-multiarch-binary.sh`.

**Generated / removed:**  
Static `Dockerfile.general-ledger` … `Dockerfile.bff` → generated by `rerp docker render`. **Tiltfile embedded bash** in `*-lint`, `*-service-gen`, `bff-spec-gen`, `bff-lint`, `custom_build` → replaced by `rerp` subcommands. `live_update` `run('kill -HUP 1', ...)` stays; `create_microservice_build` (components) can be removed or left unused.

### 7.3 Implementation Status: What’s Done, Gaps, and Distance to Deleting `./scripts`

**As of 2025‑01:** This section records what exists in `./tooling` (as `rerp` or library) vs what remains only in `./scripts`, and which callers (CI, Tiltfile, justfile) still invoke `./scripts`. It is updated as migration progresses.

#### Scripts in `./scripts`: Done vs Gap

| Script | Status | `rerp` / module replacement | Callers still using script |
|--------|--------|-----------------------------|----------------------------|
| `assign-port.py` | **Done** | `rerp ports` (assign, list, validate, update-configs, reconcile, fix-duplicates) | **justfile:** `validate-ports`, `fix-duplicate-ports` still call `./scripts/assign-port.py` — should be switched to `tooling/.venv/bin/rerp ports validate` and `rerp ports fix-duplicates`. CI already uses `rerp`. |
| `patch-brrtrouter-for-ci.py` | **Done** | `rerp ci patch-brrtrouter` | None; CI uses `rerp`. |
| **Inline OpenAPI validate (CI)** | **Done** | `rerp openapi validate` | None; CI uses `rerp`. |
| `generate_system_bff.py` | **Done** | `rerp bff generate-system` | None; CI uses `rerp`. |
| `generate_bff_spec.py` | **Gap** | `rerp bff generate-spec` (not impl.) | Tilt/CI prefer `bff-generator` + bff-suite-config; low priority. |
| `host-aware-build.py` | **Gap** | `rerp build components` (not impl.) | **CI:** build-and-test, build-multiarch. **Tiltfile:** `create_microservice_build` (likely dead). |
| `build-microservice.sh` | **Gap** | `rerp build microservice` (not impl.) | **CI:** build-and-test, build-push-containers. **Tiltfile:** `create_microservice_build_resource`. |
| `build-and-push-microservice-containers.sh` | **Gap** | `rerp build push-containers` (not impl.) | **CI:** build-push-containers (`--copy-only` and final push). |
| `copy-microservice-binary-simple.sh` | **Gap** | `rerp build copy-binary` or kept as thin wrapper | **Tiltfile:** `create_microservice_deployment` (copy step). |
| `build-microservice-docker-simple.sh` | **Gap** | `rerp docker build` or folded into `rerp build push-containers` | **Tiltfile:** `create_microservice_deployment` (docker step, custom_build). |
| `generate-dockerfile.py` | **Gap** | `rerp docker render` | **Tiltfile:** `create_microservice_build` (likely dead). |
| `fix_cargo_toml_paths.py` | **Gap** | Library used by `rerp brrtrouter gen` | **Tiltfile:** `create_microservice_gen`. **bootstrap_microservice.py.** |
| `bootstrap_microservice.py` | **Gap** | `rerp bootstrap microservice` | None in CI/Tilt/just; manual only. |
| `setup-tilt.sh` | **Gap** | `rerp tilt setup` | **justfile:** `setup`. |
| `teardown-tilt.sh` | **Gap** | `rerp tilt teardown` | **justfile:** `teardown`. |
| `setup-kind-registry.sh` | **Gap** | `rerp k8s kind-registry` (or equivalent) | **justfile:** `dev-up`. |
| `setup-persistent-volumes.sh` | **Gap** | `rerp k8s persistent-volumes` (or equivalent) | **justfile:** `dev-up`. |
| `tail-tilt-logs.sh` | **Gap** | `rerp tilt logs` | **justfile:** `logs`. |
| `fix_operation_id_casing.py` | **Gap** | `rerp openapi fix-operation-id-casing` | None. |
| `generate_*_openapi.py` (4) | **Gap** | `rerp openapi generate-*` | None. |
| `build-microservice-docker.sh`, `copy-microservice-binary.sh`, `copy-multiarch-binary.sh`, `build-multiarch-docker.sh` | **Defer** | Components layout; deprecate or later | **Tiltfile:** `create_microservice_build` (components path; likely dead). |

**Data / not code:** `port-registry.json` (in `scripts/` or project root), `README-port-registry.md`, `scripts/requirements.txt`. `port-registry.json` stays at root or `RERP_PORT_REGISTRY`; `requirements.txt` goes away when `./scripts` is removed.

#### Callers: What Still Invokes `./scripts`

| Caller | Scripts / commands used | When switched to `rerp` |
|--------|-------------------------|--------------------------|
| **CI** | `rerp` for: openapi validate, bff generate-system, ports validate, ci patch-brrtrouter. Still **scripts:** `host-aware-build.py`, `build-microservice.sh`, `build-and-push-microservice-containers.sh`. | Build / push: when `rerp build components`, `rerp build microservice`, `rerp build push-containers` exist. |
| **Tiltfile** | `build-microservice.sh`, `copy-microservice-binary-simple.sh`, `build-microservice-docker-simple.sh`, `fix_cargo_toml_paths.py`. Also `create_microservice_build` (host-aware-build, copy-microservice-binary, generate-dockerfile, build-microservice-docker) if that path is live. | When `rerp build microservice`, `rerp build copy-binary` (or equiv.), `rerp docker build` (or equiv.), `rerp brrtrouter gen` (which uses fix_cargo_toml_paths) exist and Tiltfile is updated. |
| **justfile** | `setup-kind-registry.sh`, `setup-persistent-volumes.sh` (`dev-up`); `setup-tilt.sh`, `teardown-tilt.sh`; `assign-port.py validate`, `assign-port.py fix-duplicates`; `tail-tilt-logs.sh`. | Ports: as soon as justfile is updated to `rerp ports validate` and `rerp ports fix-duplicates`. Tilt/k8s: when `rerp tilt setup|teardown`, `rerp k8s kind-registry`, `rerp k8s persistent-volumes`, `rerp tilt logs` exist. |

#### How Far Are We from Deprecating and Deleting `./scripts`?

**Done (can remove script and use only `rerp`):**  
`assign-port.py`, `patch-brrtrouter-for-ci.py`, `generate_system_bff.py`. CI already uses `rerp` for these. **Remaining:** point justfile `validate-ports` and `fix-duplicate-ports` at `rerp` and remove `assign-port.py` from the “in use” set. `assign-port.py` can only be deleted once **all** references (including justfile) use `rerp`; today justfile still uses it.

**Blockers to delete `./scripts` entirely:**

1. **Build and Docker (highest impact)**  
   - `rerp build components` (replaces `host-aware-build.py`)  
   - `rerp build microservice` (replaces `build-microservice.sh`)  
   - `rerp build push-containers` (replaces `build-and-push-microservice-containers.sh`)  
   - `rerp docker render` and `rerp docker build` or equivalent (replace `generate-dockerfile.py`, `build-microservice-docker-simple.sh`)  
   - Copy step: `rerp build copy-binary` or absorption into `rerp build push-containers` (replaces `copy-microservice-binary-simple.sh`)  
   - **Callers to update:** CI (build-and-test, build-multiarch, build-push-containers), Tiltfile (`create_microservice_build_resource`, `create_microservice_deployment` copy/docker/custom_build).

2. **Brrtrouter and BFF codegen**  
   - `rerp brrtrouter gen` (wraps brrtrouter-gen + `fix_cargo_toml_paths`); `rerp bff spec-gen` (wraps bff-generator).  
   - **Callers:** Tiltfile (`create_microservice_gen`, `bff-spec-gen`). These can stay as-is for a while if we only target “delete `./scripts`” first; `fix_cargo_toml_paths.py` is also used by `bootstrap_microservice.py`.

3. **Tilt / k8s helpers**  
   - `rerp tilt setup`, `rerp tilt teardown`, `rerp tilt logs`, `rerp k8s kind-registry`, `rerp k8s persistent-volumes`.  
   - **Callers:** justfile (`dev-up`, `setup`, `teardown`, `logs`).  
   - These unblock deletion of the corresponding shell scripts but are less critical than build/Docker for CI and Tilt.

4. **Justfile (quick win)**  
   - `validate-ports` → `rerp ports validate`  
   - `fix-duplicate-ports` → `rerp ports fix-duplicates`  
   - No new `rerp` commands; only justfile edits. After that, `assign-port.py` has no remaining callers and can be removed.

5. **Optional / low priority**  
   - `rerp bff generate-spec`, `rerp openapi fix-operation-id-casing`, `rerp openapi generate-*`, `rerp bootstrap microservice`.  
   - Not required to delete `./scripts` if we move or retire the scripts and leave no references.

**Rough distance:**

- **Close (1–2 days):**  
  - **Delete now (no remaining callers):** `patch-brrtrouter-for-ci.py`, `generate_system_bff.py` — CI uses `rerp` only.  
  - **Justfile:** switch `validate-ports` and `fix-duplicate-ports` to `rerp ports validate` and `rerp ports fix-duplicates` (e.g. `tooling/.venv/bin/rerp` after `just init`). Then delete `assign-port.py`.  
  - Optionally implement `rerp tilt logs` (thin around `tilt logs`) and switch justfile `logs`; enables deleting `tail-tilt-logs.sh`.

- **Medium (1–2 sprints):**  
  - `rerp build components` (from `host-aware-build.py`), `rerp build microservice`, `rerp build push-containers`, and copy/docker steps; wire CI and Tiltfile to them.  
  - `rerp docker render` and, if we want to remove `build-microservice-docker-simple.sh`, `rerp docker build` or its logic inside `rerp build push-containers`.

- **Larger (Tiltfile and brrtrouter/bff):**  
  - `rerp brrtrouter lint`, `rerp brrtrouter gen` (including `fix_cargo_toml_paths`), `rerp bff spec-gen`; `rerp tilt ensure-image-and-push`; move `fix_cargo_toml_paths` into tooling as a library.  
  - Tiltfile: replace embedded bash and script calls with `rerp`; optionally `rerp tiltfile generate` and generated `PACKAGE_NAMES` / `BINARY_NAMES` / deps.  
  - `rerp tilt setup`, `rerp tilt teardown`, `rerp k8s kind-registry`, `rerp k8s persistent-volumes`; justfile `dev-up`, `setup`, `teardown` to `rerp`.

**Summary:** We can deprecate and delete a **first batch** of scripts (`assign-port.py`, `patch-brrtrouter-for-ci.py`, `generate_system_bff.py`) as soon as justfile is updated for ports. **Full** deletion of `./scripts` is blocked mainly by **build** and **Docker** (`host-aware-build`, `build-microservice.sh`, `build-and-push-microservice-containers.sh`, `copy-microservice-binary-simple.sh`, `build-microservice-docker-simple.sh`, `generate-dockerfile.py`) and by **Tiltfile** and **brrtrouter/bff** wiring. A practical order: (1) justfile ports + remove the three done scripts, (2) `rerp build *` and `rerp docker *` and switch CI + Tiltfile, (3) brrtrouter/bff and Tiltfile embedded logic, (4) tilt/k8s and justfile `dev-up`/`setup`/`teardown`/`logs`.

---

## 8. Implementation Order

0. **Create `./tooling` and migrate from `./scripts`**
   - Create `tooling/pyproject.toml` with `[project]`, deps, and `[project.scripts] rerp = "rerp_tooling.cli:main"`.
   - Create `tooling/.venv` (gitignored), `tooling/src/rerp_tooling/` layout (cli, discovery, docker), and `rerp` entry point.
   - Move `port-registry.json` to project root if it lives in `scripts/`; document `RERP_PORT_REGISTRY` / `--registry`.
   - Migrate scripts into `rerp_tooling` as libraries and `rerp` subcommands; retain `./scripts` only as the migration source until step 6, then deprecate/remove.

1. **Discovery library (in `./tooling`)**
   - Create `tooling/src/rerp_tooling/discovery/` (suites, services, ports). Move assign-port helpers into it; add `iter_microservice_crates`, `iter_services`, `ServiceInfo`.
   - Add overrides: `metadata.bff_binary_name`, `services.{n}.binary_name` in bff-suite-config.
   - Implement `rerp ports` subcommand on top of `rerp_tooling.discovery`.

2. **Docker templating (in `./tooling`)**
   - Define `docker/microservices/Dockerfile.template` for the microservices layout.
   - Implement `rerp docker render` using `iter_services()` and port-registry.
   - Run once, diff against current `Dockerfile.*`; fix `binary_name`/overrides and Cargo `[[bin]]` where needed (e.g. BFF, invoice) so rendered files match intended behaviour.
   - Remove static Dockerfiles and commit generated ones (or choose .gitignore strategy).

3. **Build / copy (in `./tooling`)**
   - `rerp build microservice` and `rerp build workspace` (replace `build-microservice.sh`).
   - `rerp build push-containers` (replace `build-and-push-microservice-containers.sh`).
   - Keep `copy-microservice-binary-simple.sh` or add `rerp build copy-binary --service` that uses discovery; Tiltfile can be updated to use it.

4. **Setup / teardown (in `./tooling`)**
   - `rerp tilt setup`, `rerp tilt teardown`, and equivalents for `setup_kind_registry`, `setup_persistent_volumes`.

4b. **Tiltfile-embedded scripts → `rerp` subcommands (in `./tooling`)**
   - `rerp brrtrouter lint`: `--spec`, `--brrtrouter-dir`; try debug binary then `cargo run`. Replace inline bash in `create_microservice_lint` and `bff-lint`.
   - `rerp brrtrouter gen`: `--spec`, `--output-dir` or `--suite`/`--name`; generate then `fix_cargo_toml_paths`. Replace inline bash in `create_microservice_gen`. No `microservices/accounting` literal.
   - `rerp bff spec-gen`: `--suite`; run `bff-generator` with paths from bff-suite-config. Optional `--deps-json` for Tiltfile `deps`. Replace inline bash in `bff-spec-gen`.
   - `rerp tilt ensure-image-and-push`: `--image-name`, `--dockerfile`, `--hash-path`, `--artifact-path`, `--tag`; read `EXPECTED_REF` from env. If image missing, run build; then `docker push` or `kind load`. Replace inline bash in `custom_build`.
   - (Optional) `rerp noop` for `accounting-all-gens`.

5. **Tiltfile**
   - `rerp tiltfile generate` to produce `Tiltfile.generated.star`.
   - Update Tiltfile: load generated dicts and BFF deps; replace all embedded bash with `rerp brrtrouter lint`, `rerp brrtrouter gen`, `rerp bff spec-gen`, `rerp tilt ensure-image-and-push`; remove `PACKAGE_NAMES`, `BINARY_NAMES`, `ACCOUNTING_SERVICES`, hardcoded `get_service_port` and bff-spec deps. Remove or leave unused `create_microservice_build` (components).

6. **Bootstrap and docs**
   - Point `bootstrap_microservice.py` (or `rerp bootstrap microservice`) at `--suite`, `rerp docker render`, and `rerp tiltfile generate`.
   - Update `tooling/README.md`, `AGENT.md`, and `.agent/memory-bank` to describe discovery, `rerp` CLI, Docker rendering, Tiltfile generation, and that new suites only require bff-suite-config + `microservices/{suite}/{name}/` + port-registry (and optionally `rerp tiltfile generate`). **Deprecate and remove `./scripts`** once all callers use `rerp` or `./tooling/.venv/bin/rerp`.

---

## 9. Adding a New Suite (After Implementation)

1. Add `openapi/{suite}/bff-suite-config.yaml` with `bff_service_name`, `services`, and `metadata.bff_binary_name` if the BFF binary name differs from Cargo.
2. Add `openapi/{suite}/{name}/openapi.yaml` for each backend; generate `openapi/{suite}/openapi_bff.yaml` (e.g. `bff-generator`).
3. Create `microservices/{suite}/{name}/` (brrtrouter-gen or bootstrap) and add to `microservices/Cargo.toml`.
4. Run `rerp ports assign` (and `rerp ports update-configs`) for new services; ensure `port-registry.json` and Helm have them.
5. Run `rerp docker render` → new `Dockerfile.{registry_name}`.
6. Run `rerp tiltfile generate` to regenerate Tiltfile data; run `rerp build microservice` / `rerp build push-containers` as needed.

No edits to `rerp ports`, `rerp build`, `rerp docker render`, or `rerp tilt teardown` when only adding a suite.

---

## 10. Open Points

- **Components layout:** Full migration of `host-aware-build`, `build-microservice-docker`, `copy-microservice-binary`, `copy-multiarch-binary`, `build-multiarch-docker` to discovery and to `components/`-aware logic is out of scope for this design; they can remain as-is or be deprecated.
- **Tilt load() of generated Starlark:** Confirm Tilt supports `load('Tiltfile.generated.star', 'PACKAGE_NAMES', ...)`. If not, use JSON + a small Starlark helper or inline generation of a `.star` that only contains data.
- **`binary_name` vs Cargo:** Align BFF and `invoice` (and any other mismatch) via `[[bin]]` in Cargo and/or bff-suite-config overrides; decide before mass render.
- **CI:** Add `rerp docker render` (and optionally `rerp tiltfile generate`) to CI so PRs that add services get correct Dockerfiles and Tiltfile data; add a check that `Dockerfile.*` match `rerp docker render` output if we commit them.
- **Tilt `custom_build` and `EXPECTED_REF`:** `rerp tilt ensure-image-and-push` must read `EXPECTED_REF` from the environment; Tilt sets it in `custom_build`. Verify that `docker push $EXPECTED_REF` and `kind load docker-image $EXPECTED_REF` work when invoked by Tilt (e.g. no extra shell wrapping that drops env).
- **`./tooling` setup:** Decide whether `.venv` lives in `tooling/` or at repo root (with `pip install -e ./tooling`). Document `pyproject.toml` build/install and migration path from `./scripts` (e.g. parallel run during cutover, or big-bang once all `rerp` subcommands exist).

---

## 11. Summary

| Deliverable | Description |
|-------------|-------------|
| **`./tooling`** | pyproject-based package with `.venv`, `rerp_tooling` (discovery, docker, cli), and `rerp` CLI. Replaces `./scripts`. |
| **`rerp_tooling.discovery`** | Shared module: suites, BFFs, `iter_services()` with Cargo + bff-suite-config + port-registry. `rerp ports` and all other subcommands use it. |
| **`rerp docker render`** | Renders `docker/microservices/Dockerfile.{registry_name}` from one template using `iter_services()`. Removes need for static Dockerfiles when adding suites. |
| **`rerp` subcommands** | `rerp ports`, `rerp build microservice|workspace|push-containers`, `rerp tilt setup|teardown|ensure-image-and-push`, `rerp brrtrouter lint|gen`, `rerp bff spec-gen`, `rerp tiltfile generate`; discovery-driven, no suite literals. |
| **Tiltfile-embedded → `rerp`** | `rerp brrtrouter lint`, `rerp brrtrouter gen`, `rerp bff spec-gen`, `rerp tilt ensure-image-and-push`; replace all inline bash in `*-lint`, `*-service-gen`, `bff-spec-gen`, `bff-lint`, `custom_build`. Suite-aware args; no `accounting` or path literals. |
| **`rerp tiltfile generate`** | Writes `Tiltfile.generated.star` with `PACKAGE_NAMES`, `BINARY_NAMES`, ports, BFF deps, BFF spec deps. |
| **bff-suite-config** | Optional `metadata.bff_binary_name`, `services.{n}.binary_name` for overrides. |
| **Cargo** | Add `[[bin]]` where we want a binary name different from `[package].name` (e.g. BFF, invoice) to reduce overrides. |

---

*End of design document.*
