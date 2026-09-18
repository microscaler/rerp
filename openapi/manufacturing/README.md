# Manufacturing

## Overview

Manufacturing operations from BOM management to production planning and repair.

## Services

### Core
- **Path**: `manufacturing/core/`
- **Description**: Core manufacturing service orchestrating production orders and BOM management
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Bom
- **Path**: `manufacturing/bom/`
- **Description**: Bill of Materials service managing product structures and multi-level BOMs
- **Documentation**: [Bom README](./bom/README.md)
- **API Spec**: [Bom OpenAPI](./bom/openapi.yaml)

### Production Planning
- **Path**: `manufacturing/production-planning/`
- **Description**: Production planning service handling production scheduling and MRP
- **Documentation**: [Production Planning README](./production-planning/README.md)
- **API Spec**: [Production Planning OpenAPI](./production-planning/openapi.yaml)

### Repair
- **Path**: `manufacturing/repair/`
- **Description**: Repair service managing product repairs and repair orders
- **Documentation**: [Repair README](./repair/README.md)
- **API Spec**: [Repair OpenAPI](./repair/openapi.yaml)

### Subcontracting
- **Path**: `manufacturing/subcontracting/`
- **Description**: Subcontracting service managing outsourced production
- **Documentation**: [Subcontracting README](./subcontracting/README.md)
- **API Spec**: [Subcontracting OpenAPI](./subcontracting/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/manufacturing` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The manufacturing services work together to provide complete functionality:

*Integration patterns specific to manufacturing will be documented as services are implemented.*
