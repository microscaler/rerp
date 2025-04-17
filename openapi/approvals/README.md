# Approval Workflows

## Overview

Multi-level approval processes for business transactions.

## Services

### Core
- **Path**: `approvals/core/`
- **Description**: Approval workflow service managing multi-level approval processes
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/approvals` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The approval workflows services work together to provide complete functionality:

*Integration patterns specific to approval workflows will be documented as services are implemented.*
