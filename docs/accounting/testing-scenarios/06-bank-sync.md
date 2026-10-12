# Bank Sync — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/bank-sync
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for bank accounts, statement import, sync,
transaction matching (manual/auto/suggestions), reconciliations (with
completion rules), write-offs, exchange differences, and reconciliation/cash
position reports.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/reconciliation-rules-engine.md`](../rules-engines/reconciliation-rules-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 7 — Statement of Cash Flows | Cash and cash equivalents classification | <https://www.ifrs.org/issued-standards/list-of-standards/ias-7-statement-of-cash-flows/> |
| IAS 21 — Effects of Changes in Foreign Exchange Rates | FX differences on settlement of monetary items | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |
| IAS 1 — Presentation of Financial Statements | Bank reconciliation as internal control over cash presentation | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |

## Endpoint inventory (24 operations)

Bank accounts CRUD; statements (import/trigger-sync); transactions (list,
match, auto-match, suggestions, reconcile, unreconcile, write-off,
exchange-difference); reconciliations (start/complete); reconciliation
models; reports (reconciliation, cash-position).

## Use-case scenarios

### UC-BSY-01 — Create bank account with opening balance

Given bank `B-1` and GL account `1010 Cash USD`.
When `POST /bank-accounts {name: "Operating", account_number: "0001234", currency_code: USD, opening_balance: "25000.00", opening_balance_date: 2026-01-01}`.
Then created with current_balance 25,000.00; a duplicate IBAN or a
non-existent bank is rejected (FK RESTRICT).

### UC-BSY-02 — Import a statement with transactions

Given the account above.
When `POST /statements {bank_account_id, statement_number: "ST-1", opening_balance: "25000.00", closing_balance: "27000.00", currency_code: USD, transactions: [deposit 3000.00, withdrawal -1000.00]}`.
Then the statement is IMPORTED with transaction_count 2, total_credits
3,000.00, total_debits 1,000.00; opening + credits − debits must reconcile
to the closing balance; a duplicate `statement_number` is rejected.

### UC-BSY-03 — Statement/transaction sign convention

Given transactions `+3000.00` (deposit) and `-1000.00` (withdrawal).
Then types map CREDIT/DEBIT per the sign convention; amount 0 with
transaction_type FEE is allowed but never silently reclassified.

### UC-BSY-04 — Manual match with status guard

Given a RAW transaction of −1,200.00 and an unmatched GL entry of the same
amount.
When `POST /transactions/{id}/match {matched_entry_id, matched_entry_type: GL_ENTRY}`.
Then status MATCHED with matched ids set; matching an already
MATCHED/RECONCILED transaction → 409.

### UC-BSY-05 — Auto-match with tolerance

Given five unmatched transactions and three GL entries where two pairs match
exactly and one pair differs by 0.50.
When `POST /transactions/auto-match {bank_account_id, date_range_from, date_range_to, match_tolerance: 1.00, description_similarity_threshold: 0.8}`.
Then the three pairs match (tolerance 1.00 covers the 0.50 difference) and
`matched_count=3`, `remaining_unmatched=2`; with tolerance 0.10 only the two
exact pairs match.

### UC-BSY-06 — Ranked reconciliation suggestions

Given an unmatched transaction of −1,200.00 dated 2026-01-20 and model
rules for amount ±1% and ±2 days.
When `GET /transactions/{id}/suggestions`.
Then suggestions are ranked by confidence (0..1) with `model_id` and
`reason`; the GL entry of −1,204.00 within 2 days ranks above a −1,190.00
entry 5 days away.

### UC-BSY-07 — Start reconciliation, duplicate guard

Given account A and statement ST-1.
When `POST /reconciliations {bank_account_id, statement_id, reconciliation_date: 2026-01-31, book_balance: "26500.00"}`.
Then created DRAFT with `difference = book_balance − bank_balance`
(26,500 − 27,000 = −500.00); starting a second reconciliation for the same
account and date → 409.

### UC-BSY-08 — Complete requires zero difference

Given a reconciliation with difference −500.00.
When `POST /reconciliations/{id}/complete`.
Then 400 (unbalanced); after posting an outstanding withdrawal of 500.00 the
difference is 0.00 and completion succeeds (status COMPLETED, reconciled_at
set); completing an already-completed reconciliation → 409.

### UC-BSY-09 — Write-off adjustment

Given a matched transaction −20.00 (bank fee) with no GL counterpart.
When `POST /transactions/{id}/write-off {amount: "20.00", account_id: <fees>, reason: "bank fee"}`.
Then a ReconciliationAdjustment of 20.00 is created and the transaction can
complete reconciliation.

### UC-BSY-10 — Exchange-difference adjustment

Given a USD account, a EUR-origin invoice settled for 1,000.00 EUR when the
invoice was booked at rate 1.10 but settled at 1.08.
When `POST /transactions/{id}/exchange-difference {amount: "20.00", currency_code: USD, account_id, exchange_rate_id}`.
Then an adjustment of 20.00 USD captures the settlement FX difference
(1,000 × (1.10 − 1.08)) with the rate reference attached.

### UC-BSY-11 — Unreconcile

Given a RECONCILED transaction.
When `POST /transactions/{id}/unreconcile`.
Then status returns to MATCHED (link preserved), the reconciliation's
reconciled counters decrease, and the audit trail records the reversal.

### UC-BSY-12 — Reconciliation report

Given account activity over January 2026 (opening 25,000.00, deposits
3,000.00, withdrawals 1,500.00, 90% of transactions reconciled).
When `GET /reports/reconciliation?bank_account_id&period_start&period_end`.
Then opening/closing balances, totals, reconciled and unreconciled amounts,
and `reconciliation_percentage` 0.90 are returned.

### UC-BSY-13 — Cash position

Given accounts A (12,000.00) and B (8,000.00) with pending transactions of
−500.00.
When `GET /reports/cash-position?company_id&as_of_date`.
Then total_cash 20,000.00, available_balance 19,500.00, by_account lists
both, and pending_transactions lists the −500.00 item.

### UC-BSY-14 — Delete account guard

Given an account with transactions.
Then DELETE → 409; a closed/inactive account with no transactions → 204.

## Test coverage matrix

All scenarios: **not yet implemented** — the service has models and
migrations but every controller is a stub (`// TODO: Implement your business
logic here`); no `#[cfg(test)]` modules exist.

| Scenario | Existing test | Status |
|---|---|---|
| UC-BSY-01..14 | none | not yet implemented |

## Known drift / gaps

- **No business logic implemented at all** — models/migrations exist, all 24
  endpoints return stubs.
- OpenAPI status enums drift from the model comments (transaction
  RAW/MATCHED/RECONCILED/DISPUTED/FILTERED vs UNMATCHED/…/IGNORED;
  reconciliation DRAFT/IN_PROGRESS/BALANCED/IMBALANCED/COMPLETED vs
  DRAFT/IN_PROGRESS/RECONCILED/DISPUTED; statement 7 vs 4 statuses). The
  scenario suite should be implemented against the **OpenAPI** enums and the
  model comments aligned afterwards.
- Spec text says auto-match `match_tolerance` is "in seconds" — the schema
  intent is clearly an amount tolerance; fix the spec description.
- The reconciliation engine dossier
  (`docs/accounting/rules-engines/reconciliation-rules-engine.md`) defines the
  target rule behaviour these scenarios elaborate on.
