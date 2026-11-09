# CRM-MSG-02: crm/messaging — Senders, SMS senders, tracking domains, list exclusions and providers

**Service:** crm/messaging · **Areas:** `senders`, `sms`, `tracking-domains`, `list-exclusions`, `providers` (13 operations) · **Build step:** 3 · **Depends on:** CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_06, US_55_08, US_55_29 · **Points:** 6

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_06, US_55_08, US_55_29 run against crm/messaging instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/messaging/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/messaging/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/messaging.yaml` — tables: `senders`, `sms_senders`, `tracking_domains`, `list_exclusions`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_06_*.md` — what the screen expects (Lists and consent).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_08_*.md` — what the screen expects (Email campaigns — list and wizard).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).

## Rules for this area

Verification checks DNS (SPF, DKIM, DMARC, CNAME) through the provider adapter. Provider credentials are in Secret Manager; `GET /providers` never returns a secret.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_senders` | `GET /senders` | owner, admin, marketer, sales, read-only | — → 200 inline object | Sender identities. |
| `create_sender` | `POST /senders` | owner, admin | `CreateSenderRequest` → 201 `Sender` | Starts domain verification; the response carries the DNS records (DKIM, SPF, DMARC, return-path) the operator publishes through their DNS tooling. |
| `get_sender` | `GET /senders/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Sender` | Sender with verification state. 404 when the record does not exist in this workspace. |
| `delete_sender` | `DELETE /senders/{id}` | owner, admin | — → 204 — | Remove a sender identity. Audited. 409 while something still uses it. 404 when the record does not exist in this workspace. |
| `verify_sender` | `POST /senders/{id}/verify` | owner, admin | — → 200 `Sender` | Re-check the sender's DNS records. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_sms_senders` | `GET /sms/senders` | owner, admin, marketer, sales, read-only | — → 200 `SmsSenderPage` | SMS senders. Paged (`page`, `limit` ≤ 100). |
| `list_tracking_domains` | `GET /tracking-domains` | owner, admin, marketer, sales, read-only | — → 200 `TrackingDomainPage` | Tracking domains. Paged (`page`, `limit` ≤ 100). |
| `create_tracking_domain` | `POST /tracking-domains` | owner, admin | `CreateTrackingDomainRequest` → 201 `TrackingDomain` | Add a tracking domain. Audited. |
| `verify_tracking_domain` | `POST /tracking-domains/{id}/verify` | owner, admin | — → 200 `TrackingDomain` | Re-check DNS and TLS. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_list_exclusions` | `GET /list-exclusions` | owner, admin, marketer, sales, read-only | — → 200 `ListExclusionPage` | List exclusions. Paged (`page`, `limit` ≤ 100). |
| `create_list_exclusion` | `POST /list-exclusions` | owner, admin, marketer | `CreateListExclusionRequest` → 201 `ListExclusion` | Add a list exclusion. Audited. |
| `delete_list_exclusion` | `DELETE /list-exclusions/{id}` | owner, admin, marketer | — → 204 — | Remove a list exclusion. Audited. 404 when the record does not exist in this workspace. |
| `get_sending_providers` | `GET /providers` | owner, admin, marketer, sales, read-only | — → 200 `SendingProviders` | Configured sending providers. |

## Steps

1. Services: `microservices/crm/messaging/impl/src/services/sending_setup.rs` (add `pub mod sending_setup;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/sending_setup.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/messaging/openapi.yaml`
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
cd microservices && cargo clippy -p rerp_crm_messaging --all-targets -- -D warnings && cargo test -p rerp_crm_messaging
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
