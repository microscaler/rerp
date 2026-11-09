# CRM-MSG-05: crm/messaging — Connected mailboxes (1:1 email)

**Service:** crm/messaging · **Areas:** `mailboxes` (8 operations) · **Build step:** 3 · **Depends on:** CRM-MSG-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_26 · **Points:** 4

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_26 run against crm/messaging instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/messaging/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/messaging/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/messaging.yaml` — tables: `mailboxes`, `mailbox_messages`.
- `openapi/crm/docs/10-events.md` — the events published here: `message.sent`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_26_*.md` — what the screen expects (Connected mailboxes).

## Rules for this area

OAuth through the edge callback; tokens in Secret Manager. Sync logs messages to deals and contacts by address.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_mailboxes` | `GET /mailboxes` | owner, admin, marketer, sales, read-only | — → 200 `MailboxPage` | Mailboxes the caller owns or that are shared with them. |
| `connect_mailbox` | `POST /mailboxes` | owner, admin | `ConnectMailboxRequest` → 201 `ConnectMailboxResponse` | Start an OAuth connection (returns authorize_url) or connect an IMAP/SMTP account directly. |
| `get_mailbox` | `GET /mailboxes/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Mailbox` | Settings and sync state of one mailbox. |
| `update_mailbox` | `PATCH /mailboxes/{id}` | owner, admin | `UpdateMailboxRequest` → 200 `Mailbox` | Sync, tracking, signature, sharing and IMAP settings. |
| `disconnect_mailbox` | `DELETE /mailboxes/{id}` | owner, admin | — → 204 — | Revokes tokens and stops sync; synced messages stay on the timelines. |
| `list_mailbox_messages` | `GET /mailboxes/{id}/messages` | owner, admin, marketer, sales, read-only | — → 200 `MailboxMessagePage` | Messages synced from or sent through this mailbox. |
| `send_mailbox_message` | `POST /mailboxes/{id}/send` | owner, admin, marketer, sales | `SendMailboxMessageRequest` → 201 `MailboxMessage` | Send (or schedule) a one-to-one email from this mailbox as its user, logged to the contact / deal. Publishes `message.sent` on `crm.messaging.v1`. |
| `sync_mailbox` | `POST /mailboxes/{id}/sync` | owner, admin, marketer, sales | — → 202 `Mailbox` | Queue an immediate sync; returns the mailbox with status SYNCING. |

## Steps

1. Services: `microservices/crm/messaging/impl/src/services/mailboxes.rs` (add `pub mod mailboxes;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/mailboxes.rs`.
4. Idempotency: `send_mailbox_message`, `sync_mailbox` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/messaging/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `message.sent` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `send_mailbox_message`, `sync_mailbox` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
