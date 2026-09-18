# Sequence Diagrams

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Vendor Invoice to Payment

```mermaid
sequenceDiagram
    participant Vendor
    participant DOC as Documents<br/>Extraction
    participant INV as Invoice Service
    participant AP as Accounts<br/>Payable
    participant GL as General Ledger
    participant BANK as Bank Sync
    participant TREASURY as Treasury
    
    Vendor->>DOC: Submit Invoice (PDF/EDI)
    DOC->>DOC: OCR & Extract Data
    DOC->>INV: Create Invoice
    INV->>GL: Create Journal Entry
    GL-->>INV: Entry {balanced: true}
    INV-->>AP: Invoice Created
    AP->>AP: Schedule Payment
    
    Note over AP,GL: Payment Due Date
    AP->>TREASURY: Check Cash Position
    TREASURY->>BANK: Get Bank Balances
    BANK-->>TREASURY: Available Balance
    TREASURY-->>AP: Cash Sufficient
    
    AP->>INV: Approve Invoice
    INV->>GL: Post Journal Entry
    GL->>BANK: Create Payment Transaction
    BANK-->>GL: Payment {status: sent}
    GL-->>AP: Payment Confirmed
    AP-->>Vendor: Payment Sent
    
    BANK->>TREASURY: Update Balance
    TREASURY->>TREASURY: Recalculate Forecast
```

---

## Revenue Recognition Schedule

```mermaid
sequenceDiagram
    participant Customer
    participant INV as Invoice Service
    participant GL as General Ledger
    participant REV as Revenue<br/>Recognition
    participant REPORTS as Financial<br/>Reports
    
    Customer->>INV: Purchase Service
    INV->>GL: Create Deferred Revenue Entry
    GL-->>INV: Entry {type: deferred}
    
    INV->>REV: Notify Invoice Posted
    REV->>REV: Create Recognition Rule
    REV->>REV: Generate Schedule
    
    loop Monthly Recognition
        REV->>REV: Run Recognition Engine
        REV->>GL: Post Revenue Entry
        GL-->>REV: Entry {type: revenue}
        REV->>REPORTS: Update Deferred Revenue
        REPORTS-->>REV: Schedule Updated
    end
    
    Note over REV,GL: Revenue recognized<br/>over service period
```

---

## Multi-Entity Consolidation

```mermaid
sequenceDiagram
    participant Controller
    participant CONSOL as Consolidation<br/>Service
    participant GL as GL (Entity A)
    participant GL2 as GL (Entity B)
    participant GL3 as GL (Entity C)
    participant AUDIT as Audit Controls
    participant REPORTS as Financial<br/>Reports
    
    Controller->>CONSOL: Create Consolidation Group
    CONSOL->>CONSOL: Define Parent/Child Entities
    CONSOL->>AUDIT: Validate Access Policies
    
    Controller->>CONSOL: Start Consolidation Run
    CONSOL->>GL: Fetch Trial Balance
    GL-->>CONSOL: Entity A Trial Balance
    CONSOL->>GL2: Fetch Trial Balance
    GL2-->>CONSOL: Entity B Trial Balance
    CONSOL->>GL3: Fetch Trial Balance
    GL3-->>CONSOL: Entity C Trial Balance
    
    CONSOL->>CONSOL: Apply Exchange Rates
    CONSOL->>CONSOL: Aggregate Balances
    CONSOL->>CONSOL: Apply Elimination Rules
    CONSOL->>AUDIT: Record Elimination Entries
    
    CONSOL->>REPORTS: Generate Consolidated Report
    REPORTS-->>Controller: Consolidated Financials
    CONSOL-->>Controller: Consolidation Run Complete
```

---

## Month-End Close Process

```mermaid
sequenceDiagram
    participant User
    participant BFF as Accounting BFF
    participant GL as General Ledger
    participant TAX as Tax Compliance
    participant CONSOL as Consolidation
    participant REPORTS as Financial Reports
    participant AUDIT as Audit Controls
    
    User->>BFF: POST /periods/close
    BFF->>GL: POST /journal-entries/close-period
    GL->>GL: Validate all entries posted
    GL->>GL: Balance check (debit=credit)
    GL->>AUDIT: Record close attempt
    GL-->>BFF: Period {status: closed}
    
    BFF->>TAX: POST /tax-periods/close
    TAX->>TAX: Calculate tax liabilities
    TAX-->>BFF: TaxPeriod {status: filed}
    
    BFF->>CONSOL: POST /consolidation-runs
    CONSOL->>CONSOL: Aggregate entity data
    CONSOL->>CONSOL: Apply elimination rules
    CONSOL-->>BFF: ConsolidationRun {status: complete}
    
    BFF->>REPORTS: POST /reports/generate
    REPORTS->>GL: GET /trial-balance
    REPORTS->>GL: GET /balance-sheet
    REPORTS->>GL: GET /income-statement
    REPORTS->>GL: GET /cash-flow-statement
    REPORTS-->>BFF: Reports generated
    
    BFF-->>User: Close complete
    User->>REPORTS: View reports
```

---

## Bank Reconciliation Process

```mermaid
sequenceDiagram
    participant Bank
    participant BANK_SVC as Bank Sync<br/>Service
    participant GL as General Ledger
    participant MATCH as Matching Engine
    participant TREASURY as Treasury
    participant USER as Accountant
    
    Bank->>BANK_SVC: Push Transaction Feed
    BANK_SVC->>BANK_SVC: Create Bank Transaction
    BANK_SVC->>GL: Fetch Posted Entries
    GL-->>BANK_SVC: Return Journal Entries
    
    BANK_SVC->>MATCH: Run Auto-Match
    MATCH->>MATCH: Compare Amount/Date/Ref
    MATCH-->>BANK_SVC: Match Results
    
    alt Perfect Match
        BANK_SVC->>BANK_SVC: Auto-Reconcile
        BANK_SVC->>TREASURY: Update Cash Position
    else No Match
        BANK_SVC->>USER: Flag for Review
        USER->>BANK_SVC: Manual Match Selection
        BANK_SVC->>BANK_SVC: Create Reconciliation
        BANK_SVC->>TREASURY: Update Cash Position
    end
    
    BANK_SVC->>BANK_SVC: Generate Reconciliation Report
    BANK_SVC-->>USER: Report {reconciled, pending, differences}
```

---

## Chart Template Deployment

```mermaid
sequenceDiagram
    participant Admin
    participant COA as Chart of Accounts<br/>Service
    participant GL as General Ledger
    participant DEPLOY as Template<br/>Deployment
    
    Admin->>COA: Create Chart Template
    COA->>COA: Define Account Structure
    COA-->>Admin: Template {id, accounts: [...]}
    
    Admin->>GL: Request Template Deployment
    GL->>DEPLOY: Validate Template
    DEPLOY->>DEPLOY: Check for Conflicts
    DEPLOY->>GL: Check Existing Accounts
    
    alt No Conflicts
        DEPLOY->>GL: Deploy Accounts
        GL->>GL: Create Accounts in Schema
        GL-->>DEPLOY: Deployed {count: N}
    else Conflicts
        DEPLOY->>GL: Identify Conflicts
        GL-->>DEPLOY: Conflicts {accounts: [...]}
        DEPLOY->>Admin: Conflict Report
        Admin->>GL: Resolve Conflicts
    end
    
    DEPLOY-->>Admin: Deployment Complete
```

---

## Approval Workflow

```mermaid
sequenceDiagram
    participant Clerk
    participant INV as Invoice Service
    participant WORKFLOW as Approval<br/>Workflow
    participant APPROVER1 as Approver 1<br/>Manager
    participant APPROVER2 as Approver 2<br/>Director
    participant GL as General Ledger
    
    Clerk->>INV: Submit Invoice ($50,000)
    INV->>WORKFLOW: Evaluate Approval Rules
    WORKFLOW->>WORKFLOW: Amount > $10k → 2 approvals
    
    WORKFLOW->>APPROVER1: Request Approval
    Approver1->>APPROVER1: Review Invoice
    APPROVER1-->>WORKFLOW: Approve
    WORKFLOW->>APPROVER2: Request Approval
    Approver2->>Approver2: Review Invoice
    APPROVER2-->>WORKFLOW: Approve
    
    WORKFLOW->>INV: All Approvals Complete
    INV->>INV: Mark Invoice Approved
    INV->>GL: Post Journal Entry
    GL-->>INV: Entry Posted
    INV-->>Clerk: Invoice Posted
    
    Note over WORKFLOW,APPROVER2: Audit trail recorded
    WORKFLOW->>AUDIT: Record Approval Events
```

---

*Continue to [API Contracts](./06-api-contracts.md)*
