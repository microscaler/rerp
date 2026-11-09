# crm/insights — what it adds up to

Implementation of `openapi/crm/insights/openapi.yaml`. Design and purpose: [openapi/crm/insights/README.md](../../../openapi/crm/insights/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_insights_gen`, generated, never edited) · `impl/` (`rerp_crm_insights`, binary `insights`) |
| Contract | [openapi.yaml](../../../openapi/crm/insights/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/insights/asyncapi.yaml) |
| Tables | [data-model/insights.yaml](../../../openapi/crm/docs/data-model/insights.yaml) — database `crm_insights`, schema `insights`; migrations in `../migrations/insights/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-INS-00](../../../docs/crm/stories/CRM-INS-00-insights-data-model.md) | LifeModels and migrations (16 tables) |
| [CRM-INS-01](../../../docs/crm/stories/CRM-INS-01-insights-read-models.md) | Read models and deal, pipeline and forecast reports (20 ops) |
| [CRM-INS-02](../../../docs/crm/stories/CRM-INS-02-insights-marketing-reports.md) | Contact, email, automation, task and revenue reports (19 ops) |
| [CRM-INS-03](../../../docs/crm/stories/CRM-INS-03-insights-dashboards.md) | Dashboards and widgets (10 ops) |
| [CRM-INS-04](../../../docs/crm/stories/CRM-INS-04-insights-scores.md) | Named scores (7 ops) |
| [CRM-INS-EV](../../../docs/crm/stories/CRM-INS-EV-insights-consumers.md) | Event consumers (55 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_insights --bin insights -- --spec ../openapi/crm/insights/openapi.yaml --config crm/insights/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service insights` (gen only; never touches impl).
