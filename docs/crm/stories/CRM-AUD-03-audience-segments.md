# CRM-AUD-03: crm/audience — Tags, segments and bulk jobs

**Service:** crm/audience · **Areas:** `tags`, `segments`, `bulk-jobs` (16 operations) · **Build step:** 1 · **Depends on:** CRM-AUD-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_03, US_55_04, US_55_07, US_55_08, US_55_29, US_55_32 · **Points:** 7

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_03, US_55_04, US_55_07, US_55_08, US_55_29, US_55_32 run against crm/audience instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/audience/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/audience/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/audience.yaml` — tables: `tags`, `contact_tags`, `segments`, `bulk_jobs`, `bulk_job_changes`.
- `openapi/crm/docs/10-events.md` — the events published here: `contact.updated`, `subscription.changed`, `tag.added`, `tag.removed`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_03_*.md` — what the screen expects (Contacts list, filters and bulk actions).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_04_*.md` — what the screen expects (Contact profile and activity timeline).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_07_*.md` — what the screen expects (Segment builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_08_*.md` — what the screen expects (Email campaigns — list and wizard).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_32_*.md` — what the screen expects (Ad audiences and warm-up).

## Rules for this area

Segments evaluate the shared condition tree over contact_facts with rerp_crm_runtime::conditions (p95 ≤ 700 ms at 100 k contacts). Bulk jobs run in the background, record before/after per contact and can be undone for 7 days; SUBSCRIBE needs a consent source.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `add_contact_tags` | `POST /contacts/{id}/tags` | owner, admin, marketer, sales | `TagNames` → 200 `Contact` | Emits TAG_ADDED for each tag newly present, which crm/automation listens for. Publishes `tag.added` on `crm.audience.v1`. |
| `remove_contact_tags` | `DELETE /contacts/{id}/tags` | owner, admin, marketer, sales | `TagNames` → 200 `Contact` | Remove tags from a contact. Audited. 404 when the record does not exist in this workspace. Publishes `tag.removed` on `crm.audience.v1`. |
| `list_tags` | `GET /tags` | owner, admin, marketer, sales, read-only | — → 200 `TagPage` | Tags with contact counts. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_tag` | `POST /tags` | owner, admin, marketer | `CreateTagRequest` → 201 `Tag` | Create a tag. Audited. |
| `update_tag` | `PUT /tags/{id}` | owner, admin, marketer | `CreateTagRequest` → 200 `Tag` | Rename or recolour a tag. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `delete_tag` | `DELETE /tags/{id}` | owner, admin, marketer | — → 204 — | Delete a tag (removed from every contact). Audited. 409 while something still uses it. 404 when the record does not exist in this workspace. |
| `audience_update_tag` | `PATCH /tags/{id}` | owner, admin, marketer | `UpdateTagRequest` → 200 `Tag` | Rename, recolour or merge a tag. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `list_segments` | `GET /segments` | owner, admin, marketer, sales, read-only | — → 200 `SegmentPage` | Saved segments. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_segment` | `POST /segments` | owner, admin, marketer | `CreateSegmentRequest` → 201 `Segment` | Save a segment. Audited. |
| `evaluate_segment` | `POST /segments/evaluate` | owner, admin, marketer | inline object → 200 inline object | Count and sample the contacts a condition tree matches. Audited. |
| `get_segment` | `GET /segments/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Segment` | Get a segment. 404 when the record does not exist in this workspace. |
| `update_segment` | `PUT /segments/{id}` | owner, admin, marketer | `CreateSegmentRequest` → 200 `Segment` | Update a segment. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `delete_segment` | `DELETE /segments/{id}` | owner, admin, marketer | — → 204 — | Delete a segment. Audited. 409 while something still uses it. 404 when the record does not exist in this workspace. |
| `list_bulk_jobs` | `GET /bulk-jobs` | owner, admin, marketer, sales, read-only | — → 200 `BulkJobPage` | Bulk jobs, newest first. Paged (`page`, `limit` ≤ 100). |
| `create_bulk_job` | `POST /bulk-jobs` | owner, admin, marketer | `CreateBulkJobRequest` → 202 `BulkJob` | Runs in the background on a segment or an explicit list of contacts. Publishes `contact.updated`, `subscription.changed`, `tag.added`, `tag.removed` on `crm.audience.v1`. |
| `undo_bulk_job` | `POST /bulk-jobs/{id}/undo` | owner, admin, marketer | — → 202 `BulkJob` | Restores the before-values. 409 when the job is not undoable or older than 7 days. Publishes `contact.updated` on `crm.audience.v1`. |

## Steps

1. Services: `microservices/crm/audience/impl/src/services/segments.rs` (add `pub mod segments;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/segments.rs`.
4. Idempotency: `create_bulk_job`, `undo_bulk_job` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/audience/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `contact.updated`, `subscription.changed`, `tag.added`, `tag.removed` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `create_bulk_job`, `undo_bulk_job` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_audience --all-targets -- -D warnings && cargo test -p rerp_crm_audience
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
