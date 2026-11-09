# CRM-WSP-01: crm/workspace — Workspaces, memberships and the team facade

**Service:** crm/workspace · **Areas:** `workspaces`, `team` (9 operations) · **Build step:** 1 · **Depends on:** CRM-WSP-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_22, US_55_29 · **Points:** 5

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_22, US_55_29 run against crm/workspace instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/workspace/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/workspace/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/workspace.yaml` — tables: `workspaces`, `memberships`, `invitations`.
- `openapi/crm/docs/10-events.md` — the events published here: `member.changed`, `workspace.created`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_22_*.md` — what the screen expects (Tasks and task settings).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).

## Rules for this area

Workspaces are the tenancy boundary every other service relies on (ADR 004). `GET /workspaces` and the membership operations name the workspace in the path and take no `X-Workspace-Id` (`x-no-workspace`). Removing or demoting the last owner is 409 `owner_required`. Publish `workspace.created` and `member.changed` so services seed defaults and the gateway drops cached memberships.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_workspaces` | `GET /workspaces` | owner, admin, marketer, sales, read-only | — → 200 `WorkspacePage` | Workspaces I am a member of. Paged (`page`, `limit` ≤ 100). |
| `create_workspace` | `POST /workspaces` | owner | `WorkspaceRequest` → 201 `Workspace` | Create a workspace (a brand). Audited. Publishes `workspace.created` on `crm.workspace.v1`. |
| `get_workspace` | `GET /workspaces/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Workspace` | A workspace. 404 when the record does not exist in this workspace. |
| `update_workspace` | `PATCH /workspaces/{id}` | owner, admin | `WorkspaceRequest` → 200 `Workspace` | Change a workspace. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `list_members` | `GET /workspaces/{id}/members` | owner, admin, marketer, sales, read-only | — → 200 `MembershipPage` | Members and their roles. Paged (`page`, `limit` ≤ 100). 404 when the record does not exist in this workspace. |
| `add_member` | `POST /workspaces/{id}/members` | owner, admin | `AddMemberRequest` → 201 `Membership` | Add or invite a member. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `member.changed` on `crm.workspace.v1`. |
| `update_member` | `PATCH /workspaces/{id}/members/{user_id}` | owner, admin | `UpdateMemberRequest` → 200 `Membership` | 409 when it would leave the workspace without an owner. Publishes `member.changed` on `crm.workspace.v1`. |
| `remove_member` | `DELETE /workspaces/{id}/members/{user_id}` | owner, admin | — → 204 — | 409 when it would leave the workspace without an owner. Publishes `member.changed` on `crm.workspace.v1`. |
| `list_team` | `GET /team` | owner, admin, marketer, sales, read-only | — → 200 `TeamMemberPage` | Members of this workspace, for the CRM screens. Paged (`page`, `limit` ≤ 100). |

## Steps

1. Services: `microservices/crm/workspace/impl/src/services/workspaces.rs` (add `pub mod workspaces;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/workspaces.rs`.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/workspace/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `member.changed`, `workspace.created` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
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
cd microservices && cargo clippy -p rerp_crm_workspace --all-targets -- -D warnings && cargo test -p rerp_crm_workspace
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
