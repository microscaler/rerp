# ESG

## Overview

Environmental, Social, and Governance tracking and reporting.

## Services

### Core
- **Path**: `esg/core/`
- **Description**: ESG service tracking sustainability metrics and ESG reporting
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/esg` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The esg services work together to provide complete functionality:

*Integration patterns specific to esg will be documented as services are implemented.*
