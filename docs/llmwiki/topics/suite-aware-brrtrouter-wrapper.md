# Suite-Aware BRRTRouter Wrapper

- **Status**: `verified`
- **Source docs**: [`AGENTS.md`](../../../AGENTS.md), [`docs/TOOLS_ALIGNMENT_FINDINGS.md`](../../TOOLS_ALIGNMENT_FINDINGS.md), [`tooling/README.md`](../../../tooling/README.md)
- **Code anchors**: `tooling/src/rerp_tooling/cli/main.py`, `tooling/tests/test_rerp_cli_translations.py`, `openapi/accounting/bff-suite-config.yaml`, `microservices/Cargo.toml`
- **Last updated**: 2026-04-25

## What It Is

RERP uses BRRTRouter like Hauliage, but RERP is a multi-suite ERP and must keep a suite-nested layout. The `rerp` CLI wrapper adapts BRRTRouter tooling to this layout while preserving Hauliage's proven generated/implementation crate split.

The contract is:

- OpenAPI specs live at `openapi/{suite}/{service}/openapi.yaml`.
- Service crates live at `microservices/{suite}/{service}/gen` and `microservices/{suite}/{service}/impl`.
- Implementation crates should converge on `rerp_{suite}_{service}`.
- Generated crates should converge on `rerp_{suite}_{service}_gen`.
- Suite BFF specs are generated to `openapi/{suite}/openapi_bff.yaml`.

## Where It Lives

The wrapper entry point is `tooling/src/rerp_tooling/cli/main.py`. It prepends BRRTRouter tooling to `sys.path`, derives suite/package data from the RERP tree, then delegates to BRRTRouter's raw CLI modules.

Regression tests live in `tooling/tests/test_rerp_cli_translations.py`. These tests are important because the prior failure mode was mostly orchestration drift rather than one Rust compiler error.

## How It Works

`rerp gen suite <suite> --service <name>` patches BRRTRouter's package-name callbacks so generated crates use the `_gen` suffix:

```text
rerp_accounting_general_ledger_gen
```

This is the Hauliage naming model with a suite prefix. It must not regress to `<module>_service_api` or to a generated crate name that matches the implementation crate.

`rerp build microservice <name>` resolves the suite from `openapi/{suite}/{service}/openapi.yaml`, then reads `microservices/{suite}/{service}/impl/Cargo.toml` to find the actual implementation package. This keeps the wrapper compatible with current migration state, including services that still use package names ending in `_impl`, while preventing builds from targeting generated crates.

`rerp bff generate-system` scans suites with `bff-suite-config.yaml` and writes:

```text
openapi/{suite}/openapi_bff.yaml
```

Use `--suite <name>` or `--system <name>` for one suite. `rerp bff generate --suite accounting` is shorthand for the raw BRRTRouter/BFF suite-config flags.

The suite config is also the path namespacing contract. Each service can keep local paths such as `/payments`; the generated BFF must publish them under the configured service `base_path`, such as `/api/accounts-payable/payments` and `/api/accounts-receivable/payments`. Do not hand-rename service-local OpenAPI paths to avoid collisions.

## Gotchas And Drift

> **Do not flatten RERP to Hauliage.**
> Hauliage is the working example for naming/build semantics, not for RERP's directory shape. RERP needs nested suites because it represents multiple suites of systems.

> **Migration state exists.**
> Some current crates may still have names such as `<module>_service_api` or `*_impl`. Treat these as migration state. Do not document them as the desired convention, and do not make new automation depend on them except by reading the actual impl manifest during transition.

> **BFF output path matters.**
> Raw BRRTRouter defaults are not enough for RERP. The wrapper must explicitly write suite-local BFF specs to `openapi/{suite}/openapi_bff.yaml`, otherwise Tilt/CI can lint or build against stale BFF specs.

> **BFF public paths are namespaced.**
> The BFF generator must prefix public paths with each service `base_path` from `bff-suite-config.yaml`. Local service path collisions are valid when they belong to different service namespaces.

## Verification

Focused wrapper verification:

```bash
PYTHONPATH="tooling/src:../BRRTRouter/tooling/src" pytest tooling/tests/test_rerp_cli_translations.py -q
ruff check tooling/src/rerp_tooling/cli/main.py tooling/tests/test_rerp_cli_translations.py
ruff format --check tooling/src/rerp_tooling/cli/main.py tooling/tests/test_rerp_cli_translations.py
```

The 2026-04-25 wrapper update passed all focused tests and lints.

## Cross-References

- [`../index.md`](../index.md) — Wiki catalog.
- [`hauliage-reference-operating-model.md`](./hauliage-reference-operating-model.md) — Hauliage patterns translated to RERP's nested suites.
- [`service-implementation-and-database-layout.md`](./service-implementation-and-database-layout.md) — RERP service/database responsibilities.
- [`../../TOOLS_ALIGNMENT_FINDINGS.md`](../../TOOLS_ALIGNMENT_FINDINGS.md) — RERP vs Hauliage analysis that identified the layout/tooling drift.
- [`../../../tooling/README.md`](../../../tooling/README.md) — User-facing wrapper command reference.
- [`../../../.agent/memory-bank/suiteArchitecture.md`](../../../.agent/memory-bank/suiteArchitecture.md) — Memory Bank summary of the suite model.
