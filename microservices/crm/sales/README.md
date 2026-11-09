# crm/sales — selling

Implementation of `openapi/crm/sales/openapi.yaml`. Design and purpose: [openapi/crm/sales/README.md](../../../openapi/crm/sales/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_sales_gen`, generated, never edited) · `impl/` (`rerp_crm_sales`, binary `sales`) |
| Contract | [openapi.yaml](../../../openapi/crm/sales/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/sales/asyncapi.yaml) |
| Tables | [data-model/sales.yaml](../../../openapi/crm/docs/data-model/sales.yaml) — database `crm`, schema `sales`; migrations in `../migrations/sales/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-SAL-00](../../../docs/crm/stories/CRM-SAL-00-sales-data-model.md) | LifeModels and migrations (21 tables) |
| [CRM-SAL-01](../../../docs/crm/stories/CRM-SAL-01-sales-pipelines.md) | Pipelines, stages and deal settings (23 ops) |
| [CRM-SAL-02](../../../docs/crm/stories/CRM-SAL-02-sales-deals.md) | Deals, deal contacts, notes, files, emails and status (24 ops) |
| [CRM-SAL-03](../../../docs/crm/stories/CRM-SAL-03-sales-leads.md) | Leads and triage (10 ops) |
| [CRM-SAL-04](../../../docs/crm/stories/CRM-SAL-04-sales-tasks.md) | Tasks, task types, outcomes, calendars and saved responses (43 ops) |
| [CRM-SAL-05](../../../docs/crm/stories/CRM-SAL-05-sales-teams.md) | Teams, members and assignment (10 ops) |
| [CRM-SAL-EV](../../../docs/crm/stories/CRM-SAL-EV-sales-consumers.md) | Event consumers (13 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_sales --bin sales -- --spec ../openapi/crm/sales/openapi.yaml --config crm/sales/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service sales` (gen only; never touches impl).
