# Shape diff — reference tenant objects vs `openapi/crm/*`

Working notes from `dump/` (v3 API responses). "Theirs" is the field as the API returns it; "ours" is where
it lands, or what to add. Only differences worth a spec change are listed.

## Contact (`contacts/*.json`) → `crm/contacts.Contact`
| Theirs | Meaning | Ours |
|---|---|---|
| `sms_consent`, `sms_consent_updated_at` | **channel consent at contact level**, not per list | Add `channel_consents[]` {channel, status, at, source} on Contact; list subscriptions stay per list for email. SMS lists (below) still exist but consent is contact-wide. |
| `open_tracking_optout`, `mpp_tracking` | per-contact tracking opt-out; Apple Mail Privacy Protection flag on opens | Add `tracking_optout` on Contact; add `MessageEvent.machine_open: bool` (MPP / proxy opens) so open rates can exclude them |
| `best_send_hour` | predictive send | defer |
| `bounced_hard`, `bounced_soft`, `sentcnt` | counters on the contact | Add read-only `message_stats` {sent, hard_bounces, soft_bounces, last_open_at, last_click_at} on Contact |
| `scoreValues[]` | one value per named score | Supports the rule-based multi-score model (`crm/intelligence` additions) |
| `accountContacts[]`, `organization`, `orgid`, `orgname` | contact ↔ account link with job title | Marketing-side **company** grouping is needed (decision in feature map §1); add `company_id`/`company_name` on Contact and a light `companies` resource in `crm/contacts` distinct from converted `crm/accounts` |
| `anonymized`, `deleted_at` | GDPR state | Add `anonymized_at` on Contact; GDPR request jobs in `crm/platform` |
| `email_domain`, `email_local`, `hash` | derived | ours derives; `hash` = public token for links, we mint per send instead |
| `whatsapp_id`, `whatsapp_username` | channel identity | later, with WhatsApp channel |

## List (`lists/*.json`) → `crm/outreach.MailingList`
| Theirs | Ours |
|---|---|
| `channel` = email \| sms | **Lists are per channel.** Add `channel` to MailingList (default EMAIL). |
| `optinoptout` (double opt-in on), `optinmessageid`, `optoutconf` (confirm on unsubscribe) | `double_opt_in`, `confirmation_template_id` exist; add `unsubscribe_confirmation: bool` |
| `get_unsubscribe_reason` | ask why on unsubscribe | Add `ask_unsubscribe_reason: bool`; `UnsubscribeRequest.reason` exists |
| `sender_name`, `sender_addr1/2/city/state/zip/country`, `sender_url`, `sender_reminder` | CAN-SPAM postal address and "why you got this" line, **per list** | Add `postal_address_id` and `reminder_text` to MailingList; `PostalAddress` entity in `crm/outreach` |
| `require_name`, `p_use_tracking`, `p_use_captcha` | list-level defaults | `track_*` exist on campaign; leave |
| `subscription_notify`, `unsubscription_notify` | staff emails notified | → `NotificationRule` in `crm/platform` |
| `active_subscribers`, `non_deleted_subscribers` | counts | `subscriber_count`, `unsubscribed_count` exist |

## Campaign (`campaigns/*.json`) → `crm/outreach.Campaign`
| Theirs | Ours |
|---|---|
| `type` = single \| split \| recurring \| responder \| reminder \| activerss | Add `RECURRING` (schedule, e.g. `recurring: day1`), `RESPONDER` (send N after subscribe — ours is an automation; keep as automation), `REMINDER` (date-field campaign, e.g. renewal) to `CampaignKind`; RSS exists |
| `split_type` even \| ..., `split_offset` + `split_offset_type`, `split_content`, `split_winner_messageid` | `ABTestConfig` covers; add `split_content: bool` (body vs subject test) |
| `tracklinks` = all \| mime \| html \| text \| none; `trackreads` | ours booleans; fine |
| `embed_images`, `public` (web archive), `mail_cleanup` | add `public_archive: bool`; others no |
| `segmentid` | campaign audience segment | `segment` exists |
| `survey`, `unsubreasons`, `replies`, `forwards`, `socialshares` | engagement counters incl. replies/forwards/shares | `MESSAGE_FORWARDED`/`SHARED` events + report counters |
| `predictive_send`, `has_predictive_content` | ML send-time and content | defer |
| `messages[]` (campaign → message 1..n) | **content is a separate `message` object** (fromname, fromemail, reply2, subject, preheader, html, text, priority, language) reused across campaigns and automations | Ours: `Template` plays this role; keep. Add `priority` and `language_code` to Template. |
| `reminder_field`, `reminder_offset*`, `reminder_type` | date-field driven send | covered by REMINDER kind |

## Automation (`automations/*.json`) → `crm/automation.Workflow`
| Theirs | Ours |
|---|---|
| `multientry` | ours `reentry` enum; fine |
| `exit_on_unsubscribe`, `exit_on_conversion` | **add** `stop_on_unsubscribe: bool`, `stop_on_conversion_id` |
| `dedup_configuration` | dedupe contacts entering | covered by reentry |
| `entered`, `exited` | counters | `active_enrolments`, `completed_enrolments` exist; add `entered_total` |
| `screenshot` | graph thumbnail | nice-to-have, skip |
| triggers are a sub-resource (not exposed at `/automationTriggers`) | several per automation | `triggers[]` on Workflow (feature map §2.1) |

## Score (`scores/*.json`)
`reltype` = contact \| deal, `status`, rules via UI only. → `crm/intelligence`: `Score {id, name, entity: CONTACT|DEAL, status}` + `ScoreRule {score_id, conditions, points, decay_days}`; contact carries `score_values[]`.

## Segment (`segments/*.json`)
`logic` and \| or, `seriesid` (segment owned by an automation trigger), `hidden`. → ours: `ConditionGroup.op`; add `owner_workflow_id` + `hidden` so trigger segments do not clutter the list.

## Conversion (`conversions/*.json`)
`name`, `value`, `value_option` (fixed \| from field), `currency`, `limit` + `enforcelimit` (count once per contact), `field_data`. → `crm/marketing.Conversion {name, value_mode: FIXED|FIELD, value, value_field, currency, once_per_contact}`; `CONVERT` action; `Workflow.stop_on_conversion_id`.

## Deal / pipeline (`deals`, `dealGroups`, `dealStages`, `dealTasktypes`, `dealRoles`)
| Theirs | Ours |
|---|---|
| `dealGroup` (pipeline): `currency`, `allgroups/allusers` (visibility), `autoassign`, `win_probability_initialize_date` | **add `Pipeline`** in `crm/pipeline`: name, currency, visibility, auto-assign; stages carry `pipeline_id` |
| `dealStage`: `order`, `color`, `width`, `dealOrder` (card sort), `cardRegion1..5` (card layout) | ours has sequence/color/probability; add `card_fields[]` (what the kanban card shows) |
| `deal.status` 0 open / 1 won / 2 lost; `winProbability`; `nextdate` (next task due); `nextdealid`; `account`/`customerAccount`; `fields[]` | ours `won_status`; add `next_task_at`, `pipeline_id`, `owner_id`, `company_id`; custom fields via platform |
| `dealTasktypes` with `outcomes[]`, `defduration` | **Task types with outcomes** → `Task`, `TaskType {title, default_minutes, outcomes[]}` |
| `dealRoles` | **add `DealRole`** and `deal.contacts[] {contact_id, role_id}` (several contacts per deal) |

## Message (`messages/*.json`) and template (`templates/*.json`)
Message = sendable content with sender; template = starting layout. Ours merges both into versioned `Template` with `kind`; keep, add `priority`, `language_code`, `reply_to` override.

## Not reachable on the trial / by API
Forms, site tracking domains, goals, addresses, brandings details, saved responses, calendars: created in the UI; dumped when present.
