# crm/audience — who we can reach

Implementation of `openapi/crm/audience/openapi.yaml`. Design and purpose: [openapi/crm/audience/README.md](../../../openapi/crm/audience/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_audience_gen`, generated, never edited) · `impl/` (`rerp_crm_audience`, binary `audience`) |
| Contract | [openapi.yaml](../../../openapi/crm/audience/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/audience/asyncapi.yaml) |
| Tables | [data-model/audience.yaml](../../../openapi/crm/docs/data-model/audience.yaml) — database `crm`, schema `audience`; migrations in `../migrations/audience/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-AUD-00](../../../docs/crm/stories/CRM-AUD-00-audience-data-model.md) | LifeModels and migrations (15 tables) |
| [CRM-AUD-01](../../../docs/crm/stories/CRM-AUD-01-audience-contacts.md) | Contacts, organisations and notes (23 ops) |
| [CRM-AUD-02](../../../docs/crm/stories/CRM-AUD-02-audience-lists.md) | Lists, subscriptions, consent and suppressions (13 ops) |
| [CRM-AUD-03](../../../docs/crm/stories/CRM-AUD-03-audience-segments.md) | Tags, segments and bulk jobs (16 ops) |
| [CRM-AUD-EV](../../../docs/crm/stories/CRM-AUD-EV-audience-consumers.md) | Event consumers (28 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_audience --bin audience -- --spec ../openapi/crm/audience/openapi.yaml --config crm/audience/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service audience` (gen only; never touches impl).
