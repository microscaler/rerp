# Financial Reports — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/financial-reports
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for the statement generators (balance sheet,
income statement, cash flow, general-ledger report, trial balance), custom
report definitions and executions, report definitions with expression lines,
drill-down, exports, and statutory report packs.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/report-expression-engine.md`](../rules-engines/report-expression-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IAS 1 — Presentation of Financial Statements | Balance sheet + income statement structure | <https://www.ifrs.org/issued-standards/list-of-standards/ias-1-presentation-of-financial-statements/> |
| IAS 7 — Statement of Cash Flows | Operating/investing/financing classification | <https://www.ifrs.org/issued-standards/list-of-standards/ias-7-statement-of-cash-flows/> |
| IAS 8 — Accounting Policies, Changes in Estimates and Errors | Report definitions are versioned; historical reports immutable | <https://www.ifrs.org/issued-standards/list-of-standards/ias-8-accounting-policies-changes-in-accounting-estimates-and-errors/> |
| IFRS 18 — Presentation and Disclosure in Financial Statements (forward) | Replaces IAS 1 for periods from 2027; statement structure changes | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-18-presentation-and-disclosure-in-financial-statements/> |

## Endpoint inventory (19 operations)

Report generators (balance-sheet, income-statement, cash-flow,
general-ledger, trial-balance); custom-reports CRUD + execute; report
executions; report definitions + lines; drill-down; report exports;
statutory report packs.

## Use-case scenarios

### UC-FR-01 — Balance sheet structure and balancing

Given period-end 2026-12-31: cash 25,000.00, AR 12,000.00, equipment
40,000.00 (accum 10,000.00), AP 8,000.00, share capital 50,000.00, retained
earnings 9,000.00.
When `GET /reports/balance-sheet?company_id&period_end=2026-12-31`.
Then assets: current 37,000.00, non-current 30,000.00, total 67,000.00;
liabilities 8,000.00; equity 59,000.00; assets == liabilities + equity;
`include_detail=true` adds `account_details[]`.

### UC-FR-02 — Income statement and net margin

Given revenue 120,000.00 (sales 100,000.00 + service 20,000.00), COGS
60,000.00, operating expenses 25,000.00, other income/expense −1,000.00,
tax 8,500.00.
When `GET /reports/income-statement?company_id&period_start&period_end`.
Then gross_profit 60,000.00, operating_income 35,000.00, net_income
25,500.00, net_margin 21.25%; `compare_previous_period=true` adds the
comparative column.

### UC-FR-03 — Cash flow classification

Given transactions: customer receipts 50,000.00 (OPERATING), vendor payments
30,000.00 (OPERATING), equipment purchase 10,000.00 (INVESTING), loan draw
5,000.00 (FINANCING).
When `GET /reports/cash-flow`.
Then operating 20,000.00, investing −10,000.00, financing 5,000.00,
net_cash_flow 15,000.00, and beginning + net == ending cash; each
`account_details[]` line carries its activity_type.

### UC-FR-04 — Trial balance and general-ledger reports

Given the GL service data from `01-general-ledger.md` UC-GL-05.
Then the trial-balance report matches the GL service's trial balance
(totals, per-account balances, `is_balanced=true`); the general-ledger
report shows running balance per entry and `total_debits == total_credits`.

### UC-FR-05 — Custom report CRUD and execute

Given `POST /custom-reports {name: "Monthly OPEX by dept", type: AGGREGATION, company_id, definition: {...}}`.
Then CRUD works (get/update/delete), and
`POST /custom-reports/{id}/execute {period_start, period_end, currency_code}`.
returns a ReportExecution PENDING → RUNNING → COMPLETED with `data`;
executing a deleted report → 404; an unknown `type` is rejected at create.

### UC-FR-06 — Expression-line report definition

Given a definition with lines: L1 ACCOUNT_BALANCE (account group
"Revenue"), L2 ACCOUNT_BALANCE (group "Expenses"), L3 FORMULA (L1 − L2),
L4 CONSTANT ("Net result").
When executed over a period.
Then L3 = L1 − L2 with L4 as its label; a line referencing an unknown line,
or a cycle (L3 → L4 → L3), fails validation with the offending reference —
the definition never executes.

### UC-FR-07 — Drill-down from a cell

Given an executed report with cell L3 = 25,500.00.
When `GET /report-cells/{id}/drill-down`.
Then source lines are listed (source_type JOURNAL_ITEM/ACCOUNT, amounts,
descriptions) and their sum equals the cell value.

### UC-FR-08 — Definition versioning and immutability

Given definition v1 executed in January.
When the definition is changed (v2) and re-executed in February.
Then January's execution keeps v1's numbers and expression snapshot — no
retroactive mutation (IAS 8); v2 execution shows the new structure.

### UC-FR-09 — Report export

Given a COMPLETED execution.
When `POST /report-exports {report_execution_id, format: XLSX}`.
Then 202 QUEUED → READY with `artifact_uri`; exporting a non-completed
execution fails; formats PDF/XLSX/CSV/JSON are accepted.

### UC-FR-10 — Statutory report pack

Given jurisdiction `ID` and report type per local regulation.
When listing statutory report packs.
Then packs show DRAFT → GENERATING → READY → FILED → ARCHIVED lifecycle with
`jurisdiction_code`; only a READY pack can transition to FILED.

### UC-FR-11 — Read-only evaluation

Given any report execution.
Then evaluation must never write to the GL/entity tables — the report is a
pure read; asserting this via RLS (read-only role) is part of acceptance.

### UC-FR-12 — Boundary validation

Given page 0, limit 101, name > 200 chars, unknown format, malformed date.
Then each is rejected 400; defaults page=1 limit=20.

### UC-FR-13 — Tenant isolation

Given report definitions of tenant A and tenant B with the same name.
Then each tenant sees only its own definitions/executions/exports.

### UC-FR-14 — Determinism

Given identical definition version, options, and source-data window.
Then two executions produce identical outputs (no wall-clock leakage).

## Test coverage matrix

All scenarios: **not yet implemented** — no `#[cfg(test)]` modules; models and
migrations exist (financial_reports, report_data, report_schedules,
report_templates) but all 19 controllers are stubs returning placeholders
(e.g. `net_income: 3.14`).

| Scenario | Existing test | Status |
|---|---|---|
| UC-FR-01..14 | none | not yet implemented |

## Known drift / gaps

- All controllers are stubs; the five statement generators are declared as
  GET with a requestBody (unusual) — the generator endpoints need a spec
  cleanup pass before implementation.
- The expression engine is design-only
  (`docs/accounting/rules-engines/report-expression-engine.md`): expression
  categories ACCOUNT_BALANCE/JOURNAL_SUM/FORMULA/CONSTANT/SOURCE_LINK are
  defined there and referenced by UC-FR-06/07.
- IFRS 18 will restructure the income statement (operating/investing/
  financing categories with new subtotals); the report engine should model
  statement layouts as versioned templates (`report_templates` table already
  exists for this).
