# Marketing

## Overview

Marketing automation, email campaigns, and social media management.

## Services

### Email
- **Path**: `marketing/email/`
- **Description**: Email marketing service managing email campaigns and contact lists
- **Documentation**: [Email README](./email/README.md)
- **API Spec**: [Email OpenAPI](./email/openapi.yaml)

### Automation
- **Path**: `marketing/automation/`
- **Description**: Marketing automation service providing workflow automation and lead nurturing
- **Documentation**: [Automation README](./automation/README.md)
- **API Spec**: [Automation OpenAPI](./automation/openapi.yaml)

### Social Media
- **Path**: `marketing/social-media/`
- **Description**: Social media service managing multi-platform social media posting
- **Documentation**: [Social Media README](./social-media/README.md)
- **API Spec**: [Social Media OpenAPI](./social-media/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/marketing` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The marketing services work together to provide complete functionality:

*Integration patterns specific to marketing will be documented as services are implemented.*
