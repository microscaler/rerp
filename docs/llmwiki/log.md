# RERP LLM Wiki — Log

Chronological, append-only. New entries go at the bottom with:

`## [YYYY-MM-DD] <op> | <short>`

where `<op>` is `ingest`, `query`, `lint`, or `contribute`.

---

## [2026-04-25] contribute | seed wiki and suite-aware wrapper contract

Seeded the RERP llm-wiki after making the `rerp` BRRTRouter wrapper suite-aware.

- Added core wiki files: [`SCHEMA.md`](./SCHEMA.md), [`README.md`](./README.md), [`index.md`](./index.md), and this log.
- Added [`topics/suite-aware-brrtrouter-wrapper.md`](./topics/suite-aware-brrtrouter-wrapper.md) to preserve the corrected contract: nested RERP layout, Hauliage-style crate split, impl-package build targeting, and suite-local BFF output.
- Source changes summarized from `tooling/src/rerp_tooling/cli/main.py`, `tooling/tests/test_rerp_cli_translations.py`, `tooling/README.md`, and Memory Bank updates.

## [2026-04-25] ingest | Hauliage operating model for RERP services and DB

Expanded the RERP llm-wiki with Hauliage-derived service and database guidance, translated into RERP's suite-nested layout.

- Added [`docs-catalog.md`](./docs-catalog.md) to inventory source docs and sibling references.
- Added [`topics/hauliage-reference-operating-model.md`](./topics/hauliage-reference-operating-model.md) covering Hauliage's two-crate service loop, Register & Overwrite behavior, Tilt build/deploy stages, database lessons, and the "do not copy flat paths" warning.
- Added [`topics/service-implementation-and-database-layout.md`](./topics/service-implementation-and-database-layout.md) covering RERP `openapi/`, `gen/`, `impl`, shared `entities`, Lifeguard migration flow, and shared-kind database config.
- Added [`reconciliation/legacy-root-llmwiki-location.md`](./reconciliation/legacy-root-llmwiki-location.md) to mark root `llmwiki/` as historical source material and `docs/llmwiki/` as canonical per `AGENTS.md`.
- Updated [`index.md`](./index.md) and [`SCHEMA.md`](./SCHEMA.md).

## [2026-04-25] ingest | accounting OpenAPI vs Odoo Enterprise

Benchmarked RERP accounting service OpenAPI specs against Odoo Enterprise accounting modules.

- Added [`../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md) with service-by-service gaps and prioritized remediation.
- Added [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) as the durable wiki summary.
- Key finding: service-level accounting specs are richer than the older audit states, but generated suite artifacts expose stale 46-path CRUD surfaces versus about 111 service-level path templates. Treat this as a BFF generator coverage issue: the generator must aggregate all service OpenAPI docs selected by `openapi/accounting/bff-suite-config.yaml`.
- Odoo-derived gaps: reconciliation depth, configurable report engine, partner ledgers/aging/tax reports, payment rails, document extraction, follow-up automation, and localization/statutory modules.
- Updated [`index.md`](./index.md) and [`docs-catalog.md`](./docs-catalog.md).

## [2026-04-25] ingest | accounting OpenAPI Odoo service map

Continued the robust comparison by converting the broad gap analysis into a service-by-service backlog map.

- Added [`../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md).
- Mapped RERP services (`general-ledger`, `invoice`, `accounts-receivable`, `accounts-payable`, `bank-sync`, `asset`, `budget`, `financial-reports`, `edi`) to Odoo Enterprise anchors.
- Key Odoo anchors included `account_accountant` reconciliation wizard/bank statement logic, `account_reports` report engine and statutory returns, `account_asset` lifecycle, `account_budget` analytic budgets/revisions, `account_followup` dunning policies, `account_batch_payment`/`account_iso20022`/`account_sepa_direct_debit` payment rails, `account_online_synchronization`, and `documents_account`.
- Updated [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) and [`docs-catalog.md`](./docs-catalog.md).

## [2026-04-25] correction | accounting BFF generator coverage

Clarified P0 accounting follow-up language.

- Corrected wording from "fix accounting BFF/top-level spec drift" to "ensure the BFF generator updates from all accounting OpenAPI docs selected by the suite config."
- Updated [`../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md), [`../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md), [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md), and [`index.md`](./index.md).
- The generated artifacts should not be hand-patched; fixes belong in the suite-aware BFF generator/config mapping plus validation that generated operationIds cover selected service specs.

## [2026-04-25] implementation | accounting BFF generator coverage and namespacing

Implemented the P0 accounting BFF generator fix.

- Updated BRRTRouter BFF merge behavior so generated public BFF paths are prefixed with each service `base_path` from `bff-suite-config.yaml`.
- Updated the RERP wrapper so `rerp bff generate-system --suite accounting` uses config-based BFF generation rather than raw directory discovery.
- Added RERP wrapper regression coverage proving all configured service specs contribute operationIds to the generated BFF.
- Regenerated `openapi/accounting/openapi_bff.yaml`; it now contains 111 paths and 185 operations, including distinct `/api/accounts-payable/payments` and `/api/accounts-receivable/payments`.
- Updated [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) and [`topics/suite-aware-brrtrouter-wrapper.md`](./topics/suite-aware-brrtrouter-wrapper.md).

## [2026-04-25] analysis | accounting OpenAPI product maturity

Continued the accounting suite OpenAPI review against the target of a world-class accounting system.

- Current service baseline: 9 services, 111 service path templates, 185 generated BFF operations, and about 225 service schemas.
- `general-ledger` is the strongest component and now looks like the accounting control core rather than a scaffold.
- The largest product gaps are accounting engines, not more CRUD: reconciliation rules/suggestions, configurable report definitions/lines/audit cells, AR follow-up policies, AP payment batches/files, document extraction, EDI submission lifecycle, statutory returns, lock controls, and multi-currency/intercompany flows.
- Noted one OpenAPI contract hygiene issue for future cleanup: `financial-reports` uses `GET` operations with request bodies for several report generation endpoints.

## [2026-04-25] contribute | accounting maturity target and BDD backlog

Captured the target state for a world-class RERP accounting deliverable before expanding OpenAPI contracts.

- Updated [`../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md) with target-state accounting engines and definition-of-done criteria.
- Added [`../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../ACCOUNTING_BDD_FEATURE_BACKLOG.md) with broad BDD feature groups for generated BFF coverage, GL controls, reconciliation, reporting, AR follow-up, AP payments, documents/extraction, assets/budgets, statutory/EDI/localization, and OpenAPI hygiene.
- Updated [`../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md) so resource planning points to the BDD backlog and reflects the implemented BFF generator coverage.
- Updated [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md), [`docs-catalog.md`](./docs-catalog.md), and [`index.md`](./index.md).

## [2026-04-25] contribute | accounting service boundary map

Documented how missing accounting capabilities should map to existing services, new accounting microservices, or localization extensions.

- Updated [`../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md) with service-boundary decisions and initial OpenAPI surfaces for proposed services.
- Updated [`../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md`](../OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md) with the microservice boundary target.
- Updated [`../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../ACCOUNTING_BDD_FEATURE_BACKLOG.md) so broad BDD feature groups reference proposed services including `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, and `lease-accounting`.
- Updated [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) with agent-facing service-boundary guidance.

## [2026-04-25] implementation | accounting missing service OpenAPI build-out

Built the first OpenAPI contract pass for the missing accounting services and enriched existing thin service surfaces.

- Added initial `openapi.yaml` files for `tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, and `audit-controls`.
- Added those services to `openapi/accounting/bff-suite-config.yaml` with namespaced public base paths and service ports 8016-8022.
- Enriched existing accounting specs with lifecycle and engine entry points: bank reconciliation models/suggestions/unreconcile/write-offs/exchange differences, AR follow-up and collection cases, AP payment batches/files/3-way match, report definitions/drill-down/exports, GL lock dates/journal-item reconciliation/revaluation, invoice payment/e-invoice/deferral handoffs, asset lifecycle operations, budget lifecycle and revisions, and EDI profiles/submissions/validation/errors.
- Regenerated `openapi/accounting/openapi_bff.yaml`; it now contains 203 paths and 320 operations.
- Updated [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md), [`index.md`](./index.md), and [`../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md).

## [2026-04-25] implementation | accounting pre-rules contract stabilization

Stabilized the accounting OpenAPI contracts before expanding rules engines.

- Replaced high-risk anonymous request/response bodies with named schemas for reconciliation models/suggestions/actions, AP payment batches/files/3-way match, report definitions/drill-down/exports/statutory packs, AR follow-up/collections/statements, GL lock dates/journal-item reconciliation/revaluation, invoice payment/e-invoice/deferral handoffs, asset models/modifications, and budget revisions.
- Corrected the new service port assignments away from existing `ftebe`, `idam`, `amd`, `marketing`, and `billing` ports. The new accounting services now use `8016-8022`, and `port-registry.json` advances to `8023`.
- Regenerated `openapi/accounting/openapi_bff.yaml`; it remains at 203 paths and 320 operations with the tightened component schemas included.
- Port validation via `rerp ports validate` could not run through the current direct wrapper entry point because it reported `unknown command "ports"`; static checks confirmed the new service OpenAPI server URLs and registry assignments are aligned.

## [2026-04-25] contribute | accounting first implementation slices

Converted the pre-rules-engine plan into implementation-ready acceptance slices.

- Updated [`../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../ACCOUNTING_BDD_FEATURE_BACKLOG.md) with Slice 1A-1D for reconciliation suggestions, reconcile/write-off/exchange-difference/unreconcile actions, report definitions/drill-down/exports, and generated BFF acceptance.
- Updated [`../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md`](../OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md) with a runtime scaffolding gate for the seven new contract-only services.
- Updated [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) so future sessions start with existing-service BDD slices before expanding rules engines.

## [2026-04-25] contribute | accounting rules-engine dossiers

Added contract-first rules-engine design documentation.

- Added [`../accounting/rules-engines/README.md`](../accounting/rules-engines/README.md) with the standard dossier structure and pre-implementation gate.
- Added detailed first-build dossiers for [`reconciliation-rules-engine.md`](../accounting/rules-engines/reconciliation-rules-engine.md) and [`report-expression-engine.md`](../accounting/rules-engines/report-expression-engine.md).
- Added scaffold dossiers for tax compliance, document extraction/classification, consolidation eliminations, revenue recognition, lease accounting, and audit controls.
- Updated [`docs-catalog.md`](./docs-catalog.md) and [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md).

## [2026-04-25] implementation | Tilt accounting service autodetection

Aligned the RERP Tiltfile with the suite-aware microservice discovery model.

- Updated `Tiltfile` so accounting service contracts are discovered from `openapi/accounting/bff-suite-config.yaml` instead of a hardcoded `ACCOUNTING_SERVICES` list.
- Runtime resources are now filtered by scaffold readiness: service OpenAPI spec, `gen` crate, `impl` crate, and Helm values must exist before Tilt stands up that service.
- Service ports come from the suite config, the BFF/legacy fallback comes from `port-registry.json`, and Rust package names are read from each service's `impl/Cargo.toml`.
- `bff-spec-gen` watches all configured accounting specs, including contract-only services, and runs `rerp bff generate-system --suite accounting`.
- Fixed the namespace resource binding and verified the Tiltfile with `tilt alpha tiltfile-result --file Tiltfile`.

## [2026-04-25] analysis | accounting enterprise ERP parity gaps

Captured the second-phase accounting gaps after the current documented target is delivered.

- Added [`../ACCOUNTING_ENTERPRISE_ERP_GAP_ANALYSIS.md`](../ACCOUNTING_ENTERPRISE_ERP_GAP_ANALYSIS.md) comparing the delivered RERP target against SAP S/4HANA Finance, Microsoft Dynamics 365 Finance, NetSuite, Sage Intacct, Workday, and similar finance suites.
- Documented remaining enterprise gaps: multi-ledger/accounting-principle models, close management, consolidation, intercompany, localization factory, tax/legal updates, payment rails, treasury, revenue recognition, lease accounting, controlling, procurement/inventory/project/payroll integrations, GRC, reporting UX, migration tooling, scale proof, ecosystem, and industry variants.
- Updated [`docs-catalog.md`](./docs-catalog.md) and [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) so future planning treats this as a post-target enterprise parity track.
