# Asset — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/asset
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for fixed-asset registration, depreciation
(single + bulk), revaluation, disposal, categories, asset models, pause/
resume, modification, validation, and the asset summary report.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 16 — Property, Plant and Equipment | Cost model, depreciation, disposals, revaluation | <https://www.ifrs.org/issued-standards/list-of-standards/ias-16-property-plant-and-equipment/> |
| IAS 36 — Impairment of Assets | Recoverable-amount checks when indicators exist | <https://www.ifrs.org/issued-standards/list-of-standards/ias-36-impairment-of-assets/> |
| IAS 23 — Borrowing Costs | Capitalisable borrowing costs on qualifying assets | <https://www.ifrs.org/issued-standards/list-of-standards/ias-23-borrowing-costs/> |
| IFRS 13 — Fair Value Measurement | Revaluation inputs and disclosures | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-13-fair-value-measurement/> |
| IAS 21 — Effects of Changes in Foreign Exchange Rates | Foreign-currency asset books | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |

## Endpoint inventory (21 operations)

Assets CRUD + depreciate + bulk-depreciate + validate + pause/resume +
modify; depreciation entries; revaluations; disposals; categories; asset
models; asset summary report.

## Use-case scenarios

### UC-AST-01 — Create asset with straight-line schedule

Given `POST /assets {acquisition_cost: "50000.00", acquisition_date: 2026-01-15, asset_number: "FA-100", name: "CNC Mill", useful_life_months: 60, salvage_value: "5000.00"}`.
Then a schedule is created: start 2026-01-15, end 2031-01-15,
total_depreciation 45,000.00, monthly_rate 1.666667% (6 dp), status ACTIVE,
currency USD (default).

### UC-AST-02 — Straight-line monthly amount

Given cost 50,000.00, salvage 5,000.00, life 60 months.
Then monthly depreciation = (50000 − 5000) / 60 = **750.00**; salvage above
cost is rejected; life < 1 month is rejected.

### UC-AST-03 — Declining balance monthly amount

Given book value 10,000.00 and annual rate 24%.
Then monthly = 10,000 × 24% / 12 = **200.00**.

### UC-AST-04 — Depreciate one period, floor at salvage

Given the asset above after 59 entries (accumulated 44,250.00).
When depreciating month 60 (750.00).
Then book_value = 5,000.00 (floor), status → FULLY_DEPRECIATED, entry POSTED;
an attempt to depreciate beyond cost − salvage is a Policy error; a paused
asset cannot be depreciated.

### UC-AST-05 — Bulk month-end depreciation

Given three assets: A ACTIVE (SL), B ACTIVE (declining balance), C paused,
D with method SUM_OF_YEARS_DIGITS.
When `POST /assets/bulk-depreciate {period: 2026-06}`.
Then A and B get entries, C is skipped, D appears in `errors[]` (method not
supported by bulk), and the summary reports entries/amounts/processed counts.

### UC-AST-06 — Disposal gain/loss math

Given asset cost 30,000.00, accumulated 25,000.00 (NBV 5,000.00).
When disposing with proceeds 10,000.00 (SALE).
Then `gain_loss = +5,000.00`, status DISPOSED, disposal_date set;
SCRAP/WRITE_OFF/THEFT → status SCRAPPED; disposing again → 409; negative
proceeds rejected.

### UC-AST-07 — Revaluation

Given asset current_value 50,000.00.
When `POST /revaluations {asset_id, new_value: "55,000.00" → 55000.00, revaluation_date, reason}`.
Then a RevaluationRecord (previous 50,000.00 → new 55,000.00) and an
AssetTransaction (type REVALUATION, amount +5,000.00) are written; new_value
negative or revaluing a DISPOSED/SCRAPPED asset is rejected.

### UC-AST-08 — Pause and resume depreciation

Given an ACTIVE asset.
When pause → is_paused true; pause again → 409; resume → is_paused false;
resume without pause → 409; either on a disposed asset → conflict.

### UC-AST-09 — Modify accounting values

Given asset useful_life 60, residual 5,000.00.
When `POST /assets/{id}/modify {effective_date, reason: "extension", new_useful_life_months: 72}`.
Then the schedule end_date and total_depreciation are recomputed, an
ADJUSTMENT transaction is written; a request with none of the three optional
values, a residual > cost, or an adjustment pushing book value below zero is
rejected.

### UC-AST-10 — Validate asset

Given an asset with cost 10,000.00, salvage 2,000.00, life 24, in_service ≥
purchase.
Then `validate` returns `{valid: true, issues: []}`; salvage > cost, life 0,
unknown method, or in_service < purchase each add an issue.

### UC-AST-11 — Categories and slug codes

Given category name "Office Equipment".
Then code = `OFFICE_EQUIPMENT` (alphanumerics uppercased, spaces → `_`,
≤ 50 chars, unique per tenant); a category with an unknown parent is
rejected.

### UC-AST-12 — Asset models

Given `POST /asset-models {name: "Fleet Vehicle", depreciation_method: STRAIGHT_LINE, useful_life_months: 96, residual_value_percent: 10}`.
Then created; residual_value_percent outside 0–100 or a duplicate name per
tenant is rejected.

### UC-AST-13 — Asset summary report

Given assets A (ACTIVE, cost 50,000.00, accum 10,000.00) and B (DISPOSED,
purchase_date before as_of).
When `GET /reports/asset-summary?company_id&as_of_date&include_disposed=false`.
Then totals cost/accum/NBV reflect A only, by_category/by_status counts are
correct, `currency_code` is the most frequent asset currency, and
depreciation-expense-current-year sums entries in the as_of calendar year.

### UC-AST-14 — Delete guards

Given an ACTIVE asset with no depreciation entries → delete succeeds (204);
an asset with entries, or any non-ACTIVE asset → 409.

### UC-AST-15 — Tenant isolation and company scope

Given assets for tenant A and tenant B.
Then tenant B only sees its own; a `company_id` that is not the caller's
legal entity is a Policy error.

## Test coverage matrix

| Scenario | Existing test | Status |
|---|---|---|
| UC-AST-01 | none (schedule creation untested) | not yet implemented |
| UC-AST-02 | `assets.rs::straight_line_monthly_amount`, `straight_line_rejects_salvage_above_cost` | implemented |
| UC-AST-03 | `assets.rs::declining_balance_monthly_amount` | implemented |
| UC-AST-04 | none (floor/policy rules untested) | not yet implemented |
| UC-AST-05 | none | not yet implemented |
| UC-AST-06 | `assets.rs::disposal_gain_loss_math` | implemented |
| UC-AST-07 | none | not yet implemented |
| UC-AST-08/09 | none | not yet implemented |
| UC-AST-10 | none | not yet implemented |
| UC-AST-11 | `assets.rs::category_slug_codes` | implemented |
| UC-AST-12 | none | not yet implemented |
| UC-AST-13 | none | not yet implemented |
| UC-AST-14/15 | none | not yet implemented |

## Known drift / gaps

- `post_to_gl` on revaluation is accepted but ignored — no journal entries
  are produced by the asset service today.
- IAS 36 impairment (recoverable-amount testing) has no endpoint; the
  revaluation endpoint is a fair-value adjustment, not an impairment test.
- Decimal parsing rounds to 4 dp; storage NUMERIC(19,4) except
  `depreciation_rate` NUMERIC(19,6).
- SYD / units-of-production / FX-adjustment methods exist in the enum but
  bulk depreciation rejects them; only SL and declining balance compute.
