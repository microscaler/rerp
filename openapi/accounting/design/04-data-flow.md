# Data Flow

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Invoice Processing Pipeline

```mermaid
graph TB
    subgraph "Document Intake"
        DOC[Document Source<br/>Email, EDI, Upload, Scan]
        DOC --> EXTRACT[Documents Extraction Service]
        EXTRACT --> OCR[OCR Processing]
        OCR --> CLASSIFY[Classify Document]
    end
    
    subgraph "Invoice Creation"
        CLASSIFY --> INV_DATA[Extracted Invoice Data]
        INV_DATA --> INV_SVC[Invoice Service]
        INV_SVC --> VALIDATE[Validate & Match]
        VALIDATE --> WORKFLOW[Approval Workflow]
    end
    
    subgraph "Posting"
        WORKFLOW --> APPROVED[Approved]
        APPROVED --> POST[Post to GL]
        POST --> GL[General Ledger]
        GL --> ENTRY[Journal Entry]
        ENTRY --> BALANCED[Balance Verified]
    end
    
    subgraph "Post-Posting"
        BALANCED --> REV_REC[Revenue Recognition]
        BALANCED --> TAX_CALC[Tax Calculation]
        BALANCED --> AR_AP[AR/AP Update]
        REV_REC --> SCHEDULE[Recognition Schedule]
        TAX_CALC --> RETURNS[Tax Returns]
        AR_AP --> PAYMENT[Payment/Collection]
    end
    
    classDef intake fill:#fdebd0,stroke:#e67e22
    classDef create fill:#fadbd8,stroke:#c0392b
    classDef post fill:#d5f5e3,stroke:#27ae60
    classDef postflow fill:#e8daef,stroke:#8e44ad
    
    class DOC,EXTRACT,OCR,CLASSIFY intake
    class INV_DATA,INV_SVC,VALIDATE,WORKFLOW create
    class APPROVED,POST,GL,ENTRY,BALANCED post
    class REV_REC,TAX_CALC,AR_AP,SCHEDULE,RETURNS,PAYMENT postflow
```

### Invoice State Machine

```mermaid
stateDiagram-v2
    [*] --> Draft
    Draft --> PendingApproval: Submit for Approval
    PendingApproval --> Approved: Approve
    PendingApproval --> Rejected: Reject
    Rejected --> Draft: Revise & Resubmit
    Approved --> Posted: Post to GL
    Posted --> PartiallyPaid: Record Payment
    PartiallyPaid --> FullyPaid: Final Payment
    FullyPaid --> [*]
    
    note right of Draft
        Initial state after document
        extraction or manual entry
    end note
    
    note right of PendingApproval
        Waits for approval workflow
        based on amount/permissions
    end note
    
    note right of Posted
        Journal entries created in GL
        Debits and credits balanced
    end note
    
    note right of FullyPaid
        All payments collected
        Invoice closed
    end note
```

---

## Bank Reconciliation Flow

```mermaid
graph TB
    subgraph "Bank Data Ingestion"
        BANK_API[Bank API/Plaid]
        BANK_API --> STATEMENT[Bank Statement Import]
        STATEMENT --> TRANSACTIONS[Bank Transactions]
    end
    
    subgraph "GL Data"
        GL[General Ledger]
        GL --> ENTRIES[Journal Entries]
    end
    
    subgraph "Matching Engine"
        TRANSACTIONS --> MATCH[Auto-Match Transactions]
        ENTRIES --> MATCH
        MATCH --> AUTO[Automatically Matched]
        MATCH --> MANUAL[Requires Manual Review]
    end
    
    subgraph "Reconciliation"
        AUTO --> RECONCILED[Reconciled]
        MANUAL --> REVIEW[Manual Review]
        REVIEW --> RECONCILED
        RECONCILED --> DIFF[Identify Differences]
        DIFF --> ADJUST[Adjusting Entries]
        ADJUST --> GL
    end
    
    subgraph "Reporting"
        RECONCILED --> REPORT[Reconciliation Report]
        REPORT --> TREASURY[Treasury Service]
    end
    
    classDef bank fill:#d4e6f1,stroke:#2980b9
    classDef gl fill:#d5f5e3,stroke:#27ae60
    classDef match fill:#fdebd0,stroke:#e67e22
    classDef recon fill:#e8daef,stroke:#8e44ad
    
    class BANK_API,STATEMENT,TRANSACTIONS bank
    class GL,ENTRIES gl
    class MATCH,AUTO,MANUAL match
    class RECONCILED,REVIEW,DIFF,ADJUST,REPORT,RECONCILED recon
```

### Reconciliation Matching Logic

```mermaid
graph LR
    subgraph "Matching Criteria"
        AMOUNT[Amount Match]
        DATE[Date Match<br/>±3 days]
        DESC[Description Match<br/>Text similarity]
        REF[Reference Match<br/>Invoice number]
    end
    
    subgraph "Match Result"
        PERFECT[Perfect Match<br/>All criteria met]
        PARTIAL[Partial Match<br/>Some criteria met]
        NO_MATCH[No Match<br/>Manual review]
    end
    
    AMOUNT --> PERFECT
    DATE --> PARTIAL
    DESC --> PARTIAL
    REF --> PERFECT
    
    PERFECT --> AUTO[Auto-Reconcile]
    PARTIAL --> REVIEW[Flag for Review]
    NO_MATCH --> MANUAL[Manual Assignment]
    
    AUTO --> RECONCILED[Reconciled]
    REVIEW --> RECONCILED
    MANUAL --> RECONCILED
    
    classDef criteria fill:#fdebd0,stroke:#e67e22
    classDef result fill:#d5f5e3,stroke:#27ae60
    classDef action fill:#d4e6f1,stroke:#2980b9
    
    class AMOUNT,DATE,DESC,REF criteria
    class PERFECT,PARTIAL,NO_MATCH result
    class AUTO,REVIEW,MANUAL,RECONCILED action
```

---

## Month-End Close Process

### Close Workflow

```mermaid
graph TB
    subgraph "Pre-Close Checklist"
        C1[All Invoices Posted]
        C2[Bank Reconciled]
        C3[Depreciation Run]
        C4[Accruals Recorded]
        C5[Intercompany Eliminations]
    end
    
    subgraph "Close Execution"
        CLOSE[Close Period]
        CLOSE --> LOCK[Lock Entries]
        LOCK --> VERIFY[Verify Balances]
        VERIFY --> REPORT[Generate Reports]
    end
    
    subgraph "Post-Close"
        POST1[Tax Filing]
        POST2[Consolidation]
        POST3[Budget Variance]
        POST4[Audit Trail]
    end
    
    C1 & C2 & C3 & C4 & C5 --> CLOSE
    REPORT --> POST1
    REPORT --> POST2
    REPORT --> POST3
    REPORT --> POST4
    
    classDef pre fill:#fdebd0,stroke:#e67e22
    classDef exec fill:#d5f5e3,stroke:#27ae60
    classDef post fill:#e8daef,stroke:#8e44ad
    
    class C1,C2,C3,C4,C5 pre
    class CLOSE,LOCK,VERIFY,REPORT exec
    class POST1,POST2,POST3,POST4 post
```

### Period State Machine

```mermaid
stateDiagram-v2
    [*] --> Open
    Open --> Closed: Close Period
    Closed --> Locked: Lock Period
    Locked --> Reopened: Override (Admin)
    Reopened --> Open: Allow Retroactive Entries
    Open --> [*]
    
    note right of Open
        Normal operations allowed
        New entries can be posted
    end note
    
    note right of Closed
        Entries can still be added
        Reports can be generated
    end note
    
    note right of Locked
        No entries allowed
        Final reports published
    end note
```

---

## Revenue Recognition Flow

```mermaid
graph TB
    subgraph "Revenue Sources"
        INV[Customer Invoice]
        SUBSCRIPTION[Subscription Billing]
        MILESTONE[Milestone Billing]
    end
    
    subgraph "Deferral Logic"
        INV --> DEFER[Defer Revenue]
        SUBSCRIPTION --> DEFER
        MILESTONE --> DEFER
        DEFER --> RULES[Recognition Rules]
    end
    
    subgraph "Recognition Engine"
        RULES --> SCHEDULE[Generate Schedule]
        SCHEDULE --> MONTHLY[Monthly Recognition]
        MONTHLY --> POST[Post to GL]
    end
    
    subgraph "Reporting"
        POST --> DEFERRED[Deferred Revenue Report]
        POST --> RECOGNIZED[Revenue Report]
        POST --> VARIANCE[Variance Analysis]
    end
    
    classDef source fill:#fdebd0,stroke:#e67e22
    classDef logic fill:#d5f5e3,stroke:#27ae60
    classDef report fill:#e8daef,stroke:#8e44ad
    
    class INV,SUBSCRIPTION,MILESTONE source
    class DEFER,RULES,SCHEDULE,MONTHLY,POST logic
    class DEFERRED,RECOGNIZED,VARIANCE report
```

---

*Continue to [Sequence Diagrams](./05-sequence-diagrams.md)*
