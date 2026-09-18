# Security & Multi-Tenancy

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## Tenancy Model

### Tenant Isolation Architecture

```mermaid
graph TB
    subgraph "Tenant Isolation Layers"
        subgraph "Network Layer"
            DOMAIN[Domain Routing<br/>tenant.company.rerp.local]
            DOMAIN --> SSL[SSL/TLS Termination]
        end
        
        subgraph "Authentication Layer"
            AUTH[JWT Validation]
            AUTH --> TENANT[Extract Tenant ID]
            TENANT --> COMPANY[Extract Company ID]
        end
        
        subgraph "Database Layer"
            DB[PostgreSQL]
            subgraph "Schema Isolation"
                SCHEMA1[Schema: tenant_1_*]
                SCHEMA2[Schema: tenant_2_*]
                SCHEMA3[Schema: tenant_3_*]
            end
            DB --> SCHEMA1
            DB --> SCHEMA2
            DB --> SCHEMA3
        end
        
        subgraph "Row-Level Security"
            RLS[Row-Level Security<br/>company_id filter]
            RLS --> VERIFY[JWT Validation]
            VERIFY --> TENANT_CTX[Session Context]
            TENANT_CTX --> FILTER[WHERE clause injection]
        end
    end
    
    SCHEMA1 -.->|RLS Policy| RLS
    SCHEMA2 -.->|RLS Policy| RLS
    SCHEMA3 -.->|RLS Policy| RLS
    
    SSL --> AUTH
    AUTH --> DB
    
    classDef network fill:#d4e6f1,stroke:#2980b9
    classDef auth fill:#e8daef,stroke:#8e44ad
    classDef db fill:#d5f5e3,stroke:#27ae60
    classDef security fill:#fadbd8,stroke:#c0392b
    
    class DOMAIN,SSL network
    class AUTH,TENANT,COMPANY auth
    class DB,SCHEMA1,SCHEMA2,SCHEMA3 db
    class RLS,VERIFY,TENANT_CTX,FILTER security
```

### Multi-Tenant Data Flow

```mermaid
sequenceDiagram
    participant User
    participant Gateway
    participant Auth
    participant Service
    participant DB
    
    User->>Gateway: GET /api/v1/invoices
    Note over User,Gateway: Header: X-Tenant-ID<br/>Header: X-Company-ID
    Gateway->>Auth: Validate JWT
    Auth-->>Gateway: Token valid + claims
    Gateway->>Service: Forward Request<br/>+ Tenant ID<br/>+ Company ID
    Service->>Service: Set session context
    Service->>DB: Query with RLS
    Note over DB: SET LOCAL rerp.tenant_id = '<uuid>'<br/>SET LOCAL rerp.company_id = '<uuid>'
    DB-->>Service: Filtered Results
    Service-->>Gateway: Response
    Gateway-->>User: Data
    
    rect rgb(248, 240, 232)
        Note over DB: All queries automatically<br/>filtered by tenant/company
    end
```

### Tenant Hierarchy

```mermaid
graph TB
    GROUP[Corporate Group]
    
    GROUP --> CO_A[Company A]
    GROUP --> CO_B[Company B]
    GROUP --> CO_C[Company C]
    
    CO_A --> TEN_A1[Tenant A1]
    CO_A --> TEN_A2[Tenant A2]
    CO_B --> TEN_B1[Tenant B1]
    CO_C --> TEN_C1[Tenant C1]
    
    TEN_A1 --> SCHEMA_A1[Schema: a1_*]
    TEN_A2 --> SCHEMA_A2[Schema: a2_*]
    TEN_B1 --> SCHEMA_B1[Schema: b1_*]
    TEN_C1 --> SCHEMA_C1[Schema: c1_*]
    
    classDef group fill:#d4e6f1,stroke:#2980b9,stroke-width:3px
    classDef company fill:#d5f5e3,stroke:#27ae60
    classDef tenant fill:#e8daef,stroke:#8e44ad
    classDef schema fill:#fdebd0,stroke:#e67e22
    
    class GROUP group
    class CO_A,CO_B,CO_C company
    class TEN_A1,TEN_A2,TEN_B1,TEN_C1 tenant
    class SCHEMA_A1,SCHEMA_A2,SCHEMA_B1,SCHEMA_C1 schema
```

---

## Authentication Flow

```mermaid
sequenceDiagram
    participant User
    participant Frontend
    participant BFF
    participant Auth
    participant Gateway
    
    User->>Frontend: Login
    Frontend->>Auth: POST /auth/login
    Auth-->>Frontend: JWT Token
    Note over Frontend: Token contains:<br/>- tenant_id<br/>- company_id<br/>- user_id<br/>- roles
    
    Frontend->>BFF: GET /api/v1/invoices
    Note over Frontend,BFF: Authorization: Bearer <token>
    BFF->>BFF: Validate JWT signature
    BFF->>BFF: Extract claims
    BFF->>BFF: Set session context
    BFF->>GL: Forward Request<br/>+ X-Tenant-ID<br/>+ X-Company-ID
    GL->>GL: Validate tenant access
    GL-->>BFF: Response (filtered)
    BFF-->>Frontend: Response
    Frontend-->>User: Display Data
```

### JWT Token Structure

```json
{
  "sub": "user-uuid",
  "tenant_id": "tenant-uuid",
  "company_id": "company-uuid",
  "roles": ["accountant", "viewer"],
  "permissions": [
    "invoices:read",
    "invoices:write",
    "journal_entries:read",
    "reports:read"
  ],
  "iat": 1715452800,
  "exp": 1715456400
}
```

---

## Role-Based Access Control

### RBAC Hierarchy

```mermaid
graph TB
    subgraph "Role Hierarchy"
        ROLE_ADMIN[Administrator<br/>Full Access]
        ROLE_ACC_MGR[Accounting Manager<br/>Approve/Post/Close]
        ROLE_ACCOUNTANT[Accountant<br/>Create/Edit/Submit]
        ROLE_CLERK[Clerk<br/>Create Only]
        ROLE_VIEWER[Viewer<br/>Read Only]
    end
    
    subgraph "Permissions Matrix"
        subgraph "GL Permissions"
            GL_READ[Read Accounts]
            GL_WRITE[Write Accounts]
            GL_POST[Post Entries]
            GL_APPROVE[Approve Entries]
            GL_CLOSE[Close Periods]
        end
        
        subgraph "AP Permissions"
            AP_CREATE[Create Invoices]
            AP_APPROVE[Approve Invoices]
            AP_PAY[Process Payments]
        end
        
        subgraph "AR Permissions"
            AR_CREATE[Create Invoices]
            AR_COLLECT[Record Collections]
            AR_WRITEOFF[Write Off]
        end
        
        subgraph "Report Permissions"
            REPORT_READ[View Reports]
            REPORT_EXPORT[Export Reports]
        end
    end
    
    ROLE_ADMIN --> GL_READ
    ROLE_ADMIN --> GL_WRITE
    ROLE_ADMIN --> GL_POST
    ROLE_ADMIN --> GL_APPROVE
    ROLE_ADMIN --> GL_CLOSE
    ROLE_ADMIN --> AP_CREATE
    ROLE_ADMIN --> AP_APPROVE
    ROLE_ADMIN --> AP_PAY
    ROLE_ADMIN --> AR_CREATE
    ROLE_ADMIN --> AR_COLLECT
    ROLE_ADMIN --> REPORT_READ
    ROLE_ADMIN --> REPORT_EXPORT
    
    ROLE_ACC_MGR --> GL_READ
    ROLE_ACC_MGR --> GL_POST
    ROLE_ACC_MGR --> GL_APPROVE
    ROLE_ACC_MGR --> GL_CLOSE
    ROLE_ACC_MGR --> AP_CREATE
    ROLE_ACC_MGR --> AP_APPROVE
    ROLE_ACC_MGR --> AR_CREATE
    ROLE_ACC_MGR --> AR_COLLECT
    ROLE_ACC_MGR --> REPORT_READ
    ROLE_ACC_MGR --> REPORT_EXPORT
    
    ROLE_ACCOUNTANT --> GL_READ
    ROLE_ACCOUNTANT --> GL_WRITE
    ROLE_ACCOUNTANT --> AP_CREATE
    ROLE_ACCOUNTANT --> AR_CREATE
    ROLE_ACCOUNTANT --> REPORT_READ
    
    ROLE_CLERK --> GL_READ
    ROLE_CLERK --> AP_CREATE
    ROLE_CLERK --> AR_CREATE
    ROLE_CLERK --> REPORT_READ
    
    ROLE_VIEWER --> GL_READ
    ROLE_VIEWER --> REPORT_READ
    
    classDef admin fill:#fadbd8,stroke:#c0392b,stroke-width:3px
    classDef manager fill:#fdebd0,stroke:#e67e22
    classDef accountant fill:#d5f5e3,stroke:#27ae60
    classDef clerk fill:#d4e6f1,stroke:#2980b9
    classDef viewer fill:#e8daef,stroke:#8e44ad
    
    class ROLE_ADMIN admin
    class ROLE_ACC_MGR manager
    class ROLE_ACCOUNTANT accountant
    class ROLE_CLERK clerk
    class ROLE_VIEWER viewer
```

### Permission Enforcement

```mermaid
graph TB
    subgraph "Permission Check Flow"
        REQUEST[API Request]
        REQUEST --> TOKEN[Extract JWT]
        TOKEN --> ROLE[Identify Role]
        ROLE --> CHECK{Has Permission?}
        
        CHECK -->|Yes| ALLOW[Allow Request]
        CHECK -->|No| DENY[Return 403]
        
        ALLOW --> SERVICE[Process Request]
        SERVICE --> RESPONSE[Return Response]
    end
    
    subgraph "Service-Level Checks"
        GL_CHK[GL: Entry Status<br/>Closed → Deny Write]
        AP_CHK[AP: Invoice Status<br/>Posted → Deny Edit]
        AR_CHK[AR: Payment Status<br/>Applied → Deny Cancel]
    end
    
    DENY --> GL_CHK
    DENY --> AP_CHK
    DENY --> AR_CHK
    
    classDef flow fill:#d4e6f1,stroke:#2980b9
    classDef check fill:#fadbd8,stroke:#c0392b
    classDef service fill:#d5f5e3,stroke:#27ae60
    
    class REQUEST,TOKEN,ROLE,CHECK,ALLOW,DENY,RESPONSE flow
    class CHECK check
    class GL_CHK,AP_CHK,AR_CHK service
```

---

## Segregation of Duties

### SoD Matrix

```mermaid
graph TB
    subgraph "Segregation Rules"
        RULE1[Create ≠ Approve<br/>Same person cannot<br/>create and approve]
        RULE2[Approve ≠ Post<br/>Same person cannot<br/>approve and post]
        RULE3[Request ≠ Approve<br/>Same person cannot<br/>request and approve payment]
        RULE4[Reconcile ≠ Approve<br/>Same person cannot<br/>reconcile and approve]
    end
    
    subgraph "Enforcement"
        AUDIT[Audit Controls Service]
        AUDIT --> LOG[Log Violation Attempts]
        LOG --> ALERT[Alert Compliance]
        ALERT --> BLOCK[Block Transaction]
    end
    
    RULE1 --> AUDIT
    RULE2 --> AUDIT
    RULE3 --> AUDIT
    RULE4 --> AUDIT
    
    classDef rule fill:#fadbd8,stroke:#c0392b
    classDef enforce fill:#d5f5e3,stroke:#27ae60
    
    class RULE1,RULE2,RULE3,RULE4 rule
    class AUDIT,LOG,ALERT,BLOCK enforce
```

### SoD Check Flow

```mermaid
sequenceDiagram
    participant User
    participant API
    participant AUDIT as Audit Controls
    participant SERVICE as Target Service
    
    User->>API: POST /invoices/{id}/approve
    API->>API: Extract user_id
    API->>AUDIT: Check SoD Rule
    AUDIT->>AUDIT: Verify user didn't<br/>create this invoice
    alt SoD Violation
        AUDIT-->>API: Violation Detected
        API-->>User: 403 Forbidden<br/>Segregation of Duties
        AUDIT->>AUDIT: Log Attempt
    else SoD Compliant
        AUDIT-->>API: Compliant
        API->>SERVICE: Process Request
        SERVICE-->>API: Success
        API-->>User: Invoice Approved
    end
```

---

## Audit Trail

### Audit Event Model

```mermaid
erDiagram
    AuditEvent ||--|| User : "created by"
    AuditEvent ||--|| CompanyAccount : "pertains to"
    AuditEvent }o--|| AuditEvent : "parent event"
    
    AuditEvent {
        uuid id PK
        uuid user_id FK
        uuid company_account_id FK
        string event_type
        string resource_type
        uuid resource_id
        string action
        jsonb old_values
        jsonb new_values
        string ip_address
        string user_agent
        datetime occurred_at
        string status
    }
    
    User {
        uuid id PK
        string username
        string email
    }
    
    CompanyAccount {
        uuid id PK
        string name
        string fiscal_year
    }
```

### Audit Event Types

```mermaid
graph TB
    subgraph "Audit Event Categories"
        subgraph "Authentication"
            LOGIN[User Login]
            LOGOUT[User Logout]
            LOGIN_FAIL[Failed Login]
            TOKEN_REFRESH[Token Refresh]
        end
        
        subgraph "Data Operations"
            CREATE[Entity Created]
            UPDATE[Entity Updated]
            DELETE[Entity Deleted]
            APPROVE[Entity Approved]
            POST[Entry Posted]
            CLOSE[Period Closed]
        end
        
        subgraph "Security"
            SO_D[Violation Detected]
            ACCESS[Unauthorized Access]
            EXPORT[Report Exported]
            OVERRIDE[Policy Override]
        end
    end
    
    subgraph "Event Storage"
        TABLE[Audit Events Table]
        TABLE --> INDEX[Indexed by:<br/>- user_id<br/>- event_type<br/>- occurred_at<br/>- resource_type]
    end
    
    LOGIN & LOGOUT & LOGIN_FAIL & TOKEN_REFRESH --> TABLE
    CREATE & UPDATE & DELETE & APPROVE & POST & CLOSE --> TABLE
    SO_D & ACCESS & EXPORT & OVERRIDE --> TABLE
    
    classDef auth fill:#d4e6f1,stroke:#2980b9
    classDef data fill:#d5f5e3,stroke:#27ae60
    classDef security fill:#fadbd8,stroke:#c0392b
    classDef storage fill:#e8daef,stroke:#8e44ad
    
    class LOGIN,LOGOUT,LOGIN_FAIL,TOKEN_REFRESH auth
    class CREATE,UPDATE,DELETE,APPROVE,POST,CLOSE data
    class SO_D,ACCESS,EXPORT,OVERRIDE security
    class TABLE,INDEX storage
```

---

## Security Headers & Best Practices

### Required Headers

| Header | Purpose | Example |
|--------|---------|---------|
| `X-Company-ID` | Tenant scoping | `550e8400-e29b-41d4-a716-446655440000` |
| `X-Tenant-ID` | Company scoping | `650e8400-e29b-41d4-a716-446655440001` |
| `Authorization` | Bearer token | `Bearer eyJhbGciOi...` |
| `Content-Type` | Request format | `application/json` |
| `Accept` | Response format | `application/json` |

### Security Checklist

- [ ] All endpoints require authentication
- [ ] JWT tokens validated on every request
- [ ] Tenant/company context injected via session
- [ ] RLS policies enforce data isolation
- [ ] SoD rules validated on sensitive operations
- [ ] Audit events logged for all mutations
- [ ] Rate limiting applied per tenant
- [ ] CORS configured for approved origins
- [ ] HTTPS enforced (TLS 1.2+)
- [ ] Sensitive data encrypted at rest

---

*Continue to [Implementation Roadmap](./09-implementation-roadmap.md)*
