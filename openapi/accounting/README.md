# Accounting Services

## Overview

Comprehensive financial management including general ledger, accounts payable/receivable, financial reporting, and compliance.

## Services

### General Ledger
- **Path**: `accounting/general-ledger/`
- **Description**: Core accounting service managing general ledger and journal entries
- **Documentation**: [General Ledger README](./general-ledger/README.md)
- **API Spec**: [General Ledger OpenAPI](./general-ledger/openapi.yaml)

### Accounts Payable
- **Path**: `accounting/accounts-payable/`
- **Description**: Accounts payable service managing vendor invoices and payments
- **Documentation**: [Accounts Payable README](./accounts-payable/README.md)
- **API Spec**: [Accounts Payable OpenAPI](./accounts-payable/openapi.yaml)

### Accounts Receivable
- **Path**: `accounting/accounts-receivable/`
- **Description**: Accounts receivable service managing customer invoices and collections
- **Documentation**: [Accounts Receivable README](./accounts-receivable/README.md)
- **API Spec**: [Accounts Receivable OpenAPI](./accounts-receivable/openapi.yaml)

### Financial Reports
- **Path**: `accounting/financial-reports/`
- **Description**: Financial reporting service generating P&L and balance sheets
- **Documentation**: [Financial Reports README](./financial-reports/README.md)
- **API Spec**: [Financial Reports OpenAPI](./financial-reports/openapi.yaml)

### Asset
- **Path**: `accounting/asset/`
- **Description**: Fixed asset management service tracking asset acquisition and depreciation
- **Documentation**: [Asset README](./asset/README.md)
- **API Spec**: [Asset OpenAPI](./asset/openapi.yaml)

### Budget
- **Path**: `accounting/budget/`
- **Description**: Budgeting service for budget creation and budget vs actual analysis
- **Documentation**: [Budget README](./budget/README.md)
- **API Spec**: [Budget OpenAPI](./budget/openapi.yaml)

### Invoice
- **Path**: `accounting/invoice/`
- **Description**: Invoice management service handling invoice creation and approval workflows
- **Documentation**: [Invoice README](./invoice/README.md)
- **API Spec**: [Invoice OpenAPI](./invoice/openapi.yaml)

### Edi
- **Path**: `accounting/edi/`
- **Description**: Electronic Data Interchange service supporting PEPPOL and UBL formats
- **Documentation**: [Edi README](./edi/README.md)
- **API Spec**: [Edi OpenAPI](./edi/openapi.yaml)

### Bank Sync
- **Path**: `accounting/bank-sync/`
- **Description**: Bank synchronization service importing bank statements and reconciliation
- **Documentation**: [Bank Sync README](./bank-sync/README.md)
- **API Spec**: [Bank Sync OpenAPI](./bank-sync/openapi.yaml)

### Tax Compliance
- **Path**: `accounting/tax-compliance/`
- **Description**: Statutory tax control service managing tax periods, jurisdiction rules, returns, payments, filings, and audit packs
- **Documentation**: [Tax Compliance README](./tax-compliance/README.md)
- **API Spec**: [Tax Compliance OpenAPI](./tax-compliance/openapi.yaml)

### Documents Extraction
- **Path**: `accounting/documents-extraction/`
- **Description**: Accounting document automation service for ingestion, classification, OCR/extraction, review, approval, and linkage
- **Documentation**: [Documents Extraction README](./documents-extraction/README.md)
- **API Spec**: [Documents Extraction OpenAPI](./documents-extraction/openapi.yaml)

### Treasury
- **Path**: `accounting/treasury/`
- **Description**: Treasury service for cash positioning, forecasts, liquidity planning, bank relationships, and controlled cash transfers
- **Documentation**: [Treasury README](./treasury/README.md)
- **API Spec**: [Treasury OpenAPI](./treasury/openapi.yaml)

### Consolidation
- **Path**: `accounting/consolidation/`
- **Description**: Multi-company consolidation service for consolidation groups, runs, eliminations, and group reporting packs
- **Documentation**: [Consolidation README](./consolidation/README.md)
- **API Spec**: [Consolidation OpenAPI](./consolidation/openapi.yaml)

### Revenue Recognition
- **Path**: `accounting/revenue-recognition/`
- **Description**: Recognition service for deferred revenue, deferred expenses, schedules, recognition rules, and recognition runs
- **Documentation**: [Revenue Recognition README](./revenue-recognition/README.md)
- **API Spec**: [Revenue Recognition OpenAPI](./revenue-recognition/openapi.yaml)

### Lease Accounting
- **Path**: `accounting/lease-accounting/`
- **Description**: Lease accounting service for leases, payment schedules, liabilities, right-of-use assets, and modifications
- **Documentation**: [Lease Accounting README](./lease-accounting/README.md)
- **API Spec**: [Lease Accounting OpenAPI](./lease-accounting/openapi.yaml)

### Audit Controls
- **Path**: `accounting/audit-controls/`
- **Description**: Cross-service accounting governance service for approvals, segregation rules, signatures, audit events, and control exceptions
- **Documentation**: [Audit Controls README](./audit-controls/README.md)
- **API Spec**: [Audit Controls OpenAPI](./audit-controls/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/accounting` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The accounting services services work together to provide complete functionality:

1. **Invoice Flow**: 
   - `invoice/` creates invoices
   - `accounts-receivable/` manages customer invoices
   - `accounts-payable/` manages vendor invoices
   - `general-ledger/` records journal entries

2. **Financial Reporting**:
   - `general-ledger/` provides transaction data
   - `financial-reports/` generates P&L, balance sheets
   - `budget/` compares actuals vs budget

3. **Compliance**:
   - `edi/` handles electronic document exchange
   - `bank-sync/` imports bank statements
   - `asset/` tracks fixed assets

## Incremental Implementation Order

The accounting suite should be delivered in value slices. Do not try to implement every documented service at once. Each slice should produce a usable accounting outcome, keep the generated BFF stable, and add only the services needed for that outcome.

### Slice 0: Contract And Runtime Baseline

Goal: keep the accounting API surface reliable while implementation proceeds.

Deliver:

- Preserve `bff-suite-config.yaml` as the accounting service inventory.
- Keep `openapi_bff.yaml` generated from all selected accounting service specs.
- Keep existing runtime services buildable and deployable: `general-ledger`, `invoice`, `accounts-receivable`, `accounts-payable`, `bank-sync`, `asset`, `budget`, `edi`, `financial-reports`, and `bff`.
- Keep the new services contract-visible but runtime-gated until scaffolded: `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, and `audit-controls`.

Visible value:

- Developers and clients can see the full target accounting surface through the generated BFF.
- New work can be planned against stable OpenAPI contracts instead of ad-hoc endpoints.

### Slice 1: Operational Core Posting

Goal: make the core accounting loop useful before adding advanced engines.

Implement first:

1. `general-ledger`
2. `invoice`
3. `accounts-receivable`
4. `accounts-payable`

Deliver:

- Chart of accounts, journals, fiscal periods, and journal entries.
- Invoice approval, posting, cancellation, and void flows.
- Customer invoice/payment application and vendor invoice/payment basics.
- GL posting references from invoice, AR, and AP workflows.

Visible value:

- A user can create invoices and bills, post accounting entries, and inspect the ledger impact.
- This establishes the source data needed by reconciliation and reporting.

### Slice 2: Cash, Reconciliation, And First Reports

Goal: give accountants daily operational value.

Implement next:

1. `bank-sync`
2. `financial-reports`
3. Supporting GL/AR/AP read models

Deliver:

- Bank accounts, statements, transactions, imports, and cash position.
- Reconciliation models, ranked suggestions, reconcile, unreconcile, write-off, and exchange-difference actions.
- Report definitions, report lines, drill-down, exports, balance sheet, income statement, cash flow, trial balance, and general ledger reports.

Visible value:

- Accountants can import bank activity, reconcile transactions, and produce first financial statements with source drill-down.
- This is the first slice that feels like an operational accounting product.

### Slice 3: Receivables And Payables Control

Goal: improve cash collection and vendor payment governance.

Implement next:

1. `accounts-receivable`
2. `accounts-payable`
3. `edi`
4. `invoice`

Deliver:

- AR follow-up policies, customer statements, collection cases, disputes, promise-to-pay, and dunning holds.
- AP payment batches, payment files, approval gates, vendor payment registration, 3-way match entry points, and supplier reporting handoff.
- EDI profiles, validation profiles, submissions, acknowledgments, retry, status, and errors.

Visible value:

- Finance teams can manage overdue customers, controlled vendor payments, and electronic document/payment handoffs.
- This adds immediate back-office value without needing full enterprise consolidation or tax depth.

### Slice 4: Assets, Budgets, And Management Reporting

Goal: broaden the product beyond transaction processing.

Implement next:

1. `asset`
2. `budget`
3. `financial-reports`
4. `general-ledger`

Deliver:

- Asset models, lifecycle events, depreciation, revaluation, disposal, modifications, and generated journal-entry links.
- Budget revisions, lifecycle states, analytic dimensions, variance analysis, and forecast comparison.
- Management reporting packs that combine actuals, budgets, and asset impacts.

Visible value:

- Management can track fixed assets, compare budget to actuals, and review operational financial performance.

### Slice 5: Document Automation

Goal: reduce manual accounting entry and improve source-document traceability.

Scaffold and implement:

1. `invoice`
2. `accounts-payable`
3. `accounts-receivable`
4. `edi`

**Note:** Document ingestion, classification, OCR/extraction, confidence scoring, and review workflows are provided by the [Documents suite](../documents/). This slice consumes the Documents suite API for document processing and links reviewed data to accounting records.

Deliver:

- Integrate with the Documents suite for document upload, classification, OCR/extraction, confidence scoring, and review workflows.
- Link reviewed document data to invoices, vendor bills, bank statement support, and EDI records via the Documents suite API.
- Conversion of reviewed document fields into accounting records.

Visible value:

- Users can turn documents into accounting records via the centralized Documents suite, with review and audit traceability.
- This is a strong differentiator once core posting and reconciliation already work.

### Slice 6: Tax, Statutory, And Localization Foundation

Goal: create the compliance extension point before adding many country packs.

Scaffold and implement:

1. `tax-compliance`
2. `edi`
3. `financial-reports`
4. `general-ledger`

Deliver:

- Tax periods, tax rules, returns, validation, submission lifecycle, tax payments, audit packs, and statutory working files.
- First reference localization pack for one jurisdiction.
- E-invoicing/statutory report hooks through EDI and reporting.

Visible value:

- RERP can support real compliance workflows for a chosen jurisdiction instead of only generic tax fields.
- This creates the pattern for future country packs.

### Slice 7: Treasury And Cash Planning

Goal: move from bank reconciliation to finance planning.

Scaffold and implement:

1. `treasury`
2. `bank-sync`
3. `accounts-payable`
4. `accounts-receivable`

Deliver:

- Cash positions, cash forecasts, liquidity plans, bank relationships, cash transfers, and funding plans.
- Forecast inputs from open AR, AP, bank balances, payment batches, and expected collections.

Visible value:

- Finance users can manage liquidity, not just record historical transactions.

### Slice 8: Revenue, Lease, And Advanced Accounting

Goal: add enterprise-grade accounting engines after the operational core is proven.

Scaffold and implement:

1. `revenue-recognition`
2. `lease-accounting`
3. `invoice`
4. `asset`
5. `general-ledger`

Deliver:

- Recognition rules, schedules, deferred revenue/expense, recognition runs, and posting controls.
- Leases, payment schedules, right-of-use assets, liabilities, modifications, remeasurements, and generated journal entries.

Visible value:

- RERP can support recurring revenue, deferrals, and lease accounting workflows expected by mature finance teams.

### Slice 9: Consolidation And Audit Controls

Goal: approach enterprise finance maturity once company-level accounting is solid.

Scaffold and implement:

1. `consolidation`
2. `audit-controls`
3. `general-ledger`
4. `financial-reports`
5. `tax-compliance`

Deliver:

- Consolidation groups, consolidation runs, elimination rules, elimination entries, group reporting packs, and intercompany matching.
- Approval policies, segregation rules, signature requests, immutable audit events, control exceptions, and evidence packs.
- Close-management entry points should be introduced here or immediately after this slice.

Visible value:

- Multi-company groups can produce consolidated reporting with control evidence.
- This is the first slice that starts competing with enterprise ERP finance expectations.

## Service Delivery Principles

- Start with existing runtime services before contract-only services unless the slice explicitly needs a new service owner.
- Every slice must keep `openapi_bff.yaml` regenerated and operation coverage intact.
- Add BDD scenarios before handler implementation.
- Prefer typed reusable schemas over anonymous request or response bodies.
- Do not implement rules engines before their dossier, BDD slice, OpenAPI contract, and audit/reversal behavior are clear.
- Treat localization as an extension factory, not hardcoded country behavior in core services.
