# CRM-ACQ-01: crm/acquisition — Forms, submissions, tracked sites and visitors

**Service:** crm/acquisition · **Areas:** `marketing-forms`, `visitors`, `events` (12 operations) · **Build step:** 5 · **Depends on:** CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_14, US_55_15 · **Points:** 6

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_14, US_55_15 run against crm/acquisition instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/acquisition/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/acquisition/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/acquisition.yaml` — tables: `forms`, `form_submissions`, `tracked_sites`, `event_types`, `visitors`, `page_views`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_14_*.md` — what the screen expects (Forms and the form builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_15_*.md` — what the screen expects (Site tracking).

## Rules for this area

Publish the signed form-config snapshot the edge serves. Submissions arrive admitted from intake.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_forms` | `GET /forms` | owner, admin, marketer, sales, read-only | — → 200 `WebFormPage` | List web forms. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_form` | `POST /forms` | owner, admin, marketer | `CreateWebFormRequest` → 201 `WebForm` | Create a web form. Audited. |
| `list_recent_form_submissions` | `GET /forms/submissions` | owner, admin, marketer, sales, read-only | — → 200 `FormSubmissionPage` | Latest submissions across forms. Paged (`page`, `limit` ≤ 100). Filters: `form_id`. |
| `update_form` | `PUT /forms/{id}` | owner, admin, marketer | `CreateWebFormRequest` → 200 `WebForm` | Update a form's fields, style and actions. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `get_form` | `GET /forms/{id}` | owner, admin, marketer, sales, read-only | — → 200 `WebForm` | A form with its fields, actions, style and embed. 404 when the record does not exist in this workspace. |
| `archive_form` | `POST /forms/{id}/archive` | owner, admin, marketer | — → 200 `WebForm` | Archive a form (its public endpoint stops accepting submissions). A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `get_form_embed` | `GET /forms/{id}/embed` | owner, admin, marketer, sales, read-only | — → 200 `FormEmbed` | A script tag for the inline/floating/modal variants and the hosted page URL. |
| `list_form_submissions` | `GET /forms/{id}/submissions` | owner, admin, marketer, sales, read-only | — → 200 `FormSubmissionPage` | Submissions to a form. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `list_visitors` | `GET /analytics/visitors` | owner, admin, marketer, sales, read-only | — → 200 `VisitorTrackingPage` | List all analytics |
| `identify_visitor` | `POST /analytics/visitors/identify` | owner, admin, marketer | `IdentifyVisitorRequest` → 200 inline object | Identify a visitor by their session |
| `get_live_visitors` | `GET /tracking/live` | owner, admin, marketer, sales, read-only | — → 200 `LiveVisitors` | Identified visitors in the last 30 minutes. |
| `get_event_registrations` | `GET /events/{id}/registrations` | owner, admin, marketer, sales, read-only | — → 200 `EventRegistrationPage` | Get event registrations |

## Steps

1. Services: `microservices/crm/acquisition/impl/src/services/forms.rs` (add `pub mod forms;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/forms.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/acquisition/openapi.yaml`
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
cd microservices && cargo clippy -p rerp_crm_acquisition --all-targets -- -D warnings && cargo test -p rerp_crm_acquisition
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
