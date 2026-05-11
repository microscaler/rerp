# CRM Entity Relationships

> **Version:** 1.0.0
> **Scope:** Complete entity relationship diagrams for all 11 CRM services
> **Status:** Active design spec

---

## 1. Core Entity Relationship Diagram

```mermaid
erDiagram
    LEAD ||--o{ STAGE : "belongs_to"
    LEAD ||--o{ STAGE_HISTORY : "transitions"
    LEAD ||--o| CONTACT : "partner_id"
    LEAD ||--o| ACCOUNT : "company_id"
    LEAD ||--o| TEAM : "team_id"
    LEAD ||--o| USER : "user_id"
    LEAD }|--|{ LEAD : "duplicate_lead_ids"
    LEAD ||--|| LOST_REASON : "lost_reason_id"
    LEAD {
        uuid id PK
        string name
        enum type
        string email_normalized
        string phone_sanitized
        enum priority
        boolean active
        boolean is_blacklisted
        int color
        uuid stage_id FK
        int probability
        float expected_revenue
        uuid partner_id FK
        uuid company_id FK
        uuid user_id FK
        uuid team_id FK
        uuid lost_reason_id FK
    }

    CONTACT ||--o| ACCOUNT : "company_id"
    CONTACT ||--o| LEAD : "partner_id"
    CONTACT ||--o{ USER : "user_ids (M:N)"
    CONTACT {
        uuid id PK
        string name
        string email
        string phone
        boolean is_company
        uuid parent_id FK
        string function
        string department
    }

    ACCOUNT ||--o{ CONTACT : "has_many"
    ACCOUNT ||--o{ ACCOUNT : "parent/child_tree"
    ACCOUNT ||--o{ LEAD : "has_many"
    ACCOUNT {
        uuid id PK
        string name
        string email
        string phone
        string website
        uuid industry_id FK
        int company_size
        int employees
        float annual_revenue
        uuid parent_id FK
    }

    STAGE {
        uuid id PK
        string name
        int sequence
        int probability
        boolean is_won
        boolean is_lost
        int rotting_threshold_days
        uuid team_id FK
    }

    STAGE_HISTORY {
        uuid id PK
        uuid lead_id FK
        uuid old_stage_id FK
        uuid new_stage_id FK
        int probability_before
        int probability_after
        uuid user_id FK
        datetime transition_date
        int days_in_old_stage
    }

    LOST_REASON {
        uuid id PK
        string name
        string category
        int sequence
        boolean active
    }

    TEAM {
        uuid id PK
        string name
        string description
        boolean use_leads
        boolean use_opportunities
        boolean assignment_enabled
    }

    USER {
        uuid id PK
        string email
        string name
        uuid team_id FK
    }
```

---

## 2. Pipeline Service Entities

```mermaid
erDiagram
    LEAD ||--|| STAGE : "assigned"
    LEAD ||--o{ STAGE_HISTORY : "tracks_changes"
    LEAD }|--|| ACCOUNT : "linked_to"
    LEAD }|--|| CONTACT : "linked_to"
    LEAD ||--|| LOST_REASON : "assigns_reason"
    LEAD ||--|| TEAM : "owned_by"
    LEAD ||--|| USER : "assigned_to"
    LEAD ||--|| UTMMEDIUM : "source_medium"
    LEAD ||--|| UTM_SOURCE : "source_ref"
    LEAD ||--|| UTMCAMPAIGN : "source_campaign"

    LEAD {
        uuid id PK
        string name "Lead title"
        enum type "LEAD|OPPORTUNITY"
        string email "Normalized"
        string phone "Sanitized"
        string company_name
        string website
        string description "Internal notes"
        enum priority "LOW|NORMAL|HIGH|URGENT"
        boolean active "Soft delete"
        uuid stage_id FK
        uuid partner_id FK
        uuid company_id FK
        uuid user_id FK
        uuid team_id FK
        uuid campaign_id FK
        uuid medium_id FK
        uuid source_id FK
        int probability
        float expected_revenue
        float prorated_revenue
    }

    STAGE {
        uuid id PK
        string name
        int sequence
        int probability
        boolean is_won
        boolean is_lost
        int rotting_threshold_days
        text requirements
        int fold
        int color
        uuid team_id FK
    }

    STAGE_HISTORY {
        uuid id PK
        uuid lead_id FK
        uuid old_stage_id FK
        uuid new_stage_id FK
        int probability_before
        int probability_after
        uuid user_id FK
        datetime transition_date
        int days_in_old_stage
    }

    LOST_REASON {
        uuid id PK
        string name
        string category
        int sequence
        boolean active
    }

    UTMCAMPAIGN {
        uuid id PK
        string name
        uuid medium_id FK
        int sequence
    }

    UTMMEDIUM {
        uuid id PK
        string name
        uuid source_id FK
        int sequence
    }

    UTM_SOURCE {
        uuid id PK
        string name
        int sequence
    }
```

---

## 3. Contacts & Accounts Service Entities

```mermaid
erDiagram
    CONTACT ||--|| CONTACT : "parent/child_tree"
    CONTACT ||--|| ACCOUNT : "company_link"
    CONTACT ||--o{ LEAD : "partner_link"
    CONTACT ||--o{ TASK : "assigned"
    CONTACT ||--o{ ACTIVITY : "belongs_to"
    CONTACT ||--o{ FOLLOW_UP : "has_followups"

    ACCOUNT ||--o{ ACCOUNT : "parent/child_tree"
    ACCOUNT ||--o{ CONTACT : "has_contacts"
    ACCOUNT ||--o{ LEAD : "has_leads"
    ACCOUNT ||--o{ INDUSTRY_CLASS : "belongs_to"

    INDUSTRY_CLASS {
        uuid id PK
        string name
        string code
        int parent_id FK
    }

    ACCOUNT {
        uuid id PK
        string name
        string email
        string phone
        string website
        uuid industry_id FK
        int company_size
        int employees
        float annual_revenue
        uuid parent_id FK
        uuid child_ids FK[]
    }

    CONTACT {
        uuid id PK
        string name
        string email
        string phone
        string mobile
        string title "Salutation"
        string function "Job title"
        string department
        boolean is_company
        uuid company_id FK
        uuid parent_id FK
        uuid[] user_ids
        uuid partner_id FK
    }

    ACTIVITY {
        uuid id PK
        enum activity_type "EMAIL|CALL|MEETING|NOTE|TASK|FOLLOW_UP"
        string summary
        text description
        uuid related_id FK
        enum related_type "LEAD|CONTACT|ACCOUNT"
        uuid user_id FK
        datetime scheduled_date
        datetime completed_date
    }

    FOLLOW_UP {
        uuid id PK
        string title
        text description
        date due_date
        enum priority
        enum status "TODO|IN_PROGRESS|DONE|CANCELLED"
        uuid assigned_to FK
        uuid related_id FK
        datetime reminder_date
    }
```

---

## 4. Automation Service Entities

```mermaid
erDiagram
    WORKFLOW ||--o{ RULE : "contains"
    WORKFLOW ||--|| TRIGGER : "has"
    WORKFLOW ||--o{ EXECUTION_LOG : "generates"
    TRIGGER ||--|| WORKFLOW : "belongs_to"
    RULE ||--|| WORKFLOW : "belongs_to"
    EXECUTION_LOG ||--|| WORKFLOW : "belongs_to"

    WORKFLOW {
        uuid id PK
        string name
        boolean is_active
        enum trigger_type "STAGE_CHANGE|FIELD_CHANGE|TIME_BASED|WEBHOOK|MANUAL"
        json trigger_config
        enum action_type
        json action_config
        json conditions
        int sequence
        datetime last_triggered
        int run_count
    }

    RULE {
        uuid id PK
        uuid workflow_id FK
        string condition_field
        enum condition_operator "EQUALS|GREATER_THAN|CONTAINS|BEGINS_WITH|ENDS_WITH"
        enum condition_value_type "STRING|NUMBER|BOOLEAN|DATE"
        string condition_value
        enum action_type
        json action_config
        int sequence
        boolean is_active
    }

    TRIGGER {
        uuid id PK
        uuid workflow_id FK
        enum type "STAGE_CHANGE|FIELD_CHANGE|SCHEDULED|WEBHOOK"
        string entity "leads|contacts|accounts"
        string field
        json schedule "Cron expression"
        boolean enabled
        datetime last_fired
    }

    EXECUTION_LOG {
        uuid id PK
        uuid workflow_id FK
        enum status "SUCCESS|FAILED|SKIPPED"
        json executed_rules
        json errors
        datetime started_at
        datetime completed_at
        int actions_ran
    }
```

---

## 5. Intelligence Service Entities

```mermaid
erDiagram
    LEAD_SCORE ||--|| LEAD : "computed_on"
    SCORING_FREQUENCY ||--|| TEAM : "grouped_by"
    SCORING_FREQUENCY ||--|| LEAD : "applied_to"
    ENRICHMENT_REQUEST ||--|| LEAD : "initiated_for"
    ENRICHMENT_RESULT ||--|| ENRICHMENT_REQUEST : "for_request"
    DATA_VERIFICATION ||--|| LEAD : "for_lead"
    LEAD_SCORE ||--|| SCORING_FREQUENCY : "uses"

    LEAD_SCORE {
        uuid id PK
        uuid lead_id FK
        float probability
        boolean is_automated_probability
        json top_factors
        datetime computation_date
        int total_records
    }

    SCORING_FREQUENCY {
        uuid id PK
        string variable "email|phone|title|industry..."
        string value
        int won_count
        int lost_count
        uuid team_id FK
        int total
        float probability "won_count/total"
    }

    ENRICHMENT_REQUEST {
        uuid id PK
        uuid lead_id FK
        string email
        string company_name
        string website
        enum status "PENDING|SUCCESS|FAILED"
        enum source "CLEARBIT|HUNTER|CUSTOM"
        json request_data
        datetime created_at
        datetime completed_at
    }

    ENRICHMENT_RESULT {
        uuid id PK
        uuid enrichment_request_id FK
        string industry
        string size
        string website
        json tech_stack
        float funding_total
        int employee_count
        string person_name
        string person_title
        enum person_role
        enum person_seniority
        string person_email
    }

    DATA_VERIFICATION {
        uuid id PK
        string email
        string phone
        boolean email_valid
        boolean email_disposable
        boolean email_role_based
        boolean mx_records_exist
        boolean phone_valid
        string phone_carrier
        enum phone_type "MOBILE|LANDLINE|VOIP"
        datetime verified_at
    }
```

---

## 6. Engagement Service Entities

```mermaid
erDiagram
    SUBSCRIPTION ||--|| LEAD : "for_opportunity"
    SUBSCRIPTION ||--|| CONTACT : "customer"
    SUBSCRIPTION ||--o{ RENEWAL_ALERT : "has"
    SUBSCRIPTION ||--|| GOAL : "drives"
    RENEWAL_ALERT ||--|| GOAL : "triggers"
    GOAL ||--|| USER : "assigned_to"
    GOAL ||--|| TEAM : "team_goal"
    USER ||--o{ USER_BADGE : "has"
    BADGE ||--o{ USER_BADGE : "awarded_to"

    SUBSCRIPTION {
        uuid id PK
        uuid opportunity_id FK
        uuid customer_id FK
        enum status "ACTIVE|EXPIRED|CANCELLED|TRIAL|PAST_DUE"
        uuid plan_id FK
        date start_date
        date end_date
        date renewal_date
        boolean renewal_automatic
        float value
        float monthly_value "MRR"
        enum billing_cycle "MONTHLY|QUARTERLY|ANNUAL"
        float churn_risk_score
    }

    RENEWAL_ALERT {
        uuid id PK
        uuid subscription_id FK
        enum alert_type "RENEWAL_REMINDER|PRICE_CHANGE|CONTRACT_END|CANCELLATION_WARNING"
        date alert_date
        enum status "PENDING|NOTIFIED|RESOLVED"
        uuid assigned_to FK
        text message
    }

    GOAL {
        uuid id PK
        string name
        enum type "REVENUE|DEALS|CALLS|MEETINGS|LEADS"
        float target_value
        date start_date
        date end_date
        uuid user_id FK
        uuid team_id FK
        float completed_value
        float completion_percentage
        enum status "ACTIVE|COMPLETED|EXPIRED"
    }

    BADGE {
        uuid id PK
        string name
        text description
        string icon
        json criteria
        boolean is_active
    }

    USER_BADGE {
        uuid id PK
        uuid user_id FK
        uuid badge_id FK
        datetime earned_at
    }
```

---

## 7. Livechat Service Entities

```mermaid
erDiagram
    CHAT_SESSION ||--o{ CHAT_MESSAGE : "has"
    CHAT_SESSION ||--|| AGENT : "assigned_to"
    CHAT_SESSION ||--|| CONTACT : "contact"
    CHAT_SESSION ||--|| LEAD : "converted_to"
    AGENT ||--o{ CHAT_SESSION : "handles"
    CHAT_SESSION ||--|| VISITOR : "initiated_by"

    CHAT_SESSION {
        uuid id PK
        uuid visitor_id FK
        uuid contact_id FK
        uuid lead_id FK
        uuid agent_id FK
        enum status "WAITING|ACTIVE|CLOSED|TRANSFERRED"
        datetime started_at
        datetime ended_at
        json transcript
        float satisfaction_rating
    }

    CHAT_MESSAGE {
        uuid id PK
        uuid session_id FK
        enum sender_type "AGENT|VISITOR|BOT"
        text message
        boolean is_system
        datetime sent_at
        datetime read_at
    }

    AGENT {
        uuid id PK
        uuid user_id FK
        boolean is_online
        int active_sessions
        int max_concurrent
        int total_chats_today
        float avg_response_time_seconds
        float satisfaction_avg
        int queue_position
        json skills
    }

    VISITOR {
        uuid id PK
        string session_id
        string user_agent
        string ip_address
        string country
        string city
        string referrer
        string url_visited
        datetime first_seen
        datetime last_seen
        int pages_viewed
        boolean converted_to_lead
        uuid lead_id FK
    }
```

---

## 8. Marketing Service Entities

```mermaid
erDiagram
    UTM_CAMPAIGN ||--o{ LEAD : "tracks"
    UTM_CAMPAIGN ||--|| UTM_MEDIUM : "uses"
    UTM_CAMPAIGN ||--|| UTM_SOURCE : "uses"
    UTM_MEDIUM ||--o{ UTM_SOURCE : "belongs_to"
    WEB_FORM ||--o{ FORM_SUBMISSION : "captures"
    FORM_SUBMISSION ||--|| LEAD : "creates"
    FORM_SUBMISSION ||--|| CONTACT : "creates"
    FORM_SUBMISSION ||--|| UTM_CAMPAIGN : "tracked_by"
    EVENT_REGISTRATION ||--|| LEAD : "linked"
    SURVEY_RESPONSE ||--|| CONTACT : "linked"

    UTM_CAMPAIGN {
        uuid id PK
        string name
        uuid medium_id FK
        uuid source_id FK
        int sequence
        boolean active
    }

    UTM_MEDIUM {
        uuid id PK
        string name
        uuid source_id FK
        int sequence
    }

    UTM_SOURCE {
        uuid id PK
        string name
        int sequence
    }

    WEB_FORM {
        uuid id PK
        string name
        json capture_fields "JSON schema"
        string redirect_url
        uuid team_id FK
        uuid user_id FK
        boolean consent_required
        string api_endpoint
        string embed_code
    }

    FORM_SUBMISSION {
        uuid id PK
        uuid form_id FK
        json submission_data
        uuid lead_id FK
        uuid contact_id FK
        datetime submitted_at
        string ip_address
        json consent
    }

    EVENT_REGISTRATION {
        uuid id PK
        string event_name
        uuid lead_id FK
        uuid contact_id FK
        datetime registered_at
        bool attended
    }

    SURVEY_RESPONSE {
        uuid id PK
        string survey_id
        uuid contact_id FK
        json responses
        datetime submitted_at
    }
```

---

## 9. Platform Service Entities

```mermaid
erDiagram
    CUSTOM_FIELD ||--o{ LEAD : "augments"
    CUSTOM_FIELD ||--o{ CONTACT : "augments"
    CUSTOM_FIELD ||--o{ ACCOUNT : "augments"
    WEBHOOK ||--o{ WEBHOOK_DELIVERY_LOG : "records"
    WEBHOOK_DELIVERY_LOG ||--|| WEBHOOK : "for"
    AUDIT_LOG ||--|| LEAD : "logs"
    AUDIT_LOG ||--|| CONTACT : "logs"
    AUDIT_LOG ||--|| ACCOUNT : "logs"
    API_KEY ||--o{ AUDIT_LOG : "used_by"
    INTEGRATION ||--o{ AUDIT_LOG : "triggers"

    CUSTOM_FIELD {
        uuid id PK
        string entity "leads|contacts|accounts"
        string name
        string label
        enum field_type "STRING|INTEGER|FLOAT|BOOLEAN|DATE|EMAIL|PHONE|TEXT|HTML|SELECT| MANY2ONE|MANY2MANY"
        json options
        boolean required
        boolean visible
        boolean searchable
        json default_value
    }

    WEBHOOK {
        uuid id PK
        string name
        string url
        json events "lead.create|lead.update..."
        string secret "HMAC key"
        boolean is_active
        json headers
        int retry_count
        datetime last_triggered
        int last_status
    }

    WEBHOOK_DELIVERY_LOG {
        uuid id PK
        uuid webhook_id FK
        string event
        int response_status
        json response_body
        datetime delivered_at
        int retry_count
    }

    AUDIT_LOG {
        uuid id PK
        string entity "leads|contacts|accounts"
        uuid entity_id FK
        enum action "CREATE|UPDATE|DELETE|READ|EXPORT|IMPORT"
        json changes "before/after"
        uuid user_id FK
        string ip_address
        string user_agent
        datetime created_at
    }

    API_KEY {
        uuid id PK
        string name
        string key_hash
        json scopes
        int rate_limit
        json ip_whitelist
        boolean is_active
        datetime last_used
        datetime expires_at
    }

    INTEGRATION {
        uuid id PK
        string name
        enum type "ERP|ACCOUNTING|MARKETING|SUPPORT|CUSTOM"
        json config
        boolean is_active
        enum sync_direction "CRM_TO_EXTERNAL|EXTERNAL_TO_CRM|BIDIRECTIONAL"
        datetime last_sync
        enum sync_status "PENDING|SUCCESS|FAILED"
    }
```

---

## 10. Reporting Service Entities

```mermaid
erDiagram
    PIPELINE_SUMMARY ||--|| STAGE : "aggregates"
    PIPELINE_SUMMARY ||--o{ LEAD : "counts"
    CONVERSION_RATE ||--|| LEAD : "counts"
    REP_PERFORMANCE ||--|| USER : "per_user"
    TEAM_PERFORMANCE ||--|| TEAM : "per_team"
    WIN_LOSS_ANALYSIS ||--|| LEAD : "analyzes"
    TIME_TO_CLOSE ||--|| STAGE_HISTORY : "calculates"
    FORECAST ||--|| LEAD : "predicts"
    SAVED_REPORT ||--|| USER : "owned"
    REPORT_SCHEDULE ||--|| SAVED_REPORT : "runs"
    REPORT_SCHEDULE ||--o{ USER : "sends_to"

    PIPELINE_SUMMARY {
        uuid id PK
        json stages "stage_id, count, total_revenue, weighted_revenue, avg_days"
        int total_leads
        float total_revenue
        float weighted_revenue
        datetime calculated_at
    }

    CONVERSION_RATE {
        uuid id PK
        date period_start
        date period_end
        int leads_created
        int opportunities_created
        int opportunities_won
        int opportunities_lost
        float lead_to_opp_rate
        float opp_to_won_rate
        float overall_conversion_rate
        float total_won_revenue
    }

    REP_PERFORMANCE {
        uuid id PK
        uuid user_id FK
        uuid team_id FK
        string user_name
        string period
        int leads_created
        int opportunities_active
        int opportunities_won
        int opportunities_lost
        float revenue_expected
        float revenue_won
        float weighted_pipeline
        float avg_deal_size
        int avg_sales_cycle_days
        float quota_attainment
        int activity_count
    }

    FORECAST {
        uuid id PK
        string period "monthly|quarterly|annual"
        float weighted_pipeline
        float expected_close_revenue
        float confidence
        float actual_close_revenue
        float variance
        date forecast_date
    }
```

---

## 11. Unified Entity Relationship Summary

```mermaid
graph TB
    subgraph "Core Domain"
        Lead[LEAD]
        Contact[CONTACT]
        Account[ACCOUNT]
        Stage[STAGE]
        LostReason[LOST_REASON]
    end

    subgraph "Organization"
        Team[TEAM]
        User[USER]
        TeamMember[TEAM_MEMBER]
    end

    subgraph "Automation"
        Workflow[WORKFLOW]
        Rule[RULE]
        Trigger[TRIGGER]
        ExecutionLog[EXECUTION_LOG]
    end

    subgraph "Intelligence"
        LeadScore[LEAD_SCORE]
        ScoringFrequency[SCORING_FREQUENCY]
        EnrichmentRequest[ENRICHMENT_REQUEST]
        EnrichmentResult[ENRICHMENT_RESULT]
        DataVerification[DATA_VERIFICATION]
    end

    subgraph "Engagement"
        Subscription[SUBSCRIPTION]
        RenewalAlert[RENEWAL_ALERT]
        Goal[GOAL]
        Badge[BADGE]
        UserBadge[USER_BADGE]
    end

    subgraph "Communication"
        ChatSession[CHAT_SESSION]
        ChatMessage[CHAT_MESSAGE]
        Agent[AGENT]
        Visitor[VISITOR]
    end

    subgraph "Marketing"
        UTMCampaign[UTM_CAMPAIGN]
        UTMMedium[UTM_MEDIUM]
        UTMSource[UTM_SOURCE]
        WebForm[WEB_FORM]
        FormSubmission[FORM_SUBMISSION]
        EventRegistration[EVENT_REGISTRATION]
        SurveyResponse[SURVEY_RESPONSE]
    end

    subgraph "Analytics"
        PipelineSummary[PIPELINE_SUMMARY]
        ConversionRate[CONVERSION_RATE]
        RepPerformance[REP_PERFORMANCE]
        WinLossAnalysis[WIN_LOSS_ANALYSIS]
        Forecast[FORECAST]
        TimeToClose[TIME_TO_CLOSE]
        SavedReport[SAVED_REPORT]
        ReportSchedule[REPORT_SCHEDULE]
    end

    subgraph "Platform"
        CustomField[CUSTOM_FIELD]
        Webhook[WEBHOOK]
        WebhookDeliveryLog[WEBHOOK_DELIVERY_LOG]
        AuditLog[AUDIT_LOG]
        APIKey[API_KEY]
        Integration[INTEGRATION]
    end

    Lead --> Contact
    Lead --> Account
    Lead --> Stage
    Lead --> Team
    Lead --> User
    Lead --> LeadScore
    Lead --> Subscription
    Lead --> UTMCampaign
    Lead --> PipelineSummary
    Lead --> RepPerformance
    Lead --> WinLossAnalysis
    Lead --> CustomField
    Lead --> AuditLog

    Contact --> Account
    Contact --> LeadScore
    Contact --> Subscription
    Contact --> CustomField
    Contact --> AuditLog

    Account --> LeadScore
    Account --> CustomField
    Account --> AuditLog

    Stage --> PipelineSummary
    Stage --> TimeToClose

    User --> TeamMember
    User --> Team
    User --> RepPerformance
    User --> Goal
    User --> UserBadge
    User --> APIKey

    Workflow --> Rule
    Workflow --> Trigger
    Workflow --> ExecutionLog

    LeadScore --> ScoringFrequency

    Subscription --> RenewalAlert
    Subscription --> Goal

    ChatSession --> ChatMessage
    ChatSession --> Agent
    ChatSession --> Visitor

    UTMCampaign --> UTMMedium
    UTMCampaign --> UTMSource
    WebForm --> FormSubmission
    FormSubmission --> Lead

    PipelineSummary --> Stage
    RepPerformance --> User
    Forecast --> Lead

    Webhook --> WebhookDeliveryLog
    APIKey --> AuditLog
    Integration --> AuditLog
```

---

*This document defines all entity relationships. The core lead entity is the central hub connecting all 11 services. Each service owns its entities and exposes them via its OpenAPI spec.*
