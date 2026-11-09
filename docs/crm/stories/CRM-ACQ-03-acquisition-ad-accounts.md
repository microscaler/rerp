# CRM-ACQ-03: crm/acquisition — Ad accounts, audiences and sync

**Service:** crm/acquisition · **Areas:** `ad-audiences` (18 operations) · **Build step:** 5 · **Depends on:** CRM-ACQ-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_31, US_55_32, US_55_33, US_55_34 · **Points:** 8

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_31, US_55_32, US_55_33, US_55_34 run against crm/acquisition instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/acquisition/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/acquisition/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/acquisition.yaml` — tables: `ad_accounts`, `ad_audiences`, `ad_audience_members`, `ad_audience_sync_runs`.
- `openapi/crm/docs/10-events.md` — the events published here: `ad_audience.synced`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_31_*.md` — what the screen expects (Ad accounts and adapters).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_32_*.md` — what the screen expects (Ad audiences and warm-up).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_33_*.md` — what the screen expects (Ad buys).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_34_*.md` — what the screen expects (Ad tracking and conversion relay).

## Rules for this area

Upload only hashed identifiers; respect consent basis and suppressions; an audience below the platform minimum is not synced.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_ad_accounts` | `GET /ads/accounts` | owner, admin, marketer, sales, read-only | — → 200 inline object | Connected ad platforms. |
| `connect_ad_account` | `POST /ads/accounts` | owner, admin | `ConnectAdAccountRequest` → 201 `AdAccount` | Connect an ad platform account (OAuth or API key). Audited. |
| `get_ad_account` | `GET /ads/accounts/{id}` | owner, admin, marketer, sales, read-only | — → 200 `AdAccount` | One ad account with its capabilities and last sync. 404 when the record does not exist in this workspace. |
| `update_ad_account` | `PATCH /ads/accounts/{id}` | owner, admin | `UpdateAdAccountRequest` → 200 `AdAccount` | Rename, change the default conversion event names, pause. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `disconnect_ad_account` | `DELETE /ads/accounts/{id}` | owner, admin | — → 204 — | Disconnect; audiences on it stop syncing (platform-side audiences are left in place). Audited. 404 when the record does not exist in this workspace. |
| `reconnect_ad_account` | `POST /ads/accounts/{id}/reconnect` | owner, admin | — → 200 `AdAccount` | Re-run the OAuth flow after a token expiry. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_ad_audiences` | `GET /ads/audiences` | owner, admin, marketer, sales, read-only | — → 200 `AdAudiencePage` | Audience bindings (a segment, list or tag kept in step with a platform custom audience). Paged (`page`, `limit` ≤ 100). Filters: `account_id`, `status`. |
| `create_ad_audience` | `POST /ads/audiences` | owner, admin, marketer | `CreateAdAudienceRequest` → 201 `AdAudience` | Identifiers are normalised and SHA-256 hashed server-side before leaving the workspace; raw emails and phones never reach the platform. `consent_basis` is recorded on the audience and on every membership row. Contacts that are unsubscribed, suppressed, erased, or match `suppress_on` are removed on the next sync. |
| `preview_ad_audience` | `POST /ads/audiences/preview` | owner, admin, marketer | `CreateAdAudienceRequest` → 200 `AdAudiencePreview` | Count the source and estimate the platform match before creating the audience. Audited. |
| `get_ad_audience` | `GET /ads/audiences/{id}` | owner, admin, marketer, sales, read-only | — → 200 `AdAudience` | One audience with its stats. 404 when the record does not exist in this workspace. |
| `update_ad_audience` | `PATCH /ads/audiences/{id}` | owner, admin, marketer | `UpdateAdAudienceRequest` → 200 `AdAudience` | Change source, identifiers, suppression rules, lookalike or mode. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `delete_ad_audience` | `DELETE /ads/audiences/{id}` | owner, admin, marketer | — → 204 — | Stop syncing and, when `delete_remote=true`, delete the platform audience. Audited. 404 when the record does not exist in this workspace. |
| `list_ad_audience_members` | `GET /ads/audiences/{id}/members` | owner, admin, marketer, sales, read-only | — → 200 `AdAudienceMemberPage` | Membership rows: who is in the platform audience, when they were added / removed and why. Paged (`page`, `limit` ≤ 100). Filters: `status`. 404 when the record does not exist in this workspace. |
| `pause_ad_audience` | `POST /ads/audiences/{id}/pause` | owner, admin, marketer | — → 200 `AdAudience` | Pause syncing; members stay on the platform. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `resume_ad_audience` | `POST /ads/audiences/{id}/resume` | owner, admin, marketer | — → 200 `AdAudience` | Resume syncing. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_ad_audience_runs` | `GET /ads/audiences/{id}/runs` | owner, admin, marketer, sales, read-only | — → 200 `AdAudienceSyncRunPage` | Sync history. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `sync_ad_audience` | `POST /ads/audiences/{id}/sync` | owner, admin, marketer | — → 202 `AdAudienceSyncRun` | Run a sync now (adds, removals, re-hash after an identifier change). A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `ad_audience.synced` on `crm.acquisition.v1`. |
| `list_contact_ad_audiences` | `GET /ads/contacts/{id}/audiences` | owner, admin, marketer, sales, read-only | — → 200 inline object | The audiences one contact is in (shown on the contact profile). 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/acquisition/impl/src/services/ad_accounts.rs` (add `pub mod ad_accounts;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/ad_accounts.rs`.
4. Idempotency: `reconnect_ad_account`, `sync_ad_audience` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/acquisition/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `ad_audience.synced` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `reconnect_ad_account`, `sync_ad_audience` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
