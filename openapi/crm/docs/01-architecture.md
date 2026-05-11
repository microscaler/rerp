# CRM System Architecture Design

> **Version:** 1.0.0
> **Scope:** High-level architecture, service boundaries, deployment topology, and communication patterns
> **Status:** Active design spec

---

## 1. System Topology

RERP CRM is an **OpenAPI-first, Rust-native, self-hosted** platform with 11 independent microservices. Each service owns its OpenAPI spec, generated code (gen/), and business logic (impl/).

```mermaid
graph TB
    subgraph "Client Tier"
        Web[Web Dashboard<br/>SolidJS]
        Mobile[Mobile App<br/>Flutter]
        CLI[CLI Tools<br/>rust + python]
        Ext[External Systems<br/>ERP/Accounting]
    end

    subgraph "Gateway Layer"
        GW[CRM Gateway<br/>openapi.yaml]
        Auth[Auth Middleware<br/>bearerAuth + RBAC]
    end

    subgraph "Service Tier — Core"
        Pipeline[pipeline/ — Leads & Stages<br/>9 paths, 18 schemas]
        Contacts[contacts/ — Contacts & Tree<br/>7 paths, 9 schemas]
        Accounts[accounts/ — Accounts & Hierarchy<br/>6 paths, 10 schemas]
        Teams[teams/ — Teams & Assignment<br/>6 paths, 12 schemas]
    end

    subgraph "Service Tier — Automation"
        Automation[automation/ — Workflows & Rules<br/>14 paths, 27 schemas]
    end

    subgraph "Service Tier — Intelligence"
        Intelligence[intelligence/ — Scoring & Enrichment<br/>21 paths, 29 schemas]
    end

    subgraph "Service Tier — Engagement"
        Engagement[engagement/ — Subscriptions & Goals<br/>14 paths, 21 schemas]
    end

    subgraph "Service Tier — Communication"
        Livechat[livechat/ — Chat Sessions & Agents<br/>15 paths, 23 schemas]
    end

    subgraph "Service Tier — Marketing"
        Marketing[marketing/ — UTM & Forms<br/>14 paths, 26 schemas]
    end

    subgraph "Service Tier — Analytics"
        Reporting[reporting/ — Analytics & Forecasts<br/>17 paths, 23 schemas]
    end

    subgraph "Service Tier — Platform"
        Platform[platform/ — Custom Fields, Webhooks, Audit<br/>14 paths, 27 schemas]
    end

    subgraph "Storage Tier"
        DB[(PostgreSQL<br/>CRM Core DB)]
    end

    Web --> GW
    Mobile --> GW
    CLI --> GW
    Ext --> GW

    GW --> Auth
    GW --> Pipeline
    GW --> Contacts
    GW --> Accounts
    GW --> Teams
    GW --> Automation
    GW --> Intelligence
    GW --> Engagement
    GW --> Livechat
    GW --> Marketing
    GW --> Reporting
    GW --> Platform

    Pipeline --> DB
    Contacts --> DB
    Accounts --> DB
    Teams --> DB
    Automation --> DB
    Intelligence --> DB
    Engagement --> DB
    Livechat --> DB
    Marketing --> DB
    Reporting --> DB
    Platform --> DB
```

---

## 2. Service Communication Matrix

```mermaid
matrix
    title "Service Communication Matrix (direct API calls)"
    columns ["", "pipeline", "contacts", "accounts", "teams", "automation", "intelligence", "engagement", "livechat", "marketing", "reporting"]
    rows ["pipeline", "", "GET/{id}/leads", "GET/{id}/accounts", "GET user_id/team_id", "POST trigger", "POST score/bulk", "POST opportunity", "", "POST contact from lead", "GET summary"]
    rows ["contacts", "POST create lead", "", "GET/{id}/contacts", "GET user_ids", "POST contact create trigger", "", "POST subscription", "", "POST contact from form", "GET rep performance"]
    rows ["accounts", "POST create lead", "POST create contact", "", "GET contacts/leads", "", "POST enrichment", "POST subscription", "", "POST contact from form", "GET by industry"]
    rows ["teams", "", "", "", "", "GET team assignments", "", "", "", "", "GET team performance"]
    rows ["automation", "POST stage change trigger", "POST contact change", "POST account change", "", "", "POST workflow", "POST renewal alert", "", "", "POST report schedule"]
    rows ["intelligence", "", "", "", "", "", "", "POST scoring", "", "", "GET scoring frequencies"]
    rows ["engagement", "", "POST goal progress", "POST goal progress", "", "POST subscription goals", "POST enrichment", "", "", "POST goals", "GET leaderboards"]
    rows ["livechat", "", "POST create contact", "POST create account", "POST assign agent", "POST chat conversion trigger", "", "POST chat conversion", "", "", "GET conversion rates"]
    rows ["marketing", "", "POST create lead", "POST create contact", "POST create account", "", "POST enrichment", "", "", "", "GET campaign leads"]
    rows ["reporting", "GET pipeline summary", "GET contact stats", "GET account stats", "GET team stats", "GET workflow stats", "GET scoring stats", "GET subscription stats", "GET chat stats", "GET campaign stats", ""]
```

---

## 3. Deployment Architecture

```mermaid
graph TB
    subgraph "Production Environment"
        subgraph "Load Balancer"
            LB[Nginx / HAProxy]
        end

        subgraph "Application Layer (Kubernetes Pods)"
            GW_Pod[Gateway Pod<br/>1 replica]
            Pipeline_Pod[pipeline Pod<br/>3 replicas]
            Contacts_Pod[contacts Pod<br/>3 replicas]
            Accounts_Pod[accounts Pod<br/>2 replicas]
            Teams_Pod[teams Pod<br/>2 replicas]
            Automation_Pod[automation Pod<br/>2 replicas]
            Intelligence_Pod[intelligence Pod<br/>2 replicas]
            Engagement_Pod[engagement Pod<br/>2 replicas]
            Livechat_Pod[livechat Pod<br/>2 replicas]
            Marketing_Pod[marketing Pod<br/>2 replicas]
            Reporting_Pod[reporting Pod<br/>2 replicas]
            Platform_Pod[platform Pod<br/>2 replicas]
        end

        subgraph "Database Layer"
            PG_Primary[(PostgreSQL<br/>Primary<br/>16 CPU / 64GB)]
            PG_Replica[(PostgreSQL<br/>Replica<br/>Read Replicas)]
        end

        subgraph "External Services"
            Clearbit[Clearbit API]
            Hunter[Hunter.io API]
            SMTP[SMTP Server]
            Webhook[Webhook Endpoints]
        end

        LB --> GW_Pod
        GW_Pod --> Pipeline_Pod
        GW_Pod --> Contacts_Pod
        GW_Pod --> Accounts_Pod
        GW_Pod --> Teams_Pod
        GW_Pod --> Automation_Pod
        GW_Pod --> Intelligence_Pod
        GW_Pod --> Engagement_Pod
        GW_Pod --> Livechat_Pod
        GW_Pod --> Marketing_Pod
        GW_Pod --> Reporting_Pod
        GW_Pod --> Platform_Pod

        Pipeline_Pod --> PG_Primary
        Contacts_Pod --> PG_Primary
        Accounts_Pod --> PG_Primary
        Teams_Pod --> PG_Primary
        Automation_Pod --> PG_Primary
        Intelligence_Pod --> PG_Primary
        Engagement_Pod --> PG_Primary
        Livechat_Pod --> PG_Primary
        Marketing_Pod --> PG_Primary
        Reporting_Pod --> PG_Primary
        Platform_Pod --> PG_Primary

        Pipeline_Pod -.-> PG_Replica
        Contacts_Pod -.-> PG_Replica
        Reporting_Pod -.-> PG_Replica
        Intelligence_Pod -.-> PG_Replica

        Intelligence_Pod --> Clearbit
        Intelligence_Pod --> Hunter
        Automation_Pod --> SMTP
        Platform_Pod --> Webhook
    end

    subgraph "Development Environment (Tilt)"
        Tilt[Tiltfile]
        Tilt --> PG_Dev[(PostgreSQL<br/>Dev DB)]
        Tilt --> Services[All 11 Services<br/>Localhost:10352]
    end
```

---

## 4. Request Flow Architecture

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as Gateway
    participant Auth as Auth Middleware
    participant Service as Target Service
    participant DB as PostgreSQL
    participant Ext as External API

    C->>GW: POST /api/v1/pipeline/leads
    Note over GW: Request headers: Authorization: Bearer <JWT>
    GW->>GW: Rate limit check (per API key)
    GW->>Auth: Validate JWT token
    Auth->>GW: Token valid, return user_id + scopes
    GW->>Service: POST /api/v1/pipeline/leads
    Note over Service: Request body: CreateLeadRequest
    Service->>Service: Validate schema + required fields
    Service->>DB: BEGIN TRANSACTION
    DB->>Service: Transaction started
    Service->>DB: INSERT INTO leads (...)
    DB->>Service: Row ID returned
    Service->>DB: INSERT INTO stage_history (...)
    DB->>Service: Audit log written
    Service->>DB: COMMIT
    DB->>Service: Success
    Service->>DB: SELECT * FROM leads WHERE id = $1
    DB->>Service: Lead entity returned
    Service->>GW: 201 Created + Lead entity
    GW->>C: 201 Created + Lead JSON

    alt Enrichment Triggered
        Service->>Ext: POST enrichment/lookup
        Ext->>Service: EnrichmentResult
        Service->>DB: UPDATE leads SET enriched = $1
    end
```

---

## 5. Gateway Routing Topology

```mermaid
graph LR
    subgraph "Gateway Routes (openapi.yaml)"
        Gateway[Gateway<br/>openapi.yaml]
        
        subgraph "Core Routes"
            P[pipeline/leads<br/>pipeline/stages]
            C[contacts/*<br/>accounts/*]
            T[teams/*]
        end

        subgraph "Automation Routes"
            A[automation/workflows<br/>automation/rules]
        end

        subgraph "Intelligence Routes"
            I[intelligence/scoring<br/>intelligence/enrichment]
        end

        subgraph "Engagement Routes"
            E[engagement/subscriptions<br/>engagement/goals]
        end

        subgraph "Communication Routes"
            L[livechat/chats<br/>livechat/agents]
        end

        subgraph "Marketing Routes"
            M[marketing/utm<br/>marketing/forms]
        end

        subgraph "Analytics Routes"
            R[reporting/analytics<br/>reporting/reports]
        end

        subgraph "Platform Routes"
            X[platform/custom-fields<br/>platform/webhooks<br/>platform/api-keys]
        end

        Gateway --> P
        Gateway --> C
        Gateway --> T
        Gateway --> A
        Gateway --> I
        Gateway --> E
        Gateway --> L
        Gateway --> M
        Gateway --> R
        Gateway --> X
    end
```

---

## 6. Cross-Service Event Flow

```mermaid
flowchart TD
    subgraph "Trigger Events"
        LeadCreate[Lead Created<br/>pipeline]
        StageChange[Stage Changed<br/>pipeline]
        ContactCreate[Contact Created<br/>contacts]
        DealWon[Deal Won<br/>pipeline]
        FormSubmit[Form Submitted<br/>marketing]
        ChatClosed[Chat Closed<br/>livechat]
    end

    subgraph "Event Bus (Internal)"
        EventBus[Internal Event Bus<br/>pub/sub pattern]
    end

    subgraph "Listeners"
        Listen1[Automation Service<br/>Workflow Trigger]
        Listen2[Intelligence Service<br/>Scoring Update]
        Listen3[Reporting Service<br/>Stats Aggregation]
        Listen4[Engagement Service<br/>Goal Progress]
        Listen5[Platform Service<br/>Webhook Delivery]
        Listen6[Contacts Service<br/>Auto-Create Contact]
    end

    LeadCreate --> EventBus
    StageChange --> EventBus
    ContactCreate --> EventBus
    DealWon --> EventBus
    FormSubmit --> EventBus
    ChatClosed --> EventBus

    EventBus --> Listen1
    EventBus --> Listen2
    EventBus --> Listen3
    EventBus --> Listen4
    EventBus --> Listen5
    EventBus --> Listen6

    Listen1 --> Workflow[Execute Workflow Rules]
    Listen2 --> Score[Recompute Lead Score]
    Listen3 --> Stats[Update Analytics Stats]
    Listen4 --> Goal[Update Goal Progress]
    Listen5 --> Webhook[Send Webhook Payload]
    Listen6 --> Contact[Create Contact Entity]
```

---

## 7. Data Partitioning Strategy (4-DB Architecture)

```mermaid
graph TB
    subgraph "Database 1: CRM Core"
        subgraph "Schema: crm_core"
            Pipeline[(Pipeline Schema<br/>leads, stages, stage_history)]
            Contacts[(Contacts Schema<br/>contacts, contact_tree, merge_history)]
            Accounts[(Accounts Schema<br/>accounts, account_tree)]
            Teams[(Teams Schema<br/>teams, team_members)]
        end
    end

    subgraph "Database 2: CRM Analytics"
        subgraph "Schema: crm_analytics"
            Automation[(Automation Schema<br/>workflows, rules, triggers)]
            Intelligence[(Intelligence Schema<br/>scoring, enrichment)]
            Reporting[(Reporting Schema<br/>analytics, forecasts)]
        end
    end

    subgraph "Database 3: CRM Events"
        subgraph "Schema: crm_events"
            Livechat[(Livechat Schema<br/>chat_sessions, chat_messages)]
            Marketing[(Marketing Schema<br/>utm_campaigns, forms)]
        end
    end

    subgraph "Database 4: CRM Platform"
        subgraph "Schema: crm_platform"
            CustomFields[(Custom Fields Schema)]
            Webhooks[(Webhooks Schema)]
            Audit[(Audit Log Schema)]
            APIKeys[(API Keys Schema)]
            Integrations[(Integrations Schema)]
        end
    end

    subgraph "Service Connections"
        S1[pipeline/ → DB1]
        S2[contacts/ → DB1]
        S3[accounts/ → DB1]
        S4[teams/ → DB1]
        S5[automation/ → DB2]
        S6[intelligence/ → DB2]
        S7[reporting/ → DB2]
        S8[livechat/ → DB3]
        S9[marketing/ → DB3]
        S10[platform/ → DB4]
    end

    S1 --> Pipeline
    S2 --> Contacts
    S3 --> Accounts
    S4 --> Teams
    S5 --> Automation
    S6 --> Intelligence
    S7 --> Reporting
    S8 --> Livechat
    S9 --> Marketing
    S10 --> CustomFields
```

---

## 8. Authentication & Authorization Flow

```mermaid
sequenceDiagram
    participant Client as Client
    participant GW as Gateway
    participant IDAM as IDAM Service
    participant Service as Target Service
    participant DB as PostgreSQL

    Client->>GW: Request + Authorization: Bearer <JWT>
    GW->>GW: Extract JWT token
    GW->>IDAM: Validate token + get tenant_id
    IDAM->>IDAM: Verify JWT signature (RS256)
    IDAM->>IDAM: Check token expiration
    IDAM->>GW: { user_id, tenant_id, scopes, roles }
    
    GW->>GW: Check rate limit (per API key)
    GW->>GW: Apply route-level CORS policy

    GW->>Service: Request with X-Tenant-ID header
    Service->>Service: Validate tenant context
    
    alt Resource requires ownership check
        Service->>DB: Query resource WHERE owner_id = $1
        DB->>Service: Resource ownership confirmed
    end
    
    Service->>Service: Apply RLS policies
    Note over Service: SET LOCAL rls_policy = 'tenant_id = $1'
    
    Service->>DB: Execute query with RLS
    DB->>Service: Filtered results
    Service->>GW: Response with RLS-filtered data
    GW->>Client: Final response
```

---

*This document defines the high-level architecture. See **02-entity-relationships.md** for entity-level design, **03-data-flow.md** for detailed request/response flows, **04-automation-engine.md** for workflow execution details, and **05-graph-database.md** for hierarchical entity graphs.*
