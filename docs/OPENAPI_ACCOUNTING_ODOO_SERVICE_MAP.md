# RERP Accounting vs Odoo Enterprise Service Map

Date: 2026-04-25

Purpose: convert the broad Odoo comparison into concrete RERP OpenAPI backlog targets. This document maps each RERP accounting service to Odoo Enterprise modules, model/wizard/report anchors, and suggested API resources.

Source scope:

- RERP service specs: `openapi/accounting/*/openapi.yaml`
- Odoo Enterprise addons: `/Users/casibbald/Workspace/remote/caffeinated.expert/odooforks/enterprise`

Related analysis: [`OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](./OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md).

Behavior backlog: [`ACCOUNTING_BDD_FEATURE_BACKLOG.md`](./ACCOUNTING_BDD_FEATURE_BACKLOG.md). This service map owns planned OpenAPI resources; the BDD backlog owns broad feature behavior and later scenario decomposition.

## How To Read This

RERP is OpenAPI-first and service-oriented. Odoo is model/module-oriented. Do not copy Odoo's module structure directly. Use Odoo as a mature workflow benchmark and translate each capability into a RERP service boundary.

Priority codes:

- **P0**: preserve accounting BFF generator coverage and settle the legacy top-level aggregate policy.
- **P1**: accounting workflows that unlock high-value operational use.
- **P2**: enterprise breadth needed for production accounting departments.
- **P3**: jurisdiction-specific/localization breadth.

## P0 Status: BFF Generator Coverage Implemented

The generator path is now the source of truth for the suite BFF:

- Service specs expose about 111 path templates across the 9 accounting services.
- `openapi/accounting/openapi_bff.yaml` now exposes those selected service operations as 111 namespaced BFF path templates and 185 operations.
- The BFF generator prefixes public paths with each service `base_path`, so local service paths such as AP/AR `/payments` do not collide.
- The RERP wrapper validates generated operationId coverage against selected service specs.
- `openapi/accounting/openapi.yaml` still needs an explicit policy: regenerate from the same source, keep as a legacy artifact, or deprecate.
- Stale generated names such as `journal-entrys` should continue to be fixed in the generation path, not hand-patched into generated output.

Backlog:

| Priority | Target | Work |
|---|---|---|
| P0 | BFF generator | Preserve `rerp bff generate-system --suite accounting` as the source of truth for `openapi/accounting/openapi_bff.yaml`. |
| P0 | Generator validation | Keep validating that every selected service operationId appears in the generated BFF output after future service spec expansion. |
| P0 | Top-level spec | Decide whether `openapi/accounting/openapi.yaml` is a legacy aggregate. If retained, generate it from the same source as the BFF. |
| P0 | Naming | Fix stale generated names like `journal-entrys` to `journal-entries` in the generator/config mapping. |

## Service Coverage Matrix

| RERP service | Current RERP coverage | Odoo Enterprise benchmark anchors | Main gap |
|---|---|---|---|
| `general-ledger` | Accounts, chart, journals, journal entries, fiscal periods/years, fiscal positions, payment terms/methods, tax repartition, GL reports | `account_accountant`, `account_reports`, `account_inter_company_rules`, `account_reports/models/account_return.py` | Lock dates, audit cells, statutory returns, inter-company, multi-currency revaluation |
| `invoice` | Invoice CRUD, lines, tax calculation, approve/post/cancel/void, workflow history | `account_accountant/models/account_move.py`, `account_invoice_extract`, `documents_account` | Payment-register semantics, refunds/credit notes, extraction, e-invoicing, deferred revenue/expense |
| `accounts-receivable` | Customer invoices, payments, applications, auto-apply, credit memos, collections, AR aging | `account_followup`, `account_reports/models/account_partner_ledger.py`, `account_reports/models/account_customer_statement.py` | Dunning policies, follow-up channels, partner ledger, statements, dispute/promise-to-pay |
| `accounts-payable` | Vendor invoices, payments, approvals, commitments, cash-flow forecast | `account_batch_payment`, `account_iso20022`, `account_invoice_extract`, `l10n_us_1099` | Payment files, batch lifecycle, vendor bill extraction, 3-way match, statutory supplier reporting |
| `bank-sync` | Bank accounts, statements, transactions, trigger sync, match/auto-match, reconciliations, cash position | `account_accountant/models/account_bank_statement.py`, `account_accountant/wizard/account_reconcile_wizard.py`, `account_online_synchronization`, bank statement import addons | Reconciliation models/rules, suggestions, partials, write-offs, exchange diff, provider consent/import lifecycle |
| `asset` | Assets, depreciation entries, run/bulk depreciation, revaluation, disposal, categories, summary | `account_asset/models/account_asset.py`, `account_asset/wizard/asset_modify.py`, `account_asset/models/account_assets_report.py` | Asset models/groups, pause/resume/modify/sell/dispose lifecycle, generated journal entries, analytic integration |
| `budget` | Budgets, lines, submit, variance, forecasts | `account_budget/models/budget_analytic.py`, `account_budget/models/budget_line.py` | Analytic dimensions, budget revisions, confirm/done/cancel lifecycle, split wizard, pivot/report API |
| `financial-reports` | Balance sheet, income statement, cash flow, GL, trial balance, custom reports/executions | `account_reports/models/account_report.py`, report handlers, return models, report send wizard | Report engine, report lines/expressions/options, audit drill-down, exports, scheduled sends, tax returns |
| `edi` | EDI document/mapping CRUD | `documents_account`, `account_saft`, `account_intrastat`, `account_iso20022`, `account_sepa_direct_debit`, `l10n_*_edi` | Standards, submissions, polling, acknowledgments, validation profiles, embedded documents |

## Service Boundary Decisions

Use these decisions before adding new OpenAPI files. The goal is to avoid creating one microservice per feature while still splitting domains that have independent lifecycle, data ownership, scaling, or localization needs.

### Enhance Existing Accounting Services

These gaps should become richer OpenAPI contracts inside services already started:

| Existing service | Expand with | Why this belongs here |
|---|---|---|
| `general-ledger` | Lock dates, close controls, journal-item reconciliation, posting audit, core multi-currency revaluation hooks | These are accounting control and source-ledger concerns. |
| `bank-sync` | Reconciliation rules, suggestions, unreconcile/edit reconciliation, write-offs, transfers, exchange differences, bank provider/import lifecycle | The source records are bank accounts, statements, transactions, and reconciliation records. |
| `financial-reports` | Report definitions, lines, expressions, options, audit cells, source-line drill-down, exports, schedules | Reporting needs one engine over GL, AR, AP, tax, budget, and statutory outputs. |
| `accounts-receivable` | Follow-up levels, policies, statement sending, collection cases, disputes, promise-to-pay, dunning holds | These workflows own customer receivable collection state. |
| `accounts-payable` | Payment batches, vendor payment registration, payment-file handoff, 3-way match entry points, supplier reporting handoff | These workflows own vendor liability and payment selection state. |
| `asset` | Asset models/groups, validate/pause/resume/modify/close/cancel, generated journal-entry links, analytic distribution | These are fixed-asset lifecycle concerns. |
| `budget` | Confirm/cancel/complete/reopen, revisions, analytic dimensions, split workflow, budget-analysis reports | These are planning and budget governance concerns. |
| `edi` | Profiles, validation profiles, submissions, acknowledgments, retries, errors, embedded-document standards | The existing EDI service should evolve from CRUD into the message lifecycle owner. |
| `invoice` | Register-payment entry point, residuals, payment matches, refunds/credit notes, e-invoice handoff, deferral handoff | The invoice service owns the shared invoice entity and lifecycle. |

### Add New Accounting Microservices

These gaps were large enough to justify new `openapi/accounting/{service}/openapi.yaml` files and entries in `openapi/accounting/bff-suite-config.yaml`. Initial contract surfaces now exist for each proposed service:

| Proposed service | Owns | Initial OpenAPI surface |
|---|---|---|
| `tax-compliance` | Tax periods, returns, jurisdiction tax rules, filings, payments, audit packs, statutory working files | `/tax-periods`, `/tax-rules`, `/tax-returns`, `/tax-returns/{id}/validate`, `/tax-returns/{id}/submit`, `/tax-payments`, `/tax-audit-packs` |
| `documents-extraction` | Accounting document ingestion, OCR/extraction, classification, review, approval, linkage to invoices/bills/statements | `/accounting-documents`, `/extraction-jobs`, `/extraction-results`, `/documents/{id}/approve-extraction`, `/documents/{id}/link-invoice`, `/documents/{id}/link-bank-statement` |
| `treasury` | Liquidity planning, cash positioning beyond bank sync, cash forecasts, bank relationships, funding/transfer planning | `/cash-positions`, `/cash-forecasts`, `/liquidity-plans`, `/bank-relationships`, `/cash-transfers` |
| `consolidation` | Multi-company consolidation, eliminations, group reporting packs, intercompany matching at group level | `/consolidation-groups`, `/consolidation-runs`, `/elimination-rules`, `/elimination-entries`, `/group-reporting-packs` |
| `revenue-recognition` | Deferred revenue/expense, recognition schedules, recurring revenue, invoice deferrals, recognition journal entries | `/recognition-rules`, `/recognition-schedules`, `/deferred-revenues`, `/deferred-expenses`, `/recognition-runs` |
| `lease-accounting` | Right-of-use assets, lease liabilities, payment schedules, lease modifications, ASC 842/IFRS 16 style accounting | `/leases`, `/lease-payment-schedules`, `/lease-liabilities`, `/right-of-use-assets`, `/lease-modifications` |
| `audit-controls` | Cross-service segregation of duties, approval matrices, electronic signatures, immutable audit events | `/approval-policies`, `/segregation-rules`, `/signature-requests`, `/audit-events`, `/control-exceptions` |

`audit-controls` can start as per-service audit metadata if scope stays accounting-only. Promote it to a microservice when approval policy, segregation of duties, signatures, and audit event queries must span multiple accounting services or other RERP suites.

### Implement As Extension Or Localization Services

Country-specific behavior should not be hardcoded into core accounting services. Add extension services/plugins around `tax-compliance`, `edi`, `financial-reports`, `accounts-payable`, and payment-file flows:

| Extension pattern | Examples | Extends |
|---|---|---|
| Tax/local filing packs | `l10n-uk-hmrc`, `l10n-us-1099`, `l10n-eu-oss`, `l10n-saft` | `tax-compliance`, `financial-reports`, `edi` |
| E-invoicing packs | PEPPOL, Factur-X, country e-invoicing, UBL variants | `edi`, `documents-extraction`, `invoice` |
| Payment rail packs | ISO20022, SEPA credit transfer, SEPA direct debit, NACHA, check printing | `accounts-payable`, `bank-sync`, `edi` |
| Statutory report packs | Intrastat, SAF-T, VAT/GST returns, HMRC obligations | `tax-compliance`, `financial-reports`, `edi` |

### OpenAPI Build-Out Order

1. Initial contracts have been added for the missing microservices and included in the generated accounting BFF.
2. Deepen the P1 engines next: `bank-sync`, `financial-reports`, `accounts-receivable`, `accounts-payable`, and `general-ledger`.
3. Expand `tax-compliance` before broad statutory/localization work, because returns and filings need a core owner.
4. Expand `documents-extraction` before OCR, bill extraction, and document-to-invoice workflows.
5. Add localization/payment/EDI extension services only after the core extension points are explicit in OpenAPI.

### Runtime Scaffolding Gate

The new accounting services are contract-visible but not runtime-ready until service scaffolding exists. Before implementing rules engines inside any of the new services, add the same deployment and generation surface used by the existing accounting services:

| Service | Contract port | Runtime status | Required scaffolding |
|---|---:|---|---|
| `tax-compliance` | 8016 | Contract-only | `microservices/accounting/tax-compliance/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |
| `documents-extraction` | 8017 | Contract-only | `microservices/accounting/documents-extraction/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |
| `treasury` | 8018 | Contract-only | `microservices/accounting/treasury/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |
| `consolidation` | 8019 | Contract-only | `microservices/accounting/consolidation/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |
| `revenue-recognition` | 8020 | Contract-only | `microservices/accounting/revenue-recognition/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |
| `lease-accounting` | 8021 | Contract-only | `microservices/accounting/lease-accounting/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |
| `audit-controls` | 8022 | Contract-only | `microservices/accounting/audit-controls/{gen,impl}`, Helm values, Dockerfile, generated docs, workspace registration |

Do not start by implementing business logic in these services. First confirm the `rerp`/BRRTRouter generation path can create the two-crate service shape, compile generated crates, and expose each service through the same BFF and deployment conventions as the existing accounting services.

Implementation order before rules engines:

1. Stabilize existing P1 services with BDD slices: `bank-sync` reconciliation and `financial-reports` report definitions.
2. Scaffold the seven new contract-only services without deep business logic.
3. Run generation and BFF regeneration to confirm ports, crate names, and component names stay stable.
4. Only then expand rule engines: reconciliation matching rules first, report expression rules second.

## `general-ledger` Backlog

Current RERP anchors:

- `openapi/accounting/general-ledger/openapi.yaml`
- Key operations: `list_accounts`, `create_account`, `list_journal_entries`, `approve_journal_entry`, `post_journal_entry`, `reverse_journal_entry`, `bulk_approve_journal_entries`, `close_fiscal_year`, `reopen_fiscal_year`, `list_fiscal_positions`, `list_payment_terms`, `list_payment_methods`, `list_tax_repartition_lines`.

Odoo anchors:

- `account_accountant/models/account_move.py`: journal item reconciliation and payment widget hooks.
- `account_accountant/wizard/account_reconcile_wizard.py`: explicit reconciliation with lock-date warnings, write-off/transfer fields, partial/full reconciliation.
- `account_reports/models/account_report.py`: report engine, sections, variants, audit cells, PDF/XLSX export.
- `account_reports/models/account_return.py`: statutory return lifecycle: validate, submit, pay, archive, reset, working files, audit balances.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/lock-dates`, `/lock-dates/{id}/change-request` | Odoo accountant workflows protect period/fiscal locks during reconciliation and reporting. |
| P1 | `/journal-items`, `/journal-items/reconcile`, `/journal-items/{id}/unreconcile` | Reconciliation is journal-item centered, not only bank-transaction centered. |
| P1 | `/reports/{id}/audit-lines`, `/reports/{id}/audit-cell` | Odoo reports drill from report cells to source move lines. |
| P2 | `/multi-currency-revaluations`, `/multi-currency-revaluations/{id}/post` | Odoo has multi-currency revaluation report/workflow. |
| P2 | `/intercompany-rules`, `/intercompany-transactions` | Enterprise accounting needs inter-company automation. |
| P3 | `/statutory-returns`, `/statutory-returns/{id}/validate`, `/submit`, `/pay`, `/archive` | Odoo's return model turns reports into filing workflows. |

## `invoice` Backlog

Current RERP anchors:

- `openapi/accounting/invoice/openapi.yaml`
- Key operations: `list_invoices`, `create_invoice`, `list_invoice_line_items`, `calculate_tax`, `approve_invoice`, `post_invoice`, `cancel_invoice`, `void_invoice`, `get_invoice_workflow_history`, `invoice_summary`.

Odoo anchors:

- `account_accountant/models/account_move.py`: payment widget and invoice reconciliation hooks.
- `account_invoice_extract/models/account_invoice.py`: invoice extraction automation.
- `documents_account/models/documents_document.py`: create invoices from documents and parse embedded PDFs in UBL/XML invoice documents.
- `account_accountant/tests/test_signature.py`: posted invoice signing behavior.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/invoices/{id}/register-payment` | Odoo treats payment registration as a guided workflow, not plain payment CRUD. |
| P1 | `/invoices/{id}/residuals`, `/invoices/{id}/payment-matches` | Reconciliation depends on residual amounts and candidate matching. |
| P1 | `/credit-notes`, `/refunds`, `/invoices/{id}/reverse` | RERP should make refund semantics explicit rather than split them indirectly across AR/AP. |
| P2 | `/invoice-documents`, `/invoice-documents/{id}/extract`, `/invoice-documents/{id}/approve-extraction` | Odoo Enterprise automates invoice creation from scans/documents. |
| P2 | `/invoices/{id}/attachments`, `/invoices/{id}/embedded-documents` | Document bridge and XML/UBL embedded PDF support matter for e-invoicing. |
| P2 | `/deferred-revenues`, `/deferred-expenses`, `/invoices/{id}/deferrals` | Odoo has deferred reporting/entry workflows tied to invoices. |

## `accounts-receivable` Backlog

Current RERP anchors:

- `openapi/accounting/accounts-receivable/openapi.yaml`
- Key operations: `list_customer_invoices`, `list_payments`, `create_payment_application`, `auto_apply_payment`, `list_credit_memos`, `list_collection_activities`, `list_ar_agings`, `aging_summary`, `collections_summary`.

Odoo anchors:

- `account_followup/models/account_followup.py`: follow-up levels with due-day delay, email, SMS, activity creation, auto execution, per-company uniqueness.
- `account_reports/models/account_partner_ledger.py`: partner ledger handler with partner lines, move-line expansion, send recipients, and journal-item navigation.
- `account_reports/models/account_customer_statement.py`: statement sending.
- `account_reports/models/account_followup_report.py`: follow-up report actions.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/follow-up-levels`, `/follow-up-policies` | Collections need configurable dunning levels rather than ad-hoc collection activities. |
| P1 | `/customers/{id}/statement`, `/customers/{id}/statement/send` | Odoo has customer statements and report send flows. |
| P1 | `/reports/partner-ledger`, `/partners/{id}/ledger-lines` | Partner ledger is central to AR investigation. |
| P2 | `/follow-up-runs`, `/follow-up-runs/{id}/execute` | Odoo can execute follow-ups automatically using email/SMS/activity. |
| P2 | `/collection-cases`, `/collection-cases/{id}/promise-to-pay`, `/dispute-holds` | Enterprise AR needs case state beyond simple collection activity rows. |

## `accounts-payable` Backlog

Current RERP anchors:

- `openapi/accounting/accounts-payable/openapi.yaml`
- Key operations: `list_vendor_invoices`, `create_vendor_invoice`, `list_payments`, `create_payment`, `list_approvals`, `create_approval`, `list_payment_commitments`, `cash_flow_forecast`.

Odoo anchors:

- `account_batch_payment/models/account_batch_payment.py`: batch payment state, payment selection constraints, file generation hook, residual totals.
- `account_iso20022/models/account_batch_payment.py` and `account_iso20022/models/account_journal.py`: ISO20022/SEPA file generation and payment validation.
- `account_invoice_extract/models/account_invoice.py`: bill extraction.
- `l10n_us_1099/__manifest__.py`: supplier reporting.
- `account_3way_match`: 3-way matching addon exists in Enterprise tree.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/payment-batches`, `/payment-batches/{id}/send`, `/payment-batches/{id}/reconcile` | Odoo groups payments before bank deposit/export/reconciliation. |
| P1 | `/payment-batches/{id}/export-file` | ISO20022/SEPA/check/NACHA rails produce bank files. |
| P1 | `/vendor-invoices/{id}/register-payment` | Vendor payment workflow should mirror customer payment workflow. |
| P2 | `/vendor-bill-documents/{id}/extract` | Vendor bill extraction is high leverage for AP. |
| P2 | `/vendor-invoices/{id}/three-way-match` | Purchase receipt/invoice matching is a clear Odoo Enterprise accounting capability. |
| P3 | `/supplier-tax-reports/1099`, `/supplier-tax-reports/{id}/export` | Supplier statutory reporting needs localized extensions. |

## `bank-sync` Backlog

Current RERP anchors:

- `openapi/accounting/bank-sync/openapi.yaml`
- Key operations: `list_bank_accounts`, `list_bank_statements`, `trigger_statement_sync`, `list_bank_transactions`, `match_transaction`, `auto_match_transactions`, `list_reconciliations`, `create_reconciliation`, `complete_reconciliation`, `reconciliation_report`, `cash_position`.

Odoo anchors:

- `account_accountant/models/account_bank_statement.py`: auto-reconcile cron, partner mapping, matching queries, payment/invoice matching, early-payment discount, exchange differences, delete/unreconcile/edit reconciliation.
- `account_accountant/wizard/account_reconcile_wizard.py`: partials, write-offs, transfer account, lock-date warnings, reconciliation model autocomplete.
- `account_accountant/tests/test_reconciliation_matching_rules.py`: matching algorithms, memo/regex, duplicate payments, auto rule creation.
- `account_online_synchronization/models/account_online.py`: provider account, journal assignment, refresh/fetch status, consent-like lifecycle.
- `account_bank_statement_import` and format modules: generic import plus CAMT/CSV/OFX/QIF modules.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/reconciliation-models`, `/reconciliation-models/{id}/rules` | Odoo's matching behavior is configurable and reusable. |
| P1 | `/transactions/{id}/suggestions` | Human reconciliation needs candidate suggestions before commit. |
| P1 | `/transactions/{id}/reconcile`, `/transactions/{id}/unreconcile`, `/transactions/{id}/edit-reconciliation` | Odoo supports reconcile, unreconcile, and edit flows. |
| P1 | `/transactions/{id}/write-off`, `/transactions/{id}/transfer` | Write-off and transfer counterpart workflows are first-class in Odoo's wizard. |
| P1 | `/transactions/{id}/exchange-difference` | Multi-currency reconciliation needs explicit exchange-difference handling. |
| P2 | `/bank-connections`, `/bank-connections/{id}/refresh`, `/bank-connections/{id}/consent` | Online sync needs provider/auth lifecycle. |
| P2 | `/statement-imports`, `/statement-imports/{id}/parse`, `/statement-import-formats` | CAMT/CSV/OFX/QIF import should be an API surface. |
| P2 | `/transactions/{id}/early-payment-discount` | Odoo applies early payment discounts during reconciliation. |

## `asset` Backlog

Current RERP anchors:

- `openapi/accounting/asset/openapi.yaml`
- Key operations: `list_assets`, `create_asset`, `list_depreciation_entries`, `run_depreciation`, `bulk_depreciate`, `list_revaluations`, `create_revaluation`, `list_disposals`, `create_disposal`, `list_asset_categories`, `asset_summary`.

Odoo anchors:

- `account_asset/models/account_asset.py`: asset states `model`, `draft`, `open`, `paused`, `close`, `cancelled`; depreciation method, prorata rules, asset group, model assets, linked assets, journal/account integration.
- `account_asset/wizard/asset_modify.py`: modify, pause, sell/dispose.
- `account_asset/models/account_move.py`: asset creation from posted account moves.
- `account_asset/models/account_assets_report.py`: asset report handler.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/asset-models`, `/asset-groups` | Odoo distinguishes reusable asset models and groups from live assets. |
| P1 | `/assets/{id}/validate`, `/assets/{id}/pause`, `/assets/{id}/resume`, `/assets/{id}/close`, `/assets/{id}/cancel` | RERP currently has depreciation/disposal but not the full asset state machine. |
| P1 | `/assets/{id}/modify` | Odoo has a dedicated asset modification wizard. |
| P1 | `/assets/{id}/sell-dispose` | Disposal should be a workflow with accounting effects, not just a record. |
| P2 | `/assets/{id}/journal-entries`, `/assets/{id}/post-depreciation-entry` | Odoo creates account moves for asset lifecycle events. |
| P2 | `/assets/{id}/analytic-distribution` | Odoo assets inherit analytic capabilities. |

## `budget` Backlog

Current RERP anchors:

- `openapi/accounting/budget/openapi.yaml`
- Key operations: `list_budgets`, `create_budget`, `submit_budget`, `list_budget_lines`, `create_budget_line`, `variance_report`, `list_forecasts`, `create_forecast`.

Odoo anchors:

- `account_budget/models/budget_analytic.py`: budget status `draft`, `confirmed`, `revised`, `done`, `canceled`; parent/child revisions; budget type revenue/expense/both; budget report action.
- `account_budget/models/budget_line.py`: budget line detail.
- `account_budget/wizards/budget_split_wizard_view.xml`: split workflow.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/budgets/{id}/confirm`, `/budgets/{id}/cancel`, `/budgets/{id}/complete`, `/budgets/{id}/reopen-draft` | RERP only has submit; Odoo has a fuller lifecycle. |
| P1 | `/budgets/{id}/revisions`, `/budgets/{id}/create-revision` | Odoo budgets are revision-aware. |
| P1 | `/analytic-dimensions`, `/budgets/{id}/analytic-lines` | Odoo budgets are analytic-account centered. |
| P2 | `/budgets/{id}/split-lines` | Odoo has split wizard behavior. |
| P2 | `/reports/budget-analysis`, `/reports/budget-variance/pivot` | Budget reporting should support analytic/report views, not only summary variance. |

## `financial-reports` Backlog

Current RERP anchors:

- `openapi/accounting/financial-reports/openapi.yaml`
- Key operations: `generate_balance_sheet`, `generate_income_statement`, `generate_cash_flow`, `generate_general_ledger`, `generate_trial_balance`, `list_custom_reports`, `create_custom_report`, `execute_custom_report`, `list_report_executions`.

Odoo anchors:

- `account_reports/models/account_report.py`: report custom handlers, sections, variants, send cron, audit cells, PDF/XLSX export, composite reports, carryovers.
- `account_reports/models/account_partner_ledger.py`: partner ledger expansion and send recipients.
- `account_reports/models/account_journal_report.py`: PDF/XLSX journal export and tax summary.
- `account_reports/models/account_generic_tax_report.py`: generic tax report and tax audit.
- `account_reports/models/account_return.py`: statutory return lifecycle.
- `account_reports/wizard/account_report_send.py`: send-and-print processing.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/report-definitions`, `/report-definitions/{id}/lines`, `/report-definitions/{id}/expressions` | One-off report responses will not match Odoo-grade report configurability. |
| P1 | `/reports/partner-ledger`, `/reports/aged-receivable`, `/reports/aged-payable`, `/reports/customer-statement` | These are core accounting investigation reports. |
| P1 | `/reports/{id}/options`, `/reports/{id}/lines`, `/reports/{id}/unfold` | Odoo reports are interactive and option-driven. |
| P1 | `/reports/{id}/audit-cell`, `/reports/{id}/source-lines` | Drill-down is essential for accounting trust. |
| P1 | `/report-executions/{id}/export/pdf`, `/export/xlsx` | Odoo supports PDF/XLSX export. |
| P2 | `/report-schedules`, `/report-schedules/{id}/send` | Odoo has scheduled report sending. |
| P2 | `/reports/multicurrency-revaluation`, `/reports/deferred-revenue`, `/reports/deferred-expense`, `/reports/journal`, `/reports/tax` | These reports are prominent Odoo Enterprise gaps. |
| P3 | `/statutory-returns`, `/statutory-returns/{id}/working-files` | Statutory returns build on report definitions and exports. |

## `edi` Backlog

Current RERP anchors:

- `openapi/accounting/edi/openapi.yaml`
- Key operations: `list_edi_documents`, `create_edi_document`, `update_edi_document`, `list_edi_mappings`, `create_edi_mapping`.

Odoo anchors:

- `documents_account/models/documents_document.py`: embedded UBL/PDF extraction and create invoice/bank statement from document.
- `account_iso20022/models/account_journal.py`: ISO20022 XML generation.
- `account_sepa_direct_debit/models/sdd_mandate.py`: mandate lifecycle and direct debit collection requirements.
- `account_intrastat/__manifest__.py`: Intrastat goods/services reports and commodity codes.
- `account_saft/__manifest__.py`: SAF-T reporting base.
- `l10n_uk_hmrc/__manifest__.py`: HMRC API integration.
- `l10n_us_1099/__manifest__.py`: US 1099 reporting.

Backlog:

| Priority | Resource | Why |
|---|---|---|
| P1 | `/edi-profiles`, `/edi-validation-profiles` | CRUD mappings are not enough; EDI needs standards/profiles. |
| P1 | `/edi-submissions`, `/edi-submissions/{id}/submit`, `/edi-submissions/{id}/status`, `/edi-submissions/{id}/retry` | Submission lifecycle is the core EDI behavior. |
| P1 | `/edi-acknowledgments`, `/edi-errors` | EDI operations require acceptance/rejection tracking. |
| P2 | `/payment-files/iso20022`, `/payment-files/sepa-credit-transfer`, `/direct-debit-mandates` | Some file-based accounting rails are EDI-adjacent. |
| P2 | `/invoice-documents/ubl`, `/invoice-documents/peppol` | Odoo handles XML invoice documents with embedded PDFs. |
| P3 | `/intrastat/reports`, `/saft/reports`, `/hmrc/obligations`, `/1099/reports` | Localization modules should plug into statutory/EDI surfaces. |

## Cross-Service Design Rules

Use these rules when turning the backlog into OpenAPI:

1. Prefer workflow resources over RPC-only action names where state and audit matter.
2. Keep jurisdiction-specific APIs behind localization/statutory extensions, not core GL.
3. Model reports as definitions, options, executions, lines, exports, and audit drill-downs.
4. Model reconciliation as candidate discovery, human decision, posting, and reversal.
5. Treat documents as first-class accounting inputs with extraction status and linkage to invoices/statements.
6. Treat payment files as generated artifacts with validation, download, sent/reconciled states, and bank format metadata.
7. After any service spec expansion, run the accounting BFF generator and validate that the generated BFF contains the new service operationIds.

## Recommended Build Order

1. **P0 BFF generator coverage**: preserve generator coverage validation and settle `openapi/accounting/openapi.yaml`.
2. **P1 bank reconciliation**: rules, suggestions, reconcile/unreconcile, write-offs, exchange differences.
3. **P1 report engine**: definitions, options, lines, audit cell, PDF/XLSX export, partner ledger/aging/tax reports.
4. **P1 AR follow-up**: levels, policies, statement sending, follow-up runs.
5. **P1 AP payments**: payment batches, export files, vendor payment registration.
6. **P2 documents/extraction**: invoice/bill extraction and document linkage.
7. **P2 assets/budgets lifecycle**: lifecycle actions, revisions, analytic dimensions.
8. **P3 localization/statutory**: Intrastat, SAF-T, HMRC, 1099, country e-invoicing.

## BDD Traceability

This document should stay focused on OpenAPI resource planning. Broad behavior lives in [`ACCOUNTING_BDD_FEATURE_BACKLOG.md`](./ACCOUNTING_BDD_FEATURE_BACKLOG.md), which maps the maturity target to high-level Given/When/Then scenarios.

When expanding a service contract:

1. Start from the relevant feature group in the BDD backlog.
2. Add or update planned resources in this service map.
3. Update the service-level OpenAPI spec.
4. Regenerate `openapi/accounting/openapi_bff.yaml`.
5. Validate generated operation coverage.
6. Decompose broad BDD scenarios into implementation-level tests for state transitions, validation failures, idempotency, audit, and reversal paths.

## Verification Checklist

For each OpenAPI expansion:

- Service-level spec includes operationIds, tags, security, pagination where needed, request/response schemas, and standard error schemas.
- Suite BFF output includes the new operation.
- Generated crate compiles through the suite-aware `rerp` wrapper.
- Implementation crate owns behavior; generated crate remains disposable.
- Entity and Lifeguard migration changes are made before controller implementation.
- Tests cover workflow state transitions, validation failures, and idempotency/reversal paths.
