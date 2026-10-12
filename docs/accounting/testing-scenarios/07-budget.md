# Budget — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/budget
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for budget lifecycle (create → submit →
approve/confirm → complete → reopen/cancel), budget lines, variance vs
actuals, forecasts, and budget revisions.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

Budgeting itself is management accounting without a dedicated IFRS; the
budget-vs-actual comparison is audit evidence for two standards:

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 36 — Impairment of Assets | Material budget overruns are impairment indicators | <https://www.ifrs.org/issued-standards/list-of-standards/ias-36-impairment-of-assets/> |
| IAS 1 — Presentation of Financial Statements | Going-concern assessment uses budget/forecast evidence | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |

## Endpoint inventory (17 operations)

Budgets CRUD + submit/confirm/cancel/complete/reopen; budget lines
(list/add); variance report; forecasts (list/create); budget revisions
(list/create).

## Use-case scenarios

### UC-BUD-01 — Create budget with lines

Given fiscal year 2026.
When `POST /budgets {name: "Ops 2026", fiscal_year: 2026, version: 1, period_type: MONTHLY, currency_code: USD, company_id, budget_lines: [{gl_account_id, gl_account_code: "6200", period: "2026-01", amount: "15000.00"}]}`.
Then created DRAFT with `total_budget_amount = 15000.00`, a version row
(version 1, is_current), period row `2026-01`, and line budget_amount
15000.00; fiscal_year outside 2000–2100 or version 0 is rejected.

### UC-BUD-02 — DRAFT-only mutations

Given a SUBMITTED budget.
When PUT (replace lines), DELETE, or adding lines is attempted.
Then 409 "Cannot update/delete non-draft budgets"; the same calls on a DRAFT
budget succeed (line replace = full replacement semantics).

### UC-BUD-03 — Submit from DRAFT only

Given a DRAFT budget → `submit` returns SUBMITTED (200); submitting again,
or submitting a non-DRAFT budget → 409.

### UC-BUD-04 — Lifecycle confirm/complete/reopen/cancel

Given SUBMITTED → `confirm` makes it APPROVED/ACTIVE; `complete` closes it;
`reopen` returns it to an editable state; `cancel` is terminal. Each
transition is rejected from the wrong source state (e.g. complete on DRAFT).

### UC-BUD-05 — Variance report thresholds

Given line L1 budget 10,000.00 actual 9,500.00 (−5%), L2 budget 10,000.00
actual 12,500.00 (+25%), L3 budget 10,000.00 actual 10,000.00 (0%).
When `GET /variance?budget_id&period_start&period_end&warning_threshold=10&breach_threshold=20`.
Then L1 ON_TRACK, L2 BREACH, L3 ON_TRACK; total_variance = +2,000.00,
variance_percent +6.67%.
(Line variance sign: impl stores `actual − budget`; the OpenAPI description
says the reverse — see drift.)

### UC-BUD-06 — Variance percentage precision

Given budget 3,000.00, actual 2,900.00 → variance −100.00,
variance_percent −3.333333 (NUMERIC(19,6), 6 dp).

### UC-BUD-07 — Forecast creation

Given a DRAFT budget.
When `POST /forecasts {budget_id, name: "Q3 reforecast", forecast_type: ROLLING, period_start: 2026-07-01, period_end: 2026-09-30, currency_code: USD}`.
Then a Forecast (DRAFT) is created; forecasts are listed with filters
budget_id/forecast_type/status.

### UC-BUD-08 — Budget revision with line adjustments

Given an APPROVED budget with line L1 = 10,000.00.
When `POST /budget-revisions {budget_id, reason: "headcount change", line_adjustments: [{budget_line_id: L1, revised_amount: "12000.00"}], requested_by}`.
Then a revision (DRAFT → SUBMITTED → APPROVED → APPLIED lifecycle) is
created; on APPLIED the line amount becomes 12,000.00 and a new version is
cut.

### UC-BUD-09 — Unique keys

Given a budget with number `BUD-0001` and a (budget, version) pair (1).
Then a second budget with the same number, a duplicate version number, or a
duplicate (budget, account, period) line is rejected.

### UC-BUD-10 — Boundary validation

Given `fiscal_year: 1999`, `version: 0`, page 0, limit 101.
Then each is rejected; defaults page=1 limit=20.

### UC-BUD-11 — Line filters

Given lines across accounts 6200 (Jan) and 6300 (Feb) for two departments.
When `GET /budgets/{id}/lines?gl_account_id&period_from=2026-01&period_to=2026-02&department_id`.
Then only matching lines are returned in period order.

### UC-BUD-12 — Tenant isolation

Given budgets of tenant A and tenant B with the same name.
Then each tenant lists only its own; cross-tenant budget id → 404.

### UC-BUD-13 — Version history

Given versions 1 (superseded) and 2 (current).
Then `get_budget` returns version 2's lines; version 1 remains queryable for
audit with status SUPERSEDED and superseded_at set.

## Test coverage matrix

All scenarios: **not yet implemented** — models and migrations exist, all 17
endpoint controllers are stubs; no `#[cfg(test)]` modules.

| Scenario | Existing test | Status |
|---|---|---|
| UC-BUD-01..13 | none | not yet implemented |

## Known drift / gaps

- All controllers are stubs — e.g. `create_budget` returns an empty body
  while the spec promises 201 `BudgetWithLines`.
- OpenAPI budget status enum (DRAFT/SUBMITTED/APPROVED/ACTIVE/ARCHIVED/
  REJECTED) drifts from the model comment (DRAFT/ACTIVE/CLOSED/CANCELLED).
- `BudgetLine.variance` sign: model stores `actual − budget`; spec describes
  `(amount − actual_amount)` — opposite.
- No forecast entity exists in models (contract-only); no
  `budget_actuals` writer endpoint exists yet, so the variance report has no
  actual-data feed defined.
