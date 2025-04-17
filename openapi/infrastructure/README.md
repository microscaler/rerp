# Infrastructure

## Overview

API gateway and integration platform services.

## Services

### Gateway
- **Path**: `infrastructure/gateway/`
- **Description**: Unified API gateway providing routing, rate limiting, and authentication
- **Documentation**: [Gateway README](./gateway/README.md)
- **API Spec**: [Gateway OpenAPI](./gateway/openapi.yaml)

### Integration Platform
- **Path**: `infrastructure/integration-platform/`
- **Description**: Platform for managing third-party integrations and webhooks
- **Documentation**: [Integration Platform README](./integration-platform/README.md)
- **API Spec**: [Integration Platform OpenAPI](./integration-platform/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/infrastructure` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The infrastructure services work together to provide complete functionality:

*Integration patterns specific to infrastructure will be documented as services are implemented.*
