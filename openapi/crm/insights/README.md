# crm/insights — what it adds up to

What it adds up to: report read models (deals, contacts, email, automations, revenue, analytics), dashboards and prompt-built widgets, and rule-based named scores with their explanations.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (56 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/insights`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/insights/gen` (`rerp_crm_insights_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 5 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | reporting, and scoring from intelligence |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Read models for every report (deals, contacts, email trends, automations, revenue, analytics hub), dashboards and widgets with their saved queries, the metric catalogue, rule-based named scores with their rules, distributions and explanations (replacing the single implicit score in `crm/intelligence`). |
| **Consumes** | Events from every service; never reads another service's tables. |
| **Publishes** | `score.changed` (with band), which audience projects for segments and automation triggers on. |
| **Runtime** | Event consumers and scheduled jobs; reports p95 ≤ 800 ms over 12 months; read models at most 5 minutes behind. Its own database, so a heavy report never touches an OLTP pool. |

## API

| Area | Operations |
|---|---|
| `analytics` — Analytics | 2 |
| `automation-reports` — Automation reports: all automations, per-automation overview / goals / contacts, jump-to tracking, automated 1:1 emails | 6 |
| `contact-reports` — Contact reports: performance (active / inactive, trends, top lists and tags), field breakdown, nearby contacts, tag trends | 4 |
| `conversion` — Lead-to-revenue conversion analytics | 3 |
| `dashboards` — Per-user dashboards of widgets (a saved metric query each), with per-widget scheduled email | 10 |
| `deal-reports` — Deal reports: overview, owner overview, funnel, sales performance, forecast (crm/pipeline deals) | 5 |
| `email-reports` — Email engagement reports across campaigns and automations: client trends, open/read trends by weekday and hour, predictive-content variants | 3 |
| `forecast` — Revenue forecasting | 3 |
| `performance` — Rep and team performance metrics | 3 |
| `pipeline` — Pipeline health and revenue summary | 2 |
| `reports` — Custom report builder and scheduling | 4 |
| `revenue-reports` — Marketing revenue attributed to campaigns and automation emails from conversions (crm/marketing) within an attribution window | 1 |
| `scores` — Scores | 7 |
| `task-reports` — Task overview: load, completion and outcome sentiment per rep (crm/tasks) | 1 |
| `win-loss` — Win rate and loss analysis | 2 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#insights). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.insights.v1`:

- `score.changed` — A contact or deal score changed band or value.
- `erasure.completed` — A service erased its copy for a data request.

Consumes: `ad.clicked`, `ad_audience.synced`, `ad_buy.changed`, `campaign.cancelled`, `campaign.scheduled`, `campaign.sending`, `campaign.sent`, `contact.archived`, `contact.created`, `contact.erased`, `contact.merged`, `contact.updated`, `conversion.recorded`, `deal.created`, `deal.lost`, `deal.reopened`, `deal.stage_changed`, `deal.updated`, `deal.won`, `enrolment.completed`, `enrolment.failed`, `enrolment.goal_reached`, `enrolment.started`, `enrolment.step_completed`, `enrolment.stopped`, `erasure.requested`, `event.recorded`, `field.changed`, `field.defined`, `form.submitted`, `lead.converted`, `lead.created`, `lead.stage_changed`, `member.changed`, `message.bounced`, `message.clicked`, `message.complained`, `message.delivered`, `message.opened`, `message.replied`, `message.sent`, `organisation.created`, `organisation.updated`, `site.visited`, `social.post.failed`, `social.post.published`, `subscription.changed`, `tag.added`, `tag.removed`, `task.completed`, `task.created`, `task.reopened`, `workflow.activated`, `workflow.paused`, `workspace.created`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- None: it reads only its own read models, built from events.
