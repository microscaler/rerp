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
|- Updated [`docs-catalog.md`](./docs-catalog.md) and [`topics/accounting-openapi-odoo-gap.md`](./topics/accounting-openapi-odoo-gap.md) so future planning treats this as a post-target enterprise parity track.

## [2026-04-25] ingest | accounting build plan and codebase audit

Audited the accounting suite against `docs/ACCOUNTING_BUILD_PLAN.md` and confirmed all claims match the current codebase.

- Verified `cargo build --workspace` exits 0 for all 9 impl crates.
- Confirmed 158 total controllers across 9 services, all TODO stubs returning hardcoded data.
- Confirmed Lifeguard entities exist for all 9 services in `entities/src/accounting/` (15 entity files total).
- Confirmed 7 spec-only services lack gen/impl crates, Helm values, Dockerfiles, and K8s definitions.
- Confirmed BFF spec (`openapi_bff.yaml`) has 22,934 lines, 203 paths, 320 operations — matches plan.
- Confirmed no per-service Helm values exist under `helm/rerp-microservice/values/` and no Dockerfiles in impl dirs.
- Marked `ACCOUNTING_BUILD_PLAN.md` status as "verified — ready to execute Phase 1" with verification timestamp.
- Updated [`index.md`](./index.md) to remove `accounting-build-plan.md` from planned pages (it lives in `docs/` not wiki).

## [2026-04-28] contribute | accounting impl controller generator drift

Fixed compilation across 7 accounting services after OpenAPI spec regeneration produced new handler/type names in gen code.

- 39 orphaned impl controller files deleted across 6 services (invoice, accounts-payable, bank-sync, asset, budget, financial-reports) that imported gen handlers no longer present after spec regeneration.
- 41 orphaned `pub mod` entries removed from `controllers/mod.rs` in affected services.
- EDI gen type mismatches corrected: `EdiFormat` → `EdiStandard`, `EdiSubmission` → `EdiSubmissionStatus` type references fixed across gen handlers, controllers, and impl imports.
- Missing `EdiSubmissionStatus` struct added to `gen/src/handlers/types.rs`.
- All 7 services verified passing via `cargo test -p rerp_accounting_* --no-run`.
- Added wiki page [`topics/accounting-service-impl-controller-drift.md`](./topics/accounting-service-impl-controller-drift.md) documenting the problem, fix, and gotchas.
- Updated [`index.md`](./index.md).

## [2026-07-14] contribute | Hauliage accounting dog-food goals

Captured the immediate seven-goal execution overlay for RERP accounting.

- Preserved RERP's destination as a world-class open-source, API-first ERP with
  full ledger, tax, treasury, reconciliation, reporting, and controls.
- Defined Hauliage as the first ordinary SaaS consumer of RERP's public API,
  not a privileged or freight-specific accounting path.
- Broke the tranche into expandable goals covering development readiness,
  active runtime, shared-cluster delivery, Sesame/RLS tenancy, accounting
  foundations, invoice-to-GL, and Hauliage integration.
- Marked the older scaffold-all-services-first build order as superseded while
  retaining its long-term product inventory and research.

## [2026-07-14] contribute | restart checkpoint and Goal 1 activation

Recorded the pushed restart checkpoint and activated the development baseline.

- Confirmed d0412f3 is the origin/main restart checkpoint.
- Marked Goal 1 as active.
- Captured the empty root workspace, blocked entities build, and
  may_minihttp/BRRTRouter dependency mismatch as the first verified blockers.
- Kept Tilt/runtime restoration sequenced behind a reproducible build topology.

## [2026-07-14] contribute | Goal 1 build topology and enum-generation baseline

Completed the first reproducible-development tranche after the restart checkpoint.

- Made `rerp-entities` the explicit root workspace member; its 37-entity build passes.
- Aligned the service workspace with the rustls-capable Microscaler
  `may_minihttp` branch and removed stale crates.io/AWS-LC/Reqwest resolution
  paths from the service graph.
- Made `microservices/Cargo.lock` a committed application lockfile.
- Fixed BRRTRouter's missing OpenAPI string-enum generation and regenerated
  only the affected EDI/BFF type files; both generated crates now compile.
- Verified the invoice implementation compiles.
- Re-ran the broad workspace check and identified the next honest boundary:
  the obsolete BFF implementation has 728 stale example-stub errors and must
  be narrowed under Goal 2, not bulk-regenerated as apparent product behavior.

## [2026-07-14] implementation | first accounting kernel

Started Goal 5 with the first executable accounting behavior.

- Added `rerp-accounting-core` as a root workspace library, not a deployable or
  executor abstraction.
- Implemented decimal customer-invoice calculation, explicit rounding, open
  period enforcement, balanced invoice journals, full cross-period credit
  notes, deterministic idempotency fingerprints and trial-balance derivation.
- Added invariant coverage for calculation, invalid quantities/discounts,
  closed/out-of-period posting, fingerprint drift, credit reversal and tenant
  isolation.
- Recorded ADR 001: invoice and GL persist atomically in the existing invoice
  runtime through one Lifeguard RLS transaction.
- Expanded Goals 5 and 6 with Phase 1 functional/non-functional requirements,
  acceptance criteria, deferrals and required OpenAPI corrections.

## [2026-07-14] implementation | typed accounting foundation and live RLS acceptance

Extended the kernel with its first production-shaped persistence contract.

- Added nine typed `LifeModel + LifeRecord` models under
  `entities/src/accounting/foundation/`; the 37 broad legacy models remain
  explicitly schema-only inventory.
- Added generated base DDL and an app-owned control migration with composite
  tenant foreign keys, amount/state constraints, immutable posted facts and
  forced RLS on every tenant table.
- Vendored the Sesame RLS v1 database contract and documented runtime role
  grants as a deployment responsibility.
- Added and executed a non-superuser PostgreSQL acceptance suite proving
  context-free failure, tenant isolation, tenant-consistent foreign keys,
  balance checks, immutability and rollback of partial posting.
- Repaired stale entity generator imports/output paths and the entity doctest so
  the root workspace and generated migration path build cleanly.

## [2026-07-14] implementation | authenticated invoice-to-ledger runtime

Delivered the first usable public accounting runtime on the existing invoice
process.

- Replaced the broad placeholder invoice surface with four Phase 1 operations:
  post invoice, retrieve invoice, retrieve journal and post a full credit note.
- Used decimal-string commercial facts and removed caller control over tenant,
  legal entity, fiscal period and internal GL accounts.
- Converted validated Sesame claims into the complete Lifeguard session context
  and persisted the document, lines, balanced journal, audit and idempotency
  result in one pinned RLS transaction.
- Added sequential retry/conflict behavior and public responses that omit tenant
  scope and internal account mapping.
- Corrected the typed `rounding_minor_units`/PostgreSQL `INTEGER` mismatch and
  made accounting persistence timestamps explicit.
- Fixed database setup ordering and runtime grants for the complete vendored
  Sesame RLS v1 contract.
- Proved post, retry, conflict, retrieve and full credit against disposable
  PostgreSQL as a non-superuser.
- Added transaction-scoped legal-entity/year sequence locking and proved two
  simultaneous postings receive distinct numbers without exposing rollbacks.
- Kept Goal 6 open for HTTPS generated-client proof and immutable
  rendered-document retrieval.

## [2026-07-15] contribute | Documents generation and rendition boundary

Defined the product contract beyond the delivered basic PDF renderer.

- Selected externalized HTML/CSS template bundles with one constrained,
  Jinja-compatible expression contract as the MVP direction.
- Defined immutable posted render snapshots, published template versions,
  deterministic original materialization, explicit copy artifacts and lineage.
- Added functional and non-functional requirements covering template APIs,
  Unicode, pagination, sandboxing, RLS, idempotency, object storage,
  observability, recovery and shared-cluster performance.
- Kept qualified electronic seals, timestamps, PAdES long-term validation and
  visible trust panels as a provider-neutral post-MVP phase.
- Located the permanent capability in `documents/render`: source suites own
  business facts and immutable snapshots; Documents owns templates, rendering,
  artifacts, copies, and future trust services.
- Added ADR 002, the first API contract and a generated Rust component scaffold
  under the suite-nested Documents layout.
- Cross-referenced the PRD from Documents, Goal 6, the roadmap and the LLM wiki
  catalog.
## [2026-07-15] contribute | Canonical suite and microservice ownership contract

Reconciled RERP's contribution and agent guidance with Hauliage's complete
service anatomy while preserving RERP's mandatory suite boundary.

- Defined suite-owned `core`, foundation `entities`, migrations, SQL, scripts,
  and cross-service tests as optional installation boundaries.
- Defined every HTTP service's generated `gen/` and user-owned `impl/` anatomy,
  including controllers, application services, models, validators, config,
  seeds, tests, documentation, registry build, and executable composition.
- Established one effective table/view → one `LifeModel` owner → one migration
  provider, with service ownership as the default and suite entities reserved
  for genuine foundations.
- Required the single top-level migrator to use explicit `(suite, service)`
  providers and produce/apply only suite-local migration and seed orders.
- Flagged current Accounting duplicate models and flat migrator paths as drift
  to reconcile rather than patterns to extend.
## [2026-07-15] contribute | Hauliage Accounting service readiness plan

Added the executable work-through plan for the RERP services required by
Hauliage's first dogfood tranche.

- Sequenced structural/entity/migrator reconciliation before domain expansion.
- Added service-level tasks and acceptance criteria for General Ledger,
  Invoice, AR, AP, banking/reconciliation, financial reporting, Documents
  Render, Accounting BFF, and generated-client dogfood proof.
- Defined connector-neutral banking delivery and the official documentation,
  sandbox, security, and credentials gate for a first Stanbic or Standard
  Chartered adapter in Zimbabwe/South Africa.
- Added order-to-cash, procure-to-pay, retry/failure, tenant-isolation, and
  cross-ledger reconciliation acceptance scenarios.
- Recorded cross-cutting non-functional requirements and explicit deferrals so
  broad generated Accounting surface is not mistaken for delivered behavior.

## [2026-07-15] document | Accounting entity relationship inventory

Enriched the Accounting suite README with Mermaid relationship diagrams
derived from the reconciled `LifeModel` registries and delivered foundation
migrations.

- Covered all 47 effective Accounting entities across the foundation and nine
  service providers.
- Distinguished the 10-table delivered foundation schema from 37 registry
  models whose migrations are not yet approved for activation.
- Drew only declared or migration-enforced foreign keys and listed UUID-shaped
  logical references that currently lack database constraints.
- Exposed missing `customers` and `vendors` entity ownership and the unresolved
  semantic boundary between legacy General Ledger tables and the authoritative
  `accounting_*` posting foundation.

## [2026-07-15] contribute | Documentation authority and supersession governance

- Added `docs/README.md` as the human current-authority index, explicitly
  separating normative product intent from delivered runtime truth.
- Added controlled lifecycle, metadata, retirement, external-source, and ADR
  supersession rules in `docs/DOCUMENTATION_GOVERNANCE.md`.
- Added `docs/authority.json`, the ADR register, and a practical ADR template;
  normalized ADR 001 and ADR 002 to `ACCEPTED` with stable scopes.
- Added and tested a stdlib-only governance validator covering authority
  clashes, paths, statuses, ADR registration, and reciprocal supersession.
- Wired validation into the existing tooling CI job and documented the policy
  in `CONTRIBUTING.md` and this wiki.

## [2026-07-15] curate | First governed documentation history tranche

- Created `docs/history/` with bootstrap, implementation-snapshot,
  architecture-snapshot, audit, and superseded-plan classifications.
- Archived January bootstrap/entity migration reports, early OpenAPI/BFF
  completion reports, the retired service mapping, the service-matrix audit,
  and the superseded Accounting build-order plan.
- Added historical banners and repaired current roadmaps, contributor guidance,
  entity documentation, catalogs, and wiki references.
- Left active PRDs, ADRs, product gap analyses, and the Hauliage dogfood
  readiness plan in their current authoritative locations.

## [2026-07-16] fix | Restore canonical Invoice runtime and converge the ledger schema

- Promoted the five-route Phase 1 Invoice contract to canonical
  `openapi/accounting/invoice/openapi.yaml` and regenerated disposable server
  artifacts deterministically.
- Restored dependencies lost during the suite-layout move; Accounting core,
  Invoice, and migrator tests compile and pass again.
- Retired the five undelivered General Ledger predecessor models. The
  RLS-protected `accounting_accounts`, `accounting_journal_entries`, and
  `accounting_journal_lines` foundation is now the only ledger schema.
- Added migrator regression coverage preventing the parallel-ledger table names
  from returning; Accounting validation now reports 42 uniquely owned tables.
- Kept the broad General Ledger HTTP surface classified as scaffold pending a
  narrow, authenticated implementation over the foundation records.
- Confirmed the targeted delivered crates pass while the full workspace still
  fails in inactive generated-era AP, banking, GL, EDI, and BFF implementation
  stubs; those are to be removed by service narrowing, not patched into fake
  success handlers.

## [2026-07-16] implement | Deliver narrow General Ledger inspection runtime

- Replaced the 72-operation generated-era General Ledger contract with four
  canonical read operations for accounts, fiscal periods, immutable journal
  retrieval, and as-of-date single-currency trial balance.
- Removed the inactive impl-controller tree and registered only protected,
  human-owned controllers; generated mock handlers are not part of the runtime.
- Added Sesame-to-Lifeguard identity handling, exact
  `accounting:ledger:read` authorization, transaction-scoped RLS, and explicit
  tenant/legal-entity query predicates.
- Derived trial balance from authoritative immutable lines and added integrity
  checks preventing invalid individual journals from cancelling out only at
  aggregate level.
- Confirmed OpenAPI lint, deterministic regeneration, and nine focused runtime
  tests. Mutation, posting/reversal, opening-balance, dimension, and scale gates
  remain in WP1.

## [2026-07-16] operate | Make RERP database bootstrap HA- and SOPS-native

- Replaced the Tilt-owned plaintext database Secret with the RERP-owned
  `dev/rerp/accounting` SOPS profile and separate database/object-store credentials.
- Adopted the SAM Flux ownership model: shared-cluster composition sources the
  profile from RERP, Flux decrypts/reconciles it, and Tilt only gates on the
  resulting Ready Kustomization during workload-ownership migration.
- Made Tilt validate the profile, initialize the database automatically, and
  gate Accounting workloads on successful role, schema, migration, seed, and
  grant setup.
- Updated the initializer to discover the elected Bitnami PostgreSQL HA primary
  and verify that Pgpool's SOPS custom-user credential matches the application
  Secret before applying SQL, then prove the loaded Pgpool contract with an
  authenticated query before workloads may start.
- Made the two hand-authored foundation control migrations safely re-runnable
  using transactional completion markers.

## [2026-07-16] operate | Complete the Accounting Flux ownership split

- Split the RERP-owned dev profile into runtime configuration, gated
  database/object-store bootstrap, and delivered-service reconciliation.
- Added an in-cluster, kubectl-free database initializer image which verifies
  the Pgpool application login before the foundation Kustomization can become
  Ready.
- Added idempotent MinIO bucket/user/policy provisioning beside the
  platform-owned administrator Secret without copying platform credentials to
  the RERP namespace.
- Added Flux Helm releases for only the honestly delivered General Ledger and
  Invoice services; the broader generated Accounting surface remains absent.
- Narrowed Tilt to code/image work and clean registry repositories. It now
  publishes monotonic `dev-<nanoseconds>` tags for Flux discovery and applies
  no Kubernetes resources.
- Kept Git-writing image automation credential-gated; ImageRepository and
  ImagePolicy discovery may operate before a scoped RERP deploy key exists.
- Added a manual Tilt acceptance cycle derived from the useful deployment-watch
  behavior in the supplied Skaffold shell helper. The Python checker passively
  verifies Flux gates, bootstrap completion, image convergence, rollout, and
  HTTP health without applying or reconciling resources.

## [2026-07-16] operate | Separate database bootstrap from application migrations

- Restricted the Flux database Job to Pgpool credential validation, role,
  database, schema, default privileges, and application-login verification.
- Removed Accounting migration and RLS content from the bootstrap image so a
  Flux reconciliation cannot mutate application schema.
- Added the manual Tilt `accounting-apply-migrations` cycle for ordered
  migrations, the vendored Sesame RLS contract, seeds, and post-migration
  grants during rapid development.

## [2026-07-16] operate | Move the complete Accounting component catalog to Flux ownership

- Added Helm runtime descriptors for all seventeen Accounting source
  components, while keeping Tilt image publication restricted to the two
  functionally delivered services.
- Added a separate Flux catalog component containing fifteen explicitly
  suspended, `scaffold-only` HelmRelease declarations. This transfers lifecycle
  and pruning ownership without deploying generated example APIs.
- Recorded compile evidence: eight suspended components compile but have no
  real behavior tests; seven have generated contract/controller drift and do
  not compile. Compilation is not an activation criterion.
- Extended passive deployment acceptance to prove the catalog Kustomization is
  reconciled, every catalog release remains suspended, and no corresponding
  Deployment exists.
- Bumped the immutable database-bootstrap pod template after its initial
  one-shot failure; the same image subsequently completed the full idempotent
  migration sequence successfully in a diagnostic pod.

## [2026-07-16] integrate | Consume standalone Sesame-IDAM client

- Added the sibling `microscaler/sesame-idam-client` development dependency to
  the Accounting workspace.
- Replaced duplicate Invoice and General Ledger claim parsers with the shared
  strict parser while retaining RERP's Lifeguard `SessionContext` mapping.
- Verified both binary identity test suites on ms02 (three tests each).

## [2026-10-27] contribute | accounting crates back in the build; bff blocked on BFF tooling

Brought the accounting suite back into `default-members`
([`microservices/Cargo.toml`](../../microservices/Cargo.toml)) after
regenerating every regenerable `gen/` crate with BRRTRouter 0e8f307c
(PR #530: allOf merge + body-less 2xx fix) and refreshing the `impl/`
handler stubs (`rerp gen stubs --force`/`--sync`; all failing controllers
were verified pure generated stubs). All accounting gen and impl crates
now compile except `accounting/bff/impl`, which stays out of
`default-members`.

- Known drift: `openapi/accounting/openapi_bff.yaml` references
  `#/components/responses/*` (copied from general-ledger/invoice member
  specs) but the BFF generator never carries `components.responses` into
  the merged spec (`brrtrouter_tooling/bff/merge.py::_merge_components`
  merges only schemas and parameters). The fix-rev's fail-closed `$ref`
  validation therefore rejects the BFF spec, so bff's `gen/` (252 stale
  handler modules vs 155 current spec ops) and its 116 impl stubs cannot
  be regenerated — 671 errors remain. Unblock path: a BRRTRouter tooling
  change merging member `components.responses`.
- Also blocked for regeneration (same fail-closed validation, member
  specs not affected): consolidation, documents-extraction,
  revenue-recognition, tax-compliance, treasury specs contain dangling
  `#/components/responses/{Unauthorized,ValidationError,Conflict,...}`
  refs with no `components.responses` section; their gen/ crates are
  stale but still compile.
- `rerp gen stubs` now emits `impl/src/impl_registry.rs` (ADR-0001 layer-2
  registry) for budget, asset, edi, bank-sync, accounts-payable,
  audit-controls, lease-accounting, financial-reports; those are
  generated but not yet wired into `main.rs` (still `register_from_spec`).

## [2026-10-28] contribute | accounting suite fully green, incl. bff

With BRRTRouter 9768e35 (PR #533 merges member `components.responses`
and the other component maps into BFF specs, service-prefixed and
ref-rewritten) the whole accounting suite — including
`accounting/bff/impl` — compiles and is back in `default-members`
([`microservices/Cargo.toml`](../../microservices/Cargo.toml)).

- Spec fixes (human-reviewed rule change): the five member specs
  consolidation, documents-extraction, revenue-recognition,
  tax-compliance, treasury gained the `components.responses` entries
  their operations `$ref`'d but never defined (modelled on
  general-ledger/invoice; `ValidationError` on inventory/stock). No
  payload schema, operation, operationId, endpoint or status code
  changed.
- `openapi/accounting/openapi_bff.yaml` regenerated with
  `rerp bff generate-system --suite accounting`: `components.responses`
  now carries the 12 prefixed GeneralLedger*/Invoice* responses;
  `bff/merge.py::merge_named_components` + `prefix_named_components`
  are the tooling behind it. All 155 operations are proxy-only
  (`x-service` + `x-brrtrouter-downstream-path`), so `generate-stubs`
  skips them ("Skipped proxy-only stub",
  `brrtrouter/src/generator/project/generate.rs`) and the 115 stale
  pure-TODO impl controller files were removed; `bff/impl` keeps an
  empty `controllers/mod.rs` and generated `impl_registry.rs` (0
  controllers). `main.rs` dispatches gen mock handlers via
  `register_from_spec` only — unchanged, still the open wiring
  question for services that gain real impl controllers.
- Verification: `cargo check --workspace --all-targets` 0 errors,
  `cargo test --workspace --lib` 0 failures (21 tests passed),
  `cargo fmt --all --check` clean, `just qa` 29 passed.

## [2026-10-28] maintain | rebase hermes/accounting-build onto main 818c6cb6

- Operator directed a rebase onto `origin/main` (`818c6cb6`: lifeguard
  `869b2c5`, may_postgres from the fork's `integration/microscaler-fork`).
  24 accounting commits replayed; the two main pin-bump commits deduplicated
  automatically. Conflicts were only the `brrtrouter`/`sesame-idam-client`
  pins in `microservices/Cargo.toml` + `Cargo.lock`, resolved toward main's
  newer `9768e35` (the branch's `0e8f307c` is its ancestor; the branch tip
  already agreed with main).
- Pitfall found: the replayed spec-era commit had silently downgraded
  `tooling/pyproject.toml`'s brrtrouter-tooling pin below main's
  (`9bcb2db0`); restored from main in the lock-refresh commit (28644e76).
  An earlier branch copy was also clobbered back onto the pre-rebase tip by a
  stale `git pull --rebase -X ours` — history rebuilt from the reflog chain.
- Verified on the new base: `cargo check --workspace --all-targets --locked`
  0 errors, `cargo test --workspace --lib --no-fail-fast` 21 passed / 0
  failures, `cargo fmt --all --check` clean, `just qa` 29 passed.

## [2026-10-28] contribute | accounting scaffold runtimes made honest (Epic A1/A2)

- `microservices/accounting/BUILDOUT_ANALYSIS.md` added at the suite root:
  verified current-state delta vs the service-readiness plan plus an
  11-epic story backlog for the build-out.
- All 14 scaffold-only accounting services dispatched generated mock
  handlers (`registry::register_from_spec`), so their HTTP surface served
  fabricated example payloads while impl controllers were dead code. Wired
  every `main.rs` to `impl_registry::register_impl` (ADR 0001 Layer 2, the
  `invoice`/`general-ledger` pattern), rewrote 173 fabricated controller
  stubs to typed 501s via `http_support::not_implemented`, and removed the
  dead blanket imports. `bff` untouched — its `register_from_spec` is proxy
  routing. Note: wiring `mod impl_registry;` makes rustfmt see
  `impl_registry.rs` for the first time; the generator's whitespace-era
  drift was formatted in the same commit.
- `tooling/tests/test_accounting_runtime_honesty.py` (NFR-DOG-013, in
  `just qa`): scaffold mains must register impl controllers only, and every
  scaffold controller must use the honest 501 or opt out via the
  `// BRRTRouter: user-owned` sentinel when a work package delivers it.
- Verified: `cargo check --workspace --all-targets` 0 errors, lib tests
  21 passed, `cargo fmt --all --check` clean, `just qa` 57 passed
  (29 baseline + 28 new). Shipped via PR #111.

## [2026-10-28] contribute | accounts-receivable contract narrowed to the WP3 first vertical (Epic B1)

- `openapi/accounting/accounts-receivable/openapi.yaml`: 24 research-era ops
  narrowed to 9 (open items, customer payments, payment applications, aging
  summary, statement send). Collections/follow-ups/promises/aging-snapshot
  CRUD/auto-apply/receipt-update-delete dropped; 22 orphan schemas pruned;
  all retained money fields converted from double to the invoice
  decimal-string pattern, examples quoted. Receipt mutations deferred to
  explicit reversal ops in the WP3 impl slice (immutability constraint).
- Regeneration via BRRTRouter 9768e35 (hermes-lab sibling checkout; the
  microscaler/BRRTRouter sibling is at c9f0884 and must not be used for rerp
  gen until re-pinned): full-suite regen touched only accounts-receivable.
- Impl: 9 stubs -> honest 501s, 15 orphan controllers removed, registry
  re-synced. Gates: check 0 errors, 21 lib tests, fmt, `just qa` 57,
  `rerp openapi validate` 121 specs valid.

## [2026-10-28] contribute | accounts-payable contract narrowed to the intake-ledger vertical (Epic B2)

- `openapi/accounting/accounts-payable/openapi.yaml`: 20 ops -> 7 (vendor
  invoices list/create/get, vendor payments list/create/get, invoice
  payment registration). Dropped approvals, commitments, cash-flow
  forecast, payment batches (+approve/export), three-way match (needs
  procurement POs), receipt update/delete. Money + quantity -> decimal
  strings; ApprovalRecord.threshold_met double -> boolean; 16 orphan
  schemas pruned.
- Gen regen (9768e35) touched only accounts-payable; impl: 7 honest 501s,
  13 orphan controllers removed, registry re-synced.
- Gates: check 0 errors, 21 lib tests, fmt, `just qa` 57, specs valid 121.

## [2026-10-28] contribute | bank-sync contract narrowed to the reconciliation vertical (Epic B3)

- `openapi/accounting/bank-sync/openapi.yaml`: 25 ops -> 15 (registry
  CRUD minus delete, provider-idempotent statement ingestion, transaction
  match/auto-match/reconcile/unreconcile, reconciliation lifecycle).
  Dropped trigger-sync, suggestions, standalone write-off/exchange-
  difference, both reports (B4 owns reporting), reconciliation models.
  Added required external_reference as the provider-idempotency anchor;
  balances/amounts -> decimal strings (write_off_amount nullable); 12
  orphan schemas pruned.
- Gen regen (9768e35) touched only bank-sync; impl: 15 honest 501s, 9
  orphan controllers removed, registry re-synced.
- Gates: check 0 errors, 21 lib tests, fmt, `just qa` 57, specs valid 121.

## [2026-10-28] contribute | financial-reports narrowed to core statements (Epic B4)

- `openapi/accounting/financial-reports/openapi.yaml`: 18 ops -> 4
  (trial balance, income statement, balance sheet, general-ledger
  detail). Custom-report engine, cash-flow statement, exports and
  statutory packs deferred. currency_code + ISO-4217 pattern; all report
  numerics -> decimal strings; 19 orphan schemas pruned.
- Pitfall found: report GET ops embed Create*Request schemas for their
  query structs — adding duplicate query parameters makes the generator
  emit duplicate Request fields (E0124). Scoping lives in the request
  schemas; do not re-declare as parameters.
- Gen regen touched only financial-reports; impl: 4 honest 501s, 14
  orphans removed, registry re-synced.
- Gates: check 0, 21 lib tests, fmt, `just qa` 57, specs valid 121.

## [2026-10-28] contribute | decimal-money contract gate closes Epic B (B5)

- `tooling/tests/test_accounting_money_types.py`: every accounting
  contract classified STRICT (float-free) or DEFERRED (shrink-only
  list); unclassified fails. Epic B is now complete (B1-B5).
- Gate caught 4 inline floats B3's component-scope sweep missed in
  bank-sync (query filters + auto-match tolerances) -> decimal strings,
  gen regenerated.
- Pitfall: rerp pre-commit runs `just qa` and conventional-commit lint
  rejects `test(...)` type; the tooling gate commit rides `fix(...)`.
  Commit hygiene note: files staged from an aborted round got absorbed
  by the next commit - always `git reset HEAD -- .` + fresh staging.

## [2026-10-28] contribute | registry-wiring gate closes C1 (rescoped)

- `tooling/tests/test_accounting_registry_wiring.py`: spec operationIds ==
  impl_registry arms == controller files for all 16 impl services (bff
  excluded as proxy). Epic B + C1 done; next: Epic D1 (chart of
  accounts lifecycle) pending user go-ahead; C2-C4 untouched.

## [2026-10-28] contribute | first GL write path: create_ledger_account (Epic D1a)

- POST /v1/accounts with contract-level control-account protection (no
  control_role in request schema), body idempotency (replay 200 /
  payload-conflict 409 idempotency_conflict / duplicate code 409
  duplicate_account_code via the DB unique constraint), audit fact
  ACCOUNT_CREATED, exact accounting:ledger:write gate on top of read.
- Foundation entities/migrations sufficed - zero SQL changes needed.
- Live acceptance (#[ignore]) mirrors invoice pattern incl. replay and
  both conflict kinds. Gates: fmt, workspace check 0, GL 14 pass + 1
  ignored, just qa 74, 121 specs valid.

## [2026-10-29] contribute | account mutation lifecycle closes Epic D1 (D1a+D1b)

- D1a (#118): POST /v1/accounts. D1b: PATCH /v1/accounts/{id} — control
  protection (403 for control_role accounts), posted-field freeze
  (journal-line check -> 409 posted_account_immutable), update fingerprint
  binds operation+account+fields (no create/update replay collision),
  Lifeguard Record dirty-field update, ACCOUNT_UPDATED audit fact.
- GL now at 6 ops, all wiring/honesty/money gates green; qa 106, GL tests
  16 pass + 1 ignored live. Next: D2 fiscal-period state machine.

## [2026-10-29] fix | PATCH /v1/accounts mount bug from D1b + repo-wide gate

- D1b mounted PATCH under /v1/accounts with a phantom 'id' path param;
  route generated as PATCH /v1/accounts, parameter never binds. Schema
  validation does not check parameter/template agreement — CI missed it.
- Fixed contract + regenerated; new tooling gate
  test_openapi_path_parameters.py asserts every in:path param appears as
  {name} in its template across all 121 specs (GL was the only violation).
- qa now 227. Lesson: hand-patched OpenAPI YAML must round-trip through a
  parameter-declaration check, not just schema validation.

## [2026-10-30] contribute | fiscal-period state machine closes Epic D2

- GL now 8 ops: create_fiscal_period + transition_fiscal_period. Closed
  transition matrix; REOPENED requires explicit reason; overlap 409 at
  creation; fingerprints bind operation markers so create/transition/
  account replay namespaces can never collide.
- Corrected invoice resolve_open_period: accepted ONLY state='OPEN', so a
  properly REOPENED period silently rejected postings. Now
  IN (OPEN, REOPENED) — posting-allowed is exactly the D2 contract.
- D2 spec work also surfaced (and #120 fixed) the D1b mount bug; the
  path-parameter gate caught nothing new here because the transition op's
  parameters were authored correctly after the idem_param mixup was
  removed.

## [2026-10-30] contribute | manual journal post + reversal closes Epic D3

- GL now 10 ops. journals.rs: post_journal_entry / reverse_journal_entry.
  Balance + active-account + currency-match + OPEN/REOPENED-period gates;
  strict decimal-string amounts; mirrored NEW reversal (originals
  immutable), linked via posted_documents.original_document_id;
  double-reversal 409.
- Advisory-lock numbering shared with invoice (same FNV key space) —
  first cross-service use of invoice's scheme; JE numbering is one
  sequence per entity/year across manual and invoice postings.
- Nil-UUID counterparty sentinel on manual posted_documents (customer_id
  NOT NULL, no FK — documented, not abused).

## [2026-10-30] contribute | opening balances (D4a); D4b deferred explicitly

- GL now 11 ops. opening.rs: POST /v1/opening-balances — must be the
  FIRST posting in the period covering Jan 1 of the year (409
  period_has_postings) and once per period (409
  opening_balances_already_set); OPB numbering + OPENING_BALANCES_SET
  audit via the now-shared journals machinery (persist_posting gained
  an explicit audit_action parameter).
- Retained-earnings close deliberately NOT bundled here: D4b slice.

## [2026-10-30] contribute | retained-earnings year close (D4b); Epic D4 complete

- GL now 12 ops. yearclose.rs: POST /v1/year-closes nets REVENUE/EXPENSE
  accounts of the calendar-year periods into an EQUITY RE account
  (YCL-{year}-{n}, FISCAL_YEAR_CLOSED audit). Closing entries are
  excluded from their own aggregation; year closes once
  (year_already_closed); no P&L activity → nothing_to_close; Dec-31
  period must be OPEN/REOPENED. Lifeguard range ops are gte/lte/
  between — there is no ge/le.

## [2026-10-30] contribute | trial balance dimensions (D5); Epic D complete

- /v1/trial-balance now takes fiscal_period_id, account_type and
  group_by_account_type. Integrity pre-validation runs on the FULL
  selected source BEFORE dimension filters; 'balanced' = source set,
  totals = reported slice. Deref-generics don't work for owned-slice
  callers (Model: Deref unsatisfied) — slices are cloned before the
  pure derive helpers. Epic D (General Ledger) now complete.

## [2026-10-31] contribute | durable render outbox (E1)

- Invoice posting atomically enqueues accounting_render_outbox (0004,
  RLS) with the immutable render model; NFR-DOG-010. Materialiser:
  claim (attempt+, IN_PROGRESS blocks), render from model, failure ->
  PENDING + truncated error; DONE in the artifact transaction. Legacy
  documents render from DB and backfill DONE. GET /v1/render-outbox =
  operator queue surface. Claim race is benign (content-addressed
  render + artifact unique).

## [2026-10-31] contribute | partial credit notes (E2a)

- credit-notes POST gained optional explicit `lines` (original_line_number
  + quantity). Core partial_credit_customer_invoice runs the same
  pipeline, flips journal sides for selected quantities, preserves
  original line numbers, binds selection into the fingerprint.
  Remaining = original qty - fold(prior credit note document lines);
  over-credit incl. merged duplicate rows -> 422 OverCredit.
  Full credit blocks further credits automatically.
- E2b (debit notes) deferred: needs AccountingDocumentType + DB check
  widening; recorded in BUILDOUT_ANALYSIS.

## [2026-10-31] contribute | customer debit notes (E2b)

- New posted-document type CUSTOMER_DEBIT_NOTE (enum + 0005 widens the
  0002 type CHECK; acceptance SQL proves debit-note insert + unknown-type
  rejection). POST /v1/customer-invoices/{id}/debit-notes: explicit line
  selections priced from the original (unit price/discount/tax inherited),
  invoice-direction journal (receivable DEBITED — debit ADDS to what the
  customer owes), original immutable, line numbers preserved, DBN- number
  series under the same advisory sequence lock, render-outbox queued.
- Deliberately NO quantity cap (additional charges are not bounded by the
  original quantities, unlike credits); selections required.
- Debit fingerprint carries a document-type discriminator: same
  key/original/selection as a credit note can never fingerprint-collide,
  so the idempotency layer cannot replay the wrong document kind.
- Consistency fix: partial credits now record CREDIT_NOTE_POSTED (they
  previously logged INVOICE_POSTED while full credits logged the note).

## [2026-10-31] correction | accounts-receivable impl attribution (PR #129 review)

- PR #129 rewrote the 2026-10-28 accounts-receivable entry in place, breaking
  the append-only rule; the bullet is restored verbatim by this change. The
  correction it intended to record: the 9 retained stubs already returned
  honest 501s from #111, so the #112 gen-sync commit removed orphan
  controllers rather than authoring 501s.

## [2026-10-31] process | PR review comments are a merge gate (PR #129 review)

- Gate: every review comment on a PR (inline + issue + review-body surfaces)
  must be fixed or answered on that PR before merge; CI green is necessary,
  not sufficient. Applies to review-backlog PRs themselves — #129 merged with
  two unresolved findings on itself (vacuous DEFERRED baseline copy; log.md
  rewritten in place), both fixed by this change.
- Codified: `CONTRIBUTING.md` PR checklist, new topic
  `docs/llmwiki/topics/pr-review-merge-gate.md` (registered in index), and
  prerequisite 8 in `microservices/accounting/BUILDOUT_ANALYSIS.md`.
- Gate fix: `DEFERRED_BASELINE` in `tooling/tests/test_accounting_money_types.py`
  is now a literal 9-name snapshot instead of `frozenset(DEFERRED)`, so a
  STRICT→DEFERRED relabelling actually fails `test_deferred_set_never_grows`.

## [2026-11-01] delivered | Epic E3 live-HTTPS consumer proof (goal 6 close)

- `microservices/accounting/invoice/impl/tests/e2e_https.rs` (ignored,
  disposable-DB like the tests/sql acceptance harness): boots the REAL
  invoice binary with config-registered `JwksBearerProvider` (HS256
  `at+jwt`, mock JWKS endpoint), terminates TLS at a test-only rustls/rcgen
  edge, and drives the generated contract types over `may_minihttp`'s
  rustls client in a coroutine: post 201 (120.00, balanced journal),
  replay 200 same doc, changed-payload 409, typed retrieve, full credit
  note 201, additional-receivable debit note 201 (DBN-, 240.00), original
  immutable. `RERP_E2E_BIN` overrides the binary path.
- Contract bug fixed en route: `nullable: true` is an OAS 3.0 keyword the
  response validator ignores, so genuinely-null response fields failed
  validation with 500 (POST /v1/customer-invoices had never passed
  response validation over HTTP). invoice + general-ledger canonical specs
  converted (10 sites) to the `type: [x, 'null']` convention already used
  by asset/budget/edi; regen is doc-only + zero impl churn.
- Host finding for the platform backlog: with the may fork (2156e04) +
  generator 0.8.11 on rustc 1.99 Linux, DEBUG builds of rerp services abort
  via generator's coroutine stack guard on some request paths (repro: any
  POST after the first; persists at BRRTR_STACK_SIZE=2MiB; release builds
  clean; same-stack isolate crates pass).
- Consumer-side keep-alive on this may_minihttp build resets second
  requests; harness sends `Connection: close` (matches what worked in the
  live manual curl session).

## [2026-11-01] correction | E3 entry: nullability landed as a full 3.1.0 migration

- Superseding detail for today's E3 entry (PR #132, merged b64cadda): the
  two documents were the repo's last `openapi: 3.0.3` declarations (123
  specs already on 3.1.0, and BRRTRouter is built for 3.1 semantics —
  that is why `nullable` was ignored). The fix is not a convention
  transplant into a 3.0.3 document (type arrays are invalid there); both
  documents now declare 3.1.0, `tax` is an `anyOf [$ref, null]` union
  (codegen maps it to the same Option<serde_json::Value>, zero impl
  churn), the last dead `nullable` repo-wide (bank-sync
  write_off_amount) is gone, the BFF aggregate + gen crate were
  regenerated, and CodeRabbit's three review passes' findings were all
  resolved before merge. E2E green fresh-database and same-database rerun.

## [2026-11-01] delivered | F1a AR overlay entities + migration 0006 + live RLS

- Epic F starts: the four AR scaffold models (customer_invoices,
  ar_payments, ar_payment_applications, ar_agings) were re-anchored —
  phantom invoices/customers FKs replaced by a tenant-scoped composite FK
  onto accounting_posted_documents, plain customer_id, tenant/legal-entity
  columns, NUMERIC(19,6) money matching the posted facts; derived drift
  columns (days_overdue, aging_bucket, credit_limit/used,
  collection_status) dropped (aging derives at read; F3 snapshots).
- migrations/foundation/0006_ar_overlay.sql: 4 tables, FORCE RLS tenant
  policies with the policy-as-completion-marker guard (re-apply skip
  verified live), status/terms CHECKs, composite-scope FKs. Appended to
  apply_order.txt; setup-db grant_rerp_dml covers new tables.
- Live acceptance (impl/tests/ar_overlay_rls.rs, ignored): all four
  tables cross-tenant-invisible through rls_set_session (transaction-local
  GUC — reads must run inside BEGIN), no-session sees nothing and cannot
  write, cross-tenant document anchor FK-rejected; run-unique fixture ids
  (rerun-safe, passed twice).
- Slice plan recorded at docs/EPIC_F1_ACCOUNTS_RECEIVABLE_PLAN.md;
  BUILDOUT Epic F links it. F1b = open items over posted facts.

## [2026-11-01] delivered | F1b open items derived from posted facts (2 of 9 AR ops live)

- accounts-receivable runtime went from scaffold-only to real for
  list_open_items/get_open_item: open items derived at read from
  accounting_posted_documents (invoices + debit notes netted by credit
  notes linked via original_document_id, minus F2 allocations and overlay
  write-offs); aging bucket, days_overdue and payment terms computed from
  due_date/document_date at read; OpenItem id is the posted document id.
- Runtime gained the invoice-shaped plumbing: environment-configured
  Lifeguard pool with fail-fast init, sesame claims -> rls_set_session
  context, contract-shaped error bodies (ErrorResponse enum codes —
  custom strings get validator-wrapped into 500s).
- Spec: x-brrtrouter-impl true on list_open_items; every AR example now
  enum-valid; BFF aggregate + gen regenerated.
- Proof: 8 unit tests, live-DB derivation acceptance (netting, PARTIAL,
  filters, payment_history, cross-tenant NotFound), live HTTP smoke on
  the release binary (200/404 NOT_FOUND/400 enum codes/search/limit).
- Honesty gates pass for the remaining 7 scaffold ops (F2/F3 territory).

## [2026-11-01] correction | F1b log entry overstated example cleanup

- The F1b entry claimed every AR example is enum-valid. True only for the
  open-item/aging examples touched by F1b; the repo-wide placeholder
  examples (`example: 1000.0`, `delivery_method: bank_transfer`) still sit
  in the AR spec plus nine other scaffold specs. Tracked as a dedicated
  sweep in docs/EPIC_F1_ACCOUNTS_RECEIVABLE_PLAN.md (surfaced by the
  PR #135 review).

## [2026-11-02] delivered | F1c overlay projection + AR control-account agreement

- projection.rs lands the rebuildable cache: project_settlement_state
  writes one customer_invoices row per posted customer document with
  values exactly equal to compose_item (cached original_amount is the
  CREDIT-NETTED original like the read path; gross would have disagreed).
  Idempotent; manual WRITTEN_OFF / DISPUTED rows preserved verbatim.
  ar_control_drift compares ACCOUNTS_RECEIVABLE journal movement against
  signed document totals and degrades to None over read bounds.
- Live acceptance: create 2 / refresh 1 / preserve 1 with drift 0; stored
  dump agrees value-for-value with list_open_items; from-scratch rebuild
  byte-identical; stray control-account DEBIT reads drift +7; tenant B
  invisible. Rerun-safe. Runtime consumer arrives with F3 (documented
  allow(dead_code) until then).
- Plan status -> F2 next (receipts + allocations). PR #141.

## [2026-11-02] fixed | setup-db.sh must grant the RLS contract to the app role

- sql/rls/v1/install.sql REVOKEs EXECUTE from PUBLIC on every
  sesame_*/rls_* function; grant_rerp_dml only covered tables and
  sequences, so a DB provisioned purely by setup-db.sh left the rerp
  role unable to call rls_set_session (live DB suites die with
  "permission denied for function sesame_current_tenant_id"). Cluster
  provisioning had been covering the gap out-of-band.
- grant_rerp_dml now loops GRANT EXECUTE over the contract functions in
  public (name-scoped, idempotent) plus ALTER DEFAULT PRIVILEGES for
  future re-applies. Verified against a fresh dockerized disposable DB
  (revoke, re-run the block, has_function_privilege -> t).

## [2026-11-02] added | Epic F2 slice plan (receipts + allocations)

- docs/EPIC_F2_AR_RECEIPTS_PLAN.md: F2a payments live (external-reference
  unique via migration 0007, derived contract status axis, auto_apply 501
  until F2b), F2b applications (fact-derived balance guards, pair-accumulate
  under the 0006 UNIQUE constraint, event journal, advisory locks), F2c
  contract addition POST /payments/applications/{id}/reverse (no reversal op
  exists today), F2d live acceptance incl. the
  invoice+credit+receipt+allocation+outstanding identity.
- Contract-vs-0006 vocabulary mismatches recorded as mapping decisions;
  payment-op example placeholders flagged for the F2a spec commit.

## [2026-11-02] contribute | F2a receipts live (Epic F2 1/4)

- PR #144 (feat/accounting-f2a-payments): create/list/get customer
  payments over ar_payments. Server PAY-<year>-<seq> under the
  ar-payment-sequence advisory lock; migration 0007 partial-unique
  external reference (tenant, entity, reference WHERE NOT NULL) is the
  duplicate guard (409); contract status derived from allocation facts
  (PENDING/APPLIED_PARTIAL/APPLIED, CANCELLED->FAILED); auto_apply=true
  honest 501 until F2b; posted_to_gl/gl_entry_id honest until Epic H.
- Scope-class fix: resolve_scope_entities replaces F1b\u2019s arbitrary-
  entity resolve_legal_entity; unscoped list/get cover all org entities,
  ambiguous writes rejected. payment-op example placeholders fixed
  (statement ops wait for F3).
- Proof: 15 units + live acceptance (two orgs one tenant, duplicate
  guard, per-entity sequences via psql-through-RLS, derived axis,
  filters/pagination, cross-org/tenant denials); ledger+projection live
  suites rerun green; qa 230; fmt+workspace check clean.
- Next: F2b applications (balance guards, pair-accumulate, events
  journal), F2c contract additions (idempotency key + reversal op),
  F2d acceptance suite.

## [2026-11-02] contribute | F2b applications live (Epic F2 2/4)

- PR #145 (feat/accounting-f2b-applications): create/list payment
  applications + real auto_apply FIFO (F2a's honest 501 retired).
- Id-space decision: contract customer_invoice_id stays the OPEN-ITEM
  (posted document) id; applications FK the overlay row, so the flow
  heals a missing overlay row through F1c's projection refresh and maps
  ids internally — clients never see overlay row ids, cache is never
  hand-patched.
- Guards: derived document capacity and payment unapplied both 409;
  cross-customer/foreign-document 404; CANCELLED receipts rejected.
  Payment advisory lock always before document lock (no cycle).
- Pair row accumulates per (payment, document); every delta journaled to
  new ar_payment_application_events (0008, RLS, amount>0, kind carries
  sign; REVERSED reserved for F2c). Payment applied_at now derived on
  read (last allocation instant, batched per page) instead of null.
- Proof: 18 units + live acceptance (accumulation, journal counts via
  psql-through-RLS, healed overlays 60/20, FIFO due-date order, RLS-
  bounded journal); ledger/projection/payments live suites rerun green;
  clippy -D warnings all-targets clean.
- Next: F2c reversal (spec commit first, then regen) + idempotency-key
  discussion, F2d receipts identity acceptance suite.

## [2026-11-02] contribute | F2c reversal + idempotency live (Epic F2 3/4)

- PR #146 (feat/accounting-f2c-reversal), spec-first commits: contract
  (required idempotency_key on POST /payments; POST
  /payments/applications/{id}/reverse), disposable regen, behavior.
- Reversal semantics: the pair row SURVIVES at applied_amount 0 —
  migration 0009 replaces the 0006 inline CHECK (>0) with a named >= 0,
  because the events journal FKs the pair row and CASCADE would destroy
  the audit trail. Money returns in full to payment + overlay (F1c
  refresh); a REVERSED event carries actor, time, required reason;
  re-apply accumulates on the same pair row as a fresh APPLIED event.
- Idempotency rides accounting_idempotency_records with GL fingerprints
  (tenant+entity+action+every body field; reason included on reversals);
  replay re-serves value-for-value before the duplicate-reference guard;
  mismatch is 409. AR leaves document_id NULL — it FKs posted documents
  (live test caught the violation).
- Read axis filters applied > 0 in application list, open-item history,
  and applied_at stamps.
- Proof: 18 units + 7 live (idempotent replay/conflict suite; reversal
  balances, journal agg APPLIED|REVERSED|APPLIED on one pair row, replay
  moves nothing, double-reverse + reason-fingerprint 409, foreign 404);
  rerun-safe; clippy + pre-commit clean.
- Next: F2d receipts identity acceptance suite (epic closer).

## [2026-11-05] contribute | F2d acceptance suite live — Epic F2 complete (Epic F2 4/4)

- PR #147 (feat/accounting-f2d-receipts-identity), tests-only. No contract
  or impl change: live_f2d_receipt_identity_acceptance pins the epic thesis
  invoice + credit + receipt + allocation + outstanding value-for-value
  across read path (composed items), cache (projection refreshed==0,
  honest drift None without GL account), and events journal (per-pair
  net(APPLIED-REVERSED)==rows; per-receipt applied+unapplied==amount).
- Covers full/partial/multi-doc receipts, over-balance + over-payment 409
  with reason assertions, reversal reopen + replay stability (CURRENT
  state, not historical snapshot — asserted honestly), cross-tenant
  invisibility with identical customer uuid; rerun-safe twice; distinct
  run suffix isolates uuid slots from the sibling F2b scenario.
- #146 review round (pre-F2d): CodeRabbit 8 findings all valid — reload
  snapshots AFTER advisory locks (apply/FIFO/reverse — a class missed by
  #145's review), idempotency lookup inside the lock, write-permission
  gate on AR mutations, raw+auto_apply+presence in the fingerprint, FIFO
  inside create (replays never allocate), sequence lock before the
  idempotency window, 0009 NOT VALID + separate VALIDATE. Merged d89725bb.
- Epic F2 merged state: contracts + migrations 0007-0009 + live suites
  (21 units + 8 live, serial). Next per BUILDOUT: Epic F3 (aging +
  statements) or Epic G; awaiting priority.

## [2026-11-05] plan | Epic F3 aging & statements planned (Epic F3 0/4)

- Epic F2 closed (PR #147 merged 20b2eec2; plan header COMPLETE).
- New plan docs/EPIC_F3_AGING_STATEMENTS_PLAN.md (PR pending): F3a
  fact-derived as-of-reproducible aging_summary (never reads the overlay
  cache; buckets == ledger::aging_bucket; per (entity, currency)); F3b
  statement jobs = Epic E1 invoice outbox precedent — AR owns job +
  frozen render model + durable instruction, honest QUEUED only (render
  scaffold must not be wired; no fake SENDING/COMPLETED); F3c acceptance
  suite. Spec example defects (statement_date 1000.0, delivery_method
  bank_transfer) fixed in F3b's spec commit.

## [2026-11-05] contribute | F3a aging summary live (Epic F3 1/4)

- PR pending (feat/accounting-f3a-aging): GET /reports/aging-summary
  delivered — fact-derived as-of replay (documents/credit notes by
  posted_at, applied net from the APPLIED/REVERSED event journal by
  occurred_at; never overlay_state/cache columns; write_off counts at
  every as_of, documented). Single (company, base-currency) scope;
  foreign company in another tenant refused, not substituted.
- DB constraint learned: accounting_legal_entities is unique per
  (tenant, org) — multi-entity orgs are multi-TENANT; the
  company-disambiguation guard stays defensive, live tests prove the
  silent single-entity default + Policy reject of foreign company.
- Live proof: journal replay separates an application from its later
  reversal at an intermediate as_of (row sums cannot); summary ==
  open-items value-for-value with poisoned overlay cache; EUR/GBP
  separation; rerun-safe on the disposable DB.

## [2026-11-05] contribute | PR #149 review round: snapshot aging reads, bounded replay

- CodeRabbit round on the F3a aging summary: the aggregate now reads
  inside ONE REPEATABLE READ transaction via the new lifeguard
  `with_session_transaction_isolated` (microscaler/lifeguard#236,
  merged 44a2412) — the level rides on the BEGIN because the pipelined
  `rls_set_session` is already the transaction's first query. Every
  explicitly requested level is spelled out, so a server
  `default_transaction_isolation` cannot override it; the legacy
  wrapper keeps the byte-identical plain BEGIN.
- Journal replay is now fetch-bound on every read and the
  overlay/application IN lists are chunked at 10k ids (16-bit bind
  limit), with the event bound enforced on the running total.
- rerp pins lifeguard 44a2412 (lock regenerated with local path
  patches set aside — git sources only).

## [2026-11-09] contribute | CRM story slice 1: CRM-RT-01 request context and errors

- `rerp_crm_runtime` now carries the shared `RequestContext`
  (claims + `X-Workspace-Id` + role, 403 `workspace_forbidden`
  fail-closed, `system()` for workers), `ApiError` rendered as the
  contract `ErrorResponse`, `MembershipSource` with static and 60 s
  cached implementations, and `workspace_guard_sql()` for RLS
  transactions. First story done from `docs/crm/stories/` under the
  HANDOVER/SUPERVISION process; report in `docs/crm/reports/`.
- Legacy `openapi/crm/{accounts,pipeline,...}` specs keep the old
  `{code, message}` error body — the AC-3 contract test is scoped to
  the nine story services by design.

## [2026-11-09] contribute | CRM story slice 2: CRM-DB-01 local databases and migration pipeline

- `microservices/crm/scripts/crm-db.sh` (with `test-crm-db.sh` and a
  `tooling/tests/test_crm_db.py` wrapper) creates the three CRM databases
  (`crm`, `crm_insights`, `crm_intake`) and one login role per service —
  non-superuser, non-BYPASSRLS, `CONNECT` scoped to its own database,
  schema-scoped table grants plus default privileges, audit INSERT for all
  services and audit SELECT only for `crm_workspace`. No role for
  `crm/edge` (ADR 004). `apply` walks
  `microservices/crm/migrations/apply_order.txt` (migrator format) with a
  sorted-glob fallback while the migrator has no CRM LifeModels.
- `audit.audit_log` and `crm_current_workspace_id()` are provisioned by
  `up`, not by a service migration, because no single service owns them;
  apply runs as the admin with pre-granted default privileges so templates'
  RLS policies apply from the first migration.
- AC-3 Mac compatibility is by construction (bash-3.2-safe, plain
  psql/sed/grep) — one real Mac run is still outstanding.

## [2026-11-10] contribute | CRM story slice 3: CRM-RT-02 event machinery (PR #154)

- `microservices/crm/runtime/src/events.rs`: `Envelope` (UUID v7
  `event_id`, field names asserted against each service's generated
  asyncapi payload contracts), `append` (outbox row inside the caller's
  transaction), `relay_batch`/`run_relay` to `crm.<service>.v1` with
  `event_type`/`workspace_id`/`schema_version` attributes and
  `entity_id` ordering key, `consume_batch`/`run_consumer` (handler +
  `processed_events` marker in one workspace-guarded transaction, ack
  after commit, duplicates acked, poison bodies acked away).
- All table access goes through the service's own Lifeguard entity via
  the `OutboxTable`/`ProcessedTable` traits — no raw SQL except the
  shared transaction-scoped `crm.workspace_id` guard from `context`.
  Per-service entity impls ship with each service's data-model story.
- A failed publish holds that entity's later rows for the rest of the
  round (per-entity ordering; retry next round oldest-first) —
  CodeRabbit-found ordering hazard, fixed pre-merge.
- Pub/Sub REST client moved `crm/conversion` → shared `crm/runtime`;
  publish body carries `orderingKey` (PubsubMessage proto field, not
  `messageOrderingKey`).
- Template gains the documented permissive relay SELECT policy on
  `<schema>.outbox` (the service schema is the isolation boundary; the
  relay reads every workspace).
- Six `#[ignore]`d tests in `tests/events_db.rs` run the full cycle
  over real entities when `CRM_TEST_DATABASE_URL` points at a migrated
  CRM database.

## [2026-11-10] contribute | CRM story slice 4: CRM-AUD-00 audience data model (PR #155)

- 15 LifeModels in `microservices/crm/audience/impl/src/models/` from
  `openapi/crm/docs/data-model/audience.yaml`, design order and types exact;
  generated `migrations/audience/20261110131017_*.sql` + authored
  `0002_rls_and_infrastructure.sql` (no audience RLS exceptions).
- Foreign keys carry the `audience.` schema prefix in `#[foreign_key]` — the
  generated CREATE must apply without a service search_path; future services
  follow the same rule.
- Expression uniques (`workspace_id, lower(email)`) arrive as
  `#[index = "UNIQUE idx_…(workspace_id, lower(email) | email)"]`; the right
  side of `|` lists coverage columns (lifeguard expression-index grammar).
- Composite primary keys needed lifeguard#242 (merged): more than one
  `#[primary_key]` now emits one table-level `PRIMARY KEY (a, b)` clause and
  no generated defaults on members. rerp's pinned lifeguard rev must bump to
  the merged commit before the next service regenerates.
- The suite migrator strips `templates/` from `apply_order.txt` (authored
  `{{placeholder}}` inputs are never deliverables); `apply_order.txt` is fully
  tool-produced — the glob FK-orders the authored 0002 last, so story step 5's
  manual append is no longer needed.
- Verified on throwaway Postgres 15 via `scripts/crm-db.sh` (reset/up/apply ×2)
  as the non-superuser `crm_audience` role: A/B/unset visibility 1/0/0,
  cross-workspace insert and the status CHECK rejected, untagged outbox insert
  refused, 18 tables, 13 FKs, all 15 CHECKs and both composite pkeys present.
