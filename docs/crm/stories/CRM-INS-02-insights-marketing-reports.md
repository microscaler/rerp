# CRM-INS-02: crm/insights — Contact, email, automation, task and revenue reports

**Service:** crm/insights · **Areas:** `contact-reports`, `email-reports`, `automation-reports`, `task-reports`, `revenue-reports`, `reports` (19 operations) · **Build step:** 5 · **Depends on:** CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_13, US_55_24 · **Points:** 8

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_13, US_55_24 run against crm/insights instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/insights/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/insights/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/insights.yaml` — tables: `rm_contacts`, `rm_messages`, `rm_enrolments`, `rm_list_daily`, `saved_reports`, `report_schedules`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_13_*.md` — what the screen expects (Automation map, enrolments and report).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_24_*.md` — what the screen expects (Reports — hub, deals, contacts, email, revenue).

## Rules for this area

Follow the operation descriptions below; they are the business rules.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `get_field_breakdown` | `GET /contacts/field-breakdown` | owner, admin, marketer, sales, read-only | — → 200 `FieldBreakdownReport` | Records per value of one field, for records created in the window. |
| `get_nearby_contacts` | `GET /contacts/nearby` | owner, admin, marketer, sales, read-only | — → 200 `NearbyContactPage` | Contacts within a radius of an address or point (geocoded from the contact address). |
| `get_contact_performance` | `GET /contacts/performance` | owner, admin, marketer, sales, read-only | — → 200 `ContactPerformanceReport` | Total / active / inactive vs previous period, trends, top lists and tags. |
| `get_tag_trends` | `GET /contacts/tag-trends` | owner, admin, marketer, sales, read-only | — → 200 `TagTrendsReport` | Tag applications over time; a window of up to 3 months keeps it fast. |
| `get_email_client_trends` | `GET /email/client-trends` | owner, admin, marketer, sales, read-only | — → 200 `EmailClientTrendsReport` | Opens per email client / device family in the window. |
| `get_open_trends` | `GET /email/open-trends` | owner, admin, marketer, sales, read-only | — → 200 `OpenTrendsReport` | Opens by weekday and by hour of day. |
| `get_predictive_content` | `GET /email/predictive-content` | owner, admin, marketer, sales, read-only | — → 200 `PredictiveContentReport` | Sends, clicks and click-to-open per content variant across campaigns. |
| `list_automation_reports` | `GET /automations` | owner, admin, marketer, sales, read-only | — → 200 `AutomationListRowPage` | One row per automation with contacts entered; filter by folder. |
| `get_jump_to_tracking` | `GET /automations/jump-to` | owner, admin, marketer, sales, read-only | — → 200 `JumpToRowPage` | Every GOAL step across automations: how many contacts jumped to it vs passed it. |
| `get_one_to_one_emails` | `GET /automations/one-to-one-emails` | owner, admin, marketer, sales, read-only | — → 200 `OneToOneEmailRowPage` | SEND_1TO1_EMAIL steps across automations with opens, clicks, replies and bounces. |
| `get_automation_contacts` | `GET /automations/{id}/contacts` | owner, admin, marketer, sales, read-only | — → 200 `AutomationContactsReport` | Completion KPIs, top tags / lists, completion-time histogram and the contact list. |
| `get_automation_goals` | `GET /automations/{id}/goals` | owner, admin, marketer, sales, read-only | — → 200 `AutomationGoalsReport` | Achieved / skipped / waiting per goal step. |
| `get_automation_overview` | `GET /automations/{id}/overview` | owner, admin, marketer, sales, read-only | — → 200 `AutomationOverviewReport` | KPI tiles, sends/opens/clicks over time and per-message rows for one automation. |
| `get_task_overview` | `GET /tasks/overview` | owner, admin, marketer, sales, read-only | — → 200 `TaskOverviewReport` | Open/overdue/completed tasks and outcome sentiment per rep and per task type. |
| `get_marketing_revenue` | `GET /revenue/marketing` | owner, admin, marketer, sales, read-only | — → 200 `MarketingRevenueReport` | Revenue attributed to campaigns and automation messages from conversions within the attribution window. |
| `build_report` | `POST /reports/build` | owner, admin, marketer, sales | `ReportBuildRequest` → 200 inline object | Build a custom report |
| `export_report` | `POST /reports/export` | owner, admin, marketer, sales | inline object → 200 inline object | Export report data to a file |
| `get_saved_report` | `GET /reports/{id}` | owner, admin, marketer, sales, read-only | — → 200 `SavedReport` | Get reports by ID |
| `schedule_report` | `POST /reports/{id}/schedule` | owner, admin, marketer, sales | `ReportScheduleRequest` → 201 `ReportSchedule` | Schedule a report to run on a recurring basis |

## Steps

1. Services: `microservices/crm/insights/impl/src/services/marketing_reports.rs` (add `pub mod marketing_reports;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/marketing_reports.rs`.
4. Idempotency: `export_report`, `schedule_report` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/insights/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-6 **Idempotency.** Repeating `export_report`, `schedule_report` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
| TC-6 | Backend | Repeat an idempotent command with the same key / same key, other body | Same response, one effect / 409 |
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
