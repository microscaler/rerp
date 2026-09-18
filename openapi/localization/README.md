# Localization

## Overview

Country-specific configurations and compliance management.

## Services

### Core
- **Path**: `localization/core/`
- **Description**: Localization service providing country-specific configurations
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Compliance
- **Path**: `localization/compliance/`
- **Description**: Compliance service managing regulatory compliance and audit trails
- **Documentation**: [Compliance README](./compliance/README.md)
- **API Spec**: [Compliance OpenAPI](./compliance/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/localization` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The localization services work together to provide complete functionality:

*Integration patterns specific to localization will be documented as services are implemented.*
