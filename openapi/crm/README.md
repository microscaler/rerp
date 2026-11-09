# CRM Services

## Overview

RERP CRM is an **OpenAPI-first, Rust-native, self-hosted** customer relationship management platform. The system is organized into **13 independent microservices**, each with its own OpenAPI specification, generated code, and business logic. No single service failure disables the entire CRM.

**Architecture:** 13 services, 5 databases, event-driven communication.

> **Restructuring (accepted 2026-11-09):** [docs/08-service-decomposition.md](docs/08-service-decomposition.md) and
> [ADR 004](../../docs/adrs/004-crm-capture-intake-and-workspaces.md) regroup the CRM into seven services
> (`audience`, `messaging`, `automation`, `sales`, `acquisition`, `insights`, `workspace`), the public `edge` and the
> `intake` quarantine. Their specs are the contracts of record for new work. The older folders (`contacts`, `outreach`,
> `social`, `platform`, `teams`, `tasks`, `marketing`, `reporting`) are **deprecated** and stay only until each new
> service is implemented; `pipeline` and `accounts` remain in use by the running services (`accounts` stays the
> customer master, ADR 003).

## Architecture

```
openapi/crm/
├── openapi.yaml          # Gateway spec: routing, auth, rate-limiting, service discovery
├── pipeline/             # Lead/Opportunity lifecycle, stages, conversion, merge, dedup
├── contacts/             # Contact CRUD, org hierarchy, address management, contact merge
├── accounts/             # Company/account CRUD, industry classification, hierarchy
├── teams/                # Sales teams, team members, auto-assignment, unassigned queue
├── automation/           # Workflow automation, rules, triggers, scheduled actions
├── livechat/             # Chat sessions, messages, agent management, chat-to-lead
├── engagement/           # Subscriptions, renewals, MRR/churn, gamification, health scoring
├── intelligence/         # Lead scoring, enrichment, verification, ICP mining
├── reporting/            # Analytics, dashboards, forecasts, performance metrics
├── marketing/            # UTM tracking, web forms (builder), site tracking, events, surveys, attribution, ad audiences + ad tracking
├── outreach/             # Lists & consent, templates, campaigns, sends, engagement events, transactional, senders
├── social/               # Connected accounts, content calendar, approval, engagement, social lead capture
├── platform/             # Custom fields, webhooks, audit log, API keys, integrations
└── CRM_ANALYSIS/         # Odoo CRM module analysis and design references
```

## Service Architecture

Each service owns its OpenAPI sub-spec, generated code (gen/), and business logic (impl/). Services are grouped into 4 databases, each with independent connection pools and schema ownership.

### Database Layout

| Database | Services | Responsibility |
|----------|----------|----------------|
| **crm_core** | pipeline, contacts, accounts, teams, tasks | High-frequency transactional operations |
| **crm_analytics** | automation, reporting, intelligence | Read-heavy analytics and processing |
| **crm_events** | livechat, marketing | Communication and event capture |
| **crm_outreach** | outreach, social | Marketing store outside the tenant DB (ADR 003): lists, consent, sends, events, posts |
| **crm_platform** | platform | Infrastructure and extensibility |

Within each database, services use independent schemas. Cross-service JOINs are possible within the same database; cross-database queries use HTTP calls or async event subscription.

## Services

### Core Transactional Services (crm_core database)

#### Pipeline Service
- **Path**: `crm/pipeline/`
- **Description**: Lead and opportunity lifecycle management, pipeline stages, conversion, duplicate detection, and lead merge
- **API Spec**: [Pipeline OpenAPI](./pipeline/openapi.yaml)
- **Key capabilities**:
  - CRUD for leads (qualification mode) and opportunities (pipeline mode)
  - Multiple pipelines, each with its own ordered stages (probability, colour, rotting days, card fields)
  - Deals: value/currency, owner, status (OPEN/WON/LOST), contacts with roles and a primary contact, notes, files, activity, emails
  - Deal roles, deal settings (win-probability mode, value brackets, default pipeline), bulk deal operations
  - Stage transitions with validation and probability tracking
  - Lead conversion (lead → opportunity + contact/account)
  - Duplicate detection by email/phone/name
  - Lead merge with data consolidation
  - Pipeline summary and weighted revenue

#### Contacts Service
- **Path**: `crm/contacts/`
- **Description**: Contact management with org hierarchy, address management, and contact merge
- **API Spec**: [Contacts OpenAPI](./contacts/openapi.yaml)
- **Key capabilities**:
  - Full CRUD for contacts (individuals and companies)
  - Organization hierarchy (parent/child relationships)
  - Contact merge with reference updates
  - Email-based dedup lookup
  - Linked leads and accounts

#### Accounts Service
- **Path**: `crm/accounts/`
- **Description**: Company and account management with industry classification, hierarchy, and reporting
- **API Spec**: [Accounts OpenAPI](./accounts/openapi.yaml)
- **Key capabilities**:
  - Full CRUD for company accounts
  - Corporate hierarchy (parent/subsidiary)
  - Industry classification and company sizing
  - Linked contacts and leads
  - Account-level analytics (by industry, size)

#### Teams Service
- **Path**: `crm/teams/`
- **Description**: Sales team and member management with capacity-based auto-assignment
- **API Spec**: [Teams OpenAPI](./teams/openapi.yaml)
- **Key capabilities**:
  - Team CRUD with privacy/security settings
  - Team member management with quota tracking
  - Capacity-based auto-assignment algorithm
  - Unassigned lead queue
  - Manual assignment triggers

#### Tasks Service
- **Path**: `crm/tasks/`
- **Description**: Calls, emails, meetings and to-dos on contacts, deals and accounts, with task types, outcomes, reminders, calendar feeds and saved responses
- **API Spec**: [Tasks OpenAPI](./tasks/openapi.yaml)
- **Key capabilities**:
  - Task CRUD with due date/time, assignee, reminders, complete/reopen and bulk operations
  - Task types (ordered, each with its allowed outcomes) and an outcome catalogue (label + POSITIVE/NEUTRAL/NEGATIVE sentiment)
  - Task settings (require an outcome on completion, default reminders)
  - Automation hook: a task can start a workflow when it is added or completed; completion fires `TASK_COMPLETED`
  - Per-user iCal calendar feeds (scope all / deals / contacts, pipeline and assignee filters, signed public URL)
  - Saved responses (categorised, merge-tag templates) rendered for 1:1 emails and tasks
  - Task overview numbers feed `crm/reporting` task reports

### Marketing Delivery Services (crm_outreach database)

#### Outreach Service
- **Path**: `crm/outreach/`
- **Description**: Lists and consent, versioned templates, one-time and A/B campaigns, per-recipient sends and engagement events, transactional mail, suppression, sender identities, and the public open/click/unsubscribe/preference endpoints
- **API Spec**: [Outreach OpenAPI](./outreach/openapi.yaml) - [README](./outreach/README.md)
- **Key capabilities**:
  - Per-list consent with an untrimmed audit trail and double opt-in
  - Templates (Markdown/MJML/HTML/text) with merge fields and versions
  - Campaign audience resolution (lists minus exclusions, suppressed, unsubscribed; segment filter)
  - A/B variants with sample share, winner metric and decision time
  - Delivery log and normalized provider events; RFC 8058 one-click unsubscribe
  - Transactional send sharing the suppression list
  - Provider adapters (SES, Postmark, Resend, Twilio, Meta) as deployment configuration

#### Social Service
- **Path**: `crm/social/`
- **Description**: Connected social accounts, content calendar, approval gate, engagement metrics, social lead capture
- **API Spec**: [Social OpenAPI](./social/openapi.yaml) - [README](./social/README.md)
- **Key capabilities**:
  - Posts with per-platform targets and results
  - Schedule, approve, skip, publish now; pause per account
  - Engagement per post and per account over time
  - Comment / DM / mention / lead-form capture into crm/pipeline

### Analytics & Automation Services (crm_analytics database)

#### Automation Service
- **Path**: `crm/automation/`
- **Description**: Workflow automation engine with triggers, rules, and scheduled actions
- **API Spec**: [Automation OpenAPI](./automation/openapi.yaml)
- **Key capabilities**:
  - Workflow definitions with triggers (stage change, field change, time-based)
  - Condition-action rules with test evaluation
  - Scheduled and recurring actions
  - Workflow execution history and logging
  - Email notifications, field updates, lead assignment

#### Reporting Service
- **Path**: `crm/reporting/`
- **Description**: Business intelligence, dashboards, forecasting, and performance analytics
- **API Spec**: [Reporting OpenAPI](./reporting/openapi.yaml)
- **Key capabilities**:
  - Deal reports: overview (totals, value histogram, owner and stage tables), owner overview, funnel (stage-to-stage progression), sales performance (stage × owner), forecast with accuracy
  - Task overview (load, completion and outcome sentiment per rep)
  - Automation reports (overview, goals, contacts, jump-to, automated 1:1 emails), contact reports (performance, field breakdown, nearby, tag trends), email trends (clients, open times, predictive content), marketing revenue, per-user dashboards with prompt-built widgets
  - Pipeline summaries by stage, team, and rep
  - Conversion rate analysis and funnel visualization
  - Lead source effectiveness and attribution
  - Individual and team performance metrics
  - Win/loss analysis and time-to-close breakdowns
  - Monthly/quarterly revenue forecasting
  - Custom report builder with scheduling and export

#### Intelligence Service
- **Path**: `crm/intelligence/`
- **Description**: Predictive lead scoring, external data enrichment, verification, and ICP mining
- **API Spec**: [Intelligence OpenAPI](./intelligence/openapi.yaml)
- **Key capabilities**:
  - Bayesian lead scoring with explainable top factors
  - Hot/warm/cold lead bucketing
  - External enrichment (Clearbit, Hunter, ZoomInfo)
  - Email and phone verification
  - ICP-based lead discovery and mining
  - Batch scoring and enrichment operations

### Communication Services (crm_events database)

#### Livechat Service
- **Path**: `crm/livechat/`
- **Description**: Real-time chat with visitor tracking, agent management, and chat-to-lead conversion
- **API Spec**: [Livechat OpenAPI](./livechat/openapi.yaml)
- **Key capabilities**:
  - Chat session lifecycle (waiting → active → closed/transferred)
  - Real-time messaging with transcripts
  - Agent online status and queue management
  - Capacity-based agent assignment
  - Agent performance metrics
  - Chat-to-lead conversion

#### Marketing Service
- **Path**: `crm/marketing/`
- **Description**: Campaign tracking, web forms, visitor analytics, events, and surveys
- **API Spec**: [Marketing OpenAPI](./marketing/openapi.yaml)
- **Key capabilities**:
  - UTM campaign/medium/source tracking
  - Public web form capture (creates leads automatically)
  - Anonymous visitor tracking and page views
  - Visitor identification by email
  - Campaign attribution and revenue tracking
  - Event registration and survey responses

### Engagement Services

#### Engagement Service
- **Path**: `crm/engagement/`
- **Description**: Customer engagement with subscriptions, renewals, gamification, and health scoring
- **API Spec**: [Engagement OpenAPI](./engagement/openapi.yaml)
- **Key capabilities**:
  - Subscription lifecycle (creation, renewal, cancellation)
  - MRR and churn rate tracking
  - Automatic renewal alerts
  - Gamification goals with progress tracking
  - Team and individual leaderboards
  - Badges and achievements
  - Customer health scoring and at-risk detection

### Platform Services (crm_platform database)

#### Platform Service
- **Path**: `crm/platform/`
- **Description**: Extensibility platform with custom fields, webhooks, audit logging, API keys, and integrations
- **API Spec**: [Platform OpenAPI](./platform/openapi.yaml)
- **Key capabilities**:
  - Connected mailboxes (OAuth Google/Microsoft or IMAP): two-way sync of 1:1 mail onto contacts and deals, 1:1 sends with open/click tracking
  - Import/export jobs with progress, row counts, per-row errors and result download
  - Per-entity custom field definitions
  - Webhook delivery with retry and HMAC signing
  - Immutable audit log for all operations
  - API key management with scoped permissions
  - Third-party integration configuration
  - Data sync triggers and health monitoring

## API Gateway

This system provides a unified API gateway at `/api/v1/crm` that:

- Routes requests to appropriate sub-services (each with `/api/v1/crm/{service}/` path)
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration and aggregated responses
- Aggregates metrics and monitoring

The gateway spec (`openapi.yaml`) is auto-generated from all sub-service specs by `brrtrouter-gen`.

## Service Communication

### Synchronous (REST/JSON over HTTP/2)
- Read-only cross-service queries (reporting querying pipeline, intelligence querying contacts)
- Simple reference lookups (pipeline fetching account details by ID)
- Configuration data access (teams, stages, custom fields)

### Asynchronous (Pub/Sub)
- State transitions requiring multiple reactors (lead won → subscription + reporting + workflows)
- Lead creation from multiple sources (forms, chat, manual)
- Enrichment completion callbacks
- Webhook event distribution

## Integration Patterns

The CRM services work together to provide complete functionality:

- **Lead lifecycle**: Marketing/form → Lead creation → Pipeline assignment → Stage progression → Conversion → Subscription creation
- **Contact management**: Created manually, via form, or from lead conversion → linked to accounts → enriched by intelligence service
- **Analytics pipeline**: All services publish events → reporting service aggregates → dashboards and forecasts
- **Automation engine**: Listens to pipeline events → fires workflows → updates fields, sends notifications, triggers external integrations

*Detailed integration patterns are documented in [DESIGN.md](./DESIGN.md).*

## API Documentation

All OpenAPI specs are in `openapi/crm/`:

| Service | OpenAPI Spec | Tags | Paths | Schemas |
|---------|-------------|------|-------|---------|
| Pipeline | [pipeline/openapi.yaml](./pipeline/openapi.yaml) | lead-conversion, lead-deduplication, lead-stages, leads, pipeline-analytics, pipeline-stage, stages, pipelines, deals, deal-roles | 28 | 47 |
| Contacts | [contacts/openapi.yaml](./contacts/openapi.yaml) | tags, segments, contacts, contacts-accounts, contacts-hierarchy, contacts-leads, contacts-merge, contacts-search | 18 | 22 |
| Accounts | [accounts/openapi.yaml](./accounts/openapi.yaml) | accounts, accounts-contacts, accounts-hierarchy, accounts-leads, accounts-reporting | 6 | 11 |
| Teams | [teams/openapi.yaml](./teams/openapi.yaml) | lead-assignment, members, team-members, teams | 6 | 12 |
| Tasks | [tasks/openapi.yaml](./tasks/openapi.yaml) | tasks, task-types, outcomes, calendars, saved-responses, settings | 21 | 27 |
| Automation | [automation/openapi.yaml](./automation/openapi.yaml) | enrolments, workflow-steps, actions, automation-execution, rules, triggers, workflows | 31 | 48 |
| Livechat | [livechat/openapi.yaml](./livechat/openapi.yaml) | agent-metrics, agents, chats, chats-conversion, chats-messages, messages | 15 | 23 |
| Engagement | [engagement/openapi.yaml](./engagement/openapi.yaml) | badges, engagement-badges, engagement-health, engagement-leaderboard, goals, goals-progress, health, renewals, subscriptions, subscriptions-analytics, subscriptions-renewal | 14 | 21 |
| Intelligence | [intelligence/openapi.yaml](./intelligence/openapi.yaml) | enrichment, lead-enrichment, lead-mining, lead-queues, lead-scoring, lead-scoring-frequencies, lead-scoring-thresholds, lead-verification, mining, scoring, verification | 21 | 29 |
| Reporting | [reporting/openapi.yaml](./reporting/openapi.yaml) | conversion, forecast, performance, pipeline, report-analytics, report-forecast, report-performance, report-scheduling, report-time-to-close, report-win-loss, reports, win-loss, deal-reports, task-reports, automation-reports, contact-reports, email-reports, revenue-reports, dashboards | 43 | 84 |
| Marketing | [marketing/openapi.yaml](./marketing/openapi.yaml) | ad-attribution, ad-audiences, ad-buying, ad-tracking, campaigns, events, forms, marketing-analytics, marketing-campaigns, marketing-events, marketing-forms, marketing-surveys, surveys, utm, utm-campaigns, utm-mediums, utm-sources, visitors | 63 | 73 |
| Outreach | [outreach/openapi.yaml](./outreach/openapi.yaml) | lists, subscriptions, templates, campaigns, sends, events, transactional, suppressions, senders, public, provider-webhooks, outreach-reports | 40 | 65 |
| Social | [social/openapi.yaml](./social/openapi.yaml) | social-accounts, social-posts, social-calendar, social-engagement, social-leads | 13 | 18 |
| Platform | [platform/openapi.yaml](./platform/openapi.yaml) | api-keys, audit-log, auth, custom-fields, integrations, webhooks, mailboxes, jobs | 25 | 49 |
| **Total** | | **130 tags** | **344** | **529** |

*The gateway spec (`openapi.yaml`) is auto-generated by brrtrouter-gen from all sub-service specs.*

## Design Documentation

- **[DESIGN.md](./DESIGN.md)** — Complete architectural blueprint, data model, API contracts, and event-driven infrastructure

## Code Generation

Each service follows the two-crate model:
- **Generated crate**: `microservices/{suite}/{service}/gen/` — Auto-generated from OpenAPI spec using `brrtrouter-gen`
- **Implementation crate**: `microservices/{suite}/{service}/impl/` — Business logic

To generate code from a spec:
```bash
cd microservices/crm/{service}/impl
brrtrouter-gen --spec ../../../openapi/crm/{service}/openapi.yaml --output ../gen
```
