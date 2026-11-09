# CRM-ACQ-04: crm/acquisition — Ad buys and creatives

**Service:** crm/acquisition · **Areas:** `ad-buying` (14 operations) · **Build step:** 5 · **Depends on:** CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_31, US_55_33, US_55_35 · **Points:** 6

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_31, US_55_33, US_55_35 run against crm/acquisition instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/acquisition/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/acquisition/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/acquisition.yaml` — tables: `ad_buys`, `ad_creatives`.
- `openapi/crm/docs/10-events.md` — the events published here: `ad_buy.changed`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_31_*.md` — what the screen expects (Ad accounts and adapters).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_33_*.md` — what the screen expects (Ad buys).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_35_*.md` — what the screen expects (Ad attribution and LTV).

## Rules for this area

Through the adapter interface (docs 07-ad-buying-landscape.md); no platform SDK leaks outside the adapter.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_ad_adapters` | `GET /ads/adapters` | owner, admin, marketer, sales, read-only | — → 200 inline object | The buying adapters this deployment ships, with the capability matrix per platform (audiences, conversions, spend, campaign create / pause / budget / bid, creative upload, placements, frequency caps, deal ids). |
| `list_ad_buys` | `GET /ads/buys` | owner, admin, marketer, sales, read-only | — → 200 `AdBuyPage` | Campaigns we created through an adapter (not the imported platform campaigns): objective, budget, audience, placements, status, delivery. Paged (`page`, `limit` ≤ 100). Filters: `account_id`, `status`, `purpose`. |
| `create_ad_buy` | `POST /ads/buys` | owner, admin, marketer | `CreateAdBuyRequest` → 201 `AdBuy` | The spec is what every adapter can do; anything an adapter cannot map is reported in `unsupported[]` rather than silently dropped. `audience_id` binds a synced `AdAudience` (and its exclusion); `warm_up_for_campaign_id` ties the buy to the outreach campaign it warms up, which sets the schedule from the send time and the audience's `days_before_send`. Publishes `ad_buy.changed` on `crm.acquisition.v1`. |
| `get_ad_buy` | `GET /ads/buys/{id}` | owner, admin, marketer, sales, read-only | — → 200 `AdBuy` | One buy with delivery (spend, impressions, reach, frequency) per account. 404 when the record does not exist in this workspace. |
| `update_ad_buy` | `PATCH /ads/buys/{id}` | owner, admin, marketer | `UpdateAdBuyRequest` → 200 `AdBuy` | Change budget, bid, schedule, frequency cap, creatives or status; pushed to every account. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. Publishes `ad_buy.changed` on `crm.acquisition.v1`. |
| `delete_ad_buy` | `DELETE /ads/buys/{id}` | owner, admin, marketer | — → 204 — | End the buy and archive the platform campaigns. Audited. 404 when the record does not exist in this workspace. |
| `apply_ad_recommendation` | `POST /ads/buys/{id}/apply-recommendation` | owner, admin, marketer | inline object → 200 `AdBuy` | Apply a scale / cut recommendation (budget change or pause) from the attribution report, with an audit note. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `ad_buy.changed` on `crm.acquisition.v1`. |
| `get_ad_buy_delivery` | `GET /ads/buys/{id}/delivery` | owner, admin, marketer, sales, read-only | — → 200 `AdBuyDelivery` | Delivery by day and account: spend, impressions, reach, frequency, clicks, plus CRM-attributed outcomes. Query: `from`, `to`. 404 when the record does not exist in this workspace. |
| `pause_ad_buy` | `POST /ads/buys/{id}/pause` | owner, admin, marketer | — → 200 `AdBuy` | Pause on every account. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `ad_buy.changed` on `crm.acquisition.v1`. |
| `resume_ad_buy` | `POST /ads/buys/{id}/resume` | owner, admin, marketer | — → 200 `AdBuy` | Resume on every account. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `ad_buy.changed` on `crm.acquisition.v1`. |
| `list_ad_creatives` | `GET /ads/creatives` | owner, admin, marketer, sales, read-only | — → 200 `AdCreativePage` | Creative assets registered for buying (image, video, HTML5, native, text), with per-platform upload status. Paged (`page`, `limit` ≤ 100). Filters: `kind`, `account_id`. |
| `create_ad_creative` | `POST /ads/creatives` | owner, admin, marketer | `CreateAdCreativeRequest` → 201 `AdCreative` | Register a creative (asset from the outreach asset store or a URL) and push it to the chosen accounts. Audited. |
| `get_ad_creative` | `GET /ads/creatives/{id}` | owner, admin, marketer, sales, read-only | — → 200 `AdCreative` | One creative with its platform ids and review status. 404 when the record does not exist in this workspace. |
| `delete_ad_creative` | `DELETE /ads/creatives/{id}` | owner, admin, marketer | — → 204 — | Remove locally; platform copies are archived. Audited. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/acquisition/impl/src/services/ad_buying.rs` (add `pub mod ad_buying;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/ad_buying.rs`.
4. Idempotency: `apply_ad_recommendation` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/acquisition/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `ad_buy.changed` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `apply_ad_recommendation` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_acquisition --all-targets -- -D warnings && cargo test -p rerp_crm_acquisition
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
