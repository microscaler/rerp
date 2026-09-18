# RERP Suite Architecture

RERP is composed of **suites of systems**. Each suite has:

1. **Microservices** — `openapi/{suite}/{name}/openapi.yaml` (e.g. `openapi/accounting/general-ledger/openapi.yaml`)
2. **One BFF per suite** — `openapi/{suite}/bff-suite-config.yaml` and generated `openapi/{suite}/openapi_bff.yaml` that aggregates that suite’s services

**Example: accounting suite**
- Microservices: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports
- BFF: `openapi/accounting/bff-suite-config.yaml` (with `bff_service_name: bff`) → `openapi/accounting/openapi_bff.yaml`

**Adding a new suite with a BFF**
- Add `openapi/{suite}/bff-suite-config.yaml` with `bff_service_name: <helm/registry name>` (e.g. `bff` or `hr-bff`) and generate `openapi/{suite}/openapi_bff.yaml`
- Helm, Tiltfile, port-registry: add the new BFF service and any microservices
- **No script changes**: `assign-port.py` discovers suites by listing `openapi/` subdirs that contain `bff-suite-config.yaml` and reads `bff_service_name` from each config

**Tooling (fully dynamic, no hardcoded suites)**
- `assign-port.py`: Discovers suites via `_suites_with_bff()` (list `openapi/*/bff-suite-config.yaml`), BFF registry names from `bff_service_name` in each config, microservices by walking `openapi/{suite}/{name}/openapi.yaml`. No `BFF_SERVICE_TO_SUITE` or accounting-specific logic.
- `rerp` CLI wrapper: RERP stays nested. Do **not** flatten to Hauliage's directory layout. Use Hauliage as the naming/build reference only: impl crate `rerp_{suite}_{service}`, generated crate `rerp_{suite}_{service}_gen`. The wrapper resolves suite from `openapi/{suite}/{service}/openapi.yaml`, reads the impl `Cargo.toml` package for builds, and writes BFF output to `openapi/{suite}/openapi_bff.yaml`.
- `bff-generator`: `--config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml`
