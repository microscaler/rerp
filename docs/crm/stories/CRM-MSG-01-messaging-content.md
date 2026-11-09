# CRM-MSG-01: crm/messaging — Templates, snippets, assets and postal addresses

**Service:** crm/messaging · **Areas:** `templates`, `snippets`, `assets`, `postal-addresses` (17 operations) · **Build step:** 3 · **Depends on:** CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_06, US_55_08, US_55_11, US_55_12, US_55_29, US_55_33 · **Points:** 7

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_06, US_55_08, US_55_11, US_55_12, US_55_29, US_55_33 run against crm/messaging instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/messaging/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/messaging/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/messaging.yaml` — tables: `templates`, `template_versions`, `snippets`, `assets`, `postal_addresses`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_06_*.md` — what the screen expects (Lists and consent).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_08_*.md` — what the screen expects (Email campaigns — list and wizard).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_11_*.md` — what the screen expects (Templates, snippets and assets).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_12_*.md` — what the screen expects (Automations and the automation builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_33_*.md` — what the screen expects (Ad buys).

## Rules for this area

Every template save creates a version; sends record the version used. A snippet used by a template cannot be deleted (409 `in_use`). Assets are uploaded with a signed URL to object storage; only metadata is stored.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_merge_fields` | `GET /merge-fields` | owner, admin, marketer, sales, read-only | — → 200 inline object | Contact fields, custom fields (crm/platform), list and campaign fields, and system links (unsubscribe, preferences, web view). |
| `list_templates` | `GET /templates` | owner, admin, marketer, sales, read-only | — → 200 `TemplatePage` | List message templates. Paged (`page`, `limit` ≤ 100). Filters: `search`, `channel`, `kind`. |
| `create_template` | `POST /templates` | owner, admin, marketer | `CreateTemplateRequest` → 201 `Template` | Create a message template. Audited. |
| `get_template` | `GET /templates/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Template` | Get a template. 404 when the record does not exist in this workspace. |
| `update_template` | `PUT /templates/{id}` | owner, admin, marketer | `UpdateTemplateRequest` → 200 `Template` | Update a template (creates a new version). Replaces the resource with the body; audited. 404 when the record does not exist in this workspace. |
| `delete_template` | `DELETE /templates/{id}` | owner, admin, marketer | — → 204 — | Archive a template. Audited. 409 while something still uses it. 404 when the record does not exist in this workspace. |
| `render_template` | `POST /templates/{id}/render` | owner, admin, marketer | `RenderRequest` → 200 `RenderedMessage` | Merge fields are resolved from the given contact, or from the sample values supplied. Nothing is sent. |
| `test_send_template` | `POST /templates/{id}/test-send` | owner, admin, marketer | `TestSendRequest` → 202 `Send` | Send a template to a staff address. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_snippets` | `GET /snippets` | owner, admin, marketer, sales, read-only | — → 200 `SnippetPage` | Snippets. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_snippet` | `POST /snippets` | owner, admin, marketer | `SnippetRequest` → 201 `Snippet` | Create a snippet. Audited. |
| `update_snippet` | `PUT /snippets/{id}` | owner, admin, marketer | `SnippetRequest` → 200 `Snippet` | Templates that include it pick up the change on their next render. |
| `delete_snippet` | `DELETE /snippets/{id}` | owner, admin, marketer | — → 204 — | 409 while a template uses it. |
| `list_assets` | `GET /assets` | owner, admin, marketer, sales, read-only | — → 200 `AssetPage` | Images in the asset store. Paged (`page`, `limit` ≤ 100). Filters: `search`. |
| `create_asset` | `POST /assets` | owner, admin, marketer | `CreateAssetRequest` → 201 `AssetUpload` | Returns a signed URL to PUT the bytes to (images up to 5 MB). |
| `list_postal_addresses` | `GET /postal-addresses` | owner, admin, marketer, sales, read-only | — → 200 `PostalAddressPage` | Postal addresses for footers. Paged (`page`, `limit` ≤ 100). |
| `create_postal_address` | `POST /postal-addresses` | owner, admin | `PostalAddressRequest` → 201 `PostalAddress` | Add a postal address. Audited. |
| `update_postal_address` | `PATCH /postal-addresses/{id}` | owner, admin | `PostalAddressRequest` → 200 `PostalAddress` | Change a postal address. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |

## Steps

1. Services: `microservices/crm/messaging/impl/src/services/content.rs` (add `pub mod content;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/content.rs`.
4. Idempotency: `test_send_template` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/messaging/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-6 **Idempotency.** Repeating `test_send_template` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_messaging --all-targets -- -D warnings && cargo test -p rerp_crm_messaging
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
