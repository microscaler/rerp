# CRM-MSG-03: crm/messaging — Campaigns, sends and the send worker

**Service:** crm/messaging · **Areas:** `campaigns`, `sends`, `transactional`, `events` (15 operations) · **Build step:** 3 · **Depends on:** CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_02, US_55_08, US_55_10, US_55_24, US_55_32 · **Points:** 7

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_02, US_55_08, US_55_10, US_55_24, US_55_32 run against crm/messaging instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/messaging/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/messaging/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/messaging.yaml` — tables: `campaigns`, `campaign_variants`, `sends`, `message_events`, `tracked_links`.
- `openapi/crm/docs/10-events.md` — the events published here: `campaign.cancelled`, `campaign.scheduled`, `message.sent`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_02_*.md` — what the screen expects (Marketing overview).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_08_*.md` — what the screen expects (Email campaigns — list and wizard).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_10_*.md` — what the screen expects (Campaign report).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_24_*.md` — what the screen expects (Reports — hub, deals, contacts, email, revenue).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_32_*.md` — what the screen expects (Ad audiences and warm-up).

## Rules for this area

Schedule checks readiness (sender verified, footer and unsubscribe link, audience non-empty) and returns `CampaignValidation` on 400. The send worker resolves the audience from audience (lists − exclusions − unsubscribed − suppressed ∩ segment), writes one send per recipient (unique per campaign and contact, so a restart resumes) and throttles per provider (≥ 100 k recipients per hour per workspace). Test sends are flagged and left out of reports.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_campaigns` | `GET /campaigns` | owner, admin, marketer, sales, read-only | — → 200 `CampaignPage` | List campaigns. Paged (`page`, `limit` ≤ 100). Filters: `search`, `status`, `kind`, `list_id`. |
| `create_campaign` | `POST /campaigns` | owner, admin, marketer | `CreateCampaignRequest` → 201 `Campaign` | Create a campaign (draft). Audited. |
| `get_campaign` | `GET /campaigns/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Campaign` | Get a campaign. 404 when the record does not exist in this workspace. |
| `update_campaign` | `PUT /campaigns/{id}` | owner, admin, marketer | `UpdateCampaignRequest` → 200 `Campaign` | Update a draft or scheduled campaign. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `delete_campaign` | `DELETE /campaigns/{id}` | owner, admin, marketer | — → 204 — | Delete a draft campaign. Audited. 409 while something still uses it. 404 when the record does not exist in this workspace. |
| `preview_campaign_audience` | `GET /campaigns/{id}/audience` | owner, admin, marketer, sales, read-only | — → 200 `AudiencePreview` | Lists minus exclusions minus suppressed minus unsubscribed, filtered by the segment. Returns the count and a sample. |
| `cancel_campaign` | `POST /campaigns/{id}/cancel` | owner, admin, marketer | — → 200 `Campaign` | Cancel a scheduled campaign or stop one mid-send. A state change on one resource; audited. 409 when the current state does not allow it. 404 when the record does not exist in this workspace. Publishes `campaign.cancelled` on `crm.messaging.v1`. |
| `duplicate_campaign` | `POST /campaigns/{id}/duplicate` | owner, admin, marketer | — → 201 `Campaign` | Copy a campaign as a new draft. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `schedule_campaign` | `POST /campaigns/{id}/schedule` | owner, admin, marketer | `ScheduleCampaignRequest` → 200 `Campaign` | Validates the campaign (sender verified, template renders, audience non-empty, unsubscribe link present) and moves it to SCHEDULED. Omit send_at to send immediately. Publishes `campaign.scheduled` on `crm.messaging.v1`. |
| `test_send_campaign` | `POST /campaigns/{id}/test-send` | owner, admin, marketer | `TestSendRequest` → 200 `TestSendResult` | Marked as a test and left out of every report. Publishes `message.sent` on `crm.messaging.v1`. |
| `list_campaign_sends` | `GET /campaigns/{id}/sends` | owner, admin, marketer, sales, read-only | — → 200 `SendPage` | Per-recipient sends for a campaign. Paged (`page`, `limit` ≤ 100). Filters: `status`. 404 when the record does not exist in this workspace. |
| `list_contact_sends` | `GET /contacts/{id}/sends` | owner, admin, marketer, sales, read-only | — → 200 `SendPage` | Everything sent to a contact. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `get_send` | `GET /sends/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Send` | One send with its events. 404 when the record does not exist in this workspace. |
| `send_transactional` | `POST /transactional/send` | owner, admin, marketer | `TransactionalSendRequestBody` → 202 `Send` | For welcome, password, receipt and alert mail. Honors the suppression list and hard bounces but not list subscription state (a transactional message is not marketing). Requires a template of kind TRANSACTIONAL. Publishes `message.sent` on `crm.messaging.v1`. |
| `list_events` | `GET /events` | owner, admin, marketer, sales, read-only | — → 200 `MessageEventPage` | Filter by contact, campaign, send or type. The automation service subscribes to these to fire EMAIL_OPENED / EMAIL_CLICKED triggers. |

## Steps

1. Services: `microservices/crm/messaging/impl/src/services/campaigns.rs` (add `pub mod campaigns;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/campaigns.rs`.
4. Idempotency: `cancel_campaign`, `duplicate_campaign`, `schedule_campaign`, `test_send_campaign`, `send_transactional` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/messaging/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `campaign.cancelled`, `campaign.scheduled`, `message.sent` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `cancel_campaign`, `duplicate_campaign`, `schedule_campaign`, `test_send_campaign`, `send_transactional` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_messaging --all-targets -- -D warnings && cargo test -p rerp_crm_messaging
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
