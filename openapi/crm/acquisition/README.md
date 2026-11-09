# crm/acquisition — where contacts come from and what they are worth

Where contacts come from and what they are worth: forms, tracked sites and events, UTM, conversions and business goals, surveys and events, ad accounts and adapters, ad audiences, creatives and buys, ad tracking settings and link decoration, the conversion relay and attribution.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (96 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/acquisition`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/acquisition/gen` (`rerp_crm_acquisition_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 5 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | marketing (public paths to the edge) |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Forms (definition, embed, submissions), tracked sites and event types, the visitor / click store, conversions and business goals, ad accounts and adapters, ad audiences and sync runs, creatives and buys, tracking settings and link-decoration rules, conversion relay rules and the relay log, attribution models, touchpoints, LTV cohorts. |
| **Publishes** | `form.submitted`, `site.visited`, `event.recorded`, `ad.clicked`, `conversion.recorded`, `ad_audience.synced`, `ad_buy.changed`. |
| **Runtime** | API plus adapter workers (platform rate limits, retries, secrets in the secrets service). An ad-platform outage stays inside this service. |

## API

| Area | Operations |
|---|---|
| `ad-attribution` — Ad attribution: journeys, models, spend vs attributed revenue, LTV cohorts | 9 |
| `ad-audiences` — Ad audiences: connected ad accounts and segments / lists / tags kept in step with platform custom audiences (hashed server-side), warm-up sequences, lookalikes | 18 |
| `ad-buying` — Ad buying: one platform-agnostic buy spec pushed through adapters (walled-garden APIs, self-serve DSP APIs, bidder-as-a-service, own OpenRTB bidder), creatives, delivery, recommendations applied | 14 |
| `ad-tracking` — Ad tracking: first-party click capture, link decoration, server-side conversion relay (Meta CAPI, Google offline / enhanced, LinkedIn, TikTok), call touchpoints | 17 |
| `campaigns` — Campaign reporting and attribution | 2 |
| `conversions` — Conversions | 4 |
| `events` — Event registration tracking | 1 |
| `goals` — Goals | 3 |
| `marketing-analytics` — Visitor tracking and analytics | 7 |
| `marketing-campaigns` — Campaign lead and revenue attribution | 1 |
| `marketing-forms` — Marketing form management | 8 |
| `utm` — Campaign, medium, and source tracking | 9 |
| `visitors` — Anonymous visitor tracking | 3 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#acquisition). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.acquisition.v1`:

- `form.submitted` — An admitted submission was recorded against its form, visitor and ad click.
- `site.visited` — An identified visitor viewed a tracked page (from edge track.received).
- `event.recorded` — A custom event was recorded for a visitor or contact.
- `ad.clicked` — A landing with ad click ids was recorded (from edge ad_click.received).
- `conversion.recorded` — A conversion definition fired; the relay sends it to the ad platforms.
- `ad_audience.synced` — An ad audience sync run finished.
- `ad_buy.changed` — An ad buy was created, changed, paused or resumed.
- `erasure.completed` — A service erased its copy for a data request.

Consumes: `ad_call.received`, `ad_click.received`, `ad_webhook.received`, `campaign.scheduled`, `contact.created`, `contact.erased`, `contact.merged`, `deal.won`, `erasure.requested`, `field.changed`, `field.defined`, `intake.admitted`, `message.clicked`, `oauth.completed`, `subscription.changed`, `suppression.added`, `suppression.removed`, `track.received`, `workspace.created`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- Calls audience to resolve ad-audience sources.
- Answers automation (ad audience membership).
