# Consolidation — Use-Case and Testing Scenarios

Date: 2026-10-05

- **Status**: PROPOSED
- **Owner**: accounting maintainers
- **Scope**: microservices/accounting/consolidation
- **Last reviewed**: 2026-10-05

Purpose: acceptance scenarios for consolidation groups, runs, elimination
rules/entries, and group reporting packs.

Related docs:

- [`standards/ifrs-reference.md`](standards/ifrs-reference.md) — international standards (IFRS/IAS).
- [`../rules-engines/consolidation-elimination-engine.md`](../rules-engines/consolidation-elimination-engine.md)
- [`../../ACCOUNTING_BDD_FEATURE_BACKLOG.md`](../../ACCOUNTING_BDD_FEATURE_BACKLOG.md)

## Standards reference

### IFRS (international)

| Standard | Relevance | URL |
|---|---|---|
| IFRS 10 — Consolidated Financial Statements | Control, group structure, uniform policies, eliminations | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-10-consolidated-financial-statements/> |
| IFRS 3 — Business Combinations | Acquisition accounting feeding group balances | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-3-business-combinations/> |
| IAS 21 — Effects of Changes in Foreign Exchange Rates | Translation of foreign operations into the reporting currency | <https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/> |
| IFRS 12 — Disclosure of Interests in Other Entities | Group disclosure data | <https://www.ifrs.org/issued-standards/list-of-standards/ifrs-12-disclosure-of-interests-in-other-entities/> |

## Endpoint inventory (10 operations)

Consolidation groups (list/create); consolidation runs (list/create/
execute); elimination rules (list/create); elimination entries (list);
group reporting packs (list/create).

## Use-case scenarios

### UC-CON-01 — Define a consolidation group

Given companies: parent P (holding), subsidiaries S1 and S2 (both wholly
owned).
When `POST /consolidation-groups {name: "Group P", reporting_currency_code: USD, company_ids: [P, S1, S2]}`.
Then created with the three members; a 2-char currency code, an empty
member list, or a duplicate member is rejected.

### UC-CON-02 — Execute a consolidation run (dry run first)

Given group "Group P" and fiscal period 2026-Q1.
When `POST /consolidation-runs {group_id, fiscal_period_id}` then
`POST /consolidation-runs/{id}/execute`.
Then status flows DRAFT → READY → RUNNING → COMPLETED; the run snapshots
member balances as at period end and records `executed_at`; a FAILED status
carries the failing member/rule reference.

### UC-CON-03 — Intercompany AR/AP elimination

Given S1 owes P 50,000.00 (AR on P's books, AP on S1's books).
When an elimination rule `INTERCOMPANY_AR_AP` runs for the pair (P, S1).
Then an elimination entry of 50,000.00 is generated with
`source_company_id=P, target_company_id=S1`, and both the AR and AP balances
drop out of the group total; a mismatch (P: 50,000.00 vs S1: 48,000.00)
surfaces as an unmatched variance of 2,000.00, never a silent net.

### UC-CON-04 — Intercompany revenue/expense elimination

Given P billed S1 100,000.00 of services in the period (P revenue, S1
expense).
When the `INTERCOMPANY_REVENUE_EXPENSE` rule runs.
Then group revenue and expenses are both reduced by 100,000.00; profit is
unchanged; the entry cites the invoice as source.

### UC-CON-05 — Investment/equity elimination

Given P's books carry the S1 investment at 1,000,000.00 and S1's equity is
1,000,000.00.
When the `INVESTMENT_EQUITY` rule runs.
Then both are eliminated; a difference (investment 1,000,000.00 vs equity
950,000.00) produces a 50,000.00 goodwill/OCI difference entry flagged for
accountant review (IFRS 3 mechanics, IFRS 10 elimination).

### UC-CON-06 — FX translation of a foreign subsidiary

Given S2 reports in EUR and the reporting currency is USD at period-end rate
0.92.
When the run translates S2's balances.
Then each balance is translated at the stated rate, the rate id/date is
stored with the run, and re-running with a different rate table changes the
result (no silent rate reuse across periods).

### UC-CON-07 — Run reproducibility

Given the same member snapshot data, rule versions, and rate table.
Then two executions produce identical elimination entries (deterministic),
and each entry links to the rule version that produced it.

### UC-CON-08 — Group reporting pack

Given a COMPLETED run.
When `POST /group-reporting-packs {run_id}`.
Then status GENERATING → READY with an `artifact_uri`; the pack is immutable
once READY (archival snapshot for the audit file).

### UC-CON-09 — Rule activation control

Given a CUSTOM elimination rule created inactive.
Then it never fires; activating it applies it from the next run onward —
historical runs keep their original rule set (versioned).

### UC-CON-10 — Boundary validation

Given page 0, limit 101, unknown rule_type, group with no members.
Then each is rejected 400; defaults page=1 limit=20.

### UC-CON-11 — Tenant isolation

Given groups of tenant A and tenant B with identical member ids.
Then tenant B sees only its own groups/runs/entries; cross-tenant group id →
404.

## Test coverage matrix

All scenarios: **not yet implemented** — contract-only service; all
controllers are stubs, no models/migrations/tests.

| Scenario | Existing test | Status |
|---|---|---|
| UC-CON-01..11 | none | not yet implemented |

## Known drift / gaps

- Contract-only: no eliminations, no status transitions, no numeric logic;
  `execute_consolidation_run` returns `status: {}` placeholders that are not
  even shape-correct for the spec enum.
- No migrations exist for consolidation tables.
- The elimination-engine dossier defines ownership and open questions
  (group-membership versioning, FX rate sourcing, run reproducibility) that
  these scenarios answer as acceptance targets.
