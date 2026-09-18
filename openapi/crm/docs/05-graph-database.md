# CRM Entity Graph & Hierarchy Design

> **Version:** 1.0.0
> **Scope:** Hierarchical entity trees (accounts, contacts, pipelines), relationship graphs, and self-referential patterns
> **Status:** Active design spec

---

## 1. Unified Lead-Contact-Account Relationship Graph

```mermaid
graph TB
    subgraph "Lead Entity (Central Hub)"
        Lead[LEAD<br/>uuid + email + phone]
        LeadStage[Stage]
        LeadProb[Probability]
        LeadRev[Revenue]
    end

    subgraph "Contact Linkage"
        Contact[CONTACT<br/>uuid + email]
        ContactTree[Contact Tree<br/>parent/child]
    end

    subgraph "Account Linkage"
        Account[ACCOUNT<br/>uuid + name]
        AccountTree[Account Tree<br/>parent/child]
    end

    subgraph "Organizational Linkage"
        User[USER]
        Team[TEAM]
    end

    subgraph "Marketing Linkage"
        Campaign[UTM_CAMPAIGN]
        Medium[UTM_MEDIUM]
        Source[UTM_SOURCE]
    end

    subgraph "Lifecycle Tracking"
        StageHistory[STAGE_HISTORY<br/>audit trail]
        LostReason[LOST_REASON]
        Duplicate[self-ref: duplicates]
    end

    Lead --> LeadStage
    Lead --> LeadProb
    Lead --> LeadRev
    Lead --> Contact
    Lead --> Account
    Lead --> User
    Lead --> Team
    Lead --> Campaign
    Lead --> StageHistory
    Lead --> LostReason
    Lead --> Duplicate

    Contact --> ContactTree
    Contact --> Account
    Account --> AccountTree
```

---

## 2. Account Hierarchy Tree

```mermaid
graph TB
    subgraph "Parent-Child Tree Structure"
        Parent[Acme Corp<br/>parent_id: NULL<br/>level: 0]
        
        subgraph "Level 1: Subsidiaries"
            Sub1[Acme EU<br/>parent_id: Acme Corp]
            Sub2[Acme APAC<br/>parent_id: Acme Corp]
        end

        subgraph "Level 2: Branches"
            Branch1[Acme Berlin<br/>parent_id: Acme EU]
            Branch2[Acme Tokyo<br/>parent_id: Acme APAC]
        end
    end

    subgraph "Related Entities"
        ContactA[Contact: CEO]
        ContactB[Contact: CFO]
        LeadA[Lead: Berlin Office]
        LeadB[Lead: Tokyo Office]
    end

    Parent --> Sub1
    Parent --> Sub2
    Sub1 --> Branch1
    Sub2 --> Branch2

    Parent -.-> ContactA
    Sub1 -.-> ContactB
    Sub2 -.-> LeadA
    Branch1 -.-> LeadB
```

---

## 3. Contact Tree Hierarchy

```mermaid
graph TB
    subgraph "Company Contact Tree"
        Main[John Doe<br/>CEO, Acme Corp<br/>is_company: true]
        
        subgraph "Subsidiary Contacts"
            Manager1[Sarah Smith<br/>Director, Acme EU]
            Manager2[Li Wei<br/>Manager, Acme APAC]
        end

        subgraph "Individual Contacts"
            Contact1[Jane Smith<br/>CTO, Acme Corp]
            Contact2[Bob Wilson<br/>VP Sales, Acme EU]
        end
    end

    subgraph "Linked Opportunities"
        Opp1[Opportunity: EU Expansion]
        Opp2[Opportunity: APAC Growth]
    end

    Main --> Manager1
    Main --> Manager2
    Manager1 --> Contact1
    Manager2 --> Contact2

    Main -.-> Opp1
    Manager1 -.-> Opp1
    Manager2 -.-> Opp2
```

---

## 4. Pipeline Stage Graph (Sequential Flow)

```mermaid
graph LR
    subgraph "Pipeline Flow"
        Prospecting[Prospecting<br/>seq: 1<br/>prob: 10%]
        Qualified[Qualified<br/>seq: 2<br/>prob: 25%]
        Proposal[Proposal<br/>seq: 3<br/>prob: 50%]
        Negotiation[Negotiation<br/>seq: 4<br/>prob: 75%]
        Won[Won<br/>seq: 5<br/>prob: 100%<br/>is_won: true]
        Lost[Lost<br/>seq: -1<br/>prob: 0%<br/>is_lost: true]
    end

    subgraph "Stage Transitions"
        T1[Transition: seq 1→2]
        T2[Transition: seq 2→3]
        T3[Transition: seq 3→4]
        T4[Transition: seq 4→5]
        TL[Transition: any→lost]
    end

    subgraph "Rotation Detection"
        Stale[Stale Lead<br/>rotting_threshold:<br/>30 days in stage]
    end

    Prospecting --> T1 --> Qualified
    Qualified --> T2 --> Proposal
    Proposal --> T3 --> Negotiation
    Negotiation --> T4 --> Won
    Negotiation --> TL --> Lost
    Prospecting --> TL --> Lost
    Qualified --> TL --> Lost
    Proposal --> TL --> Lost

    Prospecting -.-> Stale
    Qualified -.-> Stale
    Proposal -.-> Stale
    Negotiation -.-> Stale
```

---

## 5. Lead Scoring Graph (Bayesian PLS)

```mermaid
graph TB
    subgraph "Input Signals"
        Email[Email Format<br/>valid/disposable/role]
        Phone[Phone Type<br/>mobile/landline/voip]
        Title[Job Title<br/>executive/mid/entry]
        Industry[Industry<br/>tech/finance/healthcare]
        Company[Company Size<br/>1-10/11-50/51-200/200+]
        Source[Lead Source<br/>form/ referral/referral/organic]
    end

    subgraph "Scoring Frequencies"
        F1[P(won|email_valid) = 0.72]
        F2[P(won|mobile) = 0.65]
        F3[P(won|executive) = 0.81]
        F4[P(won|tech) = 0.58]
        F5[P(won|51-200) = 0.62]
        F6[P(won|referral) = 0.74]
    end

    subgraph "Bayesian Aggregation"
        Prior[Prior Probability<br/>P(won) = base_rate]
        Bayes[Bayesian Update:<br/>P(won|signals) = prior × ∏ P(signal|won)]
        Normalize[Normalize Probability<br/>P(won) ∈ [0, 100]]
    end

    subgraph "Output"
        Score[Lead Score<br/>probability: 68%]
        Factors[Top Factors<br/>1. executive title<br/>2. referral source<br/>3. mobile phone]
        Bucket[Score Bucket<br/>HOT / WARM / COLD]
    end

    Email --> F1
    Phone --> F2
    Title --> F3
    Industry --> F4
    Company --> F5
    Source --> F6

    F1 --> Prior --> Bayes
    F2 --> Prior
    F3 --> Prior
    F4 --> Prior
    F5 --> Prior
    F6 --> Prior

    Bayes --> Normalize
    Normalize --> Score
    Normalize --> Factors
    Score --> Bucket
```

---

## 6. Team Assignment Graph

```mermaid
graph TB
    subgraph "Team Structure"
        Team[CRM Team]
        Member1[Member: Alice<br/>max: 10 leads]
        Member2[Member: Bob<br/>max: 10 leads]
        Member3[Member: Carol<br/>max: 10 leads]
    end

    subgraph "Current Load"
        Load1[Alice: 8/10 assigned]
        Load2[Bob: 10/10 assigned]
        Load3[Carol: 3/10 assigned]
    end

    subgraph "New Lead"
        NewLead[Unassigned Lead<br/>type: OPPORTUNITY<br/>source: referral]
    end

    Team --> Member1 --> Load1
    Team --> Member2 --> Load2
    Team --> Member3 --> Load3

    NewLead --> Assigner[Assignment Engine]
    Assigner --> Load1
    Assigner --> Load2
    Assigner --> Load3

    Assigner -.->|Best fit| Member3
    Member3 -->|Assigned| NewLead
```

---

## 7. Campaign-to-Lead-Conversion Funnel

```mermaid
flowchart TD
    subgraph "UTM Hierarchy"
        Campaign[UTM Campaign<br/>name, source, medium]
        Medium[UTM Medium<br/>email, social, organic]
        Source[UTM Source<br/>newsletter, linkedin, google]
    end

    subgraph "Funnel Stages"
        Impressions[Impressions<br/>10,000]
        Clicks[Clicks<br/>1,500]
        FormSubmits[Form Submissions<br/>300]
        Leads[Leads Created<br/>250]
        Contacts[Contacts Linked<br/>200]
        Opportunities[Opportunities<br/>100]
        Won[Deals Won<br/>25]
    end

    Campaign --> Medium --> Source
    Source -.-> Campaign

    Impressions --> Clicks --> FormSubmits --> Leads --> Contacts --> Opportunities --> Won

    Campaign -.->|tracks| Leads
    Campaign -.->|tracks| Opportunities
```

---

## 8. Subscription Lifecycle Graph

```mermaid
stateDiagram-v2
    [*] --> Trial: Subscription Created
    
    Trial --> Active: Trial Period Ends
    Trial --> Cancelled: Trial Not Converted
    
    Active --> PastDue: Payment Failed
    Active --> Cancelled: Customer Cancels
    Active --> Renewed: Auto-Renewal
    
    PastDue --> Active: Payment Restored
    PastDue --> Cancelled: Payment Still Failed
    
    Renewed --> Active: Renewal Confirmed
    Renewed --> PastDue: Payment Failed
    
    Cancelled --> [*]: Subscription Closed
```

---

## 9. Chat Session Conversion Graph

```mermaid
graph TB
    subgraph "Pre-Conversion"
        Visitor[Visitor]
        Widget[Chat Widget]
        Agent[Agent Online]
        Session[Chat Session<br/>status: ACTIVE]
    end

    subgraph "During Chat"
        Messages[Messages Exchange]
        Qualification[Qualification]
        DataCapture[Data Capture<br/>name, email, company]
    end

    subgraph "Post-Chat"
        Conversion[Convert to Lead]
        Contact[Create Contact]
        Account[Create Account]
        Score[Score Lead]
        Assign[Assign to Team]
    end

    Visitor --> Widget --> Agent --> Session
    Session --> Messages --> Qualification --> DataCapture
    DataCapture --> Conversion
    Conversion --> Contact
    Conversion --> Account
    Conversion --> Score
    Score --> Assign
```

---

## 10. Custom Field Extension Graph

```mermaid
graph TB
    subgraph "Custom Fields (Extensible)"
        CF1[Field: Industry<br/>entity: leads<br/>type: SELECT]
        CF2[Field: Budget<br/>entity: leads<br/>type: INTEGER]
        CF3[Field: Competitors<br/>entity: leads<br/>type: TEXT]
        CF4[Field: Contract Date<br/>entity: accounts<br/>type: DATE]
        CF5[Field: Priority<br/>entity: contacts<br/>type: STRING]
    end

    subgraph "Entities"
        Lead[Lead Entity]
        Contact[Contact Entity]
        Account[Account Entity]
    end

    subgraph "Custom Field Storage"
        LeadCF[Lead Custom Fields JSON]
        ContactCF[Contact Custom Fields JSON]
        AccountCF[Account Custom Fields JSON]
    end

    CF1 --> Lead
    CF2 --> Lead
    CF3 --> Lead
    CF4 --> Account
    CF5 --> Contact

    Lead --> LeadCF
    Contact --> ContactCF
    Account --> AccountCF
```

---

## 11. Webhook Event Graph

```mermaid
graph TB
    subgraph "Event Sources"
        LeadCreate[lead.create]
        LeadUpdate[lead.update]
        LeadDelete[lead.delete]
        DealWon[opportunity.won]
        DealLost[opportunity.lost]
        ContactCreate[contact.create]
        SubRenewal[subscription.renewal_due]
    end

    subgraph "Webhook Destinations"
        W1[ERP Integration<br/>URL: https://erp.example.com/webhook]
        W2[Accounting System<br/>URL: https://acc.example.com/webhook]
        W3[Marketing Automation<br/>URL: https://ma.example.com/webhook]
        W4[Custom Slack Bot<br/>URL: https://hooks.slack.com/...]
    end

    LeadCreate --> W1
    LeadUpdate --> W1
    LeadDelete --> W1
    DealWon --> W2
    DealLost --> W2
    ContactCreate --> W3
    SubRenewal --> W3
    DealWon --> W4
    DealLost --> W4
```

---

## 12. Complete Entity Graph Summary

```mermaid
graph TD
    subgraph "Core Hub"
        Lead[LEAD]
    end

    subgraph "Entity Relationships"
        Contact[CONTACT]
        Account[ACCOUNT]
        Stage[STAGE]
        Team[TEAM]
        User[USER]
        Campaign[UTM_CAMPAIGN]
    end

    subgraph "Analytics & Intelligence"
        LeadScore[LEAD_SCORE]
        PipelineSummary[PIPELINE_SUMMARY]
        ConversionRate[CONVERSION_RATE]
        Forecast[FORECAST]
    end

    subgraph "Automation & Events"
        Workflow[WORKFLOW]
        Webhook[WEBHOOK]
        AuditLog[AUDIT_LOG]
    end

    subgraph "Engagement & Communication"
        Subscription[SUBSCRIPTION]
        ChatSession[CHAT_SESSION]
        EmailComm[EMAIL_COMMUNICATION]
    end

    Lead --> Contact
    Lead --> Account
    Lead --> Stage
    Lead --> Team
    Lead --> User
    Lead --> Campaign
    Lead --> LeadScore
    Lead --> PipelineSummary
    Lead --> Workflow
    Lead --> AuditLog
    Lead --> Subscription
    Lead --> ChatSession
    Lead --> EmailComm
```

---

## 13. Hierarchical Tree Queries (SQL Pattern)

```
-- Account hierarchy with recursive CTE
WITH RECURSIVE account_tree AS (
    -- Base case: top-level accounts (no parent)
    SELECT id, name, parent_id, 0 AS level, ARRAY[id] AS path
    FROM accounts WHERE parent_id IS NULL
    
    UNION ALL
    
    -- Recursive case: children
    SELECT a.id, a.name, a.parent_id, at.level + 1, at.path || a.id
    FROM accounts a
    JOIN account_tree at ON a.parent_id = at.id
)
SELECT * FROM account_tree ORDER BY path;

-- Contact tree with recursive CTE
WITH RECURSIVE contact_tree AS (
    SELECT id, name, parent_id, 0 AS level
    FROM contacts WHERE parent_id IS NULL
    
    UNION ALL
    
    SELECT c.id, c.name, c.parent_id, ct.level + 1
    FROM contacts c
    JOIN contact_tree ct ON c.parent_id = ct.id
)
SELECT * FROM contact_tree;

-- Pipeline summary with stage aggregation
SELECT 
    s.name AS stage,
    s.probability AS stage_probability,
    COUNT(l.id) AS lead_count,
    SUM(l.expected_revenue) AS total_revenue,
    SUM(l.expected_revenue * s.probability / 100.0) AS weighted_revenue,
    AVG(EXTRACT(DAY FROM l.date_closed - l.date_open)) AS avg_days_in_stage
FROM leads l
JOIN stages s ON l.stage_id = s.id
WHERE l.active = true
GROUP BY s.name, s.probability
ORDER BY s.sequence;
```

---

*This document defines all entity graph and hierarchy patterns. The lead entity serves as the central hub connecting contacts, accounts, stages, teams, and marketing campaigns. Recursive CTEs handle hierarchical trees for accounts and contacts. Bayesian scoring aggregates multiple input signals into a probability score.*
