# Marketplace

## Overview

App marketplace and integration hub.

## Services

### Core
- **Path**: `marketplace/core/`
- **Description**: App marketplace service for discovering and installing extensions
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Integration Hub
- **Path**: `marketplace/integration-hub/`
- **Description**: Integration hub service providing pre-built connectors
- **Documentation**: [Integration Hub README](./integration-hub/README.md)
- **API Spec**: [Integration Hub OpenAPI](./integration-hub/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/marketplace` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The marketplace services work together to provide complete functionality:

*Integration patterns specific to marketplace will be documented as services are implemented.*
