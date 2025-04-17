# Helpdesk

## Overview

Customer support ticket management and knowledge base.

## Services

### Core
- **Path**: `helpdesk/core/`
- **Description**: Helpdesk service managing customer support tickets and SLA tracking
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Knowledge Base
- **Path**: `helpdesk/knowledge-base/`
- **Description**: Knowledge base service providing self-service documentation
- **Documentation**: [Knowledge Base README](./knowledge-base/README.md)
- **API Spec**: [Knowledge Base OpenAPI](./knowledge-base/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/helpdesk` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The helpdesk services work together to provide complete functionality:

*Integration patterns specific to helpdesk will be documented as services are implemented.*
