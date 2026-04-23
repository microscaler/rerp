# Top-Level System Specs Plan

## Purpose

Each top-level system directory (e.g., `accounting/`, `sales/`, `crm/`) contains empty `openapi.yaml` and `README.md` files. These serve as **system-level aggregation points** for:

1. **System Gateway/Orchestration API** - Aggregated OpenAPI spec combining all sub-services
2. **System Documentation** - Overview of all services in the system and how they work together

## Plan

### 1. System-Level OpenAPI Specs (`openapi.yaml`)

**Purpose**: Create aggregated gateway/orchestration specs that:
- Combine all sub-service endpoints under a unified system API
- Provide routing/orchestration endpoints that delegate to sub-services
- Include system-level operations (health checks, metrics, service discovery)
- Use OpenAPI 3.1.0 `$ref` to include sub-service specs

**Structure**:
```yaml
openapi: 3.1.0
info:
  title: {System} API Gateway
  version: 1.0.0
  description: Aggregated API gateway for all {System} services
servers:
  - url: /api/v1/{system}
    description: {System} API Gateway
paths:
  # System-level operations
  /health:
    get:
      operationId: systemHealth
      summary: System health check
      responses:
        '200':
          description: System health status
          
  /services:
    get:
      operationId: listServices
      summary: List all services in system
      responses:
        '200':
          description: List of services
          
  # Delegated paths from sub-services
  # These would reference the actual sub-service specs
  /{module}/*:
    # Proxy/delegate to sub-service
```

**Implementation Options**:
- **Option A**: Use OpenAPI `$ref` to include sub-service specs (OpenAPI 3.1 supports this)
- **Option B**: Generate aggregated paths by reading all sub-service specs
- **Option C**: Create gateway routing endpoints that delegate to sub-services

### 2. System-Level README (`README.md`)

**Purpose**: Provide system-level documentation that:
- Overview of the system and its purpose
- List all sub-services with brief descriptions
- Explain how services work together
- Provide integration examples
- Link to individual service READMEs

**Structure**:
```markdown
# {System} Services

## Overview
Brief description of the system and its role in RERP.

## Services

### {Service 1}
- **Path**: `{system}/{module}/`
- **Description**: Brief description
- **Key Capabilities**: List key features
- **Documentation**: [Link to service README]

### {Service 2}
...

## Integration Patterns

How services in this system work together:
- Service orchestration flows
- Data flow between services
- Common use cases

## API Gateway

This system provides a unified API gateway at `/api/v1/{system}` that:
- Routes requests to appropriate sub-services
- Provides system-level operations
- Handles cross-service orchestration
```

## Implementation Strategy

### Phase 1: Generate System READMEs
1. Read all sub-service READMEs in the system
2. Extract service descriptions and capabilities
3. Generate system-level README with:
   - System overview
   - Service catalog
   - Integration patterns
   - Links to sub-services

### Phase 2: Generate System OpenAPI Specs
1. Read all sub-service OpenAPI specs
2. Generate aggregated spec with:
   - System-level endpoints (health, services, metrics)
   - Delegated paths to sub-services
   - Gateway routing configuration
   - Service discovery endpoints

### Phase 3: Gateway Implementation
1. Create gateway service that:
   - Loads system-level OpenAPI spec
   - Routes requests to sub-services
   - Provides orchestration capabilities
   - Handles cross-service transactions

## Example: Accounting System

### `accounting/README.md`
```markdown
# Accounting Services

## Overview
The Accounting system provides comprehensive financial management capabilities including general ledger, accounts payable/receivable, financial reporting, and compliance.

## Services

### General Ledger
- **Path**: `accounting/general-ledger/`
- **Description**: Core accounting service managing general ledger and journal entries
- **Key Capabilities**: Chart of accounts, journal entries, double-entry bookkeeping
- **Documentation**: [General Ledger README](./general-ledger/README.md)

### Accounts Payable
- **Path**: `accounting/accounts-payable/`
- **Description**: Accounts payable service managing vendor invoices and payments
- **Key Capabilities**: Vendor invoice processing, payment management, AP aging
- **Documentation**: [Accounts Payable README](./accounts-payable/README.md)

... (all 9 accounting services)

## Integration Patterns

The Accounting services work together to provide complete financial management:

1. **Invoice Flow**: 
   - `invoice/` creates invoices
   - `accounts-receivable/` manages customer invoices
   - `accounts-payable/` manages vendor invoices
   - `general-ledger/` records journal entries

2. **Financial Reporting**:
   - `general-ledger/` provides transaction data
   - `financial-reports/` generates P&L, balance sheets
   - `budget/` compares actuals vs budget

3. **Compliance**:
   - `edi/` handles electronic document exchange
   - `bank-sync/` imports bank statements
   - `asset/` tracks fixed assets
```

### `accounting/openapi.yaml`
```yaml
openapi: 3.1.0
info:
  title: Accounting API Gateway
  version: 1.0.0
  description: Aggregated API gateway for all Accounting services
servers:
  - url: /api/v1/accounting
    description: Accounting API Gateway
paths:
  /health:
    get:
      operationId: accountingHealth
      summary: Accounting system health check
      responses:
        '200':
          description: System health status
          content:
            application/json:
              schema:
                type: object
                properties:
                  status: {type: string}
                  services: 
                    type: array
                    items:
                      type: object
                      properties:
                        name: {type: string}
                        status: {type: string}
                        
  /services:
    get:
      operationId: listAccountingServices
      summary: List all accounting services
      responses:
        '200':
          description: List of services
          content:
            application/json:
              schema:
                type: array
                items:
                  type: object
                  properties:
                    name: {type: string}
                    path: {type: string}
                    description: {type: string}
                    
  # Gateway routing - delegate to sub-services
  /general-ledger/*:
    # Proxy to general-ledger service
    x-gateway-route:
      service: general-ledger
      basePath: /api/v1/accounting/general-ledger
      
  /accounts-payable/*:
    x-gateway-route:
      service: accounts-payable
      basePath: /api/v1/accounting/accounts-payable
      
  # ... other services
```

## Benefits

1. **Unified API Access**: Clients can access all system services through one gateway
2. **Service Discovery**: System-level endpoints for discovering available services
3. **Orchestration**: System-level operations that coordinate multiple services
4. **Documentation**: Clear overview of how services in a system work together
5. **Gateway Implementation**: Ready-to-use specs for API gateway services

## Next Steps

1. ✅ Create plan (this document)
2. ⏳ Generate system-level READMEs for all systems
3. ⏳ Generate system-level OpenAPI specs for all systems
4. ⏳ Implement gateway service using system specs
5. ⏳ Test gateway routing and orchestration

---

**Status**: Plan created - Ready for implementation
