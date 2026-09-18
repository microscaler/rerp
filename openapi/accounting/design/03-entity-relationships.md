# Entity Relationships

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Cross-Service Entity Links

### Accounting Document Flow

```mermaid
graph LR
    subgraph "Document Flow"
        DOC[Documents Extraction]
        DOC --> CLASSIFY[Classify Document]
        CLASSIFY --> INV_CREATE[Create Invoice]
        INV_CREATE --> INV_APPROVE[Invoice Approved]
        INV_APPROVE --> INV_POST[Post to GL]
        INV_POST --> GL_ENTRY[Journal Entry Created]
        GL_ENTRY --> GL_REVIEW[GL Reviewed]
        GL_REVIEW --> GL_POSTED[GL Posted]
        GL_POSTED --> FIN_REPORT[Financial Report]
    end
    
    DOC -.->|vendor_tax_id| TAX[Tax Compliance]
    GL_POSTED -.->|period| BUDGET[Budget Mgmt]
    GL_POSTED -.->|balances| REPORTS[Financial Reports]
    
    classDef document fill:#fdebd0,stroke:#e67e22
    classDef invoice fill:#fadbd8,stroke:#c0392b
    classDef gl fill:#d5f5e3,stroke:#27ae60
    classDef report fill:#e8daef,stroke:#8e44ad
    
    class DOC,CLASSIFY,FIN_REPORT document
    class INV_CREATE,INV_APPROVE,INV_POST invoice
    class GL_ENTRY,GL_REVIEW,GL_POSTED gl
    class REPORTS report
```

### Financial Transaction Flow

```mermaid
erDiagram
    JournalEntry ||--o{ JournalEntryLine : "has lines"
    JournalEntry }o--|| Account : "debits to"
    JournalEntry }o--|| Account : "credits to"
    VendorInvoice }o--|| JournalEntry : "generates"
    CustomerInvoice }o--|| JournalEntry : "generates"
    Asset }o--|| JournalEntry : "depreciation entries"
    Lease }o--|| JournalEntry : "lease entries"
    
    JournalEntryLine ||--|| Account : "references"
    JournalEntryLine }o--|| FiscalPeriod : "belongs to"
    
    VendorInvoice ||--|| Vendor : "from"
    CustomerInvoice ||--|| Customer : "from"
    
    Asset ||--|| AssetCategory : "categorized"
    Lease ||--|| LeasePaymentSchedule : "has schedule"
    
    JournalEntry {
        uuid id PK
        uuid company_account_id FK
        string entry_number
        datetime entry_date
        string period
        string status
        decimal total_debit
        decimal total_credit
        boolean balanced
    }
    
    JournalEntryLine {
        uuid id PK
        uuid journal_entry_id FK
        uuid account_id FK
        string description
        decimal debit
        decimal credit
        integer line_number
    }
    
    Account {
        uuid id PK
        string code
        string name
        string type
        string category
    }
    
    VendorInvoice {
        uuid id PK
        string invoice_number
        decimal amount
        string status
    }
    
    CustomerInvoice {
        uuid id PK
        string invoice_number
        decimal amount
        string status
    }
```

### Master Data Relationships

```mermaid
graph TB
    subgraph "Tenant Structure"
        COMPANY[Company]
        COMPANY --> TENANT[Tenant]
        TENANT --> COA[Chart of Accounts]
        TENANT --> VENDORS[Vendors]
        TENANT --> CUSTOMERS[Customers]
        TENANT --> BANK_ACCTS[Bank Accounts]
        TENANT --> ASSETS[Fixed Assets]
        TENANT --> LEASES[Leases]
    end
    
    subgraph "Financial Records"
        COA --> ACCOUNTS[Accounts]
        ACCOUNTS --> JOURNAL_ENTRIES[Journal Entries]
        VENDORS --> VENDOR_INV[Vendor Invoices]
        CUSTOMERS --> CUST_INV[Customer Invoices]
        VENDOR_INV --> PAYMENTS[Vendor Payments]
        CUST_INV --> COLLECTIONS[Customer Collections]
    end
    
    subgraph "Reporting"
        JOURNAL_ENTRIES --> TRIAL_BALANCE[Trial Balance]
        JOURNAL_ENTRIES --> BS[Balance Sheet]
        JOURNAL_ENTRIES --> IS[Income Statement]
        JOURNAL_ENTRIES --> CF[Cash Flow]
    end
    
    COMPANY -.->|multiple| TENANT
    VENDOR_INV -.->|posts to| JOURNAL_ENTRIES
    CUST_INV -.->|posts to| JOURNAL_ENTRIES
    ASSETS -.->|depreciation| JOURNAL_ENTRIES
    LEASES -.->|amortization| JOURNAL_ENTRIES
    
    classDef master fill:#d4e6f1,stroke:#2980b9
    classDef transaction fill:#d5f5e3,stroke:#27ae60
    classDef report fill:#e8daef,stroke:#8e44ad
    
    class COMPANY,TENANT,COA,VENDORS,CUSTOMERS,BANK_ACCTS,ASSETS,LEASES master
    class ACCOUNTS,JOURNAL_ENTRIES,VENDOR_INV,CUST_INV,PAYMENTS,COLLECTIONS transaction
    class TRIAL_BALANCE,BS,IS,CF report
```

---

## Cross-Service API Relationships

### Service-to-Service References

```mermaid
graph LR
    subgraph "References By Invoice Service"
        INV_REF1[General Ledger<br/>POST /journal-entries]
        INV_REF2[Accounts Payable<br/>POST /vendor-invoices]
        INV_REF3[Accounts Receivable<br/>POST /customer-invoices]
    end
    
    subgraph "References By GL Service"
        GL_REF1[Budget Mgmt<br/>GET /budgets]
        GL_REF2[Reports Service<br/>GET /trial-balance]
        GL_REF3[Audit Controls<br/>POST /audit-events]
    end
    
    subgraph "References By AP/AR"
        APAR_REF1[Invoice Service<br/>GET /invoices]
        APAR_REF2[Bank Sync<br/>GET /bank-transactions]
        APAR_REF3[Documents Extraction<br/>POST /extract]
    end
    
    INV -->|posts| INV_REF1
    AP -->|creates| INV_REF2
    AR -->|creates| INV_REF3
    GL -->|aggregates| GL_REF1
    GL -->|reports| GL_REF2
    GL -->|monitors| GL_REF3
    AP -->|checks| APAR_REF1
    AP -->|pays via| APAR_REF2
    AP -->|receives from| APAR_REF3
    
    classDef invoice fill:#fadbd8,stroke:#c0392b
    classDef gl fill:#d5f5e3,stroke:#27ae60
    classDef external_ref fill:#fdebd0,stroke:#e67e22
    
    class INV_REF1,INV_REF2,INV_REF3 external_ref
    class GL_REF1,GL_REF2,GL_REF3 external_ref
    class APAR_REF1,APAR_REF2,APAR_REF3 external_ref
```

### Shared Entity Dependencies

```mermaid
graph TB
    subgraph "Shared Domain Entities"
        COMPANY[Company]
        TENANT[Tenant]
        USER[User]
        CURRENCY[Currency]
        FISCAL_PERIOD[FiscalPeriod]
    end
    
    subgraph "Dependent Services"
        GL[General Ledger]
        AP[Accounts Payable]
        AR[Accounts Receivable]
        INV[Invoice Service]
        TAX[Tax Compliance]
        BUDGET[Budget Management]
        REPORTS[Financial Reports]
    end
    
    COMPANY -.-> GL
    TENANT -.-> GL
    CURRENCY -.-> GL
    FISCAL_PERIOD -.-> GL
    
    COMPANY -.-> AP
    TENANT -.-> AP
    CURRENCY -.-> AP
    FISCAL_PERIOD -.-> AP
    
    COMPANY -.-> AR
    TENANT -.-> AR
    CURRENCY -.-> AR
    FISCAL_PERIOD -.-> AR
    
    COMPANY -.-> INV
    TENANT -.-> INV
    CURRENCY -.-> INV
    FISCAL_PERIOD -.-> INV
    
    COMPANY -.-> TAX
    TENANT -.-> TAX
    CURRENCY -.-> TAX
    FISCAL_PERIOD -.-> TAX
    
    COMPANY -.-> BUDGET
    TENANT -.-> BUDGET
    CURRENCY -.-> BUDGET
    FISCAL_PERIOD -.-> BUDGET
    
    COMPANY -.-> REPORTS
    TENANT -.-> REPORTS
    CURRENCY -.-> REPORTS
    FISCAL_PERIOD -.-> REPORTS
    
    classDef shared fill:#d4e6f1,stroke:#2980b9,stroke-width:3px
    classDef service fill:#d5f5e3,stroke:#27ae60
    
    class COMPANY,TENANT,USER,CURRENCY,FISCAL_PERIOD shared
    class GL,AP,AR,INV,TAX,BUDGET,REPORTS service
```

---

## Constraint Relationships

### Business Rules

```mermaid
graph TB
    RULES[Business Rules]
    
    RULES --> R1[Double-Entry Rule<br/>Sum debits == Sum credits]
    RULES --> R2[Period Lock Rule<br/>No entries in closed periods]
    RULES --> R3[Account Hierarchy<br/>Child accounts roll up to parent]
    RULES --> R4[Currency Conversion<br/>FX rate applied consistently]
    RULES --> R5[Approval Workflows<br/>Must follow hierarchy]
    RULES --> R6[Segregation of Duties<br/>Creator != Approver]
    
    R1 --> GL[Enforced by GL Service]
    R2 --> GL
    R3 --> COA[Enforced by Chart of Accounts]
    R4 --> FX[Enforced by Currency Service]
    R5 --> WF[Enforced by Invoice/Approval]
    R6 --> AUDIT[Enforced by Audit Controls]
    
    classDef rules fill:#fdebd0,stroke:#e67e22
    classDef enforcer fill:#d5f5e3,stroke:#27ae60
    
    class RULES,R1,R2,R3,R4,R5,R6 rules
    class GL,COA,FX,WFE,AUDIT enforcer
```

---

*Continue to [Data Flow](./04-data-flow.md)*
