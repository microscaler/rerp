# CRM-AUD-02: crm/audience — Lists, subscriptions, consent and suppressions

**Service:** crm/audience · **Areas:** `lists`, `subscriptions`, `suppressions` (13 operations) · **Build step:** 1 · **Depends on:** CRM-AUD-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_03, US_55_04, US_55_06, US_55_08, US_55_14, US_55_29, US_55_32 · **Points:** 6

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_03, US_55_04, US_55_06, US_55_08, US_55_14, US_55_29, US_55_32 run against crm/audience instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/audience/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/audience/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/audience.yaml` — tables: `lists`, `subscriptions`, `consent_events`, `suppressions`.
- `openapi/crm/docs/10-events.md` — the events published here: `subscription.changed`, `suppression.added`, `suppression.removed`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_03_*.md` — what the screen expects (Contacts list, filters and bulk actions).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_04_*.md` — what the screen expects (Contact profile and activity timeline).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_06_*.md` — what the screen expects (Lists and consent).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_08_*.md` — what the screen expects (Email campaigns — list and wizard).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_14_*.md` — what the screen expects (Forms and the form builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_32_*.md` — what the screen expects (Ad audiences and warm-up).

## Rules for this area

Every status change writes a consent_events row in the same transaction. A suppressed address can be stored but never subscribed (409 `suppressed`). Admitting `intake.admitted` creates the contact and a SUBSCRIBED subscription once per submission id (see CRM-AUD-EV).

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_lists` | `GET /lists` | owner, admin, marketer, sales, read-only | — → 200 `MailingListPage` | List mailing lists. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_list` | `POST /lists` | owner, admin, marketer | `CreateMailingListRequest` → 201 `MailingList` | Create a mailing list. Audited. |
| `get_list` | `GET /lists/{id}` | owner, admin, marketer, sales, read-only | — → 200 `MailingList` | Get a mailing list. 404 when the record does not exist in this workspace. |
| `update_list` | `PUT /lists/{id}` | owner, admin, marketer | `UpdateMailingListRequest` → 200 `MailingList` | Update a mailing list. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `delete_list` | `DELETE /lists/{id}` | owner, admin, marketer | — → 204 — | Lists are archived, never hard-deleted; subscription history under them is retained for consent audit. |
| `list_contact_subscriptions` | `GET /contacts/{id}/subscriptions` | owner, admin, marketer, sales, read-only | — → 200 inline object | A contact's subscriptions across all lists. 404 when the record does not exist in this workspace. |
| `list_list_subscriptions` | `GET /lists/{id}/subscriptions` | owner, admin, marketer, sales, read-only | — → 200 `SubscriptionPage` | Subscriptions on a list. Paged (`page`, `limit` ≤ 100). Filters: `status`. 404 when the record does not exist in this workspace. |
| `subscribe` | `POST /subscriptions` | owner, admin, marketer | `SubscribeRequest` → 201 `Subscription` | Records a subscription with its consent source. With double opt-in on the list the status is PENDING until the confirmation link is followed. Re-subscribing an UNSUBSCRIBED contact requires a fresh consent source (a form submission or an explicit staff action with a note); an automation may not do it. Publishes `subscription.changed` on `crm.audience.v1`. |
| `get_subscription` | `GET /subscriptions/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Subscription` | Get a subscription with its consent history. 404 when the record does not exist in this workspace. |
| `unsubscribe` | `POST /subscriptions/{id}/unsubscribe` | owner, admin, marketer | `UnsubscribeRequest` → 200 `Subscription` | Unsubscribe a contact from a list. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `subscription.changed` on `crm.audience.v1`. |
| `list_suppressions` | `GET /suppressions` | owner, admin, marketer, sales, read-only | — → 200 `SuppressionPage` | Suppressed addresses. Paged (`page`, `limit` ≤ 100). Filters: `search`, `reason`. |
| `create_suppression` | `POST /suppressions` | owner, admin, marketer | `CreateSuppressionRequest` → 201 `Suppression` | Suppress an address manually. Audited. Publishes `suppression.added` on `crm.audience.v1`. |
| `delete_suppression` | `DELETE /suppressions/{id}` | owner, admin, marketer | — → 204 — | Only MANUAL suppressions can be lifted. COMPLAINT and HARD_BOUNCE are permanent. Publishes `suppression.removed` on `crm.audience.v1`. |

## Steps

1. Services: `microservices/crm/audience/impl/src/services/lists.rs` (add `pub mod lists;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/lists.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/audience/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `subscription.changed`, `suppression.added`, `suppression.removed` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
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
cd microservices && cargo clippy -p rerp_crm_audience --all-targets -- -D warnings && cargo test -p rerp_crm_audience
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
