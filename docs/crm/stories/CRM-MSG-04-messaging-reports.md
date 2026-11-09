# CRM-MSG-04: crm/messaging — Campaign and deliverability reports

**Service:** crm/messaging · **Areas:** `outreach-reports`, `reports` (6 operations) · **Build step:** 3 · **Depends on:** CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_02, US_55_10, US_55_24 · **Points:** 4

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_02, US_55_10, US_55_24 run against crm/messaging instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/messaging/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/messaging/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/messaging.yaml` — tables: `sends`, `message_events`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_02_*.md` — what the screen expects (Marketing overview).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_10_*.md` — what the screen expects (Campaign report).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_24_*.md` — what the screen expects (Reports — hub, deals, contacts, email, revenue).

## Rules for this area

Computed from this service's send ledger; machine opens are excluded from open rates.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `get_campaign_report` | `GET /campaigns/{id}/report` | owner, admin, marketer, sales, read-only | — → 200 `CampaignReport` | Campaign performance. 404 when the record does not exist in this workspace. |
| `list_campaign_report_contacts` | `GET /campaigns/{id}/report/contacts` | owner, admin, marketer, sales, read-only | — → 200 `CampaignReportContactPage` | Who opened / clicked / bounced / unsubscribed / forwarded / replied / did not open, with a link filter. |
| `get_campaign_performance` | `GET /reports/campaigns` | owner, admin, marketer, sales, read-only | — → 200 `CampaignPerformanceReport` | KPI tiles and per-campaign rows over a window, with period / type comparison and an option to exclude unverified (privacy-prefetch) opens. |
| `get_list_growth` | `GET /reports/lists/{id}/growth` | owner, admin, marketer, sales, read-only | — → 200 `ListGrowth` | Subscriptions and unsubscribes per day for a list. Query: `days`. 404 when the record does not exist in this workspace. |
| `get_outreach_overview` | `GET /reports/overview` | owner, admin, marketer, sales, read-only | — → 200 `OutreachOverview` | Sends, open and click rates over a period. Query: `from`, `to`, `list_id`. |
| `get_deliverability_report` | `GET /reports/deliverability` | owner, admin, marketer, sales, read-only | — → 200 `DeliverabilityReport` | Bounce, complaint and reputation by provider and domain. Query: `from`, `to`. |

## Steps

1. Services: `microservices/crm/messaging/impl/src/services/reports.rs` (add `pub mod reports;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/reports.rs`.

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
