# RERP Tooling — `rerp` CLI

Thin wrapper around `brrtrouter_tooling` that translates brrtrouter commands into
RERP-style names and conventions. The wrapper lives in
`tooling/src/rerp_tooling/cli/main.py` and is installed as the `rerp` entry
point via `pyproject.toml`.

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

**Note:** The suite is currently hardcoded to `accounting`. A future improvement
is to derive it from the crate name or from `--suite`.

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

Generate the system-level BFF spec by aggregating all suite BFFs.

```bash
rerp bff generate-system
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

This is enforced by overriding `gen_cmd.default_gen_package_name` at runtime
(see `main.py` lines 172-191).

## Architecture

```
rerp (tooling/.venv/bin/rerp)
  └── rerp_tooling.cli.main:main()
        ├── gen suite     → brrtrouter_tooling.cli.gen_cmd.run_gen_argv()
        ├── gen stubs     → brrtrouter_tooling.cli.gen_cmd.run_gen_argv()
        ├── build         → brrtrouter_tooling.cli.build.run_build_argv()
        ├── docker        → brrtrouter_tooling.cli.docker_cmd.run_docker_argv()
        ├── bff           → brrtrouter_tooling.cli.bff.run_bff_*_argv()
        └── pre-commit    → inline script (cargo fmt + rustfmt)
```

The wrapper prepends the brrtrouter_tooling source path to `sys.path` so that
imports resolve without the hauliage-style workspace patches. It then translates
RERP-style commands into the brrtrouter equivalents and rewrites `sys.argv`
before delegating.
