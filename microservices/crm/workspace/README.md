# crm/workspace — how the workspace is run

Implementation of `openapi/crm/workspace/openapi.yaml`. Design and purpose: [openapi/crm/workspace/README.md](../../../openapi/crm/workspace/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_workspace_gen`, generated, never edited) · `impl/` (`rerp_crm_workspace`, binary `workspace`) |
| Contract | [openapi.yaml](../../../openapi/crm/workspace/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/workspace/asyncapi.yaml) |
| Tables | [data-model/workspace.yaml](../../../openapi/crm/docs/data-model/workspace.yaml) — database `crm`, schema `workspace`; migrations in `../migrations/workspace/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-WSP-00](../../../docs/crm/stories/CRM-WSP-00-workspace-data-model.md) | LifeModels and migrations (17 tables) |
| [CRM-WSP-01](../../../docs/crm/stories/CRM-WSP-01-workspace-workspaces.md) | Workspaces, memberships and the team facade (9 ops) |
| [CRM-WSP-02](../../../docs/crm/stories/CRM-WSP-02-workspace-fields.md) | Custom fields and field groups (9 ops) |
| [CRM-WSP-03](../../../docs/crm/stories/CRM-WSP-03-workspace-settings.md) | Compliance, brand kit and notification rules (7 ops) |
| [CRM-WSP-04](../../../docs/crm/stories/CRM-WSP-04-workspace-access.md) | API keys, webhooks and integrations (14 ops) |
| [CRM-WSP-05](../../../docs/crm/stories/CRM-WSP-05-workspace-jobs.md) | Import and export jobs, data requests and the audit log (10 ops) |
| [CRM-WSP-EV](../../../docs/crm/stories/CRM-WSP-EV-workspace-consumers.md) | Event consumers (1 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_workspace --bin workspace -- --spec ../openapi/crm/workspace/openapi.yaml --config crm/workspace/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service workspace` (gen only; never touches impl).
