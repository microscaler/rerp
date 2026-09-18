# RERP CRM System Design Document

> **Version:** 1.0.0
> **Scope:** Complete architectural blueprint, data model, and API contract strategy
> **Priority:** Foundation for OpenAPI specification generation across all 11 components
> **Status:** Active design spec
> **Repository Structure:** `openapi/crm/`

---

## Document Structure

This document is organized into modular design documents. Use this table of contents to navigate to the specific design area you need.

| # | Document | Description | Diagrams |
|---|----------|-------------|----------|
| 0 | **This document** | Table of contents and executive overview | — |
| 1 | [01-architecture.md](docs/01-architecture.md) | System topology, service mesh, deployment, auth flow | Architecture, sequence, matrix, topology |
| 2 | [02-entity-relationships.md](docs/02-entity-relationships.md) | Complete ERD for all 11 services and their relationships | ERD diagrams per service + unified summary |
| 3 | [03-data-flow.md](docs/03-data-flow.md) | CRUD flows, event-driven patterns, CDC, batch processing | Sequence diagrams, flow charts |
| 4 | [04-automation-engine.md](docs/04-automation-engine.md) | Workflow trigger-execution model, rule engine, cron | Flow charts, sequence diagrams, state diagrams |
| 5 | [05-graph-database.md](docs/05-graph-database.md) | Hierarchical trees, relationship graphs, scoring models | Graph diagrams, state diagrams |

---

## Executive Overview

RERP CRM is an **OpenAPI-first, Rust-native, self-hosted** customer relationship management platform. Unlike proprietary CRM ecosystems that lock data into UI-driven workflows, RERP defines every entity, endpoint, and business rule in machine-readable OpenAPI 3.1.0 specifications. This enables:

- **Automatic SDK generation** for TypeScript, Python, Go, and Java
- **Strict API contracts** with zero drift between frontend, mobile, and backend
- **Rust-level performance** for sub-millisecond latency and bulk operations on 100,000+ records
- **Self-hosted data sovereignty** with no per-seat pricing, rate limits, or vendor lock-in

The system is organized into **11 functional components** (microservices), each representing a critical dimension of modern CRM.

---

## System Components

### Component Overview

| Component | OpenAPI Dir | Paths | Schemas | Description |
|-----------|-------------|-------|---------|-------------|
| **Pipeline** | `pipeline/` | 9 | 18 | Lead lifecycle, stages, stages history, won/lost, revenue |
| **Contacts** | `contacts/` | 7 | 9 | Contact management, contact tree, merge contacts |
| **Accounts** | `accounts/` | 6 | 10 | Account hierarchy, company tree, industry classification |
| **Teams** | `teams/` | 6 | 12 | Team management, member assignment, capacity-based distribution |
| **Automation** | `automation/` | 14 | 27 | Workflow automation, rules, triggers, execution logs |
| **Intelligence** | `intelligence/` | 21 | 29 | Lead scoring (Bayesian PLS), enrichment, data verification |
| **Engagement** | `engagement/` | 14 | 21 | Subscriptions, renewals, goals, gamification, health scoring |
| **Livechat** | `livechat/` | 15 | 23 | Chat sessions, messages, agent management, conversion |
| **Marketing** | `marketing/` | 14 | 26 | UTM tracking, web forms, visitor analytics, event registration |
| **Reporting** | `reporting/` | 17 | 23 | Analytics, dashboards, forecasts, KPI digests |
| **Platform** | `platform/` | 14 | 27 | Custom fields, webhooks, API keys, audit logs, integrations |

---

## Architecture at a Glance

```
Client Tier (Web/Mobile/CLI/External)
    │
    ▼
Gateway Layer (openapi.yaml)
    │
    ├── pipeline/    ← Lead lifecycle, stages, revenue
    ├── contacts/    ← Contact tree, merge, dedup
    ├── accounts/    ← Account hierarchy, industry
    ├── teams/       ← Team assignment, capacity
    ├── automation/  ← Workflows, rules, triggers
    ├── intelligence/← Scoring, enrichment, verification
    ├── engagement/  ← Subscriptions, goals, badges
    ├── livechat/    ← Chat sessions, agents, conversion
    ├── marketing/   ← UTM, forms, visitors, events
    ├── reporting/   ← Analytics, forecasts, digests
    └── platform/    ← Custom fields, webhooks, audit, API keys
    │
    ▼
Storage Tier (PostgreSQL — 4 DBs: Core, Analytics, Events, Platform)
```

---

## Key Design Principles

1. **OpenAPI-first:** Every entity, field, and endpoint defined in specs before implementation
2. **Unified Lead Model:** One entity for leads and opportunities; conversion is a state change, not a data copy
3. **Rust Performance:** Sub-millisecond API latency, async I/O, batch operations on 100K+ records
4. **Self-Hosted Sovereignty:** No per-seat pricing, no rate limits, full data ownership
5. **Explainable AI:** Bayesian scoring with top_factors, not black-box ML
6. **API-Defined Workflows:** Automation defined in JSON/JSON schema, not UI-only config

---

## Data Architecture: 4-DB Strategy

```
┌─────────────────────────────────────────────────────────┐
│  DB 1: CRM Core (Primary)                               │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Schema: crm_core                                  │  │
│  │  pipeline/  — leads, stages, stage_history        │  │
│  │  contacts/  — contacts, contact_tree, merge       │  │
│  │  accounts/  — accounts, account_tree, industry    │  │
│  │  teams/     — teams, team_members, assignments    │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│  DB 2: CRM Analytics                                    │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Schema: crm_analytics                             │  │
│  │  automation/  — workflows, rules, triggers, logs  │  │
│  │  intelligence/— scoring_frequencies, lead_scores  │  │
│  │  reporting/   — analytics, forecasts              │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│  DB 3: CRM Events                                       │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Schema: crm_events                                │  │
│  │  livechat/  — chat_sessions, chat_messages        │  │
│  │  marketing/ — utm_campaigns, web_forms, visitors  │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│  DB 4: CRM Platform                                     │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Schema: crm_platform                              │  │
│  │  custom_fields / webhooks / audit_log             │  │
│  │  api_keys / integrations                          │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## OpenAPI Compliance Standards

All 11 specs follow these BRRTRouter-compliant conventions:

- **OpenAPI 3.1.0** with `openapi: "3.1.0"` version string
- **2 server entries:** `http://localhost:10352` + `/api/v1`
- **Components:** `securitySchemes`, `parameters`, `schemas` (exactly these 3)
- **Security:** `bearerAuth` (HTTP Bearer, JWT) on every operation
- **Parameters:** Shared `Id` (path, uuid), `Page` (query, int), `Limit` (query, int)
- **Pagination:** `PaginatedResponse` base class + typed `PaginatedXxx` for list endpoints
- **Error handling:** `ErrorResponse` schema on all operations (codes 200, 201, 204, 400, 401, 403, 404, 409, 500)
- **Operation IDs:** snake_case, e.g., `create_lead`, `get_lead_by_id`
- **BRRTRouter markers:** `x-brrtrouter-impl: true` on all POST/PUT/PATCH operations

---

## Implementation Roadmap

### Phase 1: Foundation (P0 — Weeks 1-4)
- Unified `Lead` entity, `Contact`, `Account` linkage
- `Stage` entity, `StageHistory`, `LostReason`, won/lost cascades
- Revenue fields on Lead (`expected_revenue`, `prorated_revenue`)
- `Team` and `TeamMember` entities with capacity-based assignment
- Basic deduplication (email/phone normalization, fuzzy match)
- Lead conversion and merge endpoints

### Phase 2: Pipeline & Analytics (P1 — Weeks 5-8)
- Stage transition validation, rotting detection
- `ScoringFrequency` and `LeadScore` entities, Bayesian PLS cron job
- `RepPerformance`, `PipelineSummary`, `ConversionRate` analytics
- `UTMCampaign`/`UTMMedium`/`UTMSource` tracking
- Email templates, activity timeline, calendar event linking

### Phase 3: Automation & Engagement (P1/P2 — Weeks 9-12)
- `Workflow`, `Rule`, `Trigger` entities, stage-change/time-based triggers
- `ChatSession`, `ChatMessage`, `Agent` entities, livechat endpoints
- `Subscription`, `RenewalAlert` entities, MRR/churn tracking
- `Goal`, `Badge`, `UserBadge` gamification entities, leaderboard
- Customer health score calculation, at-risk detection

### Phase 4: Intelligence, Marketing & Platform (P3/P4 — Weeks 13-16)
- `EnrichmentRequest`/`EnrichmentResult` entities, Clearbit/Hunter API
- `WebForm`, `VisitorTracking`, `EventRegistration` entities
- `CustomField`, `Webhook`, `AuditLog`, `ApiKey` entities
- SDK generation pipeline, sandbox environment, two-way ERP sync
- Advanced reporting: cohort analysis, territory mapping, ad-hoc query builder

---

## Explicit Non-Goals (V1)

- No visual drag-and-drop workflow builder (API/JSON definition only)
- No built-in landing page builder (public form endpoint is enough)
- No native email client (SMTP/IMAP integration via external providers)
- No mobile app (responsive web + generated mobile SDKs)
- No built-in BI dashboard UI (API returns raw analytics data)

---

*This document serves as the single source of truth for RERP CRM architecture. All sub-specs (`pipeline/`, `contacts/`, `accounts/`, `teams/`, `automation/`, `intelligence/`, `engagement/`, `livechat/`, `marketing/`, `reporting/`, `platform/`) must align with these entities, endpoints, and patterns.*
