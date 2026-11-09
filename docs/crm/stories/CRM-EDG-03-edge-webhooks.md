# CRM-EDG-03: crm/edge — Provider and ad-platform webhooks and OAuth callbacks

**Service:** crm/edge · **Areas:** `webhooks`, `provider-webhooks`, `oauth` (3 operations) · **Build step:** 2 · **Depends on:** CRM-RT-01, CRM-RT-02, CRM-RT-04 · **Serves (PriceWhisperer Epic 55):** — · **Points:** 3

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of this area run against crm/edge instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/edge/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/edge/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/10-events.md` — the events published here: `ad_webhook.received`, `oauth.completed`, `provider_event.received`.

## Rules for this area

Verify the provider or platform signature before publishing; an invalid signature is 401 and publishes nothing. OAuth callbacks verify the signed state and redirect; tokens never reach the browser.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `ad_platform_webhook` | `POST /webhooks/ads/{platform}` | public | inline object → 202 — | Verifies the platform's signature and publishes ad.webhook.received. Lead-form leads go through crm/intake like any other form. Publishes `ad_webhook.received` on `crm.edge.v1`. |
| `ingest_provider_events` | `POST /webhooks/provider/{provider}` | public | inline object → 200 — | Verified by the provider's signature scheme (header named per provider). Every event is normalized into a MessageEvent keyed by the provider message id. Idempotent on provider event id. Unknown events are acked and dropped. Deployments should expose this only through a keyless relay (the same shape as the crm-conversions intake in ADR 003), never directly from the provider to the service. Publishes `provider_event.received` on `crm.edge.v1`. |
| `oauth_callback` | `GET /oauth/{flow}/callback` | public | — →  — | Verifies the signed state and publishes oauth.completed for the service named in it; tokens go to the secrets service, never to the browser. Publishes `oauth.completed` on `crm.edge.v1`. |

## Steps

1. Services: `microservices/crm/edge/impl/src/services/webhooks.rs` (add `pub mod webhooks;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/webhooks.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/edge/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `ad_webhook.received`, `oauth.completed`, `provider_event.received` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
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
