# WP0 Entity Ownership Inventory

**Audited:** 2026-07-15  
**Scope:** Accounting and Documents Render `LifeModel` definitions  
**Status:** effective table-name ownership reconciled; semantic model review remains

## Result

The pre-change worktree exposed 47 effective table names and 40 duplicate
table names. The duplicates came from two mechanical copy patterns:

- 37 non-foundation Accounting suite entity files were byte-for-byte copies of
  service-owned models; and
- 12 service model files were byte-for-byte copies of the shared Accounting
  foundation models for accounts, legal entities, and journal lines.

The non-foundation suite copies and the service-local foundation copies have
been removed. The obsolete `rerp-entities` migration binaries have also been
retired in favour of the top-level suite-aware migrator.

The effective inventory now contains **47 table names, 47 definitions, and no
duplicate table-name owners**. The Accounting foundation registry contains 10
models instead of the previous 47.

## Authoritative Providers

| Provider | Tables | Effective table names |
|---|---:|---|
| `accounting/foundation` | 10 | `accounting_accounts`, `accounting_audit_events`, `accounting_document_artifacts`, `accounting_fiscal_periods`, `accounting_idempotency_records`, `accounting_journal_entries`, `accounting_journal_lines`, `accounting_legal_entities`, `accounting_posted_document_lines`, `accounting_posted_documents` |
| `accounting/general-ledger` | 5 | `account_balances`, `accounts`, `chart_of_accounts`, `journal_entries`, `journal_entry_lines` |
| `accounting/invoice` | 2 | `invoice_lines`, `invoices` |
| `accounting/accounts-receivable` | 4 | `ar_agings`, `ar_payment_applications`, `ar_payments`, `customer_invoices` |
| `accounting/accounts-payable` | 4 | `ap_agings`, `ap_payment_applications`, `ap_payments`, `vendor_invoices` |
| `accounting/bank-sync` | 5 | `bank_accounts`, `bank_reconciliations`, `bank_statements`, `bank_transactions`, `banks` |
| `accounting/asset` | 4 | `asset_categories`, `asset_depreciations`, `asset_transactions`, `assets` |
| `accounting/budget` | 5 | `budget_actuals`, `budget_line_items`, `budget_periods`, `budget_versions`, `budgets` |
| `accounting/edi` | 4 | `edi_acknowledgments`, `edi_documents`, `edi_formats`, `edi_mappings` |
| `accounting/financial-reports` | 4 | `financial_reports`, `report_data`, `report_schedules`, `report_templates` |
| `documents/render` | 0 | No permanent Documents entities have been delivered yet. This is a WP7 gap, not permission to place render entities in Accounting. |

## Ownership Rules Applied

1. `microservices/accounting/entities` owns only the cross-service posting
   foundation used atomically by Invoice and the ledger kernel.
2. A service-specific table remains in
   `microservices/accounting/<service>/impl/src/models/`.
3. The migrator identifies every provider as `(suite, service)` and rejects
   multiple definitions of the same table, including duplicates inside one
   provider.
4. A provider generation error aborts validation/generation; it cannot omit a
   service silently.
5. Accounting validation is explicit:
   `cargo run -p rerp_migrator --features accounting -- validate --suite accounting`.

## Semantic Review Gates

Unique SQL names do not prove unique domain concepts. These groups require an
explicit decision before their owning work package generates or installs new
schema:

- **WP1 General Ledger:** decide whether `accounts`, `journal_entries`, and
  `journal_entry_lines` are genuine pre-posting workflow aggregates or obsolete
  predecessors of `accounting_accounts`, `accounting_journal_entries`, and
  `accounting_journal_lines`. RERP must not operate two authoritative ledgers.
- **WP1 General Ledger:** decide whether mutable `account_balances` is a valid
  projection/cache or should be a derived view/read model over immutable journal
  lines.
- **WP2/WP3 Invoice and AR:** document the boundary among Invoice workflow
  `invoices`, immutable `accounting_posted_documents`, and AR
  `customer_invoices`. They may be distinct lifecycle records, but must not
  become three competing invoice sources of truth.

Until those decisions are recorded and tested, `validate` is safe evidence but
full Accounting migration generation should not be treated as schema approval.

## Verification Evidence

- Static inventory: 47 effective tables, 0 duplicate definitions.
- `rerp-entities` build: 10 discovered foundation entities.
- Migrator tests with the Accounting provider feature: 8 passed, including
  aggregate-baseline no-drift and fail-closed change coverage.
- Accounting provider validation: 47 tables with one owner each.
- Accounting setup script syntax: `bash -n` passed and seed discovery is scoped
  to `microservices/accounting`.
