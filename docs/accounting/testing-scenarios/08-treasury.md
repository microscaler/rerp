# Treasury — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/treasury
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for cash positions, cash forecasts, liquidity
plans, bank relationships, and cash transfers.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 7 — Statement of Cash Flows | Cash/equivalents definition; forecast inputs mirror the statement structure | <https://www.ifrs.org/issued-standards/list-of-standards/ias-7-statement-of-cash-flows/> |
| IFRS 9 — Financial Instruments | Bank facilities and instruments measured per their category | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/> |
| IFRS 7 — Financial Instruments: Disclosures | Liquidity risk disclosures | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-7-financial-instruments-disclosures/> |
| IAS 21 — Effects of Changes in Foreign Exchange Rates | Multi-currency cash positions | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |

## Endpoint inventory (9 operations)

Cash positions (list); cash forecasts (list/create); liquidity plans
(list/create); bank relationships (list/create); cash transfers (list/create).

## Use-case scenarios

### UC-TRS-01 — Cash position snapshot

Given company C-1 with bank balances 25,000.00 available, 5,000.00
restricted (pledged deposit), and a 13-week net inflow projection of
+8,000.00.
When `GET /cash-positions?company_id&as_of_date=2026-03-17`.
Then `available_cash = 25000.00`, `restricted_cash = 5000.00`,
`forecast_net_cash = 8000.00`.

### UC-TRS-02 — Create 13-week cash forecast

Given company C-1.
When `POST /cash-forecasts {company_id, horizon: DAILY_13_WEEK, start_date: 2026-03-17, end_date: 2026-06-15}`.
Then created with the horizon enum validated; start > end is rejected; the
four horizons (DAILY_13_WEEK, MONTHLY_12_MONTH, QUARTERLY_8_QUARTER,
CUSTOM) are all representable.

### UC-TRS-03 — Liquidity plan with minimum cash threshold

Given company C-1.
When `POST /liquidity-plans {company_id, plan_date: 2026-03-17, minimum_cash_threshold: "10000.00"}`.
Then created DRAFT; listing filters by status (DRAFT/APPROVED/ACTIVE/
COMPLETED/CANCELLED); a negative threshold is rejected.

### UC-TRS-04 — Bank relationship registration

Given bank `BNP`.
When `POST /bank-relationships {bank_name: "BNP", relationship_type: CREDIT_FACILITY}`.
Then created ACTIVE; relationship types limited to OPERATING/CREDIT_FACILITY/
INVESTMENT/CASH_POOL/OTHER; an empty bank_name is rejected.

### UC-TRS-05 — Cash transfer lifecycle

Given source account A (12,000.00) and destination account B (8,000.00).
When `POST /cash-transfers {source_account_id: A, destination_account_id: B, amount: "2000.00", currency_code: USD}`.
Then created PLANNED; statuses flow PLANNED → APPROVED → SENT → SETTLED,
with CANCELLED/FAILED as terminal alternatives; amount ≤ 0, same
source/destination, or an amount above the source balance is rejected at
approval time.

### UC-TRS-06 — Forecast vs actual reconciliation

Given forecast F (13-week) and the actual closing balance at week 4.
Then the variance per week is derivable; a week whose inflow was promised by
a due invoice but unpaid appears as a variance entry with the source
(customer invoice) reference.

### UC-TRS-07 — Multi-currency positions

Given USD account (10,000.00) and EUR account (5,000.00) for the same
company.
When listing cash positions.
Then both appear with their currency codes; the caller may request a
presentation currency and the service must state which rate it used
(no silent mixing).

### UC-TRS-08 — Boundary validation

Given page 0, limit 101, unknown horizon, malformed date.
Then each is rejected 400; defaults page=1 limit=20.

### UC-TRS-09 — Tenant isolation

Given companies C-1 (tenant A) and C-2 (tenant B).
Then tenant B sees only C-2 positions/plans/transfers; C-1 ids resolve to
404.

### UC-TRS-10 — Contract-only status guard

Given the service is contract-only today (all controllers are stubs).
Then every endpoint returns scaffold data; the scenario suite is the
acceptance target for the first real implementation and must fail loudly
(not silently pass) until implemented.

## Test coverage matrix

All scenarios: **not yet implemented** — no models, no migrations, no tests;
`impl/src/` contains only generated stub controllers and `main.rs`.

| Scenario | Existing test | Status |
|---|---|---|
| UC-TRS-01..10 | none | not yet implemented |

## Known drift / gaps

- Contract-only service: no `models/`, no migrations, no `impl_registry.rs`
  (controllers wired through the gen registry), no business logic.
- Cross-service dependency: cash positions need `bank-sync` account balances
  and `accounts-payable`/`accounts-receivable` commitments — none of those
  feeds are wired yet.
