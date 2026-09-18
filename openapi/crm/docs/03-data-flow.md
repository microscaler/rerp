# CRM Data Flow Design

> **Version:** 1.0.0
> **Scope:** Request/response flows, event-driven patterns, change data capture, and batch processing
> **Status:** Active design spec

---

## 1. Standard CRUD Request Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as Gateway<br/>(OpenAPI Spec)
    participant Auth as Auth Middleware<br/>(IDAM Service)
    participant S as Target Service<br/>(e.g., pipeline/)
    participant DB as PostgreSQL

    C->>GW: HTTP Request + Bearer Token
    GW->>GW: Parse & validate OpenAPI spec
    GW->>GW: Rate limit check (per API key)
    
    GW->>Auth: Validate JWT
    Auth->>Auth: Verify signature (RS256)
    Auth->>Auth: Check expiration & scopes
    Auth->>GW: {user_id, tenant_id, scopes}

    GW->>S: Forward request with X-Tenant-ID
    Note over S: Route to handler by operationId

    S->>S: Parse request body
    S->>S: Validate against OpenAPI schema
    Note over S: Check required fields, formats, enums

    alt Invalid Request
        S->>GW: 400 Bad Request + ErrorResponse
        GW->>C: 400 + error details
    end

    S->>DB: BEGIN TRANSACTION
    S->>DB: Execute CRUD operation
    DB->>S: Row affected
    S->>S: Enforce RLS policies
    Note over S: SET LOCAL rls_policy = 'tenant_id = $1'
    
    S->>DB: COMMIT

    alt Audit Required
        S->>DB: INSERT INTO audit_log
    end

    alt Webhook Event
        S->>DB: INSERT INTO webhook_delivery_log
        S->>Ext: Async HTTP POST
    end

    S->>GW: 200/201/204 + Response Body
    GW->>C: Final Response + Pagination Headers
```

---

## 2. Lead Creation & Enrichment Flow

```mermaid
sequenceDiagram
    participant Client as Sales Rep
    participant GW as Gateway
    participant Pipeline as Pipeline Service
    participant Contacts as Contacts Service
    participant Intelligence as Intelligence Service
    participant Automation as Automation Service
    participant External as Clearbit/Hunter API

    Client->>GW: POST /leads
    Note over GW: Body: {name, email, company...}
    GW->>Pipeline: Forward with auth context

    Pipeline->>Pipeline: Normalize email
    Pipeline->>Pipeline: Sanitize phone
    Pipeline->>Pipeline: Check for duplicates

    alt Duplicate Found
        Pipeline->>Client: 409 Conflict + duplicate_lead_ids
    end

    Pipeline->>DB: INSERT lead + stage_history
    Pipeline->>Contacts: POST /contacts (async)
    Note over Contacts: Create contact if email unique

    Pipeline->>Intelligence: POST /enrichment/lookup
    Intelligence->>External: Fetch company data
    External->>Intelligence: Company profile + tech stack
    Intelligence->>Intelligence: Store enrichment_result
    Intelligence->>Pipeline: Return enrichment data
    Pipeline->>Pipeline: Update lead with enriched data

    Pipeline->>Automation: Publish event: lead.created
    Automation->>Automation: Match workflow triggers
    Note over Automation: Trigger: type=MANUAL

    Pipeline->>GW: 201 Created + Lead entity
    GW->>Client: Lead JSON with all fields
```

---

## 3. Stage Transition & Scoring Flow

```mermaid
sequenceDiagram
    participant Client as Sales Rep
    participant GW as Gateway
    participant Pipeline as Pipeline Service
    participant Intelligence as Intelligence Service
    participant Automation as Automation Service
    participant Reporting as Reporting Service

    Client->>GW: PATCH /leads/{id}/stage
    Note over GW: Body: {stage_id, probability}
    GW->>Pipeline: Forward request

    Pipeline->>Pipeline: Validate stage transition
    Note over Pipeline: Check: sequence forward, is_won/is_lost

    alt Invalid Transition
        Pipeline->>Client: 409 Conflict
    end

    Pipeline->>DB: BEGIN TRANSACTION
    Pipeline->>DB: UPDATE leads SET stage_id = $1, probability = $2
    Pipeline->>DB: INSERT stage_history (old → new)
    Pipeline->>DB: COMMIT

    Pipeline->>Intelligence: POST /leads/{id}/score/recompute
    Intelligence->>Intelligence: Fetch scoring_frequencies
    Intelligence->>Intelligence: Compute Bayesian PLS probability
    Intelligence->>Pipeline: Return new probability & top_factors
    Pipeline->>DB: UPDATE leads SET automated_probability = $1

    Pipeline->>Automation: Publish event: stage_changed
    Automation->>Automation: Match workflow triggers
    Note over Automation: Execute: SEND_EMAIL, CREATE_TASK, etc.

    Pipeline->>Reporting: Publish event: stats_update
    Reporting->>DB: UPDATE pipeline_summary

    Pipeline->>GW: 200 OK + Updated Lead
    GW->>Client: Lead JSON with new stage
```

---

## 4. Lead Conversion Flow (Lead → Contact + Account)

```mermaid
sequenceDiagram
    participant Client as Sales Rep
    participant GW as Gateway
    participant Pipeline as Pipeline Service
    participant Contacts as Contacts Service
    participant Accounts as Accounts Service
    participant Automation as Automation Service

    Client->>GW: POST /leads/{id}/convert
    Note over GW: Body: {type, create_contact, create_account}
    GW->>Pipeline: Forward request

    Pipeline->>Pipeline: Verify lead can be converted
    Note over Pipeline: Only OPPORTUNITY type

    Pipeline->>DB: BEGIN TRANSACTION
    Pipeline->>Pipeline: Update lead type → OPPORTUNITY

    alt create_contact = true
        Pipeline->>Contacts: POST /contacts
        Note over Contacts: Map lead fields to contact
        Contacts->>Contacts: Create contact entity
        Contacts->>Pipeline: Return contact_id
        Pipeline->>DB: UPDATE leads SET partner_id = $1
    end

    alt create_account = true
        Pipeline->>Accounts: POST /accounts
        Note over Accounts: Map lead fields to account
        Accounts->>Accounts: Create account entity
        Accounts->>Pipeline: Return account_id
        Pipeline->>DB: UPDATE leads SET company_id = $1
    end

    Pipeline->>DB: COMMIT

    Pipeline->>Automation: Publish event: lead.converted
    Automation->>Automation: Trigger subscription creation
    Note over Automation: For opportunity → subscription

    Pipeline->>GW: 200 OK + ConversionResult
    GW->>Client: {lead_id, contact_id, account_id}
```

---

## 5. Event-Driven Cross-Service Communication

```mermaid
flowchart TD
    subgraph "Event Producers"
        P[Pipeline Service]
        C[Contacts Service]
        A[Accounts Service]
        E[Engagement Service]
        M[Marketing Service]
        L[Livechat Service]
    end

    subgraph "Event Bus"
        EP[Event Producer<br/>Internal Pub/Sub]
        EP --> MP[Message Broker<br/>In-memory / Redis Streams]
    end

    subgraph "Event Types"
        EP1[lead.created]
        EP2[lead.updated]
        EP3[lead.converted]
        EP4[stage.changed]
        EP5[contact.created]
        EP6[subscription.created]
        EP7[form.submitted]
        EP8[chat.converted]
    end

    MP --> EP1
    MP --> EP2
    MP --> EP3
    MP --> EP4
    MP --> EP5
    MP --> EP6
    MP --> EP7
    MP --> EP8

    subgraph "Event Consumers"
        subgraph "Automation Service"
            AC1[Workflow: stage_changed trigger]
            AC2[Workflow: lead.created trigger]
        end

        subgraph "Intelligence Service"
            IC1[Recompute lead score]
            IC2[Trigger enrichment lookup]
        end

        subgraph "Reporting Service"
            RC1[Update pipeline summary]
            RC2[Update conversion rates]
        end

        subgraph "Engagement Service"
            EC1[Create subscription]
            EC2[Update goal progress]
        end

        subgraph "Platform Service"
            PC1[Send webhook notification]
            PC2[Log audit event]
        end
    end

    EP1 --> IC2
    EP1 --> AC2
    EP2 --> IC1
    EP3 --> EC1
    EP4 --> AC1
    EP4 --> RC1
    EP5 --> AC2
    EP6 --> EC2
    EP7 --> IC2
    EP8 --> AC2
```

---

## 6. Change Data Capture (CDC) Pattern

```mermaid
sequenceDiagram
    participant App as CRM Application
    participant DB as PostgreSQL
    participant CDC as CDC Service<br/>(Platform Service)
    participant Ext as External Systems<br/>ERP/Accounting

    App->>DB: UPDATE leads SET stage_id = $1
    DB->>DB: Trigger fires → audit_log INSERT
    DB->>CDC: WAL (Write-Ahead Log) change detected

    CDC->>CDC: Parse WAL entry
    Note over CDC: Extract: table, row_id, before_state, after_state

    CDC->>CDC: Filter by integration rules
    Note over CDC: Check: entity, sync_direction

    alt CRM → External
        CDC->>Ext: POST webhook with before/after
        Note over Ext: ERP receives: lead.updated
        Ext->>CDC: 200 OK
    end

    alt External → CRM
        Ext->>CDC: POST webhook with external data
        Note over Ext: ERP sends: account.updated
        CDC->>DB: Upsert account based on external data
        CDC->>CDC: Log to audit_log
    end

    CDC->>CDC: Update integration.last_sync
    Note over CDC: Mark sync as complete
```

---

## 7. Batch Processing Flow (Lead Scoring)

```mermaid
sequenceDiagram
    participant Cron as System Cron<br/>recompute_scores (daily 02:00)
    participant Batch as Batch Processor<br/>Intelligence Service
    participant DB as PostgreSQL
    participant Scoring as Scoring Engine<br/>Bayesian PLS

    Cron->>Batch: Trigger batch scoring job
    Batch->>DB: SELECT leads WHERE automated_probability = false
    DB->>Batch: Return all leads

    Batch->>Batch: Chunk into batches of 1000
    Note over Batch: Process 1000 leads per chunk

    loop For each chunk
        Batch->>DB: SELECT scoring_frequencies FOR chunk
        DB->>Batch: Return frequency data
        
        Batch->>Scoring: Compute Bayesian PLS probability
        Note over Scoring: P(won|email,phone,title,...)
        Scoring->>Scoring: Get top contributing factors
        Scoring->>DB: UPDATE leads SET probability = $1
        Scoring->>DB: UPDATE lead_scores SET probability = $1
        
        Batch->>DB: UPDATE scoring_frequencies
        Note over Batch: Increment won_count/lost_count
        
        Batch->>Batch: Log chunk completion
    end

    Batch->>DB: UPDATE scoring_frequencies REBUILD
    Note over DB: Rebuild from historical won/lost data

    Batch->>DB: UPDATE intelligence SET last_run = NOW()
    Batch->>Cron: Job complete — X leads scored
```

---

## 8. Webhook Delivery Flow

```mermaid
sequenceDiagram
    participant App as CRM Application
    participant Webhook as Webhook Service<br/>Platform Service
    participant Retry as Retry Mechanism
    participant Ext as External System
    participant DB as PostgreSQL

    App->>Webhook: Publish event: lead.created
    Webhook->>DB: SELECT active webhooks WHERE event = 'lead.created'
    DB->>Webhook: Return matching webhooks

    loop For each webhook
        Webhook->>Webhook: Build payload
        Note over Webhook: {event, timestamp, data: {id, fields...}}
        Webhook->>Webhook: Sign with HMAC-SHA256 using secret
        
        Webhook->>Retry: POST to webhook URL
        Retry->>Ext: HTTP POST with payload + signature
        Ext->>Retry: Response status

        alt Success (2xx)
            Retry->>Webhook: 200 OK
            Webhook->>DB: INSERT webhook_delivery_log<br/>status=200
        end

        alt Failure (non-2xx)
            Retry->>Webhook: 500 Error
            Webhook->>Webhook: Exponential backoff retry
            Note over Webhook: 1s → 2s → 4s → 8s → ...
            
            loop Max 5 retries
                Retry->>Ext: Retry POST
                Ext->>Retry: Response
                
                alt Success
                    Retry->>Webhook: 200 OK
                    Webhook->>DB: INSERT webhook_delivery_log<br/>status=200
                end
                
                alt All retries failed
                    Retry->>Webhook: Exhausted retries
                    Webhook->>DB: INSERT webhook_delivery_log<br/>status=FAILED
                end
            end
        end
    end
```

---

## 9. Marketing Form Submission Flow

```mermaid
sequenceDiagram
    participant Browser as Web Form<br/>Public Endpoint
    participant GW as Gateway
    participant Marketing as Marketing Service
    participant Pipeline as Pipeline Service
    participant Contacts as Contacts Service
    participant Intelligence as Intelligence Service
    participant Automation as Automation Service

    Browser->>Marketing: POST /forms/{id} (no auth)
    Note over Marketing: Body: {name, email, phone, custom_fields}

    Marketing->>Marketing: Validate form schema
    Note over Marketing: Check required fields from capture_fields

    alt Invalid Submission
        Marketing->>Browser: 400 Bad Request
    end

    Marketing->>Contacts: POST /contacts (async)
    Note over Contacts: Create or deduplicate contact
    
    Marketing->>Pipeline: POST /leads (async)
    Note over Pipeline: Create lead from form data
    
    Pipeline->>Intelligence: POST /enrichment/lookup (async)
    Intelligence->>Pipeline: Return enriched data

    Pipeline->>Automation: Publish event: lead.created
    Automation->>Automation: Match workflow triggers

    Marketing->>Marketing: Track form submission
    Marketing->>Browser: 302 Redirect to redirect_url
```

---

## 10. Livechat → Lead Conversion Flow

```mermaid
sequenceDiagram
    participant Visitor as Visitor<br/>Browser Widget
    participant Chat as Chat Widget<br/>Livechat Service
    participant Agent as Agent<br/>Internal App
    participant GW as Gateway
    participant Pipeline as Pipeline Service
    participant Contacts as Contacts Service

    Visitor->>Chat: Open chat session
    Chat->>Chat: Create chat_session (status=WAITING)
    Chat->>Chat: Assign to available agent
    
    Agent->>Chat: Accept chat
    Chat->>Chat: Update session (status=ACTIVE)
    Chat->>Visitor: Route to agent

    loop Chat Messages
        Visitor->>Chat: POST /chats/{id}/message
        Chat->>Chat: Store message (sender=VISITOR)
        Chat->>Agent: Push message to agent UI
        Agent->>Chat: POST /chats/{id}/message
        Chat->>Chat: Store message (sender=AGENT)
        Chat->>Visitor: Push message to visitor UI
    end

    Visitor->>Chat: POST /chats/{id}/convert
    Note over Chat: Body: {create_contact, create_lead}

    Chat->>Contacts: POST /contacts (async)
    Contacts->>Chat: Return contact_id
    
    Chat->>Pipeline: POST /leads (async)
    Pipeline->>Chat: Return lead_id
    Note over Pipeline: Create lead from chat transcript

    Chat->>Chat: Update session (status=CLOSED)
    Chat->>Chat: Record satisfaction_rating
    Chat->>Automation: Publish event: chat.converted

    Chat->>GW: 200 OK + ConversionResult
    GW->>Visitor: Confirmation + lead/contact IDs
```

---

## 11. Report Generation Flow

```mermaid
sequenceDiagram
    participant Client as Client
    participant GW as Gateway
    participant Reporting as Reporting Service
    participant DB as PostgreSQL
    participant Aggregator as Aggregation Engine

    Client->>GW: GET /analytics/pipeline-summary
    Note over GW: Query params: period=monthly&team_id=uuid
    GW->>Reporting: Forward request

    Reporting->>Reporting: Parse request params
    Reporting->>DB: BEGIN TRANSACTION

    Reporting->>DB: SELECT leads WHERE stage IS NOT NULL
    DB->>Reporting: Return leads

    Reporting->>Aggregator: Group by stage
    Aggregator->>Aggregator: Calculate:
    Note over Aggregator: count, total_revenue, weighted_revenue, avg_days_in_stage

    Reporting->>DB: SELECT stage_history for time calculations
    DB->>Reporting: Return transition history

    Reporting->>Reporting: Calculate weighted_pipeline
    Note over Reporting: revenue × probability per stage

    Reporting->>Reporting: Apply filters (team, user, period)

    Reporting->>DB: COMMIT
    Reporting->>GW: 200 OK + PipelineSummary

    GW->>Client: JSON response with stages array

    alt Saved Report
        Client->>GW: POST /reports/schedule
        GW->>Reporting: Create scheduled report
        Reporting->>Reporting: Add to cron queue
        Note over Reporting: Daily 08:00 — run report + send digest
    end
```

---

## 12. Data Flow Summary by Service

```mermaid
graph TB
    subgraph "Inbound Data"
        Forms[Web Forms<br/>Marketing]
        Chats[Live Chat<br/>Livechat]
        API[API Requests<br/>All Services]
        Sync[External Sync<br/>Platform]
        Cron[Cron Jobs<br/>Automation]
    end

    subgraph "Core Ingestion"
        Pipeline[Pipeline Service]
        Contacts[Contacts Service]
        Accounts[Accounts Service]
    end

    subgraph "Processing"
        Scoring[Intelligence<br/>Scoring]
        Enrich[Enrichment<br/>APIs]
        Rules[Automation<br/>Workflow Engine]
        Analytics[Reporting<br/>Aggregation]
    end

    subgraph "Outbound Data"
        Webhooks[Webhooks<br/>Platform]
        Digests[Email Digests<br/>Reporting]
        CDC[CDC Sync<br/>Platform]
        Dashboard[Dashboard UI<br/>All Services]
    end

    Forms --> Pipeline
    Chats --> Pipeline
    API --> Pipeline
    API --> Contacts
    API --> Accounts
    Sync --> Accounts

    Pipeline --> Scoring
    Pipeline --> Enrich
    Pipeline --> Rules

    Contacts --> Analytics
    Accounts --> Analytics

    Scoring --> Dashboard
    Enrich --> Dashboard
    Rules --> Dashboard
    Analytics --> Dashboard

    Rules --> Webhooks
    Analytics --> Digests
    Accounts --> CDC
    Pipeline --> CDC
```

---

*This document defines all data flows. The standard CRUD flow applies to all services, while event-driven flows enable cross-service communication. Batch processing handles scoring and aggregation, while CDC enables real-time sync with external systems.*
