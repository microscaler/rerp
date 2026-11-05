# Outreach

Message delivery for the CRM. This is the service that turns the CRM into a marketing platform: lists and
consent, templates, campaigns, the delivery log and engagement events, transactional mail, suppression and
sender identities, plus the public endpoints a recipient's mail client touches.

It replaces the former `openapi/marketing/email` stub.

## What it owns

| Entity | Why it is here |
|---|---|
| `MailingList`, `Subscription`, `ConsentEvent` | Consent is per list and auditable. Every status change keeps its source (form, double opt-in, import, staff, preference page) and is never trimmed. |
| `Template` (versioned) | Email, SMS and WhatsApp bodies with merge fields. Campaigns and automation steps reference templates by id; a send records the version it used. |
| `Campaign` (one-time, A/B, RSS) | A broadcast to the resolved audience of one or more lists minus exclusions, suppressed and unsubscribed, filtered by a segment. |
| `Send`, `MessageEvent` | One row per recipient per message; opens, clicks, bounces, complaints and unsubscribes as events keyed by provider event id. |
| `Suppression` | Hard bounces and complaints are permanent; manual suppressions can be lifted. |
| `Sender` | From identity plus the DNS records (DKIM, SPF, DMARC, return-path) and their verification state. |

## What it does not own

- **Contacts, tags, custom fields, segments** - `crm/contacts` and `crm/platform`. Outreach keys everything by `contact_id`.
- **Sequences and triggers** - `crm/automation`. A workflow's `SEND_MESSAGE` step calls outreach; outreach publishes
  `MESSAGE_OPENED` / `MESSAGE_CLICKED` / `MESSAGE_BOUNCED` events that automation triggers on.
- **Forms, site tracking, UTM attribution** - `crm/marketing`. A form's `SUBSCRIBE_LIST` action calls outreach with
  `consent_source: WEB_FORM` and the form id.
- **The provider.** SES, Postmark, Resend, Twilio and Meta are adapters chosen per deployment; the API never exposes
  provider-specific shape beyond `Provider` on a send and the raw webhook body.

## Data home

Per ADR 003 the marketing store sits outside RERP's tenant database. Outreach owns a Postgres of its own; a deployment
may point it at a managed project or an in-cluster instance. Nothing here crosses into RERP; only converted
customers do, through the `crm-conversions` intake.

## Public surface

`/public/*` is unauthenticated and token-addressed. Tokens are HMACs minted at send time; they name a send (and link
index) without exposing ids. Click redirects resolve their destination server-side, never from the query string.
`/public/u/{token}` implements RFC 8058 one-click unsubscribe. `/webhooks/provider/{provider}` must be reached only
through a keyless relay, as with the `crm-conversions` intake.

## Deferred

SMS and WhatsApp share the model (channel on template and campaign) and are enabled by adding a provider adapter.
