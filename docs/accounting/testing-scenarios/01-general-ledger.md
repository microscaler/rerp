# General Ledger — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/general-ledger
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for the general-ledger read path: accounts,
fiscal periods, immutable journals, and the derived trial balance. The service
owns **no tables**; it reads the shared foundation entities
(`microservices/accounting/entities/src/accounting/foundation/`) inside
tenant-scoped RLS transactions.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/reconciliation-rules-engine.md`](../rules-engines/reconciliation-rules-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 1 — Presentation of Financial Statements | Chart of accounts, journal completeness, trial-balance structure | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |
| IAS 8 — Accounting Policies, Changes in Estimates and Errors | Immutable posted journals; corrections only by new journal | <https://www.ifrs.org/issued-standards/list-of-standards/ias-8-accounting-policies-changes-in-accounting-estimates-and-errors/> |
| IAS 21 — Effects of Changes in Foreign Exchange Rates | Currency books and per-currency trial balances | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |

## Endpoint inventory (OpenAPI v0.2.0, permission `accounting:ledger:read`)

| Method | Path | Purpose |
|---|---|---|
| GET | `/v1/accounts` | List posting accounts (filter: account_type, currency_code, active, limit) |
| GET | `/v1/fiscal-periods` | List fiscal periods (filter: state, from_date, to_date, limit) |
| GET | `/v1/journal-entries` | List journal summaries (filter: from_date, to_date, currency_code, limit) |
| GET | `/v1/journal-entries/{id}` | One immutable journal with lines |
| GET | `/v1/trial-balance` | Trial balance derived from posted lines (required: as_of_date, currency_code) |

Money is serialized as strings; `balance = debit − credit` for DEBIT
normal-side accounts and `credit − debit` for CREDIT.

## Use-case scenarios

### UC-GL-01 — List accounts with a currency book

Given a tenant whose active legal entity holds accounts
`1010 Cash` (ASSET/DEBIT, USD), `4010 Sales` (REVENUE/CREDIT, USD), and
`1011 Cash EUR` (ASSET/DEBIT, EUR), and the caller holds
`accounting:ledger:read`.
When the caller requests `GET /v1/accounts?currency_code=USD&active=true&limit=10`.
Then the response contains `1010` and `4010` but **not** `1011`, and each item
omits the nullable `control_role` key instead of emitting `null`.

### UC-GL-02 — Fiscal period boundaries

Given the legal entity has periods `2026-01` (2026-01-01 → 2026-01-31, OPEN),
`2026-02` (2026-02-01 → 2026-02-28, CLOSED, closed_at=2026-03-03).
When listing `GET /v1/fiscal-periods?state=CLOSED`.
Then only `2026-02` is returned and `closed_at` is present; listing
`state=OPEN` returns `2026-01` with `closed_at` **omitted**.

### UC-GL-03 — Immutable journal retrieval

Given posted journal `JE-2026-000007` with two lines: DEBIT `1010 Cash`
1,200.00 / CREDIT `4010 Sales` 1,200.00.
When `GET /v1/journal-entries/{id}` is called.
Then total_debit == total_credit == `"1200.00"`, `lines[]` has both sides with
line numbers 1 and 2, and re-calling returns byte-identical content.

### UC-GL-04 — Journal list omits line detail

Given the journal above.
When `GET /v1/journal-entries?currency_code=USD&limit=50` is called.
Then the summary item has `entry_number`, `entry_date`, `total_debit`,
`total_credit`, and **no `lines` key**.

### UC-GL-05 — Trial balance nets to zero

Given posted lines: DEBIT `1010 Cash` 1,200.00; CREDIT `4010 Sales` 1,200.00;
DEBIT `2010 AP` 800.00 (a one-sided line on a DEBIT account representing a
payment); CREDIT `1010 Cash` 800.00.
When `GET /v1/trial-balance?as_of_date=2026-01-31&currency_code=USD`.
Then `1010` balance = `"400.00"` (1,200 − 800), `4010` = `"-1200.00"`,
`2010` = `"800.00"`, totals: debit `"2000.00"`, credit `"2000.00"`,
`difference` `"0.00"`, `balanced` true.

### UC-GL-06 — Zero-balance accounts are optional

Given account `9999 Suspense` has net activity of zero in the period.
When trial balance is requested with `include_zero_balance=false` (default).
Then `9999` is absent; with `include_zero_balance=true` it is present with
balance `"0.00"`.

### UC-GL-07 — Foreign-currency book isolation

Given the same journal is posted in USD, and account `1011 Cash EUR` has EUR
lines.
When `GET /v1/trial-balance?currency_code=EUR&as_of_date=...`.
Then only EUR accounts appear; requesting `currency_code=USD` never includes
`1011`, and lines referencing accounts outside the requested currency book
cause a validation error rather than silent inclusion.

### UC-GL-08 — Cross-journal cancellation is rejected

Given two separate one-sided journals: `JE-A` DEBIT `1010 Cash` 500.00 and
`JE-B` CREDIT `1010 Cash` 500.00 (each internally unbalanced).
When the trial balance is computed over both.
Then the read fails validation — a journal must be internally balanced; the
system never lets two wrong journals cancel each other out.

### UC-GL-09 — Header/total mismatch is rejected

Given a stored journal whose header says total_debit 1,200.00 but whose lines
sum to 1,000.00 (simulated corrupt data).
When the journal is read.
Then the response is a 500 Database error (integrity violation surfaced), and
the mismatch is logged with the entry id.

### UC-GL-10 — Boundary validation

Given requests with `limit=0`, `currency_code=usd` (lowercase), and
`from_date > to_date`.
Then each is rejected with 400 `validation_error`; `limit` defaults to 50 and
accepts 1..100.

### UC-GL-11 — Permission gate

Given a caller whose permissions do **not** include `accounting:ledger:read`
(e.g., only `accounting:ap:read`).
When any GL endpoint is called.
Then the response is 403 `insufficient_permission` — exact-match semantics;
`accounting:ledger:read-all` does not satisfy it.

### UC-GL-12 — Tenant isolation (RLS)

Given tenant A and tenant B each have an account with code `1010`.
When tenant B's caller lists accounts.
Then only tenant B's `1010` (its legal entity, its tenant_id) is visible, and
no tenant A row leaks through any filter combination.

### UC-GL-13 — No active legal entity

Given a tenant whose legal entity is deactivated.
When any GL endpoint is called.
Then the response is 403 `accounting_scope_unavailable` (Policy error), never
an empty 200.

### UC-GL-14 — Journal entry not found

Given a random UUID that matches no journal in the tenant scope.
When `GET /v1/journal-entries/{id}` is called.
Then 404 is returned with the `not_found` code.

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-GL-01 | `impl/src/ledger.rs::omits_nullable_account_fields_instead_of_emitting_null` | implemented |
| UC-GL-02 | `impl/src/ledger.rs::omits_closed_at_for_open_periods` | implemented |
| UC-GL-03 | `impl/src/ledger.rs::derives_normal_side_balances_from_one_sided_lines` (math only) | partial |
| UC-GL-04 | `impl/src/ledger.rs::formats_journal_headers_as_list_items` | implemented |
| UC-GL-05 | `impl/src/ledger.rs::derives_normal_side_balances_from_one_sided_lines` | implemented |
| UC-GL-06 | `impl/src/ledger.rs::zero_accounts_are_optional` | implemented |
| UC-GL-07 | `impl/src/ledger.rs::rejects_lines_for_accounts_outside_the_currency_book` | implemented |
| UC-GL-08 | `impl/src/ledger.rs::rejects_cross_journal_cancellation` | implemented |
| UC-GL-09 | none | not yet implemented |
| UC-GL-10 | `impl/src/ledger.rs::validates_boundary_inputs` | implemented |
| UC-GL-11 | `impl/src/http_support.rs::requires_exact_ledger_read_permission` | implemented |
| UC-GL-12 | none (requires seeded second tenant) | not yet implemented |
| UC-GL-13 | none | not yet implemented |
| UC-GL-14 | none | not yet implemented |

## Known drift / gaps

- The service is read-only in Phase 1; posting, period closing, and account
  creation happen through `invoice`/foundation migrations today. IAS 8-style
  correction journals have no endpoint yet.
- `uc-GL-09` and the two RLS/scope scenarios need a disposable PostgreSQL
  (like invoice's `#[ignore]` live test) rather than pure unit tests.
