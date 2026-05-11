# CRM Services

## Overview

RERP CRM is an **OpenAPI-first, Rust-native, self-hosted** customer relationship management platform. The system is organized into **11 independent microservices**, each with its own OpenAPI specification, generated code, and business logic. No single service failure disables the entire CRM.

**Architecture:** 11 services, 4 databases, event-driven communication.

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
├── marketing/            # UTM tracking, web forms, visitor analytics, events, surveys
├── platform/             # Custom fields, webhooks, audit log, API keys, integrations
└── CRM_ANALYSIS/         # Odoo CRM module analysis and design references
```

## Service Architecture

Each service owns its OpenAPI sub-spec, generated code (gen/), and business logic (impl/). Services are grouped into 4 databases, each with independent connection pools and schema ownership.

### Database Layout

| Database | Services | Responsibility |
|----------|----------|----------------|
| **crm_core** | pipeline, contacts, accounts, teams | High-frequency transactional operations |
| **crm_analytics** | automation, reporting, intelligence | Read-heavy analytics and processing |
| **crm_events** | livechat, marketing | Communication and event capture |
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
| Pipeline | [pipeline/openapi.yaml](./pipeline/openapi.yaml) | leads, stages | 9 | 14 |
| Contacts | [contacts/openapi.yaml](./contacts/openapi.yaml) | contacts | 7 | 6 |
| Accounts | [accounts/openapi.yaml](./accounts/openapi.yaml) | accounts | 6 | 6 |
| Teams | [teams/openapi.yaml](./teams/openapi.yaml) | teams, members | 6 | 6 |
| Automation | [automation/openapi.yaml](./automation/openapi.yaml) | workflows, rules, triggers, actions | 14 | 20 |
| Livechat | [livechat/openapi.yaml](./livechat/openapi.yaml) | chats, messages, agents | 15 | 16 |
| Engagement | [engagement/openapi.yaml](./engagement/openapi.yaml) | subscriptions, renewals, goals, badges, health | 14 | 15 |
| Intelligence | [intelligence/openapi.yaml](./intelligence/openapi.yaml) | scoring, enrichment, verification, mining | 21 | 22 |
| Reporting | [reporting/openapi.yaml](./reporting/openapi.yaml) | pipeline, conversion, performance, win-loss, forecast, reports | 17 | 13 |
| Marketing | [marketing/openapi.yaml](./marketing/openapi.yaml) | utm, forms, visitors, campaigns, events, surveys | 14 | 17 |
| Platform | [platform/openapi.yaml](./platform/openapi.yaml) | custom-fields, webhooks, audit-log, api-keys, integrations | 14 | 19 |
| **Total** | | **38 tags** | **137** | **154** |

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
