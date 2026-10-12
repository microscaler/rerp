# Accounts Receivable — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/accounts-receivable
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for customer invoices, payments and
applications, credit memos, aging, collections, follow-up levels/runs,
payment promises, and customer statements.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IFRS 9 — Financial Instruments | Receivables at amortised cost; expected credit losses (ECL) evidenced by aging | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/> |
| IFRS 15 — Revenue from Contracts with Customers | AR originates from recognised revenue | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-15-revenue-from-contracts-with-customers/> |
| IAS 1 — Presentation of Financial Statements | Gross presentation of receivables vs allowance | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |

## Endpoint inventory (29 operations)

Customer invoices CRUD; customer payments CRUD + applications + auto-apply;
credit memos; collections; AR aging snapshots + aging summary; follow-up
levels and runs; collection cases; payment promises; customer statements.

## Use-case scenarios

### UC-AR-01 — Create AR invoice starts in DRAFT with zero amounts

Given customer `C-1` with credit_limit 10,000.00 and current exposure
2,000.00.
When creating `{invoice_id: "SO-77", customer_id, currency_code: USD, terms: NET_30, credit_limit_check: true}`.
Then status DRAFT, original/outstanding 0.00 (creation takes no amount),
aging recomputed; because exposure ≤ limit the call succeeds.

### UC-AR-02 — Credit limit check

Given the same customer with exposure 9,500.00 and limit 10,000.00.
When creating an invoice whose exposure would exceed the limit, with
`credit_limit_check: true`.
Then 409 "credit limit exceeded"; with `credit_limit_check: false` the
invoice is created (management override).

### UC-AR-03 — Aging buckets by days overdue

Given due_date 2026-01-15 and as_of 2026-03-17 (61 days later).
Then `days_overdue = 61` → bucket `61-90`; boundaries: ≤0 → CURRENT, 1–30,
31–60, 61–90, else `90+`; days_overdue is 0 on or before the due date.

### UC-AR-04 — Terms map to due dates

Given invoice_date 2026-02-01: NET_15 → 2026-02-16, NET_30 → 2026-03-03,
NET_45 → 2026-03-18, NET_60 → 2026-04-02, CUSTOM → +30, DUE_ON_RECEIPT →
same day.

### UC-AR-05 — Record payment, then auto-apply oldest-first

Given two invoices: INV-A due 2026-01-10 outstanding 400.00, INV-B due
2026-03-01 outstanding 600.00.
When `POST /customer-payments {amount: "700.00", currency_code: USD, auto_apply: true}`.
Then the payment applies 400.00 to INV-A (oldest due first → PAID) and
300.00 to INV-B (PARTIAL, outstanding 300.00); payment status
APPLIED_PARTIAL, unapplied_amount 0.00, INV-A last_payment_date set.

### UC-AR-06 — Manual application is bounded

Given payment P (unapplied 500.00) and invoice outstanding 300.00.
When applying 400.00 to that invoice.
Then rejected (applied > outstanding); applying 250.00 succeeds and leaves
unapplied 250.00; a second application of the same payment to the same
invoice is rejected by the unique (payment, invoice) key; applications to a
REFUNDED/FAILED payment are rejected.

### UC-AR-07 — Money handling: 2 dp rounding, negatives rejected

Given amounts `10.555` → stored/applied as `10.56`; `-1` and `NaN` →
rejected; currency `USD` accepted, `usd`/`USDT` rejected.

### UC-AR-08 — Credit memo reduces outstanding

Given invoice outstanding 1,000.00.
When `POST /credit-memos {customer_invoice_id, amount: "300.00", currency_code: USD, reason: RETURN}`.
Then memo status AVAILABLE with remaining 300.00; invoice outstanding
700.00 (PARTIAL); a memo above outstanding or ≤ 0 is rejected.

### UC-AR-09 — Aging snapshot is idempotent per customer/day

Given customer C-1 with invoices aging to buckets 1-30 and 90+.
When creating the aging snapshot for C-1 on 2026-03-17 twice.
Then one row exists keyed (tenant, customer, aging_date) and both calls
return the same bucket totals.

### UC-AR-10 — Aging summary report

Given the snapshot above.
When `GET /reports/aging-summary?company_id&as_of_date`.
Then totals match bucket sums.
(Drift: spec fields `days_sales_outstanding`, `collection_efficiency_index`,
`bad_debt_reserves` are **not emitted** by the current impl — backlog.)

### UC-AR-11 — Collection case and activities

Given customer C-1 with two open invoices (outstanding 900.00 total).
When creating a collection case with both invoice ids.
Then case OPEN with `total_due = 900.00`; a collection activity may be
recorded only against an existing invoice; `next_follow_up_date` defaults to
+7 days.

### UC-AR-12 — Follow-up levels and runs

Given levels: L1 (sequence 1, days_overdue 0, EMAIL), L2 (sequence 2,
days_overdue 30, LETTER).
When creating a follow-up run for 2026-03-17 with `dry_run=true`.
Then status QUEUED, not persisted, and the customer set contains holders of
overdue invoices and open cases; `dry_run=false` persists with status
COMPLETED. Creating a level with sequence 0 or duplicate name is rejected.

### UC-AR-13 — Payment promise drives case status

Given an OPEN collection case for C-1.
When creating `{customer_id, promised_amount: "500.00", promised_date: +7d}`.
Then the promise is OPEN and C-1's OPEN cases become PROMISED; a promise
with amount ≤ 0 or a past date is rejected.

### UC-AR-14 — Customer statements job

Given customers C-1 (has invoices) and C-2 (no invoices).
When `POST /customer-statements/send {customer_ids: [C-1, C-2], statement_date}`.
Then 404 (C-2 has no invoices); with valid customers only, a QUEUED job with
the correct statement count is returned.

### UC-AR-15 — Delete guards

Given a payment with allocations and a non-DRAFT invoice.
Then deleting the payment reverses allocations (invoice outstanding restored,
status CONFIRMED if fully restored, else PARTIAL) before deletion; the spec's
DRAFT-only/PENDING-only delete rules are **not enforced** by the impl today
(see drift) — scenarios must assert impl behaviour and flag the gap.

### UC-AR-16 — Tenant isolation

Given tenant A and tenant B each have customer `C-1`.
When tenant B lists invoices/payments/aging.
Then only tenant B rows appear; cross-tenant ids resolve to not-found, never
to tenant A's data.

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-AR-01/02 | none (credit-limit logic untested) | not yet implemented |
| UC-AR-03 | `receivables.rs::buckets_map_days_overdue`, `days_overdue_is_zero_before_due` | implemented |
| UC-AR-04 | `receivables.rs::terms_default_due_maps_payment_terms` | implemented |
| UC-AR-05/06 | none | not yet implemented |
| UC-AR-07 | `receivables.rs::money_rejects_negative_and_nan`, `money_rounds_to_minor_units`, `currency_requires_three_uppercase_letters` | implemented |
| UC-AR-08 | none | not yet implemented |
| UC-AR-09/10 | none | not yet implemented |
| UC-AR-11..14 | none | not yet implemented |
| UC-AR-15 | none | not yet implemented |
| UC-AR-16 | none | not yet implemented |

## Known drift / gaps

- Delete endpoints do **not** enforce the spec status rules (invoice
  DRAFT-only, payment PENDING-only) — deletion is unconditional and reverses
  allocations.
- `aging_summary` omits spec-required `days_sales_outstanding`,
  `collection_efficiency_index`, `bad_debt_reserves`, and `currency_code`.
- Payment applications and aging snapshots **hardcode `currency_code: "USD"`**.
- AR money rounds to 2 dp on parse; the database stores NUMERIC(19,4)
  (service-internal inconsistency across AP/asset/AR).
- Follow-up level enum `HOLD`, payment promise `KEPT/BROKEN` transitions, and
  `collection_status` (NORMAL→COLLECTION→LEGAL) have no driving logic yet.
