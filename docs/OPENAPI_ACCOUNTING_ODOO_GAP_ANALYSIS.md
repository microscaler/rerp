# RERP Accounting OpenAPI vs Odoo Enterprise Gap Analysis

Date: 2026-04-25

Scope:

- RERP accounting suite OpenAPI specs under `openapi/accounting/`.
- Odoo Enterprise accounting-related addons under `/Users/casibbald/Workspace/remote/caffeinated.expert/odooforks/enterprise`.

Important caveat: the checked Odoo path is the Enterprise addons tree. Several base accounting models live in Odoo Community/core modules outside this tree. This analysis therefore compares RERP against Odoo Enterprise's advanced accounting surface: accountant workflow, reporting, assets, budgets, bank synchronization/import, follow-up, batch payments, EDI/localization, documents, and extraction.

## Executive Summary

RERP accounting is no longer just a skeleton. The service-level specs now cover general ledger, invoicing, AR/AP, bank sync, assets, budgets, EDI, and financial reports with security schemes, pagination schemas, workflow actions, and error schemas in most files.

The main gaps versus Odoo Enterprise are now:

1. **BFF generator coverage is fixed; top-level legacy policy remains**: `openapi/accounting/openapi_bff.yaml` now aggregates all nine service specs selected by `openapi/accounting/bff-suite-config.yaml` and exposes 111 namespaced path templates. `openapi/accounting/openapi.yaml` still needs an explicit retention/deprecation policy.
2. **Reconciliation depth is far behind Odoo**: RERP has `match_transaction`, `auto_match_transactions`, and basic reconciliation records; Odoo has rich reconciliation widgets, matching rules, partner-bank detection, early-payment discounts, exchange differences, partial/full reconciliation, unreconciliation, and extensive test coverage.
3. **Reporting is broad but not Odoo-grade**: RERP has balance sheet, income statement, cash flow, trial balance, general ledger, and custom reports. Odoo Enterprise adds configurable report engines, partner ledger, aged payable/receivable, tax reports/returns, audit cells, multi-currency revaluation, deferred reports, sales reports, PDF/XLSX export, scheduled report sending, and report sections/variants.
4. **Localization/statutory compliance is mostly absent**: Odoo Enterprise has many `l10n_*_reports`, EDI, Intrastat, SAF-T, HMRC, 1099, country payment files, and payroll-accounting integrations. RERP currently has fiscal positions and tax repartition lines, but not jurisdiction-specific statutory workflows.
5. **Payments and banking lack enterprise rails**: Odoo includes batch payments, SEPA direct debit, ISO20022/SEPA credit transfer, check printing, payment file formats, and multiple bank statement import connectors. RERP has bank accounts/statements/transactions and payment methods, but not full payment file, mandate, or import-format coverage.
6. **Document/extraction automation is missing**: Odoo has invoice OCR/extraction (`account_invoice_extract`) and accounting-documents integration (`documents_account`). RERP accounting specs do not yet expose document ingestion, attachment classification, OCR extraction, or report-to-document workflows.

## Target State: World-Class Accounting OpenAPI

RERP should not copy Odoo's module structure directly. The target is a service-oriented, OpenAPI-first accounting suite that exposes the same operational depth: guided workflows, accounting controls, human review loops, auditability, generated artifacts, and jurisdiction extension points.

The accounting OpenAPI target is reached when the suite documents and later implements these engines:

### General Ledger Controls Engine

Target capabilities:

- Period, fiscal, tax, and soft lock dates with change-request workflows.
- Journal-item reconciliation and unreconciliation, not only bank-transaction matching.
- Immutable audit trails for postings, reversals, lock changes, and approval decisions.
- Multi-company and intercompany rules, including generated reciprocal entries.
- Multi-currency revaluation and exchange-difference posting.
- Statutory return lifecycle built from report definitions and GL source balances.

Definition of done:

- The GL API can prove who changed accounting controls, when, why, and what source entries were affected.
- Closing, reopening, reversing, and revaluing are stateful workflows with idempotency and reversal semantics.
- Report audit drill-down can navigate from a balance to source journal items.

### Reconciliation Engine

Target capabilities:

- Reconciliation models/rules with memo, amount, partner, date, currency, and account predicates.
- Candidate suggestions for bank transactions and journal items before commit.
- Human-in-the-loop reconcile, unreconcile, edit-reconciliation, write-off, transfer, fee, partial, and exchange-difference flows.
- Online bank connection lifecycle, statement import formats, duplicate/missing transaction workflows, and provider consent refresh.

Definition of done:

- Reconciliation is modeled as candidate discovery, accountant decision, posting, audit, and reversal.
- Bank and GL reconciliation share common concepts where possible while preserving their service boundaries.
- Every reconciliation outcome can be traced to rules, user choices, source transactions, and generated journal entries.

### Report Engine

Target capabilities:

- Report definitions with lines, columns, expressions, options, variants, sections, and custom handlers.
- Interactive report lines with unfold, source-line drill-down, and audit-cell navigation.
- Report executions with PDF/XLSX export, scheduled sends, report-to-documents flow, and execution history.
- Partner ledger, aged receivable, aged payable, customer statement, tax, journal, deferred revenue/expense, multi-currency revaluation, and statutory-return reports.

Definition of done:

- Core financial statements are generated through reusable report definitions rather than one-off endpoints.
- Accountants can drill from any report total to source lines and export the same evidence package.
- Report executions are durable artifacts with options, actor, timestamp, source data window, and output files.

### AR Follow-Up And Collections Engine

Target capabilities:

- Follow-up levels, policies, partner/customer settings, reminder channels, and per-company defaults.
- Customer statements, partner ledger, follow-up reports, and automatic follow-up runs.
- Collection cases, dispute holds, promise-to-pay, dunning pause/resume, write-off policy, and escalation history.

Definition of done:

- Collections are policy-driven instead of ad-hoc activity rows.
- Every customer contact can be tied to overdue invoices, aging, policy level, channel, and outcome.
- Statements and reminders are reproducible accounting artifacts.

### AP Payments And Vendor Controls Engine

Target capabilities:

- Vendor payment registration, payment batches, payment selection rules, approval gates, and generated export files.
- Payment rails such as ISO20022/SEPA/NACHA/check/direct debit as format profiles, not hardcoded special cases.
- Vendor bill extraction, duplicate bill detection, 3-way match, vendor statement reconciliation, and supplier statutory reporting.

Definition of done:

- AP can move from approved invoice to controlled payment file with validation, audit, download, sent, and reconciled states.
- Payment artifacts are immutable once sent and remain traceable to source invoices and bank reconciliation.
- Supplier reporting is modeled as localization/statutory extension output.

### Documents, Extraction, And EDI Engine

Target capabilities:

- Accounting documents with upload, classification, OCR/extraction, review, approval, linkage, and embedded document parsing.
- EDI profiles, validation profiles, submissions, polling, acknowledgments, rejections, retry, audit logs, and attachments.
- Standards and localization surfaces for UBL, PEPPOL, Factur-X, SAF-T, Intrastat, HMRC, 1099, country e-invoicing, ISO20022, and direct debit mandates.

Definition of done:

- Documents are first-class accounting inputs, not only attachments.
- EDI has a submission lifecycle with external status, validation errors, retry semantics, and jurisdiction profiles.
- Extracted data can be reviewed and converted into invoices, bills, bank statements, or statutory files.

### Asset And Budget Engines

Target capabilities:

- Asset models/groups, full asset lifecycle, depreciation/revaluation/disposal journal entries, partial disposals, modification, pause/resume, and analytic distribution.
- Budget lifecycle beyond submit: confirm, approve, revise, split, complete, cancel, reopen, forecast, and compare scenarios.
- Analytic dimensions shared across budgets, GL, reporting, projects, departments, and cost centers.

Definition of done:

- Asset lifecycle events produce auditable accounting effects.
- Budgets are revision-aware and can be compared against actuals through report definitions.
- Analytic dimensions are reusable contract concepts rather than service-local filters.

## Target Deliverable Criteria

A world-class RERP accounting deliverable should meet these cross-service criteria:

- **Contract completeness:** Every engine has OpenAPI resources for setup, execution, review, reversal, audit, and generated artifacts.
- **Behavior traceability:** Each broad BDD feature maps to service boundaries, OpenAPI operationIds or planned resources, and later implementation tests.
- **Accounting controls:** State transitions protect posted data, lock dates, approvals, idempotency, and reversal paths.
- **Auditability:** Workflows expose actor, timestamp, source documents, source transactions, generated entries, and reason codes.
- **Localization extensibility:** Country-specific APIs plug into statutory, EDI, payment-file, and tax-return surfaces without bloating core GL.
- **BFF coverage:** Any service spec expansion is regenerated into `openapi/accounting/openapi_bff.yaml`, and operation coverage is validated.
- **OpenAPI hygiene:** Report generation semantics, tags, security, pagination, errors, and service port metadata stay consistent enough for generated clients.

## Microservice Boundary Target

Not every missing capability needs a new microservice. Use three categories when building the next `openapi.yaml` files:

1. **Enhance existing services** when the missing behavior belongs to a service already owning the lifecycle. Examples: reconciliation depth in `bank-sync`, report engine depth in `financial-reports`, follow-up depth in `accounts-receivable`, payment-batch depth in `accounts-payable`, lifecycle depth in `asset` and `budget`, and submission lifecycle in `edi`.
2. **Add new accounting microservices** when the capability has independent lifecycle, data ownership, or release cadence. Initial targets are `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, and eventually `audit-controls` if controls become cross-service.
3. **Add extension/localization services** when the behavior is jurisdiction-specific. Country tax filings, EDI variants, statutory reports, and payment-file formats should plug into core services rather than hardcode local rules into `general-ledger`, `accounts-payable`, or `edi`.

Proposed new accounting services:

| Proposed service | Why it should be separate |
|---|---|
| `tax-compliance` | Tax periods, returns, filing, tax payments, jurisdiction rules, and audit packs have their own statutory lifecycle. |
| `documents-extraction` | OCR, classification, extraction review, and document linkage span invoices, bills, bank statements, and EDI. |
| `treasury` | Liquidity planning and bank relationship management are broader than transactional bank sync. |
| `consolidation` | Group consolidation, eliminations, and reporting packs own multi-company lifecycle and data. |
| `revenue-recognition` | Deferred revenue/expense and recognition runs need schedule ownership separate from invoice CRUD. |
| `lease-accounting` | Lease liabilities, right-of-use assets, payment schedules, and lease modifications form a distinct accounting domain. |
| `audit-controls` | Approval matrices, segregation of duties, signatures, and immutable audit events may need a shared controls layer once they span services. |

## RERP Accounting Current Surface

Service-level path template counts:

| RERP service spec | Path templates | Character |
|---|---:|---|
| `general-ledger/openapi.yaml` | 39 | Most mature, deep GL/fiscal/payment/tax config surface |
| `accounts-receivable/openapi.yaml` | 13 | Customer invoices, payments, applications, credit memos, collections, aging |
| `bank-sync/openapi.yaml` | 12 | Bank accounts/statements/transactions, matching, reconciliations, reports |
| `invoice/openapi.yaml` | 12 | Invoices, line items, tax rates/calculation, approve/post/cancel/void workflow |
| `asset/openapi.yaml` | 9 | Assets, depreciation, revaluation, disposal, categories, summary |
| `financial-reports/openapi.yaml` | 9 | Core financial reports and custom report executions |
| `accounts-payable/openapi.yaml` | 7 | Vendor invoices, payments, approvals, commitments, cash-flow forecast |
| `budget/openapi.yaml` | 6 | Budgets, lines, submit, variance, forecasts |
| `edi/openapi.yaml` | 4 | EDI documents and mappings |

Total service-level path templates: about 111.

The service specs now include `securitySchemes.bearerAuth`, schemas, and more operational endpoints than the older `docs/OPENAPI_SPEC_AUDIT.md` suggests. That older audit is useful historically, but stale for accounting security/pagination/schema coverage.

## BFF Generator Coverage Status

The immediate BFF coverage issue has been fixed:

- `openapi/accounting/*/openapi.yaml` service specs contain the richer workflow/report/reporting APIs.
- `openapi/accounting/openapi_bff.yaml` is now generated from every service spec selected by `openapi/accounting/bff-suite-config.yaml`.
- The generated BFF exposes 111 namespaced path templates and 185 operations.
- Service-local path collisions are valid because the BFF public surface prefixes paths with each service `base_path`; for example, AP and AR local `/payments` publish as `/api/accounts-payable/payments` and `/api/accounts-receivable/payments`.
- The wrapper validates that generated operationIds cover selected service specs.

Remaining follow-up:

- Decide whether `openapi/accounting/openapi.yaml` is a legacy aggregate. If retained, regenerate it from the same source as `openapi/accounting/openapi_bff.yaml`; if not, explicitly deprecate it.
- Keep stale names such as `journal-entrys` out of generated output by fixing generator/config mappings rather than hand-editing generated artifacts.

## Odoo Enterprise Accounting Surface Observed

Relevant Odoo Enterprise modules include:

- `account_accountant`: enterprise invoicing/accountant access, reconciliation, lock dates, bank reconciliation widget, accountant menus.
- `account_reports`: report engine, balance sheet, cash flow, executive summary, P&L, bank reconciliation report, aged partner balance, general ledger, trial balance, sales report, partner ledger, customer statement, follow-up report, multi-currency revaluation, deferred reports, journal report, generic tax report, return flows, audit views.
- `account_asset`: asset management, depreciation, journal entries, asset reports, asset groups.
- `account_budget`: analytic budgets, budget lines, split wizard, budget reports.
- `account_online_synchronization`: online bank account links, scheduled synchronization, missing/duplicate transaction wizards.
- `account_bank_statement_import`: generic bank statement import framework.
- `account_batch_payment`: grouped payments for bank deposits/reconciliation.
- `account_iso20022`: SEPA credit transfer / ISO20022 payment export.
- `account_sepa_direct_debit`: SEPA direct debit mandates and pain.008 file generation.
- `account_followup`: multi-level unpaid invoice follow-up letters/emails/activities.
- `account_invoice_extract` and `documents_account`: invoice extraction and accounting-document bridge.
- `account_intrastat`, `account_saft`, and many `l10n_*_reports` / `l10n_*_edi` modules: statutory/localization reporting.

Odoo's Enterprise accounting feature set is less a single API and more a modular ecosystem around accounting operations.

## Gap Analysis By RERP Service

### General Ledger

RERP strengths:

- Accounts, chart of accounts, journals, journal entries, fiscal periods.
- Trial balance, ledger, account balances.
- Chart templates, fiscal years, fiscal positions, payment terms, payment methods, tax repartition lines.
- Journal-entry approve/post/reverse and bulk actions.

Odoo-grade gaps:

- Account lock-date and fiscal lock-date workflows are under-specified compared to `account_accountant`.
- No audit trail/report audit-cell navigation equivalent to Odoo's report audit views.
- No multi-company consolidation/inter-company rules in the GL API surface.
- No opening/closing entry workflow tied to local statutory returns.
- No analytic dimension integration in GL reports beyond basic account filters.

### Invoice

RERP strengths:

- Invoice CRUD, line items, tax rates/calculation.
- Approve/post/cancel/void workflow and workflow history.
- Invoice summary report.

Odoo-grade gaps:

- Customer/vendor invoice unification and credit note/refund flows are split across services but not clearly reconciled.
- No payment-register wizard semantics, payment matching, residuals, partial payments, or early-payment discounts.
- No e-invoicing / EDI lifecycle on invoices.
- No attachment/OCR/document ingestion.
- No recurring/deferred revenue/expense integration.
- No chatter/activity/follow-up hooks.

### Accounts Receivable

RERP strengths:

- Customer invoices, payments, payment applications, auto-apply, credit memos.
- Collections activities, AR aging, aging and collections summaries.

Odoo-grade gaps:

- Follow-up levels/policies are much thinner than Odoo `account_followup`.
- No automated reminder channel model (email, SMS, letters, activities) or per-company follow-up policy.
- No partner ledger/customer statement API comparable to Odoo reports.
- No dispute management, promise-to-pay, dunning hold, or collection case lifecycle.
- No multi-currency reconciliation details.

### Accounts Payable

RERP strengths:

- Vendor invoices, payments, approvals, payment commitments, cash-flow forecast.

Odoo-grade gaps:

- No vendor bill OCR/extraction.
- No batch payment file generation.
- No SEPA/ISO20022/check/NACHA-style payment rails.
- No 3-way match semantics despite Odoo having `account_3way_match`.
- No vendor statement/reconciliation flow.
- No country-specific supplier reporting such as 1099.

### Bank Sync / Reconciliation

RERP strengths:

- Bank accounts, statements, transactions.
- Trigger sync, match transaction, auto-match transactions.
- Reconciliations, complete reconciliation, reconciliation/cash-position reports.

Odoo-grade gaps:

- No reconciliation model/rule configuration equivalent to Odoo `account.reconcile.model`.
- No bank reconciliation widget semantics: suggested counterpart lines, partner detection, write-off models, fees, partials, exchange differences, early payment discounts.
- No import-format modules for CAMT, CSV, OFX, QIF.
- No online synchronization provider lifecycle: institution links, consent/auth refresh, duplicate/missing transaction workflows.
- No unreconcile/delete-reconciled-line flow.

### Asset

RERP strengths:

- Asset CRUD, depreciation entries, run/bulk depreciation.
- Revaluations, disposals, categories, asset summary.

Odoo-grade gaps:

- No asset group/model template semantics.
- No automatic journal entry generation for depreciation/revaluation/disposal in the API contract.
- No partial disposal, method changes, prorata/period rules, or asset modification wizard equivalent.
- No integration with project/fleet/manufacturing asset contexts.

### Budget

RERP strengths:

- Budgets, budget lines, submit workflow.
- Variance report, forecasts.

Odoo-grade gaps:

- Odoo budgets are analytic-account centered; RERP needs explicit analytic dimensions/cost centers/projects/departments integration.
- No budget split wizard, revisioning, allocations, commitments/encumbrance integration, or forecast scenario lifecycle.
- No approval matrix or version compare endpoint.

### Financial Reports

RERP strengths:

- Balance sheet, income statement, cash flow, general ledger, trial balance.
- Custom reports and report executions.

Odoo-grade gaps:

- No report engine abstraction comparable to Odoo `account.report`, report lines, expressions, columns, sections, variants, filters, unfold/audit cells, and custom handlers.
- Missing partner ledger, aged payable, aged receivable, customer statements, tax report/returns, journal report, executive summary, multi-currency revaluation, deferred revenue/expense reports.
- No export endpoints for PDF/XLSX or report-to-documents flow.
- No scheduled report sending.

### EDI

RERP strengths:

- EDI documents and mappings CRUD.

Odoo-grade gaps:

- No concrete protocol/document standards: PEPPOL, Factur-X, country e-invoicing, SAF-T, Intrastat, HMRC, local tax returns, ISO20022 files.
- No submission, polling, acceptance/rejection, retry, audit log, or attachment lifecycle.
- No per-jurisdiction validation profile.

## Cross-Cutting Missing Concepts

These concepts recur across Odoo Enterprise and should become shared RERP accounting patterns:

- Multi-company and inter-company accounting.
- Multi-currency revaluation and exchange-difference entries.
- Analytic dimensions across GL, budget, reports, projects, and products.
- Configurable report engine with audit drill-down.
- Partner ledger and statement model.
- Document attachments, OCR/extraction, and accounting documents bridge.
- Payment files, mandates, batches, and bank export/import formats.
- Country-specific statutory reporting and electronic filing.
- Reconciliation rules and a human-in-the-loop reconciliation workflow.
- Chatter/activity/follow-up automation.
- Strong workflow state machines with lock dates and audit trails.

## Prioritized Remediation Plan

### P0 — Preserve BFF Coverage And Settle Top-Level Spec Policy

1. Keep `rerp bff generate-system --suite accounting` as the source of truth for `openapi/accounting/openapi_bff.yaml`.
2. Validate generated BFF output against selected service-level operationIds after every service spec expansion.
3. Decide whether `openapi/accounting/openapi.yaml` is a legacy artifact. If yes, deprecate or regenerate it from the same source.
4. Fix stale pluralization such as `journal-entrys` in generator/config mapping, not by hand-editing generated output.

### P1 — Add Odoo-Grade Reconciliation And Reporting

1. Expand bank-sync with reconciliation rules/models, suggestions, write-offs, partner detection, partial reconciliation, exchange differences, unreconcile.
2. Expand financial-reports with partner ledger, aged receivable/payable, tax report, journal report, customer statement, report export, audit drill-down.
3. Add report engine schemas rather than one-off report request/response schemas.

### P2 — Add Enterprise Payment And Document Flows

1. Add batch payments and payment export files (ISO20022/SEPA/NACHA/check depending on localization target).
2. Add invoice document/OCR/extraction workflows.
3. Add follow-up policy levels and communication actions.

### P3 — Add Localization/Compliance Surface

1. Model statutory returns and tax filing workflows.
2. Add Intrastat/SAF-T/e-invoicing patterns.
3. Add jurisdiction-specific report modules as suite extensions rather than hardcoding them into core GL.

## Suggested New RERP OpenAPI Resources

High-value API resources to add:

- `/reconciliation-models`, `/reconciliation-suggestions`, `/bank-transactions/{id}/reconcile`, `/bank-transactions/{id}/unreconcile`
- `/reports/partner-ledger`, `/reports/aged-receivable`, `/reports/aged-payable`, `/reports/tax`, `/reports/journal`, `/reports/multicurrency-revaluation`
- `/report-definitions`, `/report-definitions/{id}/lines`, `/report-executions/{id}/export`
- `/follow-up-levels`, `/follow-up-campaigns`, `/customers/{id}/follow-up`
- `/batch-payments`, `/payment-files`, `/payment-mandates`
- `/documents/invoices/extract`, `/documents/{id}/link-invoice`
- `/statutory-returns`, `/statutory-returns/{id}/submit`
- `/edi-submissions`, `/edi-submissions/{id}/status`

## Conclusion

RERP accounting has a solid core shape and is now ahead of a pure CRUD scaffold in several services. The next step is not simply adding more CRUD resources. The biggest Odoo-derived gaps are enterprise workflows: reconciliation, reporting engine configurability, statutory compliance, bank/payment file rails, document extraction, and follow-up automation.

The BFF coverage fix means future work can proceed from a reliable suite entry point. The next documentation and implementation phases should keep the maturity target, service resource map, and broad BDD feature backlog aligned so RERP expands toward a coherent accounting product rather than a collection of unrelated endpoints.
