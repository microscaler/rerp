# crm/audience — who we can reach

Who we can reach: contacts with custom field values, organisations (prospect companies), tags, segments and the shared condition tree, lists, subscriptions with their consent history, suppressions, notes, the contact timeline and bulk jobs. Owns every "can we contact this person?" fact. Admits confirmed sign-ups from crm/intake (ADR 004).

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (52 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/audience`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/audience/gen` (`rerp_crm_audience_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 1 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | contacts; lists, subscriptions and suppressions from outreach; companies from accounts (as organisations) |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Contacts (with custom field values), organisations (prospect companies), tags, segments and the shared condition tree, lists, subscriptions with their consent history, suppressions, contact notes, the contact timeline projection, bulk jobs on contacts. |
| **Answers** | "Who matches this segment?", "Can we email / text this person on this list?", "What has happened to this person?" |
| **Does not own** | Engagement facts (they arrive as events and are projected), customers (RERP `crm/accounts`), field definitions (`crm/workspace`). |
| **Consumes** | Facts from every other service, projected into a per-contact **facts** table (last open / click, pages visited, events, deal stages, score bands, enrolments) so segment evaluation is one query in one schema. |
| **Publishes** | `contact.created / updated / merged / erased`, `subscription.changed`, `suppression.added / removed`, `tag.added / removed`, `segment.saved`. |
| **Runtime** | Synchronous API; segment evaluation p95 ≤ 700 ms on 100 k contacts. |

## API

| Area | Operations |
|---|---|
| `bulk-jobs` — Bulk jobs | 3 |
| `contacts` — Contact CRUD | 16 |
| `lists` — Mailing lists and their subscription state | 5 |
| `organisations` — Organisations | 7 |
| `segments` — Saved condition trees that select contacts | 6 |
| `subscriptions` — Contact x list subscription state and consent history | 5 |
| `suppressions` — Addresses that must never be sent to | 3 |
| `tags` — Tags on contacts | 7 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#audience). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.audience.v1`:

- `contact.created` — A contact entered the CRM store (staff, import, API, or an admitted submission).
- `contact.updated` — Contact fields changed; data carries the new state.
- `contact.merged` — Duplicates were merged; consumers repoint their references from the merged ids to the survivor.
- `contact.archived` — Contact archived: no longer listed or enrolled; history kept.
- `contact.erased` — Personal data erased after erasure.requested; consumers erase their copies and confirm with erasure.completed.
- `subscription.changed` — A list subscription changed status (also on admission, unsubscribe at the edge, hard bounce, complaint).
- `suppression.added` — An address may not be messaged on any list (also on hard bounce and complaint).
- `suppression.removed` — A suppression was lifted.
- `tag.added` — A tag was put on a contact.
- `tag.removed` — A tag was taken off a contact.
- `organisation.created` — A prospect organisation was created.
- `organisation.updated` — An organisation changed.
- `erasure.completed` — A service erased its copy for a data request.

Consumes: `deal.created`, `deal.lost`, `deal.stage_changed`, `deal.won`, `enrolment.completed`, `enrolment.failed`, `enrolment.goal_reached`, `enrolment.started`, `enrolment.step_completed`, `enrolment.stopped`, `erasure.requested`, `event.recorded`, `field.changed`, `field.defined`, `intake.admitted`, `lead.converted`, `message.bounced`, `message.clicked`, `message.complained`, `message.opened`, `message.sent`, `preferences.received`, `score.changed`, `site.visited`, `task.completed`, `task.created`, `unsubscribe.received`, `workspace.created`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- Answers messaging (resolve an audience), automation (evaluate a condition for one contact), acquisition (resolve an ad-audience source), sales (contact and organisation lookups), workspace (export and import pages).
