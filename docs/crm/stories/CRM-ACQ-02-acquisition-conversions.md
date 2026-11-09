# CRM-ACQ-02: crm/acquisition — Conversions, goals, UTM and marketing analytics

**Service:** crm/acquisition · **Areas:** `conversions`, `goals`, `utm`, `campaigns`, `marketing-campaigns`, `marketing-analytics` (26 operations) · **Build step:** 5 · **Depends on:** CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_15, US_55_16, US_55_24 · **Points:** 10

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_15, US_55_16, US_55_24 run against crm/acquisition instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/acquisition/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/acquisition/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/acquisition.yaml` — tables: `conversion_definitions`, `goals`, `utm_campaigns`, `utm_mediums`, `utm_sources`.
- `openapi/crm/docs/10-events.md` — the events published here: `event.recorded`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_15_*.md` — what the screen expects (Site tracking).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_16_*.md` — what the screen expects (Conversions and business goals).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_24_*.md` — what the screen expects (Reports — hub, deals, contacts, email, revenue).

## Rules for this area

Goal status is computed (projection = current ÷ elapsed share of the period; AHEAD ≥ 110 %, BEHIND < 90 %).

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_conversion_definitions` | `GET /conversions` | owner, admin, marketer, sales, read-only | — → 200 `ConversionDefinitionPage` | Conversions. Paged (`page`, `limit` ≤ 100). |
| `create_conversion_definition` | `POST /conversions` | owner, admin, marketer | `ConversionDefinitionRequest` → 201 `ConversionDefinition` | Define a conversion. Audited. |
| `update_conversion_definition` | `PATCH /conversions/{id}` | owner, admin, marketer | `ConversionDefinitionRequest` → 200 `ConversionDefinition` | Change a conversion. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_conversion_definition` | `DELETE /conversions/{id}` | owner, admin, marketer | — → 204 — | 409 while a goal, automation or relay rule uses it. |
| `list_goals` | `GET /goals` | owner, admin, marketer, sales, read-only | — → 200 `GoalPage` | Business goals. Paged (`page`, `limit` ≤ 100). |
| `create_goal` | `POST /goals` | owner, admin, marketer | `GoalRequest` → 201 `Goal` | Set a goal. Audited. |
| `update_goal` | `PATCH /goals/{id}` | owner, admin, marketer | `GoalRequest` → 200 `Goal` | Change a goal. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `list_utms_campaigns` | `GET /utm/campaigns` | owner, admin, marketer, sales, read-only | — → 200 `UTMCampaignPage` | List all utm |
| `create_utms_campaign` | `POST /utm/campaigns` | owner, admin, marketer | `CreateUTMCampaignRequest` → 201 `UTMCampaign` | Create a new utm |
| `get_utms_campaign` | `GET /utm/campaigns/{id}` | owner, admin, marketer, sales, read-only | — → 200 `UTMCampaign` | Get utm by ID |
| `update_utms_campaign` | `PUT /utm/campaigns/{id}` | owner, admin, marketer | `UpdateUTMCampaignRequest` → 200 `UTMCampaign` | Update utm by ID |
| `delete_utms_campaign` | `DELETE /utm/campaigns/{id}` | owner, admin, marketer | — → 204 — | Delete utm by ID |
| `list_utms_mediums` | `GET /utm/mediums` | owner, admin, marketer, sales, read-only | — → 200 `UTMMediumPage` | List all utm |
| `create_utms_medium` | `POST /utm/mediums` | owner, admin, marketer | `CreateUTMMediumRequest` → 201 `UTMMedium` | Create a new utm |
| `list_utms_sources` | `GET /utm/sources` | owner, admin, marketer, sales, read-only | — → 200 `UTMSourcePage` | List all utm |
| `create_utms_source` | `POST /utm/sources` | owner, admin, marketer | `CreateUTMSourceRequest` → 201 `UTMSource` | Create a new utm |
| `get_campaign_leads` | `GET /campaigns/{id}/leads` | owner, admin, marketer, sales, read-only | — → 200 `CampaignLeadResultPage` | Get leads associated with a campaign |
| `get_campaign_revenue` | `GET /campaigns/{id}/revenue` | owner, admin, marketer, sales, read-only | — → 200 inline object | Get revenue attributed to a campaign |
| `get_attribution_overview` | `GET /attribution/overview` | owner, admin, marketer, sales, read-only | — → 200 `AttributionOverview` | Leads and revenue by source, medium and campaign over a period. Query: `from`, `to`, `group_by`. |
| `track_custom_event` | `POST /analytics/event` | owner, admin, marketer | `TrackEventRequest` → 202 — | Public, keyed by the site's tracking key and origin check. Fires the automation EVENT trigger. Publishes `event.recorded` on `crm.acquisition.v1`. |
| `list_contact_visits` | `GET /contacts/{id}/visits` | owner, admin, marketer, sales, read-only | — → 200 `VisitorTrackingPage` | Page views and events for an identified contact. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `list_event_types` | `GET /tracking/event-types` | owner, admin, marketer, sales, read-only | — → 200 inline object | Custom event names seen or declared. |
| `declare_event_type` | `POST /tracking/event-types` | owner, admin, marketer | inline object → 201 `EventTypeDef` | Declare a custom event (so automations can trigger on it before it is first seen). Audited. |
| `list_tracked_sites` | `GET /tracking/sites` | owner, admin, marketer, sales, read-only | — → 200 inline object | Sites allowed to post tracking events. |
| `create_tracked_site` | `POST /tracking/sites` | owner, admin | inline object → 201 `TrackedSite` | Allow a site origin. Audited. |
| `delete_tracked_site` | `DELETE /tracking/sites/{id}` | owner, admin | — → 204 — | Stop accepting events from a site. Audited. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/acquisition/impl/src/services/conversions.rs` (add `pub mod conversions;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/conversions.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/acquisition/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `event.recorded` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
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
