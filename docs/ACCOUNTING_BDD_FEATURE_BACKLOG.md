# RERP Accounting BDD Feature Backlog

Date: 2026-04-25

Purpose: capture broad behavior-driven feature themes for the RERP accounting maturity target. This document is intentionally high-level. It should be exploded later into granular `.feature` files, OpenAPI contract changes, generated clients, implementation tests, and workflow acceptance suites.

Related docs:

- [`OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](./OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md) — maturity target and Odoo benchmark narrative.
- [`OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](./OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md) — service-by-service OpenAPI resource backlog.
- [`docs/llmwiki/topics/accounting-openapi-odoo-gap.md`](./llmwiki/topics/accounting-openapi-odoo-gap.md) — agent-facing summary.

## Conventions

This backlog uses broad BDD-style feature statements rather than full Gherkin files.

Each feature includes:

- **Target services**: where the future OpenAPI contract likely belongs.
- **Benchmark anchors**: Odoo Enterprise modules, models, wizards, or tests that demonstrate comparable maturity.
- **Current RERP coverage**: what the existing OpenAPI specs already expose.
- **Broad scenarios**: Given/When/Then acceptance themes for later decomposition.
- **OpenAPI gaps**: resources or contract concepts that must exist before implementation can be tested end to end.

Priority levels:

- **P0**: Preserve generated BFF coverage and settle generated artifact policy.
- **P1**: Core accounting engines needed for a credible operational accounting product.
- **P2**: Enterprise breadth needed by accounting teams in production.
- **P3**: Localization, statutory, and jurisdiction-specific breadth.

## Feature Traceability Summary

| Priority | Feature group | Primary services | Main maturity gap |
|---|---|---|---|
| P0 | Generated accounting suite contract | `bff`, all accounting services | Keep service specs, generated BFF, and behavior backlog aligned |
| P1 | GL controls and close | `general-ledger`, `financial-reports` | Lock dates, audit, close/reopen, statutory handoff |
| P1 | Reconciliation engine | `bank-sync`, `general-ledger`, `invoice`, `accounts-receivable`, `accounts-payable` | Rules, suggestions, partials, write-offs, exchange differences, unreconcile |
| P1 | Report engine | `financial-reports`, `general-ledger`, `accounts-receivable`, `accounts-payable` | Definitions, expressions, drill-down, exports, schedules |
| P1 | AR follow-up and collections | `accounts-receivable`, `financial-reports`, `invoice` | Policies, dunning levels, statements, collection cases |
| P1 | AP payment controls | `accounts-payable`, `bank-sync`, `edi`, `invoice` | Payment batches, export files, vendor payment workflow, 3-way match |
| P2 | Documents and extraction | `documents-extraction`, `invoice`, `accounts-payable`, `accounts-receivable`, `edi`, `bank-sync` | OCR, review, approval, linkage to accounting records |
| P2 | Assets and budgets | `asset`, `budget`, `general-ledger`, `financial-reports` | Lifecycle, generated journal entries, analytic dimensions, revisions |
| P2 | Treasury and cash planning | `treasury`, `bank-sync`, `accounts-payable`, `accounts-receivable` | Liquidity planning, cash forecasts, bank relationships |
| P2 | Consolidation and intercompany | `consolidation`, `general-ledger`, `financial-reports` | Group consolidation, eliminations, intercompany matching |
| P2 | Revenue and lease accounting | `revenue-recognition`, `lease-accounting`, `invoice`, `asset`, `general-ledger` | Deferrals, recognition schedules, lease liabilities |
| P3 | Statutory, EDI, and localization | `tax-compliance`, `edi`, `financial-reports`, `general-ledger`, `accounts-payable` | Returns, filings, country profiles, electronic submissions |

## Service Boundary Summary

Use this split before creating or expanding OpenAPI specs:

- **Enhance existing services** for the already-started engines: GL controls in `general-ledger`, reconciliation in `bank-sync`, report engine in `financial-reports`, AR follow-up in `accounts-receivable`, AP payment controls in `accounts-payable`, asset lifecycle in `asset`, budget lifecycle in `budget`, EDI lifecycle in `edi`, and shared invoice workflow in `invoice`.
- **Add new microservices** when the capability has its own lifecycle and data ownership: `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, and later `audit-controls` if controls become cross-service.
- **Use extension/localization services** for jurisdiction-specific tax, EDI, statutory, and payment-file rules. These should extend core services rather than hardcode country behavior into `general-ledger`, `accounts-payable`, or `edi`.

## P0 — Generated Accounting Suite Contract

**Capability statement:** The generated accounting BFF must remain the reliable public entry point for every selected service-level OpenAPI operation.

Target services:

- `bff`
- All services listed in `openapi/accounting/bff-suite-config.yaml`

Benchmark anchors:

- RERP's own suite-aware BFF generation contract.
- BRRTRouter BFF merge behavior: public paths are prefixed by each service `base_path`.

Current RERP coverage:

- `openapi/accounting/openapi_bff.yaml` now exposes 203 namespaced paths and 320 operations from the selected service specs.
- AP and AR can both own local `/payments` because the BFF publishes `/api/accounts-payable/payments` and `/api/accounts-receivable/payments`.
- New accounting contracts for `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, and `audit-controls` are included in BFF generation, but remain runtime contract-only until service scaffolding is added.

Broad scenarios:

```gherkin
Feature: Generated accounting BFF coverage

  Scenario: Service operations are visible through the suite BFF
    Given an accounting service OpenAPI document is selected by bff-suite-config.yaml
    When the accounting BFF is generated
    Then every selected service operationId appears in the generated BFF
    And the public BFF paths are namespaced by the configured service base_path

  Scenario: Local service paths can overlap safely
    Given accounts payable and accounts receivable both expose a local /payments path
    When the accounting BFF is generated
    Then each payment path is published under its owning service namespace
    And no generated operation is dropped due to a local path collision
```

OpenAPI gaps:

- Decide whether `openapi/accounting/openapi.yaml` is retained, regenerated from the same source, or explicitly deprecated.
- Preserve operation coverage checks as new maturity-target endpoints are added.

## Implementation Slice 1 — Contract-First Reconciliation And Reports

This slice is the first buildable unit before rules-engine implementation. It turns the broad P1 backlog into concrete acceptance tests that can drive handlers, persistence, and generated clients without deciding the full rules-engine language yet.

Scope:

- `bank-sync`: reconciliation models, ranked suggestions, reconcile/unreconcile, write-offs, and exchange differences.
- `financial-reports`: report definitions, report lines, report-cell drill-down, report exports, and statutory report packs.
- Supporting source records from `general-ledger`, `invoice`, `accounts-receivable`, and `accounts-payable` are treated as existing fixtures or API dependencies, not new engine work in this slice.

Out of scope for this slice:

- Full reconciliation rule DSL or expression runtime.
- Bank-provider consent/import lifecycle.
- Report expression compiler, formula sandboxing, and scheduler execution.
- Localization/statutory filing adapters.

### Slice 1A — Reconciliation Suggestions

```gherkin
Feature: Reconciliation suggestions

  Scenario: Exact-reference model ranks an invoice match
    Given a bank transaction has reference "INV-1001" and amount 1200.00
    And an open customer invoice has number "INV-1001" and residual amount 1200.00
    And an active reconciliation model matches exact reference and amount
    When suggestions are requested for the bank transaction
    Then the response contains the invoice as the highest-ranked suggestion
    And the suggestion includes the model id, confidence, candidate type, and reason
    And no reconciliation or journal entry is created

  Scenario: Suggestions explain partial and write-off candidates
    Given a bank transaction amount differs from an open invoice residual by a configured tolerance
    And the active reconciliation model allows write-off suggestions
    When suggestions are requested for the bank transaction
    Then the response includes a write-off candidate with the expected difference
    And the response identifies the account or model that would be used if accepted
```

Acceptance contract:

- `GET /transactions/{id}/suggestions` returns `ReconciliationSuggestions`.
- Suggestions must be read-only and must not mutate transaction, invoice, payment, or journal-entry state.
- Suggestion rows must include candidate id, candidate type, confidence, optional model id, and reason.

### Slice 1B — Reconcile, Write Off, Exchange Difference, And Unreconcile

```gherkin
Feature: Bank transaction reconciliation actions

  Scenario: Accountant reconciles a transaction to a source record
    Given a bank transaction is not reconciled
    And a suggested invoice match exists
    When the accountant reconciles the transaction to the invoice
    Then the transaction status changes to reconciled
    And the response identifies the reconciliation id and resulting status
    And an audit event or journal reference can be stored for later drill-down

  Scenario: Accountant records a write-off during reconciliation
    Given the remaining transaction difference is below policy tolerance
    When the accountant creates a write-off for the transaction
    Then the response returns a reconciliation adjustment
    And the adjustment records the transaction id, adjustment type, amount, and journal entry id if posted

  Scenario: Accountant reverses an incorrect reconciliation
    Given a transaction has been reconciled
    When the accountant unreconciles the transaction with a reason
    Then the transaction is returned to an unreconciled state
    And the previous reconciliation remains available for audit
```

Acceptance contract:

- `POST /transactions/{id}/reconcile` accepts `ReconcileTransactionRequest` and returns `ReconciliationResult`.
- `POST /transactions/{id}/write-off` accepts `CreateTransactionWriteOffRequest` and returns `ReconciliationAdjustment`.
- `POST /transactions/{id}/exchange-difference` accepts `CreateExchangeDifferenceRequest` and returns `ReconciliationAdjustment`.
- `POST /transactions/{id}/unreconcile` must be idempotent for already-unreconciled records or return a clear conflict.

### Slice 1C — Report Definitions And Drill-Down

```gherkin
Feature: Configurable report definitions

  Scenario: Accountant defines a reusable financial report
    Given the accountant has a company and reporting currency
    When a report definition is created with report type, name, and active flag
    Then the definition can be listed and retrieved through the report service
    And the generated BFF exposes the typed create request and response schemas

  Scenario: Report line drill-down returns source accounting lines
    Given a report execution has produced a cell balance
    When the accountant drills into the report cell
    Then the response lists source records with source id, source type, amount, and description
    And the response can be traced back to journal entries or ledger lines
```

Acceptance contract:

- `POST /report-definitions` accepts `CreateReportDefinitionRequest` and returns `ReportDefinition`.
- `GET /report-definitions/{id}/lines` returns `PaginatedReportDefinitionLines`.
- `GET /report-cells/{id}/drill-down` returns `ReportCellDrillDown`.
- `POST /report-exports` accepts `CreateReportExportRequest` and returns `ReportExport`.

### Slice 1D — Generated BFF Acceptance

```gherkin
Feature: Generated BFF preserves first-slice contracts

  Scenario: First-slice typed schemas appear in the generated BFF
    Given the accounting BFF is generated from bff-suite-config.yaml
    When the generated OpenAPI document is inspected
    Then the public paths include reconciliation and report definition operations
    And component schemas are namespaced by source service
    And the generated BFF still exposes 203 paths and 320 operations
```

Acceptance contract:

- Generated BFF contains service-prefixed components such as `BankSyncCreateReconciliationModelRequest`, `AccountsPayableCreatePaymentBatchRequest`, and `FinancialReportsCreateReportDefinitionRequest`.
- Regeneration must not reintroduce anonymous `type: object` request bodies for the first-slice endpoints.

## P1 — GL Controls And Close

**Capability statement:** Accountants can control accounting periods, journals, postings, reversals, and close processes with auditable state transitions.

Target services:

- `general-ledger`
- `financial-reports`

Benchmark anchors:

- `account_accountant/wizard/account_change_lock_date.py`
- `account_accountant/tests/test_change_lock_date_wizard.py`
- `account_reports/models/account_return.py`

Current RERP coverage:

- Accounts, chart of accounts, journals, journal entries, fiscal periods, fiscal years, chart templates, payment terms, payment methods, fiscal positions, and tax repartition lines.
- Journal-entry workflows: approve, post, reverse, bulk approve, bulk post.
- Fiscal-year close/reopen and period generation.

Broad scenarios:

```gherkin
Feature: Accounting lock controls and close

  Scenario: Lock dates protect posted accounting data
    Given a fiscal period contains posted journal entries
    When an accountant requests a lock-date change
    Then the system records the actor, reason, old lock date, and new lock date
    And blocked posting attempts return a clear accounting control error

  Scenario: Fiscal close produces auditable accounting evidence
    Given all required periods in a fiscal year are ready to close
    When the fiscal year is closed
    Then closing activity is recorded as a stateful workflow
    And reports can drill from closing balances to source journal items
```

OpenAPI gaps:

- `/lock-dates`
- `/lock-dates/{id}/change-request`
- `/journal-items`
- `/journal-items/reconcile`
- `/journal-items/{id}/unreconcile`
- `/statutory-returns`
- `/statutory-returns/{id}/validate`
- `/statutory-returns/{id}/submit`

## P1 — Reconciliation Engine

**Capability statement:** Accountants can configure matching rules, review suggestions, reconcile source records, and reverse or edit reconciliations with full auditability.

Target services:

- `bank-sync`
- `general-ledger`
- `invoice`
- `accounts-receivable`
- `accounts-payable`

Benchmark anchors:

- `account_accountant/models/account_bank_statement.py`
- `account_accountant/wizard/account_reconcile_wizard.py`
- `account_accountant/tests/test_reconciliation_matching_rules.py`
- `account_accountant/tests/test_account_reconcile_wizard.py`

Current RERP coverage:

- Bank accounts, statements, transactions, `match_transaction`, `auto_match_transactions`, reconciliations, `complete_reconciliation`, reconciliation report, and cash position report.
- AR payment applications and auto-apply.
- Invoice workflow and GL journal entries exist as source records.

Broad scenarios:

```gherkin
Feature: Reconciliation rules and suggestions

  Scenario: Matching rules produce candidate suggestions
    Given imported bank transactions and open invoices exist
    And reconciliation models define memo, amount, partner, date, and account predicates
    When an accountant requests suggestions for a transaction
    Then the system returns ranked candidate matches with reasons
    And no journal entry is posted until a suggestion is accepted

  Scenario: Accountant reconciles with a write-off and exchange difference
    Given a transaction partially matches an open invoice in a different currency
    When the accountant reconciles the transaction with a write-off and exchange-difference line
    Then the system creates the required accounting lines
    And the reconciliation can be audited, edited, or unreconciled
```

OpenAPI gaps:

- `/reconciliation-models`
- `/reconciliation-models/{id}/rules`
- `/transactions/{id}/suggestions`
- `/transactions/{id}/reconcile`
- `/transactions/{id}/unreconcile`
- `/transactions/{id}/edit-reconciliation`
- `/transactions/{id}/write-off`
- `/transactions/{id}/transfer`
- `/transactions/{id}/exchange-difference`
- `/statement-imports`
- `/statement-import-formats`
- `/bank-connections`
- `/bank-connections/{id}/consent`

## P1 — Report Engine

**Capability statement:** Financial reports are configurable, interactive, auditable, exportable accounting artifacts rather than one-off report endpoints.

Target services:

- `financial-reports`
- `general-ledger`
- `accounts-receivable`
- `accounts-payable`

Benchmark anchors:

- `account_reports/models/account_report.py`
- `account_reports/models/account_partner_ledger.py`
- `account_reports/models/account_generic_tax_report.py`
- `account_reports/wizard/account_report_send.py`
- `account_reports/tests/test_all_reports_generation.py`

Current RERP coverage:

- Balance sheet, income statement, cash flow, general ledger, trial balance.
- Custom report CRUD and report execution listing.
- OpenAPI hygiene issue: core report generators currently use `GET` request bodies.

Broad scenarios:

```gherkin
Feature: Configurable financial report engine

  Scenario: Accountant runs a report definition with options
    Given a report definition has lines, columns, expressions, and filters
    When an accountant executes the report for a company and period
    Then the report execution records the options and source data window
    And the result can be unfolded into lines and source journal items

  Scenario: Accountant exports and sends a report package
    Given a report execution has completed
    When the accountant exports the report to PDF and XLSX
    Then each export is stored as an immutable artifact
    And scheduled delivery can send the same report package to configured recipients
```

OpenAPI gaps:

- `/report-definitions`
- `/report-definitions/{id}/lines`
- `/report-definitions/{id}/expressions`
- `/reports/{id}/options`
- `/reports/{id}/lines`
- `/reports/{id}/unfold`
- `/reports/{id}/audit-cell`
- `/reports/{id}/source-lines`
- `/report-executions/{id}/export/pdf`
- `/report-executions/{id}/export/xlsx`
- `/report-schedules`
- `/reports/partner-ledger`
- `/reports/aged-receivable`
- `/reports/aged-payable`
- `/reports/customer-statement`
- `/reports/tax`
- `/reports/multicurrency-revaluation`

## P1 — AR Follow-Up And Collections

**Capability statement:** Receivables follow-up is policy-driven, auditable, multi-channel, and connected to statements, partner ledgers, disputes, and collection cases.

Target services:

- `accounts-receivable`
- `financial-reports`
- `invoice`

Benchmark anchors:

- `account_followup/models/account_followup.py`
- `account_followup/wizard/followup_manual_reminder.py`
- `account_reports/models/account_customer_statement.py`
- `account_reports/models/account_partner_ledger.py`

Current RERP coverage:

- Customer invoices, payments, payment applications, auto-apply, credit memos, collection activities, AR aging, aging summary, and collections summary.

Broad scenarios:

```gherkin
Feature: Receivables follow-up policies

  Scenario: Follow-up policy selects overdue customers
    Given customers have overdue invoices in different aging buckets
    And follow-up levels define due-day thresholds and channels
    When a follow-up run is executed
    Then the system selects customers according to policy
    And records the reminder channel, statement, activity, and follow-up level used

  Scenario: Collector manages a disputed account
    Given a customer disputes an overdue invoice
    When a collection case is opened with a dispute hold
    Then automated reminders are paused for that invoice
    And the case records promise-to-pay, resolution, or write-off outcomes
```

OpenAPI gaps:

- `/follow-up-levels`
- `/follow-up-policies`
- `/follow-up-runs`
- `/follow-up-runs/{id}/execute`
- `/customers/{id}/statement`
- `/customers/{id}/statement/send`
- `/reports/partner-ledger`
- `/partners/{id}/ledger-lines`
- `/collection-cases`
- `/collection-cases/{id}/promise-to-pay`
- `/dispute-holds`

## P1 — AP Payment Controls

**Capability statement:** Payables can move from approved invoice to controlled payment file and reconciliation through governed, auditable workflows.

Target services:

- `accounts-payable`
- `bank-sync`
- `invoice`
- `edi`

Benchmark anchors:

- `account_batch_payment/models/account_batch_payment.py`
- `account_iso20022/models/account_batch_payment.py`
- `account_sepa_direct_debit/models/sdd_mandate.py`
- `account_3way_match`
- `l10n_us_1099`

Current RERP coverage:

- Vendor invoices, payments, approvals, payment commitments, and cash-flow forecast.

Broad scenarios:

```gherkin
Feature: Vendor payment batches and files

  Scenario: Approved vendor invoices are selected for a payment batch
    Given approved vendor invoices are due for payment
    When an AP user creates a payment batch
    Then the system validates bank account, currency, payment method, and approval constraints
    And the batch records selected invoices, totals, and payment rail metadata

  Scenario: Payment file is generated and reconciled
    Given a payment batch has passed validation
    When the AP user exports a bank payment file
    Then the file becomes an immutable generated artifact
    And later bank reconciliation links returned transactions to the original batch
```

OpenAPI gaps:

- `/payment-batches`
- `/payment-batches/{id}/send`
- `/payment-batches/{id}/export-file`
- `/payment-batches/{id}/reconcile`
- `/vendor-invoices/{id}/register-payment`
- `/vendor-invoices/{id}/three-way-match`
- `/vendor-bill-documents/{id}/extract`
- `/payment-files`
- `/payment-files/iso20022`
- `/payment-files/sepa-credit-transfer`
- `/direct-debit-mandates`
- `/supplier-tax-reports/1099`

## P2 — Documents And Extraction

**Capability statement:** Accounting documents are first-class inputs with extraction, review, approval, linkage, and audit trails.

Target services:

- `documents-extraction`
- `invoice`
- `accounts-payable`
- `accounts-receivable`
- `bank-sync`
- `edi`

Benchmark anchors:

- `account_invoice_extract`
- `account_bank_statement_extract`
- `documents_account/models/documents_document.py`
- `ai_documents_account`

Current RERP coverage:

- Invoice, AP, AR, bank-sync, and EDI specs reference the accounting records that documents should create or enrich.
- No first-class document ingestion or extraction lifecycle is currently exposed.

Broad scenarios:

```gherkin
Feature: Accounting document extraction

  Scenario: Vendor bill is extracted from an uploaded document
    Given an AP document has been uploaded
    When extraction completes
    Then extracted supplier, dates, taxes, totals, and line items are available for review
    And approval creates or updates the linked vendor invoice

  Scenario: Embedded e-invoice document is linked to the invoice lifecycle
    Given an electronic invoice contains structured XML and an embedded PDF
    When the document is parsed
    Then structured fields are mapped to accounting entities
    And the original source document remains attached as audit evidence
```

OpenAPI gaps:

- `/accounting-documents`
- `/accounting-documents/{id}/classify`
- `/extraction-jobs`
- `/extraction-results`
- `/invoice-documents`
- `/invoice-documents/{id}/extract`
- `/invoice-documents/{id}/approve-extraction`
- `/vendor-bill-documents/{id}/extract`
- `/documents/{id}/link-invoice`
- `/documents/{id}/link-bank-statement`
- `/invoices/{id}/attachments`
- `/invoices/{id}/embedded-documents`

## P2 — Assets And Budgets

**Capability statement:** Assets and budgets support lifecycle workflows with generated accounting effects, analytic dimensions, revisions, and reporting.

Target services:

- `asset`
- `budget`
- `general-ledger`
- `financial-reports`

Benchmark anchors:

- `account_asset/models/account_asset.py`
- `account_asset/wizard/asset_modify.py`
- `account_budget/models/budget_analytic.py`
- `account_budget/wizards/budget_split_wizard_view.xml`
- `account_reports/tests/test_budget.py`

Current RERP coverage:

- Assets, depreciation entries, single and bulk depreciation, revaluations, disposals, categories, asset summary.
- Budgets, budget lines, submit workflow, variance report, forecasts.

Broad scenarios:

```gherkin
Feature: Asset lifecycle accounting

  Scenario: Asset lifecycle event posts accounting entries
    Given an asset is active and has a depreciation schedule
    When depreciation, revaluation, disposal, or modification is posted
    Then the system creates auditable journal entries
    And links the accounting effect back to the asset lifecycle event

Feature: Budget revisions and analytics

  Scenario: Budget revision is compared with actuals
    Given a budget has approved revisions and analytic dimensions
    When an accountant runs budget analysis
    Then the report compares actuals against the selected budget version
    And variance lines can be grouped by analytic dimension
```

OpenAPI gaps:

- `/asset-models`
- `/asset-groups`
- `/assets/{id}/validate`
- `/assets/{id}/pause`
- `/assets/{id}/resume`
- `/assets/{id}/modify`
- `/assets/{id}/sell-dispose`
- `/assets/{id}/journal-entries`
- `/assets/{id}/analytic-distribution`
- `/budgets/{id}/confirm`
- `/budgets/{id}/cancel`
- `/budgets/{id}/complete`
- `/budgets/{id}/revisions`
- `/budgets/{id}/create-revision`
- `/analytic-dimensions`
- `/budgets/{id}/analytic-lines`
- `/reports/budget-analysis`

## P2 — Treasury And Cash Planning

**Capability statement:** Finance teams can plan liquidity, monitor cash exposure, and manage bank relationships beyond transactional bank synchronization.

Target services:

- `treasury`
- `bank-sync`
- `accounts-payable`
- `accounts-receivable`

Benchmark anchors:

- `account_online_synchronization`
- `account_reports` cash-flow and bank reconciliation reports
- Payment batch and bank statement workflows across Odoo Enterprise accounting modules

Current RERP coverage:

- `bank-sync` exposes bank accounts, statements, transactions, cash position, and reconciliation reports.
- `accounts-payable` exposes cash-flow forecast.

Broad scenarios:

```gherkin
Feature: Treasury cash planning

  Scenario: Treasurer reviews projected liquidity
    Given bank balances, receivables, payables, and forecast assumptions exist
    When a treasurer generates a liquidity plan
    Then the plan shows projected inflows, outflows, gaps, and recommended transfer windows
    And each projection can be traced to source transactions or assumptions
```

OpenAPI gaps:

- `/cash-positions`
- `/cash-forecasts`
- `/liquidity-plans`
- `/bank-relationships`
- `/cash-transfers`

## P2 — Consolidation And Intercompany

**Capability statement:** Group accounting can consolidate subsidiaries, eliminate intercompany balances, and produce auditable group reporting packs.

Target services:

- `consolidation`
- `general-ledger`
- `financial-reports`

Benchmark anchors:

- `account_inter_company_rules`
- `account_reports` group/report engine patterns

Current RERP coverage:

- `general-ledger` has company-aware resources and journal entries.
- There is no dedicated consolidation or elimination owner yet.

Broad scenarios:

```gherkin
Feature: Group consolidation

  Scenario: Consolidation run creates elimination entries
    Given subsidiaries have submitted period balances
    And elimination rules define intercompany relationships
    When a consolidation run is executed
    Then group balances include generated elimination entries
    And the reporting pack links each elimination to source company balances
```

OpenAPI gaps:

- `/consolidation-groups`
- `/consolidation-runs`
- `/elimination-rules`
- `/elimination-entries`
- `/group-reporting-packs`

## P2 — Revenue And Lease Accounting

**Capability statement:** Revenue deferrals, expense deferrals, and leases are managed as accounting schedules with generated journal entries and audit evidence.

Target services:

- `revenue-recognition`
- `lease-accounting`
- `invoice`
- `asset`
- `general-ledger`

Benchmark anchors:

- Odoo deferred revenue/expense report patterns in `account_reports`
- Odoo asset lifecycle patterns in `account_asset`

Current RERP coverage:

- `invoice` exposes invoice workflows and line items.
- `asset` exposes depreciation and disposals.
- There is no dedicated recognition or lease accounting owner yet.

Broad scenarios:

```gherkin
Feature: Revenue recognition schedules

  Scenario: Posted invoice creates a recognition schedule
    Given a posted invoice line is configured for deferred recognition
    When the recognition schedule is generated
    Then the system records planned recognition periods and amounts
    And each recognition run posts auditable journal entries

Feature: Lease accounting schedules

  Scenario: Lease modification recalculates accounting schedules
    Given an active lease has payment and liability schedules
    When the lease terms are modified
    Then the right-of-use asset and lease liability schedules are recalculated
    And adjustment entries are linked to the modification event
```

OpenAPI gaps:

- `/recognition-rules`
- `/recognition-schedules`
- `/deferred-revenues`
- `/deferred-expenses`
- `/recognition-runs`
- `/leases`
- `/lease-payment-schedules`
- `/lease-liabilities`
- `/right-of-use-assets`
- `/lease-modifications`

## P3 — Statutory, EDI, And Localization

**Capability statement:** Localization is modeled as extensible statutory, EDI, tax, and payment-file profiles rather than hardcoded country behavior in core services.

Target services:

- `tax-compliance`
- `edi`
- `financial-reports`
- `general-ledger`
- `accounts-payable`
- `bank-sync`

Benchmark anchors:

- `account_reports/models/account_return.py`
- `account_intrastat`
- `account_saft`
- `l10n_uk_hmrc`
- `l10n_us_1099`
- `l10n_*_edi`
- `documents_account_peppol`

Current RERP coverage:

- Fiscal positions and tax repartition exist in GL.
- EDI currently exposes document and mapping CRUD only.

Broad scenarios:

```gherkin
Feature: Statutory return lifecycle

  Scenario: Accountant validates and submits a statutory return
    Given a statutory return is generated from report definitions and source accounting lines
    When validation checks pass
    Then the return can be submitted, paid, archived, or reset according to jurisdiction workflow
    And working files and audit balances are retained

Feature: EDI submission lifecycle

  Scenario: EDI submission is accepted or rejected by an authority
    Given an EDI profile defines the document standard and validation rules
    When a document is submitted
    Then the system records submission status, acknowledgments, validation errors, retries, and attachments
```

OpenAPI gaps:

- `/tax-periods`
- `/tax-rules`
- `/tax-returns`
- `/tax-returns/{id}/validate`
- `/tax-returns/{id}/submit`
- `/tax-payments`
- `/tax-audit-packs`
- `/edi-profiles`
- `/edi-validation-profiles`
- `/edi-submissions`
- `/edi-submissions/{id}/submit`
- `/edi-submissions/{id}/status`
- `/edi-submissions/{id}/retry`
- `/edi-acknowledgments`
- `/edi-errors`
- `/statutory-returns`
- `/statutory-returns/{id}/working-files`
- `/intrastat/reports`
- `/saft/reports`
- `/hmrc/obligations`
- `/1099/reports`
- `/invoice-documents/ubl`
- `/invoice-documents/peppol`

## OpenAPI Hygiene Features

These are not business workflows, but they protect future implementation quality.

```gherkin
Feature: Accounting OpenAPI contract hygiene

  Scenario: Generated clients can consume report contracts consistently
    Given report generation requires complex filter options
    When the OpenAPI contract is normalized
    Then report executions use POST request bodies or explicit query parameters
    And generated clients do not rely on GET request bodies

  Scenario: Every accounting service exposes consistent metadata
    Given a service participates in the accounting suite
    When its OpenAPI document is linted
    Then it includes tags, contact, security, standard error responses, pagination conventions, and correct port metadata
```

Current hygiene gaps:

- `financial-reports` uses `GET` operations with request bodies for report generation.
- `edi` has empty tags, no contact, and weaker security/error coverage than the mature accounting service specs.
- `edi` and `bank-sync` server metadata both reference localhost port `8008`, which should be reconciled against the port registry.
- `general-ledger` has a `reconciliations` tag without matching paths.
- `accounts-payable` has a `vendors` tag without vendor resource paths.
- `budget` has an `approvals` tag but only a submit operation.

## Explosion Strategy

When this backlog is decomposed into granular BDD features:

1. Start with one P1 engine at a time, beginning with reconciliation or report engine.
2. For each broad scenario, create granular success, validation failure, idempotency, authorization, audit, and reversal scenarios.
3. Update the service OpenAPI spec with the missing resources and schemas.
4. Regenerate `openapi/accounting/openapi_bff.yaml`.
5. Verify generated BFF operation coverage.
6. Add implementation tests for state transitions and accounting invariants before handler code.
