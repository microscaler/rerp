# Accounting OpenAPI vs Odoo Enterprise Gap

- **Status**: `partially-verified`
- **Source docs**: [`docs/OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md), [`docs/OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md), [`docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md), [`docs/OPENAPI_SPEC_AUDIT.md`](../../OPENAPI_SPEC_AUDIT.md)
- **Code anchors**: `openapi/accounting/*/openapi.yaml`, `openapi/accounting/openapi_bff.yaml`, `openapi/accounting/openapi.yaml`, Odoo Enterprise `account_*`, `account_reports`, `l10n_*` modules
- **Last updated**: 2026-04-25

## What It Is

RERP accounting service specs have grown beyond the older skeleton assessment: the service-level OpenAPI files cover general ledger, invoicing, AR/AP, bank sync, assets, budgets, EDI, financial reports, tax compliance, documents extraction, treasury, consolidation, revenue recognition, lease accounting, and audit controls.

Compared to Odoo Enterprise, the remaining gap is enterprise depth rather than basic scaffolding.

## Generator Coverage Status

The service-level specs now expose 203 generated BFF path templates and 320 generated BFF operations. `openapi/accounting/openapi_bff.yaml` aggregates the selected accounting service specs under namespaced paths from `bff-suite-config.yaml`.

This means future work can treat the generated BFF as the accounting suite entry point while continuing to validate operation coverage after every service spec expansion.

> **Remaining P0:** Decide whether `openapi/accounting/openapi.yaml` is a legacy aggregate. If retained, regenerate it from the same source as `openapi/accounting/openapi_bff.yaml`; if not, explicitly deprecate it.

The BFF public path contract comes from `bff-suite-config.yaml` `base_path`: service-local paths remain local, while generated BFF paths are namespaced by the service base path. For example, AP and AR can both own local `/payments`; the suite BFF must publish them as `/api/accounts-payable/payments` and `/api/accounts-receivable/payments`.

> **Implemented:** The BRRTRouter BFF merge now prefixes public paths with `base_path`, and the RERP wrapper uses the config-based BFF generator for `rerp bff generate-system`. After the missing service build-out, the regenerated accounting BFF contains 203 paths and 320 operations.

## Odoo-Derived Gap Themes

The biggest missing concepts are:

- Reconciliation rules, suggestions, write-offs, partner detection, partial/full reconciliation, unreconcile, early-payment discounts, and exchange differences.
- Report engine configurability: report definitions, lines, expressions, columns, filters, unfold/audit cells, variants, PDF/XLSX export, scheduled sends.
- Partner ledger, aged receivable/payable, customer statements, tax returns, journal report, multi-currency revaluation, deferred revenue/expense reports.
- Batch payments, payment export files, direct debit mandates, ISO20022/SEPA/NACHA/check rails, and statement import formats.
- Invoice OCR/extraction, accounting documents bridge, and report-to-documents flows.
- Localization/statutory compliance: Intrastat, SAF-T, HMRC, 1099, country e-invoicing/EDI, and electronic filing.
- Follow-up policies, dunning levels, automated reminders via email/SMS/letters/activities.

## Current Recommendation

Treat the accounting suite in three passes:

1. **Workflow depth:** prioritize reconciliation and reporting engine gaps because they unlock the most accounting value.
2. **Enterprise breadth:** add payment rails, document extraction, follow-up automation, and localization modules as suite extensions rather than hardcoding them into core GL.
3. **Contract hygiene:** normalize OpenAPI semantics before client generation hardens around them, especially report endpoints that currently use `GET` with request bodies.

## Maturity Target

The target is a service-oriented, OpenAPI-first accounting suite with accounting engines rather than disconnected CRUD endpoints:

- **GL controls:** lock dates, close controls, audit trail, journal-item reconciliation, intercompany, statutory returns, and multi-currency revaluation.
- **Reconciliation:** configurable rules, suggestions, write-offs, partials, exchange differences, unreconcile/edit flows, provider/import lifecycle.
- **Reports:** definitions, lines, expressions, options, unfold, audit-cell/source-line drill-down, exports, schedules, partner ledgers, tax reports, and statutory returns.
- **AR/AP:** follow-up policies, statements, collection cases, payment batches, payment files, vendor payment registration, 3-way match, supplier reporting.
- **Documents and EDI:** OCR/extraction, accounting document linkage, EDI profiles, submissions, acknowledgments, validation profiles, retries, and localization flows.
- **Assets and budgets:** lifecycle state machines, generated journal entries, asset models/groups, budget revisions, analytic dimensions, and variance analysis.

The broad BDD backlog now lives in `docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md`. It should be used before expanding OpenAPI contracts so feature behavior, planned resources, generated BFF operations, and implementation tests stay traceable.

## Service Boundary Guidance

Use `docs/OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md` before creating new accounting service specs. The current split is:

- **Enhance existing services:** `general-ledger`, `bank-sync`, `financial-reports`, `accounts-receivable`, `accounts-payable`, `asset`, `budget`, `edi`, and `invoice` should absorb the gaps that deepen their current lifecycle ownership.
- **Added accounting microservices:** `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, and `audit-controls` now have initial `openapi/accounting/{service}/openapi.yaml` contracts and BFF config entries.
- **Use extension/localization services:** country tax filings, e-invoicing formats, statutory reports, and payment-file formats should extend core services instead of hardcoding jurisdiction rules into core accounting.

## Pre-Rules Stabilization Status

Before rules-engine expansion, the highest-risk placeholder contracts were normalized into named schemas. This includes reconciliation model requests/results, AP payment-batch and payment-file requests, report definitions and exports, AR follow-up and collection workflows, GL lock/reconciliation/revaluation requests, invoice payment and handoff contracts, asset model/modification contracts, and budget revisions.

The new accounting services use service ports `8016-8022` because `8011-8015` are already assigned to non-accounting services in `port-registry.json`. Generated BFF coverage remains stable at 203 paths and 320 operations after regeneration.

Remaining stabilization before implementation is service scaffolding: the seven new services currently have OpenAPI contracts and BFF config entries, but still need Rust crates, Helm values, Dockerfiles, and deployment wiring before they are runtime-ready.

The first implementation-ready BDD slice is now documented in `docs/ACCOUNTING_BDD_FEATURE_BACKLOG.md`. It deliberately starts with existing services: `bank-sync` reconciliation suggestions/actions and `financial-reports` report definitions/drill-down/exports. This lets implementation start against typed contracts without committing to the full reconciliation rule DSL or report expression runtime too early.

The runtime scaffolding gate for the seven new contract-only services is documented in `docs/OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`. Each service must get the standard RERP two-crate service shape, generated docs, Helm values, Dockerfile, and workspace/deployment registration before business logic is added.

Rules-engine design dossiers now live in `docs/accounting/rules-engines/`. Use the README template before implementing engine logic. The first detailed dossiers are `reconciliation-rules-engine.md` and `report-expression-engine.md`; later engines have scaffold dossiers for tax compliance, extraction classification, consolidation eliminations, revenue recognition, lease accounting, and audit controls.

## Current Maturity Read

The accounting suite now has a broader service contract baseline:

- `general-ledger` is the strongest component, with 39 paths, 72 operations, and 82 schemas covering accounts, charts, journals, journal-entry workflows, fiscal periods/years, fiscal positions, payment terms/methods, tax repartition, and core GL reports.
- `accounts-receivable`, `accounts-payable`, `invoice`, `bank-sync`, `asset`, and `budget` have plausible service boundaries and enough typed schemas to build from.
- `financial-reports` has named report generators plus report definition, drill-down, export, and statutory-pack entry points, but still needs deeper expression/schema design.
- `edi` now has profiles, validation profiles, submissions, acknowledgments, retry/status, and error surfaces.
- The newly added services cover statutory tax lifecycle, document extraction, treasury planning, consolidation, recognition accounting, lease accounting, and cross-service audit controls at an initial contract level.

For a world-class accounting deliverable, the next work should turn these contracts into deeper accounting engines: reconciliation rule semantics, report expression models, AR follow-up policy execution, AP payment rails, tax filing adapters, extraction confidence/review models, consolidation eliminations, recognition posting controls, and audit evidence immutability.

## Service Map Addendum

`docs/OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md` converts the comparison into a backlog-style map. Use it before editing accounting specs because it ties each RERP service to Odoo Enterprise module anchors and suggested OpenAPI resources.

Highest-priority targets from that map now that generator coverage is fixed:

- `bank-sync`: `/reconciliation-models`, `/transactions/{id}/suggestions`, `/transactions/{id}/reconcile`, `/transactions/{id}/unreconcile`, write-off/transfer/exchange-difference flows.
- `financial-reports`: `/report-definitions`, report lines/expressions/options, audit-cell/source-line drill-down, PDF/XLSX exports, partner ledger and aged AR/AP reports.
- `accounts-receivable`: follow-up levels/policies, customer statements, partner ledger, follow-up runs.
- `accounts-payable`: payment batches, export files, vendor payment registration, vendor bill extraction, 3-way match.
- `edi`: EDI profiles, submissions, acknowledgments, payment file standards, and localization/statutory surfaces.

## Cross-References

- [`../../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md) — Full analysis.
- [`../../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md) — Actionable service-by-service backlog map.
- [`suite-aware-brrtrouter-wrapper.md`](./suite-aware-brrtrouter-wrapper.md) — BFF generation path guardrails.
- [`service-implementation-and-database-layout.md`](./service-implementation-and-database-layout.md) — Service/entity/database responsibilities.
