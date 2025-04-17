# CRM Services

## Overview

Customer relationship management with lead tracking, automation, and live chat.

## Services

### Core
- **Path**: `crm/core/`
- **Description**: Core CRM service managing leads, opportunities, contacts, and sales pipeline
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Automation
- **Path**: `crm/automation/`
- **Description**: Workflow automation service for CRM pipelines
- **Documentation**: [Automation README](./automation/README.md)
- **API Spec**: [Automation OpenAPI](./automation/openapi.yaml)

### Livechat
- **Path**: `crm/livechat/`
- **Description**: Real-time live chat service with visitor tracking and agent assignment
- **Documentation**: [Livechat README](./livechat/README.md)
- **API Spec**: [Livechat OpenAPI](./livechat/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/crm` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The crm services services work together to provide complete functionality:

*Integration patterns specific to crm services will be documented as services are implemented.*
