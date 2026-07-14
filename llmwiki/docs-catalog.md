# RERP — Docs & Sources Catalog

> Inventory of prose docs, specs, and analysis that feed into or are cross-referenced by the wiki.

## OpenAPI Specs

| Suite | Service | Path | Status |
|-------|---------|------|--------|
| accounting | general-ledger | `openapi/accounting/general-ledger/openapi.yaml` | Active |
| accounting | invoice | `openapi/accounting/invoice/openapi.yaml` | Active |
| accounting | accounts-receivable | `openapi/accounting/accounts-receivable/openapi.yaml` | Active |
| accounting | accounts-payable | `openapi/accounting/accounts-payable/openapi.yaml` | Active |
| accounting | bank-sync | `openapi/accounting/bank-sync/openapi.yaml` | Active |
| accounting | asset | `openapi/accounting/asset/openapi.yaml` | Active |
| accounting | budget | `openapi/accounting/budget/openapi.yaml` | Active |
| accounting | edi | `openapi/accounting/edi/openapi.yaml` | Active |
| accounting | financial-reports | `openapi/accounting/financial-reports/openapi.yaml` | Active |
| ai | llm-wiki | `openapi/ai/llm-wiki/openapi.yaml` | Active |
| analytics | dashboard | `openapi/analytics/dashboard/openapi.yaml` | Active |
| appointments | scheduling | `openapi/appointments/scheduling/openapi.yaml` | Active |
| approvals | workflow | `openapi/approvals/workflow/openapi.yaml` | Active |
| auth | identity | `openapi/auth/identity/openapi.yaml` | Active |
| automation | rules-engine | `openapi/automation/rules-engine/openapi.yaml` | Active |
| crm | contacts | `openapi/crm/contacts/openapi.yaml` | Active |
| data | etl | `openapi/data/etl/openapi.yaml` | Active |
| documents | storage | `openapi/documents/storage/openapi.yaml` | Active |
| esg | sustainability | `openapi/esg/sustainability/openapi.yaml` | Active |
| field-service | dispatch | `openapi/field-service/dispatch/openapi.yaml` | Active |
| helpdesk | tickets | `openapi/helpdesk/tickets/openapi.yaml` | Active |
| hr | employees | `openapi/hr/employees/openapi.yaml` | Active |
| infrastructure | observability | `openapi/infrastructure/observability/openapi.yaml` | Active |
| inventory | warehouse | `openapi/inventory/warehouse/openapi.yaml` | Active |
| iot | sensor | `openapi/iot/sensor/openapi.yaml` | Active |
| localization | i18n | `openapi/localization/i18n/openapi.yaml` | Active |
| manufacturing | production | `openapi/manufacturing/production/openapi.yaml` | Active |
| marketing | campaigns | `openapi/marketing/campaigns/openapi.yaml` | Active |
| marketplace | integration | `openapi/marketplace/integration/openapi.yaml` | Active |
| pos | register | `openapi/pos/register/openapi.yaml` | Active |
| product | catalog | `openapi/product/catalog/openapi.yaml` | Active |
| project | management | `openapi/project/management/openapi.yaml` | Active |
| purchase | procurement | `openapi/purchase/procurement/openapi.yaml` | Active |
| sales | orders | `openapi/sales/orders/openapi.yaml` | Active |
| website | content | `openapi/website/content/openapi.yaml` | Active |

## Prose Docs

| Path | Category | Description |
|------|----------|-------------|
| `docs/ai/OPENAPI_GENERATION_COMPLETE.md` | status | 71 services OpenAPI generation complete |
| `docs/ai/BFF_GENERATION_COMPLETE.md` | status | Suite-level BFF generation status |
| `docs/ai/FIRST_CI_AUTOMATION.md` | status | CI automation implementation |
| `docs/ai/SYSTEM_BFF_GENERATION.md` | architecture | System BFF architecture |
| `docs/ai/TOP_LEVEL_SPECS_PLAN.md` | planning | Top-level OpenAPI specs plan |
| `docs/ai/CI_AUTOMATION_SETUP.md` | setup | CI setup and configuration |
| `docs/ai/ODOO_MODULES_ANALYSIS.md` | analysis | Odoo module analysis |
| `docs/ai/MICROSERVICE_MATRIX_AUDIT.md` | analysis | 71-service matrix audit |
| `docs/EXECUTIVE_SUMMARY.md` | overview | Project overview |
| `docs/adrs/` | ADRs | Architecture Decision Records |
| `docs/mermaid/` | diagrams | Architecture diagrams |

## Entity Documentation

| Path | Description |
|------|-------------|
| `docs/entities/` | Lifeguard entity documentation and diagrams |

## Configuration

| Path | Description |
|------|-------------|
| `openapi/accounting/bff-suite-config.yaml` | Accounting suite BFF config |
| `microservices/Cargo.toml` | Workspace root + crate manifest |
| `helm/` | Helm chart templates |
| `k8s/` | Kubernetes manifests |
| `docker/` | Docker images (base, build, microservices, website) |
| `tooling/` | `rerp` CLI tooling |
