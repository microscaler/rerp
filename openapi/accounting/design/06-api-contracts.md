# API Contracts

> Part of RERP Accounting Suite Design
> See [main DESIGN.md](../DESIGN.md) for complete reference

---

## BFF Aggregation Pattern

```mermaid
graph TB
    subgraph "Client Request"
        CLIENT[Frontend Application]
    end
    
    subgraph "Accounting BFF"
        BFF[Accounting BFF Gateway]
        BFF --> PARSE[Parse & Validate Request]
        PARSE --> ROUTE[Route to Service]
        ROUTE --> AGGREGATE[Aggregate Responses]
        AGGREGATE --> FORMAT[Format Response]
    end
    
    subgraph "Microservices"
        GL[GL Service]
        AP[AP Service]
        AR[AR Service]
        REPORTS[Reports Service]
    end
    
    BFF --> GL
    BFF --> AP
    BFF --> AR
    BFF --> REPORTS
    
    FORMAT --> CLIENT
    
    classDef client fill:#fadbd8,stroke:#c0392b
    classDef bff fill:#d4e6f1,stroke:#2980b9
    classDef services fill:#d5f5e3,stroke:#27ae60
    
    class CLIENT client
    class BFF,PARSE,ROUTE,AGGREGATE,FORMAT bff
    class GL,AP,AR,REPORTS services
```

### BFF Request Routing

```mermaid
graph LR
    subgraph "Client Request"
        REQ[GET /api/v1/dashboard?company_id=xyz]
    end
    
    subgraph "BFF Routing"
        PARSE[Parse URL & Headers]
        PARSE --> IDENTIFY[Identify Required Services]
        IDENTIFY --> GL_REQ[GET /gl/balances]
        IDENTIFY --> INV_REQ[GET /invoices/summary]
        IDENTIFY --> AP_REQ[GET /payables/summary]
        IDENTIFY --> AR_REQ[GET /receivables/summary]
        
        GL_REQ --> GL[General Ledger]
        INV_REQ --> INV[Invoice Service]
        AP_REQ --> AP[Accounts Payable]
        AR_REQ --> AR[Accounts Receivable]
        
        GL --> GL_RESP[GL Response]
        INV --> INV_RESP[Invoice Response]
        AP --> AP_RESP[AP Response]
        AR --> AR_RESP[AR Response]
        
        GL_RESP --> AGG[Aggregate]
        INV_RESP --> AGG
        AP_RESP --> AGG
        AR_RESP --> AGG
    end
    
    AGG --> FINAL[Final Dashboard Response]
    
    classDef request fill:#fdebd0,stroke:#e67e22
    classDef route fill:#d4e6f1,stroke:#2980b9
    classDef service fill:#d5f5e3,stroke:#27ae60
    classDef response fill:#e8daef,stroke:#8e44ad
    
    class REQ request
    class PARSE,IDENTIFY,AGG route
    class GL,INV,AP,AR service
    class GL_RESP,INV_RESP,AP_RESP,AR_RESP,FINAL response
```

---

## Standard Response Format

### Error Response Schema

All services return consistent error responses:

```yaml
# Standard Error Response Schema
ErrorResponse:
  type: object
  required:
    - error_code
    - message
  properties:
    error_code:
      type: string
      example: "VALIDATION_ERROR"
    message:
      type: string
      example: "Invalid request: field 'amount' must be positive"
    details:
      type: array
      items:
        type: object
        properties:
          field:
            type: string
          message:
            type: string
```

### Standard Success Response (for list endpoints)

```yaml
# Standard Paginated Response
PaginatedResponse:
  type: object
  required:
    - total
    - page
    - limit
  properties:
    total:
      type: integer
      example: 150
    page:
      type: integer
      example: 1
    limit:
      type: integer
      example: 20
    items:
      type: array
      items:
        $ref: '#/components/schemas/EntitySchema'
```

### Service-Specific Pagination

Each service extends the base pagination:

```yaml
# Example: Paginated Vendor Invoices
PaginatedVendorInvoices:
  allOf:
    - $ref: '#/components/schemas/PaginatedResponse'
    - type: object
      properties:
        items:
          type: array
          items:
            $ref: '#/components/schemas/VendorInvoice'

# Example: Paginated Journal Entries
PaginatedJournalEntries:
  allOf:
    - $ref: '#/components/schemas/PaginatedResponse'
    - type: object
      properties:
        items:
          type: array
          items:
            $ref: '#/components/schemas/JournalEntry'
```

---

## OpenAPI Spec Structure

### Standard Spec Template

Each service `openapi.yaml` follows this structure:

```yaml
openapi: 3.1.0
info:
  title: Service Name
  version: 1.0.0
  description: Service description
  
servers:
  - url: https://{tenant}.{company}.rerp.local/api/v1

security:
  - bearerAuth: []

paths:
  /entities:
    get:
      operationId: listEntities
      parameters:
        - $ref: '#/components/parameters/CompanyId'
        - $ref: '#/components/parameters/Page'
        - $ref: '#/components/parameters/Limit'
      responses:
        '200':
          description: Paginated list
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/PaginatedEntities'
    post:
      operationId: createEntity
      x-brrtrouter-impl: true
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateEntityRequest'
      responses:
        '201':
          description: Entity created
        '400':
          description: Validation error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '401':
          description: Unauthorized
        '403':
          description: Forbidden
        '409':
          description: Conflict
```

### Standard Parameters

All services share these parameters:

```yaml
components:
  parameters:
    CompanyId:
      name: X-Company-ID
      in: header
      required: true
      schema:
        type: string
        format: uuid
        example: "550e8400-e29b-41d4-a716-446655440000"
    
    TenantId:
      name: X-Tenant-ID
      in: header
      required: true
      schema:
        type: string
        format: uuid
        example: "650e8400-e29b-41d4-a716-446655440001"
    
    Page:
      name: page
      in: query
      schema:
        type: integer
        default: 1
        minimum: 1
    
    Limit:
      name: limit
      in: query
      schema:
        type: integer
        default: 20
        minimum: 1
        maximum: 100
    
    Search:
      name: search
      in: query
      schema:
        type: string
        example: "invoice number:INV-001"
    
    Id:
      name: id
      in: path
      required: true
      schema:
        type: string
        format: uuid
```

### Security Scheme

```yaml
components:
  securitySchemes:
    httpBearer:
      type: http
      scheme: bearer
      bearerFormat: JWT
      description: JWT token from authentication service
```

---

## API Versioning Strategy

```mermaid
graph LR
    subgraph "API Versions"
        V1[API v1<br/>Current]
        V2[API v2<br/>Future]
    end
    
    subgraph "Version Control"
        URL[v1/]
        URL2[v2/]
        DEFAULT[Default: v1]
    end
    
    V1 --> URL
    V2 --> URL2
    URL --> DEFAULT
    URL2 --> DEFAULT
    
    classDef current fill:#d5f5e3,stroke:#27ae60
    classDef future fill:#d4e6f1,stroke:#2980b9
    classDef version fill:#fdebd0,stroke:#e67e22
    
    class V1 current
    class V2 future
    class URL,URL2,DEFAULT version
```

### Versioning Rules

1. **URL Versioning**: `/api/v1/resources`
2. **Backward Compatible Changes**: New fields, new endpoints
3. **Breaking Changes**: New major version (v2)
4. **Deprecation**: Headers (`X-Deprecation: true`, `Sunset: 2027-01-01`)
5. **Lifecycle**: Min 18 months support for active version

---

## Request/Response Examples

### Create Invoice Request

```json
{
  "vendor_id": "550e8400-e29b-41d4-a716-446655440000",
  "invoice_number": "INV-2026-001",
  "invoice_date": "2026-05-11",
  "due_date": "2026-06-11",
  "currency": "USD",
  "amount": 5000.00,
  "tax_amount": 500.00,
  "total_amount": 5500.00,
  "line_items": [
    {
      "description": "Consulting Services",
      "quantity": 1,
      "unit_price": 5000.00,
      "account_code": "5100"
    }
  ],
  "notes": "Project alpha phase 1"
}
```

### Create Invoice Response

```json
{
  "id": "650e8400-e29b-41d4-a716-446655440001",
  "vendor_id": "550e8400-e29b-41d4-a716-446655440000",
  "invoice_number": "INV-2026-001",
  "status": "pending_approval",
  "amount": 5000.00,
  "tax_amount": 500.00,
  "total_amount": 5500.00,
  "currency": "USD",
  "created_at": "2026-05-11T10:30:00Z",
  "approval_status": "pending"
}
```

### Error Response

```json
{
  "error_code": "VALIDATION_ERROR",
  "message": "Invalid request: field 'amount' must be positive",
  "details": [
    {
      "field": "amount",
      "message": "Amount must be greater than zero"
    }
  ]
}
```

---

*Continue to [Integration Patterns](./07-integration-patterns.md)*
