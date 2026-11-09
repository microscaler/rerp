# crm/edge — the public surface

The public surface (ADR 004). Every unauthenticated request lands here: form submissions and double opt-in confirmations, tracking, open pixels and click redirects, unsubscribe and preferences, provider and ad-platform webhooks, OAuth callbacks and calendar feeds. The edge holds no database credentials: it validates shape, origin and rate, resolves the workspace from what is targeted, and publishes to Pub/Sub; the owning service pulls and re-validates.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (19 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `https://t.<workspace domain>/` (public, no gateway) |
| **Generated crate** | `microservices/crm/edge/gen` (`rerp_crm_edge_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 2 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | the public paths of outreach, marketing, tasks and platform |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

The only CRM component reachable without a session. It holds **no database credentials**: it validates, rate-limits,
signs or verifies tokens, and publishes to Pub/Sub. The owning service pulls and re-validates (ADR 003's pattern).

| Public operation | Today | Becomes a message for |
|---|---|---|
| `POST /forms/{id}/submit` | ❌ gap | intake (quarantined until confirmed or validated), then audience and acquisition |
| `GET /confirm/{token}` (double opt-in) | outreach `/public/confirm` ✅ | intake, which admits the contact |
| `POST /track` (page view, custom event) | marketing `POST /analytics/track` | acquisition |
| `POST /ads/clicks` (landing with click ids) | marketing ✅ | acquisition; edge sets the first-party cookie |
| `POST /ads/conversions` (server-side events) | marketing ✅ | acquisition |
| `POST /ads/calls` (call tracking) | marketing ✅ | acquisition |
| `GET /o/{token}` open pixel · `GET /c/{token}` click redirect · `GET /view/{token}` | outreach `/public/*` ✅ | messaging |
| `GET` / `POST /u/{token}` one-click unsubscribe · `GET` / `POST /p/{token}` preferences | outreach `/public/*` ✅ | audience |
| `POST /webhooks/provider/{provider}` (bounces, complaints, inbound SMS STOP) | outreach ✅ | messaging, audience |
| `GET /calendars/{token}.ics` | tasks ✅ | served from a signed snapshot sales publishes |
| `POST /surveys/submit`, `POST /events/register` | marketing ✅ | acquisition (parked features) |
| OAuth callbacks (mailboxes, ad accounts, social) | platform `POST /mailboxes/oauth/callback` ✅, others ❌ | the service that started the flow (state is signed) |
| `POST /webhooks/ads/{platform}` (lead forms and other ad-platform pushes) | ❌ new | acquisition (lead forms go through intake like any other form) |

## API

| Area | Operations |
|---|---|
| `ad-tracking` — Ad tracking: first-party click capture, link decoration, server-side conversion relay (Meta CAPI, Google offline / enhanced, LinkedIn, TikTok), call touchpoints | 2 |
| `calendars` — iCal feeds of tasks | 1 |
| `events` — Event registration tracking | 1 |
| `forms` — Public web form capture | 2 |
| `oauth` — Oauth | 1 |
| `provider-webhooks` — Inbound delivery events from the email/SMS provider | 1 |
| `public` — Unauthenticated endpoints a recipient's mail client or browser touches | 8 |
| `surveys` — Survey response tracking | 1 |
| `visitors` — Anonymous visitor tracking | 1 |
| `webhooks` — Webhooks | 1 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#edge). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.edge.v1`:

- `form_submission.received` — A public form was submitted.
- `survey.received` — A survey response was submitted.
- `event_registration.received` — Someone registered for an event.
- `confirmation.received` — A double opt-in link was clicked.
- `open.received` — Open pixel fetched.
- `click.received` — Tracked link followed (the edge redirects at once).
- `unsubscribe.received` — One-click or page unsubscribe.
- `preferences.received` — Preference page saved.
- `track.received` — A page view from the site script.
- `ad_click.received` — Ad click ids found on landing.
- `ad_call.received` — A call-tracking webhook.
- `provider_event.received` — Delivery events from the email or SMS provider.
- `ad_webhook.received` — A push from an ad platform; lead-form leads go to intake.
- `oauth.completed` — An OAuth redirect came back; the service named in the state exchanges the code.

Consumes: Consumes nothing.

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- None: it resolves click redirects and preference pages from the signed token alone, and publishes everything else.
