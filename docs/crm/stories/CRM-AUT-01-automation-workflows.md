# CRM-AUT-01: crm/automation — Workflows, versions, steps and triggers

**Service:** crm/automation · **Areas:** `workflows`, `workflow-steps`, `triggers` (27 operations) · **Build step:** 3 · **Depends on:** CRM-AUT-00, CRM-RT-01, CRM-RT-02, CRM-RT-03 · **Serves (PriceWhisperer Epic 55):** US_55_02, US_55_03, US_55_12, US_55_13, US_55_14 · **Points:** 11

## Goal

Replace the generated stubs for these operations with real implementations backed by the service's tables, so the
portal screens of US_55_02, US_55_03, US_55_12, US_55_13, US_55_14 run against crm/automation instead of the mock, with workspace isolation, roles, audit and events.

## Read first (only these)

- `openapi/crm/automation/openapi.yaml` — the operations below (request and response schemas, examples, descriptions).
- `openapi/crm/automation/README.md` and `openapi/crm/docs/09-api-conventions.md` (errors §6, idempotency §7).
- `openapi/crm/docs/data-model/automation.yaml` — tables: `workflows`, `workflow_versions`, `workflow_steps`, `workflow_step_notes`, `workflow_triggers`.
- `openapi/crm/docs/10-events.md` — the events published here: `workflow.activated`, `workflow.paused`.
- PriceWhisperer `docs/Epics/EPIC_55/US_55_02_*.md` — what the screen expects (Marketing overview).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_03_*.md` — what the screen expects (Contacts list, filters and bulk actions).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_12_*.md` — what the screen expects (Automations and the automation builder).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_13_*.md` — what the screen expects (Automation map, enrolments and report).
- PriceWhisperer `docs/Epics/EPIC_55/US_55_14_*.md` — what the screen expects (Forms and the form builder).

## Rules for this area

`PUT /workflows/{id}/steps` validates the graph and returns `StepGraphValidation` on 400; activating publishes a new version. Recipes come from `openapi/crm/automation/recipes/`; their behaviour is specified in `openapi/crm/automation/features/*.feature`.

## Operations

| operationId | Method and path | Roles | Request → response | Description (business rules) |
|---|---|---|---|---|
| `get_automation_catalog` | `GET /catalog` | owner, admin, marketer, sales, read-only | — → 200 `AutomationCatalog` | Triggers and actions. |
| `list_workflow_recipes` | `GET /recipes` | owner, admin, marketer, sales, read-only | — → 200 inline object | Starter workflows (welcome series, abandoned signup, re-engagement, post-purchase). |
| `import_workflow_recipe` | `POST /recipes/import` | owner, admin, marketer | inline object → 201 `WorkflowRecipe` | Add a recipe to the workspace's library from a URL or an uploaded definition. Audited. |
| `instantiate_workflow_recipe` | `POST /recipes/{id}/instantiate` | owner, admin, marketer | inline object → 201 `Workflow` | Create an inactive workflow from a recipe. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `list_workflows` | `GET /workflows` | owner, admin, marketer, sales, read-only | — → 200 `WorkflowPage` | List all workflows |
| `create_workflow` | `POST /workflows` | owner, admin, marketer | `CreateWorkflowRequest` → 201 `Workflow` | Create a new workflows |
| `get_workflow` | `GET /workflows/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Workflow` | Get workflows by ID |
| `update_workflow` | `PUT /workflows/{id}` | owner, admin, marketer | `UpdateWorkflowRequest` → 200 `Workflow` | Update workflows by ID |
| `delete_workflow` | `DELETE /workflows/{id}` | owner, admin, marketer | — → 204 — | Delete workflows by ID |
| `patch_workflow` | `PATCH /workflows/{id}` | owner, admin, marketer | `UpdateWorkflowRequest` → 200 `Workflow` | Steps are replaced with PUT /workflows/{id}/steps. |
| `activate_workflow` | `POST /workflows/{id}/activate` | owner, admin, marketer | — → 200 `Workflow` | Set the workflow live. A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `workflow.activated` on `crm.automation.v1`. |
| `bind_workflow_parameters` | `PATCH /workflows/{id}/bindings` | owner, admin, marketer | inline object → 200 `Workflow` | Bind recipe parameters; references in triggers and steps are resolved in place. Changes only the fields sent; the change is audited with before and after values. 404 when the record does not exist in this workspace. |
| `duplicate_workflow` | `POST /workflows/{id}/duplicate` | owner, admin, marketer | — → 201 `Workflow` | Copy a workflow and its steps as a new inactive workflow. A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `export_workflow_recipe` | `GET /workflows/{id}/export` | owner, admin, marketer, sales, read-only | — → 200 inline object | The workflow as a recipe document - references replaced by parameters, notes kept. 404 when the record does not exist in this workspace. |
| `pause_workflow` | `POST /workflows/{id}/pause` | owner, admin, marketer | — → 200 `Workflow` | Pause the workflow (enrolments hold at their step). A state change on one resource; audited. 404 when the record does not exist in this workspace. Publishes `workflow.paused` on `crm.automation.v1`. |
| `run_workflow` | `POST /workflows/{id}/run` | owner, admin, marketer | — → 200 `WorkflowExecutionResult` | Run a workflow manually |
| `list_workflow_run_history` | `GET /workflows/{id}/run-history` | owner, admin, marketer, sales, read-only | — → 200 `WorkflowRunPage` | Get workflow execution history |
| `get_workflow_setup` | `GET /workflows/{id}/setup` | owner, admin, marketer, sales, read-only | — → 200 inline object | The setup wizard - the unbound recipe parameters, in order. 404 when the record does not exist in this workspace. |
| `add_step_note` | `POST /workflows/{id}/steps/{step_id}/notes` | owner, admin, marketer | inline object → 201 `StepNote` | Add a note to a step (or to the workflow with step_id "workflow"). A state change on one resource; audited. 404 when the record does not exist in this workspace. |
| `get_workflow_steps` | `GET /workflows/{id}/steps` | owner, admin, marketer, sales, read-only | — → 200 `WorkflowSteps` | The workflow's step graph. 404 when the record does not exist in this workspace. |
| `replace_workflow_steps` | `PUT /workflows/{id}/steps` | owner, admin, marketer | `WorkflowSteps` → 200 `WorkflowSteps` | The whole graph is replaced atomically and validated (every branch reaches an END or a GOAL, no orphan steps, every SEND_MESSAGE names an AUTOMATION template). Contacts mid-way keep their current step by id; a removed step moves them to its successor.  |
| `list_triggers` | `GET /triggers` | owner, admin, marketer, sales, read-only | — → 200 `TriggerPage` | List all triggers |
| `create_trigger` | `POST /triggers` | owner, admin, marketer | `CreateTriggerRequest` → 201 `Trigger` | Create a new triggers |
| `get_trigger` | `GET /triggers/{id}` | owner, admin, marketer, sales, read-only | — → 200 `Trigger` | Get triggers by ID |
| `update_trigger` | `PUT /triggers/{id}` | owner, admin, marketer | `UpdateTriggerRequest` → 200 `Trigger` | Update triggers by ID |
| `delete_trigger` | `DELETE /triggers/{id}` | owner, admin, marketer | — → 204 — | Delete triggers by ID |
| `fire_trigger` | `POST /triggers/{id}/fire` | owner, admin, marketer | — → 200 inline object | Fire a trigger manually |

## Steps

1. Services: `microservices/crm/automation/impl/src/services/workflows.rs` (add `pub mod workflows;` to
   `services/mod.rs`) with one function per operation. Each takes the `RequestContext` (rerp_crm_runtime::context)
   and plain inputs, runs in one transaction opened by the context (so row-level security applies), writes the audit
   row (rerp_crm_runtime::audit) and appends events to the outbox (rerp_crm_runtime::events) in that same
   transaction, and returns a domain result or an `ApiError` (rerp_crm_runtime::errors).
2. Controllers: in each `impl/src/controllers/<operationId>.rs` put `// BRRTRouter: user-owned` on the first line, build
   the context from the request, call the service, and map the result to the generated `Response` (decimal strings
   for money; computed API-only fields per the data-model YAML). No business logic in controllers.
3. Validators: rules the contract cannot express (uniqueness messages, state transitions) in
   `impl/src/validators/workflows.rs`.
4. Idempotency: `import_workflow_recipe`, `instantiate_workflow_recipe`, `activate_workflow`, `duplicate_workflow`, `run_workflow`, `fire_trigger` take `Idempotency-Key`; wrap them with rerp_crm_runtime::idempotency.

## Acceptance criteria

- AC-1 **Contract.** Every operation above returns a status and body that validate against `openapi/crm/automation/openapi.yaml`
  (the contract test replays each operation's request example and validates the response).
- AC-2 **Isolation.** A record of workspace A is never visible from workspace B: list empty, get / update / delete 404.
  A request with a workspace the caller is not a member of is 403 `workspace_forbidden`.
- AC-3 **Roles.** Each operation is allowed exactly for the roles in its `x-required-role` (11-permissions.md); others get
  403 `role_forbidden`.
- AC-4 **Audit.** Every write records one audit row with the changed fields before and after.
- AC-5 **Events.** `workflow.activated`, `workflow.paused` are published once per change, after commit, with the envelope of 10-events.md; nothing is published when the transaction rolls back.
- AC-6 **Idempotency.** Repeating `import_workflow_recipe`, `instantiate_workflow_recipe`, `activate_workflow`, `duplicate_workflow`, `run_workflow`, `fire_trigger` with the same key returns the first response and does nothing twice; the same key with a different body is 409 `idempotency_mismatch`.
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
cd microservices && cargo clippy -p rerp_crm_automation --all-targets -- -D warnings && cargo test -p rerp_crm_automation
```

## Do not

- Do not edit `gen/` or `openapi.yaml`. If the contract is wrong, stop and report it.
- Do not read or write another service's schema; ask that service through its API or learn through events.
- Do not log personal data (emails, names, IPs) above debug level.
