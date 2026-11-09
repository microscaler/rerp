# CRM-INS-03: crm/insights — Dashboards and widgets

**Service:** crm/insights · **Areas:** `dashboards` (10 operations) · **Build step:** 5 · **Depends on:** CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_25 · **Points:** 5

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_25 run against crm/insights instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/insights/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/insights/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/insights.yaml` — tables: `dashboards`, `dashboard_widgets`, `report_schedules`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_25_*.md` — what the screen expects (Prompt-built dashboard widgets).

## Rules for this area

Follow the operation descriptions below; they are the business rules.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_dashboards` | `GET /dashboards` | owner, admin, marketer, sales, read-only | — → 200 `DashboardPage` | Dashboards of the calling user. |
| `create_dashboard` | `POST /dashboards` | owner, admin, marketer, sales | `DashboardRequest` → 201 `Dashboard` | An empty dashboard; add widgets next. |
| `get_dashboard` | `GET /dashboards/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Dashboard` | Dashboard with its widgets. |
| `update_dashboard` | `PATCH /dashboards/{id}` | owner, admin, marketer, sales | `DashboardRequest` → 200 `Dashboard` | Rename or make default. |
| `delete_dashboard` | `DELETE /dashboards/{id}` | owner, admin, marketer, sales | — → 204 — | Removes the dashboard and its widgets. |
| `add_widget` | `POST /dashboards/{id}/widgets` | owner, admin, marketer, sales | `WidgetRequest` → 201 `DashboardWidget` | Add a widget from a metric query (or a natural-language prompt the server turns into one). |
| `update_widget` | `PATCH /dashboards/{id}/widgets/{widget_id}` | owner, admin, marketer, sales | `WidgetRequest` → 200 `DashboardWidget` | Change title, query, size or position. |
| `remove_widget` | `DELETE /dashboards/{id}/widgets/{widget_id}` | owner, admin, marketer, sales | — → 204 — | Remove the widget. |
| `get_widget_data` | `GET /dashboards/{id}/widgets/{widget_id}/data` | owner, admin, marketer, sales, read-only | — → 200 `WidgetData` | Run the widget query (cached a few minutes). |
| `schedule_widget` | `PUT /dashboards/{id}/widgets/{widget_id}/schedule` | owner, admin, marketer, sales | `ReportScheduleRequest` → 200 `ReportSchedule` | Daily / weekly / monthly email of the widget to recipients. |

## Steps

1. Services: `microservices/crm/insights/impl/src/services/dashboards.rs` (add `pub mod dashboards;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/dashboards.rs`.

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
