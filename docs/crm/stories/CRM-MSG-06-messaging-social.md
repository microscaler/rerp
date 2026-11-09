# CRM-MSG-06: crm/messaging — Social accounts, posts, approval and engagement

**Service:** crm/messaging · **Areas:** `social-accounts`, `social-posts`, `social-calendar`, `social-engagement`, `social`, `social-leads` (19 operations) · **Build step:** 3 · **Depends on:** CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_17 · **Points:** 8

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_17 run against crm/messaging instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/messaging/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/messaging/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/messaging.yaml` — tables: `social_accounts`, `social_posts`, `social_post_targets`, `social_post_audit`.
- `openapi/crm/docs/10-events.md` — the events published here: `social.post.published`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_17_*.md` — what the screen expects (Social publishing).

## Rules for this area

Posts needing approval cannot be scheduled until approved. Publishing goes through the platform adapter with retries; failures publish `social.post.failed`. Social leads go through intake like any other capture.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_social_accounts` | `GET /social/accounts` | owner, admin, marketer, sales, read-only | — → 200 inline object | Connected social accounts. |
| `connect_social_account` | `POST /social/accounts` | owner, admin | `ConnectSocialAccountRequest` → 201 `SocialAccount` | Returns the OAuth authorization URL for the platform; the callback completes the connection. |
| `get_social_account` | `GET /social/accounts/{id}` | owner, admin, marketer, sales, read-only | — → 200 `SocialAccount` | Get an account. 404 when the record does not exist in this workspace. |
| `disconnect_social_account` | `DELETE /social/accounts/{id}` | owner, admin | — → 204 — | Disconnect an account. Audited. 404 when the record does not exist in this workspace. |
| `pause_social_account` | `POST /social/accounts/{id}/pause` | owner, admin | inline object → 200 `SocialAccount` | Pause or resume publishing on an account. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_social_audit` | `GET /social/audit` | owner, admin, marketer, sales, read-only | — → 200 `AuditEntryPage` | Who did what to the feed. Paged (`page`, `limit` ≤ 100). |
| `list_social_posts` | `GET /social/posts` | owner, admin, marketer, sales, read-only | — → 200 `SocialPostPage` | List posts. Paged (`page`, `limit` ≤ 100). Filters: `search`, `status`, `account_id`, `from`, `to`. |
| `create_social_post` | `POST /social/posts` | owner, admin, marketer | `CreateSocialPostRequest` → 201 `SocialPost` | Create a post (draft or scheduled). Audited. |
| `get_social_post` | `GET /social/posts/{id}` | owner, admin, marketer, sales, read-only | — → 200 `SocialPost` | Get a post with its per-platform results. 404 when the record does not exist in this workspace. |
| `update_social_post` | `PUT /social/posts/{id}` | owner, admin, marketer | `UpdateSocialPostRequest` → 200 `SocialPost` | Update a draft or scheduled post. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `delete_social_post` | `DELETE /social/posts/{id}` | owner, admin, marketer | — → 204 — | Delete a draft or scheduled post. Audited. 409 while something still uses it. 404 when the record does not exist in this workspace. |
| `approve_social_post` | `POST /social/posts/{id}/approve` | owner, admin, marketer | — → 200 `SocialPost` | Approve a post awaiting approval. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `publish_social_post_now` | `POST /social/posts/{id}/publish` | owner, admin, marketer | — → 202 `SocialPost` | Publish immediately. A state change on one resource; audited. 409 when the current state does not allow it. 404 when the record does not exist in this workspace. Publishes `social.post.published` on `crm.messaging.v1`. |
| `skip_social_post` | `POST /social/posts/{id}/skip` | owner, admin, marketer | inline object → 200 `SocialPost` | Skip a scheduled post with a reason. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `get_social_calendar` | `GET /social/calendar` | owner, admin, marketer, sales, read-only | — → 200 `SocialCalendar` | Posts by day for a date range. Query: `from`, `to`. |
| `get_account_engagement` | `GET /social/accounts/{id}/engagement` | owner, admin, marketer, sales, read-only | — → 200 `AccountEngagementSeries` | Engagement over time for an account. Query: `days`. 404 when the record does not exist in this workspace. |
| `get_post_engagement` | `GET /social/posts/{id}/engagement` | owner, admin, marketer, sales, read-only | — → 200 inline object | Engagement metrics for a post. 404 when the record does not exist in this workspace. |
| `get_social_engagement` | `GET /social/engagement` | owner, admin, marketer, sales, read-only | — → 200 `SocialEngagementSummary` | Engagement per connected account. Query: `from`, `to`. |
| `capture_social_lead` | `POST /social/leads/capture` | owner, admin, marketer | `SocialLeadCaptureRequest` → 201 inline object | Create a lead in crm/pipeline from a social interaction. Audited. |

## Steps

1. Services: `microservices/crm/messaging/impl/src/services/social.rs` (add `pub mod social;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/social.rs`.
4. Idempotency: `approve_social_post`, `publish_social_post_now` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/messaging/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `social.post.published` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `approve_social_post`, `publish_social_post_now` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
