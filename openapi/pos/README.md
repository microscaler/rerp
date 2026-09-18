# Point of Sale

## Overview

Point of sale and payment gateway services.

## Services

### Core
- **Path**: `pos/core/`
- **Description**: Point of Sale service for retail and restaurant operations
- **Documentation**: [Core README](./core/README.md)
- **API Spec**: [Core OpenAPI](./core/openapi.yaml)

### Payment Gateway
- **Path**: `pos/payment-gateway/`
- **Description**: Payment gateway service integrating with payment providers
- **Documentation**: [Payment Gateway README](./payment-gateway/README.md)
- **API Spec**: [Payment Gateway OpenAPI](./payment-gateway/openapi.yaml)

## API Gateway

This system provides a unified API gateway at `/api/v1/pos` that:
- Routes requests to appropriate sub-services
- Provides system-level operations (health checks, service discovery)
- Handles cross-service orchestration
- Aggregates metrics and monitoring

## Integration Patterns

The point of sale services work together to provide complete functionality:

*Integration patterns specific to point of sale will be documented as services are implemented.*
