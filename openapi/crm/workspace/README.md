# crm/workspace — how the workspace is run

How a workspace is run: workspaces (one per brand) and memberships with a role per workspace, custom field definitions and groups, API keys, outgoing webhooks, notification rules, compliance settings, data requests (access and erasure), brand kit, integrations, import / export jobs and the audit log.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (49 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/workspace`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/workspace/gen` (`rerp_crm_workspace_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 1 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | platform (minus mailboxes and providers) |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Workspaces (one per brand) and memberships with a role per workspace, custom field definitions and groups (values live with the owning entity), API keys, outgoing webhooks and delivery logs, notification rules, compliance settings, data requests (access and erasure) and their orchestration, brand kit, import / export jobs, the audit log, a read facade over team members and roles. |
| **Does not own** | Users and roles (identity plane). |
| **Publishes** | `field.defined / changed`, `erasure.requested` (each service erases and replies `erasure.completed`), `webhook.delivery.failed`. |
| **Runtime** | API plus a job runner (imports, exports, webhook delivery, erasure saga). |

## API

| Area | Operations |
|---|---|
| `api-keys` — API key management and authentication | 3 |
| `audit-log` — Immutable audit trail of all operations | 2 |
| `brand` — Brand | 2 |
| `compliance` — Compliance | 2 |
| `custom-fields` — Custom field definitions per entity | 9 |
| `data-requests` — Data requests | 3 |
| `integrations` — Third-party integration configuration | 4 |
| `jobs` — Import / export job status: progress, row counts, per-row errors, result download | 5 |
| `notifications` — Notifications | 3 |
| `team` — Team | 1 |
| `webhooks` — Webhook delivery for entity events | 7 |
| `workspaces` — Workspaces | 8 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#workspace). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.workspace.v1`:

- `field.defined` — A custom field was defined.
- `field.changed` — A custom field definition changed or was removed.
- `erasure.requested` — An approved erasure; every service erases and confirms.
- `workspace.created` — A workspace (brand) was created; services seed their defaults.
- `member.changed` — A member was added, removed or given another role (the gateway also drops its cached memberships).

Consumes: `erasure.completed`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- Calls audience for export and import pages.
- Every service reads field definitions from it (cached, refreshed on `field.defined` / `field.changed`).
