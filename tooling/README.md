# RERP Tooling — `rerp` CLI

Suite-aware wrapper around `brrtrouter_tooling` that translates brrtrouter commands into
RERP-style names and conventions. The wrapper lives in
`tooling/src/rerp_tooling/cli/main.py` and is installed as the `rerp` entry
point via `pyproject.toml`.

> **Important for future LLM sessions**
>
> RERP must stay suite-nested because it contains multiple suites of systems:
> `openapi/{suite}/{service}` and `microservices/{suite}/{service}`. Do not
> flatten it to Hauliage's directory layout. Hauliage is the working reference
> for naming and build semantics only: implementation crates are separate from
> generated crates, and generated crates use the `_gen` suffix.

## Setup

```bash
just init          # creates tooling/.venv with rerp-tooling installed
tooling/.venv/bin/rerp --help
```

The venv is at `tooling/.venv/`. All tooling commands go through the wrapper
binary:

```bash
tooling/.venv/bin/rerp <command> [args...]
```

## Command Reference

### `gen suite <suite> --service <name>`

Generate both the **gen** crate (request/response types, handlers, registry)
and the **impl** crate (stub binary) from an OpenAPI spec.

```bash
rerp gen suite accounting --service general-ledger
```

**What it does**

1. Discovers `openapi/accounting/general-ledger/openapi.yaml`.
2. Runs `brrtrouter-gen generate` against that spec.
3. Writes `microservices/accounting/general-ledger/gen/` (lib crate).
4. Writes `microservices/accounting/general-ledger/impl/` (binary crate).
5. Patches package names to RERP convention:
   - gen crate: `rerp_accounting_general_ledger_gen`
   - impl crate: `rerp_accounting_general_ledger`

Without `--service`, regenerates **all** services in the suite.

**Key flags**

| Flag | Description |
|------|-------------|
| `--service <name>` | Generate only this service (default: all in suite) |
| `--force` | Overwrite existing files |
| `--dry-run` | Show what would change without writing |
| `--only <parts>` | Limit regeneration (e.g. `handlers,types`) |

### `gen stubs <suite> <name> [--force] [--sync]`

Generate only the **impl** stub (main.rs + controller stubs) from the existing
gen crate and OpenAPI spec. Useful after the gen crate has been created and you
want fresh impl scaffolding.

```bash
rerp gen stubs accounting general-ledger --force
```

### `build microservice <name>`

Build a single microservice impl crate via Cargo.

```bash
rerp build microservice general-ledger
```

Translates to:

```bash
cargo build -p rerp_accounting_general_ledger
```

The wrapper resolves the suite from `openapi/{suite}/{service}/openapi.yaml`,
then reads `microservices/{suite}/{service}/impl/Cargo.toml` to find the actual
implementation package. This prevents `cargo build -p` from accidentally
targeting the generated crate when a checkout is mid-migration.

Use `--suite <name>` or `RERP_SUITE=<name>` when the same service name exists in
more than one suite.

### `docker build-image-simple <image> <template> <hash> <artifact>`

Build a Docker image for a microservice.

```bash
rerp docker build-image-simple \
    my-image \
    Dockerfile.accounting_general-ledger \
    /path/to/hash \
    /path/to/artifact \
    --service general-ledger \
    --port 8001
```

Translates the `--service` flag to `--module` (with `-` → `_` conversion) and
adds system/module metadata for the underlying brrtrouter docker tool.

**Flags**

| Flag | Description |
|------|-------------|
| `--system <suite>` | Override suite (default: from --service context) |
| `--service <name>` | Service name (converted to module with - → _) |
| `--module <name>` | Module name (alternative to --service) |
| `--port <n>` | Container port |
| `--binary-name <name>` | Custom binary name |
| `--no-cache` | Disable Docker cache |
| `--prune-dangling` | Prune dangling images after build |
| `--dev-sync-only` | Only sync files, don't build image |

### `docker build-base`

Build the base Docker image shared across all service images.

```bash
rerp docker build-base
```

### `bff generate-system`

Generate suite-local BFF specs by aggregating each suite's microservices.

```bash
rerp bff generate-system
```

Default output is `openapi/{suite}/openapi_bff.yaml` for every suite with a
`bff-suite-config.yaml`. For one suite:

```bash
rerp bff generate-system --suite accounting
```

### `bff generate`

Generate an individual suite BFF spec.

```bash
rerp bff generate --suite accounting
```

### `pre-commit microservices-fmt`

Run cargo fmt across all microservices crates and rustfmt across entities.

```bash
rerp pre-commit microservices-fmt
```

## Package Naming Convention

The wrapper patches the default brrtrouter package naming to use the RERP
convention:

| Component | Package name |
|-----------|-------------|
| gen crate | `rerp_<suite>_<module>_gen` |
| impl crate | `rerp_<suite>_<module>` |

Example: `general-ledger` in the `accounting` suite produces:
- `rerp_accounting_general_ledger_gen` (gen lib crate)
- `rerp_accounting_general_ledger` (impl binary crate)

This is enforced by the wrapper's suite-aware package-name callbacks in
`tooling/src/rerp_tooling/cli/main.py`.

> **Naming Drift Guardrail (April 2026)**
>
> The wrapper now protects the Hauliage-style split while preserving RERP's
> nested suites:
> 1. `rerp gen suite <suite> --service <name>` names generated crates
>    `rerp_<suite>_<module>_gen`.
> 2. `rerp build microservice <name>` reads the impl manifest and builds the
>    implementation package, not the generated package.
> 3. `rerp bff generate-system` writes `openapi/{suite}/openapi_bff.yaml`.
>
> If an agent sees existing mixed names such as `<module>_service_api` or
> `*_impl`, treat them as migration state. Do not encode those names as the
> desired convention.

## Known Issues

### ArcSwap Pattern (Phase 1)

All impl crates must use `arc_swap::ArcSwap<Router>` instead of `RwLock<Router>`:

```rust
// ✅ Correct (ArcSwap — used by AppService::new())
let router = std::sync::Arc::new(arc_swap::ArcSwap::from_pointee(Router::new(routes.clone())));
router.load().dump_routes();

// ❌ Wrong (RwLock — causes type mismatch with AppService::new())
let router = std::sync::Arc::new(std::sync::RwLock::new(Router::new(routes.clone())));
router.read().unwrap().dump_routes();
```

Every impl crate needs `arc-swap = { workspace = true }` in its Cargo.toml.

## Architecture

```
rerp (tooling/.venv/bin/rerp)
  └── rerp_tooling.cli.main:main()
        ├── gen suite     → brrtrouter_tooling.cli.gen_cmd.run_gen_argv()
        ├── gen stubs     → brrtrouter_tooling.cli.gen_cmd.run_gen_argv()
        ├── build         → brrtrouter_tooling.cli.build.run_build_argv()
        ├── docker        → brrtrouter_tooling.cli.docker_cmd.run_docker_argv()
        ├── bff           → suite planner + brrtrouter_tooling.cli.bff.run_bff_generate_system()
        └── pre-commit    → inline script (cargo fmt + rustfmt)
```

The wrapper prepends the brrtrouter_tooling source path to `sys.path`, derives
suite/package data from the nested RERP tree, then translates RERP-style
commands into the brrtrouter equivalents before delegating.
