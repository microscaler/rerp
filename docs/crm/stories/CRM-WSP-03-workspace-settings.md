# CRM-WSP-03: crm/workspace — Compliance, brand kit and notification rules

**Service:** crm/workspace · **Areas:** `compliance`, `brand`, `notifications` (7 operations) · **Build step:** 1 · **Depends on:** CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_29 · **Points:** 4

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_29 run against crm/workspace instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/workspace/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/workspace/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/workspace.yaml` — tables: `compliance_settings`, `brand_kits`, `notification_rules`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).

## Rules for this area

Singleton rows per workspace: create the default row lazily on first GET. Compliance settings drive intake (double opt-in, expiry) and messaging (footer, unsubscribe links).

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `get_compliance` | `GET /compliance` | owner, admin, marketer, sales, read-only | — → 200 `ComplianceSettings` | Consent, retention and cookie settings. |
| `update_compliance` | `PUT /compliance` | owner, admin | `ComplianceSettings` → 200 `ComplianceSettings` | Change compliance settings. Replaces the resource with the body; audited. |
| `get_brand_kit` | `GET /brand-kit` | owner, admin, marketer, sales, read-only | — → 200 `BrandKit` | The brand kit. |
| `update_brand_kit` | `PUT /brand-kit/{id}` | owner, admin | `BrandKitInput` → 200 `BrandKit` | Change the brand kit. Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `list_notification_rules` | `GET /notification-rules` | owner, admin, marketer, sales, read-only | — → 200 `NotificationRulePage` | Notification rules. Paged (`page`, `limit` ≤ 100). |
| `create_notification_rule` | `POST /notification-rules` | owner, admin | `NotificationRuleRequest` → 201 `NotificationRule` | Add a notification rule. Audited. |
| `update_notification_rule` | `PATCH /notification-rules/{id}` | owner, admin | `NotificationRuleRequest` → 200 `NotificationRule` | Change or switch a rule on / off. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/workspace/impl/src/services/settings.rs` (add `pub mod settings;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/settings.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/workspace/openapi.yaml`
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
cd microservices && cargo clippy -p rerp_crm_workspace --all-targets -- -D warnings && cargo test -p rerp_crm_workspace
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
