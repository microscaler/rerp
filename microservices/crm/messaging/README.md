# crm/messaging — what we send

Implementation of `openapi/crm/messaging/openapi.yaml`. Design and purpose: [openapi/crm/messaging/README.md](../../../openapi/crm/messaging/README.md).

| | |
|---|---|
| Crates | `gen/` (`rerp_crm_messaging_gen`, generated, never edited) · `impl/` (`rerp_crm_messaging`, binary `messaging`) |
| Contract | [openapi.yaml](../../../openapi/crm/messaging/openapi.yaml) · [asyncapi.yaml](../../../openapi/crm/messaging/asyncapi.yaml) |
| Tables | [data-model/messaging.yaml](../../../openapi/crm/docs/data-model/messaging.yaml) — database `crm`, schema `messaging`; migrations in `../migrations/messaging/` |
| Shared code | `../runtime` (`rerp_crm_runtime`) |
| State | Every operation is a stub returning its contract example; real handlers land story by story. |

## Stories

| Story | Title |
|---|---|
| [CRM-MSG-00](../../../docs/crm/stories/CRM-MSG-00-messaging-data-model.md) | LifeModels and migrations (20 tables) |
| [CRM-MSG-01](../../../docs/crm/stories/CRM-MSG-01-messaging-content.md) | Templates, snippets, assets and postal addresses (17 ops) |
| [CRM-MSG-02](../../../docs/crm/stories/CRM-MSG-02-messaging-sending-setup.md) | Senders, SMS senders, tracking domains, list exclusions and providers (13 ops) |
| [CRM-MSG-03](../../../docs/crm/stories/CRM-MSG-03-messaging-campaigns.md) | Campaigns, sends and the send worker (15 ops) |
| [CRM-MSG-04](../../../docs/crm/stories/CRM-MSG-04-messaging-reports.md) | Campaign and deliverability reports (6 ops) |
| [CRM-MSG-05](../../../docs/crm/stories/CRM-MSG-05-messaging-mailboxes.md) | Connected mailboxes (1:1 email) (8 ops) |
| [CRM-MSG-06](../../../docs/crm/stories/CRM-MSG-06-messaging-social.md) | Social accounts, posts, approval and engagement (19 ops) |
| [CRM-MSG-EV](../../../docs/crm/stories/CRM-MSG-EV-messaging-consumers.md) | Event consumers (12 events) |

## Run locally

```bash
cd microservices
cargo run -p rerp_crm_messaging --bin messaging -- --spec ../openapi/crm/messaging/openapi.yaml --config crm/messaging/impl/config/config.yaml
```

Regenerate after a contract change: `tooling/.venv/bin/rerp gen suite crm --service messaging` (gen only; never touches impl).
