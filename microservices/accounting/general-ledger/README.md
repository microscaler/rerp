# RERP General Ledger

General Ledger owns ledger policy, controls, queries, and reporting behavior.
It does not own a second set of ledger tables.

## Persistence boundary

The authoritative Phase 1 ledger is the suite-wide Accounting foundation:

- `accounting_accounts`;
- `accounting_fiscal_periods`;
- `accounting_journal_entries`; and
- `accounting_journal_lines`.

Those models live in `microservices/accounting/entities` because Invoice and
future AR, AP, banking, and reporting capabilities must post and query the same
books atomically. The General Ledger implementation consumes those records via
`rerp_entities`; its entity provider is intentionally empty.

The retired `accounts`, `chart_of_accounts`, `journal_entries`,
`journal_entry_lines`, and mutable `account_balances` models were undelivered
scaffolding. Reintroducing them would create a parallel ledger and is protected
against by a migrator test.

## Runtime status

The broad OpenAPI and generated controller tree remain research/scaffold
surface. Their example handlers are not a delivered General Ledger runtime and
must not be enabled in Tilt, Helm, or the Accounting BFF.

The next runtime slice will narrow the canonical General Ledger contract to
authenticated, tenant-safe account, journal, fiscal-period, and trial-balance
capabilities backed by the Accounting foundation. Posted entries will remain
immutable; corrections use reversal workflows.

## Verification

```bash
cd microservices
cargo test -p rerp_migrator --features accounting
cargo run -p rerp_migrator --features accounting -- \
  validate --suite accounting --migration-history
```
