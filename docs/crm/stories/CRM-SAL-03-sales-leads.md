# CRM-SAL-03: crm/sales — Leads and triage

**Service:** crm/sales · **Areas:** `leads` (10 operations) · **Build step:** 4 · **Depends on:** CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_18, US_55_21 · **Points:** 5

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_18, US_55_21 run against crm/sales instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/sales/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/sales/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/sales.yaml` — tables: `leads`.
- `openapi/crm/docs/10-events.md` — the events published here: `lead.converted`, `lead.created`, `lead.stage_changed`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_18_*.md` — what the screen expects (Deals board and new deal).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_21_*.md` — what the screen expects (Website leads board (convert / lost)).

## Rules for this area

Leads come from `intake.admitted` (enquiries) and staff. Duplicate detection by normalised email; convert creates the deal and publishes `lead.converted`. This replaces `crm/pipeline` against Supabase.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_leads` | `GET /leads` | owner, admin, marketer, sales, read-only | — → 200 `LeadPage` | Get leads associated with a campaign |
| `create_lead` | `POST /leads` | owner, admin, sales | `CreateLeadRequest` → 201 `Lead` | Get leads associated with a campaign Publishes `lead.created` on `crm.sales.v1`. |
| `detect_duplicates` | `POST /leads/detect-duplicates` | owner, admin, sales | `DuplicateDetectionRequest` → 200 `DuplicateDetectionResult` | Create a new leads |
| `get_lead` | `GET /leads/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Lead` | Get leads by ID |
| `update_lead` | `PUT /leads/{id}` | owner, admin, sales | `UpdateLeadRequest` → 200 `Lead` | Update leads by ID |
| `delete_lead` | `DELETE /leads/{id}` | owner, admin, sales | — → 204 — | Delete leads by ID |
| `convert_lead` | `POST /leads/{id}/convert` | owner, admin, sales | `ConvertLeadRequest` → 202 `ConversionResult` | Marks the lead won and publishes a signed conversion to RERP (ADR 003). Only a converted customer crosses from the marketing funnel; RERP creates the customer (a CRM account) when it pulls the conversion, so the crossing is asynchronous. Converting the same lead again republishes the same conversion and creates nothing new. Publishes `lead.converted` on `crm.sales.v1`. |
| `merge_leads` | `POST /leads/{id}/merge` | owner, admin, sales | `MergeLeadsRequest` → 200 `MergeResult` | Perform an action on leads |
| `change_stage` | `PATCH /leads/{id}/stage` | owner, admin, sales | `StageChangeRequest` → 200 `Lead` | Partially update leads by ID Publishes `lead.stage_changed` on `crm.sales.v1`. |
| `pipeline_summary` | `GET /pipelines/summary` | owner, admin, marketer, sales, read-only | — → 200 `PipelineSummaryReport` | List all pipeline |

## Steps

1. Services: `microservices/crm/sales/impl/src/services/leads.rs` (add `pub mod leads;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/leads.rs`.
4. Idempotency: `convert_lead`, `merge_leads` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/sales/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `lead.converted`, `lead.created`, `lead.stage_changed` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `convert_lead`, `merge_leads` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
