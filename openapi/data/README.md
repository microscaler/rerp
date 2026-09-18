# Data Management

## Overview

Data cleaning, deduplication, and quality management.

## Services

### Cleaning
- **Path**: `data/cleaning/`
- **Description**: Data cleaning service providing data deduplication and quality checks
- **Documentation**: [Cleaning README](./cleaning/README.md)
- **API Spec**: [Cleaning OpenAPI](./cleaning/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/data` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The data management services work together to provide complete functionality:

*Integration patterns specific to data management will be documented as services are implemented.*
