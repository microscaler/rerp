# CRM-INS-04: crm/insights — Named scores

**Service:** crm/insights · **Areas:** `scores` (7 operations) · **Build step:** 5 · **Depends on:** CRM-INS-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_23, US_55_29, US_55_32 · **Points:** 4

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_23, US_55_29, US_55_32 run against crm/insights instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/insights/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/insights/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/insights.yaml` — tables: `scores`, `score_rules`, `entity_scores`, `score_hits`.
- `openapi/crm/docs/10-events.md` — the events published here: `score.changed`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_23_*.md` — what the screen expects (Lead scoring).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_29_*.md` — what the screen expects (Settings — custom fields, tags and bulk edit).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_32_*.md` — what the screen expects (Ad audiences and warm-up).

## Rules for this area

Recalculation is a background job; a band change publishes `score.changed`. Explain lists the rule hits.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `list_scores` | `GET /scores` | owner, admin, marketer, sales, read-only | — → 200 `ScorePage` | Named scores. Paged (`page`, `limit` ≤ 100). Filters: `entity`. |
| `create_score` | `POST /scores` | owner, admin, marketer | `ScoreRequest` → 201 `Score` | Create a score. Audited. |
| `get_score` | `GET /scores/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Score` | A score with its rules and distribution. 404 when the record does not exist in this workspace. |
| `update_score` | `PATCH /scores/{id}` | owner, admin, marketer | `ScoreRequest` → 200 `Score` | Change a score's thresholds or rules. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `activate_score` | `POST /scores/{id}/activate` | owner, admin, marketer | — → 200 `Score` | Activation queues a full recalculation. |
| `explain_score` | `GET /scores/{id}/explain` | owner, admin, marketer, sales, read-only | — → 200 `ScoreExplanation` | Why a contact or deal has its score. Query: `contact_id`, `deal_id`. 404 when the record does not exist in this workspace. |
| `recalculate_score` | `POST /scores/{id}/recalculate` | owner, admin, marketer | — → 202 `ScoreRecalculation` | Recompute the score for every contact or deal. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `score.changed` on `crm.insights.v1`. |

## Steps

1. Services: `microservices/crm/insights/impl/src/services/scores.rs` (add `pub mod scores;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/scores.rs`.
4. Idempotency: `activate_score`, `recalculate_score` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/insights/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `score.changed` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `activate_score`, `recalculate_score` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_insights --all-targets -- -D warnings && cargo test -p rerp_crm_insights
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
