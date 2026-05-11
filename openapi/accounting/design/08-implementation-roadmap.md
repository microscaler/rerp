# Implementation Roadmap

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Phased Delivery Plan

### Phase 1: Core Foundation (Weeks 1-8)

**Goal:** Establish the double-entry accounting engine with chart of accounts and journal entries.

```mermaid
gantt
    title Phase 1: Core Foundation
    dateFormat  YYYY-MM-DD
    axisFormat %Y-%m-%d
    
    section General Ledger
    Chart of Accounts Design    :done,    coa,   2026-05-01, 2026-05-14
    Account Schema & CRUD       :active,  acc,   2026-05-15, 2026-05-28
    Chart Template System       :         tmpl,  2026-05-28, 2026-06-04
    Journal Entry Schema        :         je,    2026-06-01, 2026-06-14
    Journal Entry CRUD          :         jecrud,2026-06-08, 2026-06-21
    Balance Verification        :         balance,2026-06-15,2026-06-28
    Period Management           :         period,2026-06-22,2026-07-05
    Double-Entry Engine         :         engine,2026-06-29,2026-07-12
    
    section Testing & Integration
    Unit Tests                  :         test,  2026-05-15, 2026-07-12
    Integration Tests           :         int,   2026-06-15, 2026-07-12
    BRRTRouter Lint             :         lint,  2026-07-10, 2026-07-12
```

**Deliverables:**
- General Ledger service with 93 schemas
- Chart of accounts with template support
- Journal entry creation with balance verification
- Fiscal period management
- All specs pass `brrtrouter-gen lint`

### Phase 2: Core Operations (Weeks 9-16)

**Goal:** Implement invoice processing, accounts payable, and accounts receivable.

```mermaid
gantt
    title Phase 2: Core Operations
    dateFormat  YYYY-MM-DD
    axisFormat %Y-%m-%d
    
    section Invoice Service
    Invoice Schema & Workflow   :         inv,   2026-07-01, 2026-07-21
    Approval Workflows          :         approv,2026-07-15,2026-08-04
    Invoice-to-GL Integration   :         glint, 2026-07-29,2026-08-18
    
    section Accounts Payable
    Vendor Invoice Management   :         apinv, 2026-08-01,2026-08-25
    Payment Processing          :         pay,   2026-08-15,2026-09-08
    Approval Integration        :         apappr,2026-08-29,2026-09-15
    
    section Accounts Receivable
    Customer Invoice Management :         arinv, 2026-08-01,2026-08-25
    Payment Applications        :         payapp,2026-08-15,2026-09-08
    Collection Management       :         collect,2026-09-01,2026-09-22
```

**Deliverables:**
- Invoice service with approval workflows
- Accounts payable with vendor payments
- Accounts receivable with collections
- Invoice-to-GL posting integration

### Phase 3: Financial Management (Weeks 17-24)

**Goal:** Add bank reconciliation, treasury management, budgeting, and reporting.

```mermaid
gantt
    title Phase 3: Financial Management
    dateFormat  YYYY-MM-DD
    axisFormat %Y-%m-%d
    
    section Bank Sync
    Bank Account Management     :         bank,  2026-09-01,2026-09-22
    Transaction Import          :         txn,   2026-09-15,2026-10-06
    Reconciliation Engine       :         recon, 2026-09-29,2026-10-20
    
    section Treasury
    Cash Position Tracking      :         cash,  2026-10-01,2026-10-21
    Cash Forecasting            :         forecast,2026-10-15,2026-11-03
    Liquidity Planning          :         liquidity,2026-10-29,2026-11-17
    
    section Budgeting
    Budget Schema & Lines       :         budget,2026-10-01,2026-10-28
    Forecast Generation         :         forecast,2026-11-01,2026-11-18
    
    section Reporting
    Trial Balance Report        :         tb,    2026-10-15,2026-11-04
    Balance Sheet               :         bs,    2026-10-29,2026-11-18
    Income Statement            :         is,    2026-11-01,2026-11-25
    Cash Flow Statement         :         cf,    2026-11-15,2026-12-09
```

### Phase 4: Advanced Accounting (Weeks 25-32)

**Goal:** Implement revenue recognition, tax compliance, lease accounting, and consolidation.

```mermaid
gantt
    title Phase 4: Advanced Accounting
    dateFormat  YYYY-MM-DD
    axisFormat %Y-%m-%d
    
    section Revenue Recognition
    Recognition Rules           :         rules, 2026-11-01,2026-11-25
    Deferred Revenue            :         deferred,2026-11-15,2026-12-09
    Recognition Schedule        :         schedule,2026-11-29,2026-12-23
    
    section Tax Compliance
    Tax Rules & Periods         :         taxrules,2026-12-01,2026-12-23
    Tax Returns                 :         returns,2026-12-15,2027-01-13
    Tax Payments                :         payments,2027-01-01,2027-01-20
    
    section Lease Accounting
    Lease Schema                :         lease, 2026-12-01,2026-12-29
    Lease Liability             :         liability,2027-01-05,2027-01-27
    Right-of-Use Assets         :         ROU,   2027-01-12,2027-02-03
    Payment Schedules           :         sched, 2027-01-19,2027-02-10
    
    section Consolidation
    Consolidation Groups        :         groups,2027-01-05,2027-01-27
    Elimination Rules           :         elim,  2027-01-19,2027-02-10
    Consolidation Runs          :         runs,  2027-02-01,2027-02-24
```

### Phase 5: Operations & Compliance (Weeks 33-40)

**Goal:** Implement asset management, audit controls, document extraction, and EDI.

```mermaid
gantt
    title Phase 5: Operations & Compliance
    dateFormat  YYYY-MM-DD
    axisFormat %Y-%m-%d
    
    section Asset Management
    Asset Schema & Categories   :         asset, 2027-02-01,2027-02-28
    Depreciation Engine         :         depr,  2027-02-15,2027-03-10
    Revaluation & Disposal      :         reval, 2027-03-01,2027-03-24
    
    section Audit Controls
    Audit Event Logging         :         audit, 2027-02-01,2027-02-28
    Approval Policies           :         policies,2027-03-01,2027-03-24
    Segregation of Duties       :         sod,   2027-03-08,2027-03-31
    
    section Documents Extraction
    OCR Integration             :         ocr,   2027-02-15,2027-03-17
    Document Classification     :         classify,2027-03-01,2027-03-24
    Auto-Pposting               :         autopost,2027-03-15,2027-04-07
    
    section EDI & Compliance
    EDI Profiles & Mappings     :         edi,   2027-03-01,2027-03-31
    Document Exchange           :         exchange,2027-03-15,2027-04-07
    Validation Profiles         :         validate,2027-03-29,2027-04-21
```

---

## Sprint Breakdown (Phase 1 Example)

### Sprint 1: GL Foundation (Weeks 1-2)

```mermaid
graph TB
    subgraph "Sprint 1 Tasks"
        S1A[Chart of Accounts Schema]
        S1B[Account CRUD Endpoints]
        S1C[Chart Template Data Model]
    end
    
    subgraph "Acceptance Criteria"
        ACC1[All accounts can be created]
        ACC2[Chart templates load]
        ACC3[Lint passes]
    end
    
    S1A --> S1B --> S1C --> ACC1
    S1C --> ACC2
    S1A --> ACC3
    S1B --> ACC3
    
    classDef task fill:#d4e6f1,stroke:#2980b9
    classDef accept fill:#d5f5e3,stroke:#27ae60
    
    class S1A,S1B,S1C task
    class ACC1,ACC2,ACC3 accept
```

### Sprint 2: Journal Entries (Weeks 3-4)

```mermaid
graph TB
    subgraph "Sprint 2 Tasks"
        S2A[Journal Entry Schema]
        S2B[Line Item Management]
        S2C[Balance Verification Logic]
    end
    
    subgraph "Acceptance Criteria"
        ACC4[Entries can be created]
        ACC5[Balance check enforces double-entry]
        ACC6[Unbalanced entries rejected]
    end
    
    S2A --> S2B --> S2C --> ACC4
    S2C --> ACC5
    S2C --> ACC6
    
    classDef task fill:#d4e6f1,stroke:#2980b9
    classDef accept fill:#d5f5e3,stroke:#27ae60
    
    class S2A,S2B,S2C task
    class ACC4,ACC5,ACC6 accept
```

### Sprint 3: Posting Engine (Weeks 5-6)

```mermaid
graph TB
    subgraph "Sprint 3 Tasks"
        S3A[Post/Unpost Operations]
        S3B[Bulk Operations Support]
        S3C[Period Locking Logic]
    end
    
    subgraph "Acceptance Criteria"
        ACC7[Entries can be posted]
        ACC8[Bulk post handles errors]
        ACC9[Closed periods reject entries]
    end
    
    S3A --> S3B --> S3C --> ACC7
    S3B --> ACC8
    S3C --> ACC9
    
    classDef task fill:#d4e6f1,stroke:#2980b9
    classDef accept fill:#d5f5e3,stroke:#27ae60
    
    class S3A,S3B,S3C task
    class ACC7,ACC8,ACC9 accept
```

### Sprint 4: Audit & Reporting (Weeks 7-8)

```mermaid
graph TB
    subgraph "Sprint 4 Tasks"
        S4A[Audit Trail Implementation]
        S4B[Trial Balance Report]
        S4C[Account Balances Endpoint]
    end
    
    subgraph "Acceptance Criteria"
        ACC10[All mutations logged]
        ACC11[Trial balance balances]
        ACC12[Balances queryable]
    end
    
    S4A --> ACC10
    S4B --> ACC11
    S4C --> ACC12
    S4A --> S4B --> S4C
    
    classDef task fill:#d4e6f1,stroke:#2980b9
    classDef accept fill:#d5f5e3,stroke:#27ae60
    
    class S4A,S4B,S4C task
    class ACC10,ACC11,ACC12 accept
```

---

## Implementation Priority Matrix

| Priority | Component | Effort | Impact | Rationale |
|----------|-----------|--------|--------|-----------|
| **P0** | General Ledger | High | Critical | Foundation — double-entry engine is core |
| **P0** | Chart of Accounts | Medium | Critical | Nothing works without account structure |
| **P1** | Invoice Service | High | Critical | Primary document for all transactions |
| **P1** | Accounts Payable | High | High | Core AP workflow |
| **P1** | Accounts Receivable | High | High | Core AR workflow |
| **P2** | Bank Sync | Medium | High | Cash visibility required early |
| **P2** | Financial Reports | High | High | Management needs reports to justify system |
| **P2** | Revenue Recognition | Medium | High | Required for SaaS/service businesses |
| **P3** | Tax Compliance | Medium | Medium | Can be phased; basic VAT first |
| **P3** | Asset Management | Medium | Medium | Important but not blocking |
| **P3** | Audit Controls | Low | Medium | Compliance requirement |
| **P4** | Lease Accounting | Medium | Low | Niche requirement |
| **P4** | Consolidation | High | Low | Multi-entity is advanced use case |
| **P4** | Documents Extraction | High | Low | Nice-to-have automation |
| **P4** | EDI & Compliance | High | Low | Partner-specific requirement |
| **P4** | Treasury | Medium | Low | Cash management can be phased |
| **P4** | Budget Management | Medium | Low | Planning feature, not transactional |

---

## Success Criteria

### Phase 1 Success Criteria

- [ ] General Ledger service passes all BRRTRouter lint checks
- [ ] Chart of accounts can be created and deployed from templates
- [ ] Journal entries can be created with automatic balance verification
- [ ] Unbalanced journal entries are rejected
- [ ] Fiscal periods can be opened, closed, and locked
- [ ] Trial balance report shows correct debits/credits
- [ ] All 93 schemas implemented and validated

### Phase 2 Success Criteria

- [ ] Invoice service supports approval workflows
- [ ] Vendor invoices can be created and paid
- [ ] Customer invoices can be created and collected
- [ ] Invoice posting creates balanced journal entries
- [ ] AP and AR aging reports generated correctly

### Phase 3 Success Criteria

- [ ] Bank transactions can be imported and matched
- [ ] Reconciliation identifies differences
- [ ] Cash position reflects all posted transactions
- [ ] Financial reports (BS, IS, CF) balance correctly

### Phase 4-5 Success Criteria

- [ ] Revenue recognized over time per schedules
- [ ] Tax returns can be prepared and filed
- [ ] Lease liabilities calculated per ASC 842/IFRS 16
- [ ] Multi-entity consolidation with eliminations
- [ ] Asset depreciation posted automatically
- [ ] Audit trail captures all sensitive operations
- [ ] EDI documents exchanged with trading partners

---

## Risk Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Schema complexity grows | High | Medium | Strict OpenAPI-first, review gates |
| Cross-service dependencies | High | High | Event-driven, async communication |
| Double-entry balance errors | Critical | Low | Automated balance verification |
| Performance at scale | Medium | Medium | Pagination, indexing, caching |
| Regulatory compliance gaps | High | Low | Audit trail, SoD enforcement |

---

*Continue to [Service Specifications](./10-service-specifications.md)*
