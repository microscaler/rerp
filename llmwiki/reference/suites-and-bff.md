# Suite Organization and BFF Mapping

> How suites are organized, how BFFs are generated, and the dynamic discovery mechanism.

**Status:** partially-verified

## Suite Definition

A suite is a directory under `openapi/` that contains:
- `bff-suite-config.yaml` — declares the BFF service name and lists microservices
- `{service-name}/openapi.yaml` — one per microservice in the suite

## Dynamic Discovery

Suites are **not hardcoded**. Tooling (`rerp bff generate-system`, `bff-generator`) dynamically:
1. Lists `openapi/` subdirs
2. Filters those containing `bff-suite-config.yaml`
3. Reads `bff_service_name` from each config
4. Walks all `openapi/{suite}/{name}/openapi.yaml` for microservice specs

Adding a new suite only requires adding `openapi/{suite}/bff-suite-config.yaml` — no code changes needed.

## Current Suite: Accounting

The only fully-realized suite in the project:

| Microservice | OpenAPI Path | BFF-agg'd |
|-------------|-------------|-----------|
| general-ledger | `openapi/accounting/general-ledger/` | Yes |
| invoice | `openapi/accounting/invoice/` | Yes |
| accounts-receivable | `openapi/accounting/accounts-receivable/` | Yes |
| accounts-payable | `openapi/accounting/accounts-payable/` | Yes |
| bank-sync | `openapi/accounting/bank-sync/` | Yes |
| asset | `openapi/accounting/asset/` | Yes |
| budget | `openapi/accounting/budget/` | Yes |
| edi | `openapi/accounting/edi/` | Yes |
| financial-reports | `openapi/accounting/financial-reports/` | Yes |

Config: `openapi/accounting/bff-suite-config.yaml`
Output: `openapi/accounting/openapi_bff.yaml`

## Other Suites (BFF status TBD)

Other suites exist under `openapi/` but may not have `bff-suite-config.yaml` yet:
- ai, analytics, appointments, approvals, auth, automation, crm, data, documents, esg, field-service, helpdesk, hr, infrastructure, inventory, iot, localization, manufacturing, marketing, marketplace, pos, product, project, purchase, sales, website

## BFF Current State

**Important:** The BFF layer is **non-functional** in the current dev setup. The frontend bypasses it entirely via vite proxy configured in `frontend/vite.config.js`. BFF redesign is planned as a future priority.

Proxy map example:
- `/api/v1/organizations/*` → `localhost:8009`
- `/api/v1/consignments/*` → `localhost:8003`
- `/api/v1/fleet/*` → `localhost:8002`
