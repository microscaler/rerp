# CRM-SAL-01: crm/sales — Pipelines, stages and deal settings

**Service:** crm/sales · **Areas:** `pipelines`, `stages`, `settings`, `deal-roles` (23 operations) · **Build step:** 4 · **Depends on:** CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_18, US_55_19, US_55_20, US_55_22 · **Points:** 9

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_18, US_55_19, US_55_20, US_55_22 run against crm/sales instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/sales/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/sales/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/sales.yaml` — tables: `pipelines`, `stages`, `deal_settings`, `deal_roles`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_18_*.md` — what the screen expects (Deals board and new deal).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_19_*.md` — what the screen expects (Deal detail).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_20_*.md` — what the screen expects (Pipelines and deal settings).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_22_*.md` — what the screen expects (Tasks and task settings).

## Rules for this area

A stage with deals cannot be deleted (409 `in_use`); moving stages keeps sequence unique per pipeline.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `get_deal_settings` | `GET /deal-settings` | owner, admin, marketer, sales, read-only | — → 200 `DealSettings` | Deal settings and the win-probability readiness per pipeline. |
| `update_deal_settings` | `PUT /deal-settings` | owner, admin | `DealSettings` → 200 `DealSettings` | Update deal settings. Replaces the resource with the body; audited. |
| `list_pipelines` | `GET /pipelines` | owner, admin, marketer, sales, read-only | — → 200 `PipelinePage` | List pipelines. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_pipeline` | `POST /pipelines` | owner, admin | `PipelineRequest` → 201 `Pipeline` | Create a pipeline. Audited. |
| `get_pipeline` | `GET /pipelines/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Pipeline` | Get a pipeline. 404 when the record does not exist in this workspace. |
| `update_pipeline` | `PUT /pipelines/{id}` | owner, admin | `PipelineRequest` → 200 `Pipeline` | Replace a pipeline. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_pipeline` | `PATCH /pipelines/{id}` | owner, admin | `PipelineRequest` → 200 `Pipeline` | Update part of a pipeline. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_pipeline` | `DELETE /pipelines/{id}` | owner, admin | — → 204 — | Delete a pipeline. Audited. 404 when the record does not exist in this workspace. |
| `get_pipeline_summary` | `GET /pipelines/{id}/summary` | owner, admin, marketer, sales, read-only | — → 200 `PipelineSummary` | Open deals, value, weighted value, won this month, stalled, per stage. 404 when the record does not exist in this workspace. |
| `list_stages` | `GET /stages` | owner, admin, marketer, sales, read-only | — → 200 `StagePage` | List all stages |
| `create_stage` | `POST /stages` | owner, admin | `CreateStageRequest` → 201 `Stage` | Create a new stages |
| `get_stage` | `GET /stages/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Stage` | Get stages by ID |
| `update_stage` | `PUT /stages/{id}` | owner, admin | `UpdateStageRequest` → 200 `Stage` | Update stages by ID |
| `delete_stage` | `DELETE /stages/{id}` | owner, admin | — → 204 — | Delete stages by ID |
| `patch_stage` | `PATCH /stages/{id}` | owner, admin | `UpdateStageRequest` → 200 `Stage` | Change some of a stage's settings. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `get_task_settings` | `GET /task-settings` | owner, admin, marketer, sales, read-only | — → 200 `TaskSettings` | Task settings. |
| `update_task_settings` | `PUT /task-settings` | owner, admin | `TaskSettings` → 200 `TaskSettings` | 422 TYPES_WITHOUT_OUTCOMES (listing them) when require_outcomes is turned on while an active type has no outcomes. |
| `list_deal_roles` | `GET /deal-roles` | owner, admin, marketer, sales, read-only | — → 200 `DealRolePage` | List deal roles. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_deal_role` | `POST /deal-roles` | owner, admin | `DealRoleRequest` → 201 `DealRole` | Create a deal role. Audited. |
| `get_deal_role` | `GET /deal-roles/{id}` | owner, admin, marketer, sales, read-only | — → 200 `DealRole` | Get a deal role. 404 when the record does not exist in this workspace. |
| `update_deal_role` | `PUT /deal-roles/{id}` | owner, admin | `DealRoleRequest` → 200 `DealRole` | Replace a deal role. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `patch_deal_role` | `PATCH /deal-roles/{id}` | owner, admin | `DealRoleRequest` → 200 `DealRole` | Update part of a deal role. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_deal_role` | `DELETE /deal-roles/{id}` | owner, admin | — → 204 — | Delete a deal role. Audited. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/sales/impl/src/services/pipelines.rs` (add `pub mod pipelines;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/pipelines.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/sales/openapi.yaml`
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
cd microservices && cargo clippy -p rerp_crm_sales --all-targets -- -D warnings && cargo test -p rerp_crm_sales
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
