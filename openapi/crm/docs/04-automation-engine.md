# CRM Automation Engine Design

> **Version:** 1.0.0
> **Scope:** Workflow trigger-execution model, rule engine, cron scheduling, and action processing
> **Status:** Active design spec

---

## 1. Trigger-Action Execution Model

```mermaid
flowchart LR
    subgraph "Event Source"
        LeadCreate[Lead Created]
        StageChange[Stage Changed]
        FieldChange[Field Changed]
        TimeBased[Time-Based Trigger]
        Manual[Manual Trigger]
        WebhookIn[Webhook Received]
    end

    subgraph "Match Engine"
        Match[Match Workflow by trigger_type + entity]
        Filter[Filter by conditions]
    end

    subgraph "Rule Evaluation"
        R1[Rule 1: condition_field + operator]
        R2[Rule 2: condition_field + operator]
        R3[Rule 3: condition_field + operator]
    end

    subgraph "Action Execution"
        A1[SEND_EMAIL]
        A2[CREATE_TASK]
        A3[UPDATE_FIELD]
        A4[ASSIGN_LEAD]
        A5[CALL_WEBHOOK]
        A6[SEND_SMS]
    end

    subgraph "Logging"
        Log[Execution Log]
    end

    LeadCreate --> Match
    StageChange --> Match
    FieldChange --> Match
    TimeBased --> Match
    Manual --> Match
    WebhookIn --> Match

    Match --> Filter
    Filter --> R1
    R1 --> R2
    R2 --> R3

    R1 --> A1
    R1 --> A2
    R1 --> A3
    R1 --> A4
    R2 --> A5
    R3 --> A6

    A1 --> Log
    A2 --> Log
    A3 --> Log
    A4 --> Log
    A5 --> Log
    A6 --> Log
```

---

## 2. Workflow Engine Sequence Diagram

```mermaid
sequenceDiagram
    participant Event as Event Producer
    participant Engine as Workflow Engine<br/>Automation Service
    participant DB as PostgreSQL
    participant Action as Action Executor

    Event->>Engine: Publish event: stage_changed
    Engine->>Engine: Query workflows WHERE<br/>trigger_type = 'STAGE_CHANGE'<br/>AND entity = 'leads'

    loop For each matching workflow
        Engine->>DB: SELECT rules WHERE workflow_id = $1<br/>ORDER BY sequence ASC
        DB->>Engine: Return ordered rules
        
        Engine->>Engine: Evaluate Rule 1
        alt Condition Met
            Engine->>Action: Execute SEND_EMAIL action
            Action->>Action: Render template with merge fields
            Action->>DB: INSERT into email_queue
        else Condition Not Met
            Engine->>Engine: Skip to next rule
        end

        Engine->>Engine: Evaluate Rule 2
        alt Condition Met
            Engine->>Action: Execute UPDATE_FIELD action
            Action->>DB: UPDATE leads SET probability = $1
        else Condition Not Met
            Engine->>Engine: Skip to next rule
        end

        Engine->>Engine: Evaluate Rule 3
        alt Condition Met
            Engine->>Action: Execute CALL_WEBHOOK action
            Action->>DB: INSERT webhook_delivery_log
            Action->>External: POST webhook with HMAC
            External->>Action: 200 OK
        end
    end

    Engine->>DB: INSERT execution_log<br/>(status=SUCCESS/SKIPPED)<br/>(started_at, completed_at, errors)
    Engine->>DB: UPDATE workflows SET last_triggered = NOW(), run_count += 1

    Engine->>Event: Execution complete
```

---

## 3. Rule Evaluation Flow

```mermaid
flowchart TD
    subgraph "Input"
        Rule[Rule Definition<br/>condition_field, operator, value]
        Data[Entity Data<br/>Lead/Contact/Account fields]
    end

    subgraph "Operator Router"
        EQ{Operator?<br/>EQUALS}
        GT{GREATER_THAN}
        LT{LESS_THAN}
        CONTAINS{CONTAINS}
        BEGINS{BEGINS_WITH}
        ENDS{ENDS_WITH}
        EMPTY{IS_EMPTY}
    end

    EQ --> |Yes| Eval1[Compare: data.field == rule.value]
    GT --> |Yes| Eval2[Compare: data.field > rule.value]
    LT --> |Yes| Eval3[Compare: data.field < rule.value]
    CONTAINS --> |Yes| Eval4[Compare: data.field.contains rule.value]
    BEGINS --> |Yes| Eval5[Compare: data.field.startsWith rule.value]
    ENDS --> |Yes| Eval6[Compare: data.field.endsWith rule.value]
    EMPTY --> |Yes| Eval7[Compare: data.field IS NULL OR '']

    Eval1 --> Result{Condition Met?}
    Eval2 --> Result
    Eval3 --> Result
    Eval4 --> Result
    Eval5 --> Result
    Eval6 --> Result
    Eval7 --> Result

    Result --> |Yes| Action[Execute Action]
    Result --> |No| Skip[Skip to Next Rule]
```

---

## 4. Action Execution Model

```mermaid
graph TB
    subgraph "Action Types"
        A1[SEND_EMAIL]
        A2[CREATE_TASK]
        A3[UPDATE_FIELD]
        A4[ASSIGN_LEAD]
        A5[CALL_WEBHOOK]
        A6[SEND_SMS]
    end

    subgraph "SEND_EMAIL Pipeline"
        E1[Render Template<br/>with merge fields]
        E2[Validate Recipient]
        E3[Queue via SMTP]
        E4[Log to EmailComm]
    end

    subgraph "CREATE_TASK Pipeline"
        T1[Parse Task Config]
        T2[Create FollowUpTask]
        T3[Set due_date + priority]
        T4[Assign to user]
    end

    subgraph "UPDATE_FIELD Pipeline"
        U1[Parse Field Config]
        U2[Validate Field Type]
        U3[Update Entity]
        U4[Log to AuditLog]
    end

    subgraph "ASSIGN_LEAD Pipeline"
        AS1[Get Team Members]
        AS2[Calculate Capacity]
        AS3[Select Best Fit]
        AS4[Update lead.team_id + user_id]
    end

    subgraph "CALL_WEBHOOK Pipeline"
        W1[Build Payload]
        W2[Sign HMAC-SHA256]
        W3[HTTP POST]
        W4[Log Delivery]
    end

    A1 --> E1 --> E2 --> E3 --> E4
    A2 --> T1 --> T2 --> T3 --> T4
    A3 --> U1 --> U2 --> U3 --> U4
    A4 --> AS1 --> AS2 --> AS3 --> AS4
    A5 --> W1 --> W2 --> W3 --> W4
    A6 --> E3
```

---

## 5. Cron Job Scheduling

```mermaid
flowchart TB
    subgraph "Cron Scheduler"
        Scheduler[Task Scheduler<br/>every 5 minutes]
    end

    subgraph "Scheduled Tasks"
        R1[assign_leads<br/>Daily 09:00]
        R2[run_workflows<br/>Every 5 min]
        R3[send_digests<br/>Daily 08:00]
        R4[check_rotting<br/>Hourly]
        R5[recompute_scores<br/>Daily 02:00]
        R6[cleanup_old_data<br/>Weekly]
    end

    Scheduler --> R1
    Scheduler --> R2
    Scheduler --> R3
    Scheduler --> R4
    Scheduler --> R5
    Scheduler --> R6

    subgraph "Task Details"
        subgraph "assign_leads"
            A1[Get unassigned leads]
            A2[Calculate team capacity]
            A3[Assign to best-fit user]
            A4[Log assignment]
        end

        subgraph "run_workflows"
            W1[Get active workflows<br/>with time_based triggers]
            W2[Evaluate conditions]
            W3[Execute matching rules]
            W4[Log execution]
        end

        subgraph "send_digests"
            D1[Generate KPI digest]
            D2[Format as HTML]
            D3[Send via SMTP]
            D4[Log delivery]
        end

        subgraph "check_rotting"
            C1[Get leads in stage<br/>beyond rotting_threshold]
            C2[Mark as rotting]
            C3[Create reminder task]
            C4[Notify team]
        end

        subgraph "recompute_scores"
            S1[Batch all leads]
            S2[Fetch scoring_frequencies]
            S3[Compute PLS probability]
            S4[Update lead_scores]
            S5[Rebuild frequencies]
        end

        subgraph "cleanup_old_data"
            L1[Find old activities]
            L2[Archive to cold storage]
            L3[Delete from hot DB]
            L4[Log cleanup]
        end
    end

    R1 --> A1 --> A2 --> A3 --> A4
    R2 --> W1 --> W2 --> W3 --> W4
    R3 --> D1 --> D2 --> D3 --> D4
    R4 --> C1 --> C2 --> C3 --> C4
    R5 --> S1 --> S2 --> S3 --> S4 --> S5
    R6 --> L1 --> L2 --> L3 --> L4
```

---

## 6. Workflow Execution States

```mermaid
stateDiagram-v2
    [*] --> Active: Created + is_active = true
    [*] --> Inactive: Created + is_active = false

    Active --> Evaluating: Trigger fires
    Evaluating --> Success: All rules passed
    Evaluating --> Skipped: No rules matched
    Evaluating --> Failed: Rule execution error

    Success --> [*]: Log complete
    Skipped --> [*]: Log complete
    Failed --> [*]: Log error

    Active --> Inactive: Disabled
    Inactive --> Active: Enabled

    Success --> Evaluating: Next trigger fires
    Failed --> Evaluating: Next trigger fires
```

---

## 7. Workflow-Trigger-Action Relationship

```mermaid
graph TB
    subgraph "Workflow Definition"
        WF[Workflow]
    end

    subgraph "Triggers (1:1)"
        T1[Trigger Type]
        T2[Entity]
        T3[Field]
        T4[Schedule]
    end

    subgraph "Rules (1:N)"
        R1[Rule 1<br/>Condition + Action]
        R2[Rule 2<br/>Condition + Action]
        R3[Rule 3<br/>Condition + Action]
        R4[Rule N<br/>Condition + Action]
    end

    subgraph "Actions"
        A1[SEND_EMAIL]
        A2[CREATE_TASK]
        A3[UPDATE_FIELD]
        A4[ASSIGN_LEAD]
        A5[CALL_WEBHOOK]
        A6[SEND_SMS]
    end

    WF --> T1
    WF --> T2
    WF --> T3
    WF --> T4

    WF --> R1
    WF --> R2
    WF --> R3
    WF --> R4

    R1 --> A1
    R2 --> A2
    R3 --> A3
    R4 --> A4

    A1 --> WF
    A2 --> WF
    A3 --> WF
    A4 --> WF
    A5 --> WF
    A6 --> WF
```

---

## 8. Time-Based Trigger Evaluation

```mermaid
sequenceDiagram
    participant Cron as Cron Job<br/>run_workflows (5min)
    participant Engine as Workflow Engine
    participant DB as PostgreSQL
    participant Action as Action Executor

    Cron->>Engine: Tick — evaluate time-based triggers
    Engine->>DB: SELECT workflows WHERE<br/>trigger_type = 'TIME_BASED'<br/>AND last_triggered < NOW() - schedule
    DB->>Engine: Return matching workflows

    loop For each workflow
        Engine->>Engine: Evaluate conditions
        Note over Engine: Check: condition_field, operator, value
        
        alt Conditions Met
            Engine->>Action: Execute action
            Action->>DB: INSERT action execution log
            
            Engine->>DB: UPDATE workflows SET<br/>last_triggered = NOW()
        else Conditions Not Met
            Engine->>Engine: Skip — conditions not met
        end
    end

    Engine->>DB: UPDATE triggers SET last_fired = NOW()
    Cron->>Engine: Next tick in 5 minutes
```

---

## 9. Manual Trigger Execution

```mermaid
sequenceDiagram
    participant User as Sales Rep
    participant GW as Gateway
    participant Engine as Workflow Engine
    participant DB as PostgreSQL
    participant Action as Action Executor

    User->>GW: POST /workflows/{id}/run
    Note over GW: Manual trigger for testing

    GW->>Engine: Forward request

    Engine->>DB: SELECT workflow WHERE id = $1
    DB->>Engine: Workflow definition

    Engine->>Engine: Validate workflow exists + is_active
    Engine->>Engine: Evaluate all rules

    loop For each rule in sequence
        Engine->>Engine: Evaluate condition
        alt Condition Met
            Engine->>Action: Execute action
            Action->>DB: Log action execution
        end
    end

    Engine->>DB: INSERT execution_log<br/>(status=SUCCESS, triggered_by=manual)
    Engine->>DB: UPDATE workflows SET<br/>last_triggered = NOW(), run_count += 1

    Engine->>GW: 200 OK + ExecutionResult
    GW->>User: {workflow_id, rules_executed, actions_ran, errors}
```

---

## 10. Lead Assignment Algorithm

```mermaid
flowchart TB
    subgraph "Assignment Input"
        Leads[Unassigned Leads]
        Teams[Active Teams]
    end

    subgraph "Capacity Calculation"
        Calc[Calculate Team Capacity<br/>assignment_max × days_in_period]
        Used[Calculate Current Usage<br/>lead_month_count]
        Remaining[Calculate Remaining<br/>capacity - usage]
    end

    subgraph "Selection Algorithm"
        Sort[Sort Teams by remaining capacity DESC]
        Select[Select Top Team]
        Member[Select Team Member<br/>with lowest assignment_max]
    end

    subgraph "Assignment"
        Assign[Update lead.team_id + user_id]
        Update[Update team.lead_month_count]
        Log[Log assignment in audit_log]
    end

    Leads --> Calc
    Teams --> Calc
    Calc --> Used
    Used --> Remaining
    Remaining --> Sort
    Sort --> Select
    Select --> Member
    Member --> Assign
    Assign --> Update
    Update --> Log
```

---

## 11. Action Execution Pipeline Detail

```mermaid
graph TB
    subgraph "Common Pipeline"
        Start[Action Triggered]
        Parse[Parse action_config JSON]
        Validate[Validate action_type]
        Execute[Execute Action]
        Log[Log to execution_log]
        Error[Handle Errors]
    end

    subgraph "SEND_EMAIL"
        E1[Load Email Template]
        E2[Render with Merge Fields]
        E3[Validate Recipient Address]
        E4[Send via SMTP Queue]
        E5[Update EmailComm record]
    end

    subgraph "UPDATE_FIELD"
        U1[Parse field + value]
        U2[Validate field_type]
        U3[Update Entity Row]
        U4[Write to AuditLog]
    end

    subgraph "CALL_WEBHOOK"
        W1[Build Payload]
        W2[Sign HMAC-SHA256]
        W3[HTTP POST to URL]
        W4[Retry on Failure]
        W5[Log Delivery Result]
    end

    Start --> Parse --> Validate --> Execute
    Execute --> Log
    Execute --> Error

    Parse --> E1 --> E2 --> E3 --> E4 --> E5
    Parse --> U1 --> U2 --> U3 --> U4
    Parse --> W1 --> W2 --> W3 --> W4 --> W5
```

---

## 12. Automation Service Entity Map

```mermaid
erDiagram
    WORKFLOW ||--o{ RULE : "has_many"
    WORKFLOW ||--|| TRIGGER : "has_one"
    WORKFLOW ||--o{ EXECUTION_LOG : "generates"
    RULE ||--|| WORKFLOW : "belongs_to"
    TRIGGER ||--|| WORKFLOW : "belongs_to"
    EXECUTION_LOG ||--|| WORKFLOW : "belongs_to"

    WORKFLOW {
        uuid id PK
        string name
        boolean is_active
        enum trigger_type "STAGE_CHANGE|FIELD_CHANGE|TIME_BASED|WEBHOOK|MANUAL"
        json trigger_config
        enum action_type "SEND_EMAIL|CREATE_TASK|UPDATE_FIELD|ASSIGN_LEAD|CALL_WEBHOOK"
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

*This document defines the complete automation engine. Workflows are defined by trigger type + entity, evaluated by rules in sequence order, and execute actions (SEND_EMAIL, CREATE_TASK, UPDATE_FIELD, ASSIGN_LEAD, CALL_WEBHOOK). Cron jobs handle time-based triggers and periodic maintenance.*
