# Procurement

## Overview

Purchase order management and vendor relationship management.

## Services

### Core
- **Path**: `purchase/core/`
- **Description**: Purchase order management service handling PO creation and approval workflows
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Vendor
- **Path**: `purchase/vendor/`
- **Description**: Vendor and supplier management service managing vendor profiles and performance
- **Documentation**: [Vendor README](./vendor/README.md)
- **API Spec**: [Vendor OpenAPI](./vendor/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/purchase` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The procurement services work together to provide complete functionality:

*Integration patterns specific to procurement will be documented as services are implemented.*
