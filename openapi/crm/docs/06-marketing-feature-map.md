# Marketing platform feature map — reverse-engineered from ActiveCampaign

> **Status:** Working document, 2026-11-05. Source: a full walk of an ActiveCampaign trial account
> (every primary section, every secondary panel, the automation trigger and action catalogues, the
> settings tree). Used to curate v1 of RERP's CRM + marketing platform and to keep the Claude Design
> mock honest. Nothing here is vendor-specific in our model; the names are theirs, the shapes are ours.

## 1. Inventory — everything the product exposes

### Contacts
| AC feature | Path | Our service | Status |
|---|---|---|---|
| Contacts list, profile, timeline | `/app/contacts` | `crm/contacts` | in spec |
| Accounts (B2B company grouping of contacts) | `/app/accounts` | `crm/accounts` is the *converted-customer* record (ADR 003); a marketing-side company grouping needs a decision | **decide** |
| Lists | `/app/lists` | `crm/outreach` lists + subscriptions | in spec |
| Segments | `/app/segments` | `crm/contacts` segments, shared `ConditionGroup` | in spec |
| Tags | `/app/tags` | `crm/contacts` tags | in spec |
| Custom fields | `/app/settings/fields/contacts` | `crm/platform` custom fields | in spec |
| Lead scores (rule-based, several named scores, contact and deal) | `/app/scores` | `crm/intelligence` is Bayesian only | **add rule-based scores** |
| List exclusions (block list per list, distinct from suppression) | `/app/list-exclusions` | — | **add** |
| Database sync (external DB → contacts) | `/app/sync-database` | — | defer |
| Import / export with status | `/app/contacts/import-export` | `crm/contacts` import/export jobs | in spec |
| Email subscriptions view (contact × list grid) | `/app/lists/email-subscriptions` | `/contacts/{id}/subscriptions` | in spec |
| Bulk edits | `/app/bulk-edits` | — | **add** (job: segment → add tag / list / field) |
| Archive contact | action | — | **add** |
| Reports: contact performance, field breakdown, tag trends | `/app/reports/...` | `crm/reporting` | partial |
| Reports: nearby contacts (geo) | `/report/#/contacts/nearby` | — | defer |

### Automations
| AC feature | Our service | Status |
|---|---|---|
| Automations list, map, builder | `crm/automation` steps/enrolments/map | in spec |
| **Several start triggers per automation** | one `trigger_type` per workflow | **change: `triggers[]`** |
| Automation messages (every email owned by automations, in one list) | `/templates?kind=AUTOMATION` + usage | **add usage view** |
| Transactional email list | `crm/outreach` transactional | in spec |
| Reports: all automations, jump-to | `crm/reporting` | partial |
| Trigger: on a schedule | `TIME_BASED` | in spec |
| Trigger: subscribes / unsubscribes from list | yes | in spec |
| Trigger: submits a form | yes | in spec |
| Trigger: opens / clicks / bounces | yes | in spec |
| Trigger: **shares / forwards / replies** to an email | `MESSAGE_REPLIED` only | **add FORWARDED, SHARED** (reply needs inbound mail) |
| Trigger: web page visited, event recorded | yes | in spec |
| Trigger: tag added / removed, contact created, field changes | yes | in spec |
| Trigger: **deal field / value / status changes, enters a pipeline, deal stage changes** | `DEAL_CREATED/STAGE_CHANGE/WON/LOST` | **add VALUE, STATUS, FIELD, ENTERED_PIPELINE** |
| Trigger: **account field changes** | — | **add** |
| Trigger: date based, RSS based | `DATE_FIELD` | **add RSS** |
| Trigger: score changes | yes | in spec |
| Trigger: **contact jumped to**, **task completed**, **file downloaded** | — | **add** |
| Trigger: WhatsApp flow completed | — | defer |
| Action: send email | `SEND_MESSAGE` | in spec |
| Action: **send a direct 1:1 email** (from a user's mailbox) | — | **add** (needs connected mailbox; v1 = send via sender identity with user's reply-to) |
| Action: send a transactional email | transactional templates | in spec |
| Action: **internal notification email** (to staff, no consent) | `NOTIFY` | in spec, make it explicit |
| Action: start a WhatsApp flow | — | defer |
| Action: wait, if/else, split | step kinds | in spec |
| Action: **go to** (jump within graph; terminal for its branch) | `GOTO` step | in spec (2026-11-05) |
| Action: **Jump To** (the renamed Goal: scope below/anywhere, continue-or-wait if not met, tracking) | `GOAL` + `GoalConfig` | in spec (2026-11-05) |
| Action: complete a deal task, add contact to account, account note, account owner | `COMPLETE_TASK`, `ADD_TO_ACCOUNT`, `ADD_ACCOUNT_NOTE`, `UPDATE_ACCOUNT_OWNER` | in spec (2026-11-05) |
| Action: format data (copy, date ±, extract phone/URL, math, capitalise) | `FORMAT_DATA` | in spec (2026-11-05), was deferred |
| Workflow settings: entry (once / any / after exit), exit rules (unsubscribe, conversion), unsubscribe scope, access groups, tracking to goal/conversion | `Workflow.reentry/exit_rules/unsubscribe_scope/access_group_ids/tracking` | in spec (2026-11-05) |
| Trigger: runs once / multiple, segment filter per trigger | `WorkflowTrigger.runs/segment` | in spec (2026-11-05) |
| Wait: period / specific day-time pattern in contact tz / date field on-before-after with passed-date rule / conditions with timeout | `WaitConfig` | in spec (2026-11-05) |
| Split: even / percent / test (winner by date or count) / until | `SplitConfig` | in spec (2026-11-05) |
| Action: start / **end another automation** | `START_WORKFLOW` | **add `STOP_WORKFLOW` target** |
| Action: webhook | `CALL_WEBHOOK` | in spec |
| Action: format data | — | defer |
| Action: subscribe / unsubscribe, update field, tag ±, note, score ± | yes | in spec |
| Action: **convert** (fire a conversion) | — | **add** with conversions entity |
| Action: **add a task** | `CREATE_TASK` → `crm/tasks` | in spec (2026-11-05) |
| Action: **archive contact** | — | **add** |
| Action: add deal, update title/status/owner/value/stage/custom field, deal task/note, deal score, account field | `CREATE_DEAL`, `UPDATE_DEAL_*`, `ADD_DEAL_NOTE`, `ADJUST_DEAL_SCORE` | in spec (2026-11-05) |

### Email
| AC feature | Our service | Status |
|---|---|---|
| Campaigns (one-time, A/B, RSS, automated) | `crm/outreach` campaigns | in spec |
| Content manager (image/asset library) | — | **add assets** (small) |
| Templates | versioned templates | in spec |
| **Message variables** (reusable snippets) | — | **add snippets** |
| Form archive | — | n/a |
| Brand kits (logo, colours, fonts → template defaults) | — | **add brand kit** (settings) |
| **Campaign folders** | `folder` string on campaign | in spec (thin) |
| **Resend to unopened** | — | **add** (campaign action) |
| Reports: campaign performance | yes | in spec |
| Reports: email client trends, predictive content, open/read trends | — | defer |

### Deals
| AC feature | Our service | Status |
|---|---|---|
| Deals kanban, pipelines, stages | `crm/pipeline` `Pipeline`, `Stage.card_fields`, `Deal` | in spec (2026-11-05) |
| **Tasks** (type, due, outcome, owner; on deals and contacts) | `crm/tasks` | in spec (2026-11-05) |
| Saved responses | `crm/tasks` `SavedResponse` | in spec (2026-11-05) |
| Deal custom fields (text, textarea, number, money, date, datetime, dropdown, listbox; groups) | `crm/platform` custom fields (entity = deal) | in spec (`FieldType` MONEY/MULTISELECT/TEXTAREA, `group`, `show_on_card`, 2026-11-05) |
| Deal settings: **deal roles**, win probability (ML), value brackets | `crm/pipeline` `DealRole`, `DealSettings` | in spec (2026-11-05); ML estimate itself deferred (`basis` chooses) |
| Connected emails (OAuth/IMAP sync, 1:1 send, tracking) | `crm/platform` `Mailbox` | in spec (2026-11-05) |
| Task settings (types, outcomes, require outcome) | `crm/tasks` `TaskType`, `Outcome`, `TaskSettings` | in spec (2026-11-05) |
| Calendars (iCal feeds) | `crm/tasks` `CalendarFeed` | in spec (2026-11-05) |
| Reports: deals overview, owner, funnel, sales performance, task overview, forecast | `crm/reporting` `/deals/*`, `/tasks/overview` | in spec (2026-11-05) |
| Import/Export Status | `crm/platform` `Job` | in spec (2026-11-05) |

### Website
| AC feature | Our service | Status |
|---|---|---|
| Forms (inline, floating, modal, hosted) | `crm/marketing` form builder | in spec |
| Pages (landing-page builder) | — | defer |
| Content manager | assets (see Email) | **add** |
| **Site messages** (on-site bar / popup with trigger rules) | — | **add** as `FormStyle.layout = SITE_MESSAGE` + message-only content |
| Site tracking (whitelist, code, event tracking API) | tracked sites + events | in spec |

### Business goals & conversions
| AC feature | Our service | Status |
|---|---|---|
| **Conversions** (named event with value; trigger = page / event / form / deal won / automation action) | — | **add entity** in `crm/marketing` |
| **Business goals** (target, progress, conversion attached) | — | **add** (small, on top of conversions) |
| Marketing revenue report | `CampaignReport.revenue` | **wire to conversions** |

### SMS / WhatsApp
| AC feature | Our service | Status |
|---|---|---|
| SMS broadcast + automation-triggered (US/AU/UK) | `Channel = SMS` on template/campaign/send | **add as a real channel**: provider adapter (Twilio-shaped), STOP/START keyword handling, quiet hours, per-country sender id, SMS consent on `Subscription` |
| WhatsApp flows (conversational qualify/sell) | — | defer (a builder of its own) |
| WhatsApp as a plain channel | `Channel = WHATSAPP` | later, same adapter contract |

### Settings
| AC feature | Our service | Status |
|---|---|---|
| Account, security, users | Sesame | role scopes only |
| **Addresses** (postal, several, chosen per sender) | `Sender.postal_address` | **add addresses entity** |
| **Brand kits** | — | **add** |
| Appearance | — | n/a |
| **Conversions** | — | **add** (above) |
| Developer (API keys, webhooks) | `crm/platform` | in spec |
| **Domains** (CNAME for tracking / hosted links) | — | **add tracking domain** to senders/settings |
| Import/export status | jobs | in spec |
| Integrations marketplace | `crm/platform` integrations | defer |
| **Data manager** (GDPR export / erase) | — | **add jobs** |
| **Notifications** (which staff get which internal alerts) | — | **add** |
| Saved responses | `crm/tasks` `SavedResponse` | in spec (2026-11-05) |

### Reports
| AC feature | Our service | Status |
|---|---|---|
| Your Dashboard (prompt-built widgets, per-widget schedule) | `crm/reporting` `Dashboard` / `DashboardWidget` | in spec (2026-11-05); prompt → query translation later |
| Automations: all, overview, goals, contacts, jump-to tracking | `crm/reporting` `/automations/*` | in spec (2026-11-05) |
| Automated 1:1 emails | `crm/reporting` `/automations/one-to-one-emails` | in spec (2026-11-05) |
| Contacts: performance, field breakdown, nearby, tag trends | `crm/reporting` `/contacts/*` | in spec (2026-11-05) |
| All campaigns performance (compare-to, MPP unverified opens, anonymous open rate) | `crm/outreach` `/reports/campaigns` | in spec (2026-11-05) |
| Per-campaign report (links, geo, clients, contact lists by outcome) | `crm/outreach` `CampaignReport`, `/campaigns/{id}/report/contacts` | in spec; verify after a real send |
| Email client trends, open/read trends, predictive content | `crm/reporting` `/email/*` | in spec (2026-11-05) |
| Marketing revenue (7-day attribution) | `crm/reporting` `/revenue/marketing` from conversions | in spec (2026-11-05) |
| WhatsApp reports (7), Ecommerce dashboard | — | defer (channel / integration not in scope) |

## 2. Curated v1 additions (ordered by shape change, then size)

Shape changes:
1. ~~`crm/automation.Workflow.triggers[]` — several start triggers per workflow.~~ Done 2026-11-05, with the builder's step configs (shape-diff § Automation builder).
2. ~~`crm/pipeline` — multiple pipelines (`Pipeline` entity; stages belong to a pipeline; deal has `pipeline_id`).~~ Done 2026-11-05 (shape-diff § Deals menu).
3. SMS as a first-class channel in `crm/outreach` — adapter contract, consent, keywords, quiet hours.

New entities:
4. ~~`Task` (`crm/pipeline` or new `crm/tasks`): subject, type, due, owner, outcome, linked contact/deal; `TASK_COMPLETED` trigger; `CREATE_TASK` action.~~ Done 2026-11-05 as `crm/tasks` (types, outcomes, reminders, calendar feeds, saved responses).
5. `Conversion` + `BusinessGoal` (`crm/marketing`): name, value, trigger definition, counts; `CONVERT` action; revenue attribution.
6. `ScoreRule` / `Score` (`crm/intelligence`): named scores (contact, deal), rules with points and decay.
7. `ListExclusion` (`crm/outreach`).
8. `Snippet` (message variables) and `Asset` (content manager) (`crm/outreach`).
9. `PostalAddress`, `BrandKit`, `TrackingDomain`, `NotificationRule`, `DataRequest` (GDPR) (`crm/platform` / `crm/outreach`).
10. `BulkEditJob`, `archive` on contact (`crm/contacts`).

New step/trigger/action enum values:
11. Steps: `GOTO`. Triggers: `MESSAGE_FORWARDED`, `MESSAGE_SHARED`, `FILE_DOWNLOADED`, `ENTERED_PIPELINE`, `DEAL_VALUE_CHANGED`, `DEAL_STATUS_CHANGED`, `DEAL_FIELD_CHANGED`, `ACCOUNT_FIELD_CHANGED`, `TASK_COMPLETED`, `RSS_ITEM`, `JUMPED_TO`. Actions: `SEND_1TO1_EMAIL`, `INTERNAL_NOTIFICATION` (rename of NOTIFY), `CONVERT`, `ARCHIVE_CONTACT`, `UPDATE_DEAL_OWNER`, `UPDATE_DEAL_VALUE`, `UPDATE_DEAL_STATUS`, `UPDATE_DEAL_TITLE`, `UPDATE_DEAL_FIELD`, `ADD_DEAL_NOTE`, `ADJUST_DEAL_SCORE`, `UPDATE_ACCOUNT_FIELD`, `STOP_WORKFLOW` (other workflow).
12. Campaign action: `resend_to_unopened`. (2026-11-05: campaign editor walked — `REMINDER`/`RECURRING` kinds, `CampaignAudience`, `ReminderConfig`, `TrackedLink`, readiness, AB `test_scope`/`ratios`/`decide_winner` in spec; shape-diff § Campaign editor.)
13. Form layout: `SITE_MESSAGE`. (2026-11-05: form builder walked — layout/field kinds, `SUBSCRIBE_SMS_LIST`/`EMAIL_RESULTS` actions, on_submit, submission controls in spec; shape-diff § Form builder.)

Beyond ActiveCampaign (2026-11-06):
14. `crm/marketing` **ad audiences** (`/ads/accounts`, `/ads/audiences`): a segment / list / tag kept in step with a Meta / Google / LinkedIn / X / TikTok / DSP custom audience, hashed server-side, with consent basis, suppression rules, lookalikes and a `WARM_UP` purpose (add N days before a send, remove on click). Automation actions `ADD_TO_AD_AUDIENCE` / `REMOVE_FROM_AD_AUDIENCE`. AC only has the Facebook Custom Audience action.
15. `crm/marketing` **ad tracking & attribution** (`/ads/settings`, `/ads/clicks`, `/ads/link-tags`, `/ads/conversion-rules`, `/ads/conversions`, `/ads/calls`, `/ads/campaigns`, `/ads/spend`, `/ads/attribution*`, `/ads/contacts/{id}/journey`, `/ads/ltv`): first-party click-id capture with a server-set cookie on the tracking domain, link decoration with the workspace click id and a contact token, server-side conversion relay (Meta CAPI, Google offline / enhanced, LinkedIn, TikTok), spend import, platform-reported vs attributed, custom attribution models, journeys, LTV cohorts, warm-up lift vs holdout. The Hyros-style "print" (cookie + click ids + email + phone + soft fingerprint) joined deterministically. Trigger `AD_CLICKED`.

Deferred, deliberately: landing-page builder, WhatsApp flows, win-probability ML, predictive content/sending, connected mailboxes and calendars, saved responses, database sync, geo reports, integrations marketplace, "format data".

## 3. Reference tenant

The ActiveCampaign trial is configured as a reference tenant by script (`openapi/crm/reference/activecampaign/seed.sh`) so that every object shape can be pulled back through its v3 API and diffed field by field against our schemas. Objects seeded: lists, contacts with tags and fields, segments, score rules, one pipeline with stages and deals and tasks, campaigns (one A/B) to a staff-only list, automations (welcome, abandoned signup, hot-lead handoff), forms, site tracking whitelist, conversions and a business goal. SMS and WhatsApp are paywalled on the trial; their adapter contracts come from documentation.

## 4. Mock screens still to add

Mock screens added 2026-11-05 (portal + design): deal detail, deal reports (overview / owner / funnel / sales performance / task overview / forecast), automation report (overview / goals / contacts), contact reports (performance / tag trends / field breakdown / nearby), email trends (open times / clients / predictive), marketing revenue, dashboard widgets, connected emails, import/export status, task settings (types / outcomes / calendar feeds / saved responses). Still to add: Conversions & goals detail, Score rules editor, List exclusions, Bulk edit, Snippets & assets, Brand kit, Postal addresses, Tracking domain, Notifications, GDPR data requests, Site messages, SMS campaign variant, multi-trigger in the builder, Automation messages view, deal custom-field groups in Settings.
