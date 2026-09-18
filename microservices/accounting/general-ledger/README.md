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

The first read-only General Ledger runtime slice is delivered through the
canonical `openapi/accounting/general-ledger/openapi.yaml` contract:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/v1/accounts` | List configured posting accounts with bounded filters. |
| `GET` | `/v1/fiscal-periods` | List periods overlapping an optional date range. |
| `GET` | `/v1/journal-entries/{id}` | Retrieve one immutable journal and its ordered lines. |
| `GET` | `/v1/trial-balance` | Derive an as-of-date, single-currency trial balance from posted lines. |

BRRTRouter validates the Sesame bearer token. The implementation requires the
exact `accounting:ledger:read` permission, converts the validated tenant,
organization, subject, and session claims into a Lifeguard `SessionContext`,
and installs that context in a pinned PostgreSQL transaction. Queries apply an
explicit tenant and legal-entity predicate in addition to database RLS.

The trial balance is not another stored balance. It validates every selected
journal independently, rejects unknown sides, non-positive lines, header/line
total drift, and account/currency-book inconsistencies, then derives balances
from immutable lines. Zero-balance accounts are opt-in.

The retired broad contract remains product research only. Generic raw journal
posting, mutable balances, period close/reopen, reversal, opening balances, and
multi-dimensional reporting are not delivered by this slice and are not
published as active handlers. Generated artifacts remain disposable; business
logic lives only in `impl/`.

## Verification

```bash
~/Workspace/microscaler/BRRTRouter/target/debug/brrtrouter-gen lint \
  --spec openapi/accounting/general-ledger/openapi.yaml

cd microservices
cargo test -p rerp_accounting_general_ledger --no-default-features
cargo test -p rerp_migrator --features accounting
cargo run -p rerp_migrator --features accounting -- \
  validate --suite accounting --migration-history
```
