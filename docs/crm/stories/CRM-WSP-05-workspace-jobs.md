# CRM-WSP-05: crm/workspace — Import and export jobs, data requests and the audit log

**Service:** crm/workspace · **Areas:** `jobs`, `data-requests`, `audit-log` (10 operations) · **Build step:** 1 · **Depends on:** CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_03, US_55_07, US_55_10, US_55_27, US_55_29 · **Points:** 5

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_03, US_55_07, US_55_10, US_55_27, US_55_29 run against crm/workspace instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/workspace/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/workspace/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/workspace.yaml` — tables: `jobs`, `job_errors`, `data_requests`, `data_request_steps`, `audit_entries`.
- `openapi/crm/docs/10-events.md` — the events published here: `erasure.requested`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_03_*.md` — what the screen expects (Contacts list, filters and bulk actions).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_07_*.md` — what the screen expects (Segment builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_10_*.md` — what the screen expects (Campaign report).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_27_*.md` — what the screen expects (Import and export jobs).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).

## Rules for this area

Approving an erasure publishes `erasure.requested`; every service replies `erasure.completed` and the request is DONE when all have confirmed (data_request_steps). The audit log is read here and written by every service through rerp_crm_runtime::audit.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_jobs` | `GET /jobs` | owner, admin, marketer, sales, read-only | — → 200 `JobPage` | Newest first. Import/Export Status screen. |
| `start_export` | `POST /jobs/export` | owner, admin | `ExportRequest` → 202 `Job` | Queue an export of an entity with the given filter/columns; poll /jobs/{id} and download from result_url. |
| `get_job` | `GET /jobs/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Job` | Progress and counts of one job. |
| `cancel_job` | `POST /jobs/{id}/cancel` | owner, admin | — → 200 `Job` | Cancel a QUEUED or RUNNING job; rows already written stay. |
| `list_job_errors` | `GET /jobs/{id}/errors` | owner, admin, marketer, sales, read-only | — → 200 `JobErrorPage` | Per-row errors of a PARTIAL or FAILED import. |
| `list_data_requests` | `GET /data-requests` | owner, admin, marketer, sales, read-only | — → 200 `DataRequestPage` | Access and erasure requests. Paged (`page`, `limit` ≤ 100). Filters: `status`. |
| `create_data_request` | `POST /data-requests` | owner, admin | `CreateDataRequestRequest` → 201 `DataRequest` | Record an access or erasure request. Audited. |
| `approve_data_request` | `POST /data-requests/{id}/approve` | owner | — → 202 `DataRequest` | Publishes erasure.requested (or builds the access export) and tracks each service's confirmation. Publishes `erasure.requested` on `crm.workspace.v1`. |
| `list_audit_log` | `GET /audit-log` | owner, admin, marketer, sales, read-only | — → 200 `AuditLogEntryPage` | List all audit-log |
| `get_entity_audit_log` | `GET /audit-log/{entity}/{entity_id}` | owner, admin, marketer, sales, read-only | — → 200 inline object | Get audit-log by ID |

## Steps

1. Services: `microservices/crm/workspace/impl/src/services/jobs.rs` (add `pub mod jobs;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/jobs.rs`.
4. Idempotency: `start_export`, `cancel_job`, `approve_data_request` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/workspace/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `erasure.requested` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `start_export`, `cancel_job`, `approve_data_request` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_workspace --all-targets -- -D warnings && cargo test -p rerp_crm_workspace
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
