# CRM-ACQ-05: crm/acquisition — Ad tracking, conversion relay and attribution

**Service:** crm/acquisition · **Areas:** `ad-tracking`, `ad-attribution` (26 operations) · **Build step:** 5 · **Depends on:** CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_31, US_55_34, US_55_35 · **Points:** 10

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_31, US_55_34, US_55_35 run against crm/acquisition instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/acquisition/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/acquisition/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/acquisition.yaml` — tables: `ad_clicks`, `ad_link_tag_rules`, `ad_conversion_rules`, `ad_conversions`, `touchpoints`, `attribution_models`, `ad_spend`, `ad_settings`.
- `openapi/crm/docs/10-events.md` — the events published here: `conversion.recorded`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_31_*.md` — what the screen expects (Ad accounts and adapters).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_34_*.md` — what the screen expects (Ad tracking and conversion relay).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_35_*.md` — what the screen expects (Ad attribution and LTV).

## Rules for this area

Relay conversions once per account and dedupe key; record every attempt. Attribution models are computed over touchpoints within the window.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `import_ad_spend` | `POST /ads/accounts/{id}/import-spend` | owner, admin | inline object → 202 `AdSpendImport` | Pull spend, impressions and clicks per campaign / ad set / ad for a date range. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_ad_clicks` | `GET /ads/clicks` | owner, admin, marketer, sales, read-only | — → 200 `AdClickPage` | Recorded ad clicks. Paged (`page`, `limit` ≤ 100). Filters: `platform`, `contact_id`, `visitor_id`, `from`, `to`. |
| `get_ad_click` | `GET /ads/clicks/{id}` | owner, admin, marketer, sales, read-only | — → 200 `AdClick` | One click with the touchpoints it started. 404 when the record does not exist in this workspace. |
| `list_conversion_rules` | `GET /ads/conversion-rules` | owner, admin, marketer, sales, read-only | — → 200 inline object | Which CRM events are sent to which platform as conversions (Meta CAPI, Google offline / enhanced conversions, LinkedIn CAPI, TikTok Events API). |
| `create_conversion_rule` | `POST /ads/conversion-rules` | owner, admin | `AdConversionRule` → 201 `AdConversionRule` | Add a rule. Audited. |
| `get_conversion_rule` | `GET /ads/conversion-rules/{id}` | owner, admin, marketer, sales, read-only | — → 200 `AdConversionRule` | One rule. 404 when the record does not exist in this workspace. |
| `update_conversion_rule` | `PATCH /ads/conversion-rules/{id}` | owner, admin | `AdConversionRuleInput` → 200 `AdConversionRule` | Change a rule. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_conversion_rule` | `DELETE /ads/conversion-rules/{id}` | owner, admin | — → 204 — | Remove a rule. Audited. 404 when the record does not exist in this workspace. |
| `list_ad_conversions` | `GET /ads/conversions` | owner, admin, marketer, sales, read-only | — → 200 `AdConversionPage` | Conversions sent (or failed) to the platforms. Paged (`page`, `limit` ≤ 100). Filters: `account_id`, `status`, `event`. |
| `record_ad_conversion` | `POST /ads/conversions` | owner, admin, marketer | `RecordAdConversionRequest` → 201 inline object | Internal (service-to-service). The contact's stored click ids, hashed identifiers and the first-party cookie id are attached so the platform can match without a browser pixel; `value` defaults from the rule's `value_from`. Publishes `conversion.recorded` on `crm.acquisition.v1`. |
| `retry_ad_conversion` | `POST /ads/conversions/{id}/retry` | owner, admin, marketer | — → 200 `AdConversion` | Resend a failed conversion. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_ad_link_tags` | `GET /ads/link-tags` | owner, admin, marketer, sales, read-only | — → 200 inline object | Rules that decorate outbound links (emails, SMS, social posts) with `pw_cid` and UTM so later visits join the journey. |
| `create_ad_link_tag` | `POST /ads/link-tags` | owner, admin, marketer | `AdLinkTagRule` → 201 `AdLinkTagRule` | Add a decoration rule. Audited. |
| `update_ad_link_tag` | `PATCH /ads/link-tags/{id}` | owner, admin, marketer | `AdLinkTagRuleInput` → 200 `AdLinkTagRule` | Change a rule. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_ad_link_tag` | `DELETE /ads/link-tags/{id}` | owner, admin, marketer | — → 204 — | Remove a rule. Audited. 404 when the record does not exist in this workspace. |
| `get_ad_tracking_settings` | `GET /ads/settings` | owner, admin, marketer, sales, read-only | — → 200 `AdTrackingSettings` | Workspace ad-tracking settings: first-party cookie, click-id parameters, link decoration, consent mode. |
| `update_ad_tracking_settings` | `PUT /ads/settings` | owner, admin | `AdTrackingSettings` → 200 `AdTrackingSettings` | Replace the settings. Replaces the resource with the body; audited. |
| `get_ad_attribution` | `GET /ads/attribution` | owner, admin, marketer, sales, read-only | — → 200 `AdAttributionReport` | Revenue, deals and leads by channel / campaign / ad under a model and window, with the ad-platform-reported figures beside them. Query: `model`, `window_days`, `by`, `from`, `to`. |
| `list_attribution_models` | `GET /ads/attribution/models` | owner, admin, marketer, sales, read-only | — → 200 inline object | Built-in and custom attribution models. |
| `create_attribution_model` | `POST /ads/attribution/models` | owner, admin | `AttributionModel` → 201 `AttributionModel` | Add a custom model (weights per position, window, which touchpoint kinds count). Audited. |
| `update_attribution_model` | `PATCH /ads/attribution/models/{id}` | owner, admin | `AttributionModelInput` → 200 `AttributionModel` | Change a custom model or make it the default. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_attribution_model` | `DELETE /ads/attribution/models/{id}` | owner, admin | — → 204 — | Remove a custom model. Audited. 404 when the record does not exist in this workspace. |
| `list_ad_campaigns` | `GET /ads/campaigns` | owner, admin, marketer, sales, read-only | — → 200 `AdCampaignRowPage` | Platform campaigns / ad sets / ads with imported spend next to CRM-attributed leads, deals and revenue (platform-reported vs attributed). Paged (`page`, `limit` ≤ 100). Filters: `account_id`, `level`, `from`, `to`, `model`. |
| `get_contact_journey` | `GET /ads/contacts/{id}/journey` | owner, admin, marketer, sales, read-only | — → 200 `ContactJourney` | Every touchpoint for one contact in order: ad clicks, site visits, form, emails opened / clicked, calls, deal events, payments — with the identifiers that joined them. Query: `model`. 404 when the record does not exist in this workspace. |
| `get_ad_ltv` | `GET /ads/ltv` | owner, admin, marketer, sales, read-only | — → 200 `AdLtvReport` | Lifetime value and payback by acquisition channel / campaign, in monthly cohorts. Query: `by`, `cohort_months`, `from`. |
| `list_ad_spend` | `GET /ads/spend` | owner, admin, marketer, sales, read-only | — → 200 `AdSpendRowPage` | Imported spend per account and day. Paged (`page`, `limit` ≤ 100). Filters: `account_id`, `from`, `to`. |

## Steps

1. Services: `microservices/crm/acquisition/impl/src/services/ad_tracking.rs` (add `pub mod ad_tracking;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/ad_tracking.rs`.
4. Idempotency: `import_ad_spend`, `retry_ad_conversion` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/acquisition/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `conversion.recorded` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `import_ad_spend`, `retry_ad_conversion` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
