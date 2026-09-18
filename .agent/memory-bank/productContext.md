# Product Context

**Accounting suite (live):** general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, **BFF**. OpenAPI in `openapi/accounting/{service}/openapi.yaml`; generated services in `microservices/accounting/`; BFF spec `openapi/accounting/openapi_bff.yaml` from `bff-generator` (`openapi/accounting/bff-suite-config.yaml`).

**Entities** (`entities/src/accounting/`): 9 domains, 47 entity files — general_ledger, invoice, accounts_receivable, accounts_payable, bank_sync (incl. Bank, BankAccount), asset, budget, edi, financial_reports. `rerp_entities`; Lifeguard for DB.

**Multi-suite (planned):** sales, hr, manufacturing, etc. Per-suite BFF; `openapi/{suite}/`; see `docs/BFF_COMPONENT_DESIGN_PROPOSAL.md`.

**Ports:** 8001–8009 accounting services, 8010 BFF. Helm `values/{service}.yaml`; namespace `rerp`.
