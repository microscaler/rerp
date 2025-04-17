# Field Service

## Overview

Field service management and technician dispatch.

## Services

### Core
- **Path**: `field-service/core/`
- **Description**: Field service management service handling work order scheduling
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/field-service` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The field service services work together to provide complete functionality:

*Integration patterns specific to field service will be documented as services are implemented.*
