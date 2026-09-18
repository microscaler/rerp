# Appointment Scheduling

## Overview

Appointment scheduling and calendar integration.

## Services

### Core
- **Path**: `appointments/core/`
- **Description**: Appointment scheduling service managing appointments and calendar integration
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/appointments` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The appointment scheduling services work together to provide complete functionality:

*Integration patterns specific to appointment scheduling will be documented as services are implemented.*
