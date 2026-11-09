# crm/acquisition — where contacts come from and what they are worth

Implementation of `openapi/crm/acquisition/openapi.yaml`. Design and purpose: [openapi/crm/acquisition/README.md](../../../openapi/crm/acquisition/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_acquisition_gen`, generated, never edited) · `impl/` (`rerp_crm_acquisition`, binary `acquisition`) |
| Contract | [openapi.yaml](../../../openapi/crm/acquisition/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/acquisition/asyncapi.yaml) |
| Tables | [data-model/acquisition.yaml](../../../openapi/crm/docs/data-model/acquisition.yaml) — database `crm`, schema `acquisition`; migrations in `../migrations/acquisition/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-ACQ-00](../../../docs/crm/stories/CRM-ACQ-00-acquisition-data-model.md) | LifeModels and migrations (26 tables) |
| [CRM-ACQ-01](../../../docs/crm/stories/CRM-ACQ-01-acquisition-forms.md) | Forms, submissions, tracked sites and visitors (12 ops) |
| [CRM-ACQ-02](../../../docs/crm/stories/CRM-ACQ-02-acquisition-conversions.md) | Conversions, goals, UTM and marketing analytics (26 ops) |
| [CRM-ACQ-03](../../../docs/crm/stories/CRM-ACQ-03-acquisition-ad-accounts.md) | Ad accounts, audiences and sync (18 ops) |
| [CRM-ACQ-04](../../../docs/crm/stories/CRM-ACQ-04-acquisition-ad-buying.md) | Ad buys and creatives (14 ops) |
| [CRM-ACQ-05](../../../docs/crm/stories/CRM-ACQ-05-acquisition-ad-tracking.md) | Ad tracking, conversion relay and attribution (26 ops) |
| [CRM-ACQ-EV](../../../docs/crm/stories/CRM-ACQ-EV-acquisition-consumers.md) | Event consumers (19 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_acquisition --bin acquisition -- --spec ../openapi/crm/acquisition/openapi.yaml --config crm/acquisition/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service acquisition` (gen only; never touches impl).
