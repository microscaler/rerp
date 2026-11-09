# CRM-AUT-02: crm/automation — Enrolments, the step engine and timers

**Service:** crm/automation · **Areas:** `enrolments`, `actions`, `automation-execution`, `rules` (20 operations) · **Build step:** 3 · **Depends on:** CRM-AUT-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_03, US_55_04, US_55_12, US_55_13, US_55_19 · **Points:** 8

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_03, US_55_04, US_55_12, US_55_13, US_55_19 run against crm/automation instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/automation/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/automation/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/automation.yaml` — tables: `enrolments`, `enrolment_steps`, `timers`, `scheduled_actions`, `workflow_runs`, `workflow_rules`.
- `openapi/crm/docs/10-events.md` — the events published here: `enrolment.started`, `enrolment.stopped`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_03_*.md` — what the screen expects (Contacts list, filters and bulk actions).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_04_*.md` — what the screen expects (Contact profile and activity timeline).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_12_*.md` — what the screen expects (Automations and the automation builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_13_*.md` — what the screen expects (Automation map, enrolments and report).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_19_*.md` — what the screen expects (Deal detail).

## Rules for this area

Exactly once per enrolment step: claim the step in a transaction, run it, record the outcome. Timers are released within 60 s of due. Sends go to messaging with the enrolment step as the idempotency key. The Gherkin scenarios in `openapi/crm/automation/features/` are the acceptance tests.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_contact_enrolments` | `GET /contacts/{id}/enrolments` | owner, admin, marketer, sales, read-only | — → 200 inline object | Every workflow a contact is or was in. 404 when the record does not exist in this workspace. |
| `list_deal_enrolments` | `GET /deals/{id}/enrolments` | owner, admin, marketer, sales, read-only | — → 200 `EnrolmentPage` | Automations a deal is in. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `get_enrolment` | `GET /enrolments/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Enrolment` | One contact's progress through a workflow. 404 when the record does not exist in this workspace. |
| `jump_enrolment` | `POST /enrolments/{id}/jump` | owner, admin, marketer | inline object → 200 `Enrolment` | Move a contact to a given step now. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `retry_enrolment` | `POST /enrolments/{id}/retry` | owner, admin, marketer | — → 202 `Enrolment` | 409 unless the enrolment is FAILED. Earlier steps are not repeated. |
| `stop_enrolment` | `POST /enrolments/{id}/stop` | owner, admin, marketer | — → 200 `Enrolment` | Remove a contact from the workflow. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `enrolment.stopped` on `crm.automation.v1`. |
| `list_workflow_enrolments` | `GET /workflows/{id}/enrolments` | owner, admin, marketer, sales, read-only | — → 200 `EnrolmentPage` | Contacts enrolled in a workflow. Paged (`page`, `limit` ≤ 100). Filters: `status`, `step_id`. 404 when the record does not exist in this workspace. |
| `enrol_contacts` | `POST /workflows/{id}/enrolments` | owner, admin, marketer | `EnrolRequest` → 201 `EnrolResult` | Bypasses the trigger. Honors the workflow's re-entry rule; contacts already active are reported, not re-enrolled. Publishes `enrolment.started` on `crm.automation.v1`. |
| `schedule_recurring_action` | `POST /actions/recurring` | owner, admin, marketer | `RecurringActionRequest` → 201 `ScheduledAction` | Create a new actions |
| `schedule_action` | `POST /actions/schedule` | owner, admin, marketer | `ScheduleActionRequest` → 201 `ScheduledAction` | Schedule a report to run on a recurring basis |
| `list_upcoming_actions` | `GET /actions/upcoming` | owner, admin, marketer, sales, read-only | — → 200 `ScheduledActionPage` | List all actions |
| `get_workflow_map` | `GET /workflows/{id}/map` | owner, admin, marketer, sales, read-only | — → 200 `WorkflowMap` | The live view behind an automations map - how many contacts sit at each step now, how many have passed through, and per-step send/open/click counts. |
| `list_rules` | `GET /rules` | owner, admin, marketer, sales, read-only | — → 200 `RulePage` | Get workflow rules |
| `create_rule` | `POST /rules` | owner, admin, marketer | `CreateRuleRequest` → 201 `Rule` | Get workflow rules |
| `test_rule` | `POST /rules/test` | owner, admin, marketer | inline object → 200 `RuleTestResult` | Get workflow rules |
| `get_rule` | `GET /rules/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Rule` | Get workflow rules |
| `update_rule` | `PUT /rules/{id}` | owner, admin, marketer | `UpdateRuleRequest` → 200 `Rule` | Get workflow rules |
| `delete_rule` | `DELETE /rules/{id}` | owner, admin, marketer | — → 204 — | Get workflow rules |
| `list_workflow_rules` | `GET /workflows/{id}/rules` | owner, admin, marketer, sales, read-only | — → 200 `RulePage` | Get workflow rules |
| `create_workflow_rule` | `POST /workflows/{id}/rules` | owner, admin, marketer | `CreateRuleRequest` → 201 `Rule` | Get workflow rules |

## Steps

1. Services: `microservices/crm/automation/impl/src/services/engine.rs` (add `pub mod engine;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/engine.rs`.
4. Idempotency: `retry_enrolment`, `schedule_action` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/automation/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `enrolment.started`, `enrolment.stopped` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `retry_enrolment`, `schedule_action` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_automation --all-targets -- -D warnings && cargo test -p rerp_crm_automation
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
