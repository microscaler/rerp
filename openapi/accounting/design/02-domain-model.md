# Domain Model

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Core Entities

```mermaid
erDiagram
    Company ||--o{ Tenant : "owns"
    Tenant ||--o{ CompanyAccount : "manages"
    CompanyAccount ||--o{ Account : "contains"
    CompanyAccount ||--o{ JournalEntry : "contains"
    CompanyAccount ||--o{ JournalEntryLine : "contains"
    
    Tenant ||--o{ Vendor : "manages"
    Vendor ||--o{ VendorInvoice : "issues"
    VendorInvoice ||--o{ InvoiceLineItem : "contains"
    VendorInvoice }o--|| JournalEntry : "posts to"
    
    Tenant ||--o{ Customer : "manages"
    Customer ||--o{ CustomerInvoice : "creates"
    CustomerInvoice ||--o{ InvoiceLineItem : "contains"
    CustomerInvoice }o--|| JournalEntry : "posts to"
    
    Tenant ||--o{ Asset : "owns"
    Asset }o--|| Account : "uses depreciation"
    Asset ||--o{ DepreciationEntry : "generates"
    
    Tenant ||--o{ BankAccount : "maintains"
    BankAccount ||--o{ BankTransaction : "has"
    BankTransaction }o--|| JournalEntry : "reconciled to"
    
    Tenant ||--o{ Budget : "creates"
    Budget ||--o{ BudgetLine : "contains"
    Budget ||--o{ Forecast : "generates"
    
    Tenant ||--o{ Lease : "maintains"
    Lease ||--o{ LeaseLiability : "has"
    Lease ||--o{ RightOfUseAsset : "has"
    Lease ||--o{ LeasePaymentSchedule : "generates"
    
    Tenant ||--o{ TaxPeriod : "files"
    TaxPeriod ||--o{ TaxReturn : "submits"
    TaxPeriod }o--|| JournalEntry : "posts taxes"
    
    Tenant ||--o{ ConsolidationRun : "executes"
    ConsolidationRun ||--o{ EliminationEntry : "generates"
    ConsolidationGroup ||--o{ ConsolidationRun : "triggers"
    
    Tenant ||--o{ AuditEvent : "records"
    Tenant ||--o{ RecognitionRun : "executes"
    RecognitionRun ||--o{ RecognitionSchedule : "produces"
    RecognitionRun ||--o{ DeferredItem : "tracks"
    
    Company {
        uuid id PK
        string name
        jsonb metadata
    }
    Tenant {
        uuid id PK
        uuid company_id FK
        string name
        string locale
        datetime created_at
    }
    CompanyAccount {
        uuid id PK
        uuid tenant_id FK
        string account_code
        string fiscal_year_start
        string currency
    }
    Account {
        uuid id PK
        uuid company_account_id FK
        string code
        string name
        string type
        string category
        boolean active
    }
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
    VendorInvoice {
        uuid id PK
        uuid tenant_id FK
        string invoice_number
        decimal amount
        string currency
        string status
        datetime due_date
    }
    CustomerInvoice {
        uuid id PK
        uuid tenant_id FK
        string invoice_number
        decimal amount
        string currency
        string status
        datetime due_date
    }
```

---

## Entity Ownership Matrix

```mermaid
graph TB
    subgraph "Owned By Service"
        GL_OWN[JournalEntry<br/>JournalEntryLine<br/>Account<br/>CompanyAccount<br/>ChartOfAccount]
        AP_OWN[VendorInvoice<br/>VendorPayment<br/>ApprovalRequest<br/>CashFlowForecast]
        AR_OWN[CustomerInvoice<br/>PaymentApplication<br/>CollectionCase<br/>AgingSummary]
        INV_OWN[InvoiceApproval<br/>InvoiceHandoff<br/>InvoicePaymentMatch]
        REV_OWN[RecognitionRule<br/>RecognitionRun<br/>RecognitionSchedule<br/>DeferredItem]
        TAX_OWN[TaxPeriod<br/>TaxReturn<br/>TaxPayment<br/>TaxRule]
        LEASE_OWN[Lease<br/>LeaseLiability<br/>RightOfUseAsset<br/>LeasePaymentSchedule]
        CONSOL_OWN[ConsolidationGroup<br/>ConsolidationRun<br/>EliminationRule<br/>EliminationEntry]
        BANK_OWN[BankAccount<br/>BankTransaction<br/>BankReconciliation<br/>BankStatement]
        TREASURY_OWN[CashPosition<br/>CashForecast<br/>CashTransfer<br/>LiquidityPlan]
        BUDGET_OWN[Budget<br/>BudgetLine<br/>BudgetRevision<br/>Forecast]
        REPORTS_OWN[BalanceSheet<br/>IncomeStatement<br/>CashFlowStatement<br/>TrialBalance]
        ASSET_OWN[Asset<br/>AssetCategory<br/>DepreciationEntry<br/>DepreciationSchedule]
        AUDIT_OWN[AuditEvent<br/>ApprovalPolicy<br/>SegregationRule<br/>ControlException]
        DOC_OWN[ExtractionJob<br/>ExtractionResult<br/>DocumentClassification<br/>AccountingDocument]
        EDI_OWN[EdiDocument<br/>EdiMapping<br/>EdiProfile<br/>EdiSubmission]
    end
    
    subgraph "Shared/Referenced"
        SHARED[Company<br/>User<br/>Tenant<br/>Currency<br/>FiscalPeriod]
    end
    
    SHARED -.-> GL_OWN
    SHARED -.-> AP_OWN
    SHARED -.-> AR_OWN
    SHARED -.-> INV_OWN
    SHARED -.-> REV_OWN
    SHARED -.-> TAX_OWN
    SHARED -.-> LEASE_OWN
    SHARED -.-> CONSOL_OWN
    SHARED -.-> BANK_OWN
    SHARED -.-> TREASURY_OWN
    SHARED -.-> BUDGET_OWN
    SHARED -.-> REPORTS_OWN
    SHARED -.-> ASSET_OWN
    SHARED -.-> AUDIT_OWN
    SHARED -.-> DOC_OWN
    SHARED -.-> EDI_OWN
    
    classDef owned fill:#d5f5e3,stroke:#27ae60
    classDef shared fill:#d4e6f1,stroke:#2980b9
    
    class GL_OWN,AP_OWN,AR_OWN,INV_OWN,REV_OWN,TAX_OWN,LEASE_OWN,CONSOL_OWN,BANK_OWN,TREASURY_OWN,BUDGET_OWN,REPORTS_OWN,ASSET_OWN,AUDIT_OWN,DOC_OWN,EDI_OWN owned
    class SHARED shared
```

---

## Key Domain Concepts

### Double-Entry Accounting

Every financial transaction affects at least two accounts:
- **Debits** increase assets/expenses, decrease liabilities/revenue
- **Credits** decrease assets/expenses, increase liabilities/revenue
- Every journal entry must balance: `sum(debits) == sum(credits)`

```mermaid
graph LR
    subgraph "Transaction Flow"
        TRANS[Transaction Event]
        TRANS --> DEBIT[Debit Account<br/>Increase Asset/Expense]
        TRANS --> CREDIT[Credit Account<br/>Decrease Asset/Revenue]
        DEBIT --> BAL[Balance Check<br/>Debit = Credit]
        CREDIT --> BAL
        BAL --> POST[Post to GL]
    end
    
    classDef trans fill:#fdebd0,stroke:#e67e22
    classDef action fill:#d5f5e3,stroke:#27ae60
    classDef check fill:#fadbd8,stroke:#c0392b
    
    class TRANS trans
    class DEBIT,CREDIT,POST action
    class BAL check
```

### Fiscal Period Management

```mermaid
erDiagram
    FiscalPeriod ||--o{ JournalEntry : "contains"
    FiscalPeriod ||--o{ Budget : "owns"
    FiscalPeriod ||--o{ TaxPeriod : "aligns to"
    
    FiscalPeriod {
        uuid id PK
        string name
        date start_date
        date end_date
        integer year
        integer period_number
        boolean is_closed
        boolean is_locked
    }
    
    JournalEntry {
        uuid id PK
        uuid period_id FK
        datetime entry_date
        string period
        string status
    }
    
    Budget {
        uuid id PK
        uuid period_id FK
        decimal amount
        string currency
    }
    
    TaxPeriod {
        uuid id PK
        uuid period_id FK
        string tax_type
        string status
    }
```

### Chart of Accounts Hierarchy

```mermaid
graph TB
    COA[Chart of Accounts]
    
    COA --> ASSETS[1000-1999: Assets]
    COA --> LIABILITIES[2000-2999: Liabilities]
    COA --> EQUITY[3000-3999: Equity]
    COA --> REVENUE[4000-4999: Revenue]
    COA --> EXPENSES[5000-5999: Expenses]
    
    ASSETS --> CURRENT[Current Assets<br/>1000-1999]
    ASSETS --> NON_CURRENT[Non-Current Assets<br/>1000-1999]
    
    CURRENT --> CASH[Cash & Equivalents<br/>1000-1099]
    CURRENT --> AR[Accounts Receivable<br/>1100-1199]
    CURRENT --> INV[Inventory<br/>1200-1299]
    
    NON_CURRENT --> PP&E[Property, Plant, Equip<br/>1500-1599]
    NON_CURRENT --> INTANGIBLE[Intangible Assets<br/>1600-1699]
    
    LIABILITIES --> CURRENT_LIAB[Current Liabilities<br/>2000-2999]
    LIABILITIES --> LONG_TERM[Long-Term Liabilities<br/>2500-2999]
    
    CURRENT_LIAB --> AP[Accounts Payable<br/>2000-2099]
    CURRENT_LIAB --> TAX_LIAB[Taxes Payable<br/>2100-2199]
    
    EQUITY --> COMMON_STOCK[Common Stock<br/>3000-3099]
    EQUITY --> RETAINED[Retained Earnings<br/>3100-3199]
    
    REVENUE --> SALES[Sales Revenue<br/>4000-4099]
    REVENUE --> OTHER_REV[Other Revenue<br/>4900-4999]
    
    EXPENSES --> COGS[Cost of Goods Sold<br/>5000-5099]
    EXPENSES --> OP_EXP[Operating Expenses<br/>5100-5999]
    
    classDef root fill:#d4e6f1,stroke:#2980b9,stroke-width:3px
    classDef category fill:#d5f5e3,stroke:#27ae60
    classDef sub fill:#e8daef,stroke:#8e44ad
    classDef leaf fill:#fadbd8,stroke:#c0392b
    
    class COA root
    class ASSETS,LIABILITIES,EQUITY,REVENUE,EXPENSES category
    class CURRENT,NON_CURRENT,CURRENT_LIAB,LONG_TERM,COMMON_STOCK,RETAINED,SALES,OTHER_REV,COGS,OP_EXP sub
    class CASH,AR,INV,PP&E,INTANGIBLE,AP,TAX_LIAB,REVENUE,EXPENSES leaf
```

---

*Continue to [Entity Relationships](./03-entity-relationships.md)*
