# Document Management

## Overview

Document storage, organization, and version control.

## Services

### Core
- **Path**: `documents/core/`
- **Description**: Document management service for storing and organizing documents
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/documents` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The document management services work together to provide complete functionality:

*Integration patterns specific to document management will be documented as services are implemented.*
