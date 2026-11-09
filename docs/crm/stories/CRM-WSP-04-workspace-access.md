# CRM-WSP-04: crm/workspace — API keys, webhooks and integrations

**Service:** crm/workspace · **Areas:** `api-keys`, `webhooks`, `integrations` (14 operations) · **Build step:** 1 · **Depends on:** CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_29 · **Points:** 6

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_29 run against crm/workspace instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/workspace/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/workspace/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/workspace.yaml` — tables: `api_keys`, `webhooks`, `webhook_delivery_logs`, `integrations`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).

## Rules for this area

API keys are shown once at creation and stored as a hash with a short prefix. Webhook secrets and integration credentials live in Secret Manager (`secret_ref`), never in a column or a response. Webhook delivery is a worker that consumes this workspace's events and records each attempt.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_api_keys` | `GET /api-keys` | owner, admin, marketer, sales, read-only | — → 200 `ApiKeyPage` | List all api-keys |
| `create_api_key` | `POST /api-keys` | owner | `CreateApiKeyRequest` → 201 inline object | Create a new api-keys |
| `validate_api_key` | `POST /auth/validate` | owner, admin, marketer, sales, read-only | inline object → 200 inline object | Validate an API key or token |
| `list_webhooks` | `GET /webhooks` | owner, admin, marketer, sales, read-only | — → 200 `WebhookPage` | List all webhooks |
| `create_webhook` | `POST /webhooks` | owner, admin | `CreateWebhookRequest` → 201 `Webhook` | Create a new webhooks |
| `get_webhook` | `GET /webhooks/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Webhook` | Get webhooks by ID |
| `update_webhook` | `PUT /webhooks/{id}` | owner, admin | `UpdateWebhookRequest` → 200 `Webhook` | Update webhooks by ID |
| `delete_webhook` | `DELETE /webhooks/{id}` | owner, admin | — → 204 — | Delete webhooks by ID |
| `list_webhook_logs` | `GET /webhooks/{id}/logs` | owner, admin, marketer, sales, read-only | — → 200 `WebhookDeliveryLogPage` | Get webhook delivery logs |
| `test_webhook` | `POST /webhooks/{id}/test` | owner, admin | — → 200 inline object | Test a rule configuration |
| `list_integrations` | `GET /integrations` | owner, admin, marketer, sales, read-only | — → 200 `IntegrationPage` | List all integrations |
| `create_integration` | `POST /integrations` | owner, admin | `CreateIntegrationRequest` → 201 `Integration` | Create a new integrations |
| `get_integration_status` | `GET /integrations/{id}/status` | owner, admin, marketer, sales, read-only | — → 200 `Integration` | Get or update agent status |
| `trigger_integration_sync` | `POST /integrations/{id}/sync` | owner, admin | — → 200 inline object | Sync integration data with external system |

## Steps

1. Services: `microservices/crm/workspace/impl/src/services/access.rs` (add `pub mod access;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/access.rs`.
4. Idempotency: `trigger_integration_sync` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/workspace/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-6 **Idempotency.** Repeating `trigger_integration_sync` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
| TC-6 | Backend | Repeat an idempotent command with the same key / same key, other body | Same response, one effect / 409 |
| TC-7 | Backend | Each 409 / 422 case in the descriptions | The stated status and reason |

## Verify

```bash
python3 openapi/crm/tools/check_specs.py
cd microservices && cargo clippy -p rerp_crm_workspace --all-targets -- -D warnings && cargo test -p rerp_crm_workspace
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
