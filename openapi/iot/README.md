# IoT

## Overview

IoT device integration and data collection.

## Services

### Core
- **Path**: `iot/core/`
- **Description**: IoT service integrating with IoT devices for data collection
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/iot` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The iot services work together to provide complete functionality:

*Integration patterns specific to iot will be documented as services are implemented.*
