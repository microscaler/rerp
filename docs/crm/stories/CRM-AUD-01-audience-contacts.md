# CRM-AUD-01: crm/audience — Contacts, organisations and notes

**Service:** crm/audience · **Areas:** `contacts`, `organisations` (23 operations) · **Build step:** 1 · **Depends on:** CRM-AUD-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_03, US_55_04, US_55_05, US_55_11, US_55_18, US_55_19, US_55_22, US_55_23, US_55_35 · **Points:** 9

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_03, US_55_04, US_55_05, US_55_11, US_55_18, US_55_19, US_55_22, US_55_23, US_55_35 run against crm/audience instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/audience/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/audience/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/audience.yaml` — tables: `contacts`, `organisations`, `contact_notes`, `contact_tags`, `timeline_items`, `contact_facts`.
- `openapi/crm/docs/10-events.md` — the events published here: `contact.archived`, `contact.created`, `contact.merged`, `contact.updated`, `organisation.created`, `organisation.updated`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_03_*.md` — what the screen expects (Contacts list, filters and bulk actions).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_04_*.md` — what the screen expects (Contact profile and activity timeline).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_05_*.md` — what the screen expects (Companies).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_11_*.md` — what the screen expects (Templates, snippets and assets).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_18_*.md` — what the screen expects (Deals board and new deal).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_19_*.md` — what the screen expects (Deal detail).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_22_*.md` — what the screen expects (Tasks and task settings).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_23_*.md` — what the screen expects (Lead scoring).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_35_*.md` — what the screen expects (Ad attribution and LTV).

## Rules for this area

Email is unique per workspace, case-insensitive (409 `duplicate_email`). `status` is derived from subscriptions, never set by the API. Merge keeps the survivor and repoints subscriptions, tags and notes, then publishes `contact.merged`. DELETE archives (sets `archived_at`); erasure is a separate path (data requests).

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_contacts` | `GET /contacts` | owner, admin, marketer, sales, read-only | — → 200 `ContactPage` | List all contacts |
| `create_contact` | `POST /contacts` | owner, admin, marketer, sales | `CreateContactRequest` → 201 `Contact` | Create a new contacts Publishes `contact.created` on `crm.audience.v1`. |
| `get_contact_counts` | `GET /contacts/counts` | owner, admin, marketer, sales, read-only | — → 200 `ContactCountsSummary` | Contact counts for the aside. |
| `export_contacts` | `POST /contacts/export` | owner, admin, marketer, sales | inline object → 202 `ImportJob` | Export contacts matching a segment as CSV. Runs in the background; poll or watch the job. |
| `import_contacts` | `POST /contacts/import` | owner, admin, marketer, sales | — → 202 `ImportJob` | Multipart CSV with a column mapping. Rows are validated and deduplicated by email; the response is a job the client polls. Consent for list subscription is recorded as IMPORT with the stated source. Publishes `contact.created` on `crm.audience.v1`. |
| `get_import_job` | `GET /contacts/import/{id}` | owner, admin, marketer, sales, read-only | — → 200 `ImportJob` | Import job progress. 404 when the record does not exist in this workspace. |
| `merge_contacts` | `POST /contacts/merge` | owner, admin, marketer, sales | `MergeContactsRequest` → 200 `MergeResult` | Create a new contacts Publishes `contact.merged` on `crm.audience.v1`. |
| `search_contacts_by_email` | `GET /contacts/search-by-email` | owner, admin, marketer, sales, read-only | — → 200 `ContactPage` | Search contacts by email address |
| `get_contact` | `GET /contacts/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Contact` | Get contacts by ID |
| `update_contact` | `PUT /contacts/{id}` | owner, admin, marketer, sales | `UpdateContactRequest` → 200 `Contact` | Update contacts by ID Publishes `contact.updated` on `crm.audience.v1`. |
| `delete_contact` | `DELETE /contacts/{id}` | owner, admin, marketer, sales | — → 204 — | Delete contacts by ID Publishes `contact.archived` on `crm.audience.v1`. |
| `patch_contact` | `PATCH /contacts/{id}` | owner, admin, marketer, sales | `UpdateContactRequest` → 200 `Contact` | Only the fields sent change; the change is audited with before and after values. Publishes `contact.updated` on `crm.audience.v1`. |
| `get_contact_accounts` | `GET /contacts/{id}/accounts` | owner, admin, marketer, sales, read-only | — → 200 `OrganisationPage` | Get contacts by ID |
| `get_contact_activity` | `GET /contacts/{id}/activity` | owner, admin, marketer, sales, read-only | — → 200 `ActivityPage` | Merged, newest first, from every CRM service - sends and opens (outreach), enrolment steps (automation), form submissions and page views (marketing), stage changes (pipeline), notes and tag changes. |
| `add_contact_note` | `POST /contacts/{id}/notes` | owner, admin, marketer, sales | inline object → 201 `Activity` | Add a note to the timeline. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `get_contact_hierarchy` | `GET /contacts/{id}/tree` | owner, admin, marketer, sales, read-only | — → 200 `ContactPage` | Get hierarchical tree structure |
| `list_accounts` | `GET /organisations` | owner, admin, marketer, sales, read-only | — → 200 `OrganisationPage` | List all accounts |
| `create_account` | `POST /organisations` | owner, admin, marketer, sales | `CreateOrganisationRequest` → 201 `Organisation` | Create a new accounts Publishes `organisation.created` on `crm.audience.v1`. |
| `get_account` | `GET /organisations/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Organisation` | Get accounts by ID |
| `update_account` | `PUT /organisations/{id}` | owner, admin, marketer, sales | `UpdateOrganisationRequest` → 200 `Organisation` | Update accounts by ID Publishes `organisation.updated` on `crm.audience.v1`. |
| `delete_account` | `DELETE /organisations/{id}` | owner, admin, marketer, sales | — → 204 — | Delete accounts by ID |
| `get_account_contacts` | `GET /organisations/{id}/contacts` | owner, admin, marketer, sales, read-only | — → 200 `ContactPage` | Get accounts by ID |
| `get_account_hierarchy` | `GET /organisations/{id}/tree` | owner, admin, marketer, sales, read-only | — → 200 `OrganisationPage` | Get hierarchical tree structure |

## Steps

1. Services: `microservices/crm/audience/impl/src/services/contacts.rs` (add `pub mod contacts;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/contacts.rs`.
4. Idempotency: `export_contacts`, `import_contacts`, `merge_contacts` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/audience/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `contact.archived`, `contact.created`, `contact.merged`, `contact.updated`, `organisation.created`, `organisation.updated` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `export_contacts`, `import_contacts`, `merge_contacts` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
