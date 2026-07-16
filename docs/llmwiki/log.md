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
