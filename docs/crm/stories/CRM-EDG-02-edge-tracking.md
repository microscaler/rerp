# CRM-EDG-02: crm/edge — Pixels, click redirects, unsubscribe, preferences, tracking and ad clicks

**Service:** crm/edge · **Areas:** `public`, `visitors`, `ad-tracking`, `calendars` (12 operations) · **Build step:** 2 · **Depends on:** CRM-RT-01, CRM-RT-02, CRM-RT-04 · **Serves (PriceWhisperer Epic 55):** — · **Points:** 6

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of this area run against crm/edge instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/edge/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/edge/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/10-events.md` — the events published here: `ad_call.received`, `ad_click.received`, `click.received`, `confirmation.received`, `open.received`, `preferences.received`, `track.received`, `unsubscribe.received`.

## Rules for this area

Click redirects and preference pages answer from the signed token alone (rerp_crm_runtime::tokens), never by calling a service. One-click unsubscribe follows RFC 8058. Set the first-party visitor cookie on ad landings.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `public_click` | `GET /c/{token}` | public | — →  — | Records a CLICKED event (with the link index) and 302s to the destination held server-side. Destinations are never taken from the query string. Publishes `click.received` on `crm.edge.v1`. |
| `public_confirm_subscription` | `GET /confirm/{token}` | public | — → 200 — | Double opt-in confirmation. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. Publishes `confirmation.received` on `crm.edge.v1`. |
| `public_open` | `GET /o/{token}` | public | — → 200 — | Records an OPENED event for the send the token names and returns a 1x1 GIF. Unauthenticated; the token is an HMAC of the send id. Publishes `open.received` on `crm.edge.v1`. |
| `public_preferences_get` | `GET /p/{token}` | public | — → 200 `PreferencePage` | Preference page state. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. |
| `public_preferences_update` | `POST /p/{token}` | public | `PreferenceUpdate` → 200 `PreferencePage` | Update list subscriptions from the preference page. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. Publishes `preferences.received` on `crm.edge.v1`. |
| `public_unsubscribe_page` | `GET /u/{token}` | public | — → 200 — | Unsubscribe landing page. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. |
| `public_unsubscribe_one_click` | `POST /u/{token}` | public | — → 200 — | One-click unsubscribe (RFC 8058 List-Unsubscribe-Post). Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. Publishes `unsubscribe.received` on `crm.edge.v1`. |
| `public_web_view` | `GET /view/{token}` | public | — → 200 — | View the message in a browser. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. |
| `track_page_view` | `POST /track` | public | `TrackPageViewRequest` → 201 `VisitorTracking` | Track visitor activity on a page Publishes `track.received` on `crm.edge.v1`. |
| `record_ad_call` | `POST /ads/calls` | public | `RecordAdCallRequest` → 201 `AdTouchpoint` | Record an inbound call (telephony webhook) as a touchpoint; matched to a contact by caller number or by the dynamically inserted number's session. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. Publishes `ad_call.received` on `crm.edge.v1`. |
| `record_ad_click` | `POST /ads/clicks` | public | `RecordAdClickRequest` → 201 `AdClickResult` | Called by the tracking snippet (`/tracking/sites`) on every landing that carries a click id (`gclid`, `wbraid`, `gbraid`, `fbclid`, `ttclid`, `li_fat_id`, `msclkid`, `twclid`, `epik`, or the workspace's own `pw_cid`) or UTM parameters. The origin must be an allowed tracked site. When `cookie.server_set` is on, the response carries a `Set-Cookie` for the tracking domain so the id survives ITP's 7-day cap on script-set cookies. Publishes `ad_click.received` on `crm.edge.v1`. |
| `get_calendar_ics` | `GET /calendars/{token}.ics` | public | — → 200 — | The feed as iCalendar (no auth; the token is the secret). One VEVENT per task with due date/time, the type's default duration, assignee as attendee, related contact/deal in the description, VALARMs from reminders when include_alerts. Public: no authentication; the edge validates shape, origin and rate, publishes, and returns at once. |

## Steps

1. Services: `microservices/crm/edge/impl/src/services/tracking.rs` (add `pub mod tracking;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/tracking.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/edge/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `ad_call.received`, `ad_click.received`, `click.received`, `confirmation.received`, `open.received`, `preferences.received`, `track.received`, `unsubscribe.received` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
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
cd microservices && cargo clippy -p rerp_crm_edge --all-targets -- -D warnings && cargo test -p rerp_crm_edge
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
