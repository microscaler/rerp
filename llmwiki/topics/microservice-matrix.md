# 71-Service Microservice Matrix

> Overview of all 71 RERP microservices organized by implementation phase and suite.

**Status:** historical synthesis (source audit moved to `docs/history/audits/MICROSERVICE_MATRIX_AUDIT.md`)

## Phase 1: Core Foundation (7 services)
- auth/identity
- infrastructure/observability
- product/catalog
- *(plus 4 more in core foundation)*

## Phase 2: Business Operations (14 services)
- crm/contacts
- sales/orders
- purchase/procurement
- inventory/warehouse
- *(plus 10 more in business operations)*

## Phase 3: Financial & HR (16 services)
- accounting/general-ledger
- accounting/invoice
- accounting/accounts-receivable
- accounting/accounts-payable
- accounting/bank-sync
- accounting/asset
- accounting/budget
- accounting/edi
- accounting/financial-reports
- hr/employees
- *(plus 6 more in financial/HR)*

## Phase 4: Advanced Operations (7 services)
- manufacturing/production
- project/management
- *(plus 5 more in advanced operations)*

## Phase 5: Customer-Facing (10 services)
- marketing/campaigns
- website/content
- pos/register
- helpdesk/tickets
- marketplace/integration
- *(plus 5 more in customer-facing)*

## Phase 6: Extensions (5 services)
- ai/llm-wiki
- analytics/dashboard
- automation/rules-engine
- data/etl
- documents/storage
- *(plus additional services for IoT, ESG, etc.)*

## Additional Services
- esg/sustainability
- iot/sensor
- localization/i18n
- field-service/dispatch
- appointments/scheduling
- approvals/workflow

## Key Stats
- **Total services:** 71
- **Total crates:** 142 (71 generated + 71 implementation)
- **Suites with BFF:** accounting (1), others TBD
- **Shared DB pool:** 1 Postgres instance, `DB_POOL_MAX` env var
- **Frontend:** bypasses BFF via vite proxy

## Code Anchors
- Full spec list: `openapi/*/openapi.yaml`
- Historical audit: `docs/history/audits/MICROSERVICE_MATRIX_AUDIT.md`
- Workspace: `microservices/Cargo.toml`
