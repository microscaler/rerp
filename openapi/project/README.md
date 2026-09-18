# Project Management

## Overview

Project and task management with timesheet tracking.

## Services

### Core
- **Path**: `project/core/`
- **Description**: Project management service handling project creation and task tracking
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Timesheet
- **Path**: `project/timesheet/`
- **Description**: Timesheet service managing time tracking for projects and tasks
- **Documentation**: [Timesheet README](./timesheet/README.md)
- **API Spec**: [Timesheet OpenAPI](./timesheet/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/project` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The project management services work together to provide complete functionality:

*Integration patterns specific to project management will be documented as services are implemented.*
