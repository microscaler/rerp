# Codebase Entry Points by Suite

> Where to find the main entry point for each suite's service.

**Status:** partially-verified

## Workspace Structure

- Workspace root: `microservices/Cargo.toml`
- Each service: `microservices/{suite}/{name}/gen/` (generated) + `microservices/{suite}/{name}/impl/` (business logic)

## Entry Points

### Accounting Suite

| Service | Gen Crate | Impl Crate |
|---------|-----------|------------|
| general-ledger | `microservices/accounting/general-ledger/gen/` | `microservices/accounting/general-ledger/impl/` |
| invoice | `microservices/accounting/invoice/gen/` | `microservices/accounting/invoice/impl/` |
| accounts-receivable | `microservices/accounting/accounts-receivable/gen/` | `microservices/accounting/accounts-receivable/impl/` |
| accounts-payable | `microservices/accounting/accounts-payable/gen/` | `microservices/accounting/accounts-payable/impl/` |
| bank-sync | `microservices/accounting/bank-sync/gen/` | `microservices/accounting/bank-sync/impl/` |
| asset | `microservices/accounting/asset/gen/` | `microservices/accounting/asset/impl/` |
| budget | `microservices/accounting/budget/gen/` | `microservices/accounting/budget/impl/` |
| edi | `microservices/accounting/edi/gen/` | `microservices/accounting/edi/impl/` |
| financial-reports | `microservices/accounting/financial-reports/gen/` | `microservices/accounting/financial-reports/impl/` |

### Other Suites (partial list)

| Suite | Path Pattern |
|-------|-------------|
| ai | `microservices/ai/*/` |
| analytics | `microservices/analytics/*/` |
| auth | `microservices/auth/*/` |
| crm | `microservices/crm/*/` |
| product | `microservices/product/*/` |
| sales | `microservices/sales/*/` |
| purchase | `microservices/purchase/*/` |
| inventory | `microservices/inventory/*/` |
| hr | `microservices/hr/*/` |
| manufacturing | `microservices/manufacturing/*/` |
| project | `microservices/project/*/` |
| marketing | `microservices/marketing/*/` |
| website | `microservices/website/*/` |
| pos | `microservices/pos/*/` |
| helpdesk | `microservices/helpdesk/*/` |
| marketplace | `microservices/marketplace/*/` |

Total: 71 suites × microservices = 142 crates (71 gen + 71 impl)

## Key Files

- `microservices/Cargo.toml` — Workspace root
- `tooling/src/` — `rerp` CLI tooling
- `rerp` — main automation CLI (run via `just init` → `tooling/.venv/bin/rerp`)
