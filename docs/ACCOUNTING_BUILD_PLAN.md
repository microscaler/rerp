# Accounting Suite Build Plan

> **Date:** 2026-04-25
> **Status:** verified — ready to execute Phase 1
> **Verified:** 2026-04-25 — all claims cross-checked against source
> **Related docs:**
> - `ACCOUNTING_BDD_FEATURE_BACKLOG.md` — BDD feature scenarios
> - `OPENAPI_ACCOUNTING_ODOO_GAP_ANALYSIS.md` — Odoo Enterprise benchmark
> - `OPENAPI_ACCOUNTING_ODOO_SERVICE_MAP.md` — service-by-service resource backlog
> - `ACCOUNTING_ENTERPRISE_ERP_GAP_ANALYSIS.md` — second-phase parity targets
> - `accounting/rules-engines/README.md` — rules engine dossier index
> - `llmwiki/topics/accounting-openapi-odoo-gap.md` — agent-facing summary
> - `llmwiki/topics/suite-aware-brrtrouter-wrapper.md` — wrapper contract
> - `llmwiki/topics/hauliage-reference-operating-model.md` — Hauliage patterns → RERP
> - `llmwiki/topics/service-implementation-and-database-layout.md` — service/entity/db layout

---

## Current State

### What's Done
- 17 accounting services with OpenAPI specs (9 impl + 7 spec-only)
- Full RERP workspace build passes (all 9 impl crates)
- BFF generation: 203 namespaced paths, 320 operations
- Tooling wrapper (`rerp`) working for gen/build/bff
- ArcSwap pattern stabilized across all impl crates
- `rerp_accounting_accounts_receivable` naming fixed
- LLMWiki has accounting pages with cross-references
- BDD feature backlog documented with Gherkin scenarios
- Rules-engine dossier template defined with build order

### What's Missing

#### 7 Services: Contract-Only (No Runtime)
These have OpenAPI specs and BFF config entries but lack Rust crates, Helm values, Dockerfiles, and deployment wiring:

| Service | Spec Size | Port | Entity Defs | Gap |
|---------|-----------|------|-------------|-----|
| `tax-compliance` | 16.5K | 8016 | No | Needs full scaffolding |
| `documents-extraction` | 13.3K | 8017 | No | Needs full scaffolding |
| `treasury` | 12.2K | 8018 | No | Needs full scaffolding |
| `consolidation` | 11.6K | 8019 | No | Needs full scaffolding |
| `revenue-recognition` | 11.6K | 8020 | No | Needs full scaffolding |
| `lease-accounting` | 10.9K | 8021 | No | Needs full scaffolding |
| `audit-controls` | 12.4K | 8022 | No | Needs full scaffolding |

#### 9 Services: Impl Exists, Business Logic Stubs
These build but have only TODO placeholder controllers (no real business logic):

| Service | Impl Status | Controllers | Entities | Key Gaps |
|---------|-------------|-------------|----------|----------|
| `general-ledger` | Builds ✅ | 36 | Yes (account, chart_of_accounts, journal_entry, journal_entry_line, account_balance) | Lock dates, close controls, audit trail, multi-currency revaluation |
| `invoice` | Builds ✅ | 10 | Yes (invoice, invoice_line) | Payment-register, refunds/credit notes, extraction, e-invoicing |
| `accounts-receivable` | Builds ✅ | 22 | Yes (ar_aging, ar_payment, ar_payment_application, customer_invoice) | Follow-up policies, statements, partner ledger, disputes |
| `accounts-payable` | Builds ✅ | 15 | Yes (ap_aging, ap_payment, ap_payment_application, vendor_invoice) | Payment batches, export files, 3-way match, supplier reporting |
| `bank-sync` | Builds ✅ | 15 | Yes (bank, bank_account, bank_statement, bank_transaction, bank_reconciliation) | Reconciliation models/rules, suggestions, partials, write-offs |
| `asset` | Builds ✅ | 15 | Yes (asset, asset_category, asset_depreciation, asset_transaction) | Asset models/groups, pause/resume/modify/sell/dispose lifecycle |
| `budget` | Builds ✅ | 15 | Yes (budget, budget_actual, budget_line_item, budget_period, budget_version) | Analytic dimensions, revisions, confirm/done/cancel lifecycle |
| `financial-reports` | Builds ✅ | 10 | Yes (financial_report, report_data, report_schedule, report_template) | Report definitions/expressions, drill-down, exports, schedules |
| `edi` | Builds ✅ | 10 | Yes (edi_acknowledgment, edi_document, edi_format, edi_mapping) | Standards, submissions, polling, acknowledgments, validation profiles |

**Verified 2026-04-25:** All 9 impl crates build cleanly (`cargo build --workspace` exits 0). Controllers are stubs with TODO placeholders — no real business logic. Lifeguard entities defined for all 9 services in `entities/src/accounting/`. No Helm values, Dockerfiles, or K8s service definitions exist.

---

## Priority Matrix

Priority levels from BDD backlog:

| Code | Meaning |
|------|---------|
| P0 | Preserve generated BFF coverage and settle generated artifact policy |
| P1 | Core accounting engines needed for credible operational accounting product |
| P2 | Enterprise breadth needed by accounting teams in production |
| P3 | Localization, statutory, jurisdiction-specific breadth |

---

## Plan: Three Phases

### Phase 1: Scaffolding (7 Spec-Only Services)

**Goal:** Make all 17 services runtime-ready.

For each service (`tax-compliance`, `documents-extraction`, `treasury`, `consolidation`, `revenue-recognition`, `lease-accounting`, `audit-controls`):

1. `rerp gen suite accounting --service <name> --force` — generates gen + impl crates
2. Fix package naming in impl Cargo.toml (ensure `_impl` suffix convention)
3. Fix main.rs ArcSwap pattern
4. Add `arc-swap` dependency to impl Cargo.toml
5. Create Helm values for the service
6. Create Dockerfile
7. Register in workspace Cargo.toml members
8. Verify `cargo build --workspace` passes

**Why first:** Rules-engine dossiers explicitly gate on runtime scaffolding existing. The pre-implementation gate says: "Runtime scaffolding must exist for any new microservice owner: gen, impl, Helm values, Dockerfile, workspace registration, and generated docs."

**Estimated:** ~1 service per session. 7 sessions total.

### Phase 2: Deepening P1 Services (Contract Expansion)

**Goal:** Implement the first buildable BDD slices from the backlog.

Per the BDD backlog, **Slice 1A** is the entry point: reconciliation suggestions on `bank-sync`.

#### Slice 1A: Reconciliation Suggestions (`bank-sync`)

From `ACCOUNTING_BDD_FEATURE_BACKLOG.md`:

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
```

Steps:
1. Read `docs/accounting/rules-engines/reconciliation-rules-engine.md` (the first dossier)
2. Expand `openapi/accounting/bank-sync/openapi.yaml` with:
   - `/reconciliation-models` — CRUD for matching rules
   - `/transactions/{id}/suggestions` — ranked candidate matches
   - Typed schemas: `ReconciliationModel`, `ReconciliationSuggestion`, `ReconciliationRule`
3. Regenerate gen crate: `rerp gen suite accounting --service bank-sync --force`
4. Implement impl controllers for suggestions endpoint
5. Implement the matching logic (exact reference, amount tolerance, memo match)
6. Add impl unit tests
7. Regenerate BFF and verify coverage

#### Slice 1B: Reconcile, Write Off, Unreconcile (`bank-sync`)

After 1A works end-to-end:
- `POST /transactions/{id}/reconcile` — accept reconciliation
- `POST /transactions/{id}/write-off` — record write-off
- `POST /transactions/{id}/unreconcile` — reverse reconciliation

#### Slice 1C: Report Definitions And Drill-Down (`financial-reports`)

- `POST /report-definitions` — create configurable reports
- `GET /report-definitions/{id}/lines` — report structure
- `GET /report-cells/{id}/drill-down` — source-line drill-down
- `POST /report-exports` — PDF/XLSX exports

#### Slice 1D: Generated BFF Acceptance (Cross-Cutting)

After expanding service specs, regenerate BFF and verify:
- New typed schemas appear in generated BFF
- Service-prefixed component names (e.g., `BankSyncCreateReconciliationModelRequest`)
- No anonymous `type: object` request bodies

**Why this order:** The BDD backlog deliberately starts here — it builds against typed contracts without committing to the full reconciliation rule DSL or report expression runtime too early.

### Phase 3: Rules Engine Implementation

**Goal:** Implement the reconciliation rules engine (first of 8 engines).

After Slice 1A/B contracts are live:

1. Study `reconciliation-rules-engine.md` dossier
2. Implement rule model: rule types, predicates, actions, priority, versioning
3. Implement execution lifecycle: draft, validate, simulate, run, approve, post, reverse
4. Implement explainability: match reasons, source lines, confidence, trace IDs
5. Implement auditability: immutable events, actors, timestamps, before/after state
6. Implement dry-run mode (no accounting side effects)
7. Wire rules engine to reconciliation suggestions endpoint

**After reconciliation engine**, build order from README:
1. ~~reconciliation~~ → 2. report-expression-engine → 3. tax-compliance → 4. extraction-classification → 5. consolidation-elimination → 6. revenue-recognition → 7. lease-accounting → 8. audit-controls

---

## Execution Order (Summary)

```
Phase 1: Scaffold 7 services                    [sessions 1-7]
  ├── tax-compliance
  ├── documents-extraction
  ├── treasury
  ├── consolidation
  ├── revenue-recognition
  ├── lease-accounting
  └── audit-controls

Phase 2: Deepen P1 services (contract expansion) [sessions 8-12+]
  ├── Slice 1A: Reconciliation suggestions       [session 8]
  ├── Slice 1B: Reconcile/write-off/unreconcile  [session 9]
  ├── Slice 1C: Report definitions/drill-down    [session 10]
  └── Slice 1D: BFF acceptance verification      [session 11]

Phase 3: Rules engines                           [sessions 12+]
  ├── Reconciliation rules engine
  ├── Report expression engine
  └── Remaining 6 engines (tax, extraction, consolidation, revenue, lease, audit)
```

---

## Conventions To Follow

### Source-of-Truth Workflow
1. OpenAPI spec first (`openapi/{suite}/{service}/openapi.yaml`)
2. Regenerate gen crate via `rerp gen suite`
3. Implement business logic in `impl/` controllers
4. Regenerate BFF after contract changes
5. Verify with `cargo build --workspace`

### Naming Convention
- Gen crate: `rerp_accounting_{module}` (NOT `*_gen` suffix — confirmed from general-ledger pattern)
- Impl crate: `rerp_accounting_{module}_impl`
- Controller imports: `use rerp_accounting_{module}::handlers::*`
- main.rs ArcSwap pattern: `Arc<ArcSwap<Router>>`, not `Arc<RwLock<Router>>`

### Database/Entity
- Lifeguard entities in `entities/src/` (shared crate)
- Do NOT edit migration files directly — update entity structs, then run migrator
- Index columns must exist on the struct they're defined on

### Rules Engine Pre-Gate
Before implementing any engine:
- Owning service must have generated BFF contract
- Required request/response schemas must be named and service-prefixed
- BDD slice must identify fixtures, inputs, outputs, expected side effects
- Runtime scaffolding must exist (gen, impl, Helm, Dockerfile)
- Engine must define dry-run mode (no accounting side effects)

---

## Known Issues To Track

1. **`openapi/accounting/openapi.yaml`** — legacy top-level aggregate (370K, 22K lines). Decide: regenerate, keep, or deprecate. BFF is the source of truth now.
2. **`journal-entrys`** → `journal-entries` — stale generated name fix needed in generator config.
3. **No Helm values or Dockerfiles** — no per-service Helm values under `helm/rerp-microservice/values/` and no Dockerfiles in impl dirs. All 9 services are bare-bones impl-only.
4. **No Playwright/BDD frontend tests** — project is backend-only, no frontend testing infrastructure exists.
5. **Controllers are stubs** — all 158 controllers across 9 services are TODO placeholder functions returning example/hardcoded data. See controller counts above.
6. **Port registry** — accounting services use 8011-8015; new services use 8016-8022. Verify Tiltfile reads ports from suite config, not hardcoded.

---

## First Session: Phase 1 Entry Point

Per the rules-engine dossier build order and BDD backlog priority, the natural first target is `tax-compliance` (simplest spec at 16.5K, first in the engine build order for engine #3).

### Phase 1 Scaffolding Checklist (per service)

For each of the 7 spec-only services, execute these steps in order:

1. `rerp gen suite accounting --service <name> --force` — generates gen + impl crates
2. Fix package naming in impl Cargo.toml (ensure `rerp_accounting_<name>_impl` convention)
3. Fix main.rs ArcSwap pattern (use `Arc<ArcSwap<Router>>`, not `Arc<RwLock<Router>>`)
4. Add `arc-swap` dependency to impl Cargo.toml
5. Create Dockerfile in impl/
6. Create Helm values in `helm/rerp-microservice/values/<name>/`
7. Register in workspace `microservices/Cargo.toml` members
8. Verify `cargo build --workspace` passes
9. Verify generated BFF still at 203 paths / 320 operations (no regression)

### Scaffolding Order

1. `tax-compliance` (engine #3 in rules build order, simplest spec)
2. `documents-extraction` (engine #4)
3. `treasury` (P2, standalone)
4. `consolidation` (engine #5, gates on treasury for intercompany data)
5. `revenue-recognition` (engine #6, gates on invoice + GL)
6. `lease-accounting` (engine #7, gates on asset + GL)
7. `audit-controls` (engine #8, cross-cutting, depends on all others)

### Entity Defs

No entities exist for the 7 spec-only services. Entity definitions should be added to `entities/src/accounting/` after Phase 1 scaffolding, once the business logic requirements are clear from Phase 2 contract expansion.
