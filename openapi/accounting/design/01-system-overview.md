# System Overview & Architecture

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Architecture Principles

### OpenAPI-First Design

Every service is defined in OpenAPI 3.1.0 before implementation. The spec is the source of truth.

```mermaid
graph LR
    A[Domain Expert] --> B[OpenAPI Spec<br/>YAML]
    B --> C[brrtrouter-gen]
    C --> D[Generated Rust<br/>Routers/Types]
    C --> E[TypeScript SDK]
    C --> F[Client Libraries]
    B --> G[BFF Generator]
    G --> H[Suite BFF Spec]
    
    classDef spec fill:#d5f5e3,stroke:#27ae60
    classDef gen fill:#d4e6f1,stroke:#2980b9
    class B spec
    class C,D,E,F,G,H gen
```

### BRRRouter Convention Compliance

All specs adhere to strict conventions:

```mermaid
graph TB
    A[openapi.yaml] --> B{Validation}
    B -->|Pass| C[cargo run --bin brrtrouter-gen lint]
    B -->|Fail| D[Fix Spec]
    D --> B
    C --> E[cargo run --bin brrtrouter-gen generate]
    E --> F[gen/ crate<br/>Types + Routers]
    E --> G[impl/ crate<br/>Business Logic]
    
    classDef check fill:#fadbd8,stroke:#c0392b
    class B,D check
```

**Key Conventions:**
- `components` contains exactly 3 keys: `securitySchemes`, `parameters`, `schemas`
- Global security: `bearerAuth: []` referencing `httpBearer`
- List endpoints return `PaginatedResponse` → `Paginated<Entity>`
- All mutations (POST/PUT/PATCH) have `x-brrtrouter-impl: true`
- All mutations include `400`, `401`, `403`, `409` error responses

### Database Strategy

```mermaid
graph TB
    subgraph "Shared PostgreSQL Cluster"
        DB[PostgreSQL 16<br/>Primary]
        
        subgraph "Schema-per-Service"
            SCHEMA_GL[general_ledger schema]
            SCHEMA_AP[accounts_payable schema]
            SCHEMA_AR[accounts_receivable schema]
            SCHEMA_INV[invoice schema]
            SCHEMA_OTHER[other 12 schemas]
        end
        
        DB --> SCHEMA_GL
        DB --> SCHEMA_AP
        DB --> SCHEMA_AR
        DB --> SCHEMA_INV
        DB --> SCHEMA_OTHER
    end
    
    subgraph "Cross-Service References"
        FK[Fake Foreign Keys<br/>via application logic<br/>NOT database-level]
        PK[(Primary Key)<br/>company_id + id<br/>composite key]
    end
    
    SCHEMA_GL -.->|company_id join| SCHEMA_AP
    SCHEMA_AP -.->|invoice_id| SCHEMA_INV
    SCHEMA_AR -.->|invoice_id| SCHEMA_INV
    
    classDef db fill:#d4e6f1,stroke:#2980b9
    classDef schema fill:#d5f5e3,stroke:#27ae60
    classDef cross fill:#fdebd0,stroke:#e67e22
    
    class DB db
    class SCHEMA_GL,SCHEMA_AP,SCHEMA_AR,SCHEMA_INV,SCHEMA_OTHER schema
    class FK,PK cross
```

**Database Rules:**
- Each service owns its schema exclusively
- No database-level foreign keys between schemas (services are independent)
- Cross-service references are application-level via `company_id` joins
- Shared domain entities (Company, User, Tenant) live in a shared `entities` crate

### Communication Patterns

```mermaid
graph LR
    subgraph "Synchronous"
        REST[REST API<br/>JSON/HTTP]
        gRPC[gRPC<br/>Internal]
    end
    
    subgraph "Asynchronous"
        EVENT[Event Bus<br/>PostgreSQL LISTEN/NOTIFY]
        MSG[Message Queue<br/>RabbitMQ/Redis Streams]
    end
    
    BFF --> REST
    GL -- invoice_created --> EVENT
    EVENT --> AP
    EVENT --> AR
    GL -- period_closed --> MSG
    MSG --> REPORTS
    MSG --> CONSOL
    
    classDef sync fill:#e8daef,stroke:#8e44ad
    classDef async fill:#fadbd8,stroke:#c0392b
    class REST,gRPC sync
    class EVENT,MSG async
```

---

## Service Inventory

| # | Service | OpenAPI Spec | Schema Count | Path Count | Tier |
|---|---------|--------------|--------------|------------|------|
| 1 | General Ledger | `general-ledger/openapi.yaml` | 93 | 44 | Foundation |
| 2 | Accounts Payable | `accounts-payable/openapi.yaml` | 28 | 12 | Core |
| 3 | Accounts Receivable | `accounts-receivable/openapi.yaml` | 39 | 18 | Core |
| 4 | Invoice | `invoice/openapi.yaml` | 24 | 17 | Core |
| 5 | Revenue Recognition | `revenue-recognition/openapi.yaml` | 15 | 6 | Advanced |
| 6 | Tax Compliance | `tax-compliance/openapi.yaml` | 21 | 7 | Advanced |
| 7 | Lease Accounting | `lease-accounting/openapi.yaml` | 15 | 6 | Advanced |
| 8 | Consolidation | `consolidation/openapi.yaml` | 16 | 6 | Advanced |
| 9 | Bank Sync | `bank-sync/openapi.yaml` | 29 | 18 | Management |
| 10 | Treasury & Cash | `treasury/openapi.yaml` | 16 | 5 | Management |
| 11 | Budget Management | `budget/openapi.yaml` | 19 | 11 | Management |
| 12 | Financial Reports | `financial-reports/openapi.yaml` | 29 | 14 | Management |
| 13 | Asset Management | `asset/openapi.yaml` | 27 | 14 | Operations |
| 14 | Audit Controls | `audit-controls/openapi.yaml` | 17 | 6 | Operations |
| 15 | Documents Extraction | `documents-extraction/openapi.yaml` | 17 | 7 | Operations |
| 16 | EDI & Compliance | `edi/openapi.yaml` | 26 | 12 | Operations |

**Totals:** 431 schemas, 203 paths, 16 services

---

## Service Topology

### Dependency Graph

```mermaid
graph TB
    subgraph "Dependency Tier 0 — Foundation"
        GL[General Ledger<br/>Chart of Accounts<br/>Journal Entries<br/>Double-Entry Engine]
    end
    
    subgraph "Dependency Tier 1 — Core Operations"
        AP[Accounts Payable<br/>Vendor Invoices<br/>Payments]
        AR[Accounts Receivable<br/>Customer Invoices<br/>Collections]
        INV[Invoice Service<br/>Invoice Workflows<br/>Approvals]
    end
    
    subgraph "Dependency Tier 2 — Advanced"
        REV[Revenue Recognition<br/>Deferred Revenue<br/>Recognition Rules]
        TAX[Tax Compliance<br/>Tax Returns<br/>Period Reporting]
        LEASE[Lease Accounting<br/>Lease Liability<br/>Right-of-Use Assets]
        CONSOL[Consolidation<br/>Multi-Entity<br/>Elimination Rules]
    end
    
    subgraph "Dependency Tier 3 — Management"
        BANK[Bank Sync<br/>Reconciliation<br/>Transactions]
        TREASURY[Treasury<br/>Cash Forecast<br/>Liquidity Planning]
        BUDGET[Budget Management<br/>Budget Lines<br/>Forecasts]
        REPORTS[Financial Reports<br/>Balance Sheet<br/>P&L, Cash Flow]
    end
    
    subgraph "Dependency Tier 4 — Operations"
        ASSET[Asset Management<br/>Depreciation<br/>Categories]
        AUDIT[Audit Controls<br/>Audit Events<br/>Segregation of Duties]
        DOC[Documents Extraction<br/>OCR Processing<br/>Document Classification]
        EDI[EDI & Compliance<br/>Document Exchange<br/>Mapping]
    end
    
    GL -->|chart_template_id| INV
    AP -->|invoice_id| INV
    AP -->|gl_posting| GL
    AR -->|invoice_id| INV
    AR -->|gl_posting| GL
    INV -->|gl_posting| GL
    
    REV -->|gl_posting| GL
    TAX -->|gl_posting| GL
    LEASE -->|gl_posting| GL
    LEASE -->|revaluation| ASSET
    
    BANK -->|cash_position| TREASURY
    BANK -->|reconciliation| GL
    BUDGET -->|variance| REPORTS
    GL -->|trial_balance| REPORTS
    GL -->|consolidation_data| CONSOL
    
    ASSET -->|depreciation| GL
    AUDIT -->|all_services| GL
    
    DOC -->|auto_posting| AP
    DOC -->|auto_posting| AR
    EDI -->|document_exchange| AP
    EDI -->|document_exchange| AR
    
    classDef foundation fill:#d5f5e3,stroke:#27ae60,stroke-width:3px
    classDef core fill:#d4e6f1,stroke:#2980b9
    classDef advanced fill:#fdebd0,stroke:#e67e22
    classDef management fill:#e8daef,stroke:#8e44ad
    classDef ops fill:#fadbd8,stroke:#c0392b
    
    class GL foundation
    class AP,AR,INV core
    class REV,TAX,LEASE,CONSOL advanced
    class BANK,TREASURY,BUDGET,REPORTS management
    class ASSET,AUDIT,DOC,EDI ops
```

### Service Interaction Matrix

```mermaid
graph LR
    subgraph "Primary Data Flow"
        INV -->|creates| AR
        INV -->|creates| AP
        INV -->|posts to| GL
        AP -->|pays| INV
        AR -->|collects| INV
        GL -->|generates| REPORTS
        GL -->|supports| BUDGET
        BANK -->|syncs to| GL
        TREASURY -->|uses| GL
    end
    
    INV --> REV
    GL --> TAX
    INV --> LEASE
    GL --> CONSOL
    AP --> DOC
    AR --> DOC
    ASSET --> GL
    AUDIT --> GL
    EDI --> INV
    
    classDef flow fill:#d5f5e3,stroke:#27ae60
    class INV,AR,AP,GL,REPORTS,BUDGET,BANK,TREASURY,REV,TAX,LEASE,CONSOL,DOC,ASSET,AUDIT,EDI flow
```

---

## System Overview

```mermaid
graph TB
    subgraph "External Systems"
        ERP[Other RERP Suites<br/>Sales, Purchasing, Inventory]
        Bank[Bank APIs<br/>Open Banking/Plaid]
        Tax[Tax Authority APIs<br/>VAT/GST/Sales Tax]
        EDI[EDI Partners<br/>Trading Partners]
        BI[BI/Analytics<br/>Looker/Tableau]
    end
    
    subgraph "RERP Accounting Suite"
        BFF[Accounting BFF<br/>Gateway Service]
        
        subgraph "Core Accounting"
            GL[General Ledger<br/>Service]
            AP[Accounts Payable<br/>Service]
            AR[Accounts Receivable<br/>Service]
            INV[Invoice Service]
        end
        
        subgraph "Advanced Accounting"
            REV[Revenue Recognition<br/>Service]
            TAX[Tax Compliance<br/>Service]
            LEASE[Lease Accounting<br/>Service]
            CONSOL[Consolidation<br/>Service]
        end
        
        subgraph "Financial Management"
            BANK[Bank Sync<br/>Service]
            TREASURY[Treasury & Cash<br/>Service]
            BUDGET[Budget Management<br/>Service]
            REPORTS[Financial Reports<br/>Service]
        end
        
        subgraph "Operations & Compliance"
            ASSET[Asset Management<br/>Service]
            AUDIT[Audit Controls<br/>Service]
            DOC[Documents Extraction<br/>Service]
            EDI_SVC[EDI & Compliance<br/>Service]
        end
    end
    
    ERP --> BFF
    Bank --> BANK
    Tax --> TAX
    EDI --> EDI_SVC
    BFF --> GL
    BFF --> AP
    BFF --> AR
    BFF --> INV
    GL --> REV
    GL --> TAX
    AP --> INV
    AR --> INV
    REV --> REPORTS
    BANK --> TREASURY
    GL --> BUDGET
    GL --> REPORTS
    ASSET --> GL
    AUDIT --> GL
    DOC --> AP
    DOC --> AR
    EDI_SVC --> INV
    
    classDef external fill:#f9e3d4,stroke:#d4894a
    classDef bff fill:#d4e6f1,stroke:#2980b9
    classDef core fill:#d5f5e3,stroke:#27ae60
    classDef advanced fill:#fdebd0,stroke:#e67e22
    classDef financial fill:#e8daef,stroke:#8e44ad
    classDef ops fill:#fadbd8,stroke:#c0392b
    
    class ERP,Bank,Tax,EDI,BI external
    class BFF bff
    class GL,AP,AR,INV core
    class REV,TAX,LEASE,CONSOL advanced
    class BANK,TREASURY,BUDGET,REPORTS financial
    class ASSET,AUDIT,DOC,EDI_SVC ops
```

---

*Continue to [Domain Model](./02-domain-model.md)*
