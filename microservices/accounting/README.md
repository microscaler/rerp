# RERP Accounting Suite

This directory is the installation and ownership boundary for Accounting
runtime code, persistence models, migrations, SQL controls, tests, and
operational assets. RERP hosts multiple suites, so Accounting-specific assets
must not be added at the repository root.

## Current persistence status

This README and its diagrams were reconciled against the effective
`LifeModel` sources and Accounting foundation migrations on **2026-07-16**.

- The effective model inventory contains **42 table models with 42 unique
  owners**.
- The suite-wide `accounting/foundation` provider owns 10 models.
- Eight service providers own the remaining 32 models; the General Ledger
  provider intentionally owns no parallel ledger tables.
- Only the 10-table foundation currently has a delivered, ordered migration
  set and app-owned Accounting/RLS controls.
- The service models describe the current registries. Their presence does not
  mean the corresponding service or migration has passed delivery acceptance.
- Documents Render is a separate suite and is deliberately absent from these
  diagrams.

The audited ownership evidence and remaining semantic decisions are recorded
in
[`../../docs/roadmap/hauliage-accounting-dogfood/service-readiness-plan/entity-ownership.md`](../../docs/roadmap/hauliage-accounting-dogfood/service-readiness-plan/entity-ownership.md).

## Model inventory

| Provider | Model count | Tables |
|---|---:|---|
| `accounting/foundation` | 10 | legal entities, fiscal periods, posting accounts, immutable posted documents/lines, journals/lines, idempotency, audit, document artifacts |
| `accounting/general-ledger` | 0 | consumes the suite foundation; no second ledger schema |
| `accounting/invoice` | 2 | invoices and invoice lines |
| `accounting/accounts-receivable` | 4 | customer invoices, receipts, allocations, aging |
| `accounting/accounts-payable` | 4 | vendor invoices, payments, allocations, aging |
| `accounting/bank-sync` | 5 | banks, bank accounts, statements, transactions, reconciliations |
| `accounting/asset` | 4 | asset categories, assets, depreciation, transactions |
| `accounting/budget` | 5 | budgets, versions, periods, lines, actuals |
| `accounting/edi` | 4 | formats, mappings, documents, acknowledgments |
| `accounting/financial-reports` | 4 | templates, schedules, reports, report data |

## Entity relationship diagrams

These are relationship-focused diagrams. They include every current entity,
all primary keys, declared/migrated foreign keys, and the main fields needed to
understand cardinality. Audit timestamps, metadata, descriptive fields, and
many non-relational values are intentionally omitted.

Relationship rules:

- A relationship is drawn only when it is declared with `#[foreign_key]` or
  enforced by the delivered foundation migrations.
- Optionality reflects nullable foreign-key fields where the current schema
  expresses it.
- `customers` and `vendors` are shown as external target tables because current
  AR/AP models declare foreign keys to them, but no Accounting `LifeModel`
  currently owns either table. That is unresolved schema ownership, not a
  delivered relationship.
- UUID fields that merely look relational are listed later and are not drawn
  as physical relationships.

### Delivered Accounting foundation

This is the currently migrated, tenant-scoped posting kernel. Composite
foreign keys in `migrations/foundation/0002_controls_and_rls.sql` enforce that
related rows share `tenant_id` and `legal_entity_id`.

```mermaid
erDiagram
    accounting_legal_entities {
        uuid id PK
        string tenant_id
        uuid organization_id
        string legal_name
        string base_currency
        boolean is_active
    }

    accounting_accounts {
        uuid id PK
        string tenant_id
        uuid legal_entity_id FK
        string code
        string account_type
        string normal_side
        string control_role
        string currency_code
        boolean is_active
    }

    accounting_fiscal_periods {
        uuid id PK
        string tenant_id
        uuid legal_entity_id FK
        date start_date
        date end_date
        string state
    }

    accounting_posted_documents {
        uuid id PK
        string tenant_id
        uuid legal_entity_id FK
        uuid fiscal_period_id FK
        uuid original_document_id FK
        uuid customer_id
        string document_number
        string document_type
        string status
        string currency_code
        decimal total_amount
    }

    accounting_posted_document_lines {
        uuid id PK
        string tenant_id
        uuid legal_entity_id
        uuid document_id FK
        uuid revenue_account_id FK
        uuid tax_liability_account_id FK
        int line_number
        decimal quantity
        decimal total_amount
    }

    accounting_journal_entries {
        uuid id PK
        string tenant_id
        uuid legal_entity_id
        uuid fiscal_period_id FK
        uuid source_document_id FK
        string entry_number
        date entry_date
        string currency_code
        decimal total_debit
        decimal total_credit
    }

    accounting_journal_lines {
        uuid id PK
        string tenant_id
        uuid legal_entity_id
        uuid journal_entry_id FK
        uuid account_id FK
        int line_number
        string side
        decimal amount
    }

    accounting_idempotency_records {
        uuid id PK
        string tenant_id
        uuid legal_entity_id FK
        uuid document_id FK
        uuid journal_entry_id FK
        string idempotency_key
        string request_fingerprint
        string status
    }

    accounting_audit_events {
        uuid id PK
        string tenant_id
        uuid legal_entity_id FK
        uuid subject_id
        uuid document_id FK
        uuid original_document_id FK
        string action
        datetime occurred_at
    }

    accounting_document_artifacts {
        uuid id PK
        string tenant_id
        uuid legal_entity_id
        uuid document_id FK
        string media_type
        string object_key
        string sha256
        bigint size_bytes
        datetime rendered_at
    }

    accounting_legal_entities ||--o{ accounting_accounts : owns
    accounting_legal_entities ||--o{ accounting_fiscal_periods : controls
    accounting_legal_entities ||--o{ accounting_posted_documents : scopes
    accounting_legal_entities ||--o{ accounting_idempotency_records : scopes
    accounting_legal_entities ||--o{ accounting_audit_events : scopes
    accounting_fiscal_periods ||--o{ accounting_posted_documents : contains
    accounting_fiscal_periods ||--o{ accounting_journal_entries : contains
    accounting_posted_documents o|--o{ accounting_posted_documents : corrects
    accounting_posted_documents ||--o{ accounting_posted_document_lines : contains
    accounting_posted_documents ||--o| accounting_journal_entries : posts
    accounting_posted_documents ||--o{ accounting_audit_events : audited_by
    accounting_posted_documents o|--o{ accounting_audit_events : original_for
    accounting_posted_documents o|--o{ accounting_idempotency_records : completes
    accounting_posted_documents ||--o{ accounting_document_artifacts : renders
    accounting_journal_entries ||--o{ accounting_journal_lines : contains
    accounting_journal_entries o|--o{ accounting_idempotency_records : completes
    accounting_accounts ||--o{ accounting_journal_lines : classifies
    accounting_accounts ||--o{ accounting_posted_document_lines : revenue_account
    accounting_accounts o|--o{ accounting_posted_document_lines : tax_account
```

### Invoice, AR, and AP registries

These are current service-owned workflow/subledger models. General Ledger no
longer owns `accounts`, `chart_of_accounts`, `journal_entries`,
`journal_entry_lines`, or mutable `account_balances`: all posting and future GL
queries use the delivered `accounting_*` foundation shown above. The remaining
semantic gate is to keep Invoice workflow and AR/AP open-item records from
becoming competing sources of posted-document truth.

```mermaid
erDiagram
    invoices {
        uuid id PK
        string invoice_number
        date invoice_date
        date due_date
        string invoice_type
        string status
        uuid customer_id
        uuid vendor_id
        decimal total_amount
        decimal outstanding_amount
        string currency_code
    }

    invoice_lines {
        uuid id PK
        uuid invoice_id FK
        int line_number
        uuid account_id
        decimal quantity
        decimal unit_price
        decimal line_total
    }

    customers {
        uuid id PK
        string ownership "referenced target has no Accounting LifeModel"
    }

    customer_invoices {
        uuid id PK
        uuid invoice_id FK
        uuid customer_id FK
        decimal outstanding_amount
        int days_overdue
        string aging_bucket
    }

    ar_payments {
        uuid id PK
        uuid customer_id FK
        string payment_number
        date payment_date
        decimal payment_amount
        decimal applied_amount
        decimal unapplied_amount
        string status
    }

    ar_payment_applications {
        uuid id PK
        uuid payment_id FK
        uuid invoice_id FK
        decimal applied_amount
        datetime applied_at
    }

    ar_agings {
        uuid id PK
        uuid customer_id FK
        date aging_date
        decimal total_outstanding
        string currency_code
    }

    vendors {
        uuid id PK
        string ownership "referenced target has no Accounting LifeModel"
    }

    vendor_invoices {
        uuid id PK
        uuid invoice_id FK
        uuid vendor_id FK
        decimal outstanding_amount
        int days_until_due
        string aging_bucket
    }

    ap_payments {
        uuid id PK
        uuid vendor_id FK
        string payment_number
        date payment_date
        decimal payment_amount
        decimal applied_amount
        string status
    }

    ap_payment_applications {
        uuid id PK
        uuid payment_id FK
        uuid invoice_id FK
        decimal applied_amount
        datetime applied_at
    }

    ap_agings {
        uuid id PK
        uuid vendor_id FK
        date aging_date
        decimal total_outstanding
        string currency_code
    }

    invoices ||--o{ invoice_lines : contains
    invoices ||--o| customer_invoices : extends_for_AR
    invoices ||--o| vendor_invoices : extends_for_AP
    customers ||--o{ customer_invoices : billed
    customers ||--o{ ar_payments : pays
    customers ||--o{ ar_agings : aged_by
    ar_payments ||--o{ ar_payment_applications : allocates
    customer_invoices ||--o{ ar_payment_applications : receives
    vendors ||--o{ vendor_invoices : bills
    vendors ||--o{ ap_payments : paid_by
    vendors ||--o{ ap_agings : aged_by
    ap_payments ||--o{ ap_payment_applications : allocates
    vendor_invoices ||--o{ ap_payment_applications : receives
```

### Banking, assets, and budgets

```mermaid
erDiagram
    banks {
        uuid id PK
        string name
        string bic
        string swift_code
        boolean is_active
    }

    bank_accounts {
        uuid id PK
        uuid bank_id FK
        string account_number
        string account_type
        string currency_code
        decimal current_balance
        decimal reconciled_balance
        string sync_provider
        boolean is_active
    }

    bank_statements {
        uuid id PK
        uuid bank_account_id FK
        date statement_date
        decimal opening_balance
        decimal closing_balance
        int transaction_count
        string status
    }

    bank_transactions {
        uuid id PK
        uuid bank_account_id FK
        uuid statement_id
        date transaction_date
        decimal amount
        decimal balance_after
        string status
    }

    bank_reconciliations {
        uuid id PK
        uuid bank_account_id FK
        uuid statement_id FK
        date reconciliation_date
        decimal book_balance
        decimal bank_balance
        decimal difference
        string status
    }

    asset_categories {
        uuid id PK
        uuid parent_id FK
        string code
        string name
        boolean is_active
    }

    assets {
        uuid id PK
        uuid category_id FK
        string asset_number
        decimal purchase_cost
        decimal current_value
        decimal accumulated_depreciation
        string status
    }

    asset_depreciations {
        uuid id PK
        uuid asset_id FK
        uuid journal_entry_id
        date period_start
        date period_end
        decimal depreciation_amount
        string status
    }

    asset_transactions {
        uuid id PK
        uuid asset_id FK
        uuid journal_entry_id
        date transaction_date
        string transaction_type
        decimal transaction_amount
    }

    budgets {
        uuid id PK
        uuid current_version_id
        string budget_number
        int fiscal_year
        date period_start
        date period_end
        decimal total_budget_amount
        string status
    }

    budget_versions {
        uuid id PK
        uuid budget_id FK
        uuid superseded_by_version_id
        int version_number
        boolean is_current
        decimal total_budget_amount
    }

    budget_periods {
        uuid id PK
        string period_name
        date period_start
        date period_end
        int fiscal_year
        int period_number
    }

    budget_line_items {
        uuid id PK
        uuid budget_id FK
        uuid version_id FK
        uuid account_id FK
        uuid period_id FK
        decimal budget_amount
        decimal actual_amount
        decimal variance
    }

    budget_actuals {
        uuid id PK
        uuid budget_id FK
        uuid account_id FK
        uuid period_id FK
        decimal budget_amount
        decimal actual_amount
        decimal variance
    }

    accounts {
        uuid id PK
    }

    banks ||--o{ bank_accounts : operates
    bank_accounts ||--o{ bank_statements : issues
    bank_accounts ||--o{ bank_transactions : records
    bank_accounts ||--o{ bank_reconciliations : reconciles
    bank_statements ||--o{ bank_reconciliations : closed_by
    asset_categories o|--o{ asset_categories : parent_of
    asset_categories ||--o{ assets : classifies
    assets ||--o{ asset_depreciations : depreciates
    assets ||--o{ asset_transactions : changes
    budgets ||--o{ budget_versions : versions
    budgets ||--o{ budget_line_items : contains
    budgets ||--o{ budget_actuals : measures
    budget_versions ||--o{ budget_line_items : defines
    budget_periods ||--o{ budget_line_items : schedules
    budget_periods ||--o{ budget_actuals : schedules
    accounts ||--o{ budget_line_items : budgets
    accounts ||--o{ budget_actuals : measures
```

### EDI and financial reporting

```mermaid
erDiagram
    edi_formats {
        uuid id PK
        string code
        string name
        string version
        boolean is_active
    }

    edi_mappings {
        uuid id PK
        uuid format_id FK
        string document_type
        string mapping_name
        json field_mappings
        boolean is_active
    }

    edi_documents {
        uuid id PK
        uuid format_id FK
        string document_number
        string document_type
        string status
        uuid related_invoice_id
        int retry_count
    }

    edi_acknowledgments {
        uuid id PK
        uuid document_id FK
        string acknowledgment_type
        string status
        datetime sent_at
        datetime received_at
    }

    report_templates {
        uuid id PK
        string template_code
        string report_type
        int version
        boolean is_active
        boolean is_system_template
    }

    report_schedules {
        uuid id PK
        uuid template_id FK
        string frequency
        datetime next_run_at
        string status
        string output_format
    }

    financial_reports {
        uuid id PK
        uuid template_id
        string report_code
        string report_type
        date report_date
        string status
        string currency_code
    }

    report_data {
        uuid id PK
        uuid report_id FK
        date report_date
        json data
        int data_version
        string currency_code
    }

    edi_formats ||--o{ edi_mappings : maps
    edi_formats ||--o{ edi_documents : parses
    edi_documents ||--o{ edi_acknowledgments : acknowledged_by
    report_templates ||--o{ report_schedules : schedules
    financial_reports ||--o{ report_data : materializes
```

## Logical references not enforced as foreign keys

The current models contain UUID references that are not annotated as foreign
keys and are not constrained by the delivered foundation migrations. They are
therefore intentionally absent as physical ERD relationships:

- `invoices.customer_id`, `vendor_id`, and `payment_term_id`;
- `invoice_lines.account_id`, `product_id`, and `tax_id`;
- `ar_payments.bank_account_id` and `ap_payments.bank_account_id`;
- asset account IDs and asset journal-entry IDs;
- `bank_transactions.statement_id`, `matched_payment_id`, and
  `reconciled_statement_id`;
- `budgets.current_version_id` and
  `budget_versions.superseded_by_version_id`;
- `financial_reports.template_id`;
- `edi_documents.related_invoice_id` and `related_purchase_order_id`;
- foundation references to external identities/business objects, including
  `accounting_legal_entities.organization_id`,
  `accounting_posted_documents.customer_id`, and subject/actor UUIDs such as
  `posted_by`, `closed_by`, `rendered_by`, and `subject_id`; and
- legacy General Ledger `fiscal_period_id` fields outside the delivered
  `accounting_fiscal_periods` foundation relationship.

Each must eventually become one of: an enforced same-suite foreign key, an
explicit immutable external/source reference, or a field removed from the
contract. A UUID name alone is not referential integrity.

## Layout

- `core/` — deterministic Accounting calculations with no transport or
  database executor.
- `entities/` — genuinely suite-wide foundation persistence models; never a
  duplicate catalog of service-owned models.
- `migrations/` — ordered Accounting schema and control migrations.
- `sql/` — vendored database contracts required by Accounting, including RLS.
- `scripts/` — Accounting database and operational setup.
- `<service>/gen/` — generated BRRTRouter contract layer.
- `<service>/impl/` — deployable, user-owned service behavior, including
  controllers, application services, service-owned Lifeguard models,
  validators, configuration, seeds, and tests.
- `../../openapi/accounting/<service>/openapi.yaml` — authoritative service
  contract; service-local transitional specs are not canonical.

The canonical architecture and ownership rules live in
[`../../CONTRIBUTING.md`](../../CONTRIBUTING.md). In particular:

- every Accounting HTTP service retains its own `gen/` and `impl/` crates;
- a service-specific `LifeModel` belongs in that service's
  `impl/src/models/`;
- only a concept with no natural service owner and shared across the suite
  belongs in `accounting/entities/`;
- the same effective table or view must never be defined by two providers;
- service seeds and tests stay with the owning implementation, while
  cross-service acceptance tests stay in `accounting/tests/`; and
- the single top-level `microservices/migrator/` emits and applies Accounting
  migrations only under this suite's `migrations/` directory.

## Service anatomy

An Accounting service adapts the Hauliage service pattern beneath the suite
directory:

```text
accounting/<service>/
├── README.md
├── gen/                         # generated; never hand-edit
│   ├── doc/
│   ├── static_site/
│   └── src/{controllers,handlers}/
└── impl/                        # user-owned and deployable
    ├── build.rs                 # Lifeguard registry generation
    ├── config/
    ├── seeds/
    ├── src/
    │   ├── controllers/
    │   ├── services/
    │   ├── models/
    │   ├── validators/
    │   ├── impl_registry.rs
    │   ├── lib.rs
    │   └── main.rs
    └── tests/
```

Not every service requires every optional directory, but responsibility must
not be moved to a suite-wide implementation merely because several services
are developed together.

## Persistence and migration ownership

Every effective table/view has one owner and one provider. Service consumers
reuse the owner's library/API; they do not copy the `LifeModel`. The top-level
migrator identifies providers by `(suite, service)`, rejects duplicate table
identity, propagates provider errors, and writes only beneath:

```text
microservices/accounting/migrations/
├── apply_order.txt
├── foundation/
├── general-ledger/
├── invoice/
├── accounts-receivable/
└── accounts-payable/
```

Validate the current registry without writing migrations:

```bash
cd microservices
cargo run -p rerp_migrator --features accounting -- \
  validate --suite accounting --migration-history
```

Full generation remains gated by the Invoice/AR posted-document boundary and
other service-model reviews noted above; unique SQL table names are necessary
but not sufficient accounting architecture.

## Verification

```bash
cd microservices
cargo test -p rerp-accounting-core
cargo check -p rerp-entities --lib
cargo test -p rerp_migrator --features accounting
cargo run -p rerp_migrator --features accounting -- \
  validate --suite accounting --migration-history
```

Database setup is location-independent:

```bash
./microservices/accounting/scripts/setup-db.sh
```
