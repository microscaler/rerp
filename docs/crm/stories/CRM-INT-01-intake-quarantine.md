# CRM-INT-01: crm/intake — The quarantine: screening, confirmation, admission and the staff view

**Service:** crm/intake · **Areas:** `submissions` (4 operations) · **Build step:** 2 · **Depends on:** CRM-INT-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** — · **Points:** 3

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of this area run against crm/intake instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/intake/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/intake/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/intake.yaml` — tables: `submissions`, `rate_limits`.
- `openapi/crm/docs/10-events.md` — the events published here: `intake.confirmation_requested`.

## Rules for this area

Consume `form_submission.received`, `survey.received`, `event_registration.received`, `ad_webhook.received` and `confirmation.received`. Screen (honeypot, rate per address and IP, syntax, disposable domains); store PENDING; publish `intake.confirmation_requested` for subscriptions; on confirmation (or at once for an enquiry) publish `intake.admitted` with the consent evidence. A job expires pending rows after 7 days and deletes decided rows after 30. The API never returns a full email address.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_submissions` | `GET /submissions` | owner, admin, marketer, sales, read-only | — → 200 `IntakeSubmissionPage` | Pending and recent submissions. Paged (`page`, `limit` ≤ 100). Filters: `status`, `form_id`. |
| `get_submission` | `GET /submissions/{id}` | owner, admin, marketer, sales, read-only | — → 200 `IntakeSubmission` | A submission. 404 when the record does not exist in this workspace. |
| `discard_submission` | `DELETE /submissions/{id}` | owner, admin, marketer | — → 204 — | Discard a submission. Audited. 404 when the record does not exist in this workspace. |
| `resend_confirmation` | `POST /submissions/{id}/resend` | owner, admin, marketer | — → 200 `IntakeSubmission` | 409 unless PENDING; at most 3 sends per submission. Publishes `intake.confirmation_requested` on `crm.intake.v1`. |

## Steps

1. Services: `microservices/crm/intake/impl/src/services/quarantine.rs` (add `pub mod quarantine;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/quarantine.rs`.
4. Idempotency: `resend_confirmation` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/intake/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `intake.confirmation_requested` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `resend_confirmation` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_intake --all-targets -- -D warnings && cargo test -p rerp_crm_intake
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
