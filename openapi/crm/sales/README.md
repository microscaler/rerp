# crm/sales — selling

Selling: website leads and their triage, pipelines and stages, deals with contacts, roles, notes, files and emails, deal settings, sales teams and assignment, task types, outcomes, settings, tasks, calendar feeds and saved responses, and the signed conversion to a RERP customer (ADR 003).

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (110 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/sales`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/sales/gen` (`rerp_crm_sales_gen`) |
| **Implementation** | Partly implemented as `crm/pipeline` (leads, stages, convert, against Supabase); moves here in step 4 of the plan. Build step 4 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | pipeline, tasks, teams |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Website leads and their triage (the EPIC-32 board), pipelines and stages, deals with contacts and roles, notes, files and emails, deal settings, task types, outcomes and settings, tasks, calendar feeds, saved responses, sales teams and assignment (from `crm/teams`), and the **conversion** to customer. |
| **Publishes** | `lead.created / stage_changed`, `deal.created / stage_changed / won / lost`, `task.created / completed`, and the signed conversion on `crm-conversions` (ADR 003). |
| **Runtime** | Synchronous API. Already partly implemented as `crm/pipeline` (leads, stages, convert). |

## API

| Area | Operations |
|---|---|
| `calendars` — iCal feeds of tasks | 7 |
| `deal-roles` — Roles a contact plays on a deal | 6 |
| `deals` — Deals: contacts and roles, stage and status, notes, files, activity, 1:1 email | 24 |
| `leads` — Lead and opportunity lifecycle | 10 |
| `members` — Team member management | 3 |
| `outcomes` — The outcome catalogue | 6 |
| `pipelines` — Pipelines, their summary and settings | 9 |
| `saved-responses` — Reusable 1:1 email responses | 13 |
| `settings` — Task settings | 2 |
| `stages` — Pipeline stage management | 6 |
| `task-types` — Task types and their outcomes | 7 |
| `tasks` — Tasks on contacts and deals | 10 |
| `teams` — Team CRUD | 7 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#sales). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.sales.v1`:

- `lead.created` — A lead was created (staff, an admitted enquiry, or a social lead).
- `lead.stage_changed` — A lead moved stage.
- `lead.converted` — A lead became an opportunity (and a contact and organisation where needed).
- `deal.created` — A deal was opened.
- `deal.updated` — Deal fields changed.
- `deal.stage_changed` — A deal moved stage.
- `deal.won` — A deal was won. Sales also emits the signed conversion on crm-conversions (ADR 003).
- `deal.lost` — A deal was lost, with its reason.
- `deal.reopened` — A won or lost deal was reopened.
- `task.created` — A task was created.
- `task.completed` — A task was completed with its outcome.
- `task.reopened` — A completed task was reopened.
- `erasure.completed` — A service erased its copy for a data request.

Consumes: `contact.created`, `contact.erased`, `contact.merged`, `contact.updated`, `erasure.requested`, `field.changed`, `field.defined`, `intake.admitted`, `member.changed`, `message.replied`, `organisation.created`, `organisation.updated`, `workspace.created`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- Calls audience for contact and organisation lookups.
- Answers automation (create a deal or task).
