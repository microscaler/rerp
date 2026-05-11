# RERP CRM System Design Document

> **Version:** 1.0.0
> **Scope:** Complete architectural blueprint, data model, and API contract strategy
> **Priority:** Foundation for OpenAPI specification generation across all 12 components
> **Status:** Active design spec

---

## 1. Executive Vision

RERP CRM is an **OpenAPI-first, Rust-native, self-hosted** customer relationship management platform. Unlike proprietary CRM ecosystems that lock data into UI-driven workflows, RERP defines every entity, endpoint, and business rule in machine-readable OpenAPI 3.1.0 specifications. This enables:

- **Automatic SDK generation** for TypeScript, Python, Go, and Java
- **Strict API contracts** with zero drift between frontend, mobile, and backend
- **Rust-level performance** for sub-millisecond latency and bulk operations on 100,000+ records
- **Self-hosted data sovereignty** with no per-seat pricing, rate limits, or vendor lock-in

The system is organized into **12 functional components**, each representing a critical dimension of modern CRM: lead lifecycle, pipeline management, revenue tracking, predictive analytics, team organization, marketing integration, communication hub, intelligence/enrichment, reporting/BI, workflow automation, customer engagement, and platform extensibility.

---

## 2. Architectural Topology

RERP CRM follows a **modular microservice architecture** aligned with the 12 functional components. Each service owns its OpenAPI sub-spec, generated code (gen/), and business logic (impl/).

```
openapi/crm/
├── openapi.yaml          # Gateway spec: routing, auth, rate-limiting, service discovery
├── core/                 # Lead/Contact Mgmt, Pipeline/Stages, Opportunity/Revenue, Teams
├── automation/           # Workflow Automation, Rules, Scheduling, Cron
├── livechat/             # Livechat Widget, Chat Sessions, Agent Management
├── engagement/           # Customer Engagement: Subscriptions, Renewals, Gamification, Health Scoring
├── intelligence/         # Intelligence & Enrichment: External APIs, Auto-fill, Mining, Verification
├── reporting/            # Reporting & BI: Dashboards, Analytics, Forecasts, KPI Digests
├── marketing/            # Marketing Integration: UTM Tracking, Web Forms, Visitor Analytics, Events/Surveys
└── platform/             # Platform & Extensibility: Custom Fields, Webhooks, API Keys, Audit Logs, Integrations
```

**Service Communication Patterns:**
- **Synchronous:** REST/JSON over HTTP/2. Each service exposes full CRUD and domain-specific endpoints.
- **Asynchronous:** Internal event bus for cross-service triggers (e.g., CRM deal close → Accounting invoice creation).
- **Shared Schemas:** Core entities (`Lead`, `Contact`, `User`, `Team`) are defined in `core/` and referenced via `$ref` by other services.

---

## 3. Core Data Model (The Unified Entity Graph)

### 3.1 The Unified Lead Entity
The foundation of the entire CRM is a single, unified `Lead` entity that handles both leads (qualification mode) and opportunities (pipeline mode) via a `type` discriminator. This eliminates conversion copying and simplifies reporting.

| Field | Type | Required | Computed | Purpose |
|-------|------|----------|----------|---------|
| `id` | UUID | Yes | No | Primary key |
| `name` | String(255) | Yes | No | Lead/opportunity title (trigram-indexed) |
| `type` | Enum[LEAD, OPPORTUNITY] | Yes | No | Unified mode discriminator |
| `email_from` | String(255) | No | Yes | Primary email (normalized for dedup) |
| `email_normalized` | String(255) | No | Yes | Lowercase/stripped email |
| `phone` | String(64) | No | Yes | Primary phone (sanitized for dedup) |
| `phone_sanitized` | String(64) | No | Yes | Digits-only phone |
| `mobile` | String(64) | No | No | Mobile number |
| `title` | String(64) | No | No | Salutation (Mr/Mrs/Mlle) |
| `function` | String(128) | No | No | Job title/position |
| `contact_name` | String(255) | No | No | Contact person name (B2B) |
| `company_name` | String(255) | No | No | Company name on lead |
| `website` | String(255) | No | No | Company website |
| `description` | Text | No | Yes | Internal notes (HTML) |
| `referred_by` | String(255) | No | No | Referral source |
| `priority` | Enum[LOW, NORMAL, HIGH, URGENT] | No | No | Urgency (0-3) |
| `active` | Boolean | No | Yes | Soft delete (default: true) |
| `is_blacklisted` | Boolean | No | Yes | GDPR opt-out |
| `color` | Integer | No | No | Kanban color (1-16) |

**Pipeline Fields:**
`stage_id`, `tag_ids`, `probability`, `automated_probability`, `is_automated_probability`, `date_deadline`, `date_open`, `date_closed`, `date_last_stage_update`, `day_open`, `day_close`

**Revenue Fields:**
`expected_revenue`, `prorated_revenue`, `recurring_revenue`, `recurring_plan_id`, `recurring_revenue_monthly`, `recurring_revenue_monthly_prorated`, `company_currency_id`

**Relationships:**
`partner_id` → `Contact`, `company_id` → `Account`, `user_id` → `User`, `team_id` → `Team`, `campaign_id`/`medium_id`/`source_id` → UTM entities, `duplicate_lead_ids` → Self-reference

### 3.2 Supporting Core Entities

| Entity | Key Fields | Relationship |
|--------|-----------|--------------|
| `Stage` | `name`, `sequence`, `probability`, `is_won`, `is_lost`, `rotting_threshold_days`, `requirements`, `fold`, `color`, `team_ids` | 1:N to Lead |
| `StageHistory` | `lead_id`, `old_stage_id`, `new_stage_id`, `probability_before`, `probability_after`, `user_id`, `transition_date`, `days_in_old_stage` | Audit trail for Lead |
| `LostReason` | `name`, `category`, `sequence`, `active` | 1:N to Lead via `lost_reason_id` |
| `Team` | `name`, `description`, `alias_id`, `use_leads`, `use_opportunities`, `assignment_enabled`, `assignment_domain`, `assignment_max` | 1:N to TeamMember, M:N to Lead |
| `TeamMember` | `user_id`, `crm_team_id`, `assignment_enabled`, `assignment_max`, `lead_month_count`, `quota_monthly` | N:1 to Team, N:1 to User |
| `Contact` | `name`, `email`, `phone`, `mobile`, `title`, `function`, `department`, `company_id`, `is_company`, `parent_id`, `user_ids` | Linked to Lead via `partner_id` |
| `Account` | `name`, `email`, `phone`, `website`, `industry_id`, `company_size`, `employees`, `annual_revenue`, `parent_id`, `child_ids` | Parent company for Contacts/Leads |

### 3.3 Analytics & Reporting Entities

| Entity | Purpose |
|--------|---------|
| `PipelineSummary` | Computed response: stages array with count, total_revenue, weighted_revenue, avg_days_in_stage |
| `ConversionRate` | Period-based aggregation: leads_created, opportunities_created, won/lost counts, rates |
| `RepPerformance` | Per-user metrics: leads, opps, revenue_won, weighted_pipeline, avg_deal_size, sales_cycle_days, quota_attainment |
| `WinLossAnalysis` | Historical breakdown: total_closed, won/lost counts, rates, revenue_impact, reason_breakdown |
| `TimeToCloseAnalysis` | Aggregated avg_days across stages, reps, sources, products, deal sizes |
| `Forecast` | Period-level: weighted_pipeline, expected_close_revenue, confidence, actual_close_revenue, variance |

### 3.4 Automation & Workflow Entities

| Entity | Purpose |
|--------|---------|
| `Workflow` | `name`, `is_active`, `trigger_type` [STAGE_CHANGE, FIELD_CHANGE, TIME_BASED, WEBHOOK, MANUAL], `trigger_config` (JSON), `action_type`, `action_config` (JSON), `conditions` (JSON), `sequence` |
| `Rule` | `workflow_id`, `condition_field`, `condition_operator` [EQUALS, GREATER_THAN, CONTAINS, etc.], `condition_value`, `action_type`, `action_config`, `sequence` |
| `Trigger` | `workflow_id`, `type`, `entity`, `field`, `schedule` (cron), `enabled`, `last_fired` |

### 3.5 Marketing & Communication Entities

| Entity | Purpose |
|--------|---------|
| `UTMCampaign`/`UTMMedium`/`UTMSource` | Hierarchical tracking: Campaign → Medium → Source. Linked to Lead via FK. |
| `WebForm` | `name`, `capture_fields` (JSON schema), `redirect_url`, `team_id`, `user_id`, `consent_required`, `api_endpoint`, `embed_code` |
| `VisitorTracking` | `visitor_id`, `url_visited`, `referrer`, `user_agent`, `country`, `city`, `first_seen`, `last_seen`, `pages_viewed`, `converted_to_lead`, `lead_id` |
| `EmailCommunication` | `from`, `to`, `cc`, `bcc`, `subject`, `body`, `status` [DRAFT, SENT, DELIVERED, BOUNCED, FAILED], `related_lead_id`, `parent_message_id`, `sent_at`, `opened_at`, `tracking_open_count` |
| `EmailTemplate` | `name`, `subject`, `body` (with merge fields), `is_default`, `category`, `body_plain` |
| `SmsCommunication` | `from`, `to`, `body`, `status`, `related_lead_id`, `sent_at`, `is_inbound`, `provider` |
| `CalendarEvent` | `title`, `description`, `start`, `end`, `location`, `meeting_url`, `status`, `related_lead_id`, `organizer_id`, `attendee_ids`, `external_calendar_id` |
| `ActivityNote` | `activity_type` [EMAIL, CALL, MEETING, NOTE, TASK, FOLLOW_UP], `summary`, `description`, `related_lead_id`, `user_id`, `scheduled_date`, `completed_date` |
| `FollowUpTask` | `title`, `description`, `due_date`, `priority`, `status` [TODO, IN_PROGRESS, DONE, CANCELLED], `assigned_to`, `related_lead_id`, `reminder_date` |

### 3.6 Intelligence & Enrichment Entities

| Entity | Purpose |
|--------|---------|
| `ScoringFrequency` | `variable`, `value`, `won_count`, `lost_count`, `team_id`, `total`. Foundation of Bayesian PLS. |
| `LeadScore` | `lead_id`, `probability`, `automated_probability`, `is_automated_probability`, `top_factors` (JSON), `computation_date`, `total_records` |
| `EnrichmentRequest` | `lead_id`, `email`, `company_name`, `website`, `status` [PENDING, SUCCESS, FAILED], `source`, `request_data` (JSON), `created_at` |
| `EnrichmentResult` | `enrichment_request_id`, `industry`, `size`, `website`, `location`, `tech_stack` (JSON), `funding_total`, `employee_count`, `person_name`, `person_title`, `person_role`, `person_seniority`, `person_email` |
| `DataVerification` | `email`, `phone`, `email_valid`, `email_disposable`, `phone_valid`, `phone_carrier`, `verified_at` |

### 3.7 Customer Engagement Entities

| Entity | Purpose |
|--------|---------|
| `ChatSession` | `visitor_id`, `contact_id`, `lead_id`, `agent_id`, `status` [WAITING, ACTIVE, CLOSED, TRANSFERRED], `started_at`, `ended_at`, `transcript` (JSON), `satisfaction_rating` |
| `ChatMessage` | `session_id`, `sender_type` [AGENT, VISITOR, BOT], `message`, `is_system`, `sent_at`, `read_at` |
| `Agent` | Extends User: `is_online`, `active_sessions`, `max_concurrent`, `total_chats_today`, `avg_response_time_seconds`, `satisfaction_avg`, `queue_position`, `skills` |
| `Subscription` | `opportunity_id`, `customer_id`, `status` [ACTIVE, EXPIRED, CANCELLED, TRIAL, PAST_DUE], `plan_id`, `start_date`, `end_date`, `renewal_date`, `renewal_automatic`, `value`, `monthly_value`, `billing_cycle`, `churn_risk_score` |
| `RenewalAlert` | `subscription_id`, `alert_type` [RENEWAL_REMINDER, PRICE_CHANGE, CONTRACT_END, CANCELLATION_WARNING], `alert_date`, `status`, `assigned_to`, `message` |
| `Goal` | `name`, `type` [REVENUE, DEALS, CALLS, MEETINGS, LEADS], `target_value`, `start_date`, `end_date`, `user_id`, `team_id`, `completed_value`, `completion_percentage` |
| `Badge` | `name`, `description`, `icon`, `criteria` (JSON), `is_active` |
| `UserBadge` | `user_id`, `badge_id`, `earned_at` |

### 3.8 Platform & Extensibility Entities

| Entity | Purpose |
|--------|---------|
| `CustomField` | `entity`, `name`, `label`, `field_type` [STRING, INTEGER, FLOAT, BOOLEAN, DATE, DATETIME, EMAIL, PHONE, URL, TEXT, HTML, SELECT, MANY2ONE, MANY2MANY], `options` (JSON), `required`, `visible`, `searchable` |
| `Webhook` | `name`, `url`, `events` (JSON), `secret`, `is_active`, `headers` (JSON), `retry_count`, `last_triggered`, `last_status` |
| `AuditLog` | `entity`, `entity_id`, `action` [CREATE, UPDATE, DELETE, READ, EXPORT, IMPORT], `user_id`, `changes` (JSON), `ip_address`, `user_agent`, `created_at` |
| `ApiKey` | `name`, `key_hash`, `scopes` (JSON), `rate_limit`, `ip_whitelist`, `is_active`, `last_used`, `expires_at` |
| `Integration` | `name`, `type` [ERP, ACCOUNTING, MARKETING, SUPPORT, CUSTOM], `config` (JSON), `is_active`, `sync_direction` [CRM_TO_EXTERNAL, EXTERNAL_TO_CRM, BIDIRECTIONAL], `last_sync`, `sync_status` |

---

## 4. Service Boundaries & API Contracts

### 4.1 `openapi/crm/openapi.yaml` (Gateway)
Handles cross-cutting concerns:
- Authentication: OAuth2 client credentials, API key validation, RBAC enforcement
- Rate limiting: Per-key/configurable throttling
- Request routing: Dispatches to `core/`, `automation/`, `livechat/`, `engagement/`, `intelligence/`, `reporting/`, `marketing/`, `platform/`
- Global pagination, filtering, and error handling schemas

### 4.2 `openapi/crm/core/openapi.yaml`
**Entities:** `Lead`, `Contact`, `Account`, `Stage`, `StageHistory`, `LostReason`, `Team`, `TeamMember`, `UTMCampaign`, `UTMMedium`, `UTMSource`
**Key Endpoints:**
- `GET/POST/PATCH/DELETE /leads`
- `POST /leads/{id}/convert`
- `POST /leads/{id}/merge`
- `POST /leads/detect-duplicates`
- `GET/POST /contacts`, `/accounts`
- `GET/POST/PATCH /stages`, `PATCH /leads/{id}/stage`
- `GET /pipeline/summary`, `/pipeline/weighted`
- `GET/POST /teams`, `/teams/{id}/members`
- `POST /assign/run`, `GET /queues/unassigned`

### 4.3 `openapi/crm/automation/openapi.yaml`
**Entities:** `Workflow`, `Rule`, `Trigger`
**Key Endpoints:**
- `GET/POST/PATCH/DELETE /workflows`
- `GET/POST /workflows/{id}/rules`
- `POST /rules/test`
- `GET/POST /triggers`
- `POST /actions/schedule`, `/actions/recurring`
- `GET /actions/upcoming`

### 4.4 `openapi/crm/livechat/openapi.yaml`
**Entities:** `ChatSession`, `ChatMessage`, `Agent`
**Key Endpoints:**
- `POST /chats/start`, `/chats/{id}/message`, `/chats/{id}/close`
- `GET /chats/{id}/transcript`, `/chats/active`
- `POST /chats/convert`
- `PUT /agents/{id}/status`, `GET /agents/status`

### 4.5 `openapi/crm/engagement/openapi.yaml`
**Entities:** `Subscription`, `RenewalAlert`, `Goal`, `Badge`, `UserBadge`
**Key Endpoints:**
- `GET/POST /subscriptions`, `/subscriptions/{id}/renew`
- `GET /renewals/upcoming`, `/subscriptions/mrr`
- `GET/POST /goals`, `/goals/{id}/progress`
- `GET /leaderboard`, `/users/{id}/badges`
- `GET /customers/{id}/health`, `/customers/at-risk`

### 4.6 `openapi/crm/intelligence/openapi.yaml`
**Entities:** `ScoringFrequency`, `LeadScore`, `EnrichmentRequest`, `EnrichmentResult`, `DataVerification`
**Key Endpoints:**
- `POST /leads/score-batch`, `GET /leads/{id}/score/explain`
- `GET /scoring/frequencies`, `POST /scoring/frequencies/rebuild`
- `POST /enrichment/lookup`, `/enrichment/batch`, `/enrichment/auto-fill`
- `POST /verify/email`, `/verify/phone`
- `POST /mining/search`, `/mining/icp-match`

### 4.7 `openapi/crm/reporting/openapi.yaml`
**Entities:** `PipelineSummary`, `ConversionRate`, `RepPerformance`, `WinLossAnalysis`, `Forecast`, `KPIDigest`
**Key Endpoints:**
- `GET /analytics/pipeline-summary`, `/conversion-rates`, `/rep-performance`
- `GET /analytics/win-loss`, `/time-to-close`, `/forecast/monthly`
- `POST /reports/build`, `/reports/schedule`, `/reports/export`
- `GET /digests/{id}`

### 4.8 `openapi/crm/marketing/openapi.yaml`
**Entities:** `WebForm`, `VisitorTracking`, `EventRegistration`, `SurveyResponse`
**Key Endpoints:**
- `GET/POST /utm/campaigns`, `/mediums`, `/sources`
- `POST /forms/{id}` (public endpoint)
- `POST /analytics/track`, `/analytics/identify`
- `GET /campaigns/{id}/leads`, `/campaigns/{id}/revenue`
- `POST /events/register`, `/surveys/submit`

### 4.9 `openapi/crm/platform/openapi.yaml`
**Entities:** `CustomField`, `Webhook`, `AuditLog`, `ApiKey`, `Integration`
**Key Endpoints:**
- `GET/POST /custom-fields`, `/entities/{type}/custom-fields`
- `GET/POST /webhooks`, `/webhooks/{id}/test`
- `GET /audit-log`, `/audit-log/{entity}/{id}`
- `GET/POST /api-keys`
- `GET/POST /integrations`, `/integrations/{id}/sync`

---

## 5. Workflow & Automation Engine

### 5.1 Trigger-Action Execution Model
Workflows execute synchronously on entity mutations or asynchronously via cron. The engine evaluates rules in `sequence` order:

1. **Event fired** (e.g., `stage_change` on `crm.lead`)
2. **Workflow matched** by `trigger_type` and `entity`
3. **Conditions evaluated** using `condition_field`, `condition_operator`, `condition_value`
4. **Actions executed** in order:
   - `SEND_EMAIL`: Render template with merge fields, queue SMTP
   - `CREATE_TASK`: Generate `FollowUpTask` record
   - `UPDATE_FIELD`: Patch lead fields atomically
   - `ASSIGN_LEAD`: Run assignment algorithm
   - `CALL_WEBHOOK`: POST to external URL with HMAC signature
5. **Execution logged** to `Workflow.run_count` and `last_run`

### 5.2 System Cron Jobs
| Cron Job | Schedule | Purpose |
|----------|----------|---------|
| `assign_leads` | Daily 09:00 | Capacity-based lead distribution |
| `run_workflows` | Every 5 min | Evaluate time-based triggers |
| `send_digests` | Daily 08:00 | KPI digest email delivery |
| `check_rotting` | Hourly | Flag stalled deals |
| `recompute_scores` | Daily 02:00 | Bayesian PLS frequency rebuild |
| `cleanup_old_data` | Weekly | Archive old activities |

---

## 6. Event-Driven Infrastructure & CDC

### 6.1 Webhook Architecture
- **Payload:** `{ event, timestamp, data: { id, fields... } }`
- **Delivery:** Async HTTP POST with exponential backoff retry (configurable)
- **Security:** Optional HMAC-SHA256 signature using `secret` key
- **Events:** `lead.create`, `lead.update`, `lead.delete`, `opportunity.won`, `opportunity.lost`, `contact.create`, `subscription.renewal_due`

### 6.2 Change Data Capture (CDC)
For real-time sync with external systems (ERP, accounting, marketing):
- Triggered on `CREATE`, `UPDATE`, `DELETE` to core entities
- Publishes to internal event bus or external webhook URL
- Captures `before` and `after` state for auditing and reconciliation
- Supports bidirectional sync flags for two-way ERP integration

---

## 7. Security, Governance & Compliance

### 7.1 Authentication & Authorization
- **OAuth2/OIDC:** Standard authorization code + client credentials flows
- **API Keys:** Scoped permissions (`read`, `write`, `admin` per entity), configurable rate limits, IP whitelisting, expiration
- **RBAC:** Role-based access control (owner, team, org visibility) mapped to `Team.privacy_security`
- **Field-Level Security:** Granular controls via `CustomField.visible` and role mapping

### 7.2 GDPR & Data Governance
- **Blacklisting:** `is_blacklisted` flag on Lead/Contact, blocks automated email/SMS
- **Right-to-be-forgotten:** `DELETE /leads/{id}` performs soft delete (`active=false`), anonymizes PII on cascade
- **Consent Tracking:** Captured on `WebForm` submissions, stored in `request_data` JSON
- **Audit Logging:** Every action (`CREATE`, `UPDATE`, `DELETE`, `READ`, `EXPORT`, `IMPORT`) logged to `AuditLog` with IP, user agent, and field diffs
- **Data Retention:** Configurable `data_retention_until` fields, automated archival via cron

---

## 8. OpenAPI Generation Strategy

### 8.1 Spec Structure
- **Gateway (`openapi.yaml`):** Defines global security schemes, error response schemas, pagination envelopes, and service routing paths.
- **Sub-specs (`core/`, `automation/`, etc.):** Define entity schemas, endpoint signatures, request/response bodies, and validation rules.
- **Cross-References:** Core entities use `$ref` to shared schemas in `components/schemas/`. Sub-services reference them rather than duplicating.

### 8.2 Codegen Pipeline
1. `openapi-generator-cli` generates TypeScript (axios), Python (urllib3), Go (openapi-generator), Java (retrofit2) SDKs
2. Generated types enforce compile-time safety in client applications
3. Contract testing (`openapi-contract-test`) validates impl/ handlers against specs on CI

### 8.3 Pagination, Filtering & Sorting
All list endpoints follow unified conventions:
```yaml
# Request
?limit=20&offset=0
&sort=-create_date,+name
&filter[stage_id]=uuid
&filter[probability_gt]=50

# Response
{
  "data": [...],
  "meta": { "total": 1420, "limit": 20, "offset": 0 }
}
```

---

## 9. Phased Implementation Roadmap

### Phase 1: Foundation (P0 — Weeks 1-4)
- Unified `Lead` entity with all ~70 fields, `Contact`, `Account` linkage
- `Stage` entity, `StageHistory`, `LostReason`, won/lost cascades
- Revenue fields (`expected_revenue`, `prorated_revenue`) on Lead
- `Team` and `TeamMember` entities with capacity-based assignment algorithm
- Basic deduplication (email/phone normalization, fuzzy match)
- Lead conversion and merge endpoints

### Phase 2: Pipeline & Analytics (P1 — Weeks 5-8)
- Stage transition validation, rotting detection, pipeline summary endpoints
- `ScoringFrequency` and `LeadScore` entities, Bayesian PLS cron job
- `RepPerformance`, `PipelineSummary`, `ConversionRate` analytics endpoints
- `UTMCampaign`/`UTMMedium`/`UTMSource` tracking, basic campaign reporting
- Email templates, activity timeline aggregation, calendar event linking

### Phase 3: Automation & Engagement (P1/P2 — Weeks 9-12)
- `Workflow`, `Rule`, `Trigger` entities, stage-change and time-based triggers
- `ChatSession`, `ChatMessage`, `Agent` entities, livechat endpoints
- `Subscription`, `RenewalAlert` entities, MRR/churn tracking
- `Goal`, `Badge`, `UserBadge` gamification entities, leaderboard
- Customer health score calculation, at-risk detection

### Phase 4: Intelligence, Marketing & Platform (P3/P4 — Weeks 13-16)
- `EnrichmentRequest`/`EnrichmentResult` entities, Clearbit/Hunter API integration
- `WebForm`, `VisitorTracking`, `EventRegistration` entities, public form capture
- `CustomField`, `Webhook`, `AuditLog`, `ApiKey` entities
- SDK generation pipeline, sandbox environment, two-way ERP sync
- Advanced reporting: cohort analysis, territory mapping, ad-hoc query builder

---

## 10. Design Principles & Non-Goals

### Principles
1. **OpenAPI-first:** Every entity, field, and endpoint defined in specs before implementation
2. **Unified Lead Model:** One entity for leads and opportunities; conversion is a state change, not a data copy
3. **Rust Performance:** Sub-millisecond API latency, async I/O, batch operations on 100K+ records
4. **Self-Hosted Sovereignty:** No per-seat pricing, no rate limits, full data ownership
5. **Explainable AI:** Bayesian scoring with top_factors, not black-box ML
6. **API-Defined Workflows:** Automation defined in JSON/JSON schema, not UI-only config

### Explicit Non-Goals (for V1)
- No visual drag-and-drop workflow builder (API/JSON definition only)
- No built-in landing page builder (public form endpoint is enough)
- No native email client (SMTP/IMAP integration via external providers)
- No mobile app (responsive web + generated mobile SDKs)
- No built-in BI dashboard UI (API returns raw analytics data for client rendering)

---

*This document serves as the single source of truth for RERP CRM architecture. All sub-specs (`core/`, `automation/`, `livechat/`, etc.) must align with these entities, endpoints, and patterns. Divergences require a spec revision and CI contract-test update.*
