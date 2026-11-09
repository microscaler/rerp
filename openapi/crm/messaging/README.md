# crm/messaging — what we send

What we send: versioned templates, snippets, assets and the merge-field catalogue; campaigns (one-time, A/B, RSS, recurring, reminder; email and SMS) with their per-recipient send ledger and engagement; senders and sending domains, SMS senders, tracking domains, postal addresses, list exclusions, providers; connected 1:1 mailboxes; social accounts and posts; campaign and deliverability reports.

| | |
|---|---|
| **Contract** | [openapi.yaml](openapi.yaml) (78 operations), [asyncapi.yaml](asyncapi.yaml) |
| **Base path** | `/api/v1/crm/messaging`, `X-Workspace-Id` on every call |
| **Generated crate** | `microservices/crm/messaging/gen` (`rerp_crm_messaging_gen`) |
| **Implementation** | Not started; the generated crate serves the spec examples. Build step 3 in [13-implementation-plan.md](../docs/13-implementation-plan.md). |
| **Replaces** | outreach (minus lists and consent), social, mailboxes and providers from platform |

## Design

From [08-service-decomposition.md §4](../docs/08-service-decomposition.md):

| | |
|---|---|
| **Owns** | Templates (versioned), snippets, assets, merge-field catalogue, campaigns (one-time, A/B, RSS, recurring, reminder; email and SMS), the per-recipient send ledger and engagement events, senders and sending domains, SMS senders, tracking domains, postal addresses, list exclusions, provider configuration, connected mailboxes (1:1 email), social accounts and posts, campaign and deliverability reports. |
| **Asks audience** | To resolve an audience (lists − exclusions − unsubscribed − suppressed, ∩ segment), paged, at send time. |
| **Publishes** | `campaign.scheduled / sending / sent`, `message.sent / delivered / opened / clicked / bounced / complained / replied`, `social.post.published / failed`, `mailbox.message.synced`. |
| **Runtime** | API plus send workers (≥ 100 k recipients / hour / workspace), throttled per provider; idempotent per recipient. |

## API

| Area | Operations |
|---|---|
| `assets` — Assets | 2 |
| `campaigns` — One-time and A/B campaigns, their schedule and send | 10 |
| `events` — Engagement events (delivered, opened, clicked, bounced, complained, unsubscribed) | 1 |
| `list-exclusions` — List exclusions | 3 |
| `mailboxes` — Connected email accounts (OAuth Google/Microsoft or IMAP/SMTP): two-way sync of 1:1 mail onto contacts and deals, 1:1 sends with tracking | 8 |
| `outreach-reports` — Campaign and list performance | 5 |
| `postal-addresses` — Postal addresses | 3 |
| `providers` — Providers | 1 |
| `reports` — Reports | 1 |
| `senders` — Sender identities and sending domains (DKIM / DMARC state) | 5 |
| `sends` — Per-recipient delivery log | 3 |
| `sms` — Sms | 1 |
| `snippets` — Snippets | 4 |
| `social` — Social | 1 |
| `social-accounts` — Connected platform accounts | 5 |
| `social-calendar` — Calendar view of scheduled and published posts | 1 |
| `social-engagement` — Engagement metrics per post and per account | 2 |
| `social-leads` — Capturing leads from social interactions | 1 |
| `social-posts` — Posts, scheduling and approval | 9 |
| `templates` — Message templates (email, SMS, WhatsApp) with merge fields | 8 |
| `tracking-domains` — Tracking domains | 3 |
| `transactional` — Application-triggered single sends sharing the suppression list | 1 |

Roles per operation: [11-permissions.md](../docs/11-permissions.md#messaging). Conventions: [09-api-conventions.md](../docs/09-api-conventions.md).

## Events

Publishes on `crm.messaging.v1`:

- `campaign.scheduled` — Checks passed and a send time is set.
- `campaign.cancelled` — A scheduled campaign went back to draft.
- `campaign.sending` — The send worker started on the campaign.
- `campaign.sent` — Every recipient was processed.
- `message.sent` — The provider accepted a message (campaign, automation, transactional or 1:1). Test sends are flagged and excluded from reports.
- `message.delivered` — Provider delivery receipt.
- `message.opened` — Open pixel fetched (edge open.received, attributed).
- `message.clicked` — Tracked link followed (edge click.received, attributed).
- `message.bounced` — Soft or hard bounce from the provider; hard bounces make audience suppress the address.
- `message.complained` — Spam complaint; audience unsubscribes and suppresses.
- `message.replied` — A reply arrived in a connected mailbox.
- `social.post.published` — The platform accepted the post.
- `social.post.failed` — Publishing failed after retries.
- `erasure.completed` — A service erased its copy for a data request.

Consumes: `click.received`, `contact.erased`, `contact.merged`, `erasure.requested`, `intake.confirmation_requested`, `oauth.completed`, `open.received`, `provider_event.received`, `subscription.changed`, `suppression.added`, `suppression.removed`, `workspace.created`

Catalogue and transport: [10-events.md](../docs/10-events.md).

## Synchronous calls

- Calls audience to resolve an audience at send time and to check a contact is reachable on a list.
- Answers automation (send a template to one contact).
