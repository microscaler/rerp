# CRM-SAL-04: crm/sales — Tasks, task types, outcomes, calendars and saved responses

**Service:** crm/sales · **Areas:** `tasks`, `task-types`, `outcomes`, `calendars`, `saved-responses` (43 operations) · **Build step:** 4 · **Depends on:** CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_19, US_55_20, US_55_22 · **Points:** 16

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_19, US_55_20, US_55_22 run against crm/sales instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/sales/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/sales/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/sales.yaml` — tables: `tasks`, `task_types`, `task_outcomes`, `task_type_outcomes`, `task_settings`, `calendar_feeds`, `saved_responses`, `saved_response_categories`.
- `openapi/crm/docs/10-events.md` — the events published here: `task.completed`, `task.created`, `task.reopened`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_19_*.md` — what the screen expects (Deal detail).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_20_*.md` — what the screen expects (Pipelines and deal settings).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_22_*.md` — what the screen expects (Tasks and task settings).

## Rules for this area

Completing a task may require an outcome (task settings). Calendar feed tokens are stored hashed and served by the edge from a signed snapshot.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_tasks` | `GET /tasks` | owner, admin, marketer, sales, read-only | — → 200 `TaskPage` | List tasks. Paged (`page`, `limit` ≤ 100). Filters: `search`, `status`, `related_kind`, `related_id`, `contact_id`, `deal_id`, `type_id`, `owner_id`, `outcome_id`, `sentiment`, `due_from`, `due_to`, `overdue`, `sort`, `dir`. |
| `create_task` | `POST /tasks` | owner, admin, sales | `CreateTaskRequest` → 201 `Task` | Create a task. Audited. Publishes `task.created` on `crm.sales.v1`. |
| `bulk_update_tasks` | `POST /tasks/bulk` | owner, admin, sales | inline object → 200 inline object | Complete, reassign, reschedule or delete many tasks. Audited. Publishes `task.created` on `crm.sales.v1`. |
| `task_overview_report` | `GET /tasks/overview` | owner, admin, marketer, sales, read-only | — → 200 `TaskOverview` | The task overview report: totals, series over time, breakdowns by status/type/outcome/sentiment, outcomes by rep. Query: `related_kind`, `type_id`, `outcome_id`, `owner_id`, `group_id`, `created_from`, `created_to`, `due_from`, `due_to`, `completed_from`, `completed_to`. |
| `get_task` | `GET /tasks/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Task` | Get a task. 404 when the record does not exist in this workspace. |
| `update_task` | `PUT /tasks/{id}` | owner, admin, sales | `CreateTaskRequest` → 200 `Task` | Replace a task. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_task` | `PATCH /tasks/{id}` | owner, admin, sales | `UpdateTaskRequest` → 200 `Task` | Update part of a task. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_task` | `DELETE /tasks/{id}` | owner, admin, sales | — → 204 — | Delete a task. Audited. 404 when the record does not exist in this workspace. |
| `complete_task` | `POST /tasks/{id}/complete` | owner, admin, sales | `CompleteTaskRequest` → 200 `Task` | 422 OUTCOME_REQUIRED when settings.require_outcomes is on and no outcome_id is given; 422 OUTCOME_NOT_ALLOWED when the outcome is not one of the task type's. Publishes `task.completed` on `crm.sales.v1`. |
| `reopen_task` | `POST /tasks/{id}/reopen` | owner, admin, sales | — → 200 `Task` | Reopen a completed task (clears the outcome). A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `task.reopened` on `crm.sales.v1`. |
| `list_task_types` | `GET /task-types` | owner, admin, marketer, sales, read-only | — → 200 `TaskTypePage` | List task types. Paged (`page`, `limit` ≤ 100). Filters: `search`, `active`. |
| `create_task_type` | `POST /task-types` | owner, admin | `TaskTypeRequest` → 201 `TaskType` | Create a task type. Audited. |
| `get_task_type` | `GET /task-types/{id}` | owner, admin, marketer, sales, read-only | — → 200 `TaskType` | Get a task type. 404 when the record does not exist in this workspace. |
| `update_task_type` | `PUT /task-types/{id}` | owner, admin | `TaskTypeRequest` → 200 `TaskType` | Replace a task type. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_task_type` | `PATCH /task-types/{id}` | owner, admin | `TaskTypeRequest` → 200 `TaskType` | Update part of a task type. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_task_type` | `DELETE /task-types/{id}` | owner, admin | — → 204 — | Delete a task type. Audited. 404 when the record does not exist in this workspace. |
| `reorder_type` | `POST /task-types/{id}/reorder` | owner, admin | inline object → 200 `TaskType` | Move a task type to a position. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_outcomes` | `GET /task-outcomes` | owner, admin, marketer, sales, read-only | — → 200 `OutcomePage` | List outcomes. Paged (`page`, `limit` ≤ 100). Filters: `search`, `sentiment`. |
| `create_outcome` | `POST /task-outcomes` | owner, admin | `OutcomeRequest` → 201 `Outcome` | Create a outcome. Audited. |
| `get_outcome` | `GET /task-outcomes/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Outcome` | Get a outcome. 404 when the record does not exist in this workspace. |
| `update_outcome` | `PUT /task-outcomes/{id}` | owner, admin | `OutcomeRequest` → 200 `Outcome` | Replace a outcome. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_outcome` | `PATCH /task-outcomes/{id}` | owner, admin | `OutcomeRequest` → 200 `Outcome` | Update part of a outcome. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_outcome` | `DELETE /task-outcomes/{id}` | owner, admin | — → 204 — | Delete a outcome. Audited. 404 when the record does not exist in this workspace. |
| `list_calendar_feeds` | `GET /calendars` | owner, admin, marketer, sales, read-only | — → 200 `CalendarFeedPage` | List calendars. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_calendar_feed` | `POST /calendars` | owner, admin, sales | `CalendarFeedRequest` → 201 `CalendarFeed` | Create a calendar. Audited. |
| `get_calendar_feed` | `GET /calendars/{id}` | owner, admin, marketer, sales, read-only | — → 200 `CalendarFeed` | Get a calendar. 404 when the record does not exist in this workspace. |
| `update_calendar_feed` | `PUT /calendars/{id}` | owner, admin, sales | `CalendarFeedRequest` → 200 `CalendarFeed` | Replace a calendar. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_calendar_feed` | `PATCH /calendars/{id}` | owner, admin, sales | `CalendarFeedRequest` → 200 `CalendarFeed` | Update part of a calendar. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_calendar_feed` | `DELETE /calendars/{id}` | owner, admin, sales | — → 204 — | Delete a calendar. Audited. 404 when the record does not exist in this workspace. |
| `regenerate_calendar_token` | `POST /calendars/{id}/regenerate` | owner, admin, sales | — → 200 `CalendarFeed` | Issue a new token (the old URL stops working). A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_saved_response_categories` | `GET /saved-response-categories` | owner, admin, marketer, sales, read-only | — → 200 `SavedResponseCategoryPage` | List saved-response categories. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_saved_response_category` | `POST /saved-response-categories` | owner, admin | `SavedResponseCategory` → 201 `SavedResponseCategory` | Create a saved-response category. Audited. |
| `get_saved_response_category` | `GET /saved-response-categories/{id}` | owner, admin, marketer, sales, read-only | — → 200 `SavedResponseCategory` | Get a saved-response category. 404 when the record does not exist in this workspace. |
| `update_saved_response_category` | `PUT /saved-response-categories/{id}` | owner, admin | `SavedResponseCategoryInput` → 200 `SavedResponseCategory` | Replace a saved-response category. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_saved_response_category` | `PATCH /saved-response-categories/{id}` | owner, admin | `SavedResponseCategoryInput` → 200 `SavedResponseCategory` | Update part of a saved-response category. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_saved_response_category` | `DELETE /saved-response-categories/{id}` | owner, admin | — → 204 — | Delete a saved-response category. Audited. 404 when the record does not exist in this workspace. |
| `list_saved_responses` | `GET /saved-responses` | owner, admin, marketer, sales, read-only | — → 200 `SavedResponsePage` | List saved_responses. Paged (`page`, `limit` ≤ 100). Filters: `search`, `category_id`. |
| `create_saved_response` | `POST /saved-responses` | owner, admin, sales | `SavedResponseRequest` → 201 `SavedResponse` | Create a saved_response. Audited. |
| `get_saved_response` | `GET /saved-responses/{id}` | owner, admin, marketer, sales, read-only | — → 200 `SavedResponse` | Get a saved_response. 404 when the record does not exist in this workspace. |
| `update_saved_response` | `PUT /saved-responses/{id}` | owner, admin, sales | `SavedResponseRequest` → 200 `SavedResponse` | Replace a saved_response. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_saved_response` | `PATCH /saved-responses/{id}` | owner, admin, sales | `SavedResponseRequest` → 200 `SavedResponse` | Update part of a saved_response. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_saved_response` | `DELETE /saved-responses/{id}` | owner, admin, sales | — → 204 — | Delete a saved_response. Audited. 404 when the record does not exist in this workspace. |
| `render_saved_response` | `POST /saved-responses/{id}/render` | owner, admin, sales | inline object → 200 inline object | Render the response for a contact (merge fields resolved), for the 1:1 composer. A state change on one resource; audited. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/sales/impl/src/services/tasks.rs` (add `pub mod tasks;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/tasks.rs`.
4. Idempotency: `bulk_update_tasks`, `regenerate_calendar_token` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/sales/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `task.completed`, `task.created`, `task.reopened` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `bulk_update_tasks`, `regenerate_calendar_token` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
| TC-5 | Backend | Each publishing operation; then the same with a forced rollback | One outbox row each; none after rollback |
| TC-6 | Backend | Repeat an idempotent command with the same key / same key, other body | Same response, one effect / 409 |
| TC-7 | Backend | Each 409 / 422 case in the descriptions | The stated status and reason |

## Verify

```bash
python3 openapi/crm/tools/check_specs.py
cd microservices && cargo clippy -p rerp_crm_sales --all-targets -- -D warnings && cargo test -p rerp_crm_sales
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
