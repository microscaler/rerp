# CRM-INS-01: crm/insights — Read models and deal, pipeline and forecast reports

**Service:** crm/insights · **Areas:** `deal-reports`, `pipeline`, `forecast`, `conversion`, `win-loss`, `performance`, `analytics` (20 operations) · **Build step:** 5 · **Depends on:** CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_24 · **Points:** 8

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_24 run against crm/insights instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/insights/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/insights/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/insights.yaml` — tables: `rm_deals`, `rm_deal_stage_changes`, `rm_tasks`, `rm_conversions`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_24_*.md` — what the screen expects (Reports — hub, deals, contacts, email, revenue).

## Rules for this area

Insights never reads another service's tables: every report reads its own read models, kept at most 5 minutes behind by the consumers (CRM-INS-EV). Reports p95 ≤ 800 ms over 12 months.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `get_deal_forecast` | `GET /deals/forecast` | owner, admin, marketer, sales, read-only | — → 200 `DealForecastReport` | Expected revenue per period from open deals' expected close dates, weighted by the chosen basis, with historical accuracy. |
| `get_deal_funnel` | `GET /deals/funnel` | owner, admin, marketer, sales, read-only | — → 200 `DealFunnelReport` | Stage-by-stage progression for one pipeline over the window. |
| `get_deal_overview` | `GET /deals/overview` | owner, admin, marketer, sales, read-only | — → 200 `DealOverviewReport` | Totals, value histogram, owner and stage tables for deals in the window. |
| `get_deal_owner_overview` | `GET /deals/owner-overview` | owner, admin, marketer, sales, read-only | — → 200 `DealOwnerOverviewReport` | One row per deal owner: created, won, lost, value, average deal value, time to close, tasks completed. |
| `get_sales_performance` | `GET /deals/sales-performance` | owner, admin, marketer, sales, read-only | — → 200 `SalesPerformanceReport` | Matrix of open deals per stage per owner, plus won/lost, for one pipeline. |
| `get_pipeline_summary` | `GET /analytics/pipeline-summary` | owner, admin, marketer, sales, read-only | — → 200 `PipelineSummaryReport` | Get pipeline summary with stage breakdown |
| `get_weighted_pipeline` | `GET /analytics/pipeline-weighted` | owner, admin, marketer, sales, read-only | — → 200 `WeightedPipeline` | Get weighted pipeline revenue |
| `get_forecast_accuracy` | `GET /analytics/forecast/accuracy` | owner, admin, marketer, sales, read-only | — → 200 `ForecastAccuracyEntryPage` | Get forecast accuracy metrics |
| `get_monthly_forecast` | `GET /analytics/forecast/monthly` | owner, admin, marketer, sales, read-only | — → 200 `ForecastEntryPage` | Get monthly forecast |
| `get_quarterly_forecast` | `GET /analytics/forecast/quarterly` | owner, admin, marketer, sales, read-only | — → 200 `ForecastEntryPage` | Get quarterly forecast |
| `get_conversion_rates` | `GET /analytics/conversion-rates` | owner, admin, marketer, sales, read-only | — → 200 `ConversionRatePage` | Get conversion rate analytics |
| `get_funnel` | `GET /analytics/funnel` | owner, admin, marketer, sales, read-only | — → 200 `FunnelData` | List all analytics |
| `get_lead_source_effectiveness` | `GET /analytics/lead-sources` | owner, admin, marketer, sales, read-only | — → 200 `LeadSourceReportPage` | Get lead source analytics |
| `get_time_to_close` | `GET /analytics/time-to-close` | owner, admin, marketer, sales, read-only | — → 200 `TimeToCloseAnalysis` | Get time-to-close analytics |
| `get_win_loss_analysis` | `GET /analytics/win-loss` | owner, admin, marketer, sales, read-only | — → 200 `WinLossAnalysis` | Get win/loss analysis |
| `get_leaderboard` | `GET /analytics/leaderboard` | owner, admin, marketer, sales, read-only | — → 200 `LeaderboardEntryPage` | Get performance leaderboard |
| `get_rep_performance` | `GET /analytics/rep-performance` | owner, admin, marketer, sales, read-only | — → 200 `RepPerformance` | Get representative performance metrics |
| `get_team_performance` | `GET /analytics/team-performance` | owner, admin, marketer, sales, read-only | — → 200 `TeamPerformance` | Get team performance metrics |
| `get_automation_analytics` | `GET /analytics/automations` | owner, admin, marketer, sales, read-only | — → 200 `AutomationAnalytics` | Automations over the period. Query: `from`, `to`. |
| `get_list_growth` | `GET /analytics/list-growth` | owner, admin, marketer, sales, read-only | — → 200 `ListGrowthReport` | List growth over the period. Query: `from`, `to`. |

## Steps

1. Services: `microservices/crm/insights/impl/src/services/read_models.rs` (add `pub mod read_models;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/read_models.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/insights/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-7 **Rules.** Every 409 / 422 named in the descriptions above happens exactly in the stated case, with its reason.
- AC-8 **Portal.** The screens of the listed Epic 55 stories work against this service with the mock switched off for
  these paths.

## Test cases

| ID | Level | Case | Expected |
|---|---|---|---|
| TC-1 | Contract | Each operation with its request example | Response validates against the contract |
| TC-2 | Backend | Workspace B lists / gets / updates / deletes A's record | Empty / 404 / 404 / 404 |
| TC-3 | Backend | Each role × each write operation | Allowed exactly per `x-required-role` |
| TC-4 | Backend | One write per operation | One audit row, changed fields only |
| TC-7 | Backend | Each 409 / 422 case in the descriptions | The stated status and reason |

## Verify

```bash
python3 openapi/crm/tools/check_specs.py
cd microservices && cargo clippy -p rerp_crm_insights --all-targets -- -D warnings && cargo test -p rerp_crm_insights
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
