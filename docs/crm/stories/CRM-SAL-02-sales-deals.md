# CRM-SAL-02: crm/sales — Deals, deal contacts, notes, files, emails and status

**Service:** crm/sales · **Areas:** `deals` (24 operations) · **Build step:** 4 · **Depends on:** CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_04, US_55_05, US_55_18, US_55_19, US_55_22 · **Points:** 10

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_04, US_55_05, US_55_18, US_55_19, US_55_22 run against crm/sales instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/sales/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/sales/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/sales.yaml` — tables: `deals`, `deal_contacts`, `deal_notes`, `deal_files`, `deal_emails`, `deal_activity`.
- `openapi/crm/docs/10-events.md` — the events published here: `deal.created`, `deal.lost`, `deal.reopened`, `deal.stage_changed`, `deal.updated`, `deal.won`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_04_*.md` — what the screen expects (Contact profile and activity timeline).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_05_*.md` — what the screen expects (Companies).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_18_*.md` — what the screen expects (Deals board and new deal).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_19_*.md` — what the screen expects (Deal detail).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_22_*.md` — what the screen expects (Tasks and task settings).

## Rules for this area

`PATCH /deals/{id}/status` to WON emits the signed conversion on `crm-conversions` (ADR 003, `microservices/crm/conversion`) in addition to `deal.won`; reopening is owner/admin only. Sales users change only deals they own unless admin.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_deals` | `GET /deals` | owner, admin, marketer, sales, read-only | — → 200 `DealPage` | List deals. Paged (`page`, `limit` ≤ 100). Filters: `search`, `pipeline_id`, `stage_id`, `status`, `owner_id`, `contact_id`, `organisation_id`, `tag_id`, `stalled`, `expected_close_from`, `expected_close_to`, `value_min`, `value_max`, `sort`, `dir`. |
| `create_deal` | `POST /deals` | owner, admin, sales | `CreateDealRequest` → 201 `Deal` | Create a deal. Audited. Publishes `deal.created` on `crm.sales.v1`. |
| `bulk_update_deals` | `POST /deals/bulk` | owner, admin, sales | `BulkDealRequest` → 200 inline object | Move, assign, set status, tag or delete many deals. Audited. Publishes `deal.updated` on `crm.sales.v1`. |
| `get_deal` | `GET /deals/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Deal` | Get a deal. 404 when the record does not exist in this workspace. |
| `update_deal` | `PUT /deals/{id}` | owner, admin, sales | `CreateDealRequest` → 200 `Deal` | Replace a deal. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. Publishes `deal.updated` on `crm.sales.v1`. |
| `patch_deal` | `PATCH /deals/{id}` | owner, admin, sales | `UpdateDealRequest` → 200 `Deal` | Update part of a deal. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. Publishes `deal.updated` on `crm.sales.v1`. |
| `delete_deal` | `DELETE /deals/{id}` | owner, admin, sales | — → 204 — | Delete a deal. Audited. 404 when the record does not exist in this workspace. |
| `list_deal_activity` | `GET /deals/{id}/activity` | owner, admin, marketer, sales, read-only | — → 200 `ActivityEntryPage` | The deal's timeline: changes, notes, files, tasks, emails, workflow actions. Paged (`page`, `limit` ≤ 100). Filters: `search`, `kind`. 404 when the record does not exist in this workspace. |
| `add_deal_contact` | `POST /deals/{id}/contacts` | owner, admin, sales | `DealContact` → 201 `Deal` | Attach a contact with a role; the first one becomes primary. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_deal_contacts` | `GET /deals/{id}/contacts` | owner, admin, marketer, sales, read-only | — → 200 `DealContactPage` | Contacts on a deal, with roles. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `update_deal_contact` | `PATCH /deals/{id}/contacts/{contact_id}` | owner, admin, sales | `DealContactInput` → 200 `Deal` | Change a contact's role or make it primary. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `remove_deal_contact` | `DELETE /deals/{id}/contacts/{contact_id}` | owner, admin, sales | — → 204 — | Detach a contact. Audited. 404 when the record does not exist in this workspace. |
| `list_deal_emails` | `GET /deals/{id}/emails` | owner, admin, marketer, sales, read-only | — → 200 `DealEmailPage` | 1:1 emails on the deal (sent through connected mailboxes, and replies). Paged (`page`, `limit` ≤ 100). Filters: `search`. 404 when the record does not exist in this workspace. |
| `send_deal_email` | `POST /deals/{id}/emails` | owner, admin, sales | `SendDealEmailRequest` → 201 `DealEmail` | 422 NO_CONNECTED_MAILBOX when the caller has none (crm/platform /mailboxes). |
| `list_deal_files` | `GET /deals/{id}/files` | owner, admin, marketer, sales, read-only | — → 200 `AttachmentPage` | Files on a deal. Paged (`page`, `limit` ≤ 100). Filters: `search`. 404 when the record does not exist in this workspace. |
| `upload_deal_file` | `POST /deals/{id}/files` | owner, admin, sales | — → 201 `Attachment` | Upload a file (multipart). A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `delete_deal_file` | `DELETE /deals/{id}/files/{file_id}` | owner, admin, sales | — → 204 — | Delete a file. Audited. 404 when the record does not exist in this workspace. |
| `list_deal_notes` | `GET /deals/{id}/notes` | owner, admin, marketer, sales, read-only | — → 200 `NotePage` | Notes on a deal, newest first. Paged (`page`, `limit` ≤ 100). Filters: `search`. 404 when the record does not exist in this workspace. |
| `add_deal_note` | `POST /deals/{id}/notes` | owner, admin, sales | `NoteRequest` → 201 `Note` | Add a note. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `update_deal_note` | `PATCH /deals/{id}/notes/{note_id}` | owner, admin, sales | `NoteRequest` → 200 `Note` | Edit or pin a note. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_deal_note` | `DELETE /deals/{id}/notes/{note_id}` | owner, admin, sales | — → 204 — | Delete a note. Audited. 404 when the record does not exist in this workspace. |
| `change_deal_stage` | `PATCH /deals/{id}/stage` | owner, admin, sales | `DealStageChangeRequest` → 200 `Deal` | Move a deal to a stage (or another pipeline); fires DEAL_STAGE_CHANGE / ENTERED_PIPELINE. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. Publishes `deal.stage_changed` on `crm.sales.v1`. |
| `change_deal_status` | `PATCH /deals/{id}/status` | owner, admin, sales | `DealStatusChangeRequest` → 200 `Deal` | Mark a deal won, lost or open again; WON emits the signed conversion (ADR 003) and fires DEAL_WON. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. Publishes `deal.won`, `deal.lost`, `deal.reopened` on `crm.sales.v1`. |
| `get_deal_suggestions` | `GET /deals/{id}/suggestions` | owner, admin, marketer, sales, read-only | — → 200 `DealSuggestions` | Suggested next steps. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/sales/impl/src/services/deals.rs` (add `pub mod deals;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/deals.rs`.
4. Idempotency: `bulk_update_deals` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/sales/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `deal.created`, `deal.lost`, `deal.reopened`, `deal.stage_changed`, `deal.updated`, `deal.won` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `bulk_update_deals` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
