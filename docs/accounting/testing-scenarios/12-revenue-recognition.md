# Revenue Recognition — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/revenue-recognition
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for recognition rules, recognition schedules,
deferred revenue/expense, and recognition runs (simulate and post).

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/revenue-recognition-engine.md`](../rules-engines/revenue-recognition-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IFRS 15 — Revenue from Contracts with Customers | Five-step model: identify contract, performance obligations, transaction price, allocation, recognition over time or at a point | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-15-revenue-from-contracts-with-customers/> |

## Endpoint inventory (10 operations)

Recognition rules (list/create); recognition schedules (list/create);
deferred revenues/expenses (list/create); recognition runs (list/create/
post).

Rule methods: STRAIGHT_LINE, MILESTONE, USAGE_BASED, PERCENT_COMPLETE,
MANUAL. Run status: DRAFT, READY, POSTED, REVERSED, FAILED.

## Use-case scenarios

### UC-REV-01 — Straight-line recognition over a contract term

Given an annual maintenance contract invoiced 12,000.00 on 2026-01-01 for
12 months (STRAIGHT_LINE).
When `POST /recognition-schedules {rule_id, source_invoice_id, total_amount: "12000.00", currency_code: USD}`.
Then the schedule has 12 periods of 1,000.00 each; at month 3 the
recognized_amount is 3,000.00 and deferred revenue is 9,000.00.

### UC-REV-02 — Deferred revenue mirroring

Given the schedule above after month 3.
Then the deferred-revenues list shows one item for the schedule with
balance 9,000.00, item_type REVENUE; creating a deferred item for a
nonexistent schedule is rejected.

### UC-REV-03 — Milestone recognition

Given a 3-milestone implementation project priced 30,000.00 (MILESTONE:
30/40/30 at design/go-live/handover).
Then recognition occurs only when a milestone is reached: after design
9,000.00 recognized; before go-live still 9,000.00 (no time-based
accrual).

### UC-REV-04 — Usage-based recognition

Given a USAGE_BASED rule tied to metered units (e.g., API calls) at
0.02/unit.
Then recognized revenue = units × rate per period, sourced from the usage
counter record; a period without counter data defers recognition and flags
a missing-source warning, never assumes zero usage.

### UC-REV-05 — Percent-complete with evidence

Given a PERCENT_COMPLETE construction contract 100,000.00 at 40% certified
completion.
Then 40,000.00 is recognized and the certification document (source
reference) is attached to the run; the percentage cannot exceed 100 or be
negative.

### UC-REV-06 — Recognition run simulation (dry run)

Given fiscal period 2026-03 with the schedules above.
When `POST /recognition-runs {fiscal_period_id}` (DRAFT) and the run is
simulated.
Then the run lists every schedule's computed amount with the rule version
used; nothing is posted — the GL is untouched (verify via trial balance
before/after).

### UC-REV-07 — Post a recognition run

Given a READY run with amounts: recognize 3,000.00 revenue and reclassify
3,000.00 out of deferred revenue.
When `POST /recognition-runs/{id}/post`.
Then status POSTED with `posted_journal_entry_id` set; the journal is
balanced (credit revenue 3,000.00 / debit deferred revenue 3,000.00);
posting a run whose fiscal period is CLOSED fails with FAILED status and a
reason.

### UC-REV-08 — Posting is idempotent per run

Given a POSTED run.
When post is called again.
Then rejected — a run posts exactly once; reversal is an explicit REVERSED
status flow that produces a mirror journal.

### UC-REV-09 — Rule activation

Given an inactive rule.
Then it cannot be selected for a new schedule; deactivating a rule in use
does not change existing schedules (they keep their rule snapshot/version).

### UC-REV-10 — Explain deferred balances

Given any schedule.
Then the service can explain the deferred balance: original amount, period
recognitions so far, remaining periods, and the rule/method — required for
both IFRS 15 disclosures and the rules-engine dossier.

### UC-REV-11 — Boundary validation

Given page 0, limit 101, total_amount negative, unknown method, missing
source_invoice_id.
Then each is rejected 400; defaults page=1 limit=20.

### UC-REV-12 — Tenant isolation

Given schedules of tenant A and tenant B for the same invoice id.
Then tenant B sees only its own schedules/runs/deferred items; cross-tenant
id → 404.

## Test coverage matrix

All scenarios: **not yet implemented** — contract-only service; all
controllers are stubs (`classify`-style handlers return empty/zeroed
structs), no services/models, no tests, no migrations.

| Scenario | Existing test | Status |
|---|---|---|
| UC-REV-01..12 | none | not yet implemented |

## Known drift / gaps

- Contract-only; the OpenAPI examples are inconsistent with their own
  schemas (e.g. `source_invoice_id: 1000.0` typed as uuid, `method:
  bank_transfer` for a recognition-method enum) — spec cleanup needed.
- `DeferredItem.item_type` includes EXPENSE — deferred **expense** (e.g.
  prepaid) recognition must follow the same engine (symmetric treatment).
- Five-step model artefacts (performance obligations, contract assets,
  variable consideration) are not yet represented in the contract — the
  schedule API is a simplified first cut.
