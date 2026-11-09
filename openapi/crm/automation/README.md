# crm/automation — what happens next

What happens next: workflows and their versioned step graphs, the trigger and action catalogue, recipes, enrolments with step history and durable timers.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (47 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/automation`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/automation/gen` (`rerp_crm_automation_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 3 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | automation |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Workflows and their step graphs (with versions), the trigger and action catalogue, recipes, enrolments with step history, durable timers. |
| **Calls** | Audience (evaluate a condition for one contact), messaging (send a template to one contact), sales (create a deal or task), acquisition (add to / remove from an ad audience). |
| **Consumes** | Every trigger event (`subscription.changed`, `tag.added`, `form.submitted`, `site.visited`, `deal.stage_changed`, `ad.clicked`, `conversion.recorded`, ...). |
| **Publishes** | `enrolment.started / step_completed / goal_reached / completed / stopped / failed`. |
| **Runtime** | API plus a step engine: exactly-once per enrolment step, timers released within 60 s of due. |

## API

| Area | Operations |
|---|---|
| `actions` — Scheduled and manual actions | 3 |
| `automation-execution` — Workflow execution and run history | 1 |
| `enrolments` — Contacts moving through workflows | 8 |
| `rules` — Condition-action rules within workflows | 8 |
| `triggers` — Event hooks that fire workflows | 6 |
| `workflow-steps` — The step graph of a workflow (wait, if/else, split, goal) | 2 |
| `workflows` — Workflow definitions and management | 19 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#automation). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.automation.v1`:

- `enrolment.started` — A contact entered a workflow (trigger or manual).
- `enrolment.step_completed` — A step ran; data.current_step_id is the step reached.
- `enrolment.goal_reached` — The workflow goal was met.
- `enrolment.completed` — The contact reached END.
- `enrolment.stopped` — Stopped by a user, an unsubscribe, a conversion or an exit rule.
- `enrolment.failed` — A step failed beyond its retries; POST /enrolments/{id}/retry resumes.
- `workflow.activated` — Triggers are live.
- `workflow.paused` — Triggers stopped; waiting enrolments hold.
- `erasure.completed` — A service erased its copy for a data request.

Consumes: `ad.clicked`, `contact.archived`, `contact.created`, `contact.erased`, `contact.merged`, `contact.updated`, `conversion.recorded`, `deal.created`, `deal.lost`, `deal.reopened`, `deal.stage_changed`, `deal.won`, `erasure.requested`, `event.recorded`, `form.submitted`, `lead.converted`, `lead.created`, `lead.stage_changed`, `message.clicked`, `message.opened`, `message.replied`, `message.sent`, `score.changed`, `site.visited`, `subscription.changed`, `tag.added`, `tag.removed`, `task.completed`, `task.created`, `workspace.created`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- Calls audience (evaluate a condition for one contact), messaging (send to one contact), sales (create a deal or task), acquisition (add to or remove from an ad audience).

## Behaviour specs and recipes

- [features/](features/): Gherkin scenarios for enrolment, the step kinds (action, branching, wait), the workflow
  lifecycle and recipes. They are the acceptance tests for the step engine: each scenario becomes a test when the
  service is implemented (step 3 of [13-implementation-plan.md](../docs/13-implementation-plan.md)).
- [recipes/](recipes/): the workflow recipes `GET /recipes` and `POST /recipes/{id}/instantiate` serve (free-trial
  nurture, expired-trial follow-up, upgrade incentive, re-engagement by tag), with `$list`, `$tag` and template
  parameters bound by `PATCH /workflows/{id}/bindings`.
