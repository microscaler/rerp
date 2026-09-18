# Authentication & Authorization

## Overview

Secure authentication and role-based access control for all RERP services.

## Services

### Idam
- **Path**: `auth/idam/`
- **Description**: Unified authentication and user management service supporting multiple auth methods
- **Documentation**: [Idam README](./idam/README.md)
- **API Spec**: [Idam OpenAPI](./idam/openapi.yaml)

### Rbac
- **Path**: `auth/rbac/`
- **Description**: Role-based access control service managing permissions, roles, and authorization
- **Documentation**: [Rbac README](./rbac/README.md)
- **API Spec**: [Rbac OpenAPI](./rbac/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/auth` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The authentication & authorization services work together to provide complete functionality:

*Integration patterns specific to authentication & authorization will be documented as services are implemented.*
