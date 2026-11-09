# CRM-SAL-05: crm/sales — Teams, members and assignment

**Service:** crm/sales · **Areas:** `teams`, `members` (10 operations) · **Build step:** 4 · **Depends on:** CRM-SAL-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** — · **Points:** 5

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of this area run against crm/sales instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/sales/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/sales/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/sales.yaml` — tables: `teams`, `team_members`.

## Rules for this area

Round-robin assignment respects member opt-out and max load.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `run_assignment` | `POST /assign/run` | owner, admin | — → 200 `AssignmentResult` | Run a workflow manually |
| `list_unassigned` | `GET /queues/unassigned` | owner, admin, marketer, sales, read-only | — → 200 `UnassignedLeadPage` | Get unassigned leads in the queue |
| `list_teams` | `GET /teams` | owner, admin, marketer, sales, read-only | — → 200 `TeamPage` | List all teams |
| `create_team` | `POST /teams` | owner, admin | `CreateTeamRequest` → 201 `Team` | Create a new teams |
| `get_team` | `GET /teams/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Team` | Get teams by ID |
| `update_team` | `PUT /teams/{id}` | owner, admin | `UpdateTeamRequest` → 200 `Team` | Update teams by ID |
| `delete_team` | `DELETE /teams/{id}` | owner, admin | — → 204 — | Delete teams by ID |
| `list_team_members` | `GET /teams/{id}/members` | owner, admin, marketer, sales, read-only | — → 200 `TeamMemberPage` | Get team members |
| `add_team_member` | `POST /teams/{id}/members` | owner, admin | `AddTeamMemberRequest` → 201 `TeamMember` | Get team members |
| `remove_team_member` | `DELETE /teams/{id}/members/{member_id}` | owner, admin | — → 204 — | Get team members |

## Steps

1. Services: `microservices/crm/sales/impl/src/services/teams.rs` (add `pub mod teams;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/teams.rs`.
4. Idempotency: `run_assignment` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/sales/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-6 **Idempotency.** Repeating `run_assignment` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_sales --all-targets -- -D warnings && cargo test -p rerp_crm_sales
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
